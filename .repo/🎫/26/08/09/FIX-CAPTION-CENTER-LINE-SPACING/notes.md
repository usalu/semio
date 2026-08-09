# Notes

## Root cause
- `caption_bar` auto-scaled when width > `CAPTION_MAX_WIDTH` → uneven subtitle sizes across beats.
- Multi-line `\n` via a single Manim `Text` → lines left-aligned inside the SVG; Georgia vertical metrics looked cramped and drifted whenever scenes were rewritten instead of fixing the helper.

## Fix
- `centered_body_text()` in `manim_fonts.py`: one `Text` per line, `arrange(DOWN, center=True)` with `BODY_LINE_BUFF` / `CAPTION_LINE_BUFF`.
- `caption_bar` uses that helper, always `CAPTION_FONT_SIZE`, warns (does not shrink) if a line is too wide.
- `apply_body_font()` also sets `Text` default `line_spacing=BODY_LINE_SPACING` (0.75) for any remaining single-`Text` multi-line labels.
- `scene_title` / `beat_subtitle` go through the same center-aligned builder.

## Smoke check (2026-08-09)
- Two-line caption: both lines `x=0`, different widths, group `x=0`.
- Long caption width matches unscaled reference at `CAPTION_FONT_SIZE`.
- `beat_subtitle` two-line: both lines `x=0`.
