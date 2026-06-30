//! 🖼️ Non-destructive raster compositor on the infinite canvas (Vello/WebGPU).

pub use infinite_cavas::{self as cavas, *};
pub use std::sync::Arc;
pub use vello::kurbo::{Affine, Point, Rect, Stroke, Vec2};
pub use vello::peniko::{Blob, Color, Fill, ImageAlphaType, ImageData, ImageFormat, Mix};
pub use vello::Scene;

use cavas::camera::{Camera, Viewport};
use serde::Deserialize;
use std::collections::HashMap;

// #region 🔖Document
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind")]
enum LayerNodeJson {
    #[serde(rename = "pixel", rename_all = "camelCase")]
    Pixel {
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default = "default_opacity")]
        opacity: f32,
        #[serde(default)]
        blend_mode: String,
        transform: TransformJson,
        mask: Option<MaskJson>,
        #[serde(default)]
        clip_to_below: bool,
        width: Option<u32>,
        height: Option<u32>,
        #[serde(default)]
        image_key: Option<String>,
        #[serde(default)]
        filters: Vec<FilterJson>,
    },
    #[serde(rename = "group", rename_all = "camelCase")]
    Group {
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default = "default_opacity")]
        opacity: f32,
        #[serde(default)]
        blend_mode: String,
        transform: TransformJson,
        mask: Option<MaskJson>,
        #[serde(default)]
        clip_to_below: bool,
        children: Vec<LayerNodeJson>,
    },
    #[serde(rename = "adjustment", rename_all = "camelCase")]
    Adjustment {
        id: String,
        name: String,
        #[serde(default = "default_true")]
        visible: bool,
        #[serde(default = "default_opacity")]
        opacity: f32,
        #[serde(default)]
        blend_mode: String,
        transform: TransformJson,
        adjustment_kind: String,
        params: AdjustmentParamsJson,
    },
}

fn default_true() -> bool {
    true
}

fn default_opacity() -> f32 {
    1.0
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransformJson {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "default_one")]
    scale_x: f64,
    #[serde(default = "default_one")]
    scale_y: f64,
    #[serde(default)]
    rotation: f64,
}

fn default_one() -> f64 {
    1.0
}

#[derive(Clone, Debug, Deserialize)]
struct MaskJson {
    #[serde(default = "default_true")]
    enabled: bool,
    #[serde(default = "default_true")]
    linked: bool,
    #[serde(default)]
    invert: bool,
    width: Option<u32>,
    height: Option<u32>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FilterJson {
    kind: String,
    radius: Option<f32>,
    amount: Option<f32>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AdjustmentParamsJson {
    brightness: Option<f32>,
    contrast: Option<f32>,
    hue: Option<f32>,
    saturation: Option<f32>,
    levels_black: Option<f32>,
    levels_white: Option<f32>,
}

#[derive(Clone, Debug, Deserialize)]
struct CameraJson {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "default_one")]
    zoom: f64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DocumentJson {
    schema: String,
    id: String,
    camera: CameraJson,
    layers: Vec<LayerNodeJson>,
    #[serde(default)]
    active_tool: Option<String>,
    #[serde(default)]
    brush_size: Option<f32>,
    #[serde(default)]
    brush_opacity: Option<f32>,
}

#[derive(Clone)]
enum LayerNode {
    Pixel {
        id: String,
        visible: bool,
        opacity: f32,
        blend: Mix,
        transform: Affine,
        width: u32,
        height: u32,
        image_key: Option<String>,
        mask: Option<MaskState>,
    },
    Group {
        id: String,
        visible: bool,
        opacity: f32,
        blend: Mix,
        transform: Affine,
        children: Vec<LayerNode>,
        mask: Option<MaskState>,
    },
    Adjustment {
        id: String,
        visible: bool,
        opacity: f32,
        blend: Mix,
        kind: String,
        params: AdjustmentParamsJson,
    },
}

#[derive(Clone)]
struct MaskState {
    enabled: bool,
    invert: bool,
    width: u32,
    height: u32,
}

#[derive(Clone)]
struct RasterDocument {
    layers: Vec<LayerNode>,
}

fn blend_from_str(raw: &str) -> Mix {
    match raw {
        "multiply" => Mix::Multiply,
        "screen" => Mix::Screen,
        "overlay" => Mix::Overlay,
        "darken" => Mix::Darken,
        "lighten" => Mix::Lighten,
        "colorDodge" => Mix::ColorDodge,
        "colorBurn" => Mix::ColorBurn,
        "hardLight" => Mix::HardLight,
        "softLight" => Mix::SoftLight,
        "difference" => Mix::Difference,
        "exclusion" => Mix::Exclusion,
        "hue" => Mix::Hue,
        "saturation" => Mix::Saturation,
        "color" => Mix::Color,
        "luminosity" => Mix::Luminosity,
        _ => Mix::Normal,
    }
}

fn affine_from_json(t: &TransformJson) -> Affine {
    Affine::translate((t.x, t.y)) * Affine::rotate(t.rotation) * Affine::scale_non_uniform(t.scale_x, t.scale_y)
}

fn parse_mask(m: &MaskJson) -> MaskState {
    MaskState {
        enabled: m.enabled,
        invert: m.invert,
        width: m.width.unwrap_or(512),
        height: m.height.unwrap_or(512),
    }
}

fn parse_layer(raw: LayerNodeJson) -> LayerNode {
    match raw {
        LayerNodeJson::Pixel {
            id,
            visible,
            opacity,
            blend_mode,
            transform,
            mask,
            width,
            height,
            image_key,
            ..
        } => LayerNode::Pixel {
            id,
            visible,
            opacity: opacity.clamp(0.0, 1.0),
            blend: blend_from_str(&blend_mode),
            transform: affine_from_json(&transform),
            width: width.unwrap_or(512),
            height: height.unwrap_or(512),
            image_key,
            mask: mask.map(|m| parse_mask(&m)),
        },
        LayerNodeJson::Group {
            id,
            visible,
            opacity,
            blend_mode,
            transform,
            mask,
            children,
            ..
        } => LayerNode::Group {
            id,
            visible,
            opacity: opacity.clamp(0.0, 1.0),
            blend: blend_from_str(&blend_mode),
            transform: affine_from_json(&transform),
            children: children.into_iter().map(parse_layer).collect(),
            mask: mask.map(|m| parse_mask(&m)),
        },
        LayerNodeJson::Adjustment {
            id,
            visible,
            opacity,
            blend_mode,
            adjustment_kind,
            params,
            ..
        } => LayerNode::Adjustment {
            id,
            visible,
            opacity: opacity.clamp(0.0, 1.0),
            blend: blend_from_str(&blend_mode),
            kind: adjustment_kind,
            params,
        },
    }
}

fn parse_document(json: &str) -> Result<RasterDocument, String> {
    let doc: DocumentJson = serde_json::from_str(json).map_err(|e| e.to_string())?;
    if doc.schema != "raster.document/v1" {
        return Err(format!("unsupported schema {}", doc.schema));
    }
    Ok(RasterDocument {
        layers: doc.layers.into_iter().map(parse_layer).collect(),
    })
}
// #endregion 🔖Document

// #region 🔖Pixels
fn checkerboard_rgba(width: u32, height: u32, light: bool) -> Vec<u8> {
    let mut rgba = vec![0u8; (width * height * 4) as usize];
    let cell = 16u32;
    for y in 0..height {
        for x in 0..width {
            let idx = ((y * width + x) * 4) as usize;
            let on = ((x / cell) + (y / cell)) % 2 == 0;
            let v = if on == light { 220u8 } else { 180u8 };
            rgba[idx] = v;
            rgba[idx + 1] = v;
            rgba[idx + 2] = v;
            rgba[idx + 3] = 255;
        }
    }
    rgba
}

fn image_from_rgba(width: u32, height: u32, rgba: Vec<u8>) -> ImageData {
    ImageData {
        data: Blob::new(Arc::new(rgba)),
        format: ImageFormat::Rgba8,
        alpha_type: ImageAlphaType::Alpha,
        width,
        height,
    }
}

fn apply_brightness_contrast(rgba: &mut [u8], brightness: f32, contrast: f32) {
    let b = brightness;
    let c = contrast;
    for px in rgba.chunks_exact_mut(4) {
        for ch in 0..3 {
            let v = px[ch] as f32 / 255.0;
            let adjusted = ((v - 0.5) * (1.0 + c) + 0.5 + b).clamp(0.0, 1.0);
            px[ch] = (adjusted * 255.0).round() as u8;
        }
    }
}

fn apply_blur_box(rgba: &mut [u8], width: u32, height: u32, radius: u32) {
    if radius == 0 {
        return;
    }
    let r = radius.min(8);
    let src = rgba.to_vec();
    for y in 0..height {
        for x in 0..width {
            let mut acc = [0u32; 4];
            let mut count = 0u32;
            for dy in 0..=(r * 2) {
                for dx in 0..=(r * 2) {
                    let sx = x.saturating_add(dx).saturating_sub(r).min(width - 1);
                    let sy = y.saturating_add(dy).saturating_sub(r).min(height - 1);
                    let idx = ((sy * width + sx) * 4) as usize;
                    for c in 0..4 {
                        acc[c] += src[idx + c] as u32;
                    }
                    count += 1;
                }
            }
            let idx = ((y * width + x) * 4) as usize;
            for c in 0..4 {
                rgba[idx + c] = (acc[c] / count) as u8;
            }
        }
    }
}
// #endregion 🔖Pixels

// #region 🔖Host
pub struct RasterHost {
    camera: Camera,
    viewport: Viewport,
    document: RasterDocument,
    images: cavas::raster::RasterImageCache,
    paint_buffers: HashMap<String, Vec<u8>>,
    mask_buffers: HashMap<String, Vec<u8>>,
    active_tool: String,
    brush_size: f32,
    brush_opacity: f32,
    hovered_id: Option<String>,
    selected_ids: Vec<String>,
    panning: bool,
    painting: bool,
    last_paint: Option<Point>,
    theme_clear: Color,
}

impl Default for RasterHost {
    fn default() -> Self {
        Self::new()
    }
}

impl RasterHost {
    pub fn new() -> Self {
        Self {
            camera: Camera { x: 0.0, y: 0.0, zoom: 1.0 },
            viewport: Viewport { width: 800, height: 600, dpr: 1.0 },
            document: RasterDocument { layers: vec![] },
            images: cavas::raster::RasterImageCache::default(),
            paint_buffers: HashMap::new(),
            mask_buffers: HashMap::new(),
            active_tool: "selectMarquee".into(),
            brush_size: 24.0,
            brush_opacity: 1.0,
            hovered_id: None,
            selected_ids: vec![],
            panning: false,
            painting: false,
            last_paint: None,
            theme_clear: Color::from_rgba8(32, 32, 36, 255),
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        self.viewport.width = width.max(1);
        self.viewport.height = height.max(1);
        self.viewport.dpr = dpr.max(1.0);
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.camera.x = x;
        self.camera.y = y;
        self.camera.zoom = zoom.max(0.05);
    }

    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        let factor = if delta_y < 0.0 { 1.1 } else { 1.0 / 1.1 };
        let world_before = self.screen_to_world(sx, sy);
        self.camera.zoom = (self.camera.zoom * factor).clamp(0.05, 64.0);
        let world_after = self.screen_to_world(sx, sy);
        self.camera.x += world_before.x - world_after.x;
        self.camera.y += world_before.y - world_after.y;
    }

    fn screen_to_world(&self, sx: f64, sy: f64) -> Point {
        cavas::camera::screen_to_world(&self.camera, &self.viewport, Point::new(sx, sy))
    }

    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8) {
        if button == 1 {
            self.panning = true;
            return;
        }
        if self.active_tool.starts_with("paint") {
            self.painting = true;
            self.last_paint = Some(self.screen_to_world(sx, sy));
            self.paint_at(self.last_paint.unwrap());
        }
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64) {
        let world = self.screen_to_world(sx, sy);
        if self.panning {
            let cx = self.viewport.width as f64 * 0.5;
            let cy = self.viewport.height as f64 * 0.5;
            let prev = cavas::camera::screen_to_world(
                &self.camera,
                &self.viewport,
                Point::new(world.x * 0.0 + cx, world.y * 0.0 + cy),
            );
            let _ = prev;
            return;
        }
        if self.painting {
            if let Some(last) = self.last_paint {
                self.stroke_paint(last, world);
            }
            self.last_paint = Some(world);
        }
    }

    pub fn pointer_up_screen(&mut self, _sx: f64, _sy: f64) {
        self.panning = false;
        self.painting = false;
        self.last_paint = None;
    }

    fn layer_pixel_buffer_key(id: &str) -> String {
        format!("layer:{id}")
    }

    fn ensure_layer_buffer(&mut self, id: &str, width: u32, height: u32) -> &mut Vec<u8> {
        let key = Self::layer_pixel_buffer_key(id);
        let len = (width * height * 4) as usize;
        self.paint_buffers
            .entry(key)
            .or_insert_with(|| checkerboard_rgba(width, height, true));
        let buf = self.paint_buffers.get_mut(&Self::layer_pixel_buffer_key(id)).unwrap();
        if buf.len() != len {
            *buf = checkerboard_rgba(width, height, true);
        }
        buf
    }

    fn paint_at(&mut self, world: Point) {
        let radius = (self.brush_size as f64 * 0.5).max(1.0);
        let layer_id = self.selected_ids.first().cloned().unwrap_or_else(|| "bg".into());
        let (width, height) = (512u32, 512u32);
        let brush_opacity = self.brush_opacity;
        let is_eraser = self.active_tool == "paintEraser";
        let buf = self.ensure_layer_buffer(&layer_id, width, height);
        let cx = world.x.round() as i32;
        let cy = world.y.round() as i32;
        let r = radius as i32;
        for dy in -r..=r {
            for dx in -r..=r {
                if (dx * dx + dy * dy) > r * r {
                    continue;
                }
                let x = cx + dx;
                let y = cy + dy;
                if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 {
                    continue;
                }
                let idx = ((y as u32 * width + x as u32) * 4) as usize;
                let alpha = (brush_opacity * 255.0) as u8;
                if is_eraser {
                    buf[idx + 3] = buf[idx + 3].saturating_sub(alpha);
                } else {
                    buf[idx] = 40;
                    buf[idx + 1] = 120;
                    buf[idx + 2] = 220;
                    buf[idx + 3] = alpha.max(buf[idx + 3]);
                }
            }
        }
        let rgba = buf.clone();
        let image = image_from_rgba(width, height, rgba);
        self.images.insert(Self::layer_pixel_buffer_key(&layer_id), image);
    }

    fn stroke_paint(&mut self, from: Point, to: Point) {
        let steps = ((to.x - from.x).hypot(to.y - from.y) / 2.0).ceil().max(1.0) as i32;
        for i in 0..=steps {
            let t = i as f64 / steps as f64;
            let p = Point::new(from.x + (to.x - from.x) * t, from.y + (to.y - from.y) * t);
            self.paint_at(p);
        }
    }

    pub fn sync_document_json(&mut self, json: &str) -> Result<(), String> {
        self.document = parse_document(json)?;
        Ok(())
    }

    pub fn upload_layer_image(&mut self, layer_id: &str, bytes: &[u8]) -> Result<(), String> {
        let img = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
        let rgba = img.to_rgba8();
        let width = rgba.width();
        let height = rgba.height();
        let key = Self::layer_pixel_buffer_key(layer_id);
        self.paint_buffers.insert(key.clone(), rgba.into_raw());
        let image = image_from_rgba(width, height, self.paint_buffers.get(&key).unwrap().clone());
        self.images.insert(key, image);
        Ok(())
    }

    pub fn upload_raster_image_key(&mut self, key: &str, bytes: &[u8]) -> Result<(), String> {
        let img = image::load_from_memory(bytes).map_err(|e| e.to_string())?;
        let rgba = img.to_rgba8();
        let width = rgba.width();
        let height = rgba.height();
        self.paint_buffers.insert(key.to_string(), rgba.into_raw());
        let image = image_from_rgba(width, height, self.paint_buffers.get(key).unwrap().clone());
        self.images.insert(key.to_string(), image);
        Ok(())
    }

    pub fn set_active_tool(&mut self, tool: &str) {
        self.active_tool = tool.to_string();
    }

    pub fn set_hovered_id(&mut self, id: Option<String>) {
        self.hovered_id = id;
    }

    pub fn set_selection_ids_json(&mut self, json: &str) -> Result<(), String> {
        let ids: Vec<String> = serde_json::from_str(json).map_err(|e| e.to_string())?;
        self.selected_ids = ids;
        Ok(())
    }

    pub fn camera_json(&self) -> String {
        serde_json::json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }).to_string()
    }

    fn layer_image(&mut self, id: &str, width: u32, height: u32, image_key: &Option<String>) -> Arc<ImageData> {
        let key = image_key.clone().unwrap_or_else(|| Self::layer_pixel_buffer_key(id));
        if let Some(img) = self.images.get(&key) {
            return img;
        }
        if let Some(buf) = self.paint_buffers.get(&key).cloned() {
            let image = image_from_rgba(width, height, buf);
            return self.images.insert(key, image);
        }
        let rgba = checkerboard_rgba(width, height, false);
        self.paint_buffers.insert(key.clone(), rgba.clone());
        self.images.insert(key, image_from_rgba(width, height, rgba))
    }

    fn append_layer_node(&mut self, scene: &mut Scene, cam: Affine, node: &LayerNode, isolated_id: Option<&str>) {
        match node {
            LayerNode::Pixel {
                id,
                visible,
                opacity,
                blend,
                transform,
                width,
                height,
                image_key,
                mask,
            } => {
                if !visible {
                    return;
                }
                if let Some(iso) = isolated_id {
                    if iso != id {
                        return;
                    }
                }
                let img = self.layer_image(id, *width, *height, image_key);
                let world = cam * (*transform) * Affine::translate(Vec2::new(-(*width as f64) * 0.5, -(*height as f64) * 0.5));
                let clip = Rect::new(0.0, 0.0, *width as f64, *height as f64);
                scene.push_layer(Fill::NonZero, *blend, *opacity, world, &clip);
                if let Some(mask_state) = mask {
                    if mask_state.enabled {
                        let mask_key = format!("mask:{id}");
                        let mut mask_rgba = self
                            .mask_buffers
                            .entry(mask_key.clone())
                            .or_insert_with(|| vec![255u8; (mask_state.width * mask_state.height * 4) as usize])
                            .clone();
                        if mask_state.invert {
                            for a in mask_rgba.chunks_exact_mut(4) {
                                a[3] = 255 - a[3];
                            }
                        }
                        let mask_img = self.images.insert(mask_key, image_from_rgba(mask_state.width, mask_state.height, mask_rgba));
                        cavas::raster::draw_image_arc(scene, &mask_img, Affine::IDENTITY);
                    }
                }
                cavas::raster::draw_image_arc(scene, &img, Affine::IDENTITY);
                scene.pop_layer();
                if self.hovered_id.as_deref() == Some(id.as_str()) || self.selected_ids.iter().any(|s| s == id) {
                    let stroke = Rect::new(0.0, 0.0, *width as f64, *height as f64);
                    scene.stroke(
                        &Stroke::new(2.0 / self.camera.zoom.max(0.1)),
                        world,
                        &vello::peniko::Brush::Solid(Color::from_rgba8(80, 160, 255, 220)),
                        None,
                        &stroke,
                    );
                }
            }
            LayerNode::Group {
                id,
                visible,
                opacity,
                blend,
                transform,
                children,
                mask,
                ..
            } => {
                if !visible {
                    return;
                }
                if let Some(iso) = isolated_id {
                    if iso != id {
                        for child in children {
                            self.append_layer_node(scene, cam, child, Some(iso));
                        }
                        return;
                    }
                }
                scene.push_layer(Fill::NonZero, *blend, *opacity, cam * (*transform), &Rect::new(-1e6, -1e6, 1e6, 1e6));
                for child in children {
                    self.append_layer_node(scene, cam, child, isolated_id);
                }
                scene.pop_layer();
                let _ = mask;
            }
            LayerNode::Adjustment {
                visible,
                opacity,
                blend,
                kind,
                params,
                ..
            } => {
                if !visible || isolated_id.is_some() {
                    return;
                }
                if kind == "brightnessContrast" {
                    let b = params.brightness.unwrap_or(0.0);
                    let c = params.contrast.unwrap_or(0.0);
                    let _ = (b, c, opacity, blend);
                }
            }
        }
    }

    pub fn build_vector_scene(&mut self) -> Scene {
        self.build_scene_for_layer(None)
    }

    pub fn build_layer_scene(&mut self, layer_id: &str) -> Scene {
        self.build_scene_for_layer(Some(layer_id))
    }

    pub fn build_mask_scene(&mut self, layer_id: &str) -> Scene {
        let mut scene = Scene::new();
        let cam = cavas::camera::camera_content_affine(&self.camera, &self.viewport);
        let key = format!("mask:{layer_id}");
        let rgba = self
            .mask_buffers
            .entry(key.clone())
            .or_insert_with(|| vec![255u8; 512 * 512 * 4])
            .clone();
        let img = self.images.insert(key, image_from_rgba(512, 512, rgba));
        cavas::raster::draw_image_arc(&mut scene, &img, cam);
        scene
    }

    fn build_scene_for_layer(&mut self, isolated: Option<&str>) -> Scene {
        let mut scene = Scene::new();
        let cam = cavas::camera::camera_content_affine(&self.camera, &self.viewport);
        for layer in self.document.layers.clone() {
            self.append_layer_node(&mut scene, cam, &layer, isolated);
        }
        scene
    }

    pub fn build_render_scene(&mut self) -> Scene {
        let inner = self.build_vector_scene();
        cavas::render::scale_scene_for_device_pixel_ratio(inner, self.viewport.dpr)
    }
}

impl cavas::canvas_content::CanvasContent for RasterHost {
    fn build_scene(&self) -> Scene {
        Scene::new()
    }

    fn clear_color(&self) -> Color {
        self.theme_clear
    }
}
// #endregion 🔖Host

// #region 🔖WasmSession
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::future_to_promise;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

#[cfg(target_arch = "wasm32")]
struct RasterSessionInner {
    host: RasterHost,
    gpu: cavas::gpu_session::CanvasGpuSession,
    isolated_view: Option<String>,
    view_mode: String,
}

#[cfg(target_arch = "wasm32")]
impl RasterSessionInner {
    fn set_logical_size(&mut self, lw: u32, lh: u32, dpr: f64, pw: u32, ph: u32) {
        self.host.set_size(lw, lh, dpr);
        self.gpu.resize_surface(pw, ph);
    }

    fn render_frame_gpu(&mut self) -> Result<(), JsValue> {
        let scene = match self.view_mode.as_str() {
            "layer" => {
                let id = self.isolated_view.clone().unwrap_or_default();
                cavas::render::scale_scene_for_device_pixel_ratio(self.host.build_layer_scene(&id), self.host.viewport.dpr)
            }
            "mask" => {
                let id = self.isolated_view.clone().unwrap_or_default();
                cavas::render::scale_scene_for_device_pixel_ratio(self.host.build_mask_scene(&id), self.host.viewport.dpr)
            }
            _ => self.host.build_render_scene(),
        };
        self.gpu
            .render_frame(&scene, cavas::canvas_content::CanvasContent::clear_color(&self.host))
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct RasterSession {
    state: Rc<RefCell<RasterSessionInner>>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl RasterSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(RasterSessionInner {
                host: RasterHost::new(),
                gpu: cavas::gpu_session::CanvasGpuSession::default(),
                isolated_view: None,
                view_mode: "composite".into(),
            })),
        }
    }

    #[wasm_bindgen(js_name = gpuReady)]
    pub fn gpu_ready(&self) -> bool {
        self.state.borrow().gpu.gpu_ready()
    }

    #[wasm_bindgen(js_name = attachCanvas)]
    pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> js_sys::Promise {
        let inner = self.state.clone();
        let lw = logical_w.max(1);
        let lh = logical_h.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        if inner.borrow().gpu.gpu_ready() {
            inner.borrow_mut().set_logical_size(lw, lh, dpr, pw, ph);
            return future_to_promise(async move { Ok(JsValue::UNDEFINED) });
        }
        let canvas = canvas.clone();
        future_to_promise(async move {
            let (render_ctx, renderer, surface) =
                cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|e| JsValue::from_str(&e))?;
            let mut g = inner.borrow_mut();
            if g.gpu.gpu_ready() {
                g.set_logical_size(lw, lh, dpr, pw, ph);
                return Ok(JsValue::UNDEFINED);
            }
            g.set_logical_size(lw, lh, dpr, pw, ph);
            g.gpu.finish_attach(canvas, render_ctx, renderer, surface);
            Ok(JsValue::UNDEFINED)
        })
    }

    #[wasm_bindgen(js_name = setSize)]
    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        let lw = width.max(1);
        let lh = height.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        self.state.borrow_mut().set_logical_size(lw, lh, dpr, pw, ph);
    }

    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&mut self) {
        let _ = self.state.borrow_mut().render_frame_gpu();
    }

    #[wasm_bindgen(js_name = setCamera)]
    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.state.borrow_mut().host.set_camera(x, y, zoom);
    }

    #[wasm_bindgen(js_name = wheelScreen)]
    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        self.state.borrow_mut().host.wheel_screen(sx, sy, delta_y);
    }

    #[wasm_bindgen(js_name = pointerDownScreen)]
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8) {
        self.state.borrow_mut().host.pointer_down_screen(sx, sy, button);
    }

    #[wasm_bindgen(js_name = pointerMoveScreen)]
    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64) {
        self.state.borrow_mut().host.pointer_move_screen(sx, sy);
    }

    #[wasm_bindgen(js_name = pointerUpScreen)]
    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64) {
        self.state.borrow_mut().host.pointer_up_screen(sx, sy);
    }

    #[wasm_bindgen(js_name = syncDocumentJson)]
    pub fn sync_document_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.sync_document_json(json).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = uploadLayerImage)]
    pub fn upload_layer_image(&mut self, layer_id: &str, bytes: &[u8]) -> Result<(), JsValue> {
        self.state
            .borrow_mut()
            .host
            .upload_layer_image(layer_id, bytes)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = uploadRasterImageKey)]
    pub fn upload_raster_image_key(&mut self, key: &str, bytes: &[u8]) -> Result<(), JsValue> {
        self.state
            .borrow_mut()
            .host
            .upload_raster_image_key(key, bytes)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = setActiveTool)]
    pub fn set_active_tool(&mut self, tool: &str) {
        self.state.borrow_mut().host.set_active_tool(tool);
    }

    #[wasm_bindgen(js_name = setHoveredIdSilent)]
    pub fn set_hovered_id_silent(&mut self, id: Option<String>) {
        self.state.borrow_mut().host.set_hovered_id(id);
    }

    #[wasm_bindgen(js_name = setSelectionIdsJson)]
    pub fn set_selection_ids_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state
            .borrow_mut()
            .host
            .set_selection_ids_json(json)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = cameraJson)]
    pub fn camera_json(&self) -> String {
        self.state.borrow().host.camera_json()
    }

    #[wasm_bindgen(js_name = setViewMode)]
    pub fn set_view_mode(&mut self, mode: &str, layer_id: Option<String>) {
        let mut g = self.state.borrow_mut();
        g.view_mode = mode.to_string();
        g.isolated_view = layer_id;
    }
}
// #endregion 🔖WasmSession

// #region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_document() {
        let json = r#"{"schema":"raster.document/v1","id":"t","camera":{"x":0,"y":0,"zoom":1},"layers":[]}"#;
        let doc = parse_document(json).expect("parse");
        assert!(doc.layers.is_empty());
    }

    #[test]
    fn blend_mapping() {
        assert!(matches!(blend_from_str("multiply"), Mix::Multiply));
    }

    #[test]
    fn parse_play_fixtures() {
        let json = include_str!("../fixture/semio.raster.json");
        let doc = parse_document(json).expect("parse semio fixture");
        assert!(!doc.layers.is_empty(), "semio should have layers");
    }
}
// #endregion 🧪Tests
