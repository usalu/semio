//! 🎨️ Frame renderer of the headless video engine: the captured frames, the Vello renderer for native and wasip2 targets, the vector scene builder and the frame hashes. See [`super`].

use crate::editor::animate::engine::camera::camera::Camera;
use crate::editor::animate::engine::config::config::AnimateConfig;
use crate::editor::animate::engine::scene::sobject::{Sobject, Sobjects};
use crate::editor::animate::engine::text::color::Color;
use crate::editor::animate::engine::video::VideoError;
use geometry::Affine;
use semio_framework_raster::RasterError;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use semio_framework_raster::{SceneRasterizer, VectorScene};

/// 🖼️ Captured mobject state at one timeline sample.
pub struct CapturedFrame {
    pub time: f64,
    pub mobjects: Vec<Sobjects>,
}

/// 🖌️ Headless Vello/wgpu renderer (via `semio_framework_raster::SceneRasterizer`) with
/// static-background caching. Native/host-only body; see the `wasm32-wasip2` variant below —
/// the native site compiler reaches it through `compile_scene_to_assets` → `render_scene`, and
/// that chain must keep compiling on every target. The guest's `export-video-from-deck` never
/// comes here: it builds a `VideoRenderProgram` (`program::VideoProgramBuilder` paints exactly
/// what [`VelloRenderer::render_capture`] paints) and the host renders it.
/// `🔍️research/📓️raster-tier-split.md` has the tier split.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub struct VelloRenderer {
    rasterizer: SceneRasterizer,
    static_cache: Option<StaticBackgroundCache>,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
struct StaticBackgroundCache {
    hash: String,
    pixels: Vec<u8>,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl VelloRenderer {
    /// 🏗️ Creates a headless wgpu + Vello renderer at `width` × `height`.
    pub async fn new(width: u32, height: u32) -> Result<Self, VideoError> {
        let rasterizer = SceneRasterizer::new(width, height).await.map_err(video_error_from_raster)?;
        Ok(Self { rasterizer, static_cache: None })
    }

    /// 🖼️ Renders captured mobjects to RGBA8 pixels.
    pub fn render_capture(&mut self, capture: &CapturedFrame, camera: &Camera, config: &AnimateConfig) -> Result<Vec<u8>, VideoError> {
        let static_hash = static_layer_hash(capture, config);
        if self.static_cache.as_ref().is_some_and(|cache| cache.hash == static_hash) {
            return Ok(self.static_cache.as_ref().expect("cache").pixels.clone());
        }
        let scene = build_vector_scene(capture, camera, config);
        let background = color_to_rgba_array(config.background);
        let pixels = self.rasterizer.render(&scene, background).map_err(video_error_from_raster)?;
        self.static_cache = Some(StaticBackgroundCache { hash: static_hash, pixels: pixels.clone() });
        Ok(pixels)
    }
}

/// 🚫️ A `wasm32-wasip2` guest component has no GPU device access — WASI Preview 2 defines no
/// graphics API, so no amount of first-party wrapping makes real rasterization possible here.
/// This is NOT a stub: every call returns the same honest `RasterError::Adapter` a native host
/// reports when it genuinely finds no adapter (see the raster crate's own
/// `[TRACE] no wgpu adapter in this environment` test fallback for the precedent), surfaced to
/// the caller as a real `Fault`/error effect rather than a silently-dropped capability. Kept
/// zero-sized: it names neither `wgpu` nor `vello`, so it adds nothing to the shipped
/// component's link graph.
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
pub struct VelloRenderer;

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
impl VelloRenderer {
    /// 🏗️ Always reports "no adapter" — see struct docstring.
    pub async fn new(_width: u32, _height: u32) -> Result<Self, VideoError> {
        Err(video_error_from_raster(RasterError::Adapter("wasm32-wasip2 has no GPU device access (WASI Preview 2 defines no graphics API); headless video rendering is a native/host-only capability".into())))
    }

    /// 🖼️ Always reports "no adapter" — see struct docstring. Reachable only if a caller
    /// somehow held a `Self` past a failed `new`, which the type's own API cannot produce.
    pub fn render_capture(&mut self, _capture: &CapturedFrame, _camera: &Camera, _config: &AnimateConfig) -> Result<Vec<u8>, VideoError> {
        Err(video_error_from_raster(RasterError::Adapter("wasm32-wasip2 has no GPU device access (WASI Preview 2 defines no graphics API); headless video rendering is a native/host-only capability".into())))
    }
}

fn video_error_from_raster(error: RasterError) -> VideoError {
    match error {
        RasterError::ReadbackChannelClosed => VideoError::ReadbackChannelClosed,
        other => VideoError::backend("raster", other),
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn build_vector_scene(capture: &CapturedFrame, camera: &Camera, config: &AnimateConfig) -> VectorScene {
    let mut scene = VectorScene::new();
    let view = scene_affine(camera, config.width, config.height);
    let mut indices: Vec<usize> = (0..capture.mobjects.len()).collect();
    indices.sort_by_key(|&i| (capture.mobjects[i].z_order(), capture.mobjects[i].id()));
    for i in indices {
        paint_mobject(&mut scene, &capture.mobjects[i], view);
    }
    scene
}

/// 📐️ Scene units → device pixels (y down) for `camera` on a `width` × `height` picture.
pub(crate) fn scene_affine(camera: &Camera, width: u32, height: u32) -> Affine {
    let sx = width as f64 / camera.frame_width;
    let sy = height as f64 / camera.frame_height;
    Affine::new([sx, 0.0, 0.0, -sy, width as f64 * 0.5 - camera.frame_center.x() * sx, height as f64 * 0.5 + camera.frame_center.y() * sy]) * camera.transform
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn paint_mobject(scene: &mut VectorScene, mobj: &Sobjects, view: Affine) {
    let transform = view * mobj.transform();
    let style = mobj.style();
    let opacity = mobj.effective_opacity();
    for path in mobj.paths() {
        if let Some(fill) = style.fill {
            let color = fill.with_alpha(fill.a * style.fill_opacity * opacity);
            scene.fill(path.clone(), transform, color_to_rgba_array(color_from_style(color)));
        }
        if let Some(stroke) = style.stroke {
            let color = stroke.with_alpha(stroke.a * style.stroke_opacity * opacity);
            scene.stroke(path.clone(), transform, color_to_rgba_array(color_from_style(color)), style.stroke_width);
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn color_to_rgba_array(rgba: [f64; 4]) -> [f32; 4] {
    [rgba[0] as f32, rgba[1] as f32, rgba[2] as f32, rgba[3] as f32]
}

/// 🎨️ A style colour as straight RGBA.
pub(crate) fn color_from_style(color: Color) -> [f64; 4] {
    color.to_array()
}

pub(crate) fn static_layer_hash(capture: &CapturedFrame, config: &AnimateConfig) -> String {
    use framework_hash::{format_number_for_hash, hash_parts};
    let mut parts = vec![format_number_for_hash(config.background[0]), format_number_for_hash(config.background[1]), format_number_for_hash(config.background[2]), format_number_for_hash(config.background[3]), capture.mobjects.len().to_string()];
    for mobj in &capture.mobjects {
        parts.push(mobj.id().to_string());
        parts.push(mobj.z_order().to_string());
        parts.push(format_number_for_hash(mobj.opacity()));
        parts.push(format_number_for_hash(mobj.point_ratio()));
        let coeffs = mobj.transform().as_coeffs();
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
    hash_parts(&[format_number_for_hash(capture.time), static_layer_hash(capture, config)])
}

#[cfg(all(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
include!("../🧪️tests/🔬️renderer-unit/🦀️.rs");
