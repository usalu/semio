# 🔎️ Catalog Scan After the Shared-Code Repairs

One process of `🧪️tests/🔭️multiview-coverage/🔎️scan.ts --all`, 13:04 to 14:39 (cancelled once at 13:19 and resumed at 13:23), 5505 s for 375 assets (14.7 s per asset). Machine-readable: `🧰️matrix/scan-after-shared.json` (the compact rows: summary and the findings grouped per code, views, mode, bone and anchor with poses, scales, count and worst case). This table: `🧰️matrix/scan-table.py`.

The scan checks every pose of every authored scale with the geometry checks of the matrix and the default pose with the derived-view, paper, availability and cue checks. Clip instants, transitions, range ends and symbols are left to the matrix, so the counts are lower bounds for it. The catalog is a live worktree: each row is a snapshot of the asset at the moment it was scanned.

## Totals

- Assets: 375; with a hard finding: **206**; clean: **169**.
- Hard findings: 11660; reported group drift: 10500 in 207 assets.

| Family | Assets | Findings |
|---|---|---|
| sync | 151 | 8701 |
| anchor | 94 | 2852 |
| band-copy | 15 | 76 |
| in-band | 17 | 28 |
| cue | 0 | 0 |
| paper | 2 | 3 |
| dimension | 0 | 0 |
| view | 0 | 0 |
| error | 0 | 0 |

- Synchrony findings at rest 305, in other poses 8396; assets wrong already at rest: 61.
- Anchor findings by place: on a shape of another bone 2144, off every drawn shape 708; by anchor kind: attachment 108, contact 1091, emitter 1653.
- By code: `group-drift-forward` 4962, `sync-bone-forward` 4530, `group-drift-height` 3989, `anchor-landmark` 2852, `sync-bone-height` 2835, `group-drift-lateral` 1549, `sync-bone-lateral` 712, `sync-forward` 361, `sync-height` 243, `derived-view-band-copy` 76, `derived-view-in-band` 28, `sync-lateral` 20, `paper-height` 2, `paper-width` 1.

| Category | Assets | With a hard finding | Clean | sync | anchor | band copy | in-band | cue | paper |
|---|---|---|---|---|---|---|---|---|---|
| animals | 14 | 13 | 1 | 13 | 10 | 0 | 0 | 0 | 0 |
| appliances | 39 | 13 | 26 | 6 | 10 | 0 | 2 | 0 | 0 |
| components | 58 | 17 | 41 | 8 | 4 | 5 | 5 | 0 | 0 |
| furniture | 39 | 13 | 26 | 9 | 4 | 4 | 0 | 0 | 0 |
| humans | 75 | 71 | 4 | 71 | 26 | 0 | 0 | 0 | 1 |
| machinery | 16 | 7 | 9 | 2 | 3 | 3 | 0 | 0 | 0 |
| objects | 58 | 24 | 34 | 13 | 8 | 2 | 5 | 0 | 1 |
| vegetation | 67 | 42 | 25 | 27 | 24 | 1 | 5 | 0 | 0 |
| vehicles | 9 | 6 | 3 | 2 | 5 | 0 | 0 | 0 | 0 |

## Assets with a hard finding

Sync: per-bone and extent findings at rest / in other poses, and the extent findings among them. Anchors: findings by the place the anchor lies instead. Band copy, in-band and paper: scale and view. Cue: scale and mode. Group drift is reported only.

| Asset | Category | Sync (rest / posed) | Anchors | Band copy | In-band | Cue | Paper | Other hard codes | Group drift (reported) |
|---|---|---|---|---|---|---|---|---|---|
| dairy-cow | animals | 2 / 284; 11 extent | other bone 48; off 8 |  |  |  |  |  | 113 |
| domestic-cat | animals | 7 / 316 |  |  |  |  |  |  | 160 |
| goat | animals | 2 / 96; 5 extent | other bone 8; off 9 |  |  |  |  |  | 327 |
| hedgehog | animals | 0 / 41; 2 extent | other bone 4; off 2 |  |  |  |  |  | 13 |
| herring-gull | animals | 26 / 178; 4 extent |  |  |  |  |  |  | 80 |
| horse | animals | 0 / 135 | other bone 2; off 2 |  |  |  |  |  | 142 |
| labrador-dog | animals | 1 / 559; 35 extent | other bone 6; off 3 |  |  |  |  |  | 55 |
| mallard-duck | animals | 0 / 72; 15 extent | off 6 |  |  |  |  |  | 158 |
| pig | animals | 14 / 310; 10 extent | other bone 12; off 6 |  |  |  |  |  | 5 |
| pigeon | animals | 3 / 161; 9 extent |  |  |  |  |  |  | 89 |
| red-fox | animals | 30 / 674; 12 extent | other bone 88; off 6 |  |  |  |  |  | 15 |
| roe-deer | animals | 8 / 970; 42 extent | other bone 34 |  |  |  |  |  | 69 |
| sheep | animals | 10 / 326; 7 extent | off 8 |  |  |  |  |  | 3 |
| ceiling-diffuser | appliances |  |  |  | 200 front, 100 front, 50 front |  |  |  |  |
| electric-fan-heater | appliances |  | off 18 |  |  |  |  |  |  |
| gas-hob | appliances |  | other bone 3 |  |  |  |  |  | 6 |
| pendant-lamp | appliances |  |  |  | 200 front, 100 front, 50 front |  |  |  |  |
| used-dishwasher | appliances | 0 / 8; 3 extent |  |  |  |  |  |  | 24 |
| used-electric-cooker | appliances | 0 / 3; 3 extent | off 2 |  |  |  |  |  | 8 |
| used-electric-guitar-and-combo | appliances |  | other bone 12; off 4 |  |  |  |  |  |  |
| used-electronic-drum-kit | appliances |  | other bone 20 |  |  |  |  |  |  |
| used-electronic-organ | appliances |  | other bone 3; off 21 |  |  |  |  |  |  |
| used-fridge-freezer | appliances | 0 / 6; 3 extent | other bone 47; off 23 |  |  |  |  |  | 60 |
| used-microwave-oven | appliances | 0 / 2 | off 1 |  |  |  |  |  | 7 |
| used-tumble-dryer | appliances | 0 / 4; 3 extent | other bone 6; off 6 |  |  |  |  |  | 23 |
| used-washing-machine | appliances | 0 / 14; 12 extent | other bone 6; off 22 |  |  |  |  |  | 14 |
| etics-brick-wall-12 | components |  |  | 500 front, 200 front, 100 front, 50 front | 500 plan |  |  |  |  |
| etics-brick-wall-30 | components |  |  | 500 front, 200 front, 100 front, 50 front | 500 plan |  |  |  |  |
| glulam-beam | components |  |  |  | 500 plan |  |  |  |  |
| reclaimed-cast-iron-column | components | 0 / 4; 4 extent |  |  |  |  |  |  |  |
| reclaimed-precast-stair | components | 0 / 12; 8 extent | other bone 12; off 20 |  |  |  |  |  | 12 |
| reclaimed-trapezoidal-sheet-stack | components | 0 / 8; 8 extent |  |  |  |  |  |  | 8 |
| sectional-garage-door | components |  | other bone 5; off 6 |  |  |  |  |  | 11 |
| sliding-door | components |  | other bone 15 |  |  |  |  |  |  |
| solid-brick-wall-1960 | components |  |  | 500 front, 500 plan, 200 front, 200 plan, 100 front, 50 front | 500 plan |  |  |  |  |
| spiral-stair | components |  |  | 200 front, 100 front |  |  |  |  | 32 |
| stair-flight | components |  |  | 200 front, 100 front |  |  |  |  |  |
| steel-column | components |  |  |  | 500 front, 200 front, 100 front |  |  |  |  |
| used-air-water-heat-pump-monobloc | components | 0 / 3; 3 extent | other bone 2; off 8 |  |  |  |  |  |  |
| used-forced-draught-oil-burner | components | 0 / 1 |  |  |  |  |  |  | 10 |
| used-pellet-boiler | components | 0 / 104; 8 extent |  |  |  |  |  |  |  |
| used-towel-radiator | components | 0 / 9; 9 extent |  |  |  |  |  |  | 15 |
| used-wall-hung-gas-boiler | components | 0 / 15; 15 extent |  |  |  |  |  |  | 119 |
| cafe-chair | furniture | 0 / 2; 1 extent |  |  |  |  |  |  | 17 |
| office-swivel-chair | furniture | 0 / 4; 4 extent |  |  |  |  |  |  | 47 |
| used-bedside-table | furniture | 0 / 6; 5 extent | off 12 |  |  |  |  |  | 13 |
| used-coffee-table | furniture |  |  | 500 plan, 200 plan |  |  |  |  |  |
| used-folding-bookcase-ladder | furniture |  |  | 500 front, 200 front |  |  |  |  | 3 |
| used-high-chair | furniture | 0 / 17; 3 extent |  |  |  |  |  |  | 14 |
| used-hinged-wardrobe | furniture | 0 / 7 | other bone 3; off 3 |  |  |  |  |  | 3 |
| used-kitchen-base-cabinet | furniture | 0 / 1; 1 extent |  |  |  |  |  |  | 12 |
| used-park-armchair | furniture | 0 / 12; 4 extent | other bone 4; off 4 |  |  |  |  |  | 14 |
| used-recliner-chair | furniture | 0 / 6; 4 extent | off 8 |  |  |  |  |  | 18 |
| used-shoe-cabinet | furniture |  |  | 500 plan, 200 plan |  |  |  |  | 28 |
| used-wall-unit | furniture |  |  | 500 plan, 200 plan |  |  |  |  | 40 |
| used-wingback-armchair | furniture | 0 / 4; 4 extent |  |  |  |  |  |  | 6 |
| barista | humans | 5 / 8; 2 extent |  |  |  |  |  |  | 19 |
| basketball-player | humans | 0 / 10; 2 extent |  |  |  |  |  |  | 61 |
| child-on-kick-scooter | humans | 0 / 12; 12 extent |  |  |  |  |  |  | 1729 |
| construction-worker-with-shovel | humans | 15 / 36 |  |  |  |  |  |  | 45 |
| courier-with-hand-truck | humans | 0 / 10 | off 16 |  |  |  |  |  | 18 |
| cyclist | humans | 3 / 23; 1 extent | other bone 48 |  |  |  |  |  | 10 |
| dancer | humans | 0 / 17; 3 extent | other bone 4 |  |  |  |  |  | 95 |
| doctor-with-clipboard | humans | 2 / 20 |  |  |  |  |  |  | 12 |
| e-scooter-rider | humans | 5 / 60 |  |  |  |  |  |  | 11 |
| elderly-person-knitting | humans | 1 / 34 |  |  |  |  |  |  | 38 |
| elderly-with-cane | humans | 3 / 135 | off 2 |  |  |  |  |  | 24 |
| hiker-with-trekking-poles | humans | 0 / 7 |  |  |  |  |  |  | 8 |
| jogger | humans | 0 / 8 |  |  |  |  |  |  | 263 |
| kneeling-gardener | humans | 2 / 6 |  |  |  |  |  |  | 9 |
| leaning-person | humans | 0 / 30; 6 extent | other bone 20; off 4 |  |  |  |  |  | 24 |
| mobility-scooter-user | humans | 1 / 1 | off 32 |  |  |  |  |  | 22 |
| nurse-walking | humans | 2 / 278; 37 extent | other bone 1; off 1 |  |  |  |  |  | 52 |
| office-worker-carrying-laptop | humans | 2 / 196; 14 extent | other bone 2; off 4 |  |  |  |  |  | 63 |
| painter-at-easel | humans | 2 / 4 |  |  |  |  |  |  | 14 |
| parent-carrying-toddler | humans | 0 / 57 | off 20 |  |  |  |  |  | 13 |
| parent-with-child-on-shoulders | humans | 2 / 80; 6 extent | off 8 |  |  |  |  |  | 40 |
| parent-with-stroller | humans | 6 / 85; 2 extent | off 4 |  |  |  |  |  | 76 |
| patient-with-iv-pole | humans | 2 / 168; 6 extent | other bone 2; off 4 |  |  |  |  |  | 70 |
| person-carrying-ladder | humans | 5 / 79; 1 extent |  |  |  |  |  |  | 115 |
| person-carrying-shopping-bags | humans | 0 / 132; 4 extent |  |  |  |  |  |  | 65 |
| person-checking-watch | humans | 0 / 6 |  |  |  |  |  |  | 8 |
| person-cooking | humans | 0 / 3; 1 extent |  |  |  |  |  |  | 13 |
| person-drinking-coffee | humans | 8 / 38; 3 extent |  |  |  |  |  |  | 37 |
| person-eating-standing | humans | 0 / 11; 3 extent |  |  |  |  |  |  | 17 |
| person-hanging-laundry | humans | 2 / 37 |  |  |  |  |  |  | 13 |
| person-in-rain-poncho | humans | 0 / 86; 18 extent | other bone 2; off 4 |  |  |  |  |  | 98 |
| person-in-winter-coat | humans | 2 / 36; 4 extent |  |  |  |  |  |  | 142 |
| person-on-crutches | humans | 0 / 15; 3 extent | off 8 |  |  |  |  |  | 32 |
| person-pushing-bicycle | humans | 4 / 55 | other bone 76 |  |  |  |  |  | 12 |
| person-queuing | humans | 2 / 8 |  |  |  |  |  |  | 41 |
| person-signing | humans | 5 / 58 |  |  |  |  |  |  | 69 |
| person-stretching | humans | 4 / 36; 7 extent | off 4 |  |  |  |  |  | 67 |
| person-sweeping | humans | 9 / 39; 1 extent |  |  |  |  |  |  | 55 |
| person-taking-selfie | humans | 3 / 7; 1 extent |  |  |  |  | 200 front |  | 26 |
| person-with-arm-sling | humans | 0 / 166; 7 extent |  |  |  |  |  |  | 28 |
| person-with-guide-dog | humans | 3 / 61 |  |  |  |  |  |  | 11 |
| person-with-rollator | humans | 0 / 31 |  |  |  |  |  |  | 53 |
| person-with-umbrella | humans | 0 / 3; 1 extent |  |  |  |  |  |  | 9 |
| person-with-white-cane | humans | 3 / 19; 2 extent |  |  |  |  |  |  | 2 |
| photographer-with-camera | humans | 0 / 30; 1 extent |  |  |  |  |  |  | 12 |
| picnicker-sitting-on-ground | humans | 12 / 22 | other bone 18 |  |  |  |  |  | 11 |
| postal-carrier | humans | 5 / 222; 26 extent | other bone 2; off 4 |  |  |  |  |  | 149 |
| pregnant-person | humans | 0 / 14 |  |  |  |  |  |  |  |
| presenter-at-flipchart | humans | 2 / 12 |  |  |  |  |  |  | 5 |
| runner-with-blade-prosthesis | humans | 2 / 163; 2 extent |  |  |  |  |  |  | 104 |
| schoolchild-with-backpack | humans | 0 / 3 |  |  |  |  |  |  | 29 |
| seated-guitarist | humans | 0 / 5 | other bone 6; off 40 |  |  |  |  |  | 14 |
| seated-office-worker | humans | 2 / 23 |  |  |  |  |  |  | 45 |
| seated-reader | humans | 4 / 46 |  |  |  |  |  |  | 22 |
| skateboarder | humans | 0 / 97; 1 extent | other bone 128; off 2 |  |  |  |  |  | 63 |
| sleeping-person | humans | 1 / 30 | other bone 27 |  |  |  |  |  |  |
| standing-desk-worker | humans | 3 / 6 |  |  |  |  |  |  | 15 |
| standing-phone-user | humans | 1 / 8 |  |  |  |  |  |  | 11 |
| standing-violinist | humans | 4 / 12 |  |  |  |  |  |  | 24 |
| surveyor-with-total-station | humans | 4 / 11; 1 extent | other bone 3 |  |  |  |  |  | 32 |
| teenager-with-headphones | humans | 4 / 1; 1 extent |  |  |  |  |  |  | 8 |
| tennis-player | humans | 1 / 9; 1 extent |  |  |  |  |  |  | 56 |
| tour-guide | humans | 0 / 10 |  |  |  |  |  |  | 8 |
| traveller-with-rolling-suitcase | humans | 5 / 232; 24 extent | other bone 7; off 4 |  |  |  |  |  | 108 |
| waiter-with-tray | humans | 4 / 13; 2 extent | off 1 |  |  |  |  |  | 28 |
| walking-toddler | humans | 1 / 8; 3 extent |  |  |  |  |  |  | 13 |
| waving-person | humans | 4 / 16 |  |  |  |  |  |  | 28 |
| wheelchair-user | humans | 0 / 2 | off 34 |  |  |  |  |  | 20 |
| worker-carrying-box | humans | 0 / 217; 3 extent |  |  |  |  |  |  | 107 |
| worker-with-jackhammer | humans | 9 / 23 |  |  |  |  |  |  | 24 |
| worker-with-pallet-jack | humans | 1 / 0 | other bone 2; off 6 |  |  |  |  |  | 15 |
| diesel-generator | machinery | 0 / 2; 2 extent |  |  |  |  |  |  | 5 |
| evaporative-humidifier | machinery |  | off 6 |  |  |  |  |  | 3 |
| forklift | machinery | 0 / 6; 6 extent | other bone 8; off 20 |  |  |  |  |  | 75 |
| heat-pump-outdoor-unit | machinery |  |  | 500 plan, 200 plan |  |  |  |  |  |
| plate-heat-exchanger | machinery |  |  | 200 plan, 100 plan |  |  |  |  |  |
| scroll-compressor | machinery |  | off 6 |  |  |  |  |  |  |
| solar-thermal-collector | machinery |  |  | 100 front, 100 plan, 50 front, 50 plan |  |  |  |  | 18 |
| bicycle-stand | objects |  |  | 200 front, 200 plan, 100 front, 100 plan, 50 front, 50 plan |  |  |  |  |  |
| charcoal-grill | objects | 0 / 3; 3 extent | off 3 |  |  |  |  |  | 7 |
| stepladder | objects | 0 / 24 |  |  |  |  |  |  | 13 |
| storage-crate-stack | objects |  |  | 500 plan, 200 plan, 100 front, 50 front |  |  |  |  |  |
| tealight | objects |  |  |  | 200 front, 100 front, 50 front |  |  |  | 6 |
| used-basketball-hoop | objects | 0 / 11; 2 extent |  |  |  |  |  |  | 18 |
| used-bicycle-shelter | objects |  | other bone 12 |  |  |  |  |  |  |
| used-bollard-light | objects |  |  |  | 500 front, 200 front |  |  |  | 12 |
| used-cello | objects | 0 / 4; 4 extent |  |  |  |  |  |  | 18 |
| used-classical-guitar | objects | 0 / 38; 24 extent | other bone 45 |  |  |  |  |  | 26 |
| used-concert-harp | objects | 0 / 6 | other bone 16 |  |  |  |  |  | 34 |
| used-djembe-and-cajon | objects | 0 / 3; 3 extent |  |  |  |  |  |  | 9 |
| used-double-bass | objects |  | off 16 |  |  |  |  |  |  |
| used-double-swing | objects | 0 / 4 |  |  |  |  |  |  |  |
| used-drum-kit | objects | 0 / 5 | other bone 4 |  |  |  |  |  | 2 |
| used-football-goal | objects | 0 / 2 | other bone 10 |  |  |  |  |  | 39 |
| used-harmonium | objects |  |  |  |  |  | 100 elevation, 50 elevation |  |  |
| used-park-lantern | objects |  |  |  | 500 front |  |  |  |  |
| used-playground-roundabout | objects |  |  |  | 500 front, 200 front |  |  |  |  |
| used-spring-rocker | objects | 0 / 6; 6 extent |  |  |  |  |  |  | 6 |
| used-viola | objects |  | other bone 2 |  |  |  |  |  | 2 |
| used-water-play-table | objects | 0 / 4; 4 extent |  |  |  |  |  |  | 4 |
| used-wc-pan-cistern | objects | 0 / 12; 9 extent |  |  |  |  |  |  | 23 |
| waste-bin | objects |  |  |  | 500 front |  |  |  |  |
| apple-tree | vegetation |  | other bone 7; off 4 |  |  |  |  |  | 108 |
| bamboo-clump | vegetation | 0 / 2; 2 extent | other bone 81; off 22 |  |  |  |  |  | 47 |
| black-alder | vegetation | 0 / 23 | other bone 3 |  |  |  |  |  | 128 |
| black-locust | vegetation | 0 / 4; 4 extent |  |  |  |  |  |  | 128 |
| boxwood-ball | vegetation |  |  |  | 500 front |  |  |  |  |
| common-ash | vegetation |  | other bone 170; off 12 |  |  |  |  |  | 1 |
| common-lilac | vegetation | 2 / 6 |  |  |  |  |  |  |  |
| common-reed | vegetation | 0 / 46; 5 extent | other bone 264; off 25 |  |  |  |  |  | 36 |
| crab-apple | vegetation |  | other bone 1; off 9 |  |  |  |  |  | 17 |
| dawn-redwood | vegetation |  | other bone 16 |  |  |  |  |  |  |
| english-oak | vegetation | 0 / 6; 4 extent | other bone 84 |  |  |  |  |  | 136 |
| european-beech | vegetation |  | other bone 191 |  |  |  |  |  |  |
| field-maple | vegetation | 0 / 4; 3 extent | other bone 11; off 3 |  |  |  |  |  | 16 |
| fig-tree | vegetation |  | other bone 8 |  |  |  |  |  |  |
| hawthorn | vegetation | 0 / 33; 6 extent | other bone 13 |  |  |  |  |  | 287 |
| hornbeam-hedge | vegetation | 0 / 16; 6 extent |  |  |  |  |  |  | 74 |
| horse-chestnut | vegetation |  | other bone 104 |  |  |  |  |  |  |
| hosta-clump | vegetation | 0 / 5; 5 extent |  |  |  |  |  |  | 112 |
| hydrangea | vegetation | 2 / 6 |  |  |  |  |  |  |  |
| ivy-on-trellis | vegetation | 8 / 32; 1 extent | other bone 19 |  |  |  |  |  | 123 |
| japanese-maple | vegetation |  | other bone 10 |  |  |  |  |  |  |
| maize-row | vegetation |  | other bone 108; off 103 |  |  |  |  |  |  |
| miscanthus-grass | vegetation | 0 / 15; 4 extent | other bone 12; off 10 |  |  |  |  |  | 52 |
| multi-stem-birch | vegetation | 0 / 1 |  |  |  |  |  |  | 40 |
| norway-maple | vegetation | 0 / 3; 1 extent | other bone 15 |  |  |  |  |  | 23 |
| norway-spruce | vegetation |  |  |  | 500 front, 200 front |  |  |  |  |
| olive-tree | vegetation |  | other bone 7; off 4 |  |  |  |  |  | 99 |
| pampas-grass | vegetation | 0 / 63; 9 extent | other bone 5; off 1 |  |  |  |  |  | 91 |
| potted-monstera | vegetation | 0 / 4 |  |  |  |  |  |  |  |
| rhododendron | vegetation | 2 / 6 |  |  |  |  |  |  |  |
| rosemary-bush | vegetation | 0 / 4; 2 extent |  |  |  |  |  |  | 4 |
| rowan | vegetation | 0 / 4; 4 extent | other bone 1; off 2 |  | 500 front |  |  |  | 47 |
| saguaro-cactus | vegetation |  |  | 500 plan, 200 plan |  |  |  |  |  |
| saucer-magnolia | vegetation |  |  |  | 500 front |  |  |  | 2 |
| scots-pine | vegetation | 0 / 13 |  |  |  |  |  |  | 3 |
| small-leaved-lime | vegetation | 16 / 80 | other bone 89 |  |  |  |  |  | 244 |
| stone-pine | vegetation | 0 / 22 |  |  |  |  |  |  | 7 |
| sunflower | vegetation | 0 / 4; 4 extent |  |  |  |  |  |  | 28 |
| sycamore-maple | vegetation | 0 / 6 | other bone 21 |  |  |  |  |  | 24 |
| weeping-willow | vegetation | 0 / 8; 8 extent | other bone 10 |  |  |  |  |  | 117 |
| windmill-palm | vegetation |  |  |  | 200 front |  |  |  |  |
| wisteria-pergola | vegetation | 0 / 6 |  |  |  |  |  |  |  |
| cargo-bike | vehicles |  | other bone 24 |  |  |  |  |  | 4 |
| city-bicycle | vehicles |  | other bone 38 |  |  |  |  |  |  |
| city-bus | vehicles |  | off 16 |  |  |  |  |  | 29 |
| city-car | vehicles | 0 / 12; 12 extent |  |  |  |  |  |  | 64 |
| motor-scooter | vehicles |  | other bone 33 |  |  |  |  |  |  |
| vintage-car-1929 | vehicles | 0 / 12; 12 extent | other bone 14; off 36 |  |  |  |  |  | 31 |

## Clean assets

- **animals** (1): red-squirrel
- **appliances** (26): circulation-pump, decentralised-hrv-fan, desktop-monitor, duct-fan, electric-kettle, electricity-meter, gas-boiler, gas-meter, hot-water-cylinder, kitchen-extractor-hood, laptop, mvhr-unit, room-thermostat, server-rack, shower-head, split-ac-indoor-unit, thermostatic-radiator-valve, toaster, tower-server, used-crt-television, used-electric-bass-and-cabinet, used-floor-lamp, used-oil-filled-radiator, used-sewing-machine, used-stage-piano, vacuum-cleaner
- **components** (41): cantilever-balcony, curtain-wall-bay, external-venetian-blind, glass-balustrade, masonry-wall-with-lintel, panel-radiator, reclaimed-brick-pallet, reclaimed-cobblestone-bag, reclaimed-concrete-wall-panel, reclaimed-floorboard-stack, reclaimed-glulam-girder, reclaimed-granite-kerbs, reclaimed-hollow-core-slab, reclaimed-lattice-girder, reclaimed-oak-beam, reclaimed-parquet-crate, reclaimed-paving-slab-pallet, reclaimed-rafter-bundle, reclaimed-roof-tile-pallet, reclaimed-sandstone-window-surround, reclaimed-steel-column, reclaimed-steel-i-beam, reclaimed-steel-stair, reclaimed-stone-steps, reclaimed-timber-roof-truss, tied-canopy, tilt-window, used-brine-water-heat-pump, used-buffer-storage-tank, used-cast-iron-sectional-boiler, used-district-heating-substation, used-dry-rotor-inline-pump, used-expansion-vessels, used-insulated-copper-coils, used-panel-radiator-pallet, used-radiant-ceiling-strip, used-steel-oil-storage-tanks, used-steel-pipe-rack, used-trench-convector, used-underfloor-heating-manifold, wheelchair-ramp
- **furniture** (26): bar-stool, bookshelf, dining-table, folding-deck-chair, height-adjustable-desk, operating-table, park-bench, picnic-table, single-bed, three-seat-sofa, used-baby-cot, used-backrest-park-bench, used-beer-garden-set, used-bentwood-dining-chairs, used-bunk-bed, used-chess-table, used-chest-of-drawers, used-coat-stand, used-concrete-bench-block, used-corner-sofa, used-double-bed, used-extendable-dining-table, used-sun-lounger-bench, used-tree-surround-bench, used-tv-sideboard, used-writing-desk
- **humans** (4): running-child, standing-adult, sunbather-on-towel, walking-commuter
- **machinery** (9): concrete-mixer-truck, excavator, expansion-valve, finned-heat-exchanger, oil-pumpjack, scissor-lift, sorption-wheel, tower-crane, wheel-loader
- **objects** (34): chalkboard, cooking-pot, drinking-fountain, market-parasol, patio-heater, playground-swing, retractable-bollard, shopping-trolley, street-lamp, used-accordion, used-baby-grand-piano, used-bathtub, used-bubbler-stone-fountain, used-climbing-net-pyramid, used-concrete-table-tennis, used-dog-waste-station, used-litter-bin-post, used-marching-snare-and-bass-drum, used-moving-boxes, used-outdoor-fitness-station, used-park-noticeboard, used-pump-drinking-fountain, used-recycling-bin-cluster, used-sandpit-digger, used-seesaw, used-slide-tower, used-steel-string-acoustic-guitar, used-timpani-pair, used-trumpet, used-upright-piano, used-violin, used-washbasin-vanity, used-wayfinding-signpost, used-xylophone
- **vegetation** (25): agave, atlas-cedar, banana-plant, cherry-laurel, date-palm, english-yew, european-larch, fiddle-leaf-fig, ginkgo, grapevine-row, honey-locust, italian-cypress, japanese-cherry, lavender-bush, lombardy-poplar, london-plane-pollarded, prickly-pear, privet-hedge, red-oak, shrub-rose, snake-plant, sweetgum, tree-fern, tulip-tree, yucca
- **vehicles** (3): delivery-van, electric-kick-scooter, tram
