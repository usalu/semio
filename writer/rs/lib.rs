//! ✍️ Infinite-canvas code editor engine on Vello/WebGPU.

pub use infinite_cavas::{self as cavas, *};
use cavas::camera::{camera_content_affine, screen_to_world, world_to_screen, Camera, Viewport};
use cavas::text as canvas_text;
use serde::Deserialize;
use vello::kurbo::{Affine, Point, Rect};
use vello::peniko::Color;
use vello::Scene;

// #region 🔖Theme
#[derive(Clone, Copy, Debug)]
struct WriterVelloTheme {
    raster_clear: Color,
    grid_minor_stroke: Color,
    label_fill: Color,
    label_fill_hovered: Color,
    label_halo: Color,
    hover_fill: Color,
    selection_fill: Color,
}

impl Default for WriterVelloTheme {
    fn default() -> Self {
        Self::from_board(&ui_styling::BOARD_LIGHT)
    }
}

impl WriterVelloTheme {
    fn from_board(t: &ui_styling::BoardTheme) -> Self {
        Self {
            raster_clear: Color::new(t.raster_clear),
            grid_minor_stroke: Color::new(t.grid_minor_stroke),
            label_fill: Color::new(t.label_fill),
            label_fill_hovered: Color::new(t.label_fill_hovered),
            label_halo: Color::new(t.label_halo),
            hover_fill: Color::new(t.node_fill_hovered),
            selection_fill: Color::new(t.node_fill_selected),
        }
    }

    fn color_from_json_rgba8(arr: &[serde_json::Value]) -> Option<Color> {
        let r = u8::try_from(arr.first()?.as_u64().unwrap_or(0).min(255)).ok()?;
        let g = u8::try_from(arr.get(1)?.as_u64().unwrap_or(0).min(255)).ok()?;
        let b = u8::try_from(arr.get(2)?.as_u64().unwrap_or(0).min(255)).ok()?;
        let a = u8::try_from(arr.get(3).and_then(|x| x.as_u64()).unwrap_or(255).min(255)).ok()?;
        Some(Color::from_rgba8(r, g, b, a))
    }

    fn merge_color_field(next: &mut Color, v: &serde_json::Value, key: &str) {
        if let Some(arr) = v.get(key).and_then(|x| x.as_array()) {
            if let Some(c) = Self::color_from_json_rgba8(arr) {
                *next = c;
            }
        }
    }

    fn merge_from_json(&mut self, json: &str) -> Result<(), String> {
        let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let mut next = *self;
        Self::merge_color_field(&mut next.raster_clear, &v, "rasterClear");
        Self::merge_color_field(&mut next.grid_minor_stroke, &v, "gridMinorStroke");
        Self::merge_color_field(&mut next.label_fill, &v, "labelFill");
        Self::merge_color_field(&mut next.label_fill_hovered, &v, "labelFillHovered");
        Self::merge_color_field(&mut next.label_halo, &v, "labelHalo");
        Self::merge_color_field(&mut next.hover_fill, &v, "nodeFillHovered");
        Self::merge_color_field(&mut next.selection_fill, &v, "nodeFillSelected");
        *self = next;
        Ok(())
    }
}
// #endregion 🔖Theme

// #region 🔖EditorState
const LINE_HEIGHT: f64 = 22.0;
const GUTTER_WIDTH: f64 = 56.0;
const PAD_X: f64 = 12.0;
const LINE_ORIGIN_X: f64 = GUTTER_WIDTH + PAD_X;
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
struct SelectableSpanJson {
    start: usize,
    end: usize,
    kind: String,
    head_end: Option<usize>,
    tail_start: Option<usize>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ByteRangeJson {
    start: usize,
    end: usize,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PlaceholderJson {
    offset: usize,
    label: String,
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
    selectable_spans: Vec<SelectableSpanJson>,
    diagnostics: Vec<DiagnosticJson>,
    placeholders: Vec<PlaceholderJson>,
    hover_occurrences: Vec<ByteRangeJson>,
    selection_occurrences: Vec<ByteRangeJson>,
    extra_carets: Vec<usize>,
    panning: bool,
    pan_last: Option<Point>,
    drag_selecting: bool,
    hover_token_start: Option<usize>,
    hover_token_end: Option<usize>,
    theme: WriterVelloTheme,
    caret_visible: bool,
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
            selectable_spans: Vec::new(),
            diagnostics: Vec::new(),
            placeholders: Vec::new(),
            hover_occurrences: Vec::new(),
            selection_occurrences: Vec::new(),
            extra_carets: Vec::new(),
            panning: false,
            pan_last: None,
            drag_selecting: false,
            hover_token_start: None,
            hover_token_end: None,
            theme: WriterVelloTheme::default(),
            caret_visible: true,
        }
    }

    pub fn set_vello_theme_from_json(&mut self, json: &str) -> Result<(), String> {
        self.theme.merge_from_json(json)
    }

    pub fn set_caret_visible(&mut self, visible: bool) {
        self.caret_visible = visible;
    }

    pub fn anchor(&self) -> usize {
        self.anchor
    }

    #[cfg(test)]
    fn set_caret_anchor(&mut self, offset: usize) {
        self.caret = offset;
        self.anchor = offset;
    }

    #[cfg(test)]
    fn set_selection(&mut self, anchor: usize, caret: usize) {
        self.set_selection_range(anchor, caret);
    }

    pub fn set_selection_range(&mut self, anchor: usize, caret: usize) {
        self.anchor = anchor.min(self.text.len());
        self.caret = caret.min(self.text.len());
        if self.caret != self.anchor {
            let (start, end) = self.normalize_edit_range(self.caret.min(self.anchor), self.caret.max(self.anchor));
            self.anchor = start;
            self.caret = end;
        }
    }

    pub fn hover_token_range(&self) -> Option<(usize, usize)> {
        match (self.hover_token_start, self.hover_token_end) {
            (Some(start), Some(end)) => Some((start, end)),
            _ => None,
        }
    }

    pub fn set_hover_range(&mut self, start: Option<usize>, end: Option<usize>) {
        self.hover_token_start = start;
        self.hover_token_end = end;
    }

    pub fn select_all(&mut self) {
        self.anchor = 0;
        self.caret = self.text.len();
    }

    pub fn set_text(&mut self, text: String) {
        if self.text == text {
            return;
        }
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

    pub fn set_selectable_spans_json(&mut self, json: &str) {
        self.selectable_spans = serde_json::from_str(json).unwrap_or_default();
    }

    pub fn select_span_at(&mut self, offset: usize) {
        let offset = offset.min(self.text.len());
        if let Some(ch) = self.text[offset..].chars().next() {
            if ch == ':' || ch == '.' {
                for span in &self.selectable_spans {
                    if span.kind != "atomic" && offset >= span.start && offset < span.end {
                        self.anchor = span.start;
                        self.caret = span.end;
                        return;
                    }
                }
            }
        }
        let probe = if offset > 0 && (!self.text.is_char_boundary(offset) || offset == self.text.len()) {
            prev_char_boundary(&self.text, offset)
        } else {
            offset
        };
        let mut best: Option<&SelectableSpanJson> = None;
        for span in &self.selectable_spans {
            if span.kind != "atomic" || probe < span.start || probe >= span.end {
                continue;
            }
            let size = span.end - span.start;
            if best.map(|current| size < current.end - current.start).unwrap_or(true) {
                best = Some(span);
            }
        }
        if let Some(span) = best {
            self.anchor = span.start;
            self.caret = span.end;
            return;
        }
        let snapped = self.snap_offset_for_atomic(offset);
        self.anchor = snapped;
        self.caret = snapped;
    }

    pub fn selection_text(&self) -> String {
        let (start, end) = self.selection_range();
        if start >= end {
            return String::new();
        }
        self.text[start..end].to_string()
    }

    pub fn replace_selection(&mut self, next: &str) {
        let (start, end) = self.selection_range();
        if start >= end {
            self.insert_text(next);
            return;
        }
        self.text.replace_range(start..end, next);
        self.caret = start + next.len();
        self.anchor = self.caret;
    }

    pub fn set_diagnostics_json(&mut self, json: &str) {
        self.diagnostics = serde_json::from_str(json).unwrap_or_default();
    }

    pub fn set_placeholders_json(&mut self, json: &str) {
        self.placeholders = serde_json::from_str(json).unwrap_or_default();
    }

    pub fn set_hover_occurrences_json(&mut self, json: &str) {
        self.hover_occurrences = serde_json::from_str(json).unwrap_or_default();
    }

    pub fn set_selection_occurrences_json(&mut self, json: &str) {
        self.selection_occurrences = serde_json::from_str(json).unwrap_or_default();
    }

    pub fn set_extra_carets_json(&mut self, json: &str) {
        self.extra_carets = serde_json::from_str(json).unwrap_or_default();
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
            self.drag_selecting = false;
            self.pan_last = Some(Point::new(sx, sy));
            return;
        }
        if button == 0 {
            self.drag_selecting = true;
            let world = screen_to_world(&self.camera, &self.viewport, Point::new(sx, sy));
            let offset = self.snap_offset_for_atomic(self.hit_test_offset(world));
            self.caret = offset;
            self.anchor = offset;
            self.set_hover_at_offset(offset);
        }
    }

    pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, buttons: i32) {
        if self.panning || (buttons & 4) != 0 {
            if let Some(last) = self.pan_last {
                let dx = (sx - last.x) / self.camera.zoom;
                let dy = (sy - last.y) / self.camera.zoom;
                self.camera.x -= dx;
                self.camera.y -= dy;
            }
            self.pan_last = Some(Point::new(sx, sy));
            return;
        }
        if self.drag_selecting {
            let world = screen_to_world(&self.camera, &self.viewport, Point::new(sx, sy));
            self.caret = self.snap_offset_for_atomic(self.hit_test_offset(world));
            return;
        }
        let world = screen_to_world(&self.camera, &self.viewport, Point::new(sx, sy));
        self.set_hover_at_offset(self.hit_test_offset(world));
    }

    pub fn pointer_up_screen(&mut self, _sx: f64, _sy: f64, button: i32) {
        if button == 0 {
            self.drag_selecting = false;
            if self.caret != self.anchor {
                let (start, end) = self.normalize_edit_range(self.caret.min(self.anchor), self.caret.max(self.anchor));
                self.anchor = start;
                self.caret = end;
            }
        }
        self.panning = false;
        self.pan_last = None;
    }

    pub fn insert_text(&mut self, chunk: &str) {
        let mut start = self.caret.min(self.anchor);
        let mut end = self.caret.max(self.anchor);
        let collapsed = start == end;
        let mut insert = chunk.to_string();
        if collapsed && self.should_prefix_auto_space(start, &insert) {
            insert = format!(" {insert}");
        }
        if collapsed && is_insert_whitespace(&insert) {
            self.text.insert_str(start, &insert);
            self.caret = start + insert.len();
            self.anchor = self.caret;
            return;
        }
        if collapsed {
            for token in &self.semantic_tokens {
                if start > token.start && start < token.end {
                    start = token.start;
                    end = token.end;
                    break;
                }
            }
        }
        let (start, end) = self.normalize_edit_range(start, end);
        self.text.replace_range(start..end, &insert);
        self.caret = start + insert.len();
        self.anchor = self.caret;
    }

    fn should_prefix_auto_space(&self, offset: usize, chunk: &str) -> bool {
        let first = match chunk.chars().next() {
            Some(ch) => ch,
            None => return false,
        };
        if is_insert_whitespace(chunk) {
            return false;
        }
        if matches!(first, ':' | '.' | ',' | ')' | ']' | '-' | '!' | '=') {
            return false;
        }
        if self.token_ending_at(offset).is_none() {
            return false;
        }
        if offset < self.text.len() {
            let next = self.text[offset..].chars().next().unwrap_or(' ');
            if next.is_whitespace() {
                return false;
            }
        }
        true
    }

    fn token_ending_at(&self, offset: usize) -> Option<&SemanticTokenJson> {
        self.semantic_tokens
            .iter()
            .find(|token| token.end == offset && token.start < offset)
    }

    pub fn backspace(&mut self) {
        if self.caret != self.anchor {
            let (start, end) = self.normalize_edit_range(self.caret.min(self.anchor), self.caret.max(self.anchor));
            self.caret = start;
            self.anchor = end;
            self.insert_text("");
            return;
        }
        if self.caret == 0 {
            return;
        }
        for token in &self.semantic_tokens {
            if self.caret > token.start && self.caret <= token.end {
                self.text.replace_range(token.start..token.end, "");
                self.caret = token.start;
                self.anchor = self.caret;
                return;
            }
        }
        let prev = prev_char_boundary(&self.text, self.caret);
        self.text.replace_range(prev..self.caret, "");
        self.caret = prev;
        self.anchor = self.caret;
    }

    pub fn delete_forward(&mut self) {
        if self.caret != self.anchor {
            let (start, end) = self.normalize_edit_range(self.caret.min(self.anchor), self.caret.max(self.anchor));
            self.caret = start;
            self.anchor = end;
            self.insert_text("");
            return;
        }
        if self.caret >= self.text.len() {
            return;
        }
        for token in &self.semantic_tokens {
            if self.caret >= token.start && self.caret < token.end {
                self.text.replace_range(token.start..token.end, "");
                self.anchor = self.caret;
                return;
            }
        }
        let next = next_char_boundary(&self.text, self.caret);
        self.text.replace_range(self.caret..next, "");
        self.anchor = self.caret;
    }

    pub fn move_line_start(&mut self, extend: bool) {
        let (line, _) = offset_line_col(&self.text, self.caret);
        self.caret = offset_at_line_col(&self.text, line, 0);
        if !extend {
            self.anchor = self.caret;
        }
    }

    pub fn move_line_end(&mut self, extend: bool) {
        let (line, _) = offset_line_col(&self.text, self.caret);
        let line_len = self.text.split('\n').nth(line).map(str::len).unwrap_or(0);
        self.caret = offset_at_line_col(&self.text, line, line_len);
        if !extend {
            self.anchor = self.caret;
        }
    }

    pub fn move_left(&mut self, extend: bool) {
        let next = self
            .token_left_boundary(self.caret)
            .unwrap_or_else(|| if self.caret == 0 { 0 } else { prev_char_boundary(&self.text, self.caret) });
        self.caret = next;
        if !extend {
            self.anchor = self.caret;
        }
    }

    pub fn move_right(&mut self, extend: bool) {
        let next = self.token_right_boundary(self.caret).unwrap_or_else(|| {
            if self.caret >= self.text.len() {
                self.text.len()
            } else {
                next_char_boundary(&self.text, self.caret)
            }
        });
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

    fn set_hover_at_offset(&mut self, offset: usize) {
        self.hover_token_start = None;
        self.hover_token_end = None;
        if let Some(span) = self.token_span_at_offset(offset) {
            self.hover_token_start = Some(span.0);
            self.hover_token_end = Some(span.1);
        }
    }

    fn token_span_at_offset(&self, offset: usize) -> Option<(usize, usize)> {
        for token in &self.semantic_tokens {
            if offset >= token.start && offset < token.end {
                return Some((token.start, token.end));
            }
        }
        None
    }

    fn snap_offset_for_atomic(&self, offset: usize) -> usize {
        for token in &self.semantic_tokens {
            if offset > token.start && offset < token.end {
                let mid = token.start + (token.end - token.start) / 2;
                return if offset < mid { token.start } else { token.end };
            }
        }
        offset
    }

    fn normalize_edit_range(&self, start: usize, end: usize) -> (usize, usize) {
        let (mut s, mut e) = if start <= end { (start, end) } else { (end, start) };
        loop {
            let mut changed = false;
            for token in &self.semantic_tokens {
                if s < token.end && e > token.start && (s > token.start || e < token.end) {
                    if s > token.start {
                        s = token.start;
                        changed = true;
                    }
                    if e < token.end {
                        e = token.end;
                        changed = true;
                    }
                }
            }
            for span in &self.selectable_spans {
                if span.kind == "atomic" || !ranges_overlap(s, e, span.start, span.end) {
                    continue;
                }
                if self.allowed_composite_selection(s, e, span) {
                    continue;
                }
                if s > span.start {
                    s = span.start;
                    changed = true;
                }
                if e < span.end {
                    e = span.end;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        if start <= end { (s, e) } else { (e, s) }
    }

    fn allowed_composite_selection(&self, start: usize, end: usize, span: &SelectableSpanJson) -> bool {
        if start == span.start && end == span.end {
            return true;
        }
        match span.kind.as_str() {
            "varLabel" => {
                let head_end = span.head_end.unwrap_or(span.end);
                start == span.start && end == head_end
            }
            "propertyAccess" => {
                let head_end = span.head_end.unwrap_or(span.start);
                let tail_start = span.tail_start.unwrap_or(span.end);
                (start == span.start && end == head_end) || (start == tail_start && end == span.end)
            }
            _ => false,
        }
    }

    fn token_left_boundary(&self, offset: usize) -> Option<usize> {
        for token in &self.semantic_tokens {
            if offset > token.start && offset <= token.end {
                return Some(token.start);
            }
        }
        None
    }

    fn token_right_boundary(&self, offset: usize) -> Option<usize> {
        for token in &self.semantic_tokens {
            if offset >= token.start && offset < token.end {
                return Some(token.end);
            }
        }
        None
    }

    fn selection_range(&self) -> (usize, usize) {
        self.normalize_edit_range(self.caret.min(self.anchor), self.caret.max(self.anchor))
    }

    fn text_fill_for_abs_range(&self, start: usize, end: usize) -> Color {
        let (sel_s, sel_e) = self.selection_range();
        if sel_s != sel_e && ranges_overlap(start, end, sel_s, sel_e) {
            return self.theme.label_fill_hovered;
        }
        if let (Some(hs), Some(he)) = (self.hover_token_start, self.hover_token_end) {
            if ranges_overlap(start, end, hs, he) {
                return self.theme.label_fill_hovered;
            }
        }
        self.theme.label_fill
    }

    fn render_abs_range_highlight(&self, scene: &mut Scene, start: usize, end: usize, color: Color) {
        if start >= end {
            return;
        }
        let (s_line, s_byte) = offset_line_col(&self.text, start);
        let (e_line, e_byte) = offset_line_col(&self.text, end);
        if s_line == e_line {
            let line_text = self.text.split('\n').nth(s_line).unwrap_or("");
            let y = s_line as f64 * LINE_HEIGHT + LINE_HEIGHT * 0.75;
            let (x0, x1) = canvas_text::label_span_world_x(line_text, s_byte, e_byte, LINE_ORIGIN_X, FONT_PX);
            self.fill_highlight_rect(scene, x0, x1, y, color);
            return;
        }
        for line in s_line..=e_line {
            let line_text = self.text.split('\n').nth(line).unwrap_or("");
            let y = line as f64 * LINE_HEIGHT + LINE_HEIGHT * 0.75;
            let byte_start = if line == s_line { s_byte } else { 0 };
            let byte_end = if line == e_line {
                e_byte
            } else {
                line_text.len()
            };
            let (x0, x1) = canvas_text::label_span_world_x(line_text, byte_start, byte_end, LINE_ORIGIN_X, FONT_PX);
            self.fill_highlight_rect(scene, x0, x1, y, color);
        }
    }

    fn fill_highlight_rect(&self, scene: &mut Scene, x0: f64, x1: f64, y: f64, fill: Color) {
        let left = x0.min(x1);
        let right = x0.max(x1);
        if right <= left {
            return;
        }
        let rect = Rect::new(left, y - LINE_HEIGHT * 0.8, right, y + LINE_HEIGHT * 0.2);
        scene.fill(vello::peniko::Fill::NonZero, Affine::IDENTITY, fill, None, &rect);
    }

    fn hit_test_offset(&self, world: Point) -> usize {
        let rel_x = world.x;
        let rel_y = world.y;
        if rel_y < 0.0 {
            return 0;
        }
        let line = (rel_y / LINE_HEIGHT).floor().max(0.0) as usize;
        let line_text = self.text.split('\n').nth(line).unwrap_or("");
        let col = hit_byte_in_line(line_text, rel_x);
        offset_at_line_col(&self.text, line, col)
    }

    pub fn hit_test_offset_screen(&self, sx: f64, sy: f64) -> usize {
        let world = screen_to_world(&self.camera, &self.viewport, Point::new(sx, sy));
        self.hit_test_offset(world)
    }

    /// @emoji 🎯 Returns pick-target rows at a screen point for DOM disambiguation menus.
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        let offset = self.hit_test_offset_screen(sx, sy);
        let (line, col) = offset_line_col(&self.text, offset);
        let mut rows = Vec::new();
        rows.push(serde_json::json!({
            "domain": "line",
            "id": line.to_string(),
            "generality": 0,
            "label": format!("Line {}", line + 1),
        }));
        if let Some((start, end)) = self.token_span_at_offset(offset) {
            rows.push(serde_json::json!({
                "domain": "token",
                "id": format!("{start}:{end}"),
                "generality": 2,
                "label": self.text.get(start..end).unwrap_or("").to_string(),
            }));
        }
        serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
    }

    pub fn select_span_at_screen(&mut self, sx: f64, sy: f64) {
        let offset = self.hit_test_offset_screen(sx, sy);
        self.select_span_at(offset);
    }

    pub fn build_scene(&self) -> Scene {
        let mut world_scene = Scene::new();
        let bg = self.theme.raster_clear;
        world_scene.fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            bg,
            None,
            &Rect::new(-10_000.0, -10_000.0, 10_000.0, 10_000.0),
        );
        let lines: Vec<&str> = if self.text.is_empty() { vec![""] } else { self.text.split('\n').collect() };
        let (sel_start, sel_end) = self.selection_range();
        if !self.selection_occurrences.is_empty() {
            for range in &self.selection_occurrences {
                self.render_abs_range_highlight(&mut world_scene, range.start, range.end, self.theme.selection_fill);
            }
        } else if sel_start != sel_end {
            self.render_abs_range_highlight(&mut world_scene, sel_start, sel_end, self.theme.selection_fill);
        }
        if !self.hover_occurrences.is_empty() {
            for range in &self.hover_occurrences {
                self.render_abs_range_highlight(&mut world_scene, range.start, range.end, self.theme.hover_fill);
            }
        }
        for (i, line) in lines.iter().enumerate() {
            let y = i as f64 * LINE_HEIGHT + LINE_HEIGHT * 0.75;
            let gutter = format!("{}", i + 1);
            canvas_text::append_label(
                &mut world_scene,
                &gutter,
                Point::new(PAD_X, y),
                FONT_PX,
                self.theme.grid_minor_stroke,
                self.theme.label_halo,
            );
            if self.hover_occurrences.is_empty() {
                if let (Some(hs), Some(he)) = (self.hover_token_start, self.hover_token_end) {
                    if sel_start == sel_end || he <= sel_start || hs >= sel_end {
                        let line_start = offset_at_line_col(&self.text, i, 0);
                        let line_end = line_start + line.len();
                        if hs < line_end && he > line_start {
                            let start = hs.max(line_start) - line_start;
                            let end = he.min(line_end) - line_start;
                            let abs_s = line_start + start;
                            let abs_e = line_start + end;
                            self.render_abs_range_highlight(&mut world_scene, abs_s, abs_e, self.theme.hover_fill);
                        }
                    }
                }
            }
            self.render_colored_line(&mut world_scene, line, i, y);
        }
        self.render_placeholders(&mut world_scene);
        for offset in &self.extra_carets {
            if *offset != self.caret {
                self.render_caret_bar(&mut world_scene, *offset);
            }
        }
        self.render_caret(&mut world_scene, self.caret);
        for diag in &self.diagnostics {
            self.render_diagnostic(&mut world_scene, diag);
        }
        let aff = camera_content_affine(&self.camera, &self.viewport);
        let mut scene = Scene::new();
        scene.append(&world_scene, Some(aff));
        cavas::render::scale_scene_for_device_pixel_ratio(scene, self.viewport.dpr)
    }

    fn render_colored_line(&self, scene: &mut Scene, line: &str, line_index: usize, y: f64) {
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
            canvas_text::append_label(
                scene,
                line,
                Point::new(LINE_ORIGIN_X, y),
                FONT_PX,
                self.theme.label_fill,
                self.theme.label_halo,
            );
            return;
        }
        let color_spans: Vec<(usize, usize, Color)> = spans
            .iter()
            .map(|(start, end, _class)| {
                let abs_s = line_start + start;
                let abs_e = line_start + end;
                (*start, *end, self.text_fill_for_abs_range(abs_s, abs_e))
            })
            .collect();
        canvas_text::append_label_tspans(
            scene,
            line,
            &color_spans,
            Point::new(GUTTER_WIDTH + PAD_X, y),
            FONT_PX,
            self.theme.label_halo,
        );
    }

    fn render_placeholders(&self, scene: &mut Scene) {
        for placeholder in &self.placeholders {
            let (x, y) = offset_to_world(self, placeholder.offset);
            canvas_text::append_label(
                scene,
                &placeholder.label,
                Point::new(x, y),
                FONT_PX,
                self.theme.grid_minor_stroke,
                self.theme.label_halo,
            );
        }
    }

    fn render_caret_bar(&self, scene: &mut Scene, offset: usize) {
        let (x, y) = offset_to_world(self, offset);
        let rect = Rect::new(x, y - LINE_HEIGHT * 0.8, x + 1.5, y + LINE_HEIGHT * 0.2);
        scene.fill(
            vello::peniko::Fill::NonZero,
            Affine::IDENTITY,
            self.theme.label_fill,
            None,
            &rect,
        );
    }

    fn render_caret(&self, scene: &mut Scene, offset: usize) {
        if self.caret == self.anchor && !self.caret_visible {
            return;
        }
        self.render_caret_bar(scene, offset);
    }

    fn render_diagnostic(&self, scene: &mut Scene, diag: &DiagnosticJson) {
        let (x, y) = offset_to_world(self, diag.start);
        let (x2, _) = offset_to_world(self, diag.end.max(diag.start + 1));
        let color = match diag.severity.as_deref() {
            Some("warning") => self.theme.grid_minor_stroke,
            _ => self.theme.label_fill_hovered,
        };
        let rect = Rect::new(x, y + 2.0, x2.max(x + 8.0), y + 4.0);
        scene.fill(vello::peniko::Fill::NonZero, Affine::IDENTITY, color, None, &rect);
    }
}

fn is_insert_whitespace(chunk: &str) -> bool {
    !chunk.is_empty() && chunk.chars().all(|ch| matches!(ch, ' ' | '\t' | '\n' | '\r'))
}

fn ranges_overlap(a_start: usize, a_end: usize, b_start: usize, b_end: usize) -> bool {
    a_start < b_end && b_start < a_end
}

fn hit_byte_in_line(line: &str, world_x: f64) -> usize {
    if line.is_empty() {
        return 0;
    }
    let mut boundaries = vec![0usize];
    for (index, _) in line.char_indices() {
        if index > 0 {
            boundaries.push(index);
        }
    }
    if boundaries.last().copied() != Some(line.len()) {
        boundaries.push(line.len());
    }
    for pair in boundaries.windows(2) {
        let start = pair[0];
        let end = pair[1];
        let x0 = canvas_text::label_byte_world_x(line, start, LINE_ORIGIN_X, FONT_PX);
        let x1 = canvas_text::label_byte_world_x(line, end, LINE_ORIGIN_X, FONT_PX);
        if world_x < (x0 + x1) * 0.5 {
            return start;
        }
    }
    line.len()
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
    let (line, byte) = offset_line_col(&host.text, offset);
    let line_text = host.text.split('\n').nth(line).unwrap_or("");
    let x = canvas_text::label_byte_world_x(line_text, byte, LINE_ORIGIN_X, FONT_PX);
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
            .render_frame(&scene, self.host.theme.raster_clear)
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

    #[wasm_bindgen(js_name = setSelectableSpansJson)]
    pub fn set_selectable_spans_json(&mut self, json: &str) {
        self.state.borrow_mut().host.set_selectable_spans_json(json);
    }

    #[wasm_bindgen(js_name = setSemanticTokensJson)]
    pub fn set_semantic_tokens_json(&mut self, json: &str) {
        self.state.borrow_mut().host.set_semantic_tokens_json(json);
    }

    #[wasm_bindgen(js_name = setDiagnosticsJson)]
    pub fn set_diagnostics_json(&mut self, json: &str) {
        self.state.borrow_mut().host.set_diagnostics_json(json);
    }

    #[wasm_bindgen(js_name = setPlaceholdersJson)]
    pub fn set_placeholders_json(&mut self, json: &str) {
        self.state.borrow_mut().host.set_placeholders_json(json);
    }

    #[wasm_bindgen(js_name = setHoverOccurrencesJson)]
    pub fn set_hover_occurrences_json(&mut self, json: &str) {
        self.state.borrow_mut().host.set_hover_occurrences_json(json);
    }

    #[wasm_bindgen(js_name = setSelectionOccurrencesJson)]
    pub fn set_selection_occurrences_json(&mut self, json: &str) {
        self.state.borrow_mut().host.set_selection_occurrences_json(json);
    }

    #[wasm_bindgen(js_name = setExtraCaretsJson)]
    pub fn set_extra_carets_json(&mut self, json: &str) {
        self.state.borrow_mut().host.set_extra_carets_json(json);
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

    #[wasm_bindgen(js_name = setVelloThemeJson)]
    pub fn set_vello_theme_json(&mut self, json: &str) {
        let _ = self.state.borrow_mut().host.set_vello_theme_from_json(json);
    }

    #[wasm_bindgen(js_name = setCaretVisible)]
    pub fn set_caret_visible(&mut self, visible: bool) {
        self.state.borrow_mut().host.set_caret_visible(visible);
    }

    #[wasm_bindgen(js_name = anchor)]
    pub fn anchor(&self) -> usize {
        self.state.borrow().host.anchor()
    }

    #[wasm_bindgen(js_name = selectSpanAtScreen)]
    pub fn select_span_at_screen(&mut self, sx: f64, sy: f64) {
        self.state.borrow_mut().host.select_span_at_screen(sx, sy);
    }

    #[wasm_bindgen(js_name = pickTargetsAtScreenJson)]
    pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
        self.state.borrow().host.pick_targets_at_screen_json(sx, sy)
    }

    #[wasm_bindgen(js_name = selectSpanAt)]
    pub fn select_span_at(&mut self, offset: usize) {
        self.state.borrow_mut().host.select_span_at(offset);
    }

    #[wasm_bindgen(js_name = setSelectionRange)]
    pub fn set_selection_range(&mut self, anchor: usize, caret: usize) {
        self.state.borrow_mut().host.set_selection_range(anchor, caret);
    }

    #[wasm_bindgen(js_name = hoverTokenRangeJson)]
    pub fn hover_token_range_json(&self) -> String {
        match self.state.borrow().host.hover_token_range() {
            Some((start, end)) => serde_json::json!({ "start": start, "end": end }).to_string(),
            None => "null".into(),
        }
    }

    #[wasm_bindgen(js_name = setHoverRange)]
    pub fn set_hover_range(&mut self, start: usize, end: usize) {
        if start >= end {
            self.state.borrow_mut().host.set_hover_range(None, None);
        } else {
            self.state.borrow_mut().host.set_hover_range(Some(start), Some(end));
        }
    }

    #[wasm_bindgen(js_name = selectionText)]
    pub fn selection_text(&self) -> String {
        self.state.borrow().host.selection_text()
    }

    #[wasm_bindgen(js_name = replaceSelection)]
    pub fn replace_selection(&mut self, next: &str) {
        self.state.borrow_mut().host.replace_selection(next);
    }

    #[wasm_bindgen(js_name = selectAll)]
    pub fn select_all(&mut self) {
        self.state.borrow_mut().host.select_all();
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

    #[wasm_bindgen(js_name = moveLineStart)]
    pub fn move_line_start(&mut self, extend: bool) {
        self.state.borrow_mut().host.move_line_start(extend);
    }

    #[wasm_bindgen(js_name = moveLineEnd)]
    pub fn move_line_end(&mut self, extend: bool) {
        self.state.borrow_mut().host.move_line_end(extend);
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
    fn insert_space_inside_token_inserts_without_replacing() {
        let mut host = WriterHost::new();
        host.set_text("MATCH".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
        host.set_caret_anchor(3);
        host.insert_text(" ");
        assert_eq!(host.text(), "MAT CH");
        assert_eq!(host.caret(), 4);
    }

    #[test]
    fn insert_space_at_token_end_appends() {
        let mut host = WriterHost::new();
        host.set_text("MATCH".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
        host.set_caret_anchor(5);
        host.insert_text(" ");
        assert_eq!(host.text(), "MATCH ");
        assert_eq!(host.caret(), 6);
    }

    #[test]
    fn auto_space_before_next_token_at_token_end() {
        let mut host = WriterHost::new();
        host.set_text("MATCH".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
        host.set_caret_anchor(5);
        host.insert_text("(");
        assert_eq!(host.text(), "MATCH (");
    }

    #[test]
    fn insert_and_caret() {
        let mut host = WriterHost::new();
        host.insert_text("MATCH");
        assert_eq!(host.text(), "MATCH");
        assert_eq!(host.caret(), 5);
    }

    #[test]
    fn theme_merge_from_json_updates_clear() {
        let mut host = WriterHost::new();
        let json = r#"{"rasterClear":[240,236,221,255],"labelFill":[0,17,23,255]}"#;
        host.set_vello_theme_from_json(json).expect("theme json");
        let scene = host.build_scene();
        assert!(!scene.encoding().is_empty());
    }

    #[test]
    fn select_all_sets_range() {
        let mut host = WriterHost::new();
        host.set_text("abc".into());
        host.select_all();
        assert_eq!(host.anchor(), 0);
        assert_eq!(host.caret(), 3);
    }

    #[test]
    fn selection_snaps_var_label_composite() {
        let mut host = WriterHost::new();
        host.set_text("MATCH (a1:Piece)".into());
        host.set_semantic_tokens_json(
            r#"[{"start":0,"end":5,"class":"keyword"},{"start":7,"end":9,"class":"ident"},{"start":9,"end":10,"class":"operator"},{"start":10,"end":15,"class":"ident"}]"#,
        );
        host.set_selectable_spans_json(
            r#"[{"start":7,"end":9,"kind":"atomic"},{"start":7,"end":15,"kind":"varLabel","headEnd":9},{"start":10,"end":15,"kind":"atomic"}]"#,
        );
        host.set_selection(8, 12);
        assert_eq!(host.anchor(), 7);
        assert_eq!(host.caret(), 15);
    }

    #[test]
    fn select_span_at_picks_ident() {
        let mut host = WriterHost::new();
        host.set_text("RETURN a1.name".into());
        host.set_semantic_tokens_json(
            r#"[{"start":0,"end":6,"class":"keyword"},{"start":7,"end":9,"class":"ident"},{"start":9,"end":10,"class":"operator"},{"start":10,"end":14,"class":"ident"}]"#,
        );
        host.set_selectable_spans_json(
            r#"[{"start":7,"end":9,"kind":"atomic"},{"start":10,"end":14,"kind":"atomic"},{"start":7,"end":14,"kind":"propertyAccess","headEnd":9,"tailStart":10}]"#,
        );
        host.select_span_at(11);
        assert_eq!(host.anchor(), 10);
        assert_eq!(host.caret(), 14);
    }

    #[test]
    fn selection_snaps_fixed_keywords() {
        let mut host = WriterHost::new();
        host.set_text("MATCH x".into());
        host.set_semantic_tokens_json(r#"[{"start":0,"end":5,"class":"keyword"}]"#);
        host.set_selection(2, 4);
        assert_eq!(host.anchor(), 0);
        assert_eq!(host.caret(), 5);
    }

    #[test]
    fn drag_select_extends_range() {
        let mut host = WriterHost::new();
        host.set_size(800, 600, 1.0);
        host.set_text("hello world".into());
        host.pointer_down_screen(468.0, 317.0, 0);
        host.pointer_move_screen(560.0, 317.0, 1);
        host.pointer_up_screen(560.0, 317.0, 0);
        assert_ne!(host.caret(), host.anchor());
        assert_eq!(host.anchor(), 0);
        assert!(host.caret() > host.anchor());
    }

    #[test]
    fn punctuated_token_line_builds_scene() {
        let mut host = WriterHost::new();
        host.set_text("MATCH (a:Piece)".into());
        host.set_semantic_tokens_json(
            r#"[{"start":0,"end":5,"class":"keyword"},{"start":5,"end":6,"class":"operator"},{"start":6,"end":7,"class":"operator"},{"start":7,"end":8,"class":"plain"},{"start":8,"end":9,"class":"operator"},{"start":9,"end":14,"class":"plain"}]"#,
        );
        let scene = host.build_scene();
        assert!(!scene.encoding().path_tags.is_empty());
    }

    #[test]
    fn build_scene_has_content() {
        let mut host = WriterHost::new();
        host.set_text("MATCH (a:Piece)\nRETURN a.name".into());
        let scene = host.build_scene();
        assert!(!scene.encoding().path_tags.is_empty());
    }

    #[test]
    fn backspace_deletes_fixed_keyword_tokenwise() {
        let mut host = WriterHost::new();
        host.set_text("MATCH (a:Piece)".into());
        host.set_semantic_tokens_json(
            r#"[{"start":0,"end":5,"class":"keyword"},{"start":5,"end":6,"class":"operator"}]"#,
        );
        host.set_caret_anchor(3);
        host.backspace();
        assert_eq!(host.text(), " (a:Piece)");
        assert_eq!(host.caret(), 0);
    }

    #[test]
    fn label_span_world_x_matches_scaled_render() {
        let line = "MATCH (a:Piece)";
        let (x0, x5) = canvas_text::label_span_world_x(line, 0, 5, LINE_ORIGIN_X, FONT_PX);
        let estimate = canvas_text::label_advance("MATCH", FONT_PX);
        assert!(x5 - x0 < estimate);
        assert!(x5 > x0);
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
