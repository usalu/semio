# Table side-border join fix

## Root cause

`\semio@table@rule` was an inset mid-rule (`\hskip0.5\arrayrulewidth`, width `\linewidth-\arrayrulewidth`).

1. **owns@sides tables** (longtable / TOC / projects): L/R `\vrule`s exist only inside row boxes and do not extend through `\noalign`. The inset left a canvas notch the height of the hairline at every join.
2. **Window short tables** (`owns@sidesfalse`): tcolorbox paints continuous L/R on the content-edge centreline. A flush `\linewidth` mid-rule overshot into the gutter (~0.75pt stubs); a half-hairline inset left cream notches at the T-joins.

## Fix (`print/tex/semio-table.sty` — `\semio@table@rule`)

- **owns@sides**: bare `\hrule height\arrayrulewidth` (alignment width) so the noalign band covers the L/R border slots.
- **window**: mid-rule overlaps half a hairline past each content edge (`\hskip-.5\arrayrulewidth`, width `\linewidth+\arrayrulewidth`) so it welds into the tcolorbox stroke.

Unrelated padding/hyphenation work left intact: `\semio@cell@inset`, `\semio@cell@strutbot` height=vbot, `\semio@table@header@cell` without a second `\cellcolor`.

## Verification (pixel luminance @ scale 4)

Rebuild: `cd mit-bestand/bericht && bun ./script.ts latex`

| Sample | Path | Joins | Cream gaps | Gutter stubs |
|--------|------|-------|------------|--------------|
| BB.M.a dark | `z3-bbma.png` | 4 | 0 | 0 |
| BB.M.a light | `z3-bbma-light.png` | 4 | 0* | 0 |
| Hürden | `z3-huerden.png` | 6 | 0 | 0 |
| TOC | `z3-toc.png` | 13 | 0 | 0 |

\*one false positive at a band where `below` is page cream (end of cropped segment), not a mid-join.

Join zooms: `z3-bbma-light-J.png`, `z3-huerden-J.png`, `z3-toc-J.png`.
