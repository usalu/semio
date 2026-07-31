# Fisheye projection quality

## Symptoms
Curvilinear/fisheye view looked pixelated and had wrong colors vs other projections.

## Root cause
`WorldCurvilinearPass` blit path:
1. **Pixelation** — capture RT used `magFilter: NearestFilter` and was sized in CSS pixels (no `devicePixelRatio`), so the warped UV sample looked blocky on retina and under distortion.
2. **Wrong colors** — blit `ShaderMaterial` wrote linear capture texels to the sRGB canvas without `#include <colorspace_fragment>`, and did not mark `toneMapped: false`. Capture now also declares `LinearSRGBColorSpace`.

## Fix
- Linear min/mag filters + DPR-sized `WebGLRenderTarget`
- `colorSpace: LinearSRGBColorSpace` on capture
- Blit shader applies `colorspace_fragment`; material `toneMapped: false`
- Aspect from CSS `size` (ratio-stable vs DPR)
