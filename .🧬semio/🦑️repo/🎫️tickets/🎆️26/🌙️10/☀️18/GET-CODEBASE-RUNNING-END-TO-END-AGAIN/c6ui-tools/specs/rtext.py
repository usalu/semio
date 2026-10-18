from specs.lib import *
F = U + '/🖌️render/🖋️text/🦀️.rs'
CH = [
    ch('🔤', 'font-roles-and-files', 'font_roles_and_files', [(44, 225)],
       'Font roles and files: the font families, the text font roles and set, the font errors and the native font files with their read errors.'),
    ch('🅰', 'text-styles-and-text-system', 'text_styles_and_text_system', [(227, 503)],
       'Text styles and text system: the family choice, text style, alignment and run style, the font handles, slots and status, the font source and fetch and the `TextSystem` font registry.'),
    ch('🧶', 'shaping-and-glyph-atlas', 'shaping_and_glyph_atlas', [(509, 865)],
       'Shaping and glyph atlas: the shape cache key, the shaped glyphs and text, the `TextSystem` shaping, the glyph entries and the atlas pages with glyph rasterization.'),
    ch('📏', 'measurement-and-utf16', 'measurement_and_utf16', [(873, 998)],
       'Measurement and utf16: the text measurement results, the `TextSystem` measure and wrap operations and the utf8 to utf16 offset conversions.'),
]
SPEC = dict(orig='rtext.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='rtext.rs'))
