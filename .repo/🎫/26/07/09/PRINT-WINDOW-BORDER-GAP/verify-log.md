# Print Window Border Gap

## Problem

Window title bars showed a small gap between the horizontal separator and the content box; left/right borders did not meet the label cap corners (see user screenshot).

## Root causes

1. Title cap used `fcolorbox` with a bottom border while the gap filler drew a separate bottom stroke at a different vertical position.
2. `\semio@heading@row@wrap` inserted `\vskip\semio@block@sep@skip` after window headers.
3. Header row and `tcolorbox` body were separate boxes, so side borders could not connect vertically.
4. Calling `\semio@window@header@muted` from `\ExplSyntaxOn` without `\use:c` / invoke wrapper broke `@` macros and caused TeX to hang in error recovery.

## Fix (`print/tex/semio-window.sty`)

- Refactored muted cap into `@core` (top stroke + fill), `@tab` (core + right stroke for window headers), and full `@muted` (left + core + right for paragraph headings).
- Window header row: extended left stroke spans title bar + separator; caps use `@tab`; gap uses fill-only `@inbox` variant; full-width bottom stroke below the row.
- `\semio@window@header@row@wrap` renders without trailing block separation vskip.
- `\semio_window_vskip_double_stroke_hairline:` pulls the body `tcolorbox` up to meet the separator.
- `\semio@window@header@invoke` + `\semio_window_header_muted_use:` bridge expl3 begins to LaTeX2e header macros safely.

## Spacing fix (title bar below content)

The title row was placed in an `\hbox` beside a tall `\vrule`/`\rule`, so TeX baseline-aligned the rule bottom with the `\vtop` top. The label caps were pushed into the box depth (below the baseline) while the `tcolorbox` body followed at the normal vertical position — values appeared above labels and the cover title was clipped.

Fix: wrap the full header in `\vtop{...}` without side-by-side `\vrule`; first cap uses `\semio@heading@cap@muted` (left stroke), remaining caps use `@tab`. Removed `\semio_window_vskip_double_stroke_hairline` (replaced with single hairline overlap).

Rebuild `forschungsbericht` or `zwischenbericht` (cover page with `Window` fields) and rasterize page 1:

```bash
cd print && bun run script.ts build forschungsbericht
cd .repo/🎫/26/07/09/PRINT-WINDOW-BORDER-GAP && bun rasterize.ts
```

During development, many concurrent `tectonic` processes caused builds to hang; run a single template build for visual confirmation.
