## Root cause

Muted window/table chips were not using the same open-chip geometry as heading pairs (`\semio@window@cap@open`). A shorter side/canvas construction left a visible chip-bottom edge above the shared baseline, and painting chips in the tcolorbox overlay *plus* an inline/TikZ baseline produced parallel strokes.

## Fix

1. **`\semio@heading@cap@muted@open`** — same geometry as `\semio@window@cap@open`: shrink `titleh` by one hairline, then paint closed muted vbox + full-height side strokes at that height so the shared baseline is the sole bottom edge.

2. **`\semio@window@header@muted@core`** — same row layout as `\semio@heading@pair@row`: open chips in `\hbox to \linewidth` with `\hfil`, optional full-width baseline with `\nointerlineskip`.

3. **tcolorbox page 1** — `\semio@window@header@invoke@tcb` paints real chips+baseline inline (via `\semio@window@header@invoke`). Overlay page-1 only draws the U-frame (`sides_bottom`). `-5.75pt` seam welds `frame.north` onto that baseline (no second top hairline).

4. **Continuation pages** — overlay node still paints full `\semio@window@header@muted` (chips+baseline) on `frame.north`.

5. **Long tables** — unchanged path through `\semio@window@header@invoke` (inline baseline).

## Verification

- Probe `probe-chips.tex`: pixel grids at chip right edges match Probe heading pairs (sides → one baseline row → dark). Gap between chips has one bright row cluster.
- Zwischenbericht dark rebuild clean; scanned pages 1, 3, 12, 20, 21, 52, 90 — chip baselines are single clusters (~2px hairline + antialias), not parallel doubles.
- `bun run print/script.ts test` passes.

## Files

- `print/tex/semio-window.sty`
- `print/tex/semio-table.sty` (earlier companion fixes in this ticket)
