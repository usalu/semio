# Chip Seam Double Hairline / Page Gap

## Root cause

Muted window/table chips used `\semio@heading@cap@muted` (closed canvas geometry) and
tcolorbox page-1 omitted the inline baseline (`invoke@tcb` → `draw@baselinefalse`),
leaving only the TikZ overlay stroke at `frame.north`.

The seam vskip (`-hairline-5.75pt`) was calibrated for **chips + inline baseline**
height. Without the inline baseline, open chip bottoms floated ~5pt above
`frame.north`, so the overlay hairline sat parallel under the chips with page/cream
showing through (double hairline + gap). Longtable chrome was less affected when it
drew the baseline inline, but still used closed muted caps.

## Fix (`print/tex/semio-window.sty` only)

1. **`\semio@heading@cap@muted@open`** — rebuild like `\semio@heading@cap@nofill@open`:
   top + sides + canvas at `titleh−hairline` (no chip-local bottom).
2. **`\semio@window@header@muted@core`** — box chips with `@muted@open`.
3. **`\semio@window@header@invoke@tcb`** — paint chips **+ inline baseline** via
   `\semio@window@header@muted` (restore seam calibration).
4. **Overlay page 1** — keep `\semio_window_header_overlay_baseline:` so colback
   cannot cover the shared hairline; it coincides with the inline baseline after
   the seam weld.

## Pixel evidence

| Sample | Mid-gap hairline height | Verdict |
|--------|-------------------------|---------|
| BEFORE `FIX-TABLE-WINDOW-CHIP-BOTTOM-BORDERS/crop-markt.png` | **1.50pt** (2× hairline) | DOUBLE |
| AFTER `band-260-290.png` (Marktplätze) | **0.63pt** | SINGLE |
| AFTER `final-light-ueber.png` | **0.75pt** | SINGLE |
| AFTER side trace: canvas fill to baseline, one mid rule at weld | — | WELDED |

Probe: `probe-chips-light.tex` → `dist/probe-chips-light.pdf` + `-dark.pdf`.

## Macros touched

- `\semio@heading@cap@muted@open`
- `\semio@window@header@muted@core`
- `\semio@window@header@invoke@tcb`
- comments on `overlay~unbroken` / `overlay~first` / `WindowHeaderMuted` region
