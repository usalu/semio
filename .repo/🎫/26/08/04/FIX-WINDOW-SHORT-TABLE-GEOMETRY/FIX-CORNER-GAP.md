# Corner Gap Fix (Top × Side)

## Symptom
1-device-pixel cream notch where the chrome baseline meets body `border@L` / `border@R` (TOC “Seite” top-right; same class at top-left). Classic TeX butt-join of `\rule`/`\hrule` against `\vrule`.

## Fix (`print/tex/semio-window.sty`)
1. **Baseline corner stubs** — `\rlap`/`\llap` smashed `\vrule` stubs at both ends of the flush/non-flush baseline hbox, with depth into the next row so verticals overlap the top stroke.
2. **Open/closed chip post-and-rail** — muted/nofill open + nofill closed caps draw full-height L/R posts with the top (and bottom) rail between them (`cap@w`), not a full-`outer@w` top rule stacked above shorter verticals.

## QA (scale 8, light TOC p2)
- `corner-seite-R.png`: corner `(rightX,topY)` ink, horizontal right edge == vertical right edge.
- Left baseline band: same Y, left edge ink continuous into horizontal.
- Artifacts: `corner-seite-R.png`, `corner-toc-L3.png`, `corner-chip-TL.png`, `corner-gap.ts`.
