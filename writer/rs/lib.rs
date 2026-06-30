//! ✍️ Infinite-canvas code editor engine on Vello/WebGPU.

pub use infinite_cavas::{self as cavas, *};
use cavas::camera::{screen_to_world, world_to_screen, Camera, Viewport};
use cavas::text as canvas_text;
use serde::Deserialize;
use vello::kurbo::{Affine, Point, Rect};
use vello::peniko::Color;
use vello::Scene;

// #region 🔖EditorState
const LINE_HEIGHT: f64 = 22.0;
const GUTTER_WIDTH: f64 = 56.0;
const PAD_X: f64 = 12.0;
const FONT_PX: f64 = 14.0;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SemanticTokenJson {
    start: usize,
    end: usize,
    class: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DiagnosticJson {
    start: usize,
    end: usize,
    severity: Option<String>,
    #[allow(dead_code)]
    message: String,
}

#[derive(Clone, Debug, Deserialize)]
struct TextEditJson {
    range: TextRangeJson,
    #[serde(rename = "newText")]
    new_text: String,
}

#[derive(Clone, Debug, Deserialize)]
struct TextRangeJson {
    start: TextPosJson,
    end: TextPosJson,
}

#[derive(Clone, Debug, Deserialize)]
struct TextPosJson {
    line: u32,
    character: u32,
}

pub struct WriterHost {
    text: String,
    caret: usize,
    anchor: usize,
    camera: Camera,
    viewport: Viewport,
    semantic_tokens: Vec<SemanticTokenJson>,
    diagnostics: Vec<DiagnosticJson>,
    panning: bool,
    pan_last: Option<Point>,
}

impl Default for WriterHost {
    fn default() -> Self {
        Self::new()
    }
}

impl WriterHost {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            caret: 0,
            anchor: 0,
            camera: Camera { x: 0.0, y: 0.0, zoom: 1.0 },
            viewport: Viewport { width: 800, height: 600, dpr: 1.0 },
            semantic_tokens: Vec::new(),
            diagnostics: Vec::new(),
            panning: false,
            pan_last: None,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.caret = self.caret.min(self.text.len());
        self.anchor = self.anchor.min(self.text.len());
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn caret(&self) -> usize {
        self.caret
    }

    pub fn set_semantic_tokens_json(&mut self, json: &str) {
        self.semantic_tokens = serde_json::from_str(json).unwrap_or_default();
    }

    pub fn set_diagnostics_json(&mut self, json: &str) {
        self.diagnostics = serde_json::from_str(json).unwrap_or_default();
    }

    pub fn apply_text_edits_json(&mut self, json: &str) {
        let edits: Vec<TextEditJson> = serde_json::from_str(json).unwrap_or_default();
        let mut text = self.text.clone();
        let mut sorted = edits;
        sorted.sort_by_key(|edit| std::cmp::Reverse(position_to_offset(&text, &edit.range.start)));
        for edit in sorted {
            let start = position_to_offset(&text, &edit.range.start);
            let end = position_to_offset(&text, &edit.range.end).max(start);
            text.replace_range(start..end, &edit.new_text);
        }
        self.set_text(text);
    }

    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.camera.x = x;
        self.camera.y = y;
        self.camera.zoom = zoom.max(0.1);
    }

    pub fn camera_json(&self) -> String {
        serde_json::json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }).to_string()
    }

    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        self.viewport.width = width.max(1);
        self.viewport.height = height.max(1);
        self.viewport.dpr = dpr.max(1.0);
    }

    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        cavas::camera::wheel_screen(&mut self.camera, &self.viewport, sx, sy, delta_y);
    }

    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: i32) {
        if button == 1 {
            self.panning = true;
            self.pan_last = Some(Point::new(sx, sy));
            return;
        }
        if button == 0 {
            let world = screen_to_world(&self.camera, &self.viewport, Point::new(sx, sy));
            let offset = self.hit_test_offset(world);
            self.caret = offset;
            self.anchor = offset;
        }
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, buttons: i32) {
        if self.panning || buttons == 4 {
            if let Some(last) = self.pan_last {
                let dx = (sx - last.x) / self.camera.zoom;
                let dy = (sy - last.y) / self.camera.zoom;
                self.camera.x -= dx;
                self.camera.y -= dy;
            }
            self.pan_last = Some(Point::new(sx, sy));
            return;
        }
        if buttons == 1 {
            let world = screen_to_world(&self.camera, &self.viewport, Point::new(sx, sy));
            self.caret = self.hit_test_offset(world);
        }
    }

    pub fn pointer_up_screen(&mut self, _sx: f64, _sy: f64, _button: i32) {
        self.panning = false;
        self.pan_last = None;
    }

    pub fn insert_text(&mut self, chunk: &str) {
        let start = self.caret.min(self.anchor);
        let end = self.caret.max(self.anchor);
        self.text.replace_range(start..end, chunk);
        self.caret = start + chunk.len();
        self.anchor = self.caret;
    }

    pub fn backspace(&mut self) {
        if self.caret != self.anchor {
            self.insert_text("");
            return;
        }
        if self.caret == 0 {
            return;
        }
        let prev = prev_char_boundary(&self.text, self.caret);
        self.text.replace_range(prev..self.caret, "");
        self.caret = prev;
        self.anchor = self.caret;
    }

    pub fn delete_forward(&mut self) {
        if self.caret != self.anchor {
            self.insert_text("");
            return;
        }
        if self.caret >= self.text.len() {
            return;
        }
        let next = next_char_boundary(&self.text, self.caret);
        self.text.replace_range(self.caret..next, "");
        self.anchor = self.caret;
    }

    pub fn move_left(&mut self, extend: bool) {
        let next = if self.caret == 0 { 0 } else { prev_char_boundary(&self.text, self.caret) };
        self.caret = next;
        if !extend {
            self.anchor = self.caret;
        }
    }

    pub fn move_right(&mut self, extend: bool) {
        let next = if self.caret >= self.text.len() {
            self.text.len()
        } else {
            next_char_boundary(&self.text, self.caret)
        };
        self.caret = next;
        if !extend {
            self.anchor = self.caret;
        }
    }

    pub fn move_up(&mut self, extend: bool) {
        let (line, col) = offset_line_col(&self.text, self.caret);
        if line == 0 {
            self.caret = 0;
        } else {
            self.caret = offset_at_line_col(&self.text, line - 1, col);
        }
        if !extend {
            self.anchor = self.caret;
        }
    }

    pub fn move_down(&mut self, extend: bool) {
        let (line, col) = offset_line_col(&self.text, self.caret);
        let max_line = self.text.matches('\n').count();
        self.caret = offset_at_line_col(&self.text, (line + 1).min(max_line), col);
        if !extend {
            self.anchor = self.caret;
        }
    }

    pub fn world_to_screen_json(&self, wx: f64, wy: f64) -> String {
        let p = world_to_screen(&self.camera, &self.viewport, Point::new(wx, wy));
        serde_json::json!({ "x": p.x, "y": p.y }).to_string()
    }

    pub fn caret_world_json(&self) -> String {
        let (x, y) = offset_to_world(self, self.caret);
        serde_json::json!({ "x": x, "y": y }).to_string()
    }

    fn hit_test_offset(&self, world: Point) -> usize {
        let rel_x = world.x - GUTTER_WIDTH;
        let rel_y = world.y;
        if rel_y < 0.0 {
            return 0;
        }
        let line = (rel_y / LINE_HEIGHT).floor().max(0.0) as usize;
        let col = ((rel_x - PAD_X) / (FONT_PX * 0.6)).round().max(0.0) as usize;
        offset_at_line_col(&self.text, line, col)
    }

    pub fn build_scene(&self) -> Scene {
        let mut scene = Scene::new();
        let bg = Color::from_rgba8(18, 18, 20, 255);
        scene.fill(vello::peniko::Fill::NonZero, Affine::IDENTITY, bg, None, &Rect::new(-10_000.0, -10_000.0, 10_000.0, 10_000.0));
        let lines: Vec<&str> = if self.text.is_empty() { vec![""] } else { self.text.split('\n').collect() };
        for (i, line) in lines.iter().enumerate() {
            let y = i as f64 * LINE_HEIGHT + LINE_HEIGHT * 0.75;
            let gutter = format!("{}", i + 1);
            canvas_text::append_label(
                &mut scene,
                &gutter,
                Point::new(PAD_X, y),
                FONT_PX,
                Color::from_rgba8(120, 120, 130, 255),
                bg,
            );
            self.render_colored_line(&mut scene, line, i, y, bg);
        }
        let sel_start = self.caret.min(self.anchor);
        let sel_end = self.caret.max(self.anchor);
        if sel_start != sel_end {
            self.render_selection(&mut scene, sel_start, sel_end);
        }
        self.render_caret(&mut scene, self.caret);
        for diag in &self.diagnostics {
            self.render_diagnostic(&mut scene, diag, bg);
        }
        cavas::render::scale_scene_for_device_pixel_ratio(scene, self.viewport.dpr)
    }

    fn render_colored_line(&self, scene: &mut Scene, line: &str, line_index: usize, y: f64, bg: Color) {
        if line.is_empty() {
            return;
        }
        let line_start = offset_at_line_col(&self.text, line_index, 0);
        let line_end = line_start + line.len();
        let mut cursor = 0usize;
        let mut spans = Vec::new();
        for token in &self.semantic_tokens {
            if token.end <= line_start || token.start >= line_end {
                continue;
            }
            let start = token.start.saturating_sub(line_start);
            let end = token.end.min(line_end).saturating_sub(line_start);
            if start > cursor {
                spans.push((cursor, start, "plain"));
            }
            spans.push((start, end, token.class.as_str()));
            cursor = end;
        }
        if cursor < line.len() {
            spans.push((cursor, line.len(), "plain"));
        }
        if spans.is_empty() {
            canvas_text::append_label(scene, line, Point::new(GUTTER_WIDTH + PAD_X, y), FONT_PX, token_color("plain"), bg);
            return;
        }
        let mut x = GUTTER_WIDTH + PAD_X;
        for (start, end, class) in spans {
            let slice = &line[start..end];
            canvas_text::append_label(scene, slice, Point::new(x, y), FONT_PX, token_color(class), bg);
            let (w, _) = canvas_text::label_extent(slice, FONT_PX);
            x += w;
        }
    }

    fn render_selection(&self, scene: &mut Scene, start: usize, end: usize) {
        let color = Color::from_rgba8(60, 100, 180, 90);
        let (s_line, s_col) = offset_line_col(&self.text, start);
        let (e_line, e_col) = offset_line_col(&self.text, end);
        if s_line == e_line {
            let (x, y) = col_to_world(s_line, s_col);
            let (x2, _) = col_to_world(s_line, e_col);
            let rect = Rect::new(x, y - LINE_HEIGHT * 0.8, x2.max(x + 1.0), y + LINE_HEIGHT * 0.2);
            scene.fill(vello::peniko::Fill::NonZero, Affine::IDENTITY, color, None, &rect);
            return;
        }
        for line in s_line..=e_line {
            let (x, y) = if line == s_line {
                col_to_world(line, s_col)
            } else {
                col_to_world(line, 0)
            };
            let (x2, _) = if line == e_line {
                col_to_world(line, e_col)
            } else {
                let len = self.text.split('\n').nth(line).map(str::len).unwrap_or(0);
                col_to_world(line, len)
            };
            let rect = Rect::new(x, y - LINE_HEIGHT * 0.8, x2.max(x + 1.0), y + LINE_HEIGHT * 0.2);
            scene.fill(vello::peniko::Fill::NonZero, Affine::IDENTITY, color, None, &rect);
        }
    }

    fn render_caret(&self, scene: &mut Scene, offset: usize) {
        let (x, y) = offset_to_world(self, offset);
        let rect = Rect::new(x, y - LINE_HEIGHT * 0.8, x + 1.5, y + LINE_HEIGHT * 0.2);
        scene.fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            Color::from_rgba8(240, 240, 245, 255),
            None,
            &rect,
        );
    }

    fn render_diagnostic(&self, scene: &mut Scene, diag: &DiagnosticJson, bg: Color) {
        let (x, y) = offset_to_world(self, diag.start);
        let (x2, _) = offset_to_world(self, diag.end.max(diag.start + 1));
        let color = match diag.severity.as_deref() {
            Some("warning") => Color::from_rgba8(220, 180, 40, 255),
            _ => Color::from_rgba8(220, 70, 70, 255),
        };
        let rect = Rect::new(x, y + 2.0, x2.max(x + 8.0), y + 4.0);
        scene.fill(vello::peniko::Fill::NonZero, Affine::IDENTITY, color, None, &rect);
        let _ = bg;
    }
}

fn token_color(class: &str) -> Color {
    match class {
        "keyword" => Color::from_rgba8(120, 170, 255, 255),
        "string" => Color::from_rgba8(180, 220, 140, 255),
        "number" => Color::from_rgba8(220, 160, 120, 255),
        "operator" | "punctuation" => Color::from_rgba8(180, 180, 190, 255),
        "error" => Color::from_rgba8(255, 120, 120, 255),
        _ => Color::from_rgba8(230, 230, 235, 255),
    }
}

fn offset_line_col(text: &str, offset: usize) -> (usize, usize) {
    let clamped = offset.min(text.len());
    let mut line = 0usize;
    let mut last = 0usize;
    for (i, ch) in text.char_indices() {
        if i >= clamped {
            break;
        }
        if ch == '\n' {
            line += 1;
            last = i + 1;
        }
    }
    (line, clamped - last)
}

fn offset_at_line_col(text: &str, line: usize, col: usize) -> usize {
    let mut current_line = 0usize;
    let mut line_start = 0usize;
    for (i, ch) in text.char_indices() {
        if current_line == line {
            let line_end = text[line_start..]
                .find('\n')
                .map(|idx| line_start + idx)
                .unwrap_or(text.len());
            return line_start + col.min(line_end.saturating_sub(line_start));
        }
        if ch == '\n' {
            current_line += 1;
            line_start = i + 1;
        }
    }
    if current_line == line {
        return line_start + col.min(text.len().saturating_sub(line_start));
    }
    text.len()
}

fn offset_to_world(host: &WriterHost, offset: usize) -> (f64, f64) {
    let (line, col) = offset_line_col(&host.text, offset);
    col_to_world(line, col)
}

fn col_to_world(line: usize, col: usize) -> (f64, f64) {
    let x = GUTTER_WIDTH + PAD_X + col as f64 * FONT_PX * 0.6;
    let y = line as f64 * LINE_HEIGHT + LINE_HEIGHT * 0.75;
    (x, y)
}

fn position_to_offset(text: &str, pos: &TextPosJson) -> usize {
    offset_at_line_col(text, pos.line as usize, pos.character as usize)
}

fn prev_char_boundary(text: &str, index: usize) -> usize {
    text[..index].char_indices().next_back().map(|(i, _)| i).unwrap_or(0)
}

fn next_char_boundary(text: &str, index: usize) -> usize {
    text[index..].char_indices().nth(1).map(|(i, _)| index + i).unwrap_or(text.len())
}
// #endregion 🔖EditorState

// #region 🔖Wasm
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
struct WriterSessionInner {
    host: WriterHost,
    gpu: cavas::gpu_session::CanvasGpuSession,
}

#[cfg(target_arch = "wasm32")]
impl WriterSessionInner {
    fn set_logical_size(&mut self, lw: u32, lh: u32, dpr: f64, pw: u32, ph: u32) {
        self.host.set_size(lw, lh, dpr);
        self.gpu.resize_surface(pw, ph);
    }

    fn render_frame_gpu(&mut self) -> Result<(), JsValue> {
        let scene = self.host.build_scene();
        self.gpu
            .render_frame(&scene, Color::from_rgba8(18, 18, 20, 255))
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WriterSession {
    state: Rc<RefCell<WriterSessionInner>>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WriterSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(WriterSessionInner {
                host: WriterHost::new(),
                gpu: cavas::gpu_session::CanvasGpuSession::default(),
            })),
        }
    }

    #[wasm_bindgen(js_name = gpuReady)]
    pub fn gpu_ready(&self) -> bool {
        self.state.borrow().gpu.gpu_ready()
    }

    #[wasm_bindgen(js_name = detachGpu)]
    pub fn detach_gpu(&mut self) {
        self.state.borrow_mut().gpu.detach();
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

    #[wasm_bindgen(js_name = setText)]
    pub fn set_text(&mut self, text: String) {
        self.state.borrow_mut().host.set_text(text);
    }

    #[wasm_bindgen(js_name = text)]
    pub fn text(&self) -> String {
        self.state.borrow().host.text().to_string()
    }

    #[wasm_bindgen(js_name = caret)]
    pub fn caret(&self) -> usize {
        self.state.borrow().host.caret()
    }

    #[wasm_bindgen(js_name = setSemanticTokensJson)]
    pub fn set_semantic_tokens_json(&mut self, json: &str) {
        self.state.borrow_mut().host.set_semantic_tokens_json(json);
    }

    #[wasm_bindgen(js_name = setDiagnosticsJson)]
    pub fn set_diagnostics_json(&mut self, json: &str) {
        self.state.borrow_mut().host.set_diagnostics_json(json);
    }

    #[wasm_bindgen(js_name = applyTextEditsJson)]
    pub fn apply_text_edits_json(&mut self, json: &str) {
        self.state.borrow_mut().host.apply_text_edits_json(json);
    }

    #[wasm_bindgen(js_name = setCamera)]
    pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
        self.state.borrow_mut().host.set_camera(x, y, zoom);
    }

    #[wasm_bindgen(js_name = cameraJson)]
    pub fn camera_json(&self) -> String {
        self.state.borrow().host.camera_json()
    }

    #[wasm_bindgen(js_name = wheelScreen)]
    pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
        self.state.borrow_mut().host.wheel_screen(sx, sy, delta_y);
    }

    #[wasm_bindgen(js_name = pointerDownScreen)]
    pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: i32) {
        self.state.borrow_mut().host.pointer_down_screen(sx, sy, button);
    }

    #[wasm_bindgen(js_name = pointerMoveScreen)]
    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, buttons: i32) {
        self.state.borrow_mut().host.pointer_move_screen(sx, sy, buttons);
    }

    #[wasm_bindgen(js_name = pointerUpScreen)]
    pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, button: i32) {
        self.state.borrow_mut().host.pointer_up_screen(sx, sy, button);
    }

    #[wasm_bindgen(js_name = insertText)]
    pub fn insert_text(&mut self, chunk: &str) {
        self.state.borrow_mut().host.insert_text(chunk);
    }

    #[wasm_bindgen(js_name = backspace)]
    pub fn backspace(&mut self) {
        self.state.borrow_mut().host.backspace();
    }

    #[wasm_bindgen(js_name = deleteForward)]
    pub fn delete_forward(&mut self) {
        self.state.borrow_mut().host.delete_forward();
    }

    #[wasm_bindgen(js_name = moveLeft)]
    pub fn move_left(&mut self, extend: bool) {
        self.state.borrow_mut().host.move_left(extend);
    }

    #[wasm_bindgen(js_name = moveRight)]
    pub fn move_right(&mut self, extend: bool) {
        self.state.borrow_mut().host.move_right(extend);
    }

    #[wasm_bindgen(js_name = moveUp)]
    pub fn move_up(&mut self, extend: bool) {
        self.state.borrow_mut().host.move_up(extend);
    }

    #[wasm_bindgen(js_name = moveDown)]
    pub fn move_down(&mut self, extend: bool) {
        self.state.borrow_mut().host.move_down(extend);
    }

    #[wasm_bindgen(js_name = caretWorldJson)]
    pub fn caret_world_json(&self) -> String {
        self.state.borrow().host.caret_world_json()
    }

    #[wasm_bindgen(js_name = worldToScreenJson)]
    pub fn world_to_screen_json(&self, wx: f64, wy: f64) -> String {
        self.state.borrow().host.world_to_screen_json(wx, wy)
    }
}
// #endregion 🔖Wasm

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_caret() {
        let mut host = WriterHost::new();
        host.insert_text("MATCH");
        assert_eq!(host.text(), "MATCH");
        assert_eq!(host.caret(), 5);
    }

    #[test]
    fn build_scene_has_content() {
        let mut host = WriterHost::new();
        host.set_text("MATCH (a:Piece)\nRETURN a.name".into());
        let scene = host.build_scene();
        assert!(!scene.encoding().path_tags.is_empty());
    }

    #[test]
    fn apply_text_edits() {
        let mut host = WriterHost::new();
        host.set_text("abc def".into());
        host.apply_text_edits_json(r#"[{"range":{"start":{"line":0,"character":4},"end":{"line":0,"character":7}},"newText":"xyz"}]"#);
        assert_eq!(host.text(), "abc xyz");
    }
}
// #endregion 🔖Tests
