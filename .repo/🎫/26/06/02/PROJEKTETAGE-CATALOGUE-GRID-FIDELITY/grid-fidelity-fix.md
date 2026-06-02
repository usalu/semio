# Catalogue grid fidelity (cover only)

## Cause
Full catalogue used `<img object-fit: cover>` while split tiles used background crops with cover math that ignored bitmap aspect (normalized crop only). Wide `bauteilbörse.png` (1222×896) in an almost-square frame looked unlike the tile mosaic.

## Fix (no fill / no distort)
- Removed `fit: "fill"`.
- `FigureEmbodiment.sourceAspect` (width÷height); cover uses physical crop aspect `(crop.w/crop.h) * sourceAspect`.
- `CATALOGUE_FRAME` restored to hand-tuned `{ x: 0.127, y: 0.1, width: 0.746, height: 0.75 }`.
- Full catalogue embodiment: `crop: full image`, `fit: "contain"` (overview, not cropped sub-rectangle + cover).
- **`splitFigureGrid` crops are frame-relative** (sub-rectangle of the bitmap), not fractions of the full image — fixes Rippenplatte 1 left clip and overlap with Rippenplatte 2.
- Split tiles: `FigureEmbodiment.mosaic` includes `frame`; edge-aligned `background-position`; **uniform cover** (same `%` width and height — never stretch/distort).
- Full catalogue: `fit: "contain"` on full image in `CATALOGUE_FRAME`.
- Drag/resize: `PresentationFigureCropFrameContext` recomputes cover from the live frame so the bitmap is not squashed with the box.
- Mosaic alignment only when no `morphToFrame` (slide 7 grid); slide 8 focus cells use centered uniform cover.
- `revealMorphFromMorphToFrame` (catalogue morphTo grid slot) → `--presentation-figure-bg-grid-*` vars; 7→8 animates grid→focus via `presentation-figure-crop-morph-grid-to-focus`; slide 8 rest stays focus-centered; 8→9 still uses focus→label morph vars.

## Tests
- Renderer 93 passed; core 43 passed.
