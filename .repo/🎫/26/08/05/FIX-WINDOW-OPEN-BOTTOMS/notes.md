# Fix Window Open Bottoms

## Problem

Breakable windows (`enhanced` tcolorbox) left first/middle page fragments open at the south edge: side rules hung into the footer with no closing hairline. Repro: Zwischenbericht Blockquote `I.TW.1.a` on PDF pages 86→87.

## Root cause

`enhanced` reserves `bottomrule at break` geometry but does **not** paint the horizontal. Overlay must draw it (same pattern as StackExchange / tcolorbox docs).

## Fix (`print/tex/semio-window.sty`)

- `toprule at break=0pt` — continuation chips own the top
- `bottomrule at break=\semio@stroke@hairline` — geometry for first/middle
- `\semio@window@frame@bottom@stroke` — TikZ `(frame.south west)--(frame.south east)` on `overlay first` / `overlay middle`
- Unbroken / last still use skin `bottomrule`

## Verification

- Probe `probe-open-bottom.tex`: typeout confirmed overlay first/middle fire; pixel scan shows full-width close above footer.
- Zwischenbericht dark rebuild: page 86 left frame ends at y≈2175 with span≈1197 hairline; footer rule separate at y≈2203. Page 87 last fragment closes at y≈797–799 (skin bottomrule).
- UdK footer chip: continuous bottom hairline in pixels (logo artwork has its own internal baseline; not a broken `\fcolorbox`).

## Files

- `print/tex/semio-window.sty`
