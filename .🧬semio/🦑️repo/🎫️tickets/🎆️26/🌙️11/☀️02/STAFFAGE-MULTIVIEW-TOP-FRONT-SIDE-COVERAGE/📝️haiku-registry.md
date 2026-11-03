# 📝️ Haiku Registry Inspection (slot 19, haiku-registry)

Status: preparation only, read-only. No catalog, schema or cast manifest file was edited. Manifest row edits wait for the coordinator's instruction and the agreed schema from sonnet-contracts.

Scripts (temporary, in this ticket): `🛠️scripts/haiku-registry/inspect.mjs` (rows, folders, collections, casts), `counts.mjs` (tallies), `cross2.mjs` (manifest against asset files). `cross.mjs` is a first attempt that used the wrong asset path and is superseded by `cross2.mjs`. The `tee` redirect into `🗑️generated` failed on the emoji path, so no generated output file exists; all results are below.

## 1. Counts

| Check | Result |
|---|---|
| Manifest rows (`🗂️catalog/🔣️.json` `entries`) | 149 |
| Categories | 9 |
| Collections | 5 |
| Rows per category | humans 28, animals 14, vegetation 16, furniture 12, objects 15, vehicles 9, machinery 16, components 15, appliances 24 (sum 149) |
| Rows by `completion` | authored 149, verified 0, planned 0 |
| Duplicate row ids | 0 |
| Rows whose folder is missing or has no single `🔣️.json` | 0 |
| Category folders with a folder but no manifest row (orphans) | 0 |
| Collection assets that do not resolve to a row | 0 |
| Top-level non-category folders | `🌤️environments`, `🖨️print-profiles` (not assets; expected) |
| Rows with an extra key outside the schema | 0 |

Every row has a folder and every folder has a row: the registry is consistent on both sides.

## 2. Manifest row definition (schema `🧬️schema/🔣️.json` `$defs/ManifestEntry`)

- `additionalProperties: false`. Required: `id`, `category`, `title`, `heightM`, `widthM`, `poses`, `clips`, `representations`, `symbols`, `analytics`, `exports`, `distinction`, `completion`.
- **No per-row view field exists.** The only view or format fields are:
  - `representations`: object with exactly the keys `50`, `100`, `200`, `500`, each `authored` or `unavailable`. Has no reason field.
  - `analytics`: subset of `thermal`, `transport`, `structural`.
  - `exports`: subset of `svg`, `pdf`, `dxf`, `dwg`, `png-sequence`, `mp4`, `webm`, `component`.
  - `completion`: `planned`, `authored` or `verified`.
- The 1:50 and 1:100 bands are authored on all 149 rows. Exports are the full 8 formats on all 149 rows.

Tallies from `counts.mjs`:

| Field | Value | Rows |
|---|---|---|
| `representations` status | unavailable at `500` | 54 |
| | unavailable at `200` | 8 (all 8 also unavailable at 500) |
| | any unavailable | 54 |
| `analytics` | thermal+transport+structural | 105 |
| | thermal+structural (no transport) | 44 |
| `exports` | all 8 formats | 149 |
| `symbols` count | 0 / 1 / 2 | 103 / 39 / 7 |
| `poses` count | 3 / 4 / 5 / 6 / 7 | 80 / 47 / 18 / 3 / 1 |
| `clips` count | 2 / 3 / 4 | 97 / 50 / 2 |

## 3. Cross-check against the asset files (`cross2.mjs`)

- 149 of 149 asset files were read (`asset.representations`).
- Manifest status and asset status agree for every row and every scale: 0 mismatches.
- Every `unavailable` band in an asset file has a `reason`: 0 without one. The reason is kept in the asset file, not in the manifest row.
- Authored bands: 534 (149 × 4 minus 62 unavailable). Each has `elevation`, `front` and `plan` geometry keys: 534 of 534.

## 4. Manifest rows

Codes: `A` authored, `U` unavailable, in the order 1:50 / 1:100 / 1:200 / 1:500. Analytics `TTS` = thermal+transport+structural, `TS` = thermal+structural. Completion is `authored` on every row; exports are all 8 formats on every row (omitted from the table).

| # | Slug | Category | Completion | 50/100/200/500 | Analytics |
|---|---|---|---|---|---|
| 1 | walking-commuter | humans | authored | AAAA | TTS |
| 2 | seated-office-worker | humans | authored | AAAA | TTS |
| 3 | sleeping-person | humans | authored | AAAA | TTS |
| 4 | seated-reader | humans | authored | AAAA | TTS |
| 5 | running-child | humans | authored | AAAA | TTS |
| 6 | wheelchair-user | humans | authored | AAAA | TTS |
| 7 | elderly-with-cane | humans | authored | AAAA | TTS |
| 8 | cyclist | humans | authored | AAAA | TTS |
| 9 | worker-carrying-box | humans | authored | AAAA | TTS |
| 10 | parent-with-stroller | humans | authored | AAAA | TTS |
| 11 | jogger | humans | authored | AAAA | TTS |
| 12 | person-with-umbrella | humans | authored | AAAA | TTS |
| 13 | leaning-person | humans | authored | AAAA | TTS |
| 14 | tour-guide | humans | authored | AAAA | TTS |
| 15 | english-oak | vegetation | authored | AAAA | TTS |
| 16 | lombardy-poplar | vegetation | authored | AAAA | TTS |
| 17 | multi-stem-birch | vegetation | authored | AAAA | TTS |
| 18 | scots-pine | vegetation | authored | AAAA | TTS |
| 19 | date-palm | vegetation | authored | AAAA | TTS |
| 20 | weeping-willow | vegetation | authored | AAAA | TTS |
| 21 | hornbeam-hedge | vegetation | authored | AAAA | TTS |
| 22 | miscanthus-grass | vegetation | authored | AAAA | TTS |
| 23 | labrador-dog | animals | authored | AAAA | TTS |
| 24 | domestic-cat | animals | authored | AAAU | TTS |
| 25 | horse | animals | authored | AAAA | TTS |
| 26 | pigeon | animals | authored | AAAU | TTS |
| 27 | roe-deer | animals | authored | AAAA | TTS |
| 28 | dairy-cow | animals | authored | AAAA | TTS |
| 29 | park-bench | furniture | authored | AAAA | TS |
| 30 | cafe-chair | furniture | authored | AAAA | TS |
| 31 | office-swivel-chair | furniture | authored | AAAA | TS |
| 32 | height-adjustable-desk | furniture | authored | AAAA | TS |
| 33 | street-lamp | objects | authored | AAAA | TTS |
| 34 | market-parasol | objects | authored | AAAA | TS |
| 35 | waste-bin | objects | authored | AAAA | TS |
| 36 | playground-swing | objects | authored | AAAA | TS |
| 37 | city-car | vehicles | authored | AAAA | TTS |
| 38 | vintage-car-1929 | vehicles | authored | AAAA | TTS |
| 39 | city-bus | vehicles | authored | AAAA | TTS |
| 40 | excavator | machinery | authored | AAAA | TTS |
| 41 | tower-crane | machinery | authored | AAAA | TTS |
| 42 | forklift | machinery | authored | AAAA | TTS |
| 43 | cargo-bike | vehicles | authored | AAAA | TS |
| 44 | cantilever-balcony | components | authored | AAAA | TS |
| 45 | glulam-beam | components | authored | AAAA | TS |
| 46 | steel-column | components | authored | AAAA | TS |
| 47 | stair-flight | components | authored | AAAA | TS |
| 48 | sliding-door | components | authored | AAAA | TS |
| 49 | tilt-window | components | authored | AAAU | TS |
| 50 | glass-balustrade | components | authored | AAAU | TS |
| 51 | tied-canopy | components | authored | AAAA | TS |
| 52 | child-on-kick-scooter | humans | authored | AAAA | TTS |
| 53 | courier-with-hand-truck | humans | authored | AAAA | TTS |
| 54 | kneeling-gardener | humans | authored | AAUU | TTS |
| 55 | parent-carrying-toddler | humans | authored | AAUU | TTS |
| 56 | person-on-crutches | humans | authored | AAAA | TTS |
| 57 | person-with-rollator | humans | authored | AAAA | TTS |
| 58 | person-with-white-cane | humans | authored | AAAA | TTS |
| 59 | pregnant-person | humans | authored | AAAA | TTS |
| 60 | runner-with-blade-prosthesis | humans | authored | AAAA | TTS |
| 61 | seated-guitarist | humans | authored | AAAA | TTS |
| 62 | skateboarder | humans | authored | AAUU | TTS |
| 63 | person-cooking | humans | authored | AAAA | TTS |
| 64 | standing-adult | humans | authored | AAAA | TTS |
| 65 | standing-phone-user | humans | authored | AAAA | TTS |
| 66 | goat | animals | authored | AAAA | TTS |
| 67 | hedgehog | animals | authored | AAUU | TTS |
| 68 | herring-gull | animals | authored | AAAU | TTS |
| 69 | mallard-duck | animals | authored | AAAU | TTS |
| 70 | pig | animals | authored | AAAA | TTS |
| 71 | red-fox | animals | authored | AAAU | TTS |
| 72 | red-squirrel | animals | authored | AAUU | TTS |
| 73 | sheep | animals | authored | AAAA | TTS |
| 74 | agave | vegetation | authored | AAAA | TTS |
| 75 | apple-tree | vegetation | authored | AAAA | TTS |
| 76 | bamboo-clump | vegetation | authored | AAAA | TTS |
| 77 | hosta-clump | vegetation | authored | AAAU | TTS |
| 78 | ivy-on-trellis | vegetation | authored | AAAA | TTS |
| 79 | japanese-maple | vegetation | authored | AAAA | TTS |
| 80 | london-plane-pollarded | vegetation | authored | AAAA | TTS |
| 81 | small-leaved-lime | vegetation | authored | AAAA | TTS |
| 82 | bar-stool | furniture | authored | AAAA | TS |
| 83 | bookshelf | furniture | authored | AAAA | TS |
| 84 | dining-table | furniture | authored | AAAA | TS |
| 85 | folding-deck-chair | furniture | authored | AAAA | TS |
| 86 | picnic-table | furniture | authored | AAAA | TS |
| 87 | single-bed | furniture | authored | AAAA | TS |
| 88 | three-seat-sofa | furniture | authored | AAAA | TS |
| 89 | bicycle-stand | objects | authored | AAAA | TS |
| 90 | charcoal-grill | objects | authored | AAAA | TTS |
| 91 | drinking-fountain | objects | authored | AAAA | TTS |
| 92 | patio-heater | objects | authored | AAAA | TTS |
| 93 | laptop | appliances | authored | AAAU | TTS |
| 94 | desktop-monitor | appliances | authored | AAAU | TTS |
| 95 | tower-server | appliances | authored | AAAU | TTS |
| 96 | pendant-lamp | appliances | authored | AAAU | TTS |
| 97 | retractable-bollard | objects | authored | AAAA | TS |
| 98 | shopping-trolley | objects | authored | AAAA | TS |
| 99 | stepladder | objects | authored | AAAA | TS |
| 100 | city-bicycle | vehicles | authored | AAAU | TS |
| 101 | delivery-van | vehicles | authored | AAAA | TTS |
| 102 | electric-kick-scooter | vehicles | authored | AAAU | TS |
| 103 | motor-scooter | vehicles | authored | AAAA | TTS |
| 104 | tram | vehicles | authored | AAAA | TTS |
| 105 | concrete-mixer-truck | machinery | authored | AAAA | TTS |
| 106 | diesel-generator | machinery | authored | AAAA | TTS |
| 107 | heat-pump-outdoor-unit | machinery | authored | AAAA | TTS |
| 108 | scissor-lift | machinery | authored | AAAU | TTS |
| 109 | wheel-loader | machinery | authored | AAAA | TTS |
| 110 | curtain-wall-bay | components | authored | AAAA | TS |
| 111 | external-venetian-blind | components | authored | AAAU | TS |
| 112 | masonry-wall-with-lintel | components | authored | AAAA | TS |
| 113 | panel-radiator | components | authored | AAAU | TTS |
| 114 | sectional-garage-door | components | authored | AAAA | TS |
| 115 | spiral-stair | components | authored | AAAA | TS |
| 116 | wheelchair-ramp | components | authored | AAAA | TS |
| 117 | split-ac-indoor-unit | appliances | authored | AAAU | TTS |
| 118 | toaster | appliances | authored | AAAU | TTS |
| 119 | vacuum-cleaner | appliances | authored | AAAU | TTS |
| 120 | electric-fan-heater | appliances | authored | AAAU | TTS |
| 121 | operating-table | furniture | authored | AAAU | TTS |
| 122 | chalkboard | objects | authored | AAAU | TS |
| 123 | kitchen-extractor-hood | appliances | authored | AAAU | TTS |
| 124 | mvhr-unit | appliances | authored | AAAU | TTS |
| 125 | duct-fan | appliances | authored | AAAU | TTS |
| 126 | decentralised-hrv-fan | appliances | authored | AAAU | TTS |
| 127 | oil-pumpjack | machinery | authored | AAAA | TS |
| 128 | room-thermostat | appliances | authored | AAUU | TS |
| 129 | thermostatic-radiator-valve | appliances | authored | AAUU | TS |
| 130 | tealight | objects | authored | AAAU | TTS |
| 131 | shower-head | appliances | authored | AAAU | TTS |
| 132 | gas-hob | appliances | authored | AAAU | TTS |
| 133 | cooking-pot | objects | authored | AAAU | TTS |
| 134 | electric-kettle | appliances | authored | AAAU | TTS |
| 135 | scroll-compressor | machinery | authored | AAAU | TTS |
| 136 | expansion-valve | machinery | authored | AAUU | TS |
| 137 | finned-heat-exchanger | machinery | authored | AAAU | TS |
| 138 | sorption-wheel | machinery | authored | AAAU | TTS |
| 139 | plate-heat-exchanger | machinery | authored | AAAU | TTS |
| 140 | solar-thermal-collector | machinery | authored | AAAU | TTS |
| 141 | evaporative-humidifier | machinery | authored | AAAU | TTS |
| 142 | gas-meter | appliances | authored | AAAU | TS |
| 143 | electricity-meter | appliances | authored | AAAU | TS |
| 144 | ceiling-diffuser | appliances | authored | AAAU | TTS |
| 145 | storage-crate-stack | objects | authored | AAAU | TS |
| 146 | server-rack | appliances | authored | AAAU | TTS |
| 147 | circulation-pump | appliances | authored | AAAU | TTS |
| 148 | gas-boiler | appliances | authored | AAAU | TTS |
| 149 | hot-water-cylinder | appliances | authored | AAAU | TTS |

Rows with `U` at 1:200: kneeling-gardener, parent-carrying-toddler, skateboarder, hedgehog, red-squirrel, room-thermostat, thermostatic-radiator-valve, expansion-valve (8). These have `U` at 1:500 too.

## 5. Cast manifest (`🧍️staffage/🔣️.json`, under `⚡️energy/🏗️authoring/🎨️visuals`)

- Casts: 94. Viewpoint: absent 88 (default elevation), `front` 4, `plan` 2. No cast sets `viewpoint: elevation` explicitly.
- `view`: thermal 82, dark-line 12.
- Every cast names exactly one of `pose` or `clip`. Every cast asset is a row of the catalog manifest (0 unknown).
- Explicit `front` casts (not in the list below): `shared-radiator-front-winter` (panel-radiator, pose rated, thermal), `optimization-extrema-cooler` (split-ac-indoor-unit, pose cooling, thermal), `optimization-experiments-window` (tilt-window, pose tilted-open, thermal), `shared-radiator-thermostat-front` (panel-radiator, clip thermostat-cycle, thermal).
- Explicit `plan` casts: `excel-program-occupant` (standing-adult, pose relaxed, dark-line), `bim-information-car` (vintage-car-1929, pose parked, dark-line).

### 5.1 Casts without an explicit `viewpoint` (default elevation): 88

| # | Cast id | Asset | Pose or clip | View |
|---|---|---|---|---|
| 1 | cooling-internal-gains-sleeper | sleeping-person | clip breathing | thermal |
| 2 | cooling-internal-gains-seated-worker | seated-office-worker | clip typing | thermal |
| 3 | cooling-internal-gains-standing | standing-adult | clip idle-breathing | thermal |
| 4 | cooling-internal-gains-walker | walking-commuter | clip walk-cycle | thermal |
| 5 | cooling-internal-gains-jogger | jogger | clip run-cycle | thermal |
| 6 | cooling-internal-gains-laptop | laptop | pose open | thermal |
| 7 | cooling-internal-gains-laptop-closed | laptop | pose closed | thermal |
| 8 | cooling-internal-gains-server-rack | server-rack | pose full-load | thermal |
| 9 | shared-lamp-summer | pendant-lamp | pose on | thermal |
| 10 | cooling-internal-gains-lamp-dimmed | pendant-lamp | pose dimmed | thermal |
| 11 | shared-desk | height-adjustable-desk | pose seated-height | dark-line |
| 12 | cooling-internal-gains-office-chair | office-swivel-chair | pose seat-low | dark-line |
| 13 | shared-person-winter | standing-adult | clip idle-breathing | thermal |
| 14 | heating-internal-gains-office-worker | seated-office-worker | clip typing | thermal |
| 15 | shared-laptop-winter | laptop | pose open | thermal |
| 16 | heating-internal-gains-tower-server | tower-server | clip power-up | thermal |
| 17 | heating-internal-gains-pendant-lamp-warm-up | pendant-lamp | clip switch-on-warm-up | thermal |
| 18 | shared-lamp-winter | pendant-lamp | pose on | thermal |
| 19 | shared-person-relaxed-line | standing-adult | pose relaxed | dark-line |
| 20 | bim-consistency-decider | standing-adult | pose arms-crossed | dark-line |
| 21 | optimization-room-cooler | split-ac-indoor-unit | pose cooling | thermal |
| 22 | shared-lamp-neutral | pendant-lamp | pose on | thermal |
| 23 | shared-lamp-off-neutral | pendant-lamp | pose off | thermal |
| 24 | optimization-room-occupant | standing-adult | pose relaxed | thermal |
| 25 | optimization-experiments-occupant | standing-adult | pose relaxed | thermal |
| 26 | shared-person-pathlines | standing-adult | clip idle-breathing | thermal |
| 27 | ventilation-natural-radiator-on | panel-radiator | pose night | thermal (scale 20) |
| 28 | ventilation-natural-radiator-off | panel-radiator | pose off | thermal (scale 20) |
| 29 | ventilation-heat-flow-heater | electric-fan-heater | pose heating | thermal |
| 30 | shared-office-worker-pathlines | seated-office-worker | clip typing | thermal |
| 31 | ventilation-volume-cook | person-cooking | clip stir-and-taste | thermal |
| 32 | ventilation-volume-operating-table | operating-table | pose light-on | thermal |
| 33 | ventilation-volume-hood | kitchen-extractor-hood | pose extracting | thermal |
| 34 | ventilation-volume-blackboard | chalkboard | pose leaf-left | dark-line |
| 35 | ventilation-volume-crates-low | storage-crate-stack | pose low | dark-line |
| 36 | ventilation-volume-crates-half | storage-crate-stack | pose half | dark-line |
| 37 | ventilation-volume-crates-high | storage-crate-stack | pose high | dark-line |
| 38 | demand-certificate-sleeper | sleeping-person | clip breathing | thermal |
| 39 | physics-fundamentals-vacuum | vacuum-cleaner | clip run-and-park | thermal |
| 40 | shared-radiator-winter | panel-radiator | pose rated | thermal |
| 41 | demand-primary-energy-tree | english-oak | pose calm | dark-line |
| 42 | shared-person-neutral | standing-adult | clip idle-breathing | thermal |
| 43 | shared-lamp-off-winter | pendant-lamp | pose off | thermal |
| 44 | demand-certificate-buyer | standing-adult | pose arms-crossed | thermal |
| 45 | physics-fundamentals-clerk | standing-adult | pose pointing | thermal |
| 46 | shared-blind-lowered | external-venetian-blind | pose lowered-cut-off-45 | thermal |
| 47 | physics-fundamentals-hob | gas-hob | clip flicker | thermal |
| 48 | physics-fundamentals-pot | cooking-pot | clip heat-to-boil | thermal |
| 49 | physics-fundamentals-kettle | electric-kettle | clip boil | thermal |
| 50 | physics-fundamentals-compressor | scroll-compressor | clip spin | thermal |
| 51 | physics-fundamentals-evaporator | finned-heat-exchanger | pose cold | thermal |
| 52 | physics-fundamentals-condenser | finned-heat-exchanger | pose hot | thermal |
| 53 | physics-fundamentals-valve | expansion-valve | pose open | thermal |
| 54 | cooling-summer-strategy-wheel | sorption-wheel | clip spin | thermal |
| 55 | cooling-summer-strategy-exchanger | plate-heat-exchanger | pose full | thermal |
| 56 | cooling-summer-strategy-humidifier | evaporative-humidifier | clip drip | thermal |
| 57 | cooling-summer-strategy-collector | solar-thermal-collector | clip start-up | thermal |
| 58 | cooling-summer-strategy-heater | finned-heat-exchanger | clip warm-up | thermal |
| 59 | shared-person-fanning | standing-adult | clip fanning | thermal |
| 60 | cooling-heating-comparison-table | dining-table | pose closed | dark-line |
| 61 | shared-toaster-winter | toaster | clip toast-cycle | thermal |
| 62 | cooling-heating-comparison-toaster-summer | toaster | clip toast-cycle | thermal |
| 63 | cooling-heating-comparison-laptop | laptop | clip work-and-sleep | thermal |
| 64 | cooling-heating-comparison-laptop-summer | laptop | clip work-and-sleep | thermal |
| 65 | cooling-summer-strategy-blind | external-venetian-blind | clip sun-and-shade | thermal |
| 66 | shared-watt-laptop | laptop | clip work-and-sleep | thermal |
| 67 | shared-watt-toaster | toaster | clip toast-cycle | thermal |
| 68 | shared-watt-seated | seated-office-worker | clip typing | thermal |
| 69 | ventilation-mechanical-duct-fan | duct-fan | clip spin | thermal |
| 70 | ventilation-mechanical-mvhr | mvhr-unit | clip boost-spin | thermal |
| 71 | ventilation-mechanical-pendulum-fan | decentralised-hrv-fan | clip pendulum-cycle | thermal |
| 72 | shared-mvhr-normal | mvhr-unit | pose normal | thermal |
| 73 | heating-conduction-tealight | tealight | clip flicker | thermal |
| 74 | physics-fundamentals-shower | shower-head | clip turn-on-and-run | thermal |
| 75 | shared-boiler-heating-cycle | gas-boiler | clip heating-cycle | thermal |
| 76 | shared-tank-charge-up | hot-water-cylinder | clip charge-up | thermal |
| 77 | shared-pump-run-high | circulation-pump | clip run-high | thermal |
| 78 | demand-practice-valve | thermostatic-radiator-valve | clip close-down | thermal |
| 79 | shared-gas-meter-run | gas-meter | clip run | thermal |
| 80 | shared-electricity-meter-run | electricity-meter | clip run | thermal |
| 81 | cooling-system-sizing-diffuser | ceiling-diffuser | pose design | thermal |
| 82 | demand-primary-energy-pumpjack | oil-pumpjack | clip pump-cycle | thermal (scale 100) |
| 83 | heating-calculation-thermostat | room-thermostat | clip turn-up | thermal |
| 84 | shared-heat-pump-heating-sweep | heat-pump-outdoor-unit | clip heating-sweep | thermal |
| 85 | cooling-heating-comparison-cooler | split-ac-indoor-unit | clip start-cooling | thermal |
| 86 | shared-watt-heater | electric-fan-heater | clip warm-up | thermal |
| 87 | shared-radiator-thermostat | panel-radiator | clip thermostat-cycle | thermal |
| 88 | heating-calculation-laptop | laptop | pose open-idle | thermal |

Split of the 88: 78 thermal, 10 dark-line. All 88 use `theme: dark`.

## 6. Observations for the coordinator (no action taken)

1. **Unavailable scale reason sits in the asset file, not the manifest row.** The schema's `ManifestEntry.representations` only allows `authored` or `unavailable`. All 62 unavailable bands carry a `reason` in their asset file, so the honest-printability rule is met by the asset data. Whether the manifest should also carry the reason is a schema decision for sonnet-contracts.
2. **Cast scales outside the authored bands.** `ventilation-natural-radiator-on` and `-off` use `scale: 20` (1:20). The authored bands start at 1:50, so 1:20 is finer than any authored band. The 1:20 renders are not covered by a representation band. Flagged, not changed. `demand-primary-energy-pumpjack` uses `scale: 100`, which is authored.
3. **Unavailable scales vs the brief's "every scale band".** The brief asks that every scale band of every asset is valid. 54 rows are `unavailable` at 1:500 and 8 at 1:200 (listed in section 4). Those bands need the explicit named-failure path, not silent substitution.
4. **Completion is `authored` for all 149.** No row is `verified`. This matches the brief's "verify, do not assume" direction for the later validation pass.
5. **No manifest edits were made.** Per brief, row edits wait for the coordinator's instruction and the agreed schema.

## Task 3: Pose and clip id audit (`🛠️scripts/haiku-registry/ids.ts`)

Schema (`🧬️schema/🟦️.ts`): `Asset.poses: Pose[]` with `id` and optional `key` (in-between key poses of a clip, not named poses; excluded from the named-pose comparison and counted separately). `Asset.clips: Clip[]` with `id`, `durationS`, `fps`, `loop`. `ManifestEntry.poses: Slug[]` and `clips: Slug[]`.

Output: `🗑️generated/haiku-registry/ids.json` (per asset: named and key pose counts, clip count, each clip's `durationS` and `loop`, the four mismatch lists, duplicates, problems). The manifest was not edited.

| Measure | Value |
|---|---|
| Assets checked | 149 |
| Assets with any id mismatch or duplicate | 3 |
| Named-pose mismatches | 3 assets |
| Clip mismatches | 0 assets |
| Duplicate ids (poses or clips, file or manifest) | 0 assets |
| Asset folders or files missing | 0 |
| Total named poses (file) | 546 |
| Total clips (file) | 352 |

Mismatches (every other asset matches its manifest row exactly):

| Asset | Named poses in file | Pose in file, not in manifest | Pose in manifest, not in file |
|---|---|---|---|
| office-swivel-chair | 4 | `swivel-quarter` | none |
| cargo-bike | 4 | `turning-left` | none |
| motor-scooter | 5 | `turning-left` | none |

Assets with key poses (in-between, not named) are common: for example child-on-kick-scooter has 44 key poses and runner-with-blade-prosthesis 21. These are not manifest ids and are not reported as mismatches.

### Manifest edits applied (approved rounds)

- Named poses added to the manifest: `swivel-quarter` (office-swivel-chair); `turning-left` (cargo-bike, motor-scooter). The geometry failures on these three assets are not registry issues (D33).
- 1:500 flipped to `authored` after verifying each asset file has an authored band with elevation, front and plan: solar-thermal-collector, external-venetian-blind, hot-water-cylinder, server-rack, vacuum-cleaner.
- **Flipped, pending redraw (`representation-copied`):** chalkboard, storage-crate-stack, operating-table. Kept `authored` per D33; objects is redrawing them, and the validator keeps them FAIL until then.
- Not flipped, per D27: city-bicycle and scissor-lift stay `unavailable` with their symbol alternatives.
- steel-column `depthM` 0.26: not mirrored. The manifest row (`ManifestEntry`, `additionalProperties: false`) carries only `heightM` and `widthM`, so there is nothing to mirror.
- hedgehog and red-squirrel: not changed. Their `alternatives` are the 1:100 scale fallbacks on the unavailable 1:200 and 1:500 bands, and `UnavailableRepresentation` requires `alternatives` (`minItems: 1`). Removing them would fail the schema. Needs animals' decision (for example, a symbol alternative instead).

### Id audit after the edits (`ids-final.json`)

- Named-pose and clip mismatches: 1 asset. **curtain-wall-bay** has pose `wind-suction` and clip `wind-suction-gust` in its asset file that the manifest row does not list. Probably a components edit after the manifest was last synced; not edited, pending the coordinator's decision.
- Duplicates: 0. Totals: 547 named poses, 353 clips.

### Approved follow-up (curtain-wall-bay and hedgehog, red-squirrel)

- curtain-wall-bay: pose `wind-suction` and clip `wind-suction-gust` added to the manifest row (approved). Rerun of `ids.ts` (`ids-final2.json`): 0 mismatches, 0 duplicates, 547 named poses, 353 clips.
- hedgehog and red-squirrel: left as they are. Their `alternatives` are scale fallbacks to the drawable 1:100 band, which D9 permits (D9 forbids only asset alternatives).

### Cast cache prune list (`🛠️scripts/registry/prune-list.json`, generator `🛠️scripts/registry/prune.py`)

Read-only. The course performs the prune. The generator is Python, run with the staffage venv: it imports the staffage module's own `cast_key`, so the keys are exact. A TypeScript port would have had to reimplement the Python JSON hashing.

| Reason | Entries |
|---|---|
| `key-not-current` (folder key matches no current manifest key) | 1,626 cast folders |
| `index-without-viewpoint` (pre-migration index) | 1,427 |
| `cast-not-in-manifest` (cast id no longer in the manifest) | 490 |
| `empty-key` (key folder with no cast) | 45 |
| `tmp-folder` (stale `.tmp-*`) | 4 |

- Key folders: 1,576 (1,531 with at least one listed cast; 45 empty). Current keys: 94, one per manifest cast.
- Frames in the listed cast folders: 298,618.
- Unkeyable manifest casts: 0. The 88 casts without `viewpoint` that the earlier inventory reported are no longer in the manifest without a viewpoint (the course has since added them), so that blocker is gone.
- Every cache folder is `key-not-current`: the exporter and catalog sources changed, so the source digest moved and no existing key matches. Every existing cast folder must be regenerated.

### Course cast manifest check (`🛠️scripts/registry/casts.py`, output `🗑️generated/haiku-registry/casts.json`)

Read-only. Checks each of the 94 casts for: asset exists; pose (named) or clip exists; scale band authored; viewpoint valid; `mirror` only with elevation; and a D19 `mirrored_assets` reason for mirrored casts (`cast_rules` in `🧪️tests/🔣️staffage_only.json`).

- Result: **94 casts, 0 failing.** Every failure counter is 0; no failing ids.
- The course has changed the manifest since the earlier inventory: the scale-20 casts are gone (scales now 50 for 93 casts and 100 for 1), and the mirrored casts are now named `-mirrored` (5 casts: `optimization-room-cooler-mirrored`, `ventilation-natural-radiator-on-mirrored`, `ventilation-natural-radiator-off-mirrored`, `ventilation-volume-hood-mirrored`, `shared-blind-lowered-mirrored`). All five use elevation.
- The four D19 `mirrored_assets` (split-ac-indoor-unit, panel-radiator, kitchen-extractor-hood, external-venetian-blind) cover every mirrored asset, and each entry is used, as the guard test requires.
- Limitation: the asymmetry test (non-zero bone or anchor `z`, one-sided extruded solids) is implemented, but it only matters for assets without an allowlist reason, and every mirrored asset has one, so it did not change the result.

### D36 and D39 applied (this pass)

- D36 closed: operating-table `widthM` 2 → 2.45; skateboarder `widthM` 1.218 → 1.084 and 1:200, 1:500 flipped to `authored` (asset file verified: both bands authored with elevation, front and plan); date-palm `widthM` 6.87 → 6.3 (vegetation confirmation).
- D39 applied in one pass: 32 symbol ids renamed in the manifest `symbols` rows and in each asset file (definitions and `alternatives` `symbol` references: hosta-clump, glass-balustrade, tilt-window). Table: `🛠️scripts/registry/symbol-rename.json`. Machines' nine renames were already in place. Humans `plan-dot` shapes left alone.
- Check after the pass: no old symbol id remains in the catalog except the humans `plan-dot` shapes. `validateManifest`: 1 issue left, `parent-carrying-toddler` representations disagree between manifest and asset (not touched; owner to confirm). `ids.ts`: 0 mismatches, 547 named poses, 353 clips.
- Contracts `failures.ts` rerun after the renames: `failures-d39.json` (7 assets failing, 0 crashed: kneeling-gardener 2, pregnant-person 6, sleeping-person 2, tour-guide 11, date-palm 1, hosta-clump 1, manifest 1).
- Symbol check (`validate-manifest.ts`, extended): 0 symbol mismatches. The manifest `symbols` rows hold only Slug ids, so semantics, scales and views are not stored in the manifest. The check compares ids in both directions against the asset file, validates each asset symbol's semantics, views and denominators, and checks that every symbol alternative's scale is one the symbol offers.
- parent-carrying-toddler: humans confirmed all four bands authored; the manifest row now says 200 and 500 `authored` (asset file verified: all four authored with elevation, front and plan). Rerun: `validateManifest` 0 issues; `ids.ts` 0 mismatches (`ids-final-clean.json`).

### Dimensions check (`🛠️scripts/registry/dims.py`, output `🗑️generated/haiku-registry/dims.json`)

Read-only, no edits. 149 of 149 rows checked against the asset file `dimensions`; tolerance 1 mm.

- `heightM`: 0 mismatches.
- `widthM`: 4 mismatches (listed below; the manifest is the first value, the asset file the second).
  - date-palm: 6.87 vs 6.3 (570 mm)
  - pregnant-person: 0.72 vs 0.69 (30 mm)
  - skateboarder: 1.218 vs 1.057 (161 mm)
  - operating-table: 2 vs 2.45 (−450 mm)
- `depthM`: not compared. The manifest row carries no depth (`ManifestEntry` has `heightM` and `widthM` only), so there is nothing to check against it. The asset `depthM` values are only in the asset files.
- Owners must confirm which side is right before anything is changed (D18 envelope fixes may explain some of these).
- Bone check (`lid-axis`, `pad-axis`): no manifest entry references either bone (0 matches in `🗂️catalog/🔣️.json`), and no asset file currently references them. Rerun of `ids.ts` (`ids-d36.json`): 0 mismatches, 0 duplicates, 547 named poses, 353 clips.
- D36 applied: pregnant-person `widthM` updated in the manifest from 0.72 to 0.69 (humans confirmed 0.69, measured at every band). Rerun (`dims-after-d36.json`): 3 mismatches left, date-palm, skateboarder and operating-table, awaiting the three confirmations that will follow.
