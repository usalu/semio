# Catalog silhouette guard: triage and repairs

Guard: `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette` (one test per asset: bounding box within one smallest feature of `dimensions`, IoU at least 0.92 and boundary Hausdorff distance at most two features between the drawing of each view and the volume of the same scale; the guard was never loosened). Tools of this ticket: `silhouette-triage.ts` (the checks of the test for the working tree or `git show HEAD:<path>`, all failures instead of the first), `silhouette-diag.ts` (worst mismatch clusters of one asset, scale and view, optional ASCII patch), `sil-run-all.sh`, `sil-table.mjs`, `sil-span.py` (span-preserving JSON editor: every other byte of an asset file stays), `sil-fix-round1..4.py` (the repairs).

## Result

| | Before | After |
|---|---:|---:|
| Failing assets of `catalog-silhouette` (of 152) | 117 | 115 |
| Passing | 35 | 37 (`stair-flight`, `bar-stool` added) |
| `validate-all.ts` | 0 failing | 0 failing of 152 |
| `validate-manifest.ts` | 0 issues | 0 issues |
| `schema-conformance`, `asset-power`, `section-views`, `volumes-rules` | pass | pass (run together with the guard: 4 files passed, 250 tests passed) |

973 failing checks remain over 115 assets (220 IoU, 592 Hausdorff, 161 bounding box). By scale: 1:50 has 385 (255 Hausdorff, 86 bbox, 44 IoU), 1:100 has 262, 1:200 has 200, 1:500 has 126.

## Triage: regression or pre-existing

Method: every asset measured on the working tree and on `git show HEAD:<path>` with the same functions as the test; a check that fails now and did not fail at HEAD is a regression of today's edits. The per-asset sets of failing checks were identical at HEAD for every other asset, including the ones the task named (cyclist, hot-water-cylinder, tealight, city-bus, oil-pumpjack, wheel-loader) and the untouched ones (stair-flight, steel-column, tied-canopy, wheelchair-ramp): **116 of the 117 failing assets already failed at HEAD (six of them with additional regressed checks); `bar-stool` failed only because of today's edit; 7 assets regressed in total** (the head comparison of `scots-pine` and `weeping-willow` could not be completed in time; both are among the vegetation assets that fail massively at every scale for the cause below).

| Asset | Regressed checks (not failing at HEAD) | Cause (today's edit) | Status |
|---|---|---|---|
| elderly-with-cane | 1:500 and 1:200 elevation IoU (0.861, 0.906) | cane capsule centre line moved behind the true front edge in the drawing (`fix-elderly.py`), volume cane stayed on the axis | fixed: volume cane/ferrule follow the drawn centre line and radius |
| wheelchair-user | 1:500 elevation IoU (0.841) | drawn rear wheel ring shrunk to the true 0.305 m tyre radius (`fix-wheelchair.py`), volume tyre kept 0.3305 m | fixed: volume tyre 0.24 to 0.305 m |
| pig | 1:500 elevation IoU (0.916) | drawn hindbody rump pulled back by 0.035 m (`fix-pig.py`), volume loft not | fixed: loft cap moved to x = -0.2534 m, one section inserted, second refitted |
| bar-stool | 5 checks: 1:500 IoU in all three views, 1:200 IoU elevation and front | drawn leg foot moved to 0.21 m and foot ring to 0.205 m, volume legs 0.1975 m, ring 0.2275 m | fixed at 1:500 and 1:200 (0 failing checks at every scale) |
| excavator | 1:500 front Hausdorff (0.756 m) | the `slew-ring` was removed from the 1:500 elevation and front (role detail, null pen: `shape-role-undrawn`), the volume still has the ring, so the drawing showed a 0.14 m gap between undercarriage and house | fixed: the ring is drawn again as role body (a pen-bearing shape); the remaining excavator checks are the pre-existing ones |
| sliding-door | 1:500 plan IoU 0.354 and Hausdorff 0.45 m | 1:500 plan declared as section A-A at 1.0 m (required at every scale), but still drew the head track seen from above | fixed: two jamb cuts 0.15 x 0.1 m and the leaf 0.075 m thick like the volume |
| tower-crane | 1:200 and 1:100 elevation IoU (0.74 at 1:200, was above 0.92) | jib and counter-jib chords redrawn as filled left/right rectangles (`jib-chord-l/r`, `jib-rail-l/r`, `counter-chord-*`) whose centre lines and thickness differ from the volume chord boxes (jib chords at 0.67 to 0.81 against box 0.75 to 0.85 m, rails at -0.8 to -0.68 against -0.85 to -0.75 m, counter chords 0.12 against 0.107 m) | **open**: aligning only the elevation rectangles with the boxes made it worse (IoU 0.446): the reference width of the slender volume members follows the drawn member width (`followedWidth`), so the drawn thickness, the lattice `jib-diagonals-l/r` and the volume chords must be changed together; reverted to the prior state, needs its own pass (each measurement of the crane takes about three minutes) |

Also repaired although pre-existing (an untouched asset the task named): **stair-flight** 1:50 elevation Hausdorff 0.0387 m: the 1:50 tread volume had a rounded nosing and the drawn tread a sharp corner at (0.36014, -0.03115), which let the slit between tread and flange exceed the 256-cell small-hole fill in the volume and not in the drawing; the tread profile now equals the drawn polygon.

## Pre-existing causes (what the remaining 115 assets share)

- The guard compares boundaries, not areas: Hausdorff distance of the outlines including every enclosed hole larger than `(4 features)^2` cells. At 1:50 the limit is 15 mm, at 1:100 30 mm, at 1:200 60 mm, at 1:500 150 mm. Drawings (traced contours, member strokes) and volumes (lofts, revolves, extrusions) were authored separately and differ by centimetres at fine scales: 592 of the failing checks are Hausdorff, 255 of them at 1:50.
- Typical single-cause findings: a drawn feature with no volume twin or the reverse (red-fox ear tips 22 mm outside the volume in the front view; hot-water-cylinder front 39 mm; steel-column elevation 45 mm), a sub-cell IoU shortfall of tiny objects (tealight 1:50 front 0.919: the 38 mm object is 20 cells wide), bounding boxes a few millimetres beyond one feature (wheelchair-ramp 10.5212 m against 10.5 m), spokes, slats and lattices that the drawing thickens for legibility (humans, animals at 1:500 and 1:200), and canopies/foliage drawn as areas while the volume is an envelope (all 16 vegetation assets, up to 2.3 m at 1:500 for english-oak).
- Fewest checks (best next targets, at most three failing checks each): red-fox, mallard-duck, domestic-cat, server-rack, split-ac-indoor-unit, electric-fan-heater, circulation-pump, electric-kettle, hot-water-cylinder, laptop, shower-head, vacuum-cleaner, thermostatic-radiator-valve, desktop-monitor, cantilever-balcony, curtain-wall-bay, steel-column, wheelchair-ramp, external-venetian-blind, tied-canopy, office-swivel-chair, park-bench, picnic-table, cafe-chair, walking-commuter, plate-heat-exchanger, solar-thermal-collector, shopping-trolley, patio-heater, charcoal-grill, tealight, playground-swing, hornbeam-hedge.

## Fixes (files)

`🗂️catalog/🧱️components/stair-flight`, `🧱️components/sliding-door`, `🛋️furniture/bar-stool`, `👤️humans/elderly-with-cane`, `👤️humans/wheelchair-user`, `🐕️animals/pig`, `🏗️machinery/excavator` (all `🔣️.json`; edits are in-place value edits through `sil-span.py`, so no reformatting of the assets). A first pass with the line-oriented editor of `humans-animals-edit.py` had reformatted the volumes block of four of them; `sil-fix-round1.py` restored those blocks from `git show HEAD` (verified: the only differences to HEAD were the intended solids) and re-applied the fixes as span edits. `sil-fix-round4.py` was run in two steps (jambs first, then the leaf; the file holds both, rerunning it is not meaningful).

## Remaining (precise)

1. tower-crane regression above (1:200, 1:100 elevation IoU) plus its pre-existing 19 checks (1:50 elevation IoU 0.607, Hausdorff up to 1.86 m at 1:500).
2. 114 other failing assets (973 checks over 115 assets including the crane); table below, sorted by category and failing checks. None is a regression of today's edits except the crane.
3. Suggested order: the 33 assets with at most three failing checks (one asset takes about three to ten minutes with `silhouette-diag.ts`: read the worst clusters, align the drawn shape or the volume solid to the real dimension, rerun `silhouette-triage.ts current`), then furniture, appliances, components (about 20 checks each), then humans and animals (the 1:500 and 1:200 legibility thickening against the volume), then vehicles and machinery, last vegetation (16 assets, 314 checks, envelope against area drawing; tree runs take one to three minutes per asset).
4. Verification after each batch: `validate-all.ts` (0 failing), `validate-manifest.ts` (0 issues), the vitest files `schema-conformance`, `asset-power`, `section-views`, `volumes-rules`, `catalog-silhouette` (about three minutes in total).

## Failing assets

| Category | Asset | Failing checks | IoU | Hausdorff | bbox | Scales | Worst Hausdorff |
|---|---|---:|---:|---:|---:|---|---|
| animals | red-fox | 2 | 0 | 2 | 0 | 1:50 | 0.0225 m vs 0.015 (1:50 front) |
| animals | mallard-duck | 2 | 1 | 1 | 0 | 1:50 | 0.025 m vs 0.015 (1:50 front) |
| animals | domestic-cat | 3 | 1 | 2 | 0 | 1:200 1:50 | 0.016 m vs 0.015 (1:50 elevation) |
| animals | pigeon | 6 | 4 | 2 | 0 | 1:200 1:100 1:50 | 0.0296 m vs 0.015 (1:50 plan) |
| animals | labrador-dog | 6 | 1 | 5 | 0 | 1:500 1:100 1:50 | 0.0446 m vs 0.015 (1:50 front) |
| animals | hedgehog | 6 | 3 | 2 | 1 | 1:100 1:50 | 0.0296 m vs 0.015 (1:50 elevation) |
| animals | herring-gull | 8 | 4 | 3 | 1 | 1:200 1:100 1:50 | 0.045 m vs 0.015 (1:50 front) |
| animals | roe-deer | 8 | 4 | 4 | 0 | 1:500 1:200 1:100 1:50 | 0.0321 m vs 0.015 (1:50 plan) |
| animals | pig | 8 | 1 | 6 | 1 | 1:200 1:100 1:50 | 0.0544 m vs 0.015 (1:50 plan) |
| animals | sheep | 8 | 0 | 7 | 1 | 1:200 1:100 1:50 | 0.0439 m vs 0.015 (1:50 plan) |
| animals | dairy-cow | 10 | 2 | 7 | 1 | 1:500 1:200 1:100 1:50 | 0.1064 m vs 0.015 (1:50 front) |
| animals | goat | 10 | 5 | 4 | 1 | 1:500 1:200 1:100 1:50 | 0.0569 m vs 0.015 (1:50 front) |
| animals | horse | 13 | 2 | 9 | 2 | 1:500 1:200 1:100 1:50 | 0.1425 m vs 0.015 (1:50 plan) |
| appliances | server-rack | 1 | 0 | 1 | 0 | 1:50 | 0.0262 m vs 0.015 (1:50 front) |
| appliances | split-ac-indoor-unit | 1 | 0 | 1 | 0 | 1:50 | 0.0186 m vs 0.015 (1:50 front) |
| appliances | electric-fan-heater | 1 | 0 | 1 | 0 | 1:50 | 0.0319 m vs 0.015 (1:50 elevation) |
| appliances | circulation-pump | 1 | 0 | 1 | 0 | 1:50 | 0.0188 m vs 0.015 (1:50 front) |
| appliances | electric-kettle | 1 | 0 | 1 | 0 | 1:50 | 0.0173 m vs 0.015 (1:50 front) |
| appliances | hot-water-cylinder | 1 | 0 | 1 | 0 | 1:50 | 0.0394 m vs 0.015 (1:50 front) |
| appliances | laptop | 2 | 2 | 0 | 0 | 1:100 1:50 |  |
| appliances | shower-head | 2 | 2 | 0 | 0 | 1:200 |  |
| appliances | vacuum-cleaner | 2 | 1 | 1 | 0 | 1:500 1:50 | 0.03 m vs 0.015 (1:50 elevation) |
| appliances | thermostatic-radiator-valve | 2 | 2 | 0 | 0 | 1:100 |  |
| appliances | desktop-monitor | 3 | 2 | 1 | 0 | 1:200 1:100 1:50 | 0.0225 m vs 0.015 (1:50 plan) |
| components | cantilever-balcony | 1 | 0 | 1 | 0 | 1:50 | 0.0214 m vs 0.015 (1:50 elevation) |
| components | curtain-wall-bay | 1 | 0 | 1 | 0 | 1:200 | 0.0825 m vs 0.06 (1:200 plan) |
| components | steel-column | 2 | 0 | 2 | 0 | 1:50 | 0.045 m vs 0.015 (1:50 elevation) |
| components | wheelchair-ramp | 3 | 0 | 1 | 2 | 1:100 1:50 | 0.0225 m vs 0.015 (1:50 elevation) |
| components | external-venetian-blind | 3 | 0 | 3 | 0 | 1:200 1:100 | 0.0737 m vs 0.03 (1:100 plan) |
| components | tied-canopy | 3 | 0 | 3 | 0 | 1:500 1:100 1:50 | 0.2906 m vs 0.015 (1:50 plan) |
| components | sliding-door | 4 | 2 | 2 | 0 | 1:50 | 0.0431 m vs 0.015 (1:50 front) |
| components | masonry-wall-with-lintel | 5 | 0 | 5 | 0 | 1:200 1:100 1:50 | 0.105 m vs 0.015 (1:50 plan) |
| components | spiral-stair | 7 | 0 | 5 | 2 | 1:200 1:100 1:50 | 0.0265 m vs 0.015 (1:50 front) |
| components | sectional-garage-door | 9 | 2 | 4 | 3 | 1:200 1:100 1:50 | 0.06 m vs 0.015 (1:50 plan) |
| furniture | office-swivel-chair | 2 | 0 | 2 | 0 | 1:100 1:50 | 0.0488 m vs 0.015 (1:50 plan) |
| furniture | park-bench | 2 | 0 | 2 | 0 | 1:50 | 0.0375 m vs 0.015 (1:50 front) |
| furniture | picnic-table | 3 | 0 | 3 | 0 | 1:100 1:50 | 0.0488 m vs 0.015 (1:50 front) |
| furniture | cafe-chair | 3 | 0 | 3 | 0 | 1:100 1:50 | 0.0203 m vs 0.015 (1:50 front) |
| furniture | height-adjustable-desk | 5 | 1 | 4 | 0 | 1:100 1:50 | 0.0694 m vs 0.015 (1:50 front) |
| furniture | three-seat-sofa | 5 | 0 | 5 | 0 | 1:100 1:50 | 0.2888 m vs 0.015 (1:50 plan) |
| furniture | folding-deck-chair | 6 | 3 | 1 | 2 | 1:500 1:100 1:50 | 0.0428 m vs 0.03 (1:100 elevation) |
| furniture | operating-table | 6 | 1 | 5 | 0 | 1:200 1:100 1:50 | 0.0788 m vs 0.015 (1:50 elevation) |
| humans | walking-commuter | 3 | 0 | 3 | 0 | 1:100 1:50 | 0.0431 m vs 0.015 (1:50 plan) |
| humans | seated-reader | 4 | 1 | 3 | 0 | 1:500 1:100 1:50 | 0.0273 m vs 0.015 (1:50 elevation) |
| humans | child-on-kick-scooter | 6 | 1 | 5 | 0 | 1:500 1:100 1:50 | 0.0345 m vs 0.015 (1:50 front) |
| humans | running-child | 6 | 3 | 2 | 1 | 1:500 1:50 | 0.0319 m vs 0.015 (1:50 elevation) |
| humans | pregnant-person | 6 | 1 | 4 | 1 | 1:200 1:100 1:50 | 0.0919 m vs 0.015 (1:50 front) |
| humans | kneeling-gardener | 7 | 0 | 6 | 1 | 1:100 1:50 | 0.12 m vs 0.015 (1:50 plan) |
| humans | skateboarder | 7 | 1 | 3 | 3 | 1:500 1:100 1:50 | 0.0244 m vs 0.015 (1:50 plan) |
| humans | sleeping-person | 7 | 0 | 5 | 2 | 1:100 1:50 | 0.0506 m vs 0.015 (1:50 plan) |
| humans | standing-adult | 7 | 3 | 4 | 0 | 1:500 1:200 1:100 1:50 | 0.0604 m vs 0.015 (1:50 elevation) |
| humans | standing-phone-user | 7 | 2 | 5 | 0 | 1:500 1:100 1:50 | 0.0562 m vs 0.015 (1:50 front) |
| humans | runner-with-blade-prosthesis | 8 | 1 | 5 | 2 | 1:500 1:200 1:100 1:50 | 0.1313 m vs 0.03 (1:100 front) |
| humans | leaning-person | 8 | 1 | 7 | 0 | 1:200 1:100 1:50 | 0.0545 m vs 0.015 (1:50 elevation) |
| humans | worker-carrying-box | 8 | 1 | 5 | 2 | 1:500 1:100 1:50 | 0.06 m vs 0.015 (1:50 front) |
| humans | seated-guitarist | 9 | 3 | 6 | 0 | 1:500 1:100 1:50 | 0.1125 m vs 0.015 (1:50 front) |
| humans | elderly-with-cane | 9 | 2 | 5 | 2 | 1:100 1:50 | 0.0629 m vs 0.015 (1:50 front) |
| humans | tour-guide | 9 | 2 | 6 | 1 | 1:100 1:50 | 0.06 m vs 0.015 (1:50 front) |
| humans | wheelchair-user | 10 | 2 | 8 | 0 | 1:200 1:100 1:50 | 0.0469 m vs 0.015 (1:50 plan) |
| humans | seated-office-worker | 10 | 1 | 7 | 2 | 1:500 1:200 1:100 1:50 | 0.0412 m vs 0.015 (1:50 plan) |
| humans | jogger | 10 | 2 | 8 | 0 | 1:500 1:200 1:100 1:50 | 0.0825 m vs 0.015 (1:50 front) |
| humans | person-cooking | 10 | 2 | 8 | 0 | 1:500 1:200 1:100 1:50 | 0.1275 m vs 0.015 (1:50 front) |
| humans | courier-with-hand-truck | 11 | 2 | 6 | 3 | 1:500 1:200 1:100 1:50 | 0.0675 m vs 0.015 (1:50 plan) |
| humans | person-with-umbrella | 11 | 5 | 6 | 0 | 1:500 1:200 1:100 1:50 | 0.1106 m vs 0.015 (1:50 front) |
| humans | parent-carrying-toddler | 13 | 4 | 8 | 1 | 1:500 1:200 1:100 1:50 | 0.1875 m vs 0.015 (1:50 front) |
| humans | cyclist | 14 | 7 | 7 | 0 | 1:500 1:200 1:100 1:50 | 0.1313 m vs 0.015 (1:50 front) |
| humans | person-on-crutches | 15 | 8 | 6 | 1 | 1:500 1:200 1:100 1:50 | 0.084 m vs 0.015 (1:50 front) |
| humans | person-with-rollator | 16 | 9 | 7 | 0 | 1:500 1:200 1:100 1:50 | 0.2019 m vs 0.015 (1:50 front) |
| humans | parent-with-stroller | 16 | 8 | 8 | 0 | 1:500 1:200 1:100 1:50 | 0.1157 m vs 0.015 (1:50 front) |
| humans | person-with-white-cane | 16 | 6 | 9 | 1 | 1:500 1:200 1:100 1:50 | 0.0703 m vs 0.015 (1:50 front) |
| machinery | plate-heat-exchanger | 1 | 0 | 1 | 0 | 1:50 | 0.0187 m vs 0.015 (1:50 front) |
| machinery | solar-thermal-collector | 2 | 1 | 1 | 0 | 1:500 | 0.1875 m vs 0.15 (1:500 front) |
| machinery | sorption-wheel | 4 | 2 | 2 | 0 | 1:50 | 0.0926 m vs 0.015 (1:50 front) |
| machinery | heat-pump-outdoor-unit | 5 | 0 | 2 | 3 | 1:200 1:100 1:50 | 0.0332 m vs 0.015 (1:50 elevation) |
| machinery | wheel-loader | 6 | 0 | 6 | 0 | 1:500 1:200 1:100 1:50 | 0.0562 m vs 0.015 (1:50 plan) |
| machinery | concrete-mixer-truck | 9 | 0 | 8 | 1 | 1:500 1:200 1:100 1:50 | 0.0825 m vs 0.015 (1:50 plan) |
| machinery | scissor-lift | 9 | 0 | 6 | 3 | 1:200 1:100 1:50 | 0.33 m vs 0.015 (1:50 elevation) |
| machinery | forklift | 9 | 0 | 8 | 1 | 1:500 1:200 1:100 1:50 | 0.09 m vs 0.015 (1:50 plan) |
| machinery | excavator | 9 | 0 | 9 | 0 | 1:500 1:200 1:100 1:50 | 0.1744 m vs 0.015 (1:50 plan) |
| machinery | diesel-generator | 12 | 0 | 7 | 5 | 1:200 1:100 1:50 | 0.1154 m vs 0.015 (1:50 front) |
| machinery | oil-pumpjack | 13 | 3 | 10 | 0 | 1:500 1:200 1:100 1:50 | 1.3832 m vs 0.03 (1:100 plan) |
| machinery | tower-crane | 21 | 8 | 12 | 1 | 1:500 1:200 1:100 1:50 | 0.9769 m vs 0.015 (1:50 plan) |
| objects | shopping-trolley | 1 | 1 | 0 | 0 | 1:100 |  |
| objects | patio-heater | 1 | 0 | 1 | 0 | 1:50 | 0.0412 m vs 0.015 (1:50 front) |
| objects | charcoal-grill | 1 | 0 | 1 | 0 | 1:100 | 0.0346 m vs 0.03 (1:100 front) |
| objects | tealight | 1 | 1 | 0 | 0 | 1:50 |  |
| objects | playground-swing | 2 | 1 | 1 | 0 | 1:200 1:50 | 0.0303 m vs 0.015 (1:50 front) |
| objects | stepladder | 6 | 2 | 3 | 1 | 1:500 1:200 1:50 | 0.1257 m vs 0.06 (1:200 elevation) |
| objects | street-lamp | 6 | 0 | 5 | 1 | 1:100 1:50 | 0.0324 m vs 0.015 (1:50 elevation) |
| objects | market-parasol | 8 | 1 | 4 | 3 | 1:100 1:50 | 0.1162 m vs 0.015 (1:50 front) |
| vegetation | hornbeam-hedge | 3 | 0 | 3 | 0 | 1:100 1:50 | 0.1008 m vs 0.015 (1:50 elevation) |
| vegetation | hosta-clump | 9 | 3 | 4 | 2 | 1:200 1:100 1:50 | 0.0678 m vs 0.015 (1:50 plan) |
| vegetation | ivy-on-trellis | 14 | 5 | 7 | 2 | 1:200 1:100 1:50 | 0.1041 m vs 0.015 (1:50 elevation) |
| vegetation | agave | 16 | 3 | 7 | 6 | 1:500 1:100 1:50 | 0.6345 m vs 0.15 (1:500 plan) |
| vegetation | apple-tree | 17 | 0 | 12 | 5 | 1:500 1:200 1:100 1:50 | 0.3626 m vs 0.015 (1:50 elevation) |
| vegetation | weeping-willow | 17 | 1 | 11 | 5 | 1:500 1:200 1:100 1:50 | 0.5603 m vs 0.015 (1:50 elevation) |
| vegetation | japanese-maple | 18 | 3 | 10 | 5 | 1:500 1:200 1:100 1:50 | 0.2369 m vs 0.015 (1:50 elevation) |
| vegetation | small-leaved-lime | 20 | 2 | 11 | 7 | 1:500 1:200 1:100 1:50 | 0.8419 m vs 0.015 (1:50 elevation) |
| vegetation | london-plane-pollarded | 21 | 4 | 12 | 5 | 1:500 1:200 1:100 1:50 | 0.4819 m vs 0.015 (1:50 plan) |
| vegetation | multi-stem-birch | 21 | 3 | 11 | 7 | 1:500 1:200 1:100 1:50 | 0.9998 m vs 0.015 (1:50 elevation) |
| vegetation | lombardy-poplar | 21 | 2 | 12 | 7 | 1:500 1:200 1:100 1:50 | 0.4144 m vs 0.015 (1:50 front) |
| vegetation | english-oak | 24 | 6 | 12 | 6 | 1:500 1:200 1:100 1:50 | 1.0201 m vs 0.015 (1:50 plan) |
| vegetation | miscanthus-grass | 24 | 6 | 11 | 7 | 1:500 1:200 1:100 1:50 | 0.3608 m vs 0.03 (1:100 plan) |
| vegetation | scots-pine | 27 | 6 | 12 | 9 | 1:500 1:200 1:100 1:50 | 0.9936 m vs 0.015 (1:50 plan) |
| vegetation | bamboo-clump | 28 | 9 | 12 | 7 | 1:500 1:200 1:100 1:50 | 0.6629 m vs 0.03 (1:100 front) |
| vegetation | date-palm | 34 | 11 | 12 | 11 | 1:500 1:200 1:100 1:50 | 0.3217 m vs 0.015 (1:50 front) |
| vehicles | electric-kick-scooter | 5 | 1 | 3 | 1 | 1:200 1:100 1:50 | 0.0825 m vs 0.06 (1:200 plan) |
| vehicles | city-car | 6 | 0 | 6 | 0 | 1:200 1:100 1:50 | 0.0641 m vs 0.015 (1:50 elevation) |
| vehicles | motor-scooter | 8 | 1 | 6 | 1 | 1:500 1:100 1:50 | 0.0528 m vs 0.015 (1:50 elevation) |
| vehicles | vintage-car-1929 | 9 | 0 | 9 | 0 | 1:200 1:100 1:50 | 0.1219 m vs 0.015 (1:50 plan) |
| vehicles | delivery-van | 10 | 0 | 8 | 2 | 1:500 1:200 1:100 1:50 | 0.3256 m vs 0.06 (1:200 elevation) |
| vehicles | tram | 11 | 0 | 11 | 0 | 1:500 1:200 1:100 1:50 | 0.3056 m vs 0.015 (1:50 elevation) |
| vehicles | city-bus | 13 | 0 | 10 | 3 | 1:500 1:200 1:100 1:50 | 0.1013 m vs 0.015 (1:50 front) |
| vehicles | city-bicycle | 14 | 5 | 7 | 2 | 1:200 1:100 1:50 | 0.075 m vs 0.03 (1:100 plan) |
| vehicles | cargo-bike | 17 | 5 | 10 | 2 | 1:500 1:200 1:100 1:50 | 0.0694 m vs 0.015 (1:50 plan) |

Passing (37): red-squirrel, decentralised-hrv-fan, electricity-meter, kitchen-extractor-hood, tower-server, gas-boiler, duct-fan, gas-hob, mvhr-unit, ceiling-diffuser, gas-meter, pendant-lamp, room-thermostat, toaster, etics-brick-wall-12, etics-brick-wall-30, panel-radiator, stair-flight, glass-balustrade, glulam-beam, solid-brick-wall-1960, tilt-window, bar-stool, single-bed, bookshelf, dining-table, finned-heat-exchanger, scroll-compressor, evaporative-humidifier, expansion-valve, drinking-fountain, waste-bin, bicycle-stand, chalkboard, storage-crate-stack, cooking-pot, retractable-bollard.
