# Silhouette guard: animals, appliances, components

Scope: all 34 assets of `🐕️animals`, `🔌️appliances`, `🧱️components` that failed at the start of this ticket (20 after the first pass of the readiness ticket). Guard `catalog-silhouette` unchanged. Every fix is an in-place edit (span editor `sil-span.py`/`mvlib.py`; the animal retraces also insert or replace shapes).

Result: all 34 assets have zero failing checks at every authored scale (triage `silhouette-triage.ts current`).

Detail per group, with causes, fixes and traps:
- animals A (red-fox, mallard-duck, domestic-cat, pigeon, hedgehog, labrador-dog, herring-gull): `animals-appliances-components-a-findings.md`
- animals B (roe-deer, pig, sheep, dairy-cow, goat, horse): `animals-appliances-components-b-findings.md`
- components C (masonry-wall-with-lintel, sliding-door, spiral-stair, sectional-garage-door, tied-canopy): `animals-appliances-components-c-findings.md`
- first pass (14 assets, server-rack to wheelchair-ramp): `silhouette-animals-appliances-components.md` in the readiness ticket.

## Fixed in this pass by the parent

| Asset | Cause | Side corrected |
|---|---|---|
| external-venetian-blind 1:100 and 1:200 plan | the volume guides were `slender` and no plan shape draws them, so the reference widened them (volume reached 70 mm beyond the 1.2 m box in z) | volume: `slender` false on `guide-left`/`guide-right` (1:100, 1:200); both are 30 mm bars inside the box footprint |
| external-venetian-blind 1:100 elevation | 17 drawn lamellae lines (13 mm stroke) against 1.2 mm volume sheets that the raster only partly keeps (some lines had no volume, ends 45 mm away) | drawing: lamellae role `detail` to `texture` (like `box-lid-seam`), the 13 mm stroke no longer counts as silhouette |
| shower-head 1:200 elevation IoU 0.905 | drawn hose was a smooth Bezier, the volume is the polyline of capsules (half a cell off along the whole 1.3 m) | drawing: hose `d` is the polyline through the volume capsule end points, IoU 0.986 |

## Traps (parent)

- A thin sheet in the volume (ry 0.0006) is invisible at 1:100 unless the raster hits it; thickening it (ry 0.0015 to 0.0065) fixed the elevation but made the front 49 mm off, so the drawing side (texture role) was the right fix.
- `slender: false` is a value edit; removing the key would need the comma of the previous line.
- `humans-try.ts` cannot measure 1:500 for assets without a 1:500 representation (shower-head, red-fox, domestic-cat): pass the scales explicitly.
- Do not rerun `animals-appliances-components-a-*` ops files: relative edits.
