# Audit: authored staffage volumes (per asset, per scale)

Read-only measurement by `audit3-measure.py` (stdlib only). Raw data: `🗑️generated/audit3/results.json`; one row per asset and scale: `audit-authored.csv`. Counts are expanded by `repeat.count` unless stated. Shares are by solid count, not volume.

## 1. Splice state

- Assets: 152. Spliced (`volumes`, no old `volume`): 143. Not spliced (old `volume` still present, no `volumes`): 9. Both keys present: 0.
- Not spliced: 🌳️vegetation/japanese-maple, 🌳️vegetation/lombardy-poplar, 🌳️vegetation/london-plane-pollarded, 🌳️vegetation/miscanthus-grass, 🌳️vegetation/multi-stem-birch, 🌳️vegetation/scots-pine, 🌳️vegetation/small-leaved-lime, 🌳️vegetation/weeping-willow, 🚜️vehicles/city-bicycle.

## 2. Per-scale solid measures

Totals over spliced assets, then mean per asset with authored scale. Kind shares are counts of body solids.

| scale | authored | unavailable | body | thermal real | thermal ref | struct real | struct ref | box % | extrude % | loft % | revolve % | capsule % | ellipsoid % | slender | texture | envelope | sectionFaces | layer |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1:50 | 143 | 0 | 7697 | 1881 | 4136 | 3615 | 967 | 18 % | 10 % | 24 % | 22 % | 17 % | 9 % | 1145 | 413 | 0 | 338 | 3298 |
| 1:100 | 143 | 0 | 4418 | 543 | 3090 | 2892 | 790 | 16 % | 10 % | 32 % | 20 % | 15 % | 8 % | 707 | 23 | 0 | 236 | 937 |
| 1:200 | 137 | 6 | 2891 | 618 | 2238 | 2052 | 643 | 17 % | 11 % | 38 % | 19 % | 10 % | 4 % | 597 | 6 | 1 | 217 | 702 |
| 1:500 | 100 | 43 | 1604 | 555 | 1378 | 1631 | 394 | 18 % | 12 % | 45 % | 11 % | 12 % | 2 % | 256 | 1 | 4 | 139 | 422 |

Unresolved `of` references (sanity): 0 asset-scales [].

## 3. Scale independence

Per adjacent authored pair (finer, coarser): coarse body solids, ids shared with the finer scale, and of the shared ones identical / scaled-copy / near-copy (few numbers changed) / different. `subset` = every coarse body id exists at the finer scale.

| asset | pair | coarse body | shared ids | identical | scaled | near | different | new | subset |
|---|---|---|---|---|---|---|---|---|---|
| 🌳️vegetation/agave | 50→100 | 20 | 20 | 0 | 0 | 0 | 20 | 0 | yes |
| 🌳️vegetation/agave | 100→200 | 20 | 20 | 0 | 0 | 1 | 19 | 0 | yes |
| 🌳️vegetation/agave | 200→500 | 18 | 2 | 0 | 0 | 0 | 2 | 16 |  |
| 🌳️vegetation/apple-tree | 50→100 | 37 | 37 | 7 | 0 | 4 | 26 | 0 | yes |
| 🌳️vegetation/apple-tree | 100→200 | 27 | 27 | 0 | 0 | 5 | 22 | 0 | yes |
| 🌳️vegetation/apple-tree | 200→500 | 3 | 2 | 0 | 0 | 0 | 2 | 1 |  |
| 🌳️vegetation/bamboo-clump | 50→100 | 28 | 19 | 0 | 0 | 0 | 19 | 9 |  |
| 🌳️vegetation/bamboo-clump | 100→200 | 26 | 19 | 0 | 0 | 0 | 19 | 7 |  |
| 🌳️vegetation/bamboo-clump | 200→500 | 7 | 0 | 0 | 0 | 0 | 0 | 7 |  |
| 🌳️vegetation/date-palm | 50→100 | 24 | 24 | 1 | 0 | 0 | 23 | 0 | yes |
| 🌳️vegetation/date-palm | 100→200 | 23 | 23 | 4 | 0 | 0 | 19 | 0 | yes |
| 🌳️vegetation/date-palm | 200→500 | 11 | 3 | 0 | 0 | 0 | 3 | 8 |  |
| 🌳️vegetation/english-oak | 50→100 | 29 | 29 | 0 | 0 | 0 | 29 | 0 | yes |
| 🌳️vegetation/english-oak | 100→200 | 25 | 25 | 1 | 0 | 0 | 24 | 0 | yes |
| 🌳️vegetation/english-oak | 200→500 | 5 | 4 | 0 | 0 | 0 | 4 | 1 |  |
| 🌳️vegetation/hornbeam-hedge | 50→100 | 19 | 6 | 0 | 0 | 0 | 6 | 13 |  |
| 🌳️vegetation/hornbeam-hedge | 100→200 | 7 | 7 | 1 | 0 | 0 | 6 | 0 | yes |
| 🌳️vegetation/hornbeam-hedge | 200→500 | 2 | 1 | 1 | 0 | 0 | 0 | 1 |  |
| 🌳️vegetation/hosta-clump | 50→100 | 18 | 18 | 9 | 0 | 0 | 9 | 0 | yes |
| 🌳️vegetation/hosta-clump | 100→200 | 7 | 0 | 0 | 0 | 0 | 0 | 7 |  |
| 🌳️vegetation/ivy-on-trellis | 50→100 | 20 | 15 | 2 | 0 | 1 | 12 | 5 |  |
| 🌳️vegetation/ivy-on-trellis | 100→200 | 14 | 14 | 4 | 0 | 2 | 8 | 0 | yes |
| 🌳️vegetation/ivy-on-trellis | 200→500 | 7 | 4 | 0 | 0 | 4 | 0 | 3 |  |
| 🏗️machinery/concrete-mixer-truck | 50→100 | 50 | 48 | 16 | 0 | 19 | 13 | 2 |  |
| 🏗️machinery/concrete-mixer-truck | 100→200 | 33 | 31 | 9 | 0 | 9 | 13 | 2 |  |
| 🏗️machinery/concrete-mixer-truck | 200→500 | 15 | 7 | 3 | 0 | 0 | 4 | 8 |  |
| 🏗️machinery/diesel-generator | 50→100 | 27 | 27 | 17 | 0 | 6 | 4 | 0 | yes |
| 🏗️machinery/diesel-generator | 100→200 | 22 | 18 | 14 | 0 | 4 | 0 | 4 |  |
| 🏗️machinery/diesel-generator | 200→500 | 12 | 12 | 1 | 0 | 9 | 2 | 0 | yes |
| 🏗️machinery/evaporative-humidifier | 50→100 | 14 | 11 | 3 | 0 | 5 | 3 | 3 |  |
| 🏗️machinery/evaporative-humidifier | 100→200 | 10 | 10 | 0 | 0 | 5 | 5 | 0 | yes |
| 🏗️machinery/excavator | 50→100 | 31 | 27 | 24 | 0 | 1 | 2 | 4 |  |
| 🏗️machinery/excavator | 100→200 | 22 | 21 | 16 | 0 | 2 | 3 | 1 |  |
| 🏗️machinery/excavator | 200→500 | 18 | 18 | 15 | 0 | 1 | 2 | 0 | yes |
| 🏗️machinery/expansion-valve | 50→100 | 5 | 5 | 1 | 0 | 2 | 2 | 0 | yes |
| 🏗️machinery/finned-heat-exchanger | 50→100 | 8 | 7 | 0 | 0 | 4 | 3 | 1 |  |
| 🏗️machinery/finned-heat-exchanger | 100→200 | 6 | 6 | 0 | 0 | 0 | 6 | 0 | yes |
| 🏗️machinery/forklift | 50→100 | 33 | 22 | 20 | 0 | 1 | 1 | 11 |  |
| 🏗️machinery/forklift | 100→200 | 28 | 27 | 12 | 0 | 4 | 11 | 1 |  |
| 🏗️machinery/forklift | 200→500 | 22 | 21 | 7 | 0 | 9 | 5 | 1 |  |
| 🏗️machinery/heat-pump-outdoor-unit | 50→100 | 22 | 21 | 7 | 0 | 11 | 3 | 1 |  |
| 🏗️machinery/heat-pump-outdoor-unit | 100→200 | 13 | 12 | 4 | 0 | 2 | 6 | 1 |  |
| 🏗️machinery/heat-pump-outdoor-unit | 200→500 | 12 | 11 | 2 | 0 | 8 | 1 | 1 |  |
| 🏗️machinery/oil-pumpjack | 50→100 | 44 | 42 | 22 | 0 | 14 | 6 | 2 |  |
| 🏗️machinery/oil-pumpjack | 100→200 | 38 | 37 | 18 | 0 | 14 | 5 | 1 |  |
| 🏗️machinery/oil-pumpjack | 200→500 | 33 | 33 | 11 | 0 | 18 | 4 | 0 | yes |
| 🏗️machinery/plate-heat-exchanger | 50→100 | 14 | 14 | 6 | 0 | 1 | 7 | 0 | yes |
| 🏗️machinery/plate-heat-exchanger | 100→200 | 10 | 10 | 0 | 0 | 4 | 6 | 0 | yes |
| 🏗️machinery/scissor-lift | 50→100 | 53 | 53 | 17 | 0 | 14 | 22 | 0 | yes |
| 🏗️machinery/scissor-lift | 100→200 | 39 | 39 | 13 | 0 | 13 | 13 | 0 | yes |
| 🏗️machinery/scroll-compressor | 50→100 | 8 | 5 | 0 | 0 | 2 | 3 | 3 |  |
| 🏗️machinery/scroll-compressor | 100→200 | 6 | 6 | 1 | 0 | 2 | 3 | 0 | yes |
| 🏗️machinery/solar-thermal-collector | 50→100 | 10 | 10 | 0 | 0 | 1 | 9 | 0 | yes |
| 🏗️machinery/solar-thermal-collector | 100→200 | 9 | 8 | 0 | 0 | 1 | 7 | 1 |  |
| 🏗️machinery/solar-thermal-collector | 200→500 | 4 | 3 | 0 | 0 | 3 | 0 | 1 |  |
| 🏗️machinery/sorption-wheel | 50→100 | 8 | 8 | 2 | 0 | 3 | 3 | 0 | yes |
| 🏗️machinery/sorption-wheel | 100→200 | 5 | 5 | 0 | 0 | 0 | 5 | 0 | yes |
| 🏗️machinery/tower-crane | 50→100 | 111 | 97 | 52 | 0 | 16 | 29 | 14 |  |
| 🏗️machinery/tower-crane | 100→200 | 102 | 98 | 55 | 0 | 40 | 3 | 4 |  |
| 🏗️machinery/tower-crane | 200→500 | 60 | 49 | 28 | 0 | 19 | 2 | 11 |  |
| 🏗️machinery/wheel-loader | 50→100 | 50 | 50 | 27 | 0 | 22 | 1 | 0 | yes |
| 🏗️machinery/wheel-loader | 100→200 | 31 | 31 | 15 | 0 | 14 | 2 | 0 | yes |
| 🏗️machinery/wheel-loader | 200→500 | 28 | 27 | 12 | 0 | 13 | 2 | 1 |  |
| 🐕️animals/dairy-cow | 50→100 | 40 | 40 | 7 | 0 | 4 | 29 | 0 | yes |
| 🐕️animals/dairy-cow | 100→200 | 29 | 29 | 0 | 0 | 0 | 29 | 0 | yes |
| 🐕️animals/dairy-cow | 200→500 | 21 | 20 | 0 | 0 | 0 | 20 | 1 |  |
| 🐕️animals/domestic-cat | 50→100 | 50 | 49 | 5 | 0 | 9 | 35 | 1 |  |
| 🐕️animals/domestic-cat | 100→200 | 25 | 11 | 0 | 0 | 0 | 11 | 14 |  |
| 🐕️animals/goat | 50→100 | 31 | 31 | 3 | 0 | 1 | 27 | 0 | yes |
| 🐕️animals/goat | 100→200 | 23 | 18 | 0 | 0 | 0 | 18 | 5 |  |
| 🐕️animals/goat | 200→500 | 18 | 6 | 0 | 0 | 0 | 6 | 12 |  |
| 🐕️animals/hedgehog | 50→100 | 12 | 12 | 0 | 0 | 3 | 9 | 0 | yes |
| 🐕️animals/herring-gull | 50→100 | 14 | 12 | 0 | 0 | 0 | 12 | 2 |  |
| 🐕️animals/herring-gull | 100→200 | 13 | 12 | 0 | 0 | 0 | 12 | 1 |  |
| 🐕️animals/horse | 50→100 | 43 | 43 | 2 | 0 | 2 | 39 | 0 | yes |
| 🐕️animals/horse | 100→200 | 40 | 39 | 1 | 0 | 3 | 35 | 1 |  |
| 🐕️animals/horse | 200→500 | 24 | 24 | 1 | 0 | 0 | 23 | 0 | yes |
| 🐕️animals/labrador-dog | 50→100 | 37 | 37 | 0 | 0 | 7 | 30 | 0 | yes |
| 🐕️animals/labrador-dog | 100→200 | 23 | 23 | 0 | 0 | 0 | 23 | 0 | yes |
| 🐕️animals/labrador-dog | 200→500 | 20 | 20 | 0 | 0 | 0 | 20 | 0 | yes |
| 🐕️animals/mallard-duck | 50→100 | 14 | 12 | 0 | 0 | 0 | 12 | 2 |  |
| 🐕️animals/mallard-duck | 100→200 | 14 | 14 | 0 | 0 | 4 | 10 | 0 | yes |
| 🐕️animals/pig | 50→100 | 28 | 24 | 1 | 0 | 2 | 21 | 4 |  |
| 🐕️animals/pig | 100→200 | 24 | 24 | 0 | 0 | 2 | 22 | 0 | yes |
| 🐕️animals/pig | 200→500 | 12 | 12 | 0 | 0 | 0 | 12 | 0 | yes |
| 🐕️animals/pigeon | 50→100 | 27 | 23 | 0 | 0 | 1 | 22 | 4 |  |
| 🐕️animals/pigeon | 100→200 | 13 | 7 | 0 | 0 | 0 | 7 | 6 |  |
| 🐕️animals/red-fox | 50→100 | 22 | 22 | 0 | 0 | 0 | 22 | 0 | yes |
| 🐕️animals/red-fox | 100→200 | 21 | 21 | 0 | 0 | 0 | 21 | 0 | yes |
| 🐕️animals/red-squirrel | 50→100 | 9 | 9 | 0 | 0 | 0 | 9 | 0 | yes |
| 🐕️animals/roe-deer | 50→100 | 29 | 29 | 2 | 0 | 0 | 27 | 0 | yes |
| 🐕️animals/roe-deer | 100→200 | 26 | 26 | 0 | 0 | 0 | 26 | 0 | yes |
| 🐕️animals/roe-deer | 200→500 | 21 | 6 | 0 | 0 | 0 | 6 | 15 |  |
| 🐕️animals/sheep | 50→100 | 80 | 76 | 0 | 0 | 15 | 61 | 4 |  |
| 🐕️animals/sheep | 100→200 | 65 | 65 | 0 | 0 | 10 | 55 | 0 | yes |
| 🐕️animals/sheep | 200→500 | 12 | 10 | 0 | 0 | 0 | 10 | 2 |  |
| 👤️humans/child-on-kick-scooter | 50→100 | 31 | 30 | 0 | 0 | 1 | 29 | 1 |  |
| 👤️humans/child-on-kick-scooter | 100→200 | 29 | 25 | 5 | 0 | 2 | 18 | 4 |  |
| 👤️humans/child-on-kick-scooter | 200→500 | 26 | 13 | 0 | 0 | 0 | 13 | 13 |  |
| 👤️humans/courier-with-hand-truck | 50→100 | 75 | 73 | 14 | 0 | 10 | 49 | 2 |  |
| 👤️humans/courier-with-hand-truck | 100→200 | 49 | 43 | 4 | 0 | 3 | 36 | 6 |  |
| 👤️humans/courier-with-hand-truck | 200→500 | 28 | 28 | 2 | 0 | 3 | 23 | 0 | yes |
| 👤️humans/cyclist | 50→100 | 55 | 49 | 18 | 0 | 8 | 23 | 6 |  |
| 👤️humans/cyclist | 100→200 | 48 | 43 | 4 | 0 | 21 | 18 | 5 |  |
| 👤️humans/cyclist | 200→500 | 42 | 32 | 0 | 0 | 4 | 28 | 10 |  |
| 👤️humans/elderly-with-cane | 50→100 | 45 | 43 | 1 | 0 | 6 | 36 | 2 |  |
| 👤️humans/elderly-with-cane | 100→200 | 29 | 29 | 0 | 0 | 3 | 26 | 0 | yes |
| 👤️humans/elderly-with-cane | 200→500 | 21 | 12 | 0 | 0 | 0 | 12 | 9 |  |
| 👤️humans/jogger | 50→100 | 33 | 32 | 1 | 0 | 1 | 30 | 1 |  |
| 👤️humans/jogger | 100→200 | 23 | 22 | 0 | 0 | 1 | 21 | 1 |  |
| 👤️humans/jogger | 200→500 | 17 | 17 | 0 | 0 | 0 | 17 | 0 | yes |
| 👤️humans/kneeling-gardener | 50→100 | 32 | 32 | 3 | 0 | 2 | 27 | 0 | yes |
| 👤️humans/leaning-person | 50→100 | 35 | 35 | 6 | 0 | 3 | 26 | 0 | yes |
| 👤️humans/leaning-person | 100→200 | 24 | 24 | 2 | 0 | 4 | 18 | 0 | yes |
| 👤️humans/leaning-person | 200→500 | 19 | 17 | 1 | 0 | 0 | 16 | 2 |  |
| 👤️humans/parent-carrying-toddler | 50→100 | 31 | 31 | 3 | 0 | 0 | 28 | 0 | yes |
| 👤️humans/parent-carrying-toddler | 100→200 | 29 | 29 | 1 | 0 | 0 | 28 | 0 | yes |
| 👤️humans/parent-carrying-toddler | 200→500 | 25 | 25 | 0 | 0 | 0 | 25 | 0 | yes |
| 👤️humans/parent-with-stroller | 50→100 | 94 | 77 | 0 | 0 | 23 | 54 | 17 |  |
| 👤️humans/parent-with-stroller | 100→200 | 43 | 30 | 2 | 0 | 11 | 17 | 13 |  |
| 👤️humans/parent-with-stroller | 200→500 | 32 | 30 | 1 | 0 | 9 | 20 | 2 |  |
| 👤️humans/person-cooking | 50→100 | 62 | 62 | 6 | 0 | 8 | 48 | 0 | yes |
| 👤️humans/person-cooking | 100→200 | 40 | 36 | 0 | 0 | 1 | 35 | 4 |  |
| 👤️humans/person-cooking | 200→500 | 23 | 19 | 0 | 0 | 0 | 19 | 4 |  |
| 👤️humans/person-on-crutches | 50→100 | 35 | 35 | 0 | 0 | 1 | 34 | 0 | yes |
| 👤️humans/person-on-crutches | 100→200 | 26 | 26 | 0 | 0 | 0 | 26 | 0 | yes |
| 👤️humans/person-on-crutches | 200→500 | 20 | 20 | 0 | 0 | 1 | 19 | 0 | yes |
| 👤️humans/person-with-rollator | 50→100 | 83 | 83 | 10 | 0 | 15 | 58 | 0 | yes |
| 👤️humans/person-with-rollator | 100→200 | 46 | 37 | 11 | 0 | 1 | 25 | 9 |  |
| 👤️humans/person-with-rollator | 200→500 | 37 | 37 | 0 | 0 | 10 | 27 | 0 | yes |
| 👤️humans/person-with-umbrella | 50→100 | 50 | 50 | 9 | 0 | 6 | 35 | 0 | yes |
| 👤️humans/person-with-umbrella | 100→200 | 36 | 36 | 4 | 0 | 6 | 26 | 0 | yes |
| 👤️humans/person-with-umbrella | 200→500 | 25 | 25 | 0 | 0 | 3 | 22 | 0 | yes |
| 👤️humans/person-with-white-cane | 50→100 | 56 | 55 | 2 | 0 | 4 | 49 | 1 |  |
| 👤️humans/person-with-white-cane | 100→200 | 35 | 31 | 0 | 0 | 2 | 29 | 4 |  |
| 👤️humans/person-with-white-cane | 200→500 | 18 | 18 | 1 | 0 | 1 | 16 | 0 | yes |
| 👤️humans/pregnant-person | 50→100 | 44 | 44 | 11 | 0 | 5 | 28 | 0 | yes |
| 👤️humans/pregnant-person | 100→200 | 28 | 28 | 3 | 0 | 6 | 19 | 0 | yes |
| 👤️humans/pregnant-person | 200→500 | 21 | 21 | 0 | 0 | 0 | 21 | 0 | yes |
| 👤️humans/runner-with-blade-prosthesis | 50→100 | 37 | 37 | 5 | 0 | 6 | 26 | 0 | yes |
| 👤️humans/runner-with-blade-prosthesis | 100→200 | 27 | 27 | 2 | 0 | 6 | 19 | 0 | yes |
| 👤️humans/runner-with-blade-prosthesis | 200→500 | 22 | 22 | 0 | 0 | 1 | 21 | 0 | yes |
| 👤️humans/running-child | 50→100 | 45 | 45 | 9 | 0 | 9 | 27 | 0 | yes |
| 👤️humans/running-child | 100→200 | 24 | 24 | 2 | 0 | 0 | 22 | 0 | yes |
| 👤️humans/running-child | 200→500 | 20 | 20 | 0 | 0 | 0 | 20 | 0 | yes |
| 👤️humans/seated-guitarist | 50→100 | 41 | 41 | 5 | 0 | 16 | 20 | 0 | yes |
| 👤️humans/seated-guitarist | 100→200 | 35 | 35 | 7 | 0 | 12 | 16 | 0 | yes |
| 👤️humans/seated-guitarist | 200→500 | 25 | 24 | 1 | 0 | 6 | 17 | 1 |  |
| 👤️humans/seated-office-worker | 50→100 | 66 | 66 | 20 | 0 | 16 | 30 | 0 | yes |
| 👤️humans/seated-office-worker | 100→200 | 49 | 49 | 9 | 0 | 18 | 22 | 0 | yes |
| 👤️humans/seated-office-worker | 200→500 | 36 | 36 | 3 | 0 | 7 | 26 | 0 | yes |
| 👤️humans/seated-reader | 50→100 | 41 | 41 | 10 | 0 | 9 | 22 | 0 | yes |
| 👤️humans/seated-reader | 100→200 | 33 | 33 | 9 | 0 | 9 | 15 | 0 | yes |
| 👤️humans/seated-reader | 200→500 | 27 | 27 | 1 | 0 | 4 | 22 | 0 | yes |
| 👤️humans/skateboarder | 50→100 | 34 | 34 | 12 | 0 | 2 | 20 | 0 | yes |
| 👤️humans/skateboarder | 100→200 | 22 | 22 | 4 | 0 | 7 | 11 | 0 | yes |
| 👤️humans/skateboarder | 200→500 | 19 | 19 | 0 | 0 | 1 | 18 | 0 | yes |
| 👤️humans/sleeping-person | 50→100 | 39 | 39 | 10 | 0 | 5 | 24 | 0 | yes |
| 👤️humans/sleeping-person | 100→200 | 22 | 22 | 6 | 0 | 1 | 15 | 0 | yes |
| 👤️humans/sleeping-person | 200→500 | 16 | 16 | 3 | 0 | 3 | 10 | 0 | yes |
| 👤️humans/standing-adult | 50→100 | 41 | 41 | 6 | 0 | 7 | 28 | 0 | yes |
| 👤️humans/standing-adult | 100→200 | 21 | 21 | 0 | 0 | 0 | 21 | 0 | yes |
| 👤️humans/standing-adult | 200→500 | 18 | 18 | 0 | 0 | 0 | 18 | 0 | yes |
| 👤️humans/standing-phone-user | 50→100 | 62 | 62 | 6 | 0 | 7 | 49 | 0 | yes |
| 👤️humans/standing-phone-user | 100→200 | 40 | 36 | 0 | 0 | 1 | 35 | 4 |  |
| 👤️humans/standing-phone-user | 200→500 | 21 | 20 | 0 | 0 | 0 | 20 | 1 |  |
| 👤️humans/tour-guide | 50→100 | 23 | 23 | 1 | 0 | 0 | 22 | 0 | yes |
| 👤️humans/tour-guide | 100→200 | 21 | 21 | 0 | 0 | 2 | 19 | 0 | yes |
| 👤️humans/tour-guide | 200→500 | 20 | 20 | 0 | 0 | 2 | 18 | 0 | yes |
| 👤️humans/walking-commuter | 50→100 | 54 | 52 | 0 | 0 | 8 | 44 | 2 |  |
| 👤️humans/walking-commuter | 100→200 | 27 | 23 | 0 | 0 | 0 | 23 | 4 |  |
| 👤️humans/walking-commuter | 200→500 | 21 | 17 | 0 | 0 | 0 | 17 | 4 |  |
| 👤️humans/wheelchair-user | 50→100 | 58 | 48 | 10 | 0 | 3 | 35 | 10 |  |
| 👤️humans/wheelchair-user | 100→200 | 48 | 36 | 2 | 0 | 5 | 29 | 12 |  |
| 👤️humans/wheelchair-user | 200→500 | 34 | 26 | 0 | 0 | 1 | 25 | 8 |  |
| 👤️humans/worker-carrying-box | 50→100 | 20 | 20 | 0 | 0 | 1 | 19 | 0 | yes |
| 👤️humans/worker-carrying-box | 100→200 | 20 | 20 | 1 | 0 | 0 | 19 | 0 | yes |
| 👤️humans/worker-carrying-box | 200→500 | 19 | 19 | 1 | 0 | 0 | 18 | 0 | yes |
| 🔌️appliances/ceiling-diffuser | 50→100 | 4 | 4 | 0 | 0 | 0 | 4 | 0 | yes |
| 🔌️appliances/ceiling-diffuser | 100→200 | 3 | 2 | 0 | 0 | 1 | 1 | 1 |  |
| 🔌️appliances/circulation-pump | 50→100 | 7 | 7 | 1 | 0 | 2 | 4 | 0 | yes |
| 🔌️appliances/circulation-pump | 100→200 | 4 | 2 | 2 | 0 | 0 | 0 | 2 |  |
| 🔌️appliances/decentralised-hrv-fan | 50→100 | 10 | 10 | 1 | 0 | 0 | 9 | 0 | yes |
| 🔌️appliances/decentralised-hrv-fan | 100→200 | 7 | 7 | 0 | 0 | 1 | 6 | 0 | yes |
| 🔌️appliances/desktop-monitor | 50→100 | 5 | 5 | 0 | 0 | 3 | 2 | 0 | yes |
| 🔌️appliances/desktop-monitor | 100→200 | 3 | 0 | 0 | 0 | 0 | 0 | 3 |  |
| 🔌️appliances/duct-fan | 50→100 | 9 | 9 | 0 | 0 | 2 | 7 | 0 | yes |
| 🔌️appliances/duct-fan | 100→200 | 6 | 6 | 0 | 0 | 0 | 6 | 0 | yes |
| 🔌️appliances/electric-fan-heater | 50→100 | 8 | 6 | 0 | 0 | 3 | 3 | 2 |  |
| 🔌️appliances/electric-fan-heater | 100→200 | 8 | 8 | 0 | 0 | 3 | 5 | 0 | yes |
| 🔌️appliances/electric-kettle | 50→100 | 7 | 7 | 2 | 0 | 3 | 2 | 0 | yes |
| 🔌️appliances/electric-kettle | 100→200 | 6 | 6 | 0 | 0 | 1 | 5 | 0 | yes |
| 🔌️appliances/electricity-meter | 50→100 | 9 | 9 | 1 | 0 | 2 | 6 | 0 | yes |
| 🔌️appliances/electricity-meter | 100→200 | 6 | 5 | 1 | 0 | 1 | 3 | 1 |  |
| 🔌️appliances/gas-boiler | 50→100 | 10 | 9 | 2 | 0 | 3 | 4 | 1 |  |
| 🔌️appliances/gas-boiler | 100→200 | 5 | 5 | 1 | 0 | 2 | 2 | 0 | yes |
| 🔌️appliances/gas-hob | 50→100 | 7 | 5 | 0 | 0 | 3 | 2 | 2 |  |
| 🔌️appliances/gas-hob | 100→200 | 5 | 4 | 1 | 0 | 0 | 3 | 1 |  |
| 🔌️appliances/gas-meter | 50→100 | 8 | 6 | 0 | 0 | 3 | 3 | 2 |  |
| 🔌️appliances/gas-meter | 100→200 | 5 | 5 | 0 | 0 | 0 | 5 | 0 | yes |
| 🔌️appliances/hot-water-cylinder | 50→100 | 11 | 10 | 6 | 0 | 3 | 1 | 1 |  |
| 🔌️appliances/hot-water-cylinder | 100→200 | 7 | 5 | 1 | 0 | 2 | 2 | 2 |  |
| 🔌️appliances/hot-water-cylinder | 200→500 | 7 | 7 | 2 | 0 | 2 | 3 | 0 | yes |
| 🔌️appliances/kitchen-extractor-hood | 50→100 | 9 | 9 | 0 | 0 | 2 | 7 | 0 | yes |
| 🔌️appliances/kitchen-extractor-hood | 100→200 | 5 | 5 | 0 | 0 | 2 | 3 | 0 | yes |
| 🔌️appliances/laptop | 50→100 | 2 | 2 | 0 | 0 | 2 | 0 | 0 | yes |
| 🔌️appliances/laptop | 100→200 | 2 | 1 | 0 | 0 | 0 | 1 | 1 |  |
| 🔌️appliances/mvhr-unit | 50→100 | 7 | 7 | 0 | 0 | 2 | 5 | 0 | yes |
| 🔌️appliances/mvhr-unit | 100→200 | 7 | 7 | 0 | 0 | 1 | 6 | 0 | yes |
| 🔌️appliances/pendant-lamp | 50→100 | 6 | 6 | 1 | 0 | 0 | 5 | 0 | yes |
| 🔌️appliances/pendant-lamp | 100→200 | 4 | 3 | 0 | 0 | 0 | 3 | 1 |  |
| 🔌️appliances/room-thermostat | 50→100 | 2 | 2 | 0 | 0 | 0 | 2 | 0 | yes |
| 🔌️appliances/server-rack | 50→100 | 15 | 13 | 6 | 0 | 3 | 4 | 2 |  |
| 🔌️appliances/server-rack | 100→200 | 12 | 10 | 0 | 0 | 9 | 1 | 2 |  |
| 🔌️appliances/server-rack | 200→500 | 4 | 3 | 0 | 0 | 2 | 1 | 1 |  |
| 🔌️appliances/shower-head | 50→100 | 38 | 27 | 0 | 0 | 1 | 26 | 11 |  |
| 🔌️appliances/shower-head | 100→200 | 15 | 13 | 0 | 0 | 2 | 11 | 2 |  |
| 🔌️appliances/split-ac-indoor-unit | 50→100 | 4 | 4 | 1 | 0 | 1 | 2 | 0 | yes |
| 🔌️appliances/split-ac-indoor-unit | 100→200 | 3 | 3 | 1 | 0 | 0 | 2 | 0 | yes |
| 🔌️appliances/thermostatic-radiator-valve | 50→100 | 2 | 1 | 0 | 0 | 0 | 1 | 1 |  |
| 🔌️appliances/toaster | 50→100 | 9 | 9 | 1 | 0 | 5 | 3 | 0 | yes |
| 🔌️appliances/toaster | 100→200 | 4 | 3 | 0 | 0 | 2 | 1 | 1 |  |
| 🔌️appliances/tower-server | 50→100 | 10 | 10 | 1 | 0 | 4 | 5 | 0 | yes |
| 🔌️appliances/tower-server | 100→200 | 2 | 0 | 0 | 0 | 0 | 0 | 2 |  |
| 🔌️appliances/vacuum-cleaner | 50→100 | 25 | 25 | 13 | 0 | 1 | 11 | 0 | yes |
| 🔌️appliances/vacuum-cleaner | 100→200 | 16 | 15 | 1 | 0 | 11 | 3 | 1 |  |
| 🔌️appliances/vacuum-cleaner | 200→500 | 16 | 16 | 1 | 0 | 12 | 3 | 0 | yes |
| 🚜️vehicles/cargo-bike | 50→100 | 54 | 54 | 14 | 0 | 13 | 27 | 0 | yes |
| 🚜️vehicles/cargo-bike | 100→200 | 36 | 34 | 6 | 0 | 18 | 10 | 2 |  |
| 🚜️vehicles/cargo-bike | 200→500 | 22 | 20 | 0 | 0 | 11 | 9 | 2 |  |
| 🚜️vehicles/city-bus | 50→100 | 32 | 26 | 15 | 0 | 8 | 3 | 6 |  |
| 🚜️vehicles/city-bus | 100→200 | 24 | 20 | 11 | 0 | 0 | 9 | 4 |  |
| 🚜️vehicles/city-bus | 200→500 | 17 | 11 | 5 | 0 | 2 | 4 | 6 |  |
| 🚜️vehicles/city-car | 50→100 | 35 | 31 | 16 | 0 | 4 | 11 | 4 |  |
| 🚜️vehicles/city-car | 100→200 | 23 | 20 | 10 | 0 | 3 | 7 | 3 |  |
| 🚜️vehicles/city-car | 200→500 | 9 | 5 | 0 | 0 | 0 | 5 | 4 |  |
| 🚜️vehicles/delivery-van | 50→100 | 29 | 29 | 19 | 0 | 4 | 6 | 0 | yes |
| 🚜️vehicles/delivery-van | 100→200 | 22 | 22 | 10 | 0 | 2 | 10 | 0 | yes |
| 🚜️vehicles/delivery-van | 200→500 | 15 | 11 | 0 | 0 | 2 | 9 | 4 |  |
| 🚜️vehicles/electric-kick-scooter | 50→100 | 20 | 20 | 4 | 0 | 9 | 7 | 0 | yes |
| 🚜️vehicles/electric-kick-scooter | 100→200 | 6 | 2 | 0 | 0 | 0 | 2 | 4 |  |
| 🚜️vehicles/motor-scooter | 50→100 | 20 | 20 | 12 | 0 | 0 | 8 | 0 | yes |
| 🚜️vehicles/motor-scooter | 100→200 | 13 | 11 | 5 | 0 | 0 | 6 | 2 |  |
| 🚜️vehicles/motor-scooter | 200→500 | 9 | 9 | 4 | 0 | 2 | 3 | 0 | yes |
| 🚜️vehicles/tram | 50→100 | 77 | 77 | 57 | 0 | 8 | 12 | 0 | yes |
| 🚜️vehicles/tram | 100→200 | 59 | 57 | 33 | 0 | 6 | 18 | 2 |  |
| 🚜️vehicles/tram | 200→500 | 53 | 47 | 27 | 0 | 12 | 8 | 6 |  |
| 🚜️vehicles/vintage-car-1929 | 50→100 | 106 | 98 | 60 | 0 | 17 | 21 | 8 |  |
| 🚜️vehicles/vintage-car-1929 | 100→200 | 73 | 66 | 30 | 0 | 9 | 27 | 7 |  |
| 🚜️vehicles/vintage-car-1929 | 200→500 | 15 | 9 | 0 | 0 | 2 | 7 | 6 |  |
| 🛋️furniture/bar-stool | 50→100 | 6 | 6 | 2 | 0 | 0 | 4 | 0 | yes |
| 🛋️furniture/bar-stool | 100→200 | 5 | 4 | 1 | 0 | 0 | 3 | 1 |  |
| 🛋️furniture/bar-stool | 200→500 | 4 | 3 | 1 | 0 | 1 | 1 | 1 |  |
| 🛋️furniture/bookshelf | 50→100 | 16 | 16 | 0 | 0 | 3 | 13 | 0 | yes |
| 🛋️furniture/bookshelf | 100→200 | 16 | 16 | 0 | 0 | 1 | 15 | 0 | yes |
| 🛋️furniture/bookshelf | 200→500 | 16 | 16 | 1 | 0 | 4 | 11 | 0 | yes |
| 🛋️furniture/cafe-chair | 50→100 | 12 | 11 | 2 | 0 | 8 | 1 | 1 |  |
| 🛋️furniture/cafe-chair | 100→200 | 11 | 11 | 8 | 0 | 0 | 3 | 0 | yes |
| 🛋️furniture/cafe-chair | 200→500 | 10 | 10 | 8 | 0 | 1 | 1 | 0 | yes |
| 🛋️furniture/dining-table | 50→100 | 8 | 6 | 0 | 0 | 1 | 5 | 2 |  |
| 🛋️furniture/dining-table | 100→200 | 8 | 7 | 0 | 0 | 2 | 5 | 1 |  |
| 🛋️furniture/dining-table | 200→500 | 5 | 3 | 2 | 0 | 1 | 0 | 2 |  |
| 🛋️furniture/folding-deck-chair | 50→100 | 9 | 9 | 2 | 0 | 0 | 7 | 0 | yes |
| 🛋️furniture/folding-deck-chair | 100→200 | 5 | 5 | 0 | 0 | 0 | 5 | 0 | yes |
| 🛋️furniture/folding-deck-chair | 200→500 | 5 | 5 | 3 | 0 | 2 | 0 | 0 | yes |
| 🛋️furniture/height-adjustable-desk | 50→100 | 22 | 21 | 10 | 0 | 2 | 9 | 1 |  |
| 🛋️furniture/height-adjustable-desk | 100→200 | 17 | 17 | 5 | 0 | 1 | 11 | 0 | yes |
| 🛋️furniture/height-adjustable-desk | 200→500 | 14 | 14 | 13 | 0 | 1 | 0 | 0 | yes |
| 🛋️furniture/office-swivel-chair | 50→100 | 19 | 19 | 9 | 0 | 1 | 9 | 0 | yes |
| 🛋️furniture/office-swivel-chair | 100→200 | 16 | 15 | 0 | 0 | 1 | 14 | 1 |  |
| 🛋️furniture/office-swivel-chair | 200→500 | 14 | 14 | 0 | 0 | 0 | 14 | 0 | yes |
| 🛋️furniture/operating-table | 50→100 | 20 | 20 | 0 | 0 | 6 | 14 | 0 | yes |
| 🛋️furniture/operating-table | 100→200 | 8 | 5 | 0 | 0 | 4 | 1 | 3 |  |
| 🛋️furniture/operating-table | 200→500 | 7 | 7 | 1 | 0 | 2 | 4 | 0 | yes |
| 🛋️furniture/park-bench | 50→100 | 20 | 14 | 4 | 0 | 4 | 6 | 6 |  |
| 🛋️furniture/park-bench | 100→200 | 16 | 16 | 3 | 0 | 4 | 9 | 0 | yes |
| 🛋️furniture/park-bench | 200→500 | 8 | 4 | 0 | 0 | 2 | 2 | 4 |  |
| 🛋️furniture/picnic-table | 50→100 | 9 | 8 | 2 | 0 | 2 | 4 | 1 |  |
| 🛋️furniture/picnic-table | 100→200 | 8 | 7 | 2 | 0 | 3 | 2 | 1 |  |
| 🛋️furniture/picnic-table | 200→500 | 7 | 7 | 2 | 0 | 1 | 4 | 0 | yes |
| 🛋️furniture/single-bed | 50→100 | 12 | 9 | 4 | 0 | 5 | 0 | 3 |  |
| 🛋️furniture/single-bed | 100→200 | 9 | 9 | 3 | 0 | 0 | 6 | 0 | yes |
| 🛋️furniture/single-bed | 200→500 | 7 | 6 | 3 | 0 | 2 | 1 | 1 |  |
| 🛋️furniture/three-seat-sofa | 50→100 | 11 | 11 | 2 | 0 | 3 | 6 | 0 | yes |
| 🛋️furniture/three-seat-sofa | 100→200 | 11 | 7 | 1 | 0 | 2 | 4 | 4 |  |
| 🛋️furniture/three-seat-sofa | 200→500 | 7 | 3 | 1 | 0 | 0 | 2 | 4 |  |
| 🧱️components/cantilever-balcony | 50→100 | 7 | 5 | 3 | 0 | 0 | 2 | 2 |  |
| 🧱️components/cantilever-balcony | 100→200 | 5 | 4 | 4 | 0 | 0 | 0 | 1 |  |
| 🧱️components/cantilever-balcony | 200→500 | 3 | 2 | 0 | 0 | 1 | 1 | 1 |  |
| 🧱️components/curtain-wall-bay | 50→100 | 7 | 0 | 0 | 0 | 0 | 0 | 7 |  |
| 🧱️components/curtain-wall-bay | 100→200 | 3 | 1 | 1 | 0 | 0 | 0 | 2 |  |
| 🧱️components/curtain-wall-bay | 200→500 | 3 | 3 | 0 | 0 | 0 | 3 | 0 | yes |
| 🧱️components/etics-brick-wall-12 | 50→100 | 3 | 2 | 2 | 0 | 0 | 0 | 1 |  |
| 🧱️components/etics-brick-wall-12 | 100→200 | 2 | 2 | 1 | 0 | 1 | 0 | 0 | yes |
| 🧱️components/etics-brick-wall-12 | 200→500 | 1 | 0 | 0 | 0 | 0 | 0 | 1 |  |
| 🧱️components/etics-brick-wall-30 | 50→100 | 3 | 2 | 2 | 0 | 0 | 0 | 1 |  |
| 🧱️components/etics-brick-wall-30 | 100→200 | 2 | 2 | 1 | 0 | 1 | 0 | 0 | yes |
| 🧱️components/etics-brick-wall-30 | 200→500 | 1 | 0 | 0 | 0 | 0 | 0 | 1 |  |
| 🧱️components/external-venetian-blind | 50→100 | 24 | 21 | 1 | 0 | 17 | 3 | 3 |  |
| 🧱️components/external-venetian-blind | 100→200 | 24 | 24 | 7 | 0 | 0 | 17 | 0 | yes |
| 🧱️components/external-venetian-blind | 200→500 | 12 | 12 | 3 | 0 | 9 | 0 | 0 | yes |
| 🧱️components/glass-balustrade | 50→100 | 3 | 3 | 0 | 0 | 2 | 1 | 0 | yes |
| 🧱️components/glass-balustrade | 100→200 | 3 | 3 | 0 | 0 | 2 | 1 | 0 | yes |
| 🧱️components/glulam-beam | 50→100 | 3 | 2 | 0 | 0 | 2 | 0 | 1 |  |
| 🧱️components/glulam-beam | 100→200 | 3 | 3 | 1 | 0 | 2 | 0 | 0 | yes |
| 🧱️components/glulam-beam | 200→500 | 1 | 1 | 0 | 0 | 1 | 0 | 0 | yes |
| 🧱️components/masonry-wall-with-lintel | 50→100 | 8 | 6 | 4 | 0 | 2 | 0 | 2 |  |
| 🧱️components/masonry-wall-with-lintel | 100→200 | 3 | 2 | 0 | 0 | 2 | 0 | 1 |  |
| 🧱️components/masonry-wall-with-lintel | 200→500 | 3 | 3 | 1 | 0 | 1 | 1 | 0 | yes |
| 🧱️components/panel-radiator | 50→100 | 12 | 11 | 3 | 0 | 4 | 4 | 1 |  |
| 🧱️components/panel-radiator | 100→200 | 11 | 10 | 1 | 0 | 3 | 6 | 1 |  |
| 🧱️components/sectional-garage-door | 50→100 | 29 | 24 | 1 | 0 | 0 | 23 | 5 |  |
| 🧱️components/sectional-garage-door | 100→200 | 13 | 13 | 7 | 0 | 1 | 5 | 0 | yes |
| 🧱️components/sectional-garage-door | 200→500 | 13 | 7 | 2 | 0 | 4 | 1 | 6 |  |
| 🧱️components/sliding-door | 50→100 | 8 | 6 | 1 | 0 | 2 | 3 | 2 |  |
| 🧱️components/sliding-door | 100→200 | 5 | 5 | 3 | 0 | 1 | 1 | 0 | yes |
| 🧱️components/sliding-door | 200→500 | 4 | 3 | 0 | 0 | 1 | 2 | 1 |  |
| 🧱️components/solid-brick-wall-1960 | 50→100 | 2 | 1 | 1 | 0 | 0 | 0 | 1 |  |
| 🧱️components/solid-brick-wall-1960 | 100→200 | 2 | 2 | 0 | 0 | 2 | 0 | 0 | yes |
| 🧱️components/solid-brick-wall-1960 | 200→500 | 1 | 0 | 0 | 0 | 0 | 0 | 1 |  |
| 🧱️components/spiral-stair | 50→100 | 34 | 34 | 1 | 0 | 0 | 33 | 0 | yes |
| 🧱️components/spiral-stair | 100→200 | 18 | 3 | 0 | 0 | 0 | 3 | 15 |  |
| 🧱️components/spiral-stair | 200→500 | 18 | 18 | 1 | 0 | 1 | 16 | 0 | yes |
| 🧱️components/stair-flight | 50→100 | 5 | 5 | 0 | 0 | 0 | 5 | 0 | yes |
| 🧱️components/stair-flight | 100→200 | 2 | 1 | 0 | 0 | 0 | 1 | 1 |  |
| 🧱️components/stair-flight | 200→500 | 2 | 2 | 0 | 0 | 0 | 2 | 0 | yes |
| 🧱️components/steel-column | 50→100 | 7 | 7 | 0 | 0 | 1 | 6 | 0 | yes |
| 🧱️components/steel-column | 100→200 | 3 | 2 | 0 | 0 | 2 | 0 | 1 |  |
| 🧱️components/steel-column | 200→500 | 3 | 3 | 0 | 0 | 0 | 3 | 0 | yes |
| 🧱️components/tied-canopy | 50→100 | 16 | 16 | 6 | 0 | 7 | 3 | 0 | yes |
| 🧱️components/tied-canopy | 100→200 | 5 | 3 | 1 | 0 | 1 | 1 | 2 |  |
| 🧱️components/tied-canopy | 200→500 | 4 | 2 | 2 | 0 | 0 | 0 | 2 |  |
| 🧱️components/tilt-window | 50→100 | 9 | 9 | 7 | 0 | 0 | 2 | 0 | yes |
| 🧱️components/tilt-window | 100→200 | 5 | 4 | 3 | 0 | 1 | 0 | 1 |  |
| 🧱️components/wheelchair-ramp | 50→100 | 14 | 13 | 7 | 0 | 6 | 0 | 1 |  |
| 🧱️components/wheelchair-ramp | 100→200 | 12 | 12 | 4 | 0 | 2 | 6 | 0 | yes |
| 🧱️components/wheelchair-ramp | 200→500 | 4 | 4 | 1 | 0 | 3 | 0 | 0 | yes |
| 🪑️objects/bicycle-stand | 50→100 | 2 | 2 | 0 | 0 | 1 | 1 | 0 | yes |
| 🪑️objects/bicycle-stand | 100→200 | 2 | 2 | 0 | 0 | 1 | 1 | 0 | yes |
| 🪑️objects/bicycle-stand | 200→500 | 2 | 2 | 0 | 0 | 0 | 2 | 0 | yes |
| 🪑️objects/chalkboard | 50→100 | 8 | 8 | 0 | 0 | 1 | 7 | 0 | yes |
| 🪑️objects/chalkboard | 100→200 | 5 | 4 | 0 | 0 | 0 | 4 | 1 |  |
| 🪑️objects/chalkboard | 200→500 | 3 | 1 | 0 | 0 | 1 | 0 | 2 |  |
| 🪑️objects/charcoal-grill | 50→100 | 41 | 31 | 8 | 0 | 4 | 19 | 10 |  |
| 🪑️objects/charcoal-grill | 100→200 | 25 | 25 | 13 | 0 | 2 | 10 | 0 | yes |
| 🪑️objects/charcoal-grill | 200→500 | 18 | 18 | 14 | 0 | 2 | 2 | 0 | yes |
| 🪑️objects/cooking-pot | 50→100 | 4 | 4 | 0 | 0 | 0 | 4 | 0 | yes |
| 🪑️objects/cooking-pot | 100→200 | 4 | 4 | 0 | 0 | 0 | 4 | 0 | yes |
| 🪑️objects/drinking-fountain | 50→100 | 6 | 6 | 1 | 0 | 0 | 5 | 0 | yes |
| 🪑️objects/drinking-fountain | 100→200 | 6 | 6 | 0 | 0 | 0 | 6 | 0 | yes |
| 🪑️objects/drinking-fountain | 200→500 | 5 | 5 | 0 | 0 | 0 | 5 | 0 | yes |
| 🪑️objects/market-parasol | 50→100 | 17 | 17 | 4 | 0 | 10 | 3 | 0 | yes |
| 🪑️objects/market-parasol | 100→200 | 14 | 14 | 0 | 0 | 11 | 3 | 0 | yes |
| 🪑️objects/market-parasol | 200→500 | 5 | 5 | 1 | 0 | 1 | 3 | 0 | yes |
| 🪑️objects/patio-heater | 50→100 | 8 | 8 | 1 | 1 | 1 | 5 | 0 | yes |
| 🪑️objects/patio-heater | 100→200 | 6 | 6 | 0 | 1 | 2 | 3 | 0 | yes |
| 🪑️objects/patio-heater | 200→500 | 6 | 5 | 0 | 0 | 2 | 3 | 1 |  |
| 🪑️objects/playground-swing | 50→100 | 14 | 14 | 13 | 0 | 0 | 1 | 0 | yes |
| 🪑️objects/playground-swing | 100→200 | 14 | 14 | 5 | 0 | 2 | 7 | 0 | yes |
| 🪑️objects/playground-swing | 200→500 | 14 | 14 | 6 | 0 | 0 | 8 | 0 | yes |
| 🪑️objects/retractable-bollard | 50→100 | 12 | 12 | 7 | 0 | 3 | 2 | 0 | yes |
| 🪑️objects/retractable-bollard | 100→200 | 9 | 8 | 1 | 0 | 3 | 4 | 1 |  |
| 🪑️objects/retractable-bollard | 200→500 | 6 | 5 | 3 | 0 | 2 | 0 | 1 |  |
| 🪑️objects/shopping-trolley | 50→100 | 71 | 71 | 23 | 0 | 16 | 32 | 0 | yes |
| 🪑️objects/shopping-trolley | 100→200 | 36 | 36 | 7 | 0 | 20 | 9 | 0 | yes |
| 🪑️objects/shopping-trolley | 200→500 | 27 | 27 | 6 | 0 | 10 | 11 | 0 | yes |
| 🪑️objects/stepladder | 50→100 | 21 | 21 | 4 | 0 | 2 | 15 | 0 | yes |
| 🪑️objects/stepladder | 100→200 | 15 | 13 | 0 | 0 | 0 | 13 | 2 |  |
| 🪑️objects/stepladder | 200→500 | 13 | 13 | 0 | 0 | 2 | 11 | 0 | yes |
| 🪑️objects/storage-crate-stack | 50→100 | 15 | 15 | 0 | 0 | 0 | 15 | 0 | yes |
| 🪑️objects/storage-crate-stack | 100→200 | 10 | 10 | 0 | 0 | 5 | 5 | 0 | yes |
| 🪑️objects/storage-crate-stack | 200→500 | 5 | 0 | 0 | 0 | 0 | 0 | 5 |  |
| 🪑️objects/street-lamp | 50→100 | 8 | 8 | 2 | 0 | 1 | 5 | 0 | yes |
| 🪑️objects/street-lamp | 100→200 | 6 | 6 | 1 | 0 | 1 | 4 | 0 | yes |
| 🪑️objects/street-lamp | 200→500 | 4 | 3 | 1 | 0 | 0 | 2 | 1 |  |
| 🪑️objects/tealight | 50→100 | 4 | 3 | 1 | 0 | 0 | 2 | 1 |  |
| 🪑️objects/tealight | 100→200 | 4 | 3 | 0 | 0 | 0 | 3 | 1 |  |
| 🪑️objects/waste-bin | 50→100 | 13 | 12 | 5 | 0 | 4 | 3 | 1 |  |
| 🪑️objects/waste-bin | 100→200 | 6 | 5 | 3 | 0 | 0 | 2 | 1 |  |
| 🪑️objects/waste-bin | 200→500 | 3 | 2 | 0 | 0 | 1 | 1 | 1 |  |

### 3b. Coarse scales that are pure copies of the next finer scale

Every coarse body id exists at the finer scale and every solid is identical, scaled or near (thickness or radius numbers only).

- 🔌️appliances/laptop 50→100: 2 solids, 0 identical, 0 scaled, 2 near
- 🛋️furniture/folding-deck-chair 200→500: 5 solids, 3 identical, 0 scaled, 2 near
- 🛋️furniture/height-adjustable-desk 200→500: 14 solids, 13 identical, 0 scaled, 1 near
- 🧱️components/etics-brick-wall-12 100→200: 2 solids, 1 identical, 0 scaled, 1 near
- 🧱️components/etics-brick-wall-30 100→200: 2 solids, 1 identical, 0 scaled, 1 near
- 🧱️components/external-venetian-blind 200→500: 12 solids, 3 identical, 0 scaled, 9 near
- 🧱️components/glulam-beam 100→200: 3 solids, 1 identical, 0 scaled, 2 near
- 🧱️components/glulam-beam 200→500: 1 solids, 0 identical, 0 scaled, 1 near
- 🧱️components/solid-brick-wall-1960 100→200: 2 solids, 0 identical, 0 scaled, 2 near
- 🧱️components/wheelchair-ramp 200→500: 4 solids, 1 identical, 0 scaled, 3 near
- Total: 10 of 380 adjacent pairs.

### 3a. Top 25 most likely not modelled anew

Ranking: share of coarse-scale body solids (1:100 to 1:500) that are identical, scaled or near copies of the same-id solid at the next finer scale; ties by subset pairs.

| rank | asset | copy share | subset pairs | pairs (fine→coarse: identical+scaled+near / coarse body) |
|---|---|---|---|---|
| 1 | 🏗️machinery/wheel-loader | 94 % | 2 | 50→100: 27+0+22 / 50; 100→200: 15+0+14 / 31; 200→500: 12+0+13 / 28 |
| 2 | 🧱️components/glulam-beam | 86 % | 2 | 50→100: 0+0+2 / 3; 100→200: 1+0+2 / 3; 200→500: 0+0+1 / 1 |
| 3 | 🏗️machinery/oil-pumpjack | 84 % | 1 | 50→100: 22+0+14 / 44; 100→200: 18+0+14 / 38; 200→500: 11+0+18 / 33 |
| 4 | 🏗️machinery/diesel-generator | 84 % | 2 | 50→100: 17+0+6 / 27; 100→200: 14+0+4 / 22; 200→500: 1+0+9 / 12 |
| 5 | 🏗️machinery/excavator | 83 % | 1 | 50→100: 24+0+1 / 31; 100→200: 16+0+2 / 22; 200→500: 15+0+1 / 18 |
| 6 | 🛋️furniture/cafe-chair | 82 % | 2 | 50→100: 2+0+8 / 12; 100→200: 8+0+0 / 11; 200→500: 8+0+1 / 10 |
| 7 | 🧱️components/tilt-window | 79 % | 1 | 50→100: 7+0+0 / 9; 100→200: 3+0+1 / 5 |
| 8 | 🏗️machinery/tower-crane | 77 % | 0 | 50→100: 52+0+16 / 111; 100→200: 55+0+40 / 102; 200→500: 28+0+19 / 60 |
| 9 | 🧱️components/wheelchair-ramp | 77 % | 2 | 50→100: 7+0+6 / 14; 100→200: 4+0+2 / 12; 200→500: 1+0+3 / 4 |
| 10 | 🚜️vehicles/tram | 76 % | 1 | 50→100: 57+0+8 / 77; 100→200: 33+0+6 / 59; 200→500: 27+0+12 / 53 |
| 11 | 🪑️objects/market-parasol | 75 % | 3 | 50→100: 4+0+10 / 17; 100→200: 0+0+11 / 14; 200→500: 1+0+1 / 5 |
| 12 | 🏗️machinery/heat-pump-outdoor-unit | 72 % | 0 | 50→100: 7+0+11 / 22; 100→200: 4+0+2 / 13; 200→500: 2+0+8 / 12 |
| 13 | 🧱️components/masonry-wall-with-lintel | 71 % | 1 | 50→100: 4+0+2 / 8; 100→200: 0+0+2 / 3; 200→500: 1+0+1 / 3 |
| 14 | 🪑️objects/retractable-bollard | 70 % | 1 | 50→100: 7+0+3 / 12; 100→200: 1+0+3 / 9; 200→500: 3+0+2 / 6 |
| 15 | 🔌️appliances/vacuum-cleaner | 68 % | 2 | 50→100: 13+0+1 / 25; 100→200: 1+0+11 / 16; 200→500: 1+0+12 / 16 |
| 16 | 🧱️components/tied-canopy | 68 % | 1 | 50→100: 6+0+7 / 16; 100→200: 1+0+1 / 5; 200→500: 2+0+0 / 4 |
| 17 | 🧱️components/glass-balustrade | 67 % | 2 | 50→100: 0+0+2 / 3; 100→200: 0+0+2 / 3 |
| 18 | 🧱️components/etics-brick-wall-12 | 67 % | 1 | 50→100: 2+0+0 / 3; 100→200: 1+0+1 / 2; 200→500: 0+0+0 / 1 |
| 19 | 🧱️components/etics-brick-wall-30 | 67 % | 1 | 50→100: 2+0+0 / 3; 100→200: 1+0+1 / 2; 200→500: 0+0+0 / 1 |
| 20 | 🔌️appliances/server-rack | 64 % | 0 | 50→100: 6+0+3 / 15; 100→200: 0+0+9 / 12; 200→500: 0+0+2 / 4 |
| 21 | 🔌️appliances/hot-water-cylinder | 64 % | 1 | 50→100: 6+0+3 / 11; 100→200: 1+0+2 / 7; 200→500: 2+0+2 / 7 |
| 22 | 🏗️machinery/forklift | 64 % | 0 | 50→100: 20+0+1 / 33; 100→200: 12+0+4 / 28; 200→500: 7+0+9 / 22 |
| 23 | 🏗️machinery/scissor-lift | 62 % | 2 | 50→100: 17+0+14 / 53; 100→200: 13+0+13 / 39 |
| 24 | 🪑️objects/playground-swing | 62 % | 3 | 50→100: 13+0+0 / 14; 100→200: 5+0+2 / 14; 200→500: 6+0+0 / 14 |
| 25 | 🧱️components/external-venetian-blind | 62 % | 2 | 50→100: 1+0+17 / 24; 100→200: 7+0+0 / 24; 200→500: 3+0+9 / 12 |

Assets with zero copies at every adjacent pair: 8 of 143.

## 4. Decisions quality

| scale | entries | mean en chars | mean de chars | de missing | de same as en | de English-like | placeholder hits |
|---|---|---|---|---|---|---|---|
| 1:50 | 424 | 551 | 594 | 0 | 0 | 0 | 3 |
| 1:100 | 390 | 399 | 436 | 0 | 0 | 0 | 0 |
| 1:200 | 368 | 370 | 404 | 0 | 0 | 0 | 0 |
| 1:500 | 276 | 410 | 446 | 0 | 0 | 0 | 0 |

Template clusters: decision texts normalised (digits to `#`, the asset's own slug words to `<self>`, lower case), linked when word-3-gram Jaccard ≥ 0.5 and difflib ratio ≥ 0.9 across different assets (single linkage). Every cluster of 2+ assets is listed. `exact` = identical after normalisation.

Clusters: 4; entries in them: 26; assets touched: 8.

- C01: 6 assets, 20 entries, scales 100,200,50,500, near. Members: 👤️humans/elderly-with-cane@100, 👤️humans/elderly-with-cane@200, 👤️humans/elderly-with-cane@50, 👤️humans/elderly-with-cane@500, 👤️humans/jogger@100, 👤️humans/jogger@200, 👤️humans/jogger@50, 👤️humans/kneeling-gardener@100, 👤️humans/kneeling-gardener@50, 👤️humans/leaning-person@100, 👤️humans/leaning-person@200, 👤️humans/leaning-person@50, 👤️humans/person-with-umbrella@100, 👤️humans/person-with-umbrella@200, 👤️humans/person-with-umbrella@50, 👤️humans/person-with-umbrella@500, 👤️humans/skateboarder@100, 👤️humans/skateboarder@200, 👤️humans/skateboarder@50, 👤️humans/skateboarder@500. Sample: “Figure kit adoption (1:50): the human anatomy (skin lofts, skin under garments, ears, nose, eyes, thumbs, finger segments, joint caps over hip, knee, ankle, shoulder, elbow and wrist) and the human st”
- C02: 2 assets, 2 entries, scales 50, near. Members: 👤️humans/person-cooking@50, 👤️humans/standing-phone-user@50. Sample: “Thermal at 1:50: 124 solids over 10 regions: bare hands (21, 838 cm2), socks and sneakers (20, 0.1 m2), face and neck skin (7, 0.12 m2), short hair (1, 502 cm2), t-shirt and fleece sweatshirt on trunk”
- C03: 2 assets, 2 entries, scales 100, near. Members: 👤️humans/person-cooking@100, 👤️humans/standing-phone-user@100. Sample: “Thermal at 1:100: 56 solids over 10 regions: bare hands (4, 838 cm2), socks and sneakers (8, 0.1 m2), face and neck skin (6, 0.12 m2), short hair (1, 502 cm2), t-shirt and fleece sweatshirt on trunk a”
- C04: 2 assets, 2 entries, scales 200, near. Members: 👤️humans/person-cooking@200, 👤️humans/standing-phone-user@200. Sample: “Thermal at 1:200: 44 solids over 10 regions: bare hands (2, 838 cm2), face and neck skin (5, 0.12 m2), short hair (1, 502 cm2), t-shirt and fleece sweatshirt on trunk and arms (15, 0.93 m2), jeans ove”

## 5. Organic subjects: box and extrude shares at 1:50

Box share (count) above 20 % and extrude share above 40 % in humans, animals and vegetation.

- 🌳️vegetation/ivy-on-trellis: box 50 % (16/32), extrude 0 % (0/32) BOX

## 6. Thermal coverage


## 7. Structure sanity

- 🏗️machinery/scissor-lift 1:50: supports declared, no `part: support` solid
- 🏗️machinery/scissor-lift 1:100: supports declared, no `part: support` solid
- 🏗️machinery/scissor-lift 1:200: supports declared, no `part: support` solid
- 🔌️appliances/ceiling-diffuser 1:50: fewer than 2 member solids (1)
- 🔌️appliances/ceiling-diffuser 1:100: fewer than 2 member solids (1)
- 🔌️appliances/ceiling-diffuser 1:200: fewer than 2 member solids (1)
- 🧱️components/etics-brick-wall-12 1:100: fewer than 2 member solids (1)
- 🧱️components/etics-brick-wall-12 1:200: fewer than 2 member solids (1)
- 🧱️components/etics-brick-wall-12 1:500: fewer than 2 member solids (1)
- 🧱️components/etics-brick-wall-30 1:100: fewer than 2 member solids (1)
- 🧱️components/etics-brick-wall-30 1:200: fewer than 2 member solids (1)
- 🧱️components/etics-brick-wall-30 1:500: fewer than 2 member solids (1)
- 🧱️components/glulam-beam 1:50: fewer than 2 member solids (1)
- 🧱️components/glulam-beam 1:100: fewer than 2 member solids (1)
- 🧱️components/glulam-beam 1:200: fewer than 2 member solids (1)
- 🧱️components/glulam-beam 1:500: fewer than 2 member solids (1)
- 🧱️components/panel-radiator 1:200: fewer than 2 member solids (1)
- 🧱️components/solid-brick-wall-1960 1:100: fewer than 2 member solids (1)
- 🧱️components/solid-brick-wall-1960 1:200: fewer than 2 member solids (1)
- 🧱️components/solid-brick-wall-1960 1:500: fewer than 2 member solids (1)
- 🧱️components/steel-column 1:200: fewer than 2 member solids (1)
- 🧱️components/steel-column 1:500: fewer than 2 member solids (1)

## 8. Per category

| category | spliced / total | mean copy share | assets with copies | template entries | placeholder hits |
|---|---|---|---|---|---|
| 🌳️vegetation | 8 / 16 | 14 % | 7 | 0 | 0 |
| 🏗️machinery | 16 / 16 | 60 % | 16 | 0 | 0 |
| 🐕️animals | 14 / 14 | 9 % | 11 | 0 | 0 |
| 👤️humans | 28 / 28 | 21 % | 28 | 26 | 3 |
| 🔌️appliances | 24 / 24 | 34 % | 22 | 0 | 0 |
| 🚜️vehicles | 8 / 9 | 57 % | 8 | 0 | 0 |
| 🛋️furniture | 12 / 12 | 42 % | 12 | 0 | 0 |
| 🧱️components | 18 / 18 | 51 % | 17 | 0 | 0 |
| 🪑️objects | 15 / 15 | 37 % | 14 | 0 | 0 |

CATEGORY-LINES

## 9. What the script cannot judge

- Whether a copy at a coarser scale is wrong: identical solids can be right where the 2D drawing is identical; the script only sees geometry parameters.
- Silhouette, IoU, Hausdorff, bounds, volume, area and watertightness: needs the kernel tessellation and the 2D shapes (`volumes-check.ts`), not run here.
- Whether a decision is true for the object, whether it names what was merged, or whether a German text reads well. Only presence, length, English-stopword heuristics and text similarity are measured.
- Physical plausibility of supports, load paths, sectionFaces, areaM2 and area shares (C1, C16, C28).
- Box and extrude shares are counts; a few large boxes can dominate volume while many small boxes do not.
- Solid overlap, burial and whether thermal solids lie inside the body; these need geometry, not parameters.

