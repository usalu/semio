# Silhouette guard: machinery and vehicles

Scope: `🏗️machinery` (16 assets) and `🚜️vehicles` (9 assets) of `catalog-silhouette`. Start: 21 failing assets (12 machinery, 9 vehicles). Now: 4 fixed, 17 remaining (9 machinery including the tower crane, 8 vehicles). The guard was not loosened; edits are span-preserving value edits or inserted shapes (`mvlib.py` on top of `sil-span.py`).

Gates after the edits: `validate-all.ts` 0 failing of 152, `validate-manifest.ts` 0 issues. Library tests `schema-conformance`, `asset-power`, `three-views`, `fill-rule`, `plan-projection`, `section-views`: 6 files, 446 tests passed.

New tools in this folder: `mvlib.py` (shape/solid lookup, insert, replace, in-string substitution), `show.py` (solids and drawn shapes of one asset scale), `solids-extent.ts` (world extent per volume solid; filter is the 2nd extra argument), `shapes-world.ts` (evaluated drawn shapes in world metres), `iou-bins.ts` (drawn-only and volume-only area per bin; extra arguments: ignored, ignored, bin size), `stroke-widths.ts` (the width each slender volume member is widened to). Repairs: `mv-*.py`.

## Findings that apply to every asset

1. Front and plan shapes without a `solid` are placed in absolute world metres (lateral or x, height or lateral), shapes with a `solid` in the coordinates of their bone. A hinge drawn with bone coordinates landed 0.6 m below the object.
2. A `slender` volume member is widened to the drawn width of the nearest drawn shape of the same bone (`followedWidth`). A filled closed polygon of at least three times its length counts as one strip: a filled zigzag of lattice diagonals is a strip as wide as the whole truss (1.56 m on the crane jib), and every volume diagonal that picked it was drawn 1.56 m wide. An open or unfilled outline contributes its pen width instead.
3. The volume of a thin ring (wheel, disc) widened by a drawn wide disc strip: the sorption wheel spokes followed the 0.18 m disc in front and plan.

## Fixed (0 failing checks at every scale)

| Asset | Failing before | Cause | Repair |
|---|---|---|---|
| plate-heat-exchanger | 1:50 front Hausdorff 0.0187 | the damper hinge barrel (0.42 m by 0.02 m bar along the lateral axis, top 0.62 m) was in the volume only | `damper-hinge` rect in the 1:50 front at world x -0.21, y 0.6 (`mv-plate-heat-exchanger.py`) |
| solar-thermal-collector | 1:500 front IoU 0.895, Hausdorff 0.1875 | the front drew one 0.85 m foot block, the volume has two 75 mm rails with two 75 mm struts at z = +-0.4 m (open frame) | two 75 mm feet instead of the block (as the 1:200 front), decision texts (en, de) and the 1:500 volume decision text updated |
| sorption-wheel | 1:50 front and plan IoU 0.512, Hausdorff 0.0926 | eight slender spokes per face followed the 0.18 m wide disc strip of the front and plan | open stroke shapes for the spokes on the front and rear face (front vertical, plan horizontal, hub gap left out), `mv-sorption-wheel.py` |
| electric-kick-scooter | 5 checks (1:200 plan Hausdorff, 1:100 front IoU, 1:50 bbox height 1.1199 m, front and plan Hausdorff) | 56 mm grips pushed the height to 1.12 m against 1.11 m declared; fork blade drawn 17 mm wide in front and plan, 28 mm in the volume (1:50, 1:100); crown missing in the front; the deck notch of 1:200 left a pocket of 12 mm behind the wheel and 2.5 mm slits beside it | grips 35 mm (radius 0.0175 m) in volume and three drawings at 1:50; fork blade 28 mm; crown as an extruded solid of the fork bone in the 1:50 front; notch end at x = -0.34 m and half width 0.0225 m at 1:200 (`mv-electric-kick-scooter*.py`) |

## Tower crane (partly repaired, still failing)

Regression repaired: the 1:200 and 1:100 elevation IoU are 0.940 and 0.927 again (guard 0.92). Causes: the drawn jib chord and rail rectangles were centred 0.06 m inside the volume chord centre lines (y = +-0.8 m), the counter-jib chord rectangles likewise (bone turned by 180 degrees, volume centre lines at local y = +-0.7 m), and the filled closed zigzag outlines of `jib-diagonals-l/r` and `counter-diagonals-l/r` widened the volume diagonals to the truss width. Repair: the rectangles are centred on the volume centre lines (jib y 0.73 and -0.86 for 0.14 m and 0.12 m members, counter 0.64 and -0.76), the diagonal outlines are unfilled with the member width as pen (0.3 mm at 1:200, 0.6 mm at 1:100: 60 mm), the counter zigzag keeps the truss middle. Also drawn now at 1:200: head sheave and jib tip sheave in all views, skip gate in elevation and front; at 1:200 and 1:100: winch vent stack (`mv-tower-crane*.py`).

Remaining 19 checks of the tower crane:

- 1:500 Hausdorff elevation 1.8638, IoU front 0.909, Hausdorff front 0.9939, Hausdorff plan 0.3
- 1:200 Hausdorff elevation 0.131 (jib root at x 0.13, y 45.13), Hausdorff front 0.6975 (jib root, y 43.7 and 44.1), IoU plan 0.869, Hausdorff plan 0.15 (jib tip sheave width in plan)
- 1:100 bbox x 71.8204 against 71.8 m, Hausdorff elevation 0.2949 (x 0.33, y 44.44), Hausdorff front 0.4725, IoU plan 0.899, Hausdorff plan 0.1575
- 1:50 IoU and Hausdorff of all three views: elevation 0.607 and 0.7096, front 0.576 and 0.4575, plan 0.709 and 0.9769 (a 71.8 m lattice drawn against 15 mm: needs the lattice, mast, pendants and tower head authored against the volume member by member)

## Remaining assets with the exact failing checks (values from the final run)

| Asset | Failing checks |
|---|---|
| wheel-loader (6) | 1:500 plan H 0.3937; 1:200 plan H 0.1275; 1:100 front H 0.0557, plan H 0.0557; 1:50 front H 0.0506, plan H 0.0562 |
| heat-pump-outdoor-unit (5) | 1:200 front H 0.0825; 1:100 bbox z 1.12 vs 1.10; 1:50 bbox x 0.7124 vs 0.72, bbox z 1.12 vs 1.10, elevation H 0.0332 |
| city-car (6) | 1:200 elevation H 0.075; 1:100 elevation H 0.0638, front H 0.0369; 1:50 elevation H 0.0641, front H 0.045, plan H 0.0284 |
| motor-scooter (8) | 1:500 elevation IoU 0.917; 1:100 elevation H 0.0521, front H 0.0338, plan H 0.0564; 1:50 bbox x 1.8848, elevation H 0.0528, front H 0.0338, plan H 0.0338 |
| vintage-car-1929 (9) | 1:200 elevation H 0.1677, front H 0.1575, plan H 0.105; 1:100 elevation H 0.06, front H 0.0522, plan H 0.15; 1:50 elevation H 0.0537, front H 0.1181, plan H 0.1219 |
| scissor-lift (9) | 1:200 bbox x 1.88, front H 0.075; 1:100 bbox x 1.865, elevation H 0.33, front H 0.09; 1:50 bbox x 1.86, elevation H 0.33, front H 0.0956, plan H 0.0356 |
| forklift (9) | 1:500 elevation H 0.3187; 1:200 front H 0.0675; 1:100 elevation H 0.0338, front H 0.0638, plan H 0.0825; 1:50 bbox z 1.212, elevation H 0.0525, front H 0.0638, plan H 0.09 |
| excavator (9) | 1:500 elevation H 0.3488, plan H 0.2813; 1:200 elevation H 0.0783; 1:100 elevation H 0.083, front H 0.2625, plan H 0.15; 1:50 elevation H 0.038, front H 0.0806, plan H 0.1744 |
| concrete-mixer-truck (9) | 1:500 elevation H 0.2062; 1:200 elevation H 0.0708; 1:100 bbox z 2.591, elevation H 0.0618, front H 0.0477, plan H 0.0825; 1:50 elevation H 0.0456, front H 0.0491, plan H 0.0825 |
| delivery-van (10) | 1:500 elevation H 0.4008; 1:200 elevation H 0.3256, front H 0.0605; 1:100 bbox x 5.375, elevation H 0.0668, front H 0.0676; 1:50 bbox x 5.3779, elevation H 0.046, front H 0.0651, plan H 0.0188 |
| diesel-generator (12) | 1:200 bbox z 2.471, elevation H 0.3225, front H 0.075; 1:100 bbox x 6.09, bbox z 2.4692, elevation H 0.0747, front H 0.0891; 1:50 bbox x 6.075, bbox z 2.46, elevation H 0.0656, front H 0.1154, plan H 0.0206 |
| city-bus (13) | 1:500 elevation H 0.4535; 1:200 bbox z 2.5833, elevation H 0.1115, front H 0.0605, plan H 0.0691; 1:100 elevation H 0.0862, front H 0.1315, plan H 0.0437; 1:50 bbox y 3.1475, bbox z 2.5624, elevation H 0.0805, front H 0.1013, plan H 0.051 |
| oil-pumpjack (13) | 1:500 plan IoU 0.805, H 1.3994; 1:200 elevation H 0.1186, front H 0.105, plan IoU 0.809, H 1.3829; 1:100 elevation H 0.4725, front H 0.4508, plan IoU 0.809, H 1.3832; 1:50 elevation H 0.3506, front H 0.4181, plan H 0.0296 |
| city-bicycle (14) | 1:200 elevation IoU 0.862, H 0.1202, front IoU 0.693, plan IoU 0.842, H 0.0712; 1:100 elevation IoU 0.877, H 0.0615, plan IoU 0.9, H 0.075; 1:50 bbox x 1.883, bbox y 1.0981, elevation H 0.0332, front H 0.0225, plan H 0.017 |
| cargo-bike (17) | 1:500 elevation H 0.2349, plan IoU 0.914; 1:200 elevation IoU 0.902, H 0.1237, front H 0.1125, plan IoU 0.87, H 0.1284; 1:100 bbox x 2.7211, elevation IoU 0.915, H 0.0691, front H 0.0689, plan IoU 0.907, H 0.0863; 1:50 bbox y 1.1779, elevation H 0.039, front H 0.0342, plan H 0.0694 |
| tram (11) | 1:500 elevation H 0.1769, front H 0.2685; 1:200 elevation H 0.1778, front H 0.0928, plan H 0.1107; 1:100 elevation H 0.1761, front H 0.0956, plan H 0.1107; 1:50 elevation H 0.3056, front H 0.096, plan H 0.1115 |

(H is the boundary Hausdorff distance in metres against the limit 0.15 / 0.06 / 0.03 / 0.015 m at 1:500 / 1:200 / 1:100 / 1:50; bbox values are the volume extent in metres against the declared dimension within one smallest feature.)

Next targets by effort: heat-pump-outdoor-unit (three bbox checks: one dimension disagrees by 0.02 m, decide drawing, volume or declared size), wheel-loader (all plan and front clusters of one kind), city-car, motor-scooter.

Scratch output (`🗑️generated-machinery-vehicles`) deleted.
