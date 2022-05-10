use crate::prelude::*;
use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_pipeline_properties.html>
#[derive(Clone)]
pub struct PipelineProperties {
    handle: vk::Device,
    fp: vk::ExtPipelinePropertiesFn,
}

impl PipelineProperties {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::ExtPipelinePropertiesFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelinePropertiesEXT.html>
    ///
    /// TODO: Currently only accepts [`vk::PipelinePropertiesIdentifierEXT`]
    pub unsafe fn get_pipeline_properties(
        &self,
        pipeline_info: &vk::PipelineInfoEXT,
        // TODO: This is a new "paradigm" like how structextends is used for pNext,
        // where vk::PipelinePropertiesIdentifierEXT "extends" BaseOutStructure.
        // This does very little to bind the struct explicitly to this function
        // as soon as more interfaces start using BaseOutStructure as function argument.
        // This will be taken up with Khronos first before #619 (dependency for the generated
        // bits) is mergeable.
        pipeline_properties: *mut vk::BaseOutStructure,
    ) -> VkResult<()> {
        (self.fp.get_pipeline_properties_ext)(self.handle, pipeline_info, pipeline_properties)
            .result()
    }

    pub const fn name() -> &'static CStr {
        vk::ExtPipelinePropertiesFn::name()
    }

    pub fn fp(&self) -> &vk::ExtPipelinePropertiesFn {
        &self.fp
    }

    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
