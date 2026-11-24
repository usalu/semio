# 🧊️ Three-Views Guard, Part 3: the People of Today's Human Wave

Fixer report of 2026-11-23, 13:30 to 15:10. Scope: the 47 people of `☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE` (K; ids = files in `K/📥️manifest-rows`). `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `U` = this ticket.

## Result

- Re-measured first (`🧰️three-views/sweep-wave-3.sh before`, the guard's own `sweep` and `consistency` on the catalog files): 39 of 47 green, **8 red with 180 findings**: e-scooter-rider 1, parent-with-child-on-shoulders 92, person-carrying-ladder 11, person-carrying-shopping-bags 5, person-hanging-laundry 3, person-in-winter-coat 25, person-pushing-bicycle 22, person-with-arm-sling 21. `person-in-rain-poncho` was already green (0 findings; not touched). No wave person outside the scope list was red.
- After (`sweep-wave-3.sh after`, run to the end on the shipped catalog files): **47 of 47 green, 0 findings**, default-pose consistency 0.
- Check CLI (`🩺️check --asset <id>`): `✓ <id> (1/1)` for the eight and for the poncho.
- No `widthM`, `heightM`, `depthM` or `massKg` changed; **no row file changed**, nothing to re-merge.
- Not all "validateAsset 0": `r3-validate.ts` counts the print findings of the range ends into `validateAsset`. Every one of them was in the file before my change (same lines, same count), none is new; `ASSET` findings are 0 for all eight. See the table and "What stays open".

| Person | Sweep before → after | `r3-validate --volumes` before | after | `catalog-silhouette` failing checks (`silhouette-2.ts`) | Check CLI | Ops added |
|---|---|---|---|---|---|---|
| e-scooter-rider | 1 → 0 | schema 0, validateAsset 5 (5 PRINT range-end), checkVolumes 0 | the same | 0 → 0 | ✓ | 2 |
| parent-with-child-on-shoulders | 92 → 0 | schema 0, validateAsset 3 (3 PRINT), checkVolumes 0 | the same | 0 → 0 | ✓ | 59 |
| person-carrying-ladder | 11 → 0 | schema 0, validateAsset 1 (1 PRINT), checkVolumes 3 | schema 0, validateAsset 1 (the same PRINT), checkVolumes **2** | 5 → 5 (the same five) | ✓ | 3 |
| person-carrying-shopping-bags | 5 → 0 | schema 0, validateAsset 3 (3 PRINT), checkVolumes 0 | the same | 1 → 1 (the same) | ✓ | 2 |
| person-hanging-laundry | 3 → 0 | schema 0, validateAsset 3 (3 PRINT), checkVolumes 0 | the same | 2 → 2 (the same two) | ✓ | 1 |
| person-in-winter-coat | 25 → 0 | schema 0, validateAsset 1 (1 PRINT), checkVolumes 0 | the same | 0 → 0 | ✓ | 18 |
| person-pushing-bicycle | 22 → 0 | schema 0, validateAsset 15 (15 PRINT), checkVolumes 13 | the same | 1 → 1 (the same) | ✓ | 2 |
| person-with-arm-sling | 21 → 0 | schema 0, validateAsset 0, checkVolumes 2 | the same | 0 → 0 | ✓ | 6 |
| person-in-rain-poncho | 0 (re-measured only) | not run | not run | not run | ✓ | 0 |

The silhouette numbers are the bounding box, IoU and Hausdorff rules of the test without its sharp oracle (`silhouette-2.ts`, default pose), measured on the untouched copy and on the shipped document; I did not run the vitest suite. Largest moves inside the gates: laundry 1:500 plan IoU 0.967 → 0.954, arm sling 1:500 front Hausdorff 16 → 35 mm (gate 150), ladder 1:500 plan Hausdorff 83 → 59 mm.

## Cause, in one paragraph

A front or plan shape without a `solid` is a plate without depth that its bone carries along (README, "Front and plan shapes"), and a `solid` swept over 2 mm is the same plate. A limb swung into the viewing direction is then a sliver in that view while the side elevation shows its whole length. The fix gives the plate the thickness the side elevation already draws for the same part: the authored outline stays the profile and becomes an `extruded` solid swept along the bone axis nearest to the viewing direction, over the range of the side twin (`prism-3.ts`, way one); where the outline does not lie along the bone (a shoe on a pitched foot bone) the side profile of the twin is swept over the width of the old outline (way two, as part 2 did). No view is copied from another scale, no shape is deleted, no tolerance touched.

## Per person

### person-in-winter-coat (HW-16)
- Before: 25. All scales, `passing-left/-right`, `stamping-feet/-left`, `walk-hands-in-pockets` 0.96 s: side x from −0.152 m, plan from −0.083 m (tolerance 0.054). 1:500 also `contact-left/-right` and three walk instants: side y from −0.041 m, front from 0.052 m.
- Cause: plan thighs and coat skirts are flat cross-sections; the 1:500 front shoes are flat outlines on a foot bone that pitches.
- Fix: `thigh-l/-r`, `coat-skirt-l/-r` of the plan at all four scales swept along the thigh over its length (16 shapes); 1:500 front `shoe-l/-r` are the side profile swept over the shoe width (2).
- Sheet opened (1:100 `passing-left`): the swung leg shows in the plan as a second lobe behind the body, the three views read 1.80 / 1.80 m; front and side unchanged.

### person-with-arm-sling (HW-15)
- Before: 21. `glance-at-watch` 0.8 and 3.2 s at all scales (side hand x 0.339 … 0.521 m, plan 0.426 … 0.434 m), `contact-right`, `loading-right`, walk instants; 1:500 also the shoe case of the winter coat.
- Cause: the left hand of the plan is a traced outline of the hanging hand, a plate.
- Fix: 1:50 and 1:100: the traced outline stays; under it the plan gets `hand-l-body`, palm and fingers to the middle joints as the side profile swept over the palm thickness (z 0.205 … 0.237, the loft of the volume) (`ops-3/sling-hand.py`). 1:200 and 1:500: `hand-l` is the side profile swept over its width. 1:500 front `shoe-l/-r` as above.
- Tried and taken back: the traced hand swept along the hand bone, and the side profile in its place at 1:50 and 1:100: both made the sweep 0 but put the plan silhouette of 1:50 and 1:100 out of its gate (38 … 43 mm against 15 and 30).
- Sheet opened (1:100 `glance-at-watch` 0.8 s): the raised hand shows in the plan as a small block ahead of the body. **Imperfect:** the forearm of the plan is still a plate, so between body and hand only a thin line is drawn; restating the forearm too was measured (plan Hausdorff 26 mm at 1:50, gate 15) and not kept.
- Open, not mine: `checkVolumes` 2 (`volumes-structure-coverage`, finger bones of the right hand outside the mitten), identical before.

### person-carrying-shopping-bags (HW-13)
- Before: 5, 1:500 only (`contact-left/-right`, three walk instants): side y from −0.041 m, front from 0.027 m.
- Fix: 1:500 front `shoe-l/-r` are the side profile swept over the shoe width.
- Sheet rendered (1:500 `contact-left`) but **not opened**. Stays as found: the typified 1:500 shoe reaches 41 mm under the ground at heel strike in side and now front (0.08 mm on paper).

### person-hanging-laundry (HW-13)
- Before: 3, 1:500 (`lifting-from-basket`, `hang-shirt` 0 and 6.99 s): side x from −0.239 m, plan from −0.128 m.
- Cause: the plan jacket was a 9 cm fan at the torso base (a profile whose first coordinate encodes the half width through `zTip: [-1, 1]`), not the jacket; bent forward the jacket tail reaches back and the plan does not follow.
- Fix: 1:500 plan `jacket` is the side profile swept over the width the front view draws (±0.2 m).
- Tried and taken back: the plan outline swept along the torso; it made the default plan 0.895 m long against the declared 0.869 m (`extent-plan-width`).
- Sheet opened (1:500 `lifting-from-basket`): the plan shows the bent trunk as a block from the hips to the basket. **Plainer than before:** at rest the 1:500 plan jacket is a rectangle 0.28 × 0.40 m, no longer rounded. The same fan is in the plan of 1:200 (passes the guard behind head and arms); not changed.

### e-scooter-rider (HW-09) and person-pushing-bicycle (HW-09)
- Before: 1 (`kicking-off`: front y from −0.082 m, side −0.023 m) and 22 (every 1:500 instant with a twisted bone: front y from −0.105 m, side 0.000 m).
- Cause: the 1:500 front shoes were plan outlines swept along the bone normal with a taper, which stand under the ground on a pitched foot.
- Fix: 1:500 front `shoe-rear-l/-r` (scooter) and `shoe-l/-r` (bicycle) are the side profile swept over the shoe width.
- Sheet opened: scooter 1:500 `kicking-off` (1.80 m side, 1.78 m front). The bicycle sheet (1:500 `standing-with-bicycle`) was rendered but **not opened**.
- Open, not mine: the bicycle's `checkVolumes` 13 and its 15 print findings, identical before.

### person-carrying-ladder (HW-18)
- Before: 11. 1:50 walk poses and `carry-walk`: front y from −0.096 m, side 0.000 m. 1:200 and 1:500 `tipping-up`, `tip-and-settle`: side x to 1.331 m, plan to 1.238 m.
- Causes and fixes (`ops-3/ladder.py`): (1) the 1:50 front toe cap `shoe-toe-r` was bound to the right shank and swung with it; it is a solid of `toe-r` now, the side profile of the toe cap swept over the width of the left toe cap, mirrored. (2) The plan of 1:200 and 1:500 drew only the upper rail of the ladder carried on edge; it gets `rail-lower` under the upper rail (1:200 the solid of the side elevation, 1:500 the lower 25 mm of the frame).
- Tried and taken back: binding the traced toe outline to `toe-r` (front 60 mm too high in the default stride, `extent-front-height`).
- Side effect: `volumes-views-disagree` of 1:500 (elevation 2961 mm against plan 2877 mm, listed as not repaired in `humans-b-repair.md`) is gone: checkVolumes 3 → 2.
- Sheets opened (1:200 `tipping-up`, 1:50 `walk-heel-off-r`): the plan ladder reaches as far ahead as the tipped side view; the right foot stands on the ground line in the front view, its toe cap is a plain block.

### parent-with-child-on-shoulders (HW-17)
- Before: 92. All scales, `child-waving`, `child-waving-back`, `wave-to-the-parade`, walk instants: toddler head side x −0.322 … −0.111 m against plan −0.521 … 0.085 m, side y to 2.161 m against front 2.294 m. 1:500 also the stride: plan x 84 mm further ahead than the side.
- Causes: (1) the toddler trunk rests lying (−90 degrees) and every pose sits it up by 95 to 98 degrees; the front and plan plates of the toddler head were stretched so that the default pose looks right, so two degrees more trunk angle moved them by 0.2 m. (2) At 1:500 the side elevation drew the boots without toe caps.
- Fixes: (1) the head shapes of front and plan (head, hat, cuff, pompom and the face parts, 55 shapes over four scales) are prisms: their outline as it stands in the default pose, swept through the depth the side elevation draws for each part. (2) 1:500 side `boot-l/-r` are drawn to the toe tip (`ops-3/parent-toe.py`).
- Sheets opened (1:50 `child-waving` before and after): before, the child's head was a spike 2.29 m high in the front view and a 0.6 m ellipse in the plan; after, front and side read 2.16 m and the plan head is round. The 1:500 `walk-3` sheet was rendered but **not opened**.
- Not touched: the parent's face covered by the child's legs in the front view (`humans-b-repair.md`).

## Reproducibility

- HW-10 … HW-18 (six people): my ops are appended to `K/🧰️scripts/HW-1x/repair/<slug>.json`; `ship-b-3.ts` checked that the whole list applied to the untouched copy gives the proven candidate, and the catalog file was then written by the chain's own `REP-B/apply.ts <slug>` (byte-identical to the candidate, `cmp`).
- HW-09 (two people): the ops are the new stage `K/🧰️scripts/REP-A/ops/<slug>.70-three-views.json`; `ship-a-3.py` applied that stage with `REP-A/apply.py`'s own `apply` and layout to the catalog file, which carries the earlier stages.
- **Not proven:** I ran no worker generator. That each generator still writes the file the ops start from is as (un)proven as `humans-a-repair.md` and `humans-b-repair.md` state it; the kept generator outputs of REP-A no longer exist.

## What stays open

- Print findings of the range ends (`PRINT range-end-feature` / `range-end-gap`), all present before: e-scooter-rider 5, parent 3, ladder 1, shopping bags 3, laundry 3, winter coat 1 (`coat-centre` 0.200 mm at 1:350), bicycle 15. They are why `r3-validate.ts` does not print `validateAsset 0` for seven of the eight; the wave's own gate (`REP-B/validate.ts`) does not count them. Not repaired: each is a redraw of a small part at a band end.
- `catalog-silhouette` failing checks that were there and stay: ladder 5, laundry 2, shopping bags 1, bicycle 1.
- The guard compares whole-figure extents. Bones whose plates are hidden behind other parts at every sampled instant are still plates (for instance the forearms and shanks of the coat, sling and bag people in the plan); `bones-3.ts <id> --all` lists them per instant.
- Three of the nine final sheets were rendered and not opened (shopping bags, bicycle, parent 1:500).

## Files

Catalog (under `S/🗂️catalog/👤️humans/`): `e-scooter-rider`, `parent-with-child-on-shoulders`, `person-carrying-ladder`, `person-carrying-shopping-bags`, `person-hanging-laundry`, `person-in-winter-coat`, `person-pushing-bicycle`, `person-with-arm-sling` (each `🔣️.json`).

Wave ticket (under `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-HUMAN-COLLECTION-WAVE/🧰️scripts/`): updated `HW-13/repair/person-carrying-shopping-bags.json`, `HW-13/repair/person-hanging-laundry.json`, `HW-15/repair/person-with-arm-sling.json`, `HW-16/repair/person-in-winter-coat.json`, `HW-17/repair/parent-with-child-on-shoulders.json`, `HW-18/repair/person-carrying-ladder.json`; created `REP-A/ops/e-scooter-rider.70-three-views.json`, `REP-A/ops/person-pushing-bicycle.70-three-views.json`.

This ticket: `three-views-3.md`; new under `🧰️three-views/`: `sweep-wave-3.sh`, `bones-3.ts`, `prism-3.ts`, `build-3.sh`, `cand-3.sh`, `all-3.sh`, `look-3.sh`, `ship-b-3.ts`, `ship-a-3.py`, `ops-3/parent-toe.py`, `ops-3/ladder.py`, `ops-3/sling-hand.py`, `ops-3/<slug>.json` (the ops I appended, six files, as a record).

Not touched: shared code, tests, fixtures, the manifest, row files, `requests.md` (no shared defect found), any other asset. `🗑️generated/tv3/` removed.

Notes on this session: twice a waiting loop of mine (`until ! pgrep -f all-3.sh`) matched itself and never ended; I ended the two by their process ids. Short measurements never overlapped a running proof.
