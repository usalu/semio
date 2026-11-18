# Silhouette guard: masonry-wall-with-lintel, sliding-door, spiral-stair, sectional-garage-door, tied-canopy

All five assets now have zero failing checks at every scale (`animals-appliances-components-c-tri.sh`), `validate-all.ts` reports none of them. Edits are span edits (`animals-appliances-components-c-lib.py`, scripts `animals-appliances-components-c-<asset>-N.py`); the guard is unchanged.

## masonry-wall-with-lintel
- 1:200 front/plan, 1:100 front/plan, 1:50 plan: the drawing lacked the volume's back leaf of sill and lintel (1:100/1:50) and drew sill and lintel only 0.34 m deep where the volume spans the full 0.445 m (1:200).
- Fix (drawing wrong): 1:200 lintel/sill/plan sill widened to 0.445; 1:100 added `lintel-back`, `sill-back`, plan `sill-back-below`; 1:50 plan added `sill-back-below`, `sill-back-plaster-below`.

## sliding-door
- 1:50 front: the volume seals were `slender` boxes (9 x 80 mm plates); a slender solid is drawn as a stroke widened to the followed shape (the 0.1 m jamb), so the volume grew to 0.14 m. Fix (volume): `slender` removed from `jamb-west-seal`, `jamb-east-seal` (the plates lie inside the jamb silhouette).
- 1:50 plan (section at 1.0 m): the drawing lacked everything the volume has visible below and cut at the plane: bottom rail below (z +-25 mm), cut handle base and grip. Fix (drawing): filled `leaf-rail-below`, `handle-base-cut`, `handle-grip-cut`.

## sectional-garage-door
- bbox z 2.54/2.53/2.52 against 2.5: bearing plates reached 1.2652/1.26 m and the shaft 1.255..1.27 m. Fix (volume): plates end at 1.25 m (200: 1.22..1.25, 100: 1.235..1.25, 50: 1.24..1.25), shaft +-1.25 m; drawn flags and shaft ends adjusted to the same values.
- 1:200 elevation IoU 0.766: drawn track pen 0.25 mm (50 mm) and panel pens 0.18/0.33 mm against 40 mm members. Fix (drawing): pens 0.2 mm.
- 1:100/1:50 plan: hinge and bracket blocks (four 0.12 m blocks per row at z -1.15, -0.383, 0.383, 1.15; 50: x 0.02..0.03 plus 0.015 end bits to 0.045) and the shaft ends (1.2..1.25 m) were not drawn; the missing shaft end left a 10 mm slit between drum and track that closed in the volume and not in the drawing (hole fill rule). Fix (drawing): `hinge-block-n`, `hinge-knuckle-a/b-n`, `shaft-end-left/right`.
- 1:50 elevation (section): hinge drawn as 0.1 x 0.02 m block, volume is a T (leaf plates 0.1 x 8 mm, 23 mm knuckle); fix drawing: T polygons. Bottom bracket below floor level (y -0.012): drawn `bottom-bracket` added. A filled shape's `penMm` does not change its outline (tried 0.1, no effect).
- 1:50 front: drawn track top 2.5 m, volume rail ribbon top 2.52 m: track rect height 2.52.
- Trap: plan shapes with a `solid` or a panel bone are in bone coordinates; new world-metre shapes were copied from `track-left` (bone `track`, no solid).

## tied-canopy
- 1:500 front H 0.356: the slender strut took the width of the 1.0 m wall strip (shapes are matched by bone `canopy`, a filled strip at least three times as long as wide wins on equal mean distance), so the volume strut was drawn 1 m wide. Fix (drawing): the drawn strut starts at its real foot (2.33 m, was 2.41), so its centre line covers the solid's and wins. Removing `slender` instead was tried and violates `volumes-feature` (strut 60 mm below 75 mm at 1:500): do not.
- 1:100 elevation: the drawing had no clevis plates and ended the strut band at the corners (0, 2.88) and (4, 0) while the volume tube ends at the pins; fix (drawing): strut band points to the tube ends, `clevis-wall`, `clevis-beam` polygons copied from 1:50.
- 1:50 plan: glass-2 has a notch 0.11 m wide around the clevis in the volume (0.055 m half width) that left slits to the clevis cheeks (hole fill rule); fix: notch 0.05 m in volume and drawing (drawn glass-2 as polygon with the notch), cheek plates `from 0.041` / `to -0.041` so that no 1 mm slit remains against the beam flange.

## spiral-stair
- bbox y 3.751 against 3.733 (1:100, 1:50): the handrail tube (radius 21 mm) tops at 3.754 m. Fix (declared dimension): `heightM` 3.75 (not used elsewhere).
- 1:200 elevation/front H 0.064: the 1:200 handrail loft had 19 sections (40 degree chords); fix (volume): the 36 sections of 1:100.
- 1:50 elevation/front/plan H 0.017..0.026: the centre line of a slender helix is estimated from the mesh and smoothed (inward bias) and stacked collinear slender balusters pick the wrong width. Fix (volume): `slender` removed from `handrail` and `baluster` at 1:50 (both thicker than the thin-solid limit of 1:50). Remaining 1:50 front H 0.017: the drawn handrail stroke has projecting caps, so its first and last points were moved in by half the pen (21.25 mm).
