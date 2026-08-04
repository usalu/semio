# Fix Manim Text Font

## Problem
Scenes used `Text.set_default(font="Serif")`. `"Serif"` is a CSS generic that Pango lists but is not a real installed face, so glyph coverage/metrics vary by machine.

## Fix
- Added `tutorial/manim_fonts.py` with `resolve_serif_font()` / `BODY_FONT` / `apply_body_font()`.
- Candidates: Georgia → PT Serif → Times New Roman → Liberation Serif → DejaVu Serif → STIX Two Text → …
- Replaced all `font="Serif"` defaults in Cooling, Heating german scenes, intro, and internal-gains shim.
- Updated generate-manim-tutorial skill docs.
- Restored accidentally missing `6_lueftungssysteme/scene_6.py` from Cursor local history, then applied the font fix.

## Verified
- `BODY_FONT` resolves to `Georgia` on this Mac.
- Smoke-rendered `HeatingVsCooling` and `Beat5_Luftfuehrung` at `-ql --disable_caching`.
