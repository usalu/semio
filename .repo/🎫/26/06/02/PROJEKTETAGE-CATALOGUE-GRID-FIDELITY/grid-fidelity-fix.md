# Catalogue grid fidelity

## Cause
Bauteilkatalog showed the full `bauteilbörse.png` via `<img object-fit: cover>` while split tiles use CSS background crops stretched per cell. Image aspect (~1.36) vs `CATALOGUE_FRAME` (~1.0) made cover crop the catalogue; the 3×5 grid no longer matched the PNG.

## Fix
- `FigureEmbodiment.fit`: `"cover"` | `"fill"`.
- Full catalogue: `crop: {0,0,1,1}`, `fit: "fill"` (background slot, 100%×100% in frame).
- Split `tile()` defaults to `fit: "fill"` so cells match the mosaic.

## Tests
- Renderer 93 passed; core 43 passed.
