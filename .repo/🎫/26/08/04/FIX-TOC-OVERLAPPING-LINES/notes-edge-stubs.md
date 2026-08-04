# Edge stub fix (2026-08-04)

## Cause
1. `\semio@table@rule` used `\hrule width\linewidth`, ending past the centreline of side `\vrule`/tcolorbox strokes (~½ hairline), visible as stubs at every L/R join.
2. Windowed short tables also painted duplicate L/R borders (tcolorbox frame + per-row table `\vrule` + TikZ finish left stroke).

## Fix
- `semio-table.sty`: row rules are `\hbox to \linewidth` with a hairline inset by `0.5\arrayrulewidth` on each side; `\ifsemio@table@owns@sides` so windowed tables drop table-side `\vrule`s; window prepare sets owns-sides false.
- `semio-window.sty`: removed `finish={\semio_window_table_border_finish:}` from `semio~window~table`.

## Verification (pixel)
TOC / project / huerden / BB: `bad_overhang=0` on 10× right-edge crops.
