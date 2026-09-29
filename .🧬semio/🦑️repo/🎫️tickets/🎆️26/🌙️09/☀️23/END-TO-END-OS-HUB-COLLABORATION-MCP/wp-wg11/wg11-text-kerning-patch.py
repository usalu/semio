#!/usr/bin/env python3
"""🤝️ WG11 session 14d — T7b (after T7a): pair kerning, cached per face and pair, applied identically in measure, wrap, caret/hit
and paint (coordinator: "T7b (GPOS/kern pair kerning cached per face+pair, applied identically in measure, wrap, caret/hit and
paint; its set ADDS the kerned-column law ≤ 0.5 px vs default Chromium)").

Measured (WG11 T7b probes, ticket folder `text/`): Chromium kerns the shipped faces by default — 42 of T7a's 120 corpus rows are
up to 2.75 px narrower than their advance sum (the pangram at 2xl) — and that kerning is PAIRWISE: every row's kerned width is its
unkerned width plus the sum of each adjacent pair's own kerning within 0.008 px. T7b's corpus adds one strongly kerned en + de
pair of strings (`AVATAR To Wave`, `VATER, Tätowierung`): 54 of its 144 rows kern, by up to 9.5 px; Anta kerns 24 of its 336
pairs (A+V −0.0928 em, f+o −0.0391 em, y+space −0.0293 em …); Share Tech Mono kerns none.

1. `📝️text`: `PairKerning` reads a pair's kerning from the authored face's own tables (GPOS pair adjustment or legacy `kern`)
   by shaping the pair alone in design units with ligatures and contextual alternates off, less the two nominal advances; the
   faces are held once (stable swash cache keys) so the shaping context stays warm. The atlas caches it per face and pair
   (`kerning_for`, bounded, bitmap fallback never kerns) and applies ONE rule — a scalar is kerned against the scalar before it
   once the pen has left the line start (`pen_kerning`) — in `measure_text_face`, `measure_range_face`, `pre_wrap_lines` and
   `wrap_lines_face`; `kerning_at` / `pen_advance` / `pen_at` give the per-scalar pen walk and the caret x (a caret after a
   kerned pair includes the pair, as Chromium's does).
2. `🖌️paint`: the retained glyph painter kerns before every glyph under the same rule (wrap pricing, overflow and pen); the
   per-scalar measure steppers (stepper label, caret, tree label, end-aligned text) walk `pen_advance`; the input caret and
   selection read `pen_at`; the tooltip measures its label instead of summing advances.
3. `🪀️widgets`: the three single-line pen loops are one `pen_glyph_run` that kerns under the same rule.
4. `📌️mounted_layout`: the text worker holds its own `PairKerning` and hands each glyph preview its kerning to the next scalar of
   its run; every intrinsic measure (clipped, max-content, min-content, definite wrap) adds a pair's kerning only while both
   scalars stay on one line. Every retained window's inline layout jobs grow by the worker's shaping context: the UI surface
   slot is 165 840 B (was 164 712; the boxed-fixed-slots budget fixture pins the measured size, overlay build 6).
5. Laws: the corpus fixture gains Chromium's KERNED column (`kernedWidthPx`, schema + React-side re-measure in Chromium);
   `🔬️targets-wgpu-text-unit` keeps T7a's unkerned law on the advance sum and ADDS the kerned-column law (≤ 0.5 px), the one-source
   kerning law, the measure/wrap/caret law and the painter law; `🔬️targets-wgpu-mounted-layout-unit` adds the worker law.

Dry run by default; `--write` backs every edited file up under `.🧬semio/🌐hub/s14-wg11-backup/text-kerning/` and applies; `--revert`
restores.
"""

import difflib
import shutil
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
WORK = Path("/Users/ueli" + "/Documents/semio/.tmp-ticket/wp-wg11/text")
UI = ROOT / "🧰️framework/🔨️modules/🖱️ui"
WGPU = UI / "🎯️targets/🧊️wgpu"
TEXT = WGPU / "📝️text/🦀️.rs"
PAINT = WGPU / "🖌️paint/🦀️.rs"
WIDGETS = WGPU / "🪀️widgets/🦀️.rs"
MOUNTED = WGPU / "📌️mounted_layout/🦀️.rs"
TEXT_LAWS = UI / "🧪️tests/🔬️targets-wgpu-text-unit/🦀️.rs"
MOUNTED_LAWS = UI / "🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🦀️.rs"
FIXTURE = UI / "🧫️fixtures/🔤️text-advances/🔣️.json"
SCHEMA = UI / "🧬️schema/🔤️text-advances/🔣️.json"
TS_LAW = UI / "🧪️tests/🔤️text-advances/🟦️.ts"
SLOT_BUDGET = ROOT / "🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json"
BACKUP = ROOT / ".🧬semio/🌐hub/s14-wg11-backup/text-kerning"

REPLACED_FILES = {
    FIXTURE: ("text-advances.json", "text-advances-kerned.json"),
    SCHEMA: ("text-advances-schema.json", "text-advances-kerned-schema.json"),
    TS_LAW: ("text-advances-law.ts", "text-advances-kerned-law.ts"),
}

TEXT_EDITS = [
    (
        '''use swash::scale::{Render, ScaleContext, Source, StrikeWith};
''',
        '''use swash::scale::{Render, ScaleContext, Source, StrikeWith};
use swash::shape::ShapeContext;
''',
    ),
    (
        '''    let authored = match face {
        TextFace::Sans => ANTA_LATIN,
        TextFace::Mono => SHARE_TECH_MONO_LATIN,
    };
    advance(authored).or_else(''',
        '''    advance(authored_face_bytes(face)).or_else(''',
    ),
    (
        '''/// 📏️ The bitmap fallback's advance per logical pixel of font size.
const BITMAP_ADVANCE_EM: f32 = 0.625;
''',
        '''/// 📏️ The bitmap fallback's advance per logical pixel of font size.
const BITMAP_ADVANCE_EM: f32 = 0.625;

/// 🔤️ The embedded font file an authored face is drawn from — Anta for [`TextFace::Sans`] (React's `--font-sans`, and the file
/// every host's atlas fetches), Share Tech Mono for [`TextFace::Mono`].
fn authored_face_bytes(face: TextFace) -> &'static [u8] {
    match face {
        TextFace::Sans => ANTA_LATIN,
        TextFace::Mono => SHARE_TECH_MONO_LATIN,
    }
}

/// 🤝️ Pair kerning read from the authored faces' own tables — GPOS pair adjustments, or the legacy `kern` table — through one
/// reusable shaping context over faces held once (so the context's per-font caches stay warm): the ONE kerning source both the
/// atlas (which caches it per face and pair, [`FontAtlas::kerning_for`]) and the retained layout's text worker (which holds one
/// per job) answer to. Chromium's kerning is pairwise for the shipped faces: the corpus's kerned widths are its unkerned widths
/// plus the sum of each adjacent pair's kerning within 0.008 px (ticket 26/09/23 session 14d, WG11 T7b probe).
pub(crate) struct PairKerning {
    shaper: ShapeContext,
    sans: Option<SwashFontRef<'static>>,
    mono: Option<SwashFontRef<'static>>,
}

impl Default for PairKerning {
    fn default() -> Self {
        Self { shaper: ShapeContext::new(), sans: SwashFontRef::from_index(authored_face_bytes(TextFace::Sans), 0), mono: SwashFontRef::from_index(authored_face_bytes(TextFace::Mono), 0) }
    }
}

impl PairKerning {
    /// 🤝️ The kerning between two adjacent scalars of `face`, per logical pixel of font size: the pair shaped alone in design
    /// units with ligatures and contextual alternates off, less its two nominal advances — exactly the adjustment Chromium's
    /// shaper puts between the two nominal glyphs. A scalar outside the authored face, or a default-ignorable one, never kerns:
    /// a fallback run does not kern against its neighbour.
    pub(crate) fn em(&mut self, face: TextFace, left: char, right: char) -> f32 {
        if is_zero_width_format_char(left) || is_zero_width_format_char(right) {
            return 0.0;
        }
        let font = match face {
            TextFace::Sans => self.sans,
            TextFace::Mono => self.mono,
        };
        let Some(font) = font else { return 0.0 };
        let (charmap, metrics) = (font.charmap(), font.glyph_metrics(&[]));
        let units = f32::from(font.metrics(&[]).units_per_em);
        if units <= 0.0 || charmap.map(left) == 0 || charmap.map(right) == 0 {
            return 0.0;
        }
        let mut bytes = [0u8; 8];
        let left_len = left.encode_utf8(&mut bytes).len();
        let pair_len = left_len + right.encode_utf8(&mut bytes[left_len..]).len();
        let Ok(pair) = std::str::from_utf8(&bytes[..pair_len]) else { return 0.0 };
        let mut shaper = self.shaper.builder(font).features(&[("liga", 0), ("clig", 0), ("calt", 0)]).build();
        shaper.add_str(pair);
        let (mut shaped, mut nominal, mut glyphs) = (0.0f32, 0.0f32, 0usize);
        shaper.shape_with(|cluster| {
            for glyph in cluster.glyphs {
                shaped += glyph.advance;
                nominal += metrics.advance_width(glyph.id);
                glyphs += 1;
            }
        });
        if glyphs == 2 {
            (shaped - nominal) / units
        } else {
            0.0
        }
    }
}
''',
    ),
    (
        '''const SIZED_GLYPH_CAPACITY: usize = 16_384;
''',
        '''const SIZED_GLYPH_CAPACITY: usize = 16_384;
/// 🤝️ Kerned scalar pairs cached before the pair map is rebuilt — no document may grow it without bound.
const KERNING_PAIR_CAPACITY: usize = 16_384;
''',
    ),
    (
        '''    sized: HashMap<(TextFace, char, u32), GlyphEntry>,
''',
        '''    sized: HashMap<(TextFace, char, u32), GlyphEntry>,
    /// 🤝️ Pair kerning per face and scalar pair, in em — [`PairKerning::em`]'s answers, bounded by [`KERNING_PAIR_CAPACITY`].
    kerning: HashMap<(TextFace, char, char), f32>,
    pair_kerning: PairKerning,
''',
    ),
    (
        '''            sized: HashMap::new(),
''',
        '''            sized: HashMap::new(),
            kerning: HashMap::new(),
            pair_kerning: PairKerning::default(),
''',
        2,
    ),
    (
        '''    /// 📏️ Advance-summed width and the CSS line box height (`line_height`) — a single line of text''',
        '''    /// 📏️ Advance-and-kerning-summed width and the CSS line box height (`line_height`) — a single line of text''',
    ),
    (
        '''    /// 📏️ Measures one run with the same face-specific advances its painter consumes.
    pub fn measure_text_face(&mut self, face: TextFace, text: &str, size: f32) -> (f32, f32) {
        let mut width = 0.0f32;
        let mut max_height = 0.0f32;
        for ch in text.chars() {
            let glyph = self.ensure_glyph_for(face, ch, size);''',
        '''    /// 📏️ Measures one run with the same face-specific advances and pair kerning its painter pens.
    pub fn measure_text_face(&mut self, face: TextFace, text: &str, size: f32) -> (f32, f32) {
        let mut width = 0.0f32;
        let mut max_height = 0.0f32;
        let mut previous = None;
        for ch in text.chars() {
            width += self.pen_kerning(face, previous, width, ch, size);
            previous = Some(ch);
            let glyph = self.ensure_glyph_for(face, ch, size);''',
    ),
    (
        '''    /// 📏️ Measures a byte range with one face-specific glyph stream.
    pub fn measure_range_face(&mut self, face: TextFace, text: &str, byte: usize, end: usize, size: f32) -> f32 {
        let Some(run) = text.get(byte..end) else { return 0.0 };
        run.chars().map(|ch| self.ensure_glyph_for(face, ch, size).advance).sum()
    }
''',
        '''    /// 📏️ Measures a byte range with one face-specific glyph stream, kerned within the range only — the range's own width.
    pub fn measure_range_face(&mut self, face: TextFace, text: &str, byte: usize, end: usize, size: f32) -> f32 {
        let Some(run) = text.get(byte..end) else { return 0.0 };
        self.measure_text_face(face, run, size).0
    }

    /// 🤝️ The pair kerning between `left` and `right` in `face` at `size` — [`PairKerning::em`] cached per face and pair. The
    /// fixed-pitch bitmap fallback never kerns.
    pub fn kerning_for(&mut self, face: TextFace, left: char, right: char, size: f32) -> f32 {
        if !matches!(self.mode, AtlasMode::Shaped) {
            return 0.0;
        }
        let key = (face, left, right);
        let em = match self.kerning.get(&key).copied() {
            Some(em) => em,
            None => {
                let em = self.pair_kerning.em(face, left, right);
                if self.kerning.len() >= KERNING_PAIR_CAPACITY {
                    self.kerning.clear();
                }
                self.kerning.insert(key, em);
                em
            }
        };
        em * size
    }

    /// 🤝️ The kerning a line pens before `ch`: its pair with the `previous` scalar once the pen has left the line start, nothing
    /// at a line start — the ONE rule measure, wrap, caret and paint all apply.
    pub fn pen_kerning(&mut self, face: TextFace, previous: Option<char>, pen: f32, ch: char, size: f32) -> f32 {
        match previous {
            Some(left) if pen > 0.0 => self.kerning_for(face, left, ch, size),
            _ => 0.0,
        }
    }

    /// 🤝️ The pair kerning at `byte` of `text`: between the scalar ending there and the scalar starting there, nothing at either
    /// end of the text.
    pub fn kerning_at(&mut self, text: &str, byte: usize, size: f32) -> f32 {
        let (Some(before), Some(after)) = (text.get(..byte), text.get(byte..)) else { return 0.0 };
        match (before.chars().next_back(), after.chars().next()) {
            (Some(left), Some(right)) => self.kerning_for(TextFace::Sans, left, right, size),
            _ => 0.0,
        }
    }

    /// ✒️ How far the scalar at `byte` moves a line's pen: its advance plus its pair kerning with the scalar after it — summed over
    /// a run, exactly [`Self::measure_text`]; summed up to a caret, exactly [`Self::pen_at`].
    pub fn pen_advance(&mut self, text: &str, byte: usize, size: f32) -> f32 {
        let Some(ch) = text.get(byte..).and_then(|rest| rest.chars().next()) else { return 0.0 };
        let advance = self.ensure_glyph(ch, size).advance;
        advance + self.kerning_at(text, byte + ch.len_utf8(), size)
    }

    /// 📍️ Where the scalar at `byte` is penned on a single line of `text` — the caret's x: the run before it plus that run's pair
    /// kerning with it, where Chromium places a caret after a kerned pair.
    pub fn pen_at(&mut self, text: &str, byte: usize, size: f32) -> f32 {
        let pen = self.measure_range(text, 0, byte, size);
        pen + if pen > 0.0 { self.kerning_at(text, byte, size) } else { 0.0 }
    }
''',
    ),
    (
        '''            let mut last_break = None;
''',
        '''            let mut last_break = None;
            let mut previous = None;
''',
    ),
    (
        '''                let advance = self.ensure_glyph_for(face, ch, size).advance;
                if !is_wrap_space(ch) && pen + advance > limit + LINE_BREAK_FIT_EPSILON && byte > line_start {''',
        '''                let kerning = self.pen_kerning(face, previous, pen, ch, size);
                let advance = self.ensure_glyph_for(face, ch, size).advance;
                if !is_wrap_space(ch) && pen + kerning + advance > limit + LINE_BREAK_FIT_EPSILON && byte > line_start {''',
    ),
    (
        '''                pen += advance;
                byte = next;
                if is_wrap_space(ch) {
                    last_break = Some(next);''',
        '''                pen += kerning;
                pen += advance;
                previous = Some(ch);
                byte = next;
                if is_wrap_space(ch) {
                    last_break = Some(next);''',
    ),
    (
        '''        let (mut line_start, mut pen, mut byte) = (0usize, 0.0f32, 0usize);
        while let Some(ch) = text.get(byte..).and_then(|rest| rest.chars().next()) {
            let next = byte + ch.len_utf8();
            if ch == '\\n' {
                lines.push(line_start..byte);
                (line_start, pen, byte) = (next, 0.0, next);
                continue;
            }
            if pen > 0.0 && is_break_opportunity(text, byte) {
                let run = self.measure_range_face(face, text, byte, unbreakable_run_end(text, byte), size);
                if pen + run > limit + LINE_BREAK_FIT_EPSILON {
                    lines.push(line_start..byte);
                    (line_start, pen) = (byte, 0.0);
                }
            }
            let advance = self.ensure_glyph_for(face, ch, size).advance;
            if pen > 0.0 && !is_wrap_space(ch) && pen + advance > limit + LINE_BREAK_FIT_EPSILON {
                lines.push(line_start..byte);
                (line_start, pen) = (byte, 0.0);
            }
            pen += advance;
            byte = next;
        }''',
        '''        let (mut line_start, mut pen, mut byte, mut previous) = (0usize, 0.0f32, 0usize, None);
        while let Some(ch) = text.get(byte..).and_then(|rest| rest.chars().next()) {
            let next = byte + ch.len_utf8();
            if ch == '\\n' {
                lines.push(line_start..byte);
                (line_start, pen, byte, previous) = (next, 0.0, next, None);
                continue;
            }
            let mut kerning = self.pen_kerning(face, previous, pen, ch, size);
            if pen > 0.0 && is_break_opportunity(text, byte) {
                let run = self.measure_range_face(face, text, byte, unbreakable_run_end(text, byte), size);
                if pen + kerning + run > limit + LINE_BREAK_FIT_EPSILON {
                    lines.push(line_start..byte);
                    (line_start, pen, kerning) = (byte, 0.0, 0.0);
                }
            }
            let advance = self.ensure_glyph_for(face, ch, size).advance;
            if pen > 0.0 && !is_wrap_space(ch) && pen + kerning + advance > limit + LINE_BREAK_FIT_EPSILON {
                lines.push(line_start..byte);
                (line_start, pen, kerning) = (byte, 0.0, 0.0);
            }
            pen += kerning;
            pen += advance;
            previous = Some(ch);
            byte = next;
        }''',
    ),
]

PAINT_EDITS = [
    (
        '''    if matches!(flow, RetainedTextFlow::Wrap) && cursor.pen_x > 0.0 && crate::wgpu::text::is_break_opportunity(value, cursor.byte) {
        let run = atlas.measure_range(value, cursor.byte, crate::wgpu::text::unbreakable_run_end(value, cursor.byte), size);
        if cursor.pen_x + run > bounds.w.max(1.0) + RETAINED_TEXT_FIT_EPSILON {
            let Some(next_line) = cursor.line.checked_add(1) else { return RetainedGlyphStep::Fault };
            cursor.line = next_line;
            cursor.pen_x = 0.0;
        }
    }''',
        '''    let mut kerning = if cursor.pen_x > 0.0 { atlas.kerning_at(value, cursor.byte, size) } else { 0.0 };
    if matches!(flow, RetainedTextFlow::Wrap) && cursor.pen_x > 0.0 && crate::wgpu::text::is_break_opportunity(value, cursor.byte) {
        let run = atlas.measure_range(value, cursor.byte, crate::wgpu::text::unbreakable_run_end(value, cursor.byte), size);
        if cursor.pen_x + kerning + run > bounds.w.max(1.0) + RETAINED_TEXT_FIT_EPSILON {
            let Some(next_line) = cursor.line.checked_add(1) else { return RetainedGlyphStep::Fault };
            cursor.line = next_line;
            cursor.pen_x = 0.0;
            kerning = 0.0;
        }
    }''',
    ),
    (
        '''    let overflows = !hangs && cursor.pen_x > 0.0 && cursor.pen_x + advance > bounds.w.max(1.0) + RETAINED_TEXT_FIT_EPSILON;
    if overflows && matches!(flow, RetainedTextFlow::Clip) {
        if draw.finish_retained_output().is_err() {
            return RetainedGlyphStep::Fault;
        }
        cursor.byte = next_byte;
        cursor.pen_x += advance;
        return RetainedGlyphStep::Pending;
    }
    if overflows {
        let Some(next_line) = cursor.line.checked_add(1) else {
            let _ = draw.finish_retained_output();
            return RetainedGlyphStep::Fault;
        };
        cursor.line = next_line;
        cursor.pen_x = 0.0;
    }
''',
        '''    let overflows = !hangs && cursor.pen_x > 0.0 && cursor.pen_x + kerning + advance > bounds.w.max(1.0) + RETAINED_TEXT_FIT_EPSILON;
    if overflows && matches!(flow, RetainedTextFlow::Clip) {
        if draw.finish_retained_output().is_err() {
            return RetainedGlyphStep::Fault;
        }
        cursor.byte = next_byte;
        cursor.pen_x += kerning + advance;
        return RetainedGlyphStep::Pending;
    }
    if overflows {
        let Some(next_line) = cursor.line.checked_add(1) else {
            let _ = draw.finish_retained_output();
            return RetainedGlyphStep::Fault;
        };
        cursor.line = next_line;
        cursor.pen_x = 0.0;
        kerning = 0.0;
    }
    cursor.pen_x += kerning;
''',
    ),
    (
        '''        cursor.measure_width += atlas.measure_text(&value[cursor.measure_byte..end], theme.font_size_body).0;''',
        '''        cursor.measure_width += atlas.pen_advance(value, cursor.measure_byte, theme.font_size_body);''',
    ),
    (
        '''        let width = atlas.measure_text(&value[cursor.measure_byte..end], size).0;''',
        '''        let width = atlas.pen_advance(value, cursor.measure_byte, size);''',
    ),
    (
        '''            cursor.measure_width += atlas.measure_text(&value[cursor.measure_byte..end], size).0;''',
        '''            cursor.measure_width += atlas.pen_advance(value, cursor.measure_byte, size);''',
        2,
    ),
    (
        '''            let (x0, _) = atlas.measure_text(&edit.text[..start], theme.font_size_body);
            let (x1, _) = atlas.measure_text(&edit.text[..end], theme.font_size_body);''',
        '''            let x0 = atlas.pen_at(&edit.text, start, theme.font_size_body);
            let x1 = atlas.pen_at(&edit.text, end, theme.font_size_body);''',
    ),
    (
        '''        let (caret_x, _) = atlas.measure_text(&edit.text[..edit.caret], theme.font_size_body);''',
        '''        let caret_x = atlas.pen_at(&edit.text, edit.caret, theme.font_size_body);''',
    ),
    (
        '''    let advance = label.chars().map(|ch| atlas.ensure_glyph(ch, theme.font_size_small).advance).sum::<f32>();''',
        '''    let advance = atlas.measure_text(label, theme.font_size_small).0;''',
    ),
]

WIDGET_EDITS = [
    (
        '''/// 🔤️ Paints one run with a selected authored face into an explicit draw list.
pub fn draw_text_face_on(draw: &mut DrawList, atlas: &mut FontAtlas, face: TextFace, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    let atlas_w = atlas.width as f32;
    let atlas_h = atlas.height as f32;
    let mut cursor_x = x;
    for ch in text.chars() {
        let glyph = atlas.ensure_glyph_for(face, ch, size);
        let gw = glyph.logical_width();
        let gh = glyph.logical_height();
        let gx = cursor_x + glyph.bearing_x;
        let gy = y - gh - glyph.bearing_y;
        let uv_rect = [glyph.atlas_x as f32 / atlas_w, glyph.atlas_y as f32 / atlas_h, (glyph.atlas_x + glyph.width) as f32 / atlas_w, (glyph.atlas_y + glyph.height) as f32 / atlas_h];
        draw.push_glyph([gx, gy, gw.max(1.0), gh.max(1.0)], color, uv_rect);
        cursor_x += glyph.advance;
    }
}

pub fn draw_text_overlay_on(draw: &mut DrawList, atlas: &mut FontAtlas, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    let atlas_w = atlas.width as f32;
    let atlas_h = atlas.height as f32;
    let mut cursor_x = x;
    for ch in text.chars() {
        let glyph = atlas.ensure_glyph(ch, size);
        let gw = glyph.logical_width();
        let gh = glyph.logical_height();
        let gx = cursor_x + glyph.bearing_x;
        let gy = y - gh - glyph.bearing_y;
        let uv_rect = [glyph.atlas_x as f32 / atlas_w, glyph.atlas_y as f32 / atlas_h, (glyph.atlas_x + glyph.width) as f32 / atlas_w, (glyph.atlas_y + glyph.height) as f32 / atlas_h];
        draw.push_glyph_overlay([gx, gy, gw.max(1.0), gh.max(1.0)], color, uv_rect);
        cursor_x += glyph.advance;
    }
}
''',
        '''/// 🔤️ Paints one run with a selected authored face into an explicit draw list.
pub fn draw_text_face_on(draw: &mut DrawList, atlas: &mut FontAtlas, face: TextFace, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    pen_glyph_run(draw, atlas, face, text, (x, y), size, color, false);
}

pub fn draw_text_overlay_on(draw: &mut DrawList, atlas: &mut FontAtlas, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    pen_glyph_run(draw, atlas, TextFace::Sans, text, (x, y), size, color, true);
}

/// ✒️ Pens one single-line run from its baseline `origin`, glyph by glyph — each glyph's advance plus its pair kerning with the
/// glyph before it ([`FontAtlas::pen_kerning`], the rule [`FontAtlas::measure_text_face`] sums by) — onto the draw list's
/// regular or overlay instances.
fn pen_glyph_run(draw: &mut DrawList, atlas: &mut FontAtlas, face: TextFace, text: &str, origin: (f32, f32), size: f32, color: Rgba, overlay: bool) {
    let atlas_w = atlas.width as f32;
    let atlas_h = atlas.height as f32;
    let (mut cursor_x, mut previous) = (origin.0, None);
    for ch in text.chars() {
        cursor_x += atlas.pen_kerning(face, previous, cursor_x - origin.0, ch, size);
        previous = Some(ch);
        let glyph = *atlas.ensure_glyph_for(face, ch, size);
        let gw = glyph.logical_width();
        let gh = glyph.logical_height();
        let rect = [cursor_x + glyph.bearing_x, origin.1 - gh - glyph.bearing_y, gw.max(1.0), gh.max(1.0)];
        let uv_rect = [glyph.atlas_x as f32 / atlas_w, glyph.atlas_y as f32 / atlas_h, (glyph.atlas_x + glyph.width) as f32 / atlas_w, (glyph.atlas_y + glyph.height) as f32 / atlas_h];
        if overlay {
            draw.push_glyph_overlay(rect, color, uv_rect);
        } else {
            draw.push_glyph(rect, color, uv_rect);
        }
        cursor_x += glyph.advance;
    }
}
''',
    ),
    (
        '''/// 🔤️ Paints one run with a selected authored face through a widget context.
pub fn draw_text_face<E>(ctx: &mut WidgetContext<'_, E>, face: TextFace, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    let atlas_w = ctx.atlas.width as f32;
    let atlas_h = ctx.atlas.height as f32;
    let mut cursor_x = x;
    for ch in text.chars() {
        let glyph = ctx.atlas.ensure_glyph_for(face, ch, size);
        let gw = glyph.logical_width();
        let gh = glyph.logical_height();
        let gx = cursor_x + glyph.bearing_x;
        let gy = y - gh - glyph.bearing_y;
        let uv_rect = [glyph.atlas_x as f32 / atlas_w, glyph.atlas_y as f32 / atlas_h, (glyph.atlas_x + glyph.width) as f32 / atlas_w, (glyph.atlas_y + glyph.height) as f32 / atlas_h];
        ctx.draw.push_glyph([gx, gy, gw.max(1.0), gh.max(1.0)], color, uv_rect);
        cursor_x += glyph.advance;
    }
}''',
        '''/// 🔤️ Paints one run with a selected authored face through a widget context.
pub fn draw_text_face<E>(ctx: &mut WidgetContext<'_, E>, face: TextFace, text: &str, x: f32, y: f32, size: f32, color: Rgba) {
    pen_glyph_run(ctx.draw, ctx.atlas, face, text, (x, y), size, color, false);
}''',
    ),
]

MOUNTED_EDITS = [
    (
        '''pub(crate) struct RetainedGlyphPreview {
    pub scalar: char,
    pub advance: f32,
''',
        '''pub(crate) struct RetainedGlyphPreview {
    pub scalar: char,
    pub advance: f32,
    /// 🤝️ The pair kerning between this scalar and the next scalar of its run at the worker's size — nothing for a run's last.
    pub kerning: f32,
''',
    ),
    (
        '''    fn shape_one(&mut self, input: RetainedGlyphInput) -> RetainedGlyphPreview;''',
        '''    fn shape_one(&mut self, input: RetainedGlyphInput, next: Option<char>) -> RetainedGlyphPreview;''',
    ),
    (
        '''#[derive(Default)]
struct DeterministicTextWorker {
''',
        '''#[derive(Default)]
struct DeterministicTextWorker {
    kerning: crate::wgpu::text::PairKerning,
''',
    ),
    (
        '''    fn shape_one(&mut self, input: RetainedGlyphInput) -> RetainedGlyphPreview {
        let advance = crate::wgpu::text::font_advance_em(crate::wgpu::text::TextFace::Sans, input.scalar) * DEFAULT_TEXT_SIZE_PX;
''',
        '''    fn shape_one(&mut self, input: RetainedGlyphInput, next: Option<char>) -> RetainedGlyphPreview {
        let advance = crate::wgpu::text::font_advance_em(crate::wgpu::text::TextFace::Sans, input.scalar) * DEFAULT_TEXT_SIZE_PX;
        let kerning = next.map_or(0.0, |right| self.kerning.em(crate::wgpu::text::TextFace::Sans, input.scalar, right) * DEFAULT_TEXT_SIZE_PX);
''',
    ),
    (
        '''        RetainedGlyphPreview { scalar: input.scalar, advance, height: ''',
        '''        RetainedGlyphPreview { scalar: input.scalar, advance, kerning, height: ''',
    ),
    (
        '''        let raw_preview = self.text_worker.shape_one(input);''',
        '''        let next = self.glyphs.get(self.glyph_cursor + 1).filter(|_| self.glyph_cursor + 1 < run.glyph_end).map(|glyph| glyph.scalar);
        let raw_preview = self.text_worker.shape_one(input, next);''',
    ),
    (
        '''            node.intrinsic.width += preview.advance;''',
        '''            node.intrinsic.width += preview.advance + preview.kerning;''',
    ),
    (
        '''/// 📏️ One text node's intrinsic size against the space the solver offers it, from the shaped
/// advances alone — the worker thread holds no tree and no font atlas.''',
        '''/// 📏️ One text node's intrinsic size against the space the solver offers it, from the shaped
/// advances and pair kerning alone — the worker thread holds no tree and no font atlas. A pair's
/// kerning counts only while both scalars stay on one line, the rule the atlas and painter apply.''',
    ),
    (
        '''    let advance = |cursor: usize| previews.get(cursor).map_or(0.0, |preview| preview.advance * scale);
    let scalar = |cursor: usize| glyphs.get(cursor).map_or(' ', |glyph| glyph.scalar);
    if clipped {
        let width: f32 = (start..end).map(advance).sum();''',
        '''    let advance = |cursor: usize| previews.get(cursor).map_or(0.0, |preview| preview.advance * scale);
    let kerning = |cursor: usize| if cursor > start { previews.get(cursor - 1).map_or(0.0, |preview| preview.kerning * scale) } else { 0.0 };
    let scalar = |cursor: usize| glyphs.get(cursor).map_or(' ', |glyph| glyph.scalar);
    let single_line = || (start..end).map(|cursor| kerning(cursor) + advance(cursor)).sum::<f32>();
    if clipped {
        let width = single_line();''',
    ),
    (
        '''        MeasureConstraint::MaxContent => ((start..end).map(advance).sum(), line),''',
        '''        MeasureConstraint::MaxContent => (single_line(), line),''',
    ),
    (
        '''                if ch != '\\n' && !is_wrap_space(ch) {
                    run += advance(cursor);
                }''',
        '''                if ch != '\\n' && !is_wrap_space(ch) {
                    run += advance(cursor) + if run > 0.0 { kerning(cursor) } else { 0.0 };
                }''',
    ),
    (
        '''                let advance = advance(cursor);
                if is_wrap_space(ch) {
                    placed += run + advance;
                    run = 0.0;
                    continue;
                }
                if placed + run > 0.0 && placed + run + advance > available {
                    widest = widest.max(ink);
                    if placed > 0.0 {
                        placed = 0.0;
                    } else {
                        run = 0.0;
                    }
                    lines += 1;
                }
                run += advance;''',
        '''                let (advance, mut kerning) = (advance(cursor), if placed + run > 0.0 { kerning(cursor) } else { 0.0 });
                if is_wrap_space(ch) {
                    placed += run + kerning + advance;
                    run = 0.0;
                    continue;
                }
                if placed + run > 0.0 && placed + run + kerning + advance > available {
                    widest = widest.max(ink);
                    if placed > 0.0 {
                        placed = 0.0;
                    } else {
                        run = 0.0;
                    }
                    if run == 0.0 {
                        kerning = 0.0;
                    }
                    lines += 1;
                }
                run += kerning + advance;''',
    ),
]

T7A_CORPUS_LAW = '''/// 🔤️ LAW (WG11 T7a): the shared corpus — both faces, sizes 2xs…2xl, en + de strings — measures within the fixture's tolerance
/// (0.5 px) of Chromium's UNKERNED DOM width (`🧫️fixtures/🔤️text-advances`, re-measured in Chromium by the React-side law).
#[test]
fn the_shared_corpus_measures_as_chromium_does_without_kerning() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔤️text-advances/🔣️.json")).expect("the text advance corpus parses");
    let tolerance = fixture["tolerancePx"].as_f64().expect("tolerance") as f32;
    let mut atlas = FontAtlas::shaped_default();
    for row in fixture["rows"].as_array().expect("corpus rows") {
        let face = match row["face"].as_str() {
            Some("mono") => super::TextFace::Mono,
            _ => super::TextFace::Sans,
        };
        let text = row["text"].as_str().expect("text");
        let size = row["sizePx"].as_f64().expect("size") as f32;
        let expected = row["unkernedWidthPx"].as_f64().expect("width") as f32;
        let measured = atlas.measure_text_face(face, text, size).0;
        assert!((measured - expected).abs() <= tolerance, "{face:?} {size}px {text:?}: {measured} vs Chromium {expected}");
    }
}'''

T7B_LAWS = '''/// 🔤️ LAW (WG11 T7a): the shared corpus — both faces, sizes 2xs…2xl, en + de strings — sums its ADVANCES within the fixture's
/// tolerance (0.5 px) of Chromium's UNKERNED DOM width (`🧫️fixtures/🔤️text-advances`, re-measured in Chromium by the React-side law).
#[test]
fn the_shared_corpus_advances_sum_to_chromiums_unkerned_width() {
    for_each_corpus_row(|atlas, face, text, size, row| {
        let advances: f32 = text.chars().map(|ch| atlas.ensure_glyph_for(face, ch, size).advance).sum();
        (advances, row["unkernedWidthPx"].as_f64().expect("unkerned width") as f32)
    });
}

/// 🤝️ LAW (WG11 T7b): the same corpus MEASURES — advances plus every adjacent pair's kerning — within the fixture's tolerance
/// (0.5 px) of Chromium's default, KERNED DOM width; 54 of its 144 rows kern, by up to 9.5 px.
#[test]
fn the_shared_corpus_measures_as_chromium_does_with_kerning() {
    for_each_corpus_row(|atlas, face, text, size, row| (atlas.measure_text_face(face, text, size).0, row["kernedWidthPx"].as_f64().expect("kerned width") as f32));
}

/// 🔤️ Walks every corpus row (`🧫️fixtures/🔤️text-advances`) through `width`, which answers the wgpu width and the Chromium width it
/// must land within the fixture's tolerance of.
fn for_each_corpus_row(mut width: impl FnMut(&mut FontAtlas, super::TextFace, &str, f32, &serde_json::Value) -> (f32, f32)) {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔤️text-advances/🔣️.json")).expect("the text advance corpus parses");
    let tolerance = fixture["tolerancePx"].as_f64().expect("tolerance") as f32;
    let mut atlas = FontAtlas::shaped_default();
    for row in fixture["rows"].as_array().expect("corpus rows") {
        let face = match row["face"].as_str() {
            Some("mono") => super::TextFace::Mono,
            _ => super::TextFace::Sans,
        };
        let text = row["text"].as_str().expect("text");
        let size = row["sizePx"].as_f64().expect("size") as f32;
        let (measured, expected) = width(&mut atlas, face, text, size, row);
        assert!((measured - expected).abs() <= tolerance, "{face:?} {size}px {text:?}: {measured} vs Chromium {expected}");
    }
}

/// 🤝️ The corpus pangram — Anta kerns six of its pairs (−0.127 em in all).
const KERNED_PANGRAM: &str = "The quick brown fox jumps over the lazy dog";

/// 🤝️ LAW (WG11 T7b): the atlas and the retained layout's text worker kern by ONE source — the face's own pair table read through
/// `PairKerning` — which Anta's `A`+`V` and `f`+`o` pin at Chromium's own −0.0928 and −0.0391 em; the monospace face does not
/// kern, a scalar outside the authored face never kerns against its neighbour, and the fixed-pitch bitmap fallback never kerns.
#[test]
fn the_layout_kerning_source_is_the_atlas_kerning() {
    let mut atlas = FontAtlas::shaped_default();
    let mut source = super::PairKerning::default();
    for (left, right, chromium) in [('A', 'V', -0.0928), ('f', 'o', -0.0391)] {
        let em = source.em(super::TextFace::Sans, left, right);
        assert!((em - chromium).abs() < 0.0005, "Anta kerns {left:?}+{right:?} by Chromium's own {chromium} em: {em}");
    }
    for (face, left, right) in [(super::TextFace::Sans, 'A', 'V'), (super::TextFace::Sans, 'y', ' '), (super::TextFace::Sans, 'T', 'h'), (super::TextFace::Mono, 'f', 'o')] {
        let cached = atlas.kerning_for(face, left, right, 11.2);
        let direct = source.em(face, left, right) * 11.2;
        assert!((cached - direct).abs() < 0.0001, "{face:?} {left:?}+{right:?}: atlas {cached} vs source {direct}");
    }
    assert_eq!(source.em(super::TextFace::Mono, 'f', 'o'), 0.0, "Share Tech Mono is fixed-pitch");
    assert_eq!(source.em(super::TextFace::Sans, 'o', '🔥'), 0.0, "an emoji fallback never kerns against Anta");
    assert_eq!(FontAtlas::builtin().kerning_for(super::TextFace::Sans, 'f', 'o', 11.2), 0.0, "the bitmap fallback never kerns");
}

/// 🤝️ LAW (WG11 T7b): pair kerning is ONE rule for measure, wrap and caret — a run measures as its advances plus each adjacent
/// pair's kerning, the per-scalar pen walk (`pen_advance`) lands every caret where `pen_at` puts it (a caret after a kerned pair
/// includes the pair, as Chromium's does) and ends at the measured width, and a box exactly as wide as the kerned run holds it on
/// one line where the unkerned sum would overflow.
#[test]
fn pair_kerning_is_one_rule_for_measure_wrap_and_caret() {
    let (text, size) = (KERNED_PANGRAM, 12.8);
    let mut atlas = FontAtlas::shaped_default();
    let scalars: Vec<(usize, char)> = text.char_indices().collect();
    let advances: f32 = scalars.iter().map(|&(_, ch)| atlas.ensure_glyph(ch, size).advance).sum();
    let kerning: f32 = scalars.windows(2).map(|pair| atlas.kerning_for(super::TextFace::Sans, pair[0].1, pair[1].1, size)).sum();
    let measured = atlas.measure_text(text, size).0;
    assert!(kerning < -0.5, "the pangram kerns in Anta: {kerning}");
    assert!((measured - (advances + kerning)).abs() < 0.001, "measured {measured} vs advances {advances} + kerning {kerning}");
    let mut pen = 0.0f32;
    for &(byte, ch) in &scalars {
        let caret = atlas.pen_at(text, byte, size);
        assert!((caret - pen).abs() < 0.001, "the caret before {ch:?} at byte {byte}: {caret} vs pen walk {pen}");
        pen += atlas.pen_advance(text, byte, size);
    }
    assert!((pen - measured).abs() < 0.001, "the pen walk ends at the measured width: {pen} vs {measured}");
    assert!((atlas.pen_at(text, text.len(), size) - measured).abs() < 0.001, "the caret at the end is the measured width");
    let fits = measured + 0.01;
    assert!(advances > fits + super::LINE_BREAK_FIT_EPSILON, "the unkerned sum {advances} would overflow a {fits} box");
    assert_eq!(atlas.wrap_lines(text, fits, size).len(), 1, "the kerned run fits its own box");
    assert_eq!(atlas.pre_wrap_lines(super::TextFace::Sans, text, fits, size).len(), 1, "pre-wrap prices the same kerned run");
}

/// 🤝️ LAW (WG11 T7b): the retained painter pens every glyph at its kerned caret and breaks where the kerned wrap does — measure,
/// wrap, caret and paint read one pair-kerning table under one rule.
#[test]
fn the_retained_painter_pens_the_kerned_caret_and_breaks_where_the_kerned_wrap_does() {
    use crate::wgpu::draw::{DrawList, KIND_GLYPH};
    use crate::wgpu::geometry::Rect;
    use crate::wgpu::paint::{paint_retained_glyph_step_flowed, RetainedGlyphCursor, RetainedGlyphStep, RetainedTextFlow};
    use crate::wgpu::theme::Rgba;

    let (text, size) = (KERNED_PANGRAM, 12.8);
    let mut atlas = FontAtlas::shaped_default();
    let paint = |atlas: &mut FontAtlas, bounds: Rect, flow: RetainedTextFlow| {
        let (mut draw, mut cursor, mut lines) = (DrawList::default(), RetainedGlyphCursor::default(), Vec::new());
        loop {
            let byte = cursor.byte();
            match paint_retained_glyph_step_flowed(text, bounds, size, Rgba::new(1.0, 1.0, 1.0, 1.0), flow, atlas, &mut draw, &mut cursor) {
                RetainedGlyphStep::Pending => lines.push((byte, cursor.line())),
                RetainedGlyphStep::Complete => break,
                RetainedGlyphStep::Fault => panic!("the pangram never faults the retained painter"),
            }
        }
        let glyphs: Vec<f32> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01).map(|instance| instance.rect[0]).collect();
        (glyphs, lines)
    };
    let (glyphs, _) = paint(&mut atlas, Rect::new(10.0, 0.0, 1_000.0, 40.0), RetainedTextFlow::Clip);
    assert_eq!(glyphs.len(), text.chars().count(), "one glyph per scalar");
    for ((byte, ch), x) in text.char_indices().zip(glyphs) {
        let caret = 10.0 + atlas.pen_at(text, byte, size) + atlas.ensure_glyph(ch, size).bearing_x;
        assert!((x - caret).abs() < 0.001, "{ch:?} at byte {byte} painted at {x}, its kerned caret is {caret}");
    }
    let first_line = "The quick brown fox jumps over ";
    let width = atlas.measure_text(first_line.trim_end(), size).0 + 0.01;
    let expected = atlas.wrap_lines(text, width, size);
    assert_eq!(expected.iter().map(|line| line.start).collect::<Vec<_>>(), [0, first_line.len()], "the kerned first line keeps `over`");
    let (_, lines) = paint(&mut atlas, Rect::new(0.0, 0.0, width, 400.0), RetainedTextFlow::Wrap);
    assert_eq!(lines.len(), text.chars().count(), "every scalar is stepped exactly once");
    for (byte, line) in lines {
        let assigned = expected.iter().position(|range| range.contains(&byte)).unwrap_or_else(|| panic!("byte {byte} lands on no measured line"));
        assert_eq!(line, assigned, "scalar at byte {byte} painted on line {line}, measured onto line {assigned}");
    }
}'''

TEXT_LAW_EDITS = [
    (
        '''    let at = |atlas: &mut FontAtlas, size: f32| atlas.measure_text("feature-editor", size).0;''',
        '''    let at = |atlas: &mut FontAtlas, size: f32| "feature-editor".chars().map(|ch| atlas.ensure_glyph(ch, size).advance).sum::<f32>();''',
    ),
    (T7A_CORPUS_LAW, T7B_LAWS),
]

MOUNTED_LAW_EDITS = [
    (
        '''fn composite_intrinsic_and_layout(tree: &mut UiTree, root: NodeId, width: f32) -> f32 {''',
        '''/// 🤝️ LAW (ticket 26/09/23 session 14d, WG11 T7b): the retained layout's text worker prices a run with the atlas's own pair
/// kerning — a hugging text node is exactly as wide as the atlas measures the run, never its unkerned advance sum.
#[test]
fn mounted_layout_prices_a_text_run_with_the_atlas_pair_kerning() {
    let value = "The quick brown fox jumps over the lazy dog";
    let mut tree = UiTree::new();
    let root = mount(&mut tree, None, 0, leaf(), stack_spec(Axis::Horizontal, Align::Start, Justify::Start, false));
    let text = mount(&mut tree, Some(root), 1, UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }), LayoutSpec::default());
    tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);
    assert!(crate::wgpu::mounted_layout::layout_tree_now(&mut tree, root, Theme::default(), 1_000.0, 400.0));
    let (_, _, width, _) = solved(&tree, text);
    let mut atlas = crate::wgpu::text::FontAtlas::shaped_default();
    let kerned = atlas.measure_text(value, DEFAULT_TEXT_SIZE_PX).0;
    let unkerned: f32 = value.chars().map(|ch| atlas.ensure_glyph(ch, DEFAULT_TEXT_SIZE_PX).advance).sum();
    assert!(unkerned - kerned > 0.5, "the pangram kerns: {unkerned} vs {kerned}");
    assert!(close(width, kerned), "the laid-out run is the atlas's kerned width: {width} vs {kerned}");
}

fn composite_intrinsic_and_layout(tree: &mut UiTree, root: NodeId, width: f32) -> f32 {''',
    ),
]


SLOT_BUDGET_EDITS = [
    (
        '''      "owner": "wgpu::engine::UiSurfaceRegistry",
      "capacityConstant": "UI_LAYOUT_SURFACE_SLOTS",
      "capacity": 64,
      "elementType": "Option<UiSurfaceSlot>",
      "elementSizeBytes": 164712,''',
        '''      "owner": "wgpu::engine::UiSurfaceRegistry",
      "capacityConstant": "UI_LAYOUT_SURFACE_SLOTS",
      "capacity": 64,
      "elementType": "Option<UiSurfaceSlot>",
      "elementSizeBytes": 165840,''',
    )
]


def replaced(path: Path, source: str, edits) -> str:
    for edit in edits:
        old, new, expected = (*edit, 1) if len(edit) == 2 else edit
        count = source.count(old)
        if count != expected:
            sys.exit(f"anchor occurs {count}x (expected {expected}) in {path.parent.name}/{path.name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


EDITED = {TEXT: TEXT_EDITS, PAINT: PAINT_EDITS, WIDGETS: WIDGET_EDITS, MOUNTED: MOUNTED_EDITS, TEXT_LAWS: TEXT_LAW_EDITS, MOUNTED_LAWS: MOUNTED_LAW_EDITS, SLOT_BUDGET: SLOT_BUDGET_EDITS}


def plans():
    planned = []
    for path, (landed, kerned) in REPLACED_FILES.items():
        current = path.read_text(encoding="utf-8") if path.exists() else None
        if current != (WORK / landed).read_text(encoding="utf-8"):
            sys.exit(f"{path.relative_to(ROOT)} is not T7a's landed file — T7a first (or T7b landed already)")
        planned.append((path, current, (WORK / kerned).read_text(encoding="utf-8")))
    for path, edits in EDITED.items():
        source = path.read_text(encoding="utf-8")
        planned.append((path, source, replaced(path, source, edits)))
    return planned


def main():
    if "--revert" in sys.argv:
        for path in [*REPLACED_FILES, *EDITED]:
            backup = BACKUP / path.relative_to(ROOT)
            if not backup.exists():
                sys.exit(f"no backup for {path.relative_to(ROOT)}")
            shutil.copyfile(backup, path)
        print("REVERTED: every file restored from its backup")
        return
    write = "--write" in sys.argv
    planned = plans()
    for path, before, after in planned:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=1))
    if write:
        for path, before, _ in planned:
            backup = BACKUP / path.relative_to(ROOT)
            backup.parent.mkdir(parents=True, exist_ok=True)
            backup.write_bytes(before.encode("utf-8"))
        for path, _, after in planned:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: {len(planned)} files (crates: semio-framework-ui, async slot-budget fixture; TS law: ui text corpus kerned column via Chromium)")


if __name__ == "__main__":
    main()
