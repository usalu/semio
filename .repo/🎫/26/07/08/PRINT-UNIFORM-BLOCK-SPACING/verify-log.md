# Print Uniform Block Spacing Verify

## Fix summary

Root cause: pre-chip `\vspace` inside `\titleformat` `[block]` never reached the vertical list between body text and the chip. The gap was trapped inside the title box.

Solution:
- **Before chip (2× single unit):** `\titlespacing*{#1}{0pt}{\semio@block@sep@before@skip}{...}` for all titlesec headings; `\semio@heading@block@before` uses the same skip for `SemioNest` and window blocks.
- **After chip (1× single unit):** `\titlespacing` after-sep and `\semio@heading@block@after` for nested paragraphs.
- Removed `block@sep@done` flag and internal `row@wrap` trailing vskip (single source of truth per edge).

Token: `semio@block@sep@before@skip = 2 × semio@spacing@single` (0.4em), `semio@block@sep@skip = semio@spacing@single` (0.2em).

## Raster checks

Page 6 raster: `report-p6-spacing.png` (Arbeitspakete / AP-Erfahrung / nested paragraphs)

Notable ink-band gaps on page 6 (scale 3, expected single unit ≈ 9.6px):
| Gap | px | em | note |
| --- | --- | --- | --- |
| 22 | 22 | 0.458 | ~2× single — body→chip block gap |
| 14 | 17 | 0.354 | block gap |
| 7 | 16 | 0.333 | block gap |
| 5–6 | 10 | 0.208 | ~1× single — chip→body |

Build: `bun ./script.ts build` in `mit-bestand/bericht` — zwischenbericht PDF OK.
