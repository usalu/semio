#!/usr/bin/env python3
"""🔤️ WG11 session 14d — T7a (after T6): text ADVANCES at the exact logical×scale size, rasters on a quarter-pixel grid (coordinator
09:3x: "layout advances are computed at the exact logical×scale size from font units (never from a rounded size); rasterisation keys
the glyph cache at the exact device size on a fixed sub-pixel grid … law: a shared text corpus … within 0.5 px of Chromium";
T7a pins the UNKERNED column, T7b adds pair kerning and the kerned column).

Measured: `FontAtlas::quantize_size` rounded every device size to a whole pixel and the glyph (bitmap AND advance) came from that
rounded size — text-2xs (9.6 px) measured and painted as 10 px, text-xs (11.2) as 11: "feature-editor" read 65.4 px where Chromium's
unkerned DOM width is 62.75 (the GraphTimeline label track 83.8 vs React 80.9). Every wgpu text width sat up to ~4 % off React.

1. `📝️text`: the glyph cache is keyed on the device size on a quarter-pixel grid (`RASTER_GRID_STEPS_PER_PX`) and rasters at that
   grid size; every packed glyph records its advance per logical pixel of font size from the face's own units (`advance_em`), and
   `ensure_glyph_for` answers the entry for the EXACT requested size (`sized`, a bounded derived map) whose advance is
   `advance_em × size` — so measure, wrap and paint advance by the same exact widths while the atlas stays bounded.
   The retained layout's text worker (`📌️mounted_layout::DeterministicTextWorker`) priced every ASCII scalar at a fixed
   0.625 em and everything else at 1 em — a monospace stand-in, so a label laid out at one width and painted at another. It now
   reads the same font-unit advance (`text::font_advance_em`: authored face, emoji buckets, bitmap fallback) the atlas does.
   The shared section/field fixture's field box was sized so its details wrapped under that stand-in (112 px, 2 lines each);
   Chromium sets both on ONE line at 112 px (111.9 / 94.1 px — the first 0.06 px from its break) — the box is now 80 px, where
   Chromium wraps each onto two lines with 8 px to spare
   (WG11 probe `text/section-field-probe.ts`), so the law keeps asserting a wrap and the fixture is React's own truth. Its paint
   law painted with the fixed-pitch bitmap atlas a layout priced in the face's own advances; it paints with the atlas every host
   boots with (`FontAtlas::shaped_default`), the one that shares the layout's advance source. With exact advances the GraphTimeline
   checkpoint boundary lands on React's own (160.93 vs 160.89 px), so its inert boundary sample tightens to 161.25.
2. Laws (`🔬️targets-wgpu-text-unit`): the layout advance source IS the atlas advance; the cache key is the device size on the quarter-pixel grid (DPI law, updated); advances are
   linear in the exact size; the shared corpus — both faces, sizes 2xs…2xl, en + de strings — measures within 0.5 px of Chromium's
   UNKERNED DOM width. The corpus (`🖱️ui/🧫️fixtures/🔤️text-advances/🔣️.json`, schema `🧬️schema/🔤️text-advances`) is written by
   Chromium itself, and its React-side law (`🖱️ui/🧪️tests/🔤️text-advances/🟦️.ts`) re-measures every row in Chromium.
   Independent oracle (WG11 09:3x): the faces' `hmtx` advances × size / unitsPerEm match the corpus within 0.015 px on all 120 rows.

Dry run by default; `--write` backs every edited file up under `.🧬semio/🌐hub/s14-wg11-backup/text-advances/` (a new file's backup is
its absence) and applies; `--revert` restores.
"""

import difflib
import json
import shutil
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
WORK = Path("/Users/ueli" + "/Documents/semio/.tmp-ticket/wp-wg11/text")
UI = ROOT / "🧰️framework/🔨️modules/🖱️ui"
TEXT = UI / "🎯️targets/🧊️wgpu/📝️text/🦀️.rs"
TEXT_LAWS = UI / "🧪️tests/🔬️targets-wgpu-text-unit/🦀️.rs"
MOUNTED = UI / "🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs"
FIXTURE = UI / "🧫️fixtures/🔤️text-advances/🔣️.json"
SCHEMA = UI / "🧬️schema/🔤️text-advances/🔣️.json"
TS_LAW = UI / "🧪️tests/🔤️text-advances/🟦️.ts"
SECTION_FIELD = UI / "🧫️fixtures/📐️section-field-presentation/🔣️.json"
PAINT_LAWS = UI / "🧪️tests/🔬️targets-wgpu-paint-unit/🦀️.rs"
CHECKPOINT_FIXTURE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌳️GraphTimelineHost/🧫️fixtures/🎯️checkpoint-hit/🔣️.json"
VITEST_CONFIG = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts"
BACKUP = ROOT / ".🧬semio/🌐hub/s14-wg11-backup/text-advances"

NEW_FILES = {
    FIXTURE: (WORK / "text-advances.json").read_text(encoding="utf-8"),
    SCHEMA: (WORK / "text-advances-schema.json").read_text(encoding="utf-8"),
    TS_LAW: (WORK / "text-advances-law.ts").read_text(encoding="utf-8"),
}

TEXT_EDITS = [
    (
        '''const BITMAP_GLYPH_W: u32 = 8;
''',
        '''/// 📏️ One scalar's advance per logical pixel of font size, straight from the face's own units (`hmtx` / unitsPerEm) — the ONE
/// advance source both the retained layout's text worker (off the main thread, no atlas) and the atlas answer to: the authored
/// face, then the emoji buckets (the atlas's own fallback), then the bitmap fallback's fixed `0.625` em; a default-ignorable
/// format control advances by nothing.
pub(crate) fn font_advance_em(face: TextFace, ch: char) -> f32 {
    if is_zero_width_format_char(ch) {
        return 0.0;
    }
    let advance = |bytes: &'static [u8]| {
        let font = SwashFontRef::from_index(bytes, 0)?;
        let glyph = font.charmap().map(ch);
        let units = f32::from(font.metrics(&[]).units_per_em);
        (glyph != 0 && units > 0.0).then(|| font.glyph_metrics(&[]).advance_width(glyph) / units)
    };
    let authored = match face {
        TextFace::Sans => ANTA_LATIN,
        TextFace::Mono => SHARE_TECH_MONO_LATIN,
    };
    advance(authored).or_else(|| NOTO_EMOJI_BUCKETS.iter().find_map(|bytes| advance(bytes))).unwrap_or(BITMAP_ADVANCE_EM)
}

/// 📏️ The bitmap fallback's advance per logical pixel of font size.
const BITMAP_ADVANCE_EM: f32 = 0.625;

const BITMAP_GLYPH_W: u32 = 8;
''',
    ),
    (
        '''/// 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1g report for the whole unit model.
pub struct GlyphEntry {''',
        '''/// 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1g report for the whole unit model.
#[derive(Clone, Copy, Debug)]
pub struct GlyphEntry {''',
    ),
    (
        '''    pub advance: f32,
    pub bearing_x: f32,
    pub bearing_y: f32,
    /// 📐️ The atlas raster scale this glyph was rasterised at''',
        '''    pub advance: f32,
    /// 📏️ The advance per logical pixel of font size, from the face's own units — what [`FontAtlas::ensure_glyph_for`] scales to
    /// the EXACT requested size, so no layout ever sees the raster grid.
    pub advance_em: f32,
    pub bearing_x: f32,
    pub bearing_y: f32,
    /// 📐️ The atlas raster scale this glyph was rasterised at''',
    ),
    (
        '''    glyphs: HashMap<(TextFace, char, u32), GlyphEntry>,
''',
        '''    glyphs: HashMap<(TextFace, char, u32), GlyphEntry>,
    /// 📏️ Per EXACT device size (`f32` bits), the grid glyph with its advance at that size — derived, bounded by
    /// [`SIZED_GLYPH_CAPACITY`], rebuilt from `glyphs` on demand.
    sized: HashMap<(TextFace, char, u32), GlyphEntry>,
''',
    ),
    (
        '''        self.raster_scale = raster_scale;
        self.glyphs.clear();''',
        '''        self.raster_scale = raster_scale;
        self.glyphs.clear();
        self.sized.clear();''',
    ),
    (
        '''    /// 🔑️ Quantizes a float px size to the glyph-cache's integer key component, so float jitter
    /// (e.g. 15.999999 vs 16.0) doesn't fragment the cache into near-duplicate entries.
    fn quantize_size(size_px: f32) -> u32 {
        size_px.round().max(1.0) as u32
    }''',
        '''    /// 🔑️ The glyph-cache key of a DEVICE size: its position on the quarter-pixel raster grid (`RASTER_GRID_STEPS_PER_PX`
    /// steps per pixel), so float jitter never fragments the cache and a raster is never more than an eighth of a pixel
    /// off the size it paints. Only rasters snap: advances stay exact ([`GlyphEntry::advance_em`]).
    fn quantize_size(size_px: f32) -> u32 {
        (size_px * RASTER_GRID_STEPS_PER_PX).round().max(1.0) as u32
    }

    /// 📐️ The device size one grid key rasterises at.
    fn grid_px(key: u32) -> f32 {
        key as f32 / RASTER_GRID_STEPS_PER_PX
    }''',
    ),
    (
        '''    pub fn ensure_glyph_for(&mut self, face: TextFace, ch: char, size_px: f32) -> &GlyphEntry {
        let key = (face, ch, Self::quantize_size(size_px * self.raster_scale));
        if !self.glyphs.contains_key(&key) {
            self.rasterize_glyph(key);
        }
        self.glyphs.get(&key).expect("glyph inserted")
    }''',
        '''    /// The raster comes from the quarter-pixel grid; the answered entry's `advance` is the face's advance at the EXACT
    /// `size_px` (`advance_em × size_px`) — the one width measure, wrap and paint all pen by.
    pub fn ensure_glyph_for(&mut self, face: TextFace, ch: char, size_px: f32) -> &GlyphEntry {
        let device = size_px * self.raster_scale;
        let exact = (face, ch, device.to_bits());
        if !self.sized.contains_key(&exact) {
            let key = (face, ch, Self::quantize_size(device));
            if !self.glyphs.contains_key(&key) {
                self.rasterize_glyph(key);
            }
            let mut glyph = *self.glyphs.get(&key).expect("glyph inserted");
            glyph.advance = glyph.advance_em * size_px;
            if self.sized.len() >= SIZED_GLYPH_CAPACITY {
                self.sized.clear();
            }
            self.sized.insert(exact, glyph);
        }
        self.sized.get(&exact).expect("sized glyph inserted")
    }''',
    ),
    (
        '''        if let Some(glyph) = symbol_face_path(ch).and_then(|path| self.rasterize_symbol_glyph(path, device_size_px as f32)) {
            self.pack_glyph(key, glyph);
            return;
        }
        let glyph = match self.mode {
            AtlasMode::Bitmap => self.rasterize_bitmap_glyph(ch, device_size_px as f32),
            AtlasMode::Shaped => self.rasterize_shaped_glyph(face, ch, device_size_px as f32),
        };''',
        '''        let raster_px = Self::grid_px(device_size_px);
        if let Some(glyph) = symbol_face_path(ch).and_then(|path| self.rasterize_symbol_glyph(path, raster_px)) {
            self.pack_glyph(key, glyph);
            return;
        }
        let glyph = match self.mode {
            AtlasMode::Bitmap => self.rasterize_bitmap_glyph(ch, raster_px),
            AtlasMode::Shaped => self.rasterize_shaped_glyph(face, ch, raster_px),
        };''',
    ),
    (
        '''        self.glyphs.insert(key, GlyphEntry { atlas_x, atlas_y, width, height, advance, bearing_x, bearing_y, raster_scale, is_color });''',
        '''        let logical_size = Self::grid_px(key.2) / self.raster_scale;
        let advance_em = if logical_size > 0.0 { advance / logical_size } else { 0.0 };
        self.glyphs.insert(key, GlyphEntry { atlas_x, atlas_y, width, height, advance, advance_em, bearing_x, bearing_y, raster_scale, is_color });''',
    ),
    (
        '''const FAMILY_SANS: &str = "Anta";''',
        '''/// 🔤️ Raster grid steps per device pixel — glyph bitmaps snap to a quarter pixel, advances never do.
const RASTER_GRID_STEPS_PER_PX: f32 = 4.0;
/// 🔤️ Exact-size glyph entries kept before the derived map is rebuilt — a continuous zoom must not grow it without bound.
const SIZED_GLYPH_CAPACITY: usize = 16_384;

const FAMILY_SANS: &str = "Anta";''',
    ),
]

CONSTRUCTOR_EDIT = (
    '''            glyphs: HashMap::new(),
            raster_scale: 1.0,''',
    '''            glyphs: HashMap::new(),
            sized: HashMap::new(),
            raster_scale: 1.0,''',
    2,
)

TEXT_LAW_EDITS = [
    (
        '''    atlas.pack_glyph((super::TextFace::Sans, '🔥', 32), super::RasterizedGlyph {''',
        '''    atlas.pack_glyph((super::TextFace::Sans, '🔥', 128), super::RasterizedGlyph {''',
    ),
    (
        '''/// 🔑️ The glyph cache is keyed on the DEVICE size, so 16 logical px at 2x and 32 logical px at 1x
/// are distinct rows even though both rasterise 32 device px — otherwise a scale change would hand
/// back a correctly-sized raster with the WRONG logical metrics.
#[test]
fn the_glyph_cache_key_is_the_device_size_not_the_logical_one() {
    let mut atlas = FontAtlas::builtin();
    atlas.set_raster_scale(2.0);
    atlas.ensure_glyph('A', 16.0);
    assert_eq!(atlas.glyphs.len(), 1);
    assert!(atlas.glyphs.contains_key(&(super::TextFace::Sans, 'A', 32)), "16 logical px at 2x must be cached under its 32 device px key");
    atlas.ensure_glyph('A', 16.0);
    assert_eq!(atlas.glyphs.len(), 1, "the same logical size must hit the same cache row");
}''',
        '''/// 🔑️ The glyph cache is keyed on the DEVICE size on the quarter-pixel raster grid, so 16 logical px at 2x (32 device px =
/// 128 grid steps) is its own row, a fractional size rasterises on the nearest quarter pixel, and one grid row serves every
/// exact size that snaps to it — while each size's advance stays exact.
#[test]
fn the_glyph_cache_key_is_the_device_size_on_the_quarter_pixel_grid() {
    let mut atlas = FontAtlas::builtin();
    atlas.set_raster_scale(2.0);
    atlas.ensure_glyph('A', 16.0);
    assert_eq!(atlas.glyphs.len(), 1);
    assert!(atlas.glyphs.contains_key(&(super::TextFace::Sans, 'A', 128)), "16 logical px at 2x is cached under 32 device px = 128 quarter-pixel steps");
    atlas.ensure_glyph('A', 16.0);
    assert_eq!(atlas.glyphs.len(), 1, "the same logical size must hit the same cache row");
    atlas.set_raster_scale(1.0);
    let wide = atlas.ensure_glyph('A', 9.6).advance;
    let narrow = atlas.ensure_glyph('A', 9.55).advance;
    assert!(atlas.glyphs.contains_key(&(super::TextFace::Sans, 'A', 38)), "9.6 device px rasterises on the 9.5 px grid step");
    assert_eq!(atlas.glyphs.len(), 1, "9.55 px snaps to the same raster row");
    assert!(wide > narrow, "yet each exact size keeps its own advance: {wide} vs {narrow}");
}

/// 📏️ LAW (ticket 26/09/23 session 14d, WG11 T7a): an advance is the face's own advance at the EXACT size — linear in it, never
/// read off a rounded raster size (9.6 px measured as 10 px before: text-2xs sat 4 % wide of React).
#[test]
fn advances_are_exact_at_fractional_sizes() {
    let mut atlas = FontAtlas::shaped_default();
    let at = |atlas: &mut FontAtlas, size: f32| atlas.measure_text("feature-editor", size).0;
    let (small, double) = (at(&mut atlas, 9.6), at(&mut atlas, 19.2));
    assert!((double - 2.0 * small).abs() < 0.01, "an advance scales linearly with the exact size: {small} vs {double}");
    assert!((small - 62.75).abs() < 0.05, "Anta's own units put 'feature-editor' at 62.75 px at 9.6 px: {small}");
}

/// 📏️ LAW (WG11 T7a): the retained layout's text worker and the atlas pen by ONE advance source — `font_advance_em` from the
/// face's units — so a label lays out exactly as wide as it paints.
#[test]
fn the_layout_advance_source_is_the_atlas_advance() {
    let mut atlas = FontAtlas::shaped_default();
    for (face, text) in [(super::TextFace::Sans, "Einstellungen · Größe 1.2"), (super::TextFace::Mono, "fn main() { 42 }")] {
        for ch in text.chars() {
            let painted = atlas.ensure_glyph_for(face, ch, 11.2).advance;
            let laid_out = super::font_advance_em(face, ch) * 11.2;
            assert!((painted - laid_out).abs() < 0.001, "{face:?} {ch:?}: atlas {painted} vs layout {laid_out}");
        }
    }
}

/// 🔤️ LAW (WG11 T7a): the shared corpus — both faces, sizes 2xs…2xl, en + de strings — measures within the fixture's tolerance
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
}''',
    ),
]


MOUNTED_EDITS = [
    (
        '''        let advance = if input.scalar.is_ascii() { DEFAULT_TEXT_SIZE_PX * 0.625 } else { DEFAULT_TEXT_SIZE_PX };''',
        '''        let advance = crate::wgpu::text::font_advance_em(crate::wgpu::text::TextFace::Sans, input.scalar) * DEFAULT_TEXT_SIZE_PX;''',
    )
]

SECTION_FIELD_EDITS = [
    (
        '''"what": "React Section and Field compact-density text bands shared with the retained WGPU renderer.",''',
        '''"what": "React Section and Field compact-density text bands shared with the retained WGPU renderer — every line count is Chromium's own wrap of the Anta text at its box width.",''',
    ),
    (
        '''    "width": 112.0,''',
        '''    "width": 80.0,''',
    ),
]

PAINT_LAW_EDITS = [
    (
        '''    let mut atlas = FontAtlas::builtin();
    assert!(crate::wgpu::mounted_layout::layout_tree_now(&mut section_tree, section_root, theme, number(&["section", "width"]), 400.0));''',
        '''    let mut atlas = FontAtlas::shaped_default();
    assert!(crate::wgpu::mounted_layout::layout_tree_now(&mut section_tree, section_root, theme, number(&["section", "width"]), 400.0));''',
    ),
    (
        '''    let mut field_atlas = FontAtlas::builtin();''',
        '''    let mut field_atlas = FontAtlas::shaped_default();''',
    ),
]

CHECKPOINT_EDITS = [
    (
        '''    { "id": "description-boundary-inert", "region": "description", "row": 1, "x": 162.5, "action": null },''',
        '''    { "id": "description-boundary-inert", "region": "description", "row": 1, "x": 161.25, "action": null },''',
    )
]

VITEST_EDITS = [
    (
        '''  uiSuite("🔝️navbar-centered-band"),
''',
        '''  uiSuite("🔝️navbar-centered-band"),
  uiSuite("🔤️text-advances"),
''',
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


def plans():
    for path in NEW_FILES:
        if path.exists():
            sys.exit(f"{path.relative_to(ROOT)} exists already — landed already")
    json.loads(NEW_FILES[FIXTURE])
    json.loads(NEW_FILES[SCHEMA])
    text = TEXT.read_text(encoding="utf-8")
    laws = TEXT_LAWS.read_text(encoding="utf-8")
    config = VITEST_CONFIG.read_text(encoding="utf-8")
    mounted = MOUNTED.read_text(encoding="utf-8")
    section_field = SECTION_FIELD.read_text(encoding="utf-8")
    paint_laws = PAINT_LAWS.read_text(encoding="utf-8")
    checkpoint = CHECKPOINT_FIXTURE.read_text(encoding="utf-8")
    return [(path, None, content) for path, content in NEW_FILES.items()] + [
        (TEXT, text, replaced(TEXT, text, TEXT_EDITS + [CONSTRUCTOR_EDIT])),
        (TEXT_LAWS, laws, replaced(TEXT_LAWS, laws, TEXT_LAW_EDITS)),
        (MOUNTED, mounted, replaced(MOUNTED, mounted, MOUNTED_EDITS)),
        (VITEST_CONFIG, config, replaced(VITEST_CONFIG, config, VITEST_EDITS)),
        (SECTION_FIELD, section_field, replaced(SECTION_FIELD, section_field, SECTION_FIELD_EDITS)),
        (PAINT_LAWS, paint_laws, replaced(PAINT_LAWS, paint_laws, PAINT_LAW_EDITS)),
        (CHECKPOINT_FIXTURE, checkpoint, replaced(CHECKPOINT_FIXTURE, checkpoint, CHECKPOINT_EDITS)),
    ]


def main():
    if "--revert" in sys.argv:
        for path in NEW_FILES:
            if path.exists():
                path.unlink()
        for path in (TEXT, TEXT_LAWS, MOUNTED, VITEST_CONFIG, SECTION_FIELD, PAINT_LAWS, CHECKPOINT_FIXTURE):
            backup = BACKUP / path.relative_to(ROOT)
            if not backup.exists():
                sys.exit(f"no backup for {path.relative_to(ROOT)}")
            shutil.copyfile(backup, path)
        print("REVERTED: new files removed, edited files restored from backups")
        return
    write = "--write" in sys.argv
    planned = plans()
    for path, before, after in planned:
        sys.stdout.writelines(difflib.unified_diff((before or "").splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=1))
    if write:
        for path, before, _ in planned:
            if before is not None:
                backup = BACKUP / path.relative_to(ROOT)
                backup.parent.mkdir(parents=True, exist_ok=True)
                backup.write_bytes(before.encode("utf-8"))
        for path, _, after in planned:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(after, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: {len(NEW_FILES)} new + 7 edited files (crates: semio-framework-ui, os-renderer-wgpu GraphTimeline fixture; TS law: ui text corpus via Chromium)")


if __name__ == "__main__":
    main()
