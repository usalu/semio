## Root cause

Muted title/number chips are open-bottom tabs: one shared hairline baseline closes both chips (not per-chip bottom borders in the gap).

## Fix (single line, no double stroke)

1. **tcolorbox windows** (`Table` + `SemioTable`, `Figure`, `Window`):
   - Inline: `\semio@window@header@invoke@tcb` (chips only, `\semio@window@header@draw@baselinefalse`).
   - Overlay page 1: `\semio_window_header_overlay_baseline:` draws one line at `frame.north~west`–`frame.north~east` (must use `~` in anchor names; `frame.north west` broke the build).
   - Overlay continuation: full `\semio@window@header@muted` (with baseline) in a `\node` at `frame.north`.
   - Gap spacer `\semio@window@header@gap@open` no longer draws a mid stroke; full-width baseline row in `\semio@window@header@muted@core` supplies the bottom edge for both chips.

2. **Long tables** (`SemioTableLong`, TOC, glossary, `\SemioProject`):
   - `\semio@table@long@title@chrome@row` uses `\semio@window@header@invoke` (baseline inline).
   - No extra `\hrule` after title chrome (was parallel line with invoke baseline).

## Build status

- Print unit tests: pass (overlay hooks assert `semio_window_header_overlay_baseline:`).
- Full `zwischenbericht` build: still fails at `\begin{SemioTable}{0.07,…}` (window short-table colspec / preamble); chip overlay changes compile through cover + TOC. Separate from chip baseline logic.

## Files

- `print/tex/semio-window.sty`
- `print/tex/semio-table.sty` (long title chrome; window table colspec work in progress)
- `print/script.ts` (test assertions)
