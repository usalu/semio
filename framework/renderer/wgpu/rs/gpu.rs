//! 🖥️ WebGPU device, surface, and frame loop for the WASM renderer.

use crate::draw::{DrawList, UiPipelines};
use crate::text::FontAtlas;
use wgpu::Surface;

pub struct GpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub pipelines: UiPipelines,
    pub width: u32,
    pub height: u32,
    pub dpr: f32,
}

impl GpuContext {
    pub async fn from_canvas(canvas: web_sys::HtmlCanvasElement, dpr: f32) -> Result<Self, String> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU,
            ..Default::default()
        });
        let surface = instance
            .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
            .map_err(|err| format!("surface: {err:?}"))?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|err| format!("adapter: {err:?}"))?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("framework_renderer_wgpu"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
                experimental_features: Default::default(),
            })
            .await
            .map_err(|err| format!("device: {err:?}"))?;
        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);
        let width = 1;
        let height = 1;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
        let pipelines = UiPipelines::new(&device, &queue, format);
        Ok(Self {
            device,
            queue,
            surface,
            config,
            pipelines,
            width,
            height,
            dpr,
        })
    }

    pub fn resize(&mut self, css_width: f32, css_height: f32, dpr: f32) {
        self.dpr = dpr;
        let width = (css_width * dpr).max(1.0) as u32;
        let height = (css_height * dpr).max(1.0) as u32;
        if width == self.width && height == self.height {
            return;
        }
        self.width = width;
        self.height = height;
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render_frame(&mut self, draw: &DrawList) -> Result<(), String> {
        let frame = self
            .surface
            .get_current_texture()
            .map_err(|err| format!("frame: {err:?}"))?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("frame_encoder") });
        self.pipelines.render(
            &self.device,
            &self.queue,
            &mut encoder,
            &view,
            draw,
            self.width as f32,
            self.height as f32,
        );
        self.queue.submit(Some(encoder.finish()));
        frame.present();
        Ok(())
    }

    pub fn upload_font_atlas(&self, atlas: &FontAtlas) {
        self.pipelines
            .upload_glyph_atlas(&self.queue, &atlas.pixels, atlas.width, atlas.height);
    }
}

#[cfg(target_arch = "wasm32")]
pub fn schedule_frame(callback: impl FnMut() + 'static) {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    let mut callback = callback;
    let closure = Closure::wrap(Box::new(move || {
        callback();
    }) as Box<dyn FnMut()>);
    web_sys::window()
        .and_then(|w| {
            w.request_animation_frame(closure.as_ref().unchecked_ref())
                .ok()
        });
    closure.forget();
}
