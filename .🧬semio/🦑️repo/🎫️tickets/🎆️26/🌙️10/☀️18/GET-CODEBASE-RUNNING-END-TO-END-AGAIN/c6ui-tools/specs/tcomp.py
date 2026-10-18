from specs.lib import *
F = U + '/🔤️typography/🧩️component/🦀️.rs'
H = 'impl FontAtlas {'
B = [811, 975, 1111]
CH = [
    ch('🅰', 'glyph-entry-and-text-weight', 'glyph_entry_and_text_weight', [(28, 138)],
       'Glyph entry and text weight: the packed `GlyphEntry`, the text weight and the faux medium and bold offsets.'),
    ch('🔠', 'text-faces-and-kerning', 'text_faces_and_kerning', [(142, 270)],
       'Text faces and kerning: the raster grid and capacity limits, the font families, the text face, the emoji buckets, the font advance, the authored face bytes and the pair kerning.'),
    ch('✂', 'line-breaking', 'line_breaking', [(276, 409)],
       'Line breaking: the fit epsilon, the wrap spaces, the no-break classes and the break opportunities and unbreakable runs.'),
    ch('🔣', 'bitmap-and-symbol-faces', 'bitmap_and_symbol_faces', [(413, 574)],
       'Bitmap and symbol faces: the builtin bitmap font, the symbol face outlines and the zero-width format characters.'),
    ch('🗂', 'font-atlas-state-and-setup', 'font_atlas_state_and_setup', [(578, 811)],
       'Font atlas state and setup: the atlas mode, the rasterized and resolved glyphs, the `FontAtlas`, its builtin and font-backed construction, dirty flags, shaping and the raster scale.'),
    ch('🖼', 'glyph-rasterization', 'glyph_rasterization', [(813, 978)],
       'Glyph rasterization: the glyph ensure, the swash rasterization, the symbol, single char and resolved glyph rendering and the bitmap glyphs.'),
    ch('📏', 'glyph-packing-measure-and-kerning', 'glyph_packing_measure_and_kerning', [(980, 1117)],
       'Glyph packing, measure and kerning: the atlas packing, the text and range measurement, the kerning lookups and the pen advance.'),
    ch('🧵', 'text-wrapping-and-font-fetch', 'text_wrapping_and_font_fetch', [(1119, 1259)],
       'Text wrapping and font fetch: the pre-wrapped and wrapped lines, the wrapped measurement and the font byte fetch.'),
]
SPEC = dict(orig='tcomp.rs', file=F, children=CH, wiring='block', pre=split_impl(H, B), expect_extra=split_extra(H, B), external=mounted(F, orig='tcomp.rs'))
