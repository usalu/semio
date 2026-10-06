//! 🧊️ Canvas rendering ports keep real backend owners private in the defining UI crate.

use super::Target;
use crate::canvas::{Color, Scene};
use crate::wgpu::{GpuContext, RasterTextureAdmission, RasterTextureStageFault, RasterTextureWitness};

pub struct Texture {
    raw: wgpu::Texture,
    target: Target,
}

pub struct View {
    raw: wgpu::TextureView,
    target: Target,
}

pub struct Renderer {
    raw: vello::Renderer,
}

pub enum StageFault {
    Returned { fault: &'static str, admission: RasterTextureAdmission, texture: Texture, view: View },
    Retained(&'static str),
}

impl Texture {
    pub fn create(gpu: &GpuContext, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<Self, String> {
        gpu.validate_engine_target_texture_allocation(admission, expected)?;
        let (width, height) = admission.target_extent();
        let target = Target::try_new(width, height).map_err(|fault| fault.to_string())?;
        let raw = gpu.device().create_texture(&wgpu::TextureDescriptor {
            label: Some("engine_canvas_target"),
            size: wgpu::Extent3d { width: target.width(), height: target.height(), depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        Ok(Self { raw, target })
    }

    pub fn view(&self, gpu: &GpuContext, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<View, String> {
        gpu.validate_engine_target_view_allocation(admission, expected)?;
        Ok(View { raw: self.raw.create_view(&wgpu::TextureViewDescriptor::default()), target: self.target })
    }
}

impl Renderer {
    pub fn create(gpu: &GpuContext, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<Self, String> {
        gpu.validate_engine_renderer_allocation(admission, expected)?;
        let raw = vello::Renderer::new(gpu.device(), vello::RendererOptions { use_cpu: false, antialiasing_support: vello::AaSupport::area_only(), num_init_threads: std::num::NonZeroUsize::new(1), pipeline_cache: None })
            .map_err(|error| format!("vello renderer: {error:?}"))?;
        Ok(Self { raw })
    }

    pub fn render(&mut self, gpu: &GpuContext, scene: &Scene, view: &View, clear: Color) -> Result<(), String> {
        let params = vello::RenderParams { base_color: clear.to_peniko(), width: view.target.width(), height: view.target.height(), antialiasing_method: vello::AaConfig::Area };
        let scene = scene.vello_scene();
        self.raw.render_to_texture(gpu.device(), gpu.queue(), &scene, &view.raw, &params).map_err(|error| format!("vello render: {error:?}"))
    }
}

#[expect(clippy::result_large_err, reason = "Returns the exact reservation, texture and view owners without allocating on refusal.")]
pub fn stage(gpu: &mut GpuContext, admission: RasterTextureAdmission, texture: Texture, view: View, expected: RasterTextureWitness) -> Result<(), StageFault> {
    let target = texture.target;
    let view_target = view.target;
    match gpu.stage_engine_texture(admission, texture.raw, view.raw, expected) {
        Ok(()) => Ok(()),
        Err(RasterTextureStageFault::Returned { fault, admission, texture, view }) => Err(StageFault::Returned { fault, admission, texture: Texture { raw: texture, target }, view: View { raw: view, target: view_target } }),
        Err(RasterTextureStageFault::Retained(fault)) => Err(StageFault::Retained(fault)),
    }
}
