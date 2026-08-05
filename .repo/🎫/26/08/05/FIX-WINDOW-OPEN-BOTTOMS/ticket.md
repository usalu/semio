# Fix Window Open Bottoms

Breakable `enhanced` tcolorbox windows omit the south hairline on first/middle parts. Sides hang open above the footer (e.g. Blockquote on Zwischenbericht PDF p86).

## Fix
- `toprule at break=0pt` (continuation chips own the top)
- `bottomrule at break=\semio@stroke@hairline` (geometry)
- `\semio@window@frame@bottom@stroke` on `overlay first` / `overlay middle`

## Files
- `print/tex/semio-window.sty`
