# Validator maintainer report

Maintainer: validator (thermal side, speed, catalog runs). Module `S/🔨️modules/🗂️catalog/🧊️volumes`, swaps 12-15 in `schema-api.md`. Scripts here: `validator-*` (`validator-loop.sh` runs `schema-catalog-check.ts --workers 3` niced every 30 minutes and `validator-trend.py` fills Trend and Ready below; `validator-prof.ts`, `validator-compare.ts`, `validator-res*.ts` profile and compare scratch and live module). Output: `🗑️generated/validator/` (profiles `prof/`, per-asset result cache `cache/`, `trend.jsonl`).

## What landed (all with fixtures, tests and oracle where a third party can compute it; `volumes-rules` 96 tests)

- C22 envelope thermal solids out of the area sums; C23 `ViewSection.retained` (schema, twin, `referenceOf`, README sentence; oracle trimesh `slice_plane` + shapely).
- Information measures: `info:outerAreaM2`, `info:exposedAreaM2`, `info:exposureMeasured/Target/Deviation:<region>`.
- C29: the failing area measure is the outer area of the union of the region's own solids; exposure is information. Burial is area-weighted (samples at most two features apart; laptop on a table top; oracle trimesh surface samples).
- C24 as of swap 13 (slender solid as pen-widened stroke); C24 refined, tangent hairline and rotated-capsule `thinnestDrawn` went to the silhouette maintainer.
- C26 a, b, c (porous masses: bury nothing, soft outlines of shapes bound to exchange regions, soft cross-scale band). C30: `volumes-feature` already judges overall extents (`sortedExtents`); fixture `hollow` pins it.
- Emitters: out of area sums, only their centre in the body box; structure containment findings name the worst solids.
- Resolution defect (author 42 bollard, 48 wheel-loader, 35 crane): `encloses` was decided per tile window and the tile margin (3 features) was below the 4-feature limits. Fixed at the root; tests for resolution and tile invariance. Distances beyond a tile margin (now 6 features) count as the margin.
- Union-volume band: cell = band / 4, tiled blocks, up to 150 M voxels (excavator collapse gone).
- Sample figure is a valid asset again (a 1 mm head shift per scale and shoes at 1:500 in the fixture; `vectors.mechanical` restores the copy that the copied and bones tests need). Suites green after swap 15: volumes-rules 93 (96 with C26/C30 added, run in progress when written), model-silhouette 16, volume-traps 13, asset-validation 73, volume-schema 99; schema-conformance and section-views failures are catalog data in flux (assets of authors), not the validator.

## Speed

Profile (CPU, loaded machine): english-oak 1:50 silhouette 13.9 s, cross-scale 1:100 13.7 s, oak total about 50 s CPU; sectional-garage-door 1:50 silhouette 38 s CPU (91 s under load), all of it in `Mask.fill`, bucket queries and distance transforms. Done: nearest-boundary search replaces two of three distance transforms per tile (identical outputs on 300 random masks), one flood for enclosed holes, allocation-free triangle fast path of `fill`, cached bucket boxes, allocation-free `mayTouch`; and the catalog check caches the per-asset result by the hash of the module sources and the asset (`🗑️generated/validator/cache`, `--no-cache` to force), so an unchanged asset costs 0 s and the 30-minute runs only pay for assets the authors touched or after a swap. Cold full run: 933 s wall at 3 workers under load 20-30 (119 assets), 509 s on the second run (partially cached). Remaining hot spots: Mask.fill of large foliage and door panels per tile; the next step would be per-solid projected rasters cached by content hash (not done).

## False-failure review (task 5)

Margins from run 1 (`🗑️generated/validator/margins.txt`): the three smallest margins per rule are genuine marginal fails (ratios 1.00 to 1.15). Overlay sheets opened: date-palm 1:500 (fronds as slabs: model wrong), bamboo-clump 1:200 (plan body is a slab: model wrong), dining-table 1:500 (legs stop above the floor: model wrong). Two validator defects found by the same review of reports: the resolution/section defect (bollard, wheel-loader) and the thermal comparison against exposed instead of outer area (C29); both fixed. No further false failure found in the sample; not every rule was viewed (time).

## Proposals for the coordinator (limits unchanged)

- `halo: true` (bicycle-stand, stepladder, pendant-lamp cord): a body solid drawn wider than the object because of the pen floor buries nothing and counts no area. With C29 the thermal comparison already ignores burial by other regions, so the remaining need is only in the silhouette/volume measures; decide after the catalog run shows whether the pen-floor assets still fail.
- kneeling-gardener `hat` (body model, 0.226 m2 declared, brim of 0.47 m diameter has 0.35 m2 on its own): C18 forbids changing it; either the declared value is the one-sided brim or the model is one-sided. Needs a decision.
- dairy-cow 1:200 length: 2.4 mm over the 2 % limit; drawing decision.
- japanese-maple cross-scale volume (1:500 crown of one closed mass against three pads): C26 c gives it the soft band; re-check after the catalog run.

## Not done (queue)

Tangent hairline gap, rotated-capsule `thinnestDrawn`, C24 refined, curved/closed slender axis: silhouette maintainer. `volumes-render.ts --source` ignores the given file (tool, not product). Hat/halo decisions above.

<!-- trend:begin -->
## Trend

Failing assets per rule after each run of `schema-catalog-check.ts --workers 3` (nice 15), machine under load.

| time | wall s | spliced | clean | failing | bones | bounds | copied | cross-scale | feature | monotonic | silhouette | solids | structure-coverage | thermal-coverage | views-disagree |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 21:50 | 933 | 119 | 19 | 100 | 3 | 11 | 1 | 19 | 7 | 13 | 45 | 1 | 22 | 93 | 20 |
| 22:12 | 509 | 131 | 30 | 101 | 3 | 7 | 1 | 17 | 6 | 9 | 33 | 1 | 21 | 93 | 13 |
| 00:02 | 5280 | 119 | 10 | 109 | 1 | 4 | 0 | 17 | 3 | 3 | 22 | 1 | 8 | 82 | 5 |

## Ready

Spliced assets with no finding at any authored scale in the last run (00:02): 10.

bicycle-stand, cafe-chair, dining-table, finned-heat-exchanger, folding-deck-chair, labrador-dog, market-parasol, red-fox, stair-flight, standing-adult

<!-- trend:end -->

<!-- lists:begin -->
## Trend (validator3)

Failing assets per rule after each run (before -> after in the last two lines).

| time | wall s | spliced | clean | failing | bones | bounds | copied | cross-scale | feature | monotonic | silhouette | solids | structure-coverage | thermal-coverage | views-disagree |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 23:34 |  | 131 | 30 | 101 | 3 | 7 | 1 | 17 | 6 | 9 | 33 | 1 | 21 | 93 | 13 |

## Ready

Clean at every authored scale in the run of 23:34: 30.

bar-stool, bookshelf, cafe-chair, charcoal-grill, circulation-pump, city-bus, city-car, cooking-pot, decentralised-hrv-fan, domestic-cat, elderly-with-cane, electric-fan-heater, etics-brick-wall-12, etics-brick-wall-30, external-venetian-blind, folding-deck-chair, hedgehog, hot-water-cylinder, jogger, leaning-person, mvhr-unit, patio-heater, pregnant-person, solid-brick-wall-1960, split-ac-indoor-unit, stair-flight, standing-adult, steel-column, storage-crate-stack, thermostatic-radiator-valve

## Thermal only

Assets whose only findings are `volumes-thermal-coverage` (the area reviewers' worklist), 51: region at scale, measured outer area against areaM2, deviation against limit.

- **objects/bicycle-stand**: all regions 1:500 (0.088 vs 0.426 m2, 79 % > 30 %)
- **appliances/ceiling-diffuser**: plate 1:50 (0.212 vs 0.45 m2, 53 % > 15 %); plate 1:100 (0.254 vs 0.45 m2, 43 % > 30 %); all regions 1:200 (0.297 vs 0.49 m2, 39 % > 30 %)
- **machinery/concrete-mixer-truck**: engine-bay 1:50 (0.91 vs 3.2 m2, 72 % > 15 %); engine-bay 1:100 (0.976 vs 3.2 m2, 70 % > 30 %)
- **vehicles/delivery-van**: exhaust-system 1:50 (0.759 vs 1.2 m2, 37 % > 15 %); exhaust-system 1:100 (0.759 vs 1.2 m2, 37 % > 30 %)
- **appliances/desktop-monitor**: housing 1:50 (0.124 vs 0.22 m2, 44 % > 15 %); housing 1:100 (0.111 vs 0.22 m2, 50 % > 30 %)
- **furniture/dining-table**: table-top 1:50 (2.31 vs 1.44 m2, 61 % > 15 %); table-top 1:100 (2.31 vs 1.44 m2, 60 % > 30 %)
- **objects/drinking-fountain**: water-jet 1:100 (0.011 vs 0.0071 m2, 56 % > 30 %)
- **appliances/duct-fan**: casing 1:50 (0.212 vs 0.1 m2, 112 % > 15 %); casing 1:100 (0.203 vs 0.1 m2, 103 % > 30 %); hub 1:100 (0.019 vs 0.0145 m2, 34 % > 30 %); all regions 1:200 (0.218 vs 0.143 m2, 52 % > 30 %)
- **appliances/electric-kettle**: lid 1:50 (0.034 vs 0.025 m2, 38 % > 15 %); lid 1:100 (0.039 vs 0.025 m2, 55 % > 30 %)
- **appliances/electricity-meter**: window 1:50 (0.025 vs 0.011 m2, 131 % > 15 %); plate 1:50 (0.012 vs 0.0078 m2, 54 % > 15 %); window 1:100 (0.019 vs 0.011 m2, 77 % > 30 %); plate 1:100 (0.016 vs 0.0078 m2, 99 % > 30 %)
- **machinery/evaporative-humidifier**: trough 1:50 (0.118 vs 0.2 m2, 41 % > 15 %); trough 1:100 (0.113 vs 0.2 m2, 44 % > 30 %)
- **machinery/finned-heat-exchanger**: all regions 1:200 (0.193 vs 0.32 m2, 40 % > 30 %)
- **machinery/forklift**: chassis-paint 1:50 (5.13 vs 8 m2, 36 % > 15 %); battery-pack 1:50 (2.31 vs 4.5 m2, 49 % > 15 %); drive-motor 1:50 (0.142 vs 0.9 m2, 84 % > 15 %); mast 1:50 (2.32 vs 5 m2, 54 % > 15 %); cargo 1:50 (3.14 vs 5 m2, 37 % > 15 %); chassis-paint 1:100 (5.13 vs 8 m2, 36 % > 30 %); battery-pack 1:100 (2.29 vs 4.5 m2, 49 % > 30 %); drive-motor 1:100 (0.14 vs 0.9 m2, 84 % > 30 %); +1 more
- **appliances/gas-boiler**: controls 1:50 (0.015 vs 0.03 m2, 49 % > 15 %); flue-collar 1:50 (0.024 vs 0.05 m2, 53 % > 15 %); connections 1:50 (0.025 vs 0.04 m2, 39 % > 15 %); controls 1:100 (0.011 vs 0.03 m2, 62 % > 30 %); flue-collar 1:100 (0.027 vs 0.05 m2, 45 % > 30 %); connections 1:100 (0.022 vs 0.04 m2, 46 % > 30 %)
- **appliances/gas-hob**: knob 1:50 (0.001 vs 0.0025 m2, 60 % > 15 %); burner 1:50 (0.006 vs 0.012 m2, 47 % > 15 %); knob 1:100 (0.001 vs 0.0025 m2, 51 % > 30 %); burner 1:100 (0.005 vs 0.012 m2, 58 % > 30 %); flame 1:100 (0.004 vs 0.008 m2, 53 % > 30 %)
- **components/glass-balustrade**: panel-glass 1:50 (4.2 vs 2.2 m2, 91 % > 15 %); clamp-steel 1:50 (1.01 vs 0.1 m2, 914 % > 15 %); handrail-steel 1:50 (0.222 vs 0.13 m2, 70 % > 15 %); panel-glass 1:100 (3.88 vs 2.2 m2, 77 % > 30 %); clamp-steel 1:100 (1.06 vs 0.1 m2, 960 % > 30 %); handrail-steel 1:100 (0.222 vs 0.13 m2, 71 % > 30 %); all regions 1:200 (5 vs 2.43 m2, 106 % > 30 %)
- **components/glulam-beam**: beam-top 1:50 (1.75 vs 1.28 m2, 37 % > 15 %); beam-soffit 1:50 (1.75 vs 1.28 m2, 37 % > 15 %); bearing-steel 1:50 (0.061 vs 0.04 m2, 52 % > 15 %); beam-top 1:100 (0.71 vs 1.28 m2, 45 % > 30 %); beam-soffit 1:100 (0.71 vs 1.28 m2, 45 % > 30 %); bearing-steel 1:100 (0.066 vs 0.04 m2, 66 % > 30 %)
- **animals/goat**: horns 1:50 (0.058 vs 0.012 m2, 384 % > 30 %); hooves 1:50 (0.067 vs 0.012 m2, 456 % > 30 %); horns 1:100 (0.058 vs 0.012 m2, 379 % > 30 %); hooves 1:100 (0.077 vs 0.012 m2, 543 % > 30 %); all regions 1:200 (0.124 vs 0.054 m2, 129 % > 30 %); all regions 1:500 (0.345 vs 0.054 m2, 540 % > 30 %)
- **machinery/heat-pump-outdoor-unit**: casing 1:50 (6.12 vs 3.4 m2, 80 % > 15 %); discharge-vane 1:50 (0.414 vs 0.25 m2, 66 % > 15 %); feet 1:50 (0.187 vs 0.1 m2, 87 % > 15 %); casing 1:100 (6.2 vs 3.4 m2, 82 % > 30 %); discharge-vane 1:100 (0.404 vs 0.25 m2, 62 % > 30 %); feet 1:100 (0.187 vs 0.1 m2, 87 % > 30 %)
- **appliances/kitchen-extractor-hood**: canopy 1:50 (0.537 vs 0.9 m2, 40 % > 15 %); filter 1:50 (0.452 vs 0.2 m2, 126 % > 15 %); chimney 1:50 (0.726 vs 0.55 m2, 32 % > 15 %); canopy 1:100 (0.527 vs 0.9 m2, 41 % > 30 %); filter 1:100 (0.469 vs 0.2 m2, 135 % > 30 %); chimney 1:100 (0.735 vs 0.55 m2, 34 % > 30 %)
- **objects/market-parasol**: frame-alu 1:50 (1.13 vs 0.6 m2, 89 % > 15 %); frame-alu 1:100 (1.14 vs 0.6 m2, 89 % > 30 %)
- **components/masonry-wall-with-lintel**: masonry-field 1:50 (7.31 vs 6.03 m2, 21 % > 15 %); lintel 1:50 (0.507 vs 0.438 m2, 16 % > 15 %); sill 1:50 (0.215 vs 0.117 m2, 84 % > 15 %); masonry-field 1:100 (18 vs 6.03 m2, 198 % > 30 %); lintel 1:100 (0.98 vs 0.438 m2, 124 % > 30 %); sill 1:100 (0.593 vs 0.117 m2, 408 % > 30 %); all regions 1:200 (19.5 vs 6.58 m2, 197 % > 30 %); all regions 1:500 (21.4 vs 6.58 m2, 225 % > 30 %)
- **furniture/operating-table**: box 1:50 (box); box 1:100 (box); box 1:200 (box); box 1:500 (box); table-frame 1:50 (0.971 vs 0.807 m2, 20 % > 15 %); light-housing 1:50 (0.44 vs 0.35 m2, 26 % > 15 %)
- **components/panel-radiator**: bracket 1:50 (0.01 vs 0.004 m2, 151 % > 15 %); valve 1:50 (0.059 vs 0.012 m2, 389 % > 15 %); bracket 1:100 (0.014 vs 0.004 m2, 249 % > 30 %); valve 1:100 (0.055 vs 0.012 m2, 361 % > 30 %)
- **furniture/park-bench**: back-timber 1:50 (1.25 vs 1.7 m2, 26 % > 15 %)
- **humans/person-with-umbrella**: torso-clothing 1:50 (0.007 vs 0.037 m2, 81 % > 30 %); torso-clothing 1:100 (0.009 vs 0.037 m2, 76 % > 30 %)
- **furniture/picnic-table**: a-frame 1:50 (1.18 vs 0.998 m2, 19 % > 15 %)
- **animals/pig**: undefined 1:50 (box); undefined 1:100 (box); undefined 1:200 (box); undefined 1:500 (box); skin-head 1:50 (0.156 vs 0.1 m2, 56 % > 30 %); ears 1:50 (0.108 vs 0.07 m2, 54 % > 30 %); tail 1:50 (0.006 vs 0.01 m2, 43 % > 30 %); hooves 1:50 (0.101 vs 0.018 m2, 461 % > 30 %); +3 more
- **animals/pigeon**: legs-feet 1:50 (0.013 vs 0.002 m2, 565 % > 30 %); legs-feet 1:100 (0.01 vs 0.002 m2, 380 % > 30 %); all regions 1:200 (0.011 vs 0.003 m2, 317 % > 30 %)
- **animals/red-fox**: undefined 1:50 (box); undefined 1:100 (box); undefined 1:200 (box)
- **animals/roe-deer**: nose-skin 1:50 (0.001 vs 0.004 m2, 77 % > 30 %); hooves 1:50 (0.066 vs 0.013 m2, 409 % > 30 %); nose-skin 1:100 (0.001 vs 0.004 m2, 75 % > 30 %); hooves 1:100 (0.067 vs 0.013 m2, 416 % > 30 %); nose-skin 1:200 (box); all regions 1:200 (0.069 vs 0.014 m2, 393 % > 30 %); all regions 1:500 (0.204 vs 0.018 m2, 1033 % > 30 %)
- **appliances/room-thermostat**: dial 1:100 (0.004 vs 0.00243 m2, 78 % > 30 %)
- **humans/runner-with-blade-prosthesis**: hand-skin 1:50 (0.039 vs 0.0888 m2, 56 % > 30 %); singlet 1:50 (0.271 vs 0.541 m2, 50 % > 30 %); hand-skin 1:100 (0.04 vs 0.0888 m2, 55 % > 30 %); singlet 1:100 (0.269 vs 0.541 m2, 50 % > 30 %); socket 1:100 (0.064 vs 0.0959 m2, 33 % > 30 %)
- **humans/running-child**: face-skin 1:50 (0.061 vs 0.0406 m2, 50 % > 30 %); tshirt 1:50 (0.213 vs 0.312 m2, 32 % > 30 %); face-skin 1:100 (0.059 vs 0.0406 m2, 47 % > 30 %); tshirt 1:100 (0.209 vs 0.312 m2, 33 % > 30 %)
- **machinery/scroll-compressor**: dome 1:50 (0.047 vs 0.063 m2, 26 % > 15 %); scroll 1:50 (0.014 vs 0.006 m2, 133 % > 15 %); scroll 1:100 (0.014 vs 0.006 m2, 135 % > 30 %)
- **humans/seated-office-worker**: chair-plastic 1:50 (0.922 vs 0.35 m2, 163 % > 15 %); shirt 1:100 (0.783 vs 1.13 m2, 31 % > 30 %); chair-plastic 1:100 (0.915 vs 0.35 m2, 162 % > 30 %)
- **appliances/server-rack**: feet 1:50 (0.025 vs 0.06 m2, 59 % > 15 %); feet 1:100 (0.025 vs 0.06 m2, 58 % > 30 %)
- **animals/sheep**: undefined 1:50 (box); undefined 1:100 (box); undefined 1:200 (box); undefined 1:500 (box); ears 1:50 (0.035 vs 0.022 m2, 61 % > 30 %); hooves 1:50 (0.101 vs 0.016 m2, 529 % > 30 %); ears 1:100 (0.043 vs 0.022 m2, 94 % > 30 %); hooves 1:100 (0.079 vs 0.016 m2, 393 % > 30 %); +2 more
- **furniture/single-bed**: slats 1:50 (box); slats 1:100 (box); slats 1:50 (4.11 vs 1.5 m2, 174 % > 15 %); slats 1:100 (3.64 vs 1.5 m2, 143 % > 30 %); all regions 1:200 (10.3 vs 7.37 m2, 40 % > 30 %); all regions 1:500 (10.7 vs 7.37 m2, 45 % > 30 %)
- **humans/skateboarder**: hoodie-torso 1:50 (0.514 vs 0.809 m2, 36 % > 30 %); wheels 1:50 (0.04 vs 0.0337 m2, 19 % > 15 %); hoodie-torso 1:100 (0.509 vs 0.809 m2, 37 % > 30 %)
- **machinery/solar-thermal-collector**: housing 1:50 (5.14 vs 2.6 m2, 98 % > 15 %); absorber 1:50 (2.37 vs 1.8 m2, 32 % > 15 %); housing 1:100 (5.33 vs 2.6 m2, 105 % > 30 %)
- **machinery/sorption-wheel**: casing 1:50 (0.885 vs 0.68 m2, 30 % > 15 %); regen 1:50 (0.143 vs 0.12 m2, 20 % > 15 %); casing 1:100 (0.99 vs 0.68 m2, 46 % > 30 %); regen 1:100 (0.179 vs 0.12 m2, 49 % > 30 %); all regions 1:200 (1.16 vs 0.808 m2, 43 % > 30 %)
- **objects/stepladder**: feet 1:50 (0.04 vs 0.03 m2, 34 % > 15 %); feet 1:100 (0.047 vs 0.03 m2, 56 % > 30 %); all regions 1:500 (3.18 vs 1.73 m2, 84 % > 30 %)
- **objects/street-lamp**: plinth-concrete 1:50 (0.12 vs 0.5 m2, 76 % > 15 %); plinth-concrete 1:100 (0.144 vs 0.5 m2, 71 % > 30 %)
- **objects/tealight**: flame 1:50 (0 vs 0.0006 m2, 40 % > 15 %)
- **furniture/three-seat-sofa**: arm-pads 1:50 (2.02 vs 2.69 m2, 25 % > 15 %); arm-pads 1:100 (1.77 vs 2.69 m2, 34 % > 30 %)
- **components/tied-canopy**: wall-plate 1:50 (0.093 vs 0.48 m2, 81 % > 15 %); wall-plate 1:100 (0.048 vs 0.48 m2, 90 % > 30 %)
- **components/tilt-window**: frame-timber 1:50 (2.52 vs 0.238 m2, 958 % > 15 %); frame-sill 1:50 (0.39 vs 0.09 m2, 333 % > 15 %); frame-timber 1:100 (2.51 vs 0.238 m2, 956 % > 30 %); frame-sill 1:100 (0.39 vs 0.09 m2, 333 % > 30 %); all regions 1:200 (3.67 vs 1.17 m2, 215 % > 30 %)
- **appliances/tower-server**: side-panel 1:50 (0.271 vs 0.38 m2, 29 % > 15 %); feet 1:100 (0.003 vs 0.002 m2, 52 % > 30 %)
- **appliances/vacuum-cleaner**: body-shell 1:50 (0.309 vs 0.42 m2, 26 % > 15 %); wheels 1:50 (0.016 vs 0.05 m2, 67 % > 15 %); hose 1:50 (0.119 vs 0.14 m2, 15 % > 15 %); wand 1:50 (0.066 vs 0.09 m2, 27 % > 15 %); brush 1:50 (0.002 vs 0.01 m2, 85 % > 15 %); wheels 1:100 (0.015 vs 0.05 m2, 69 % > 30 %); brush 1:100 (0.002 vs 0.01 m2, 85 % > 30 %); all regions 1:200 (0.533 vs 0.99 m2, 46 % > 30 %); +1 more
- **vehicles/vintage-car-1929**: hood 1:50 (2.56 vs 1.5 m2, 70 % > 15 %); fenders 1:50 (5.23 vs 2.6 m2, 101 % > 15 %); interior 1:50 (6.58 vs 3.5 m2, 88 % > 15 %); exhaust-system 1:50 (0.955 vs 0.4 m2, 139 % > 15 %); hood 1:100 (2.52 vs 1.5 m2, 68 % > 30 %); fenders 1:100 (5.06 vs 2.6 m2, 94 % > 30 %); interior 1:100 (6.82 vs 3.5 m2, 95 % > 30 %); exhaust-system 1:100 (0.955 vs 0.4 m2, 139 % > 30 %)

## Geometry

Assets that fail any other rule (the authors' worklist), 50, worst first (worst ratio of limit over value among the non-thermal findings; rules with the number of findings).

- **vegetation/english-oak** worst x169.00: bones 5, bounds 1, cross-scale 8, monotonic 1, silhouette 9, structure-coverage 10, thermal-coverage 2
- **vegetation/apple-tree** worst x74.33: bones 14, bounds 2, cross-scale 5, silhouette 10, structure-coverage 7, thermal-coverage 5
- **vegetation/agave** worst x48.49: bounds 1, cross-scale 3, feature 8, monotonic 1, silhouette 3, structure-coverage 7, thermal-coverage 3
- **vegetation/hosta-clump** worst x25.22: cross-scale 1, feature 9, monotonic 2, silhouette 5, solids 9, structure-coverage 5, thermal-coverage 2
- **vegetation/bamboo-clump** worst x16.00: bones 2, bounds 1, cross-scale 4, feature 6, monotonic 3, silhouette 11, structure-coverage 5, thermal-coverage 3
- **vegetation/date-palm** worst x12.06: copied 1, cross-scale 4, monotonic 1, silhouette 6, structure-coverage 5, thermal-coverage 4, views-disagree 1
- **humans/person-cooking** worst x11.76: bounds 2, silhouette 2, structure-coverage 4, thermal-coverage 11
- **components/spiral-stair** worst x7.88: cross-scale 7, silhouette 12
- **humans/parent-with-stroller** worst x4.50: cross-scale 2, silhouette 5, structure-coverage 8, thermal-coverage 4
- **vegetation/ivy-on-trellis** worst x3.45: cross-scale 1, silhouette 4, views-disagree 3
- **animals/dairy-cow** worst x2.67: silhouette 1, structure-coverage 5, thermal-coverage 5, views-disagree 1
- **humans/parent-carrying-toddler** worst x2.56: silhouette 4, thermal-coverage 16
- **humans/person-with-rollator** worst x2.50: silhouette 9, structure-coverage 5, thermal-coverage 8
- **humans/sleeping-person** worst x2.30: feature 3, silhouette 3, structure-coverage 2, thermal-coverage 6
- **machinery/diesel-generator** worst x2.25: silhouette 1, thermal-coverage 3
- **humans/person-with-white-cane** worst x2.00: silhouette 6, structure-coverage 3, thermal-coverage 9
- **animals/herring-gull** worst x2.00: monotonic 2, silhouette 8, structure-coverage 1, thermal-coverage 7, views-disagree 1
- **objects/shopping-trolley** worst x1.90: structure-coverage 4, thermal-coverage 1
- **furniture/office-swivel-chair** worst x1.88: feature 4
- **animals/red-squirrel** worst x1.73: structure-coverage 1, thermal-coverage 4
- **components/cantilever-balcony** worst x1.67: thermal-coverage 9, views-disagree 1
- **objects/retractable-bollard** worst x1.63: silhouette 1
- **humans/seated-reader** worst x1.63: cross-scale 1, silhouette 1, thermal-coverage 3
- **machinery/tower-crane** worst x1.63: cross-scale 4, silhouette 1, thermal-coverage 12
- **humans/seated-guitarist** worst x1.62: thermal-coverage 14, views-disagree 2
- **furniture/height-adjustable-desk** worst x1.52: monotonic 1, silhouette 3, thermal-coverage 6, views-disagree 4
- **vegetation/hornbeam-hedge** worst x1.50: cross-scale 1, silhouette 1, thermal-coverage 4
- **animals/horse** worst x1.50: cross-scale 1, silhouette 3, structure-coverage 1, thermal-coverage 6
- **humans/walking-commuter** worst x1.50: bounds 2, monotonic 1, silhouette 1, structure-coverage 2, thermal-coverage 7
- **machinery/plate-heat-exchanger** worst x1.48: structure-coverage 2, thermal-coverage 2
- **appliances/toaster** worst x1.47: feature 2, structure-coverage 2, thermal-coverage 8
- **humans/standing-phone-user** worst x1.44: silhouette 2, structure-coverage 1, thermal-coverage 3
- **humans/tour-guide** worst x1.43: silhouette 4, thermal-coverage 3, views-disagree 1
- **humans/worker-carrying-box** worst x1.33: thermal-coverage 9, views-disagree 4
- **machinery/expansion-valve** worst x1.25: cross-scale 1
- **appliances/laptop** worst x1.23: silhouette 2, thermal-coverage 2
- **appliances/shower-head** worst x1.21: silhouette 4, structure-coverage 1, thermal-coverage 10
- **components/wheelchair-ramp** worst x1.20: cross-scale 1
- **components/sliding-door** worst x1.20: views-disagree 1
- **components/sectional-garage-door** worst x1.17: silhouette 1, thermal-coverage 1
- **appliances/pendant-lamp** worst x1.15: silhouette 2, thermal-coverage 1
- **animals/mallard-duck** worst x1.13: thermal-coverage 7, views-disagree 1
- **humans/person-on-crutches** worst x1.13: bounds 1, silhouette 3, thermal-coverage 9
- **machinery/excavator** worst x1.13: cross-scale 1, thermal-coverage 10
- **humans/wheelchair-user** worst x1.13: cross-scale 2, thermal-coverage 4
- **objects/chalkboard** worst x1.10: views-disagree 1
- **appliances/gas-meter** worst x1.10: thermal-coverage 1, views-disagree 1
- **objects/playground-swing** worst x1.08: silhouette 2, thermal-coverage 6
- **animals/labrador-dog** worst x1.05: silhouette 3, thermal-coverage 5
- **components/curtain-wall-bay** worst x0.00: monotonic 1, thermal-coverage 10

<!-- lists:end -->
