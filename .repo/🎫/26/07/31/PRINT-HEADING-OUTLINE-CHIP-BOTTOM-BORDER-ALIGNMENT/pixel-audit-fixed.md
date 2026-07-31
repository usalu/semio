# Fixed Outline Chip Audit

## Root cause

`\dimexpr\semio@window@cap@w+2\semio@stroke@hairline\relax` with
`\semio@stroke@hairline` → `0.75pt` token-glued to `20.75pt` instead of
`1.5pt`. Top/bottom strokes were ~19.25pt wider than the mid row (verticals +
label), so the shared `\linewidth` baseline (and chip tops) overshot the
visible right vertical (Bauteilportal).

## Fix

`\semio@window@cap@outer@w` = `cap@w + hairline + hairline` (sum, no factor).
Used by closed and open outline chip frames. Needspace keep also uses
hairline+hairline / bodypad+bodypad (same gluing class).

## Raster check (probe-p2.png, scale 4)

### Bauteilportal

- leftDelta=0, rightDelta=0
- title topSeg matches left/right verticals
- BR corner ASCII: vertical column meets baseline with no `#` past the outer edge

### Bauteilbörsen

- leftDelta=0, rightDelta=0
- titleTopMatches=true
