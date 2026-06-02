# Catalogue grid fidelity (cover only)

## Cause
Full catalogue used `<img object-fit: cover>` while split tiles used background crops with cover math that ignored bitmap aspect (normalized crop only). Wide `bauteilbörse.png` (1222×896) in an almost-square frame looked unlike the tile mosaic.

## Fix (no fill / no distort)
- Removed `fit: "fill"`.
- `FigureEmbodiment.sourceAspect` (width÷height); cover uses physical crop aspect `(crop.w/crop.h) * sourceAspect`.
- `CATALOGUE_FRAME` height = width / sourceAspect, vertically centered in the former slot.
- Full catalogue + split tiles share background cover with `CATALOGUE_SOURCE_ASPECT`.
- Split tiles: `FigureEmbodiment.mosaic` + edge-aligned `background-position` (0%, 25%, …) instead of crop-center (10%, 30%, …) so adjacent cells do not overlap.

## Tests
- Renderer 93 passed; core 43 passed.
