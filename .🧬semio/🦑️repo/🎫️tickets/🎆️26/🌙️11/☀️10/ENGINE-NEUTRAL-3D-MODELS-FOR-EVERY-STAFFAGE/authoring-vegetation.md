# Authoring: vegetation volumes

All 16 vegetation assets have a spliced, validated, authored `volume` (sources `T/volumes/vegetation/<slug>.json`, generators `<slug>.author.ts`, kit `veg-lib.ts`, checks `veg-validate.ts`, `vegetation-tools.ts`, `veg-finish.sh`). Every loft section carries an explicit `rotation` (coordinator amendment 2; all lofts re-checked with `volume-check`). Splice with `volume-insert.py` only.

## Table (1:50, rest pose for IoU; bbox error % x / y / z against `dimensions`; "default pose" = bbox in `presentation.defaultPose` per amendment 6)

| slug | solids | bbox err % rest | bbox err % default pose | IoU elev / front / plan | validator issues | poses and clips evaluated (worst joint gap) | gates |
|---|---|---|---|---|---|---|---|
| agave | 20 | +1.0 / -6.4 / -89.6 | -0.9 / -6.4 / -1.3 | 0.93 / 0.29 / 0.27 | 0 (schema 0, +874 on capital bone ids) | 11 (0.009 m) | FAIL: bbox z (depthM): -89.6 % > 8 %; front: IoU 0.288 < 0.80; plan: IoU 0.269 < 0.80 |
| apple-tree | 40 | +0.2 / -0.0 / +0.0 | 0.3 / -0.0 / 0.1 | 0.90 / 0.85 / 0.92 | 0 (schema 0, +0 on capital bone ids) | 13 (0.007 m) | PASS |
| bamboo-clump | 30 | -4.6 / +0.1 / -4.8 | -4.4 / 0.1 / -4.3 | 0.48 / 0.68 / 0.87 | 0 (schema 0, +0 on capital bone ids) | 11 (0.017 m) | FAIL: elevation: IoU 0.484 < 0.80; front: IoU 0.680 < 0.80 |
| date-palm | 43 | +13.0 / -0.1 / +1.7 | 11.3 / -0.1 / 0.4 | 0.47 / 0.46 / 0.18 | 0 (schema 0, +0 on capital bone ids) | 12 (0.007 m) | FAIL: bbox x (widthM): 13.0 % > 8 %; elevation: IoU 0.467 < 0.80; front: IoU 0.456 < 0.80; plan: IoU 0.181 < 0.80 |
| english-oak | 37 | +0.2 / +0.1 / +0.0 | -0.5 / 0.2 / -0.9 | 0.87 / 0.89 / 0.91 | 0 (schema 0, +0 on capital bone ids) | 13 (0.011 m) | PASS |
| hornbeam-hedge | 24 | +0.0 / +2.5 / +0.0 | 0.0 / 2.5 / 0.0 | 0.94 / 0.87 / 0.98 | 0 (schema 0, +0 on capital bone ids) | 13 (0.002 m) | PASS |
| hosta-clump | 27 | +3.3 / -5.2 / -69.4 | 1.4 / -5.2 / -1.0 | 0.81 / 0.88 / 0.88 | 0 (schema 0, +1260 on capital bone ids) | 10 (0.002 m) | FAIL: bbox z (depthM): -69.4 % > 8 % |
| ivy-on-trellis | 21 | +1.0 / -0.5 / +0.0 | 1.0 / -0.5 / 0.0 | 0.90 / 0.92 / 0.82 | 0 (schema 0, +138 on capital bone ids) | 11 (0.024 m) | PASS |
| japanese-maple | 50 | -0.5 / -0.1 / +0.2 | -0.4 / -0.1 / 0.4 | 0.85 / 0.91 / 0.95 | 0 (schema 0, +0 on capital bone ids) | 14 (0.007 m) | PASS |
| lombardy-poplar | 62 | -5.0 / -1.9 / +6.2 | -4.7 / -1.9 / 6.0 | 0.80 / 0.85 / 0.92 | 0 (schema 0, +0 on capital bone ids) | 12 (0.075 m) | PASS |
| london-plane-pollarded | 57 | +0.3 / -0.0 / +3.6 | 0.4 / -0.0 / 3.6 | 0.90 / 0.94 / 0.93 | 0 (schema 0, +0 on capital bone ids) | 14 (0.038 m) | PASS |
| miscanthus-grass | 49 | -0.6 / -1.9 / +0.8 | -0.6 / -1.9 / 0.8 | 0.55 / 0.41 / 0.29 | 0 (schema 0, +1440 on capital bone ids) | 13 (0.015 m) | FAIL: elevation: IoU 0.550 < 0.80; front: IoU 0.406 < 0.80; plan: IoU 0.294 < 0.80 |
| multi-stem-birch | 53 | -3.7 / -0.5 / -3.4 | -3.5 / -0.5 / -3.1 | 0.81 / 0.92 / 0.86 | 0 (schema 0, +0 on capital bone ids) | 13 (0.014 m) | PASS |
| scots-pine | 47 | -3.8 / -0.4 / -4.6 | -4.0 / -0.4 / -4.6 | 0.83 / 0.84 / 0.82 | 0 (schema 0, +0 on capital bone ids) | 13 (0.006 m) | PASS |
| small-leaved-lime | 76 | -0.2 / -0.0 / +0.6 | -0.2 / -0.1 / 0.5 | 0.91 / 0.93 / 0.96 | 0 (schema 0, +0 on capital bone ids) | 15 (0.006 m) | PASS |
| weeping-willow | 95 | -0.1 / +0.0 / +0.3 | -0.1 / 0.0 / 0.0 | 0.84 / 0.94 / 0.95 | 0 (schema 0, +0 on capital bone ids) | 12 (0.031 m) | PASS |

Validator issues: `validateAsset` volume rules 0 everywhere; Ajv conformance of the volume 0 everywhere except solids bound to bones whose own id is not a slug (`post-L`, `leaf-R1-a`, `bl-L1a`, `petiole-R1`, ...). Those errors come from the asset rig itself (capital letters), not from the volume; the whole-asset Ajv run also reports many errors outside `volume` (representations geometry `oneOf` noise, bone ids), all pre-existing. "Poses evaluated": every named pose plus three instants of every clip through `evaluate`; no failed evaluation, no solid detached (joint gap = increase of the nearest-vertex distance between a bone's solids and its nearest ancestor's solids against rest, at most 0.075 m on a 26 m tree).

## Gate misses and their causes (physically right body kept)

- **date-palm** (x +13 %, IoU 0.47 / 0.46 / 0.18): the 2D views draw all eight frond chains in the sagittal plane (plan z extent 0.46 m, front fronds edge-on) while `dimensions.depthM` is 7 m and `widthM` 6.3 m against a drawn elevation of 7.1 m. The body is a full rosette (frond lofts repeated 6 times at 30 degrees, 48 fronds), z = 7.1 m within 1.7 %; x follows the elevation (+13 % over `widthM`). Front also differs because the front bole is drawn about twice as wide as the elevation bole (0.58 times used).
- **agave** (rest bbox z -90 %, front 0.29, plan 0.27): the rest-pose staffage evaluation throws, so the check compares against the flat unplaced outlines of front and plan (the leaves only fan out in the default pose by abduction and twist). In the default pose the bbox is -0.9 / -6.4 / -1.3 %, elevation IoU 0.93. **Defect for routing**: `🔨️modules/🔺️polygon/🟦️.ts` `unionPolygons` (line 245), called from `placeShape` in `🔨️modules/🧮️evaluation/🟦️.ts` (line 668, the `solid3` taper branch), throws `PolygonUnionError: the union of 17 polygon(s) with area cannot be built: neither the edge-splitting nor the slab union closed a boundary of the expected area` for the elevation shape `leaf-L3-b` (bone `leaf-L3-b`, `solid { kind: extruded, sweep: v, z0: -0.011, z1: 0.011, zTip: [-0.002, 0.002] }`) of the 1:50 representation, when evaluated at a pose with no bone offsets (a synthetic rest pose; the named default pose evaluates). Repro: `T/volume-lib.ts` `restState` on `agave`.
- **hosta-clump** (rest bbox z -69 %): same mechanism, the 0.9 m depth exists only in the default pose; default-pose bbox +1.4 / -5.2 / -1.0 %, IoU 0.81 / 0.88 / 0.88.
- **bamboo-clump** (elevation 0.48, front 0.68): the elevation draws foliage only for culms 1 and 4 as scattered leaf-cluster polygons, front and plan draw all six masses, and the plan and front lateral positions disagree (mass-5 plan z -0.64..0.21 against front 0.05..0.85); the volume follows the plan.
- **miscanthus-grass** (0.55 / 0.41 / 0.29): thin curved blades (a few mm of drawn area per blade), the plan draws six filled areas (`crown-fill-*`, about 3 m2) that the fanning sheaves do not cover; bbox is within 2 %.

## Remarks for reviewers

- Crowns are lofts and oriented ellipsoids per authored foliage outline, each bound to the foliage bone with a twig capsule from the bone origin; plan "crown-fill" outlines are quadrant blobs (some with `weights` over two bones so they do not detach). Where plan and elevation disagree (maple cores and top, birch side lobes, pine pad heights, willow curtains), the elevation was trusted and the lobes clamped into the elevation bounds.
- multi-stem-birch elevation 0.81, lombardy-poplar elevation 0.80 and ivy plan 0.815 pass narrowly.
- Pollard/limb lateral drift follows the front outlines; branches without a front outline continue the lateral slope of their parent.
- Tool defects noted in `authoring-requests.md`: `volume-lib.ts` `meshes()` does not fill lofts without `rotation` (worked around by explicit rotations).
