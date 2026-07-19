use crate::VideoError;
use animate_core::{AnimateConfig, Camera, Color, Sobject};
use pollster::block_on;
use vello::kurbo::Stroke as KurboStroke;
use vello::peniko::Color as VelloColor;
use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions, Scene};

/// 🖼️ Captured mobject state at one timeline sample.
pub struct CapturedFrame {
    pub time: f64,
    pub mobjects: Vec<Box<dyn Sobject>>,
}

/// 🖌️ Headless Vello/wgpu renderer with static-background caching.
pub struct VelloRenderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    renderer: Renderer,
    width: u32,
    height: u32,
    target_texture: wgpu::Texture,
    target_view: wgpu::TextureView,
    readback_buffer: wgpu::Buffer,
    static_cache: Option<StaticBackgroundCache>,
}

struct StaticBackgroundCache {
    hash: String,
    pixels: Vec<u8>,
}

impl VelloRenderer {
    /// 🏗️ Creates a headless wgpu + Vello renderer at `width` × `height`.
    pub fn new(width: u32, height: u32) -> Result<Self, VideoError> {
        let width = width.max(1);
        let height = height.max(1);
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });
        let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }))
        .map_err(|err| VideoError::backend("no wgpu adapter available", format!("{err:?}")))?;
        let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("animate_video"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::Performance,
            trace: wgpu::Trace::Off,
            experimental_features: Default::default(),
        }))
        .map_err(|err| VideoError::backend("wgpu device", format!("{err:?}")))?;
        let renderer = Renderer::new(
            &device,
            RendererOptions {
                use_cpu: false,
                antialiasing_support: AaSupport::area_only(),
                num_init_threads: std::num::NonZeroUsize::new(1),
                pipeline_cache: None,
            },
        )
        .map_err(|err| VideoError::backend("vello renderer", format!("{err:?}")))?;
        let (target_texture, target_view) = create_target_texture(&device, width, height);
        let readback_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("animate_video_readback"),
            size: u64::from(width * height * 4),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        Ok(Self {
            device,
            queue,
            renderer,
            width,
            height,
            target_texture,
            target_view,
            readback_buffer,
            static_cache: None,
        })
    }

    /// 🖼️ Renders captured mobjects to RGBA8 pixels.
    pub fn render_capture(&mut self, capture: &CapturedFrame, camera: &Camera, config: &AnimateConfig) -> Result<Vec<u8>, VideoError> {
        let static_hash = static_layer_hash(capture, config);
        if self.static_cache.as_ref().is_some_and(|cache| cache.hash == static_hash) {
            return Ok(self.static_cache.as_ref().expect("cache").pixels.clone());
        }
        let scene = build_vello_scene(capture, camera, config);
        let background = color_to_vello_array(config.background);
        let pixels = self.render_scene_to_pixels(&scene, background)?;
        self.static_cache = Some(StaticBackgroundCache {
            hash: static_hash,
            pixels: pixels.clone(),
        });
        Ok(pixels)
    }

    fn render_scene_to_pixels(&mut self, scene: &Scene, background: VelloColor) -> Result<Vec<u8>, VideoError> {
        let params = RenderParams {
            base_color: background,
            width: self.width,
            height: self.height,
            antialiasing_method: AaConfig::Area,
        };
        self.renderer
            .render_to_texture(&self.device, &self.queue, scene, &self.target_view, &params)
            .map_err(|err| VideoError::backend("vello render", format!("{err:?}")))?;
        read_pixels(
            &self.device,
            &self.queue,
            &self.target_texture,
            &self.readback_buffer,
            self.width,
            self.height,
        )
    }
}

fn create_target_texture(device: &wgpu::Device, width: u32, height: u32) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("animate_video_target"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
}

fn build_vello_scene(capture: &CapturedFrame, camera: &Camera, config: &AnimateConfig) -> Scene {
    let mut scene = Scene::new();
    let view = scene_affine(camera, config.width, config.height);
    let mut indices: Vec<usize> = (0..capture.mobjects.len()).collect();
    indices.sort_by_key(|&i| (capture.mobjects[i].z_order(), capture.mobjects[i].id()));
    for i in indices {
        paint_mobject(&mut scene, capture.mobjects[i].as_ref(), view);
    }
    scene
}

fn scene_affine(camera: &Camera, width: u32, height: u32) -> vello::kurbo::Affine {
    let sx = width as f64 / camera.frame_width;
    let sy = height as f64 / camera.frame_height;
    vello::kurbo::Affine::new([
        sx,
        0.0,
        0.0,
        -sy,
        width as f64 * 0.5 - camera.frame_center.x() * sx,
        height as f64 * 0.5 + camera.frame_center.y() * sy,
    ]) * camera.transform.to_kurbo()
}

fn paint_mobject(scene: &mut Scene, mobj: &dyn Sobject, view: vello::kurbo::Affine) {
    let transform = view * mobj.transform().to_kurbo();
    let style = mobj.style();
    let opacity = mobj.effective_opacity();
    for path in mobj.paths() {
        let shape = path.to_kurbo();
        if let Some(fill) = style.fill {
            let color = fill.with_alpha(fill.a * style.fill_opacity * opacity);
            scene.fill(
                vello::peniko::Fill::NonZero,
                transform,
                color_to_vello_array(color_from_style(color)),
                None,
                &shape,
            );
        }
        if let Some(stroke) = style.stroke {
            let color = stroke.with_alpha(stroke.a * style.stroke_opacity * opacity);
            let stroke_style = KurboStroke::new(style.stroke_width);
            scene.stroke(
                &stroke_style,
                transform,
                color_to_vello_array(color_from_style(color)),
                None,
                &shape,
            );
        }
    }
}

fn color_to_vello_array(rgba: [f64; 4]) -> VelloColor {
    VelloColor::new([rgba[0] as f32, rgba[1] as f32, rgba[2] as f32, rgba[3] as f32])
}

fn color_from_style(color: Color) -> [f64; 4] {
    color.to_array()
}

pub(crate) fn static_layer_hash(capture: &CapturedFrame, config: &AnimateConfig) -> String {
    use framework_hash::{format_number_for_hash, hash_parts};
    let mut parts = vec![
        format_number_for_hash(config.background[0]),
        format_number_for_hash(config.background[1]),
        format_number_for_hash(config.background[2]),
        format_number_for_hash(config.background[3]),
        capture.mobjects.len().to_string(),
    ];
    for mobj in &capture.mobjects {
        parts.push(mobj.id().to_string());
        parts.push(mobj.z_order().to_string());
        parts.push(format_number_for_hash(mobj.opacity()));
        parts.push(format_number_for_hash(mobj.point_ratio()));
        let coeffs = mobj.transform().to_kurbo().as_coeffs();
        for c in coeffs {
            parts.push(format_number_for_hash(c));
        }
        parts.push(mobj.paths().len().to_string());
        for path in mobj.paths() {
            parts.push(path.elements().len().to_string());
        }
    }
    hash_parts(&parts)
}

pub(crate) fn frame_hash(capture: &CapturedFrame, config: &AnimateConfig) -> String {
    use framework_hash::{format_number_for_hash, hash_parts};
    hash_parts(&[
        format_number_for_hash(capture.time),
        static_layer_hash(capture, config),
    ])
}

fn read_pixels(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    texture: &wgpu::Texture,
    readback_buffer: &wgpu::Buffer,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, VideoError> {
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("animate_video_readback"),
    });
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: readback_buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(Some(encoder.finish()));
    let slice = readback_buffer.slice(..);
    let (sender, receiver) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |result| {
        let _ = sender.send(result);
    });
    let _ = device.poll(wgpu::PollType::wait_indefinitely());
    receiver
        .recv()
        .map_err(|_| VideoError::ReadbackChannelClosed)?
        .map_err(|err| VideoError::backend("map async", format!("{err:?}")))?;
    let data = slice.get_mapped_range();
    let pixels = data.to_vec();
    drop(data);
    readback_buffer.unmap();
    Ok(pixels)
}

#[cfg(test)]
mod tests {
    use super::*;
    use animate_core::VSobject;

    #[test]
    fn vello_renderer_produces_rgba_buffer() {
        let config = AnimateConfig::default().with_resolution(64, 64);
        let camera = Camera::new(config.width as f64 / 100.0, config.height as f64 / 100.0);
        let mut capture = CapturedFrame {
            time: 0.0,
            mobjects: vec![Box::new(VSobject::new())],
        };
        let mut renderer = VelloRenderer::new(config.width, config.height).expect("renderer");
        let pixels = renderer.render_capture(&capture, &camera, &config).expect("frame");
        assert_eq!(pixels.len(), 64 * 64 * 4);
        capture.mobjects.clear();
        let empty = renderer.render_capture(&capture, &camera, &config).expect("empty");
        assert_eq!(empty.len(), 64 * 64 * 4);
    }
}
