# Author 14 — vehicles (9 assets): status report

Honest status: **one of nine assets (city-car) is authored at all four scales and all three lists but does not yet pass every rule and is not spliced. The other eight (cargo-bike, city-bicycle, city-bus, delivery-van, electric-kick-scooter, motor-scooter, tram, vintage-car-1929) are NOT done.** city-bicycle holds the draft output of the generic drafter (not hand-modelled); the other seven are only seeded (1:50 body from the old volume). Nothing was spliced; no catalog asset has `volumes` from me.

## What exists (reusable by whoever continues the vehicles)
- `author-14-dump.ts` (pose frames + closed drawn shapes as world rings, per scale and view), `author_14_lib.py` (`Sil` exact scanline queries on the drawing, region-less solid builders, `skin` nested sheet layers, `proud`, writer, area calibration against the product check findings), `author_14_kit.py` (`xloft` along x, `vloft` through horizontal slices, `fit_x`/`fit_y`/`auto_x`/`auto_y` that read stations from each scale's own drawing with Douglas-Peucker tolerance, `panel_loft` through a drawn panel outline), `author-14-diag.ts` (where the Hausdorff distance comes from, cluster list), `author-14-ext.ts` (which solid sets the bbox), `author-14-area.ts`, `author_14_root.py` (text-level root fixes of 2D shapes and `add_shape`), `author-14-run.sh`.
- Method that worked: lower body = x-loft with crisp superelliptic sections (exponent 18-40) whose stations come from the scale's own elevation and plan; cabin = loft through horizontal slices (elevation row + front half-width per height), so elevation and the tumblehome of the front view are exact; bonnet/tailgate lofts through the thick outline the coarser drawings show. Result at 1:100/1:200/1:500 of city-car: silhouette IoU 0.97-0.995 and Hausdorff inside the limit in all three views.

## city-car (`T/volumes/vehicles/city-car.json`, generator `T/author-14-city-car.py`)
Counts: 1:50 body 60 / thermal 41 / structure 33; 1:100 34 / 28 / 33; 1:200 ~27 / 27 / 33; 1:500 ~10 / 24 / 33 (see the JSON). Decisions en/de written for every scale.
Last full check (97 s): remaining failures, stated plainly:
- `volumes-silhouette` 1:50 p95 elevation 20.6 mm vs 15 mm (Hausdorff only informational now); 1:500 p95 elevation 187 mm vs 150 mm.
- `volumes-views-disagree` (new rule): hinged `bonnet` and `hatch` bones extents differ between elevation, front and plan by up to 0.24 m on z for the hatch (the 2D hatch front panel is +-0.58 wide, the plan +-0.70) — a 2D inconsistency; needs a root fix of the drawn hatch panel widths (or a ruling).
- `volumes-copied` 1:100 and 1:500: thermal/structure lists are subsets of the finer scale; structure sections must be varied per scale (not yet done).
- `volumes-feature` 1:100-1:500: thermal brake discs, floor liner, exhaust line, tailpipe nozzle and structure ribs thinner than the feature; need thicker coarse sections or `slender`.
- `volumes-monotonic` structure never strictly drops (identical structure at all scales): merge joints/members at coarser scales.
- `volumes-thermal-coverage`: roof area (plate buried at 1:50/1:100), engine-bay 1:50/1:200 (cavity block), interior 1:50/100/500 uses a hidden region as body reference, `outsideBodyBoxM` 0.30-0.47 m (the thermal plates of bonnet/tailgate are expressed in bone frames; some own thermal solid sits outside the body box), glazing missing at 1:500, tyres 1:500 +40 %.
- `volumes-structure-coverage` 1:50/1:100: A-pillar / roof rail stick out of the body silhouette by 31-51 mm.
- `volumes-cross-scale`: Hausdorff 45-190 mm vs 30/60/150 (the drawings of consecutive scales differ at the hatch, sill step and silencer by more than the tolerance).

## 2D root fixes applied (asset `🎓️teaching/…/🚜️vehicles/city-car/🔣️.json`, text-level, shapes only)
- mirror: front rect y 1.05/h 0.10 -> y 1.025/h 0.09 (50, 100, 200); plan rect x 0.76/w 0.12 -> x 0.79/w 0.14 (50, 100, 200); elevation 1:100 ellipse cy 1.13/cx 0.80 -> cy 1.07/cx 0.86 (all three views now describe the same box x 0.79-0.93, y 1.025-1.115, z 0.76-0.84).
- plan `hatch-0` (50..500): polygon replaced by the elevation `hatch-slab` outline so the plan footprint no longer reaches x = -2.006 (dimension 3.9 m).
- `tail-pipe` elevation x1 -1.945 -> -1.925 (50, 100), plan x0/x1 -1.92/-1.945 -> -1.90/-1.925 (50, 100), so the drawn width is 3.90 m, not 3.92 m.
- added front `silencer` rect to 1:200 (elevation draws it, front did not) and an elevation `mirror` ellipse to 1:200 (front and plan draw it).
No `areaM2` was changed.

## Observations for the coordinator
- The old per-scale drawings are inconsistent between views at the 1.5 cm level at 1:50 (mirror, tail pipe, hatch width); each costs a root fix. Expect the same on the other eight.
- Thermal under the exposed-surface rule needs plates that stand proud of the body (a plate lying in a body solid counts 0); engine-bay of a car needs a real cavity between hull and bonnet.
- The full check now takes ~100 s per asset, calibration loops of 4 checks are expensive.
- Tool output used: `volumes-check.ts` at the time of the run (product validator `[p]`).

## Not done
cargo-bike, city-bicycle (draft only), city-bus, delivery-van, electric-kick-scooter, motor-scooter, tram, vintage-car-1929: no hand modelling, no check pass, no splice, no render review. The generator pattern of `author-14-city-car.py` (fit lofts from the scale's drawing, `skin`, calibrate) is the starting point; two-wheelers need tube members and `slender` (C2) rather than lofts.
