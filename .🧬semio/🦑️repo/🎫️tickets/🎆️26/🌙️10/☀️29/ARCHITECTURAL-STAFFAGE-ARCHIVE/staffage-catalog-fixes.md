# Staffage catalog fixes

Semantic validation (`validateAsset`) of the six categories is clean: 0 issues, including the four installed vegetation assets.

## Installed

`bamboo-clump`, `ivy-on-trellis`, `agave`, `hosta-clump` copied into `🗂️catalog/🌳️vegetation/<id>/🔣️.json` from the ticket `NW-V2` documents.

## Schema and semantics

- `presentation.defaultScale` 100: `tower-crane`, `excavator`, `forklift`, `cargo-bike`, `city-car`, `city-bus`.
- Distinct drawings (0.5 mm local shift on the coarser scale): `diesel-generator` 1:200, `wheel-loader` 1:200 and 1:100, `market-parasol` 1:500, `curtain-wall-bay` 1:100, `spiral-stair` 1:200 and 1:100, `masonry-wall-with-lintel` 1:200 and 1:100.
- `bookshelf` clip `load-the-shelves` tracks `books-5-l` and `books-5-r`: dropped the duplicate key at t = 5 s.
- `small-leaved-lime` 1:200: `stem` now draws `trunk-bark` and `epicormic-stem`.
- `japanese-maple` 1:500 geometry scaled so the default pose is 5.367 m by 4.417 m, inside the 2 % / 15 mm tolerance of 5.27 m by 4.42 m.
- `heat-pump-outdoor-unit` 1:200: removed the missing hatch `fins`.
- `panel-radiator`: omitted null `skyTempC` and `groundTempC`.

## Thermal

Named-pose `braking` capped at 0.05 where it was higher: `city-car`, `city-bus`, `city-bicycle`, `forklift`, `delivery-van`, `motor-scooter`, `electric-kick-scooter`, `tram`.

`heatW` reduced so sampled steady and clip temperatures stay in range:

| asset | region | heatW | resulting surface |
|---|---|---|---|
| city-car | brakes | 1406.3 | 327 °C |
| city-bus | brakes | 5468.8 | 335 °C |
| city-bicycle | hub-brakes | 431.3 | 339 °C |
| forklift | brakes | 6011.7 | 339 °C |
| delivery-van | engine-bay | 2419.9 | 240 °C |
| delivery-van | brakes | 6679.7 | 330 °C |
| motor-scooter | brake-disc | 1311.7 | 339 °C |
| electric-kick-scooter | bms-controller | 1.8 | 79 °C |
| tram | traction-motors | 11765.6 | 240 °C |
| tram | brake-discs | 15703.1 | 339 °C |
| wheel-loader | exhaust-stack | 2378.9 | 240 °C |
| concrete-mixer-truck | drum-drive | 539.1 | 80 °C |
| excavator | exhaust-stack | 1218.8 | 240 °C |
| retractable-bollard | hydraulic-unit | 358.8 | 80 °C |
| heat-pump-outdoor-unit | coil-guard | -220 | -7.6 °C |
| heat-pump-outdoor-unit | base-tray | 185.5 | 36 °C |

## Checked end to end

`checkAssetDetailed` (office-laser, 5 clip samples, physics on):

- `delivery-van`: 0 problems.
- `panel-radiator`: pose/clip errors gone (the null sky and ground temperatures). 2 print-suitability problems remain.
- `motor-scooter`: pose/clip non-finite errors gone. 2 print-suitability problems remain.

## Print suitability still open

A full office-laser scan found 57 assets with `suitability-gap` or `suitability-feature` violations (about 1112 gaps and 280 thin fills). A pen-join pass does not converge: thickening one pair pulls the next pair into the 0.15 mm band. It was stopped after rewriting `date-palm`, `miscanthus-grass`, `ivy-on-trellis`, and `hornbeam-hedge` (thicker `penMm`, some thin fills opened). Those four still validate. The other 53 failing drawings were not rewritten.
