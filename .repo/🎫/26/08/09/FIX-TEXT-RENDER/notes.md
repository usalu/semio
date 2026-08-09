# Notes

## What looked broken (Scene4 1080p frames)
1. Long German captions overflowed left/right after auto-shrink was removed.
2. `final_calculation/merged_scenes.py` never called `apply_scene_style` and pointed at missing `"Computer Modern"`.
3. `Write()` on labels (`Überhitzungsbereich`, formulas) produced spaced/ghosted glyphs.
4. Pango ligatures on Georgia crushed German clusters (`ämm`, `ss`, `ch`).

## Fix
- `body_text()` + `Text.set_default(..., disable_ligatures=True)` in `apply_body_font`.
- `caption_bar` word-wraps via `wrap_body_lines` / `centered_body_text(max_width=CAPTION_MAX_WIDTH)`.
- Shared helpers (`equation_row`, `chip`, `watt_anchor`, …) use `body_text`.
- `final_calculation/merged_scenes.py`: `apply_scene_style`, `font=BODY_FONT`, `FadeIn` instead of `Write` on non-title text.
- Dropped dead `FONT_NAME = "Computer Modern"` in Heating/4 merged as well.

## Verify
- Caption smoke: long Eta-h clause wraps, width < 12.2, line xs = 0.
- `manim -ql --disable_caching merged_scenes.py Scene4` — captions on-screen, labels clean at rest.
