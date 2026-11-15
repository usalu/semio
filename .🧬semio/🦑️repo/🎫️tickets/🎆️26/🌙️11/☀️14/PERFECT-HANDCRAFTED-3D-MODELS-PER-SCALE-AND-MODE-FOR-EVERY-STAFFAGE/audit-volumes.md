# 📊 Audit of the 152 staffage volumes: solids, scales, thermal coverage

Read-only audit. No source file was changed and no state-changing git command was run.
Per-asset data: `audit-volumes.csv` in this folder (152 rows, one per asset; `_path` column gives the catalog path).
Generator: `audit-volume-extract.py` (stdlib Python, run as `python3 -I`). Tool output: `🗑️generated/audit-volume/` (`extract-stdout.txt`, `extract-stderr.txt`, `rankings.json`).
Catalog read: `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/<category>/<slug>/🔣️.json`, schema `🧬️schema/🔣️.json`.

## Definitions used

- **Solids, expanded**: `volume.solids` count; expanded adds `repeat.count` copies per solid.
- **Visibility at scale s**: a solid is shown at s when it has no `maxDenominator` or `s <= maxDenominator`. This follows the schema text ("the largest denominator that shows it"; 500 means every scale). The rule ignores pose-driven hiding and any 2D clipping.
- **Scale differentiation**: a solid with `maxDenominator` 50, 100 or 200. An asset with none has zero differentiation.
- **Thermal coverage**: a thermal region is covered when at least one solid names it in `region` or `regions`. Solids with neither belong to no thermal region (schema rule).
- **Bones covered**: a bone is covered when a solid is rigid to it (`bone`) or it appears in a solid's `weights`.
- **Structural members**: the schema has no member field in `structure` (only gravity, dynamics, supports, assumptions, clinical). Members exist only as 2D shapes of `kind: "member"` (`from`, `to`, `section`). The counts below use those shapes.
- **Crude score** (heuristic, used for the top 30): `0.4 * box share (expanded) + 0.3 * (1 - min(1, expanded solids / 1:50 elevation shapes)) + 0.2 * organic (animals, humans, vegetation) + 0.1 * zero scale differentiation`. Higher is cruder.

## Corpus totals

| measure | value |
|---|---|
| assets | 152 |
| solids / expanded by repeat | 4,903 / 7,533 |
| kinds (solids): box 873 (17.8 %), ellipsoid 536, capsule 1,125, revolve 628, extrude 368, loft 1,373 |  |
| box share, expanded | about 1,591 of 7,533 (21 %) |
| solids visible at 1:50 / 1:100 / 1:200 / 1:500 (solids) | 4,903 / 4,078 / 3,500 / 2,987 |
| solids visible at 1:50 / 1:100 / 1:200 / 1:500 (expanded) | 7,533 / 5,900 / 4,928 / 3,994 |
| solids with maxDenominator 50 / 100 / 200 | 825 / 578 / 513 |
| assets with any scale-specific solid | 124 of 152 |
| assets with zero scale differentiation | 28 |
| solids with a thermal region / without | 4,738 / 165 |
| thermal regions defined / without any solid | 959 / 10 (in 8 assets) |
| solids naming an undefined thermal region | 0 |
| bones / bones without solid (rigid or weights) | 1,887 / 106 (in 57 assets) |
| 2D member shapes at 1:50 (all scales) | 690 (866) in 3 assets |
| 1:50 elevation shapes (total) | 5,860 |
| solids (expanded) per 1:50 elevation shape, median | 1.07 |
| poses / clips | 931 / 359 |
| `volume.decisions` entries / English characters | 447 / 132,639 (1 to 6 entries and 338 to 1,733 characters per asset) |

Median per asset: 4 materials, 8 tones, 2 roles.

### Per category

| category | assets | solids | expanded | box % | zero scale diff | assets with unavailable 2D scale | thermal regions without solid | assets with bones without solid |
|---|---|---|---|---|---|---|---|---|
| vegetation | 16 | 731 | 776 | 4 % | 14 | 1 | 0 | 6 |
| machinery | 16 | 574 | 1,540 | 33 % | 1 | 7 | 0 | 3 |
| animals | 14 | 418 | 420 | 0 % | 1 | 7 | 0 | 10 |
| humans | 28 | 1,410 | 1,662 | 5 % | 0 | 1 | 7 | 13 |
| appliances | 24 | 352 | 563 | 33 % | 0 | 21 | 1 | 1 |
| vehicles | 9 | 534 | 1,199 | 23 % | 0 | 2 | 2 | 1 |
| furniture | 12 | 224 | 335 | 55 % | 5 | 0 | 0 | 9 |
| components | 18 | 291 | 583 | 50 % | 6 | 3 | 0 | 10 |
| objects | 15 | 369 | 455 | 23 % | 1 | 2 | 0 | 4 |

## (a) The 30 crudest models (crude score, highest first)

| # | category/slug | solids (expanded) | box % (expanded) | 1:50 elev shapes | solids per shape | zero scale diff | score |
|---|---|---|---|---|---|---|---|
| 1 | components/glulam-beam | 3 (3) | 100 | 23 | 0.13 | yes | 0.761 |
| 2 | vegetation/hornbeam-hedge | 24 (24) | 50 | 50 | 0.48 | yes | 0.656 |
| 3 | vegetation/ivy-on-trellis | 21 (28) | 86 | 28 | 1.00 | yes | 0.643 |
| 4 | components/masonry-wall-with-lintel | 16 (16) | 75 | 57 | 0.28 | yes | 0.616 |
| 5 | furniture/single-bed | 13 (42) | 100 | 13 | 3.23 | yes | 0.500 |
| 6 | vegetation/bamboo-clump | 30 (30) | 0 | 79 | 0.38 | yes | 0.486 |
| 7 | vegetation/date-palm | 43 (57) | 0 | 123 | 0.46 | yes | 0.461 |
| 8 | components/curtain-wall-bay | 22 (30) | 90 | 12 | 2.50 | yes | 0.460 |
| 9 | vegetation/english-oak | 37 (37) | 0 | 71 | 0.52 | yes | 0.444 |
| 10 | vegetation/agave | 20 (20) | 0 | 38 | 0.53 | yes | 0.442 |
| 11 | furniture/height-adjustable-desk | 30 (32) | 97 | 37 | 0.87 | no | 0.428 |
| 12 | furniture/picnic-table | 9 (19) | 79 | 11 | 1.73 | yes | 0.416 |
| 13 | furniture/three-seat-sofa | 11 (13) | 46 | 23 | 0.57 | yes | 0.415 |
| 14 | humans/worker-carrying-box | 37 (38) | 13 | 78 | 0.49 | no | 0.407 |
| 15 | components/etics-brick-wall-12 | 5 (16) | 100 | 16 | 1.00 | no | 0.400 |
| 16 | components/etics-brick-wall-30 | 5 (16) | 100 | 16 | 1.00 | no | 0.400 |
| 17 | components/solid-brick-wall-1960 | 4 (15) | 100 | 15 | 1.00 | no | 0.400 |
| 18 | furniture/bookshelf | 26 (36) | 100 | 26 | 1.39 | no | 0.400 |
| 19 | vegetation/london-plane-pollarded | 57 (57) | 0 | 80 | 0.71 | yes | 0.386 |
| 20 | vegetation/small-leaved-lime | 76 (76) | 0 | 103 | 0.74 | yes | 0.379 |
| 21 | appliances/tower-server | 12 (14) | 93 | 11 | 1.27 | no | 0.371 |
| 22 | machinery/finned-heat-exchanger | 9 (75) | 67 | 28 | 2.68 | yes | 0.367 |
| 23 | humans/tour-guide | 38 (39) | 5 | 75 | 0.52 | no | 0.365 |
| 24 | furniture/dining-table | 10 (17) | 65 | 10 | 1.70 | yes | 0.359 |
| 25 | humans/standing-phone-user | 39 (39) | 8 | 65 | 0.60 | no | 0.351 |
| 26 | machinery/diesel-generator | 30 (89) | 88 | 15 | 5.93 | no | 0.350 |
| 27 | appliances/server-rack | 26 (102) | 87 | 72 | 1.42 | no | 0.349 |
| 28 | components/tied-canopy | 16 (19) | 74 | 23 | 0.83 | no | 0.347 |
| 29 | vegetation/multi-stem-birch | 53 (53) | 0 | 62 | 0.86 | yes | 0.344 |
| 30 | appliances/electric-fan-heater | 19 (32) | 84 | 30 | 1.07 | no | 0.338 |

Reading the list:

- The criterion "box-dominated organic subjects" matches only two assets: `ivy-on-trellis` (86 % boxes) and `hornbeam-hedge` (50 %). Every other organic entry here is crude by sparse solid count and zero scale differentiation, not by boxes.
- Organic assets with zero scale differentiation (15 of 58): bamboo-clump, date-palm, english-oak, agave, hornbeam-hedge, ivy-on-trellis, japanese-maple, lombardy-poplar, london-plane-pollarded, miscanthus-grass, multi-stem-birch, scots-pine, small-leaved-lime, weeping-willow (vegetation), hedgehog (animals).
- Furniture and components dominate by box share (furniture 55 %, components 50 %). Many of them are rightly box-shaped (bricks, walls, beams), so box share alone does not mark a model as crude.
- `single-bed` and `curtain-wall-bay` have more solids in expanded form than 1:50 shapes suggest (ratio 3.2 and 2.5). That is detail the 2D does not draw, not a shortfall.

## (b) Assets with zero scale differentiation (28)

No solid has `maxDenominator` 50, 100 or 200, so the 3D model is identical at every scale.

- vegetation (14): agave, bamboo-clump, date-palm, english-oak, hornbeam-hedge, ivy-on-trellis, japanese-maple, lombardy-poplar, london-plane-pollarded, miscanthus-grass, multi-stem-birch, scots-pine, small-leaved-lime, weeping-willow
- components (6): curtain-wall-bay, external-venetian-blind, glulam-beam, masonry-wall-with-lintel, spiral-stair, wheelchair-ramp
- furniture (5): dining-table, operating-table, picnic-table, single-bed, three-seat-sofa
- machinery (1): finned-heat-exchanger; animals (1): hedgehog; objects (1): tealight

The animals category is the contrast case: 13 of 14 assets carry scale-specific detail. The earlier authoring report says `maxDenominator` is "left out for structural parts, because the coarse 2D scales merge limbs into bars" (`authoring-animals.md:37`), which matches the data.

The 1:200 and 1:500 scales are also 2D-thin in many assets (section 6 below), so the zero-differentiation list overlaps with missing 2D detail.

## (c) Assets with thermal regions not represented in 3D (8 assets, 10 regions)

Each row lists regions defined in `thermal.regions` that no solid names in `region` or `regions`. Column "solids without region" counts solids that belong to no thermal region.

| asset | regions | without solid | ids | solids without region |
|---|---|---|---|---|
| humans/leaning-person | 10 | 2 | arm-skin, wall-face | 0 |
| humans/skateboarder | 10 | 2 | deck, trucks | 3 |
| humans/tour-guide | 11 | 1 | arm-skin | 0 |
| humans/person-with-umbrella | 10 | 1 | torso-clothing | 0 |
| humans/wheelchair-user | 10 | 1 | hand-skin | 0 |
| vehicles/delivery-van | 10 | 1 | bonnet | 23 |
| vehicles/electric-kick-scooter | 8 | 1 | bms-controller | 0 |
| appliances/shower-head | 6 | 1 | spray | 0 |

Note: `wall-face` (leaning-person) and `spray` (shower-head) are surfaces with no physical volume to match. The others are body or vehicle surfaces that the solids do not carry, so the thermal model has a region the 3D does not heat-balance. Thermal physics was not checked.

## (d) Earlier reports: flagged, approximated, compromised or failing

Quotes are from the earlier ticket `ENGINE-NEUTRAL-3D-MODELS-FOR-EVERY-STAFFAGE` (`☀️10`). Items marked "status" were checked against the current catalog.

Failing or missed gates, per report:

- `authoring-animals.md:20` red-squirrel: "GATE MISSED: x +98.2 %" (the drawn 2D is 0.416 m long, dimensions say 0.21 m). `:21` roe-deer: "GATE MISSED: y +10.3 %", "the body follows the drawing". `:22` sheep: "Thigh and humerus are not drawn (wool hides them)" and the upper leg bones carry wool tubes.
- `authoring-appliances-b.md:14` laptop: "gate miss by construction" (open pose bbox). `:24` vacuum-cleaner: "gate miss by construction". `:32` shower-head: IoU elevation 0.688, hose "authored as an open stroked polyline".
- `authoring-appliances-a.md:12` circulation-pump: "2D inconsistency: the terminal box is drawn 5 mm further out in the front view".
- `authoring-components.md:13` external-venetian-blind: "lamellae are 0.5 mm arched sheets (2D draws 12 mm bars)", elevation IoU 0.898. The current audit shows 17 of its 37 bones (`slide-0` to `slide-16`) carry no solid, so the lamellae are not in 3D. Consistent with the report.
- `authoring-components.md:18` sectional-garage-door: "GATE MISSED elevation 0.320". `:21` spiral-stair: "GATE MISSED elevation 0.687, front 0.732". `:30` names panel-radiator, sectional-garage-door and spiral-stair as missed IoU gates.
- `authoring-humans-b.md:9` pregnant-person: rest bbox x -51.0 %, "Rest-pose bbox x/z differ from dimensions". `:27`: seven humans miss the rest bbox gate.
- `authoring-humans-a.md:43` kneeling-gardener: "unavailable-but-drawable" for 1:200 and 1:500. Current data confirms: 1:200 and 1:500 `status` is "unavailable" for this asset (see section 6).
- `authoring-machinery.md:12` excavator: rest bbox x +18.1 %, y +63.8 %, "fails only because rest has boom angle 40". `:19` scissor-lift: "Ajv: 994 errors at /volume/solids/N/bone" (rig bones `A1..B5` are not Slug-valid). `:23` and `:28` tower-crane: "GATES NOT MET: IoU 0.360/0.462/0.572".
- `authoring-vehicles.md:11` cargo-bike and `:23` cargo-bike elevation IoU 0.788: "tyre-r/f and rim-r/f are filled discs". `:14` and `:21` delivery-van: "bbox x +17.0 %", rear door leaves swung open at rest.
- `authoring-vegetation.md:9` agave: "FAIL: bbox z (depthM): -89.6 % > 8 %; front IoU 0.288; plan 0.269", with the rest-pose evaluation throwing (`:31`). `:12` date-palm: "FAIL: bbox x 13.0 %; elevation IoU 0.467, front 0.456, plan 0.181". `:15` hosta-clump: "FAIL: bbox z -69.4 %". `:20` miscanthus-grass: "FAIL: elevation IoU 0.550, front 0.406, plan 0.294".
- `authoring-objects.md:22` cooking-pot: "FAILS elevation gate (0.60)". `:24` market-parasol: "FAILS rest gates (z -53 %, y +11 %)". `:26` playground-swing: "FAILS front (0.76) and plan (0.77)". `:27` retractable-bollard: "FAILS rest gates (y -33 %, elevation 0.70, front 0.71)". `:28` shopping-trolley: "FAILS front (0.60, recall 0.64)".
- `volume-agreement.md:3`: "137 of 152 pass, 15 miss". The misses table (lines below) lists agave, bamboo-clump, date-palm, miscanthus-grass, tower-crane, hedgehog, person-with-umbrella, cargo-bike, city-bicycle, panel-radiator, sectional-garage-door, spiral-stair, cooking-pot, market-parasol, retractable-bollard. `:27`: "Genuine modelling errors found: date-palm".
- `silhouette-15.md:5` says the later result is "152 of 152 assets pass" in the default pose at 1:50 (bbox at most 8 %, IoU at least 0.80). This supersedes the 15 misses above, but only for 1:50 and the default pose.
- `silhouette-15.md:118` (section 6, "Not done, stated"): "Poses other than the default and scales other than 1:50 are not guarded: the 1:100 / 1:200 simplifications of tower-crane, the furled poses of market-parasol (bounding box -69 %) and person-with-umbrella (front 0.70, plan 0.43 furled), city-bicycle and cargo-bike on the stand differ from their drawings." This is the main unverified area: no guard covers the 1:100 and 1:200 volumes.
- `silhouette-15.md:17`: "the elevation-section drawings (cooking-pot, retractable-bollard, solar-thermal-collector, panel-radiator, sectional-garage-door) could not be declared" before the schema was extended. Extended later; not re-checked here.
- `authoring-machinery.md:29` and `authoring-requests.md:25`: scissor-lift rig ids fail Ajv. Not re-checked here.
- `authoring-furniture.md:7` and `:30`: "The live catalog still lacks the mandatory solid.sweep on every extruded hint" (7 assets). Status: no longer present. The current catalog has 4,547 `extruded` hints and 0 without `sweep`.
- `authoring-requests.md:37` (market-parasol): "rests with all eight ribs collinear ... rest bbox z is -53 %". Same root cause as `authoring-objects.md:24`.
- `catalog-audit.md:480` tower-server and `:483` chalkboard: Ajv errors on 2D shapes (missing `tone`, extra rect keys). These are 2D, not volume; not re-checked.

## 6. Representation gaps found by this audit (not flagged by the earlier reports, except kneeling-gardener)

- 44 assets have at least one representation at 1:200 or 1:500 with `status: "unavailable"` and no geometry: 44 entries at 1:500 and 6 at 1:200 (expansion-valve, hedgehog, red-squirrel, kneeling-gardener, room-thermostat, thermostatic-radiator-valve appear at 1:200). Each carries `limitingFeature` text, for example: "Legs of the cat (real width 0.025 m = 0.05 mm)" (`animals/domestic-cat`, 1:500), "Paws and limbs (0.02 m = 0.1 mm, below every profile)" (`animals/hedgehog`, 1:200 and 1:500), "Front bezel depth of 25 mm" (`appliances/tower-server`, 1:500).
- So at those scales the 3D solids remain, but no 2D shape stands beside them. Whether these scales are drawn from the solids alone was not checked.
- `appliances` has the most affected assets (21 of 24).

## Bones without solid (57 assets, 106 bones)

Top cases: `components/external-venetian-blind` 17 of 37 (`slide-0` to `slide-16`); `humans/kneeling-gardener`, `humans/seated-guitarist`, `humans/skateboarder`, `humans/sleeping-person` 4 each (`hip-lift`, `hip-shift`, `toe-l`/`toe-r`, `pillow`, `duvet`); `vegetation/ivy-on-trellis` 3 (`root-plate`, `crown-l`, `crown-r`); `animals/sheep` 3 (`pelvis`, `scapula-fn`, `scapula-ff`). Most uncovered bones are control or joint bones (`hip-lift`, `hip-shift`). Bones with a visible part and no solid are the real gaps (venetian-blind slides, sheep pelvis, ivy crown and root plate).

## Structural members (2D `member` shapes)

Only 3 assets define members: `machinery/tower-crane` (19 bones, 674 at 1:50, 866 over all scales, 5 bones covered at 1:50), `objects/market-parasol` (14 bones, 12 at 1:50, 6 bones covered), `vehicles/city-bicycle` (8 bones, 4 at 1:50, 1 bone covered). The other 149 assets have no structural members in any representation. The structural model itself lists only supports, not members.

## What was not checked

- No 3D geometry was built, tessellated or rendered. Silhouettes, IoU and bounding boxes were not recomputed; earlier reports were not re-run.
- The visibility rule (solid shown at s when `maxDenominator` is absent or `s <= maxDenominator`) is read from the schema text. Pose-driven hiding, `repeat` overlap and clipping by 2D views were not modelled.
- The crude score is my heuristic, weights chosen by me. Other weights would change the top-30 order.
- Schema validation (Ajv) was not run; only the schema enums and definitions were read.
- Pose and clip contents were counted, not evaluated. `volume.decisions` were measured as length only, not read for accuracy.
- Thermal region semantics (surface physics, layers, emitters) were not checked; only the region ids and their solid bindings.
- Earlier reports were read by grep and targeted line ranges, not in full. Items not quoted above may exist.
- The `🎯️targets`, `🔨️modules`, `🧪️tests` and `🌐️site` trees were not inspected.
- Other agents write to this ticket folder (`audit-2d-*.py`, `health-run.sh`); their outputs were not read or used.
- Only 2D shape counts were used (elevation at 1:50 for the ratio); the plan and front counts are in the CSV only.
