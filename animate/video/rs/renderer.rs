use animate_core::sobject::{Sobject, SobjectShape};
use animate_core::FrameSnapshot;
use infinite_cavas::{Color, FillRule, Scene, ShapeRef, Stroke};
use mathematical_geometry::{Affine, Circle};
use pollster::block_on;
use vello::peniko::Color as VelloColor;
use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions};

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
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
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
        .ok_or_else(|| "no wgpu adapter available".to_string())?;
        let (device, queue) = block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("animate_video"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::Performance,
        }))
        .map_err(|err| format!("wgpu device: {err:?}"))?;
        let renderer = Renderer::new(
            &device,
            RendererOptions {
                use_cpu: false,
                antialiasing_support: AaSupport::area_only(),
                num_init_threads: std::num::NonZeroUsize::new(1),
                pipeline_cache: None,
            },
        )
        .map_err(|err| format!("vello renderer: {err:?}"))?;
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

    /// 🖼️ Renders a frame snapshot to RGBA8 pixels.
    pub fn render_frame(&mut self, snapshot: &FrameSnapshot) -> Result<Vec<u8>, String> {
        let static_hash = snapshot.static_layer_hash();
        let cache_hit = self.static_cache.as_ref().is_some_and(|cache| cache.hash == static_hash);
        if cache_hit && snapshot.mobjects.moving_objects().is_empty() {
            return Ok(self.static_cache.as_ref().expect("cache").pixels.clone());
        }
        if cache_hit {
            let mut pixels = self.static_cache.as_ref().expect("cache").pixels.clone();
            let moving_scene = build_vello_scene(snapshot, true);
            let overlay = self.render_scene_to_pixels(&moving_scene, [0.0, 0.0, 0.0, 0.0])?;
            alpha_composite(&mut pixels, &overlay);
            return Ok(pixels);
        }
        let full_scene = build_vello_scene(snapshot, false);
        let pixels = self.render_scene_to_pixels(&full_scene, snapshot.background_color)?;
        let static_scene = build_static_scene(snapshot);
        let static_pixels = self.render_scene_to_pixels(&static_scene, snapshot.background_color)?;
        self.static_cache = Some(StaticBackgroundCache {
            hash: static_hash,
            pixels: static_pixels,
        });
        Ok(pixels)
    }

    fn render_scene_to_pixels(&mut self, scene: &Scene, background: [f32; 4]) -> Result<Vec<u8>, String> {
        let params = RenderParams {
            base_color: VelloColor::new(background),
            width: self.width,
            height: self.height,
            antialiasing_method: AaConfig::Area,
        };
        self.renderer
            .render_to_texture(&self.device, &self.queue, scene.vello_scene(), &self.target_view, &params)
            .map_err(|err| format!("vello render: {err:?}"))?;
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

fn build_static_scene(snapshot: &FrameSnapshot) -> Scene {
    let mut scene = Scene::new();
    for sobject in snapshot.mobjects.static_objects() {
        paint_sobject(&mut scene, sobject);
    }
    scene
}

fn build_vello_scene(snapshot: &FrameSnapshot, moving_only: bool) -> Scene {
    let mut scene = Scene::new();
    let objects = if moving_only {
        snapshot.mobjects.moving_objects()
    } else {
        snapshot.mobjects.sorted()
    };
    for sobject in objects {
        paint_sobject(&mut scene, sobject);
    }
    scene
}

fn paint_sobject(scene: &mut Scene, sobject: &Sobject) {
    let transform = affine_to_cavas(sobject.transform);
    match &sobject.shape {
        SobjectShape::Circle { center, radius } => {
            let circle = Circle::new(*center, *radius);
            if let Some(fill) = &sobject.fill {
                scene.fill(FillRule::NonZero, transform, color_to_paint(fill.color), None, ShapeRef::Circle(&circle));
            }
            if let Some(stroke) = &sobject.stroke {
                let stroke_style = Stroke::new(stroke.width);
                scene.stroke(&stroke_style, transform, color_to_paint(stroke.color), None, ShapeRef::Circle(&circle));
            }
        }
        SobjectShape::Rect { rect } => {
            if let Some(fill) = &sobject.fill {
                scene.fill(FillRule::NonZero, transform, color_to_paint(fill.color), None, ShapeRef::Rect(rect));
            }
            if let Some(stroke) = &sobject.stroke {
                let stroke_style = Stroke::new(stroke.width);
                scene.stroke(&stroke_style, transform, color_to_paint(stroke.color), None, ShapeRef::Rect(rect));
            }
        }
        SobjectShape::RoundedRect { rect } => {
            if let Some(fill) = &sobject.fill {
                scene.fill(FillRule::NonZero, transform, color_to_paint(fill.color), None, ShapeRef::RoundedRect(rect));
            }
            if let Some(stroke) = &sobject.stroke {
                let stroke_style = Stroke::new(stroke.width);
                scene.stroke(&stroke_style, transform, color_to_paint(stroke.color), None, ShapeRef::RoundedRect(rect));
            }
        }
        SobjectShape::Line { line } => {
            if let Some(stroke) = &sobject.stroke {
                let stroke_style = Stroke::new(stroke.width);
                scene.stroke(&stroke_style, transform, color_to_paint(stroke.color), None, ShapeRef::Line(line));
            }
        }
        SobjectShape::Arc { arc } => {
            if let Some(fill) = &sobject.fill {
                scene.fill(FillRule::NonZero, transform, color_to_paint(fill.color), None, ShapeRef::Arc(arc));
            }
            if let Some(stroke) = &sobject.stroke {
                let stroke_style = Stroke::new(stroke.width);
                scene.stroke(&stroke_style, transform, color_to_paint(stroke.color), None, ShapeRef::Arc(arc));
            }
        }
        SobjectShape::Path { path } => {
            if let Some(fill) = &sobject.fill {
                scene.fill(FillRule::NonZero, transform, color_to_paint(fill.color), None, ShapeRef::BezPath(path));
            }
            if let Some(stroke) = &sobject.stroke {
                let stroke_style = Stroke::new(stroke.width);
                scene.stroke(&stroke_style, transform, color_to_paint(stroke.color), None, ShapeRef::BezPath(path));
            }
        }
    }
}

fn affine_to_cavas(affine: mathematical_geometry::Affine) -> Affine {
    Affine::new(affine.to_kurbo().as_coeffs())
}

fn color_to_paint(color: [f32; 4]) -> Color {
    Color::new(color)
}

fn alpha_composite(base: &mut [u8], overlay: &[u8]) {
    for (dst, src) in base.chunks_exact_mut(4).zip(overlay.chunks_exact(4)) {
        let alpha = f32::from(src[3]) / 255.0;
        if alpha <= 0.0 {
            continue;
        }
        for channel in 0..3 {
            let b = f32::from(dst[channel]);
            let o = f32::from(src[channel]);
            dst[channel] = (o * alpha + b * (1.0 - alpha)).round().clamp(0.0, 255.0) as u8;
        }
        dst[3] = 255;
    }
}

fn read_pixels(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    texture: &wgpu::Texture,
    readback_buffer: &wgpu::Buffer,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, String> {
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
    receiver.recv().map_err(|_| "readback channel closed".to_string())??;
    let data = slice.get_mapped_range();
    Ok(data.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use animate_core::sobject::{Mobility, MobjectStore, PaintStyle, Sobject, SobjectId, SobjectShape};
    use animate_core::FrameSnapshot;
    use mathematical_geometry::Point;

    fn circle_snapshot() -> FrameSnapshot {
        let mut store = MobjectStore::default();
        store.add(Sobject {
            id: SobjectId(0),
            shape: SobjectShape::Circle {
                center: Point::new(0.0, 0.0),
                radius: 1.0,
            },
            transform: mathematical_geometry::Affine::IDENTITY,
            fill: Some(PaintStyle { color: [1.0, 1.0, 1.0, 1.0] }),
            stroke: None,
            z_index: 0,
            mobility: Mobility::Static,
        });
        FrameSnapshot {
            frame_index: 0,
            time: 0.0,
            mobjects: store,
            background_color: [0.0, 0.0, 0.0, 1.0],
        }
    }

    #[test]
    fn vello_renderer_produces_rgba_buffer() {
        let mut renderer = VelloRenderer::new(64, 64).expect("renderer");
        let pixels = renderer.render_frame(&circle_snapshot()).expect("frame");
        assert_eq!(pixels.len(), 64 * 64 * 4);
        assert!(pixels.iter().any(|&b| b > 0));
    }
}
