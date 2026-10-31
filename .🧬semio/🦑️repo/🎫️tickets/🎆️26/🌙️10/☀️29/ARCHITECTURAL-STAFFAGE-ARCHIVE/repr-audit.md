# 📐️ Representation Audit — Staffage Catalog

Generated 2026-10-29 by `audit.py` (read-only, stdlib). Source: `/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage`.

Shape counts are `elevation/plan` shapes. `pt` counts coordinates per shape: poly = points, path = endpoints plus control points, capsule = 2, ellipse = 1 centre, rect = 4 corners. `pen` = distinct paper pens (PenSet values plus shape `penMm`). `h` = hatches.
Flag codes: `(a)` identical shape-id sets between two scales; `(b)` coordinates identical or uniformly scaled (k) between two scales; `(c)` smaller drawing scale (larger denominator) has more points than the larger one; `(d)` shape ratio 1:500 / 1:50 above 0.6; `(e)` unavailable entry missing reason/limitingFeature/limitingSizeM/alternatives; `(f)` leftover v1 fields; `dec` decisions not complete in en+de; `vr` validRange not containing its own denominator; `denom` denominator mismatch; `missing` scale not present.

## Per-asset table

| Asset | Category | 1:500 | 1:200 | 1:100 | 1:50 | Flags |
|---|---|---|---|---|---|---|
| dairy-cow | animals | 14/0 sh · 257 pt · 4 pen · 0 h | 40/0 sh · 808 pt · 3 pen · 0 h | 40/0 sh · 808 pt · 3 pen · 0 h | 44/0 sh · 769 pt · 4 pen · 0 h | (a) 200=100 same shape ids; (b) 200~100 identical; (c) 200 has 808 pts > 50 769 pts; (c) 100 has 808 pts > 50 769 pts |
| domestic-cat | animals | UNAVAILABLE | 21/0 sh · 266 pt · 4 pen · 0 h | 38/0 sh · 715 pt · 3 pen · 0 h | 49/0 sh · 745 pt · 4 pen · 0 h | — |
| horse | animals | 20/0 sh · 380 pt · 3 pen · 0 h | 40/0 sh · 770 pt · 3 pen · 0 h | 43/0 sh · 800 pt · 3 pen · 0 h | 57/0 sh · 833 pt · 4 pen · 0 h | — |
| labrador-dog | animals | 17/0 sh · 245 pt · 4 pen · 0 h | 19/0 sh · 457 pt · 3 pen · 0 h | 37/0 sh · 728 pt · 3 pen · 0 h | 46/0 sh · 755 pt · 4 pen · 0 h | — |
| pigeon | animals | UNAVAILABLE | 13/0 sh · 217 pt · 2 pen · 0 h | 25/0 sh · 448 pt · 3 pen · 0 h | 31/0 sh · 412 pt · 4 pen · 0 h | (c) 100 has 448 pts > 50 412 pts |
| roe-deer | animals | 13/0 sh · 235 pt · 3 pen · 0 h | 28/0 sh · 548 pt · 3 pen · 0 h | 30/0 sh · 550 pt · 3 pen · 0 h | 36/0 sh · 566 pt · 4 pen · 0 h | — |
| cantilever-balcony | components | 2/0 sh · 8 pt · 2 pen · 0 h | 5/0 sh · 18 pt · 3 pen · 0 h | 7/0 sh · 44 pt · 4 pen · 2 h | 20/0 sh · 83 pt · 7 pen · 2 h | — |
| glass-balustrade | components | UNAVAILABLE | 3/0 sh · 7 pt · 3 pen · 0 h | 3/0 sh · 9 pt · 4 pen · 0 h | 8/0 sh · 25 pt · 5 pen · 1 h | (a) 200=100 same shape ids |
| glulam-beam | components | 1/0 sh · 4 pt · 2 pen · 0 h | 3/0 sh · 8 pt · 3 pen · 0 h | 3/0 sh · 12 pt · 4 pen · 1 h | 23/0 sh · 70 pt · 5 pen · 0 h | (a) 200=100 same shape ids |
| sliding-door | components | 2/0 sh · 12 pt · 2 pen · 0 h | 7/0 sh · 24 pt · 3 pen · 0 h | 10/0 sh · 36 pt · 4 pen · 0 h | 21/0 sh · 56 pt · 5 pen · 0 h | — |
| stair-flight | components | 1/0 sh · 28 pt · 2 pen · 0 h | 13/0 sh · 28 pt · 3 pen · 0 h | 25/0 sh · 88 pt · 4 pen · 1 h | 39/0 sh · 116 pt · 5 pen · 0 h | — |
| steel-column | components | 1/0 sh · 12 pt · 2 pen · 0 h | 3/0 sh · 12 pt · 3 pen · 0 h | 6/0 sh · 20 pt · 4 pen · 0 h | 16/0 sh · 50 pt · 5 pen · 0 h | — |
| tied-canopy | components | 3/0 sh · 10 pt · 2 pen · 0 h | 5/0 sh · 16 pt · 4 pen · 0 h | 13/0 sh · 95 pt · 4 pen · 1 h | 23/0 sh · 137 pt · 6 pen · 1 h | — |
| tilt-window | components | UNAVAILABLE | 4/0 sh · 14 pt · 4 pen · 0 h | 7/0 sh · 25 pt · 4 pen · 0 h | 13/0 sh · 43 pt · 5 pen · 1 h | — |
| cafe-chair | furniture | 4/0 sh · 10 pt · 2 pen · 0 h | 6/0 sh · 14 pt · 3 pen · 0 h | 8/0 sh · 18 pt · 4 pen · 0 h | 21/0 sh · 69 pt · 7 pen · 0 h | — |
| height-adjustable-desk | furniture | 10/0 sh · 28 pt · 2 pen · 0 h | 15/0 sh · 52 pt · 3 pen · 0 h | 19/0 sh · 70 pt · 5 pen · 0 h | 37/0 sh · 136 pt · 8 pen · 1 h | — |
| office-swivel-chair | furniture | 7/0 sh · 15 pt · 2 pen · 0 h | 10/0 sh · 36 pt · 3 pen · 0 h | 12/0 sh · 65 pt · 4 pen · 0 h | 39/0 sh · 149 pt · 7 pen · 0 h | — |
| park-bench | furniture | 3/0 sh · 12 pt · 2 pen · 0 h | 6/0 sh · 76 pt · 3 pen · 0 h | 26/0 sh · 76 pt · 4 pen · 0 h | 53/0 sh · 144 pt · 6 pen · 1 h | — |
| cyclist | humans | 19/0 sh · 379 pt · 1 pen · 0 h | 41/0 sh · 680 pt · 3 pen · 0 h | 58/0 sh · 940 pt · 13 pen · 2 h | 108/0 sh · 694 pt · 4 pen · 1 h | (c) 100 has 940 pts > 50 694 pts |
| elderly-with-cane | humans | 14/0 sh · 279 pt · 1 pen · 0 h | 21/0 sh · 522 pt · 2 pen · 0 h | 31/0 sh · 655 pt · 3 pen · 2 h | 46/0 sh · 423 pt · 3 pen · 1 h | (c) 200 has 522 pts > 50 423 pts; (c) 100 has 655 pts > 50 423 pts |
| jogger | humans | 13/0 sh · 453 pt · 2 pen · 0 h | 22/0 sh · 531 pt · 3 pen · 0 h | 28/0 sh · 597 pt · 3 pen · 0 h | 55/0 sh · 639 pt · 4 pen · 0 h | — |
| leaning-person | humans | 14/0 sh · 455 pt · 2 pen · 0 h | 22/0 sh · 458 pt · 3 pen · 0 h | 29/0 sh · 518 pt · 3 pen · 0 h | 51/0 sh · 559 pt · 4 pen · 0 h | — |
| parent-with-stroller | humans | 19/0 sh · 462 pt · 1 pen · 0 h | 35/0 sh · 764 pt · 4 pen · 0 h | 49/0 sh · 929 pt · 10 pen · 3 h | 69/0 sh · 677 pt · 3 pen · 0 h | (c) 200 has 764 pts > 50 677 pts; (c) 100 has 929 pts > 50 677 pts |
| person-with-umbrella | humans | 16/0 sh · 499 pt · 2 pen · 0 h | 30/0 sh · 549 pt · 3 pen · 0 h | 40/0 sh · 604 pt · 3 pen · 0 h | 63/0 sh · 643 pt · 4 pen · 1 h | — |
| running-child | humans | 9/0 sh · 116 pt · 3 pen · 0 h | 29/0 sh · 646 pt · 3 pen · 0 h | 38/0 sh · 775 pt · 3 pen · 0 h | 47/0 sh · 456 pt · 4 pen · 0 h | (c) 200 has 646 pts > 50 456 pts; (c) 100 has 775 pts > 50 456 pts |
| seated-reader | humans | 11/0 sh · 305 pt · 2 pen · 0 h | 28/0 sh · 527 pt · 3 pen · 0 h | 42/0 sh · 696 pt · 3 pen · 1 h | 56/0 sh · 441 pt · 4 pen · 1 h | (c) 200 has 527 pts > 50 441 pts; (c) 100 has 696 pts > 50 441 pts |
| tour-guide | humans | 16/0 sh · 465 pt · 2 pen · 0 h | 23/0 sh · 462 pt · 3 pen · 0 h | 32/0 sh · 512 pt · 3 pen · 0 h | 53/0 sh · 552 pt · 4 pen · 0 h | (c) 500 has 465 pts > 200 462 pts |
| walking-commuter | humans | 8/0 sh · 233 pt · 2 pen · 0 h | 22/0 sh · 512 pt · 3 pen · 0 h | 36/0 sh · 710 pt · 3 pen · 1 h | 60/0 sh · 465 pt · 4 pen · 1 h | (c) 200 has 512 pts > 50 465 pts; (c) 100 has 710 pts > 50 465 pts |
| wheelchair-user | humans | 12/0 sh · 209 pt · 2 pen · 0 h | 38/0 sh · 534 pt · 5 pen · 0 h | 60/0 sh · 743 pt · 3 pen · 0 h | 75/0 sh · 614 pt · 4 pen · 0 h | (c) 100 has 743 pts > 50 614 pts |
| worker-carrying-box | humans | 14/0 sh · 362 pt · 1 pen · 0 h | 21/0 sh · 579 pt · 2 pen · 0 h | 36/0 sh · 704 pt · 3 pen · 1 h | 56/0 sh · 659 pt · 3 pen · 0 h | (c) 100 has 704 pts > 50 659 pts |
| excavator | machinery | 9/0 sh · 96 pt · 3 pen · 0 h | 18/0 sh · 116 pt · 3 pen · 0 h | 38/0 sh · 152 pt · 4 pen · 0 h | 57/0 sh · 299 pt · 5 pen · 0 h | — |
| forklift | machinery | 14/0 sh · 62 pt · 3 pen · 0 h | 25/0 sh · 93 pt · 3 pen · 0 h | 32/0 sh · 172 pt · 4 pen · 0 h | 41/0 sh · 289 pt · 5 pen · 0 h | — |
| tower-crane | machinery | 27/0 sh · 89 pt · 3 pen · 0 h | 35/0 sh · 172 pt · 6 pen · 0 h | 44/0 sh · 240 pt · 8 pen · 0 h | 46/0 sh · 248 pt · 5 pen · 0 h | — |
| market-parasol | objects | 4/0 sh · 16 pt · 2 pen · 0 h | 11/0 sh · 32 pt · 3 pen · 0 h | 21/0 sh · 58 pt · 4 pen · 0 h | 55/0 sh · 149 pt · 6 pen · 0 h | — |
| playground-swing | objects | 8/0 sh · 21 pt · 2 pen · 0 h | 8/0 sh · 21 pt · 3 pen · 0 h | 8/0 sh · 36 pt · 5 pen · 0 h | 38/0 sh · 119 pt · 7 pen · 0 h | (a) 500=200 same shape ids |
| street-lamp | objects | 4/0 sh · 14 pt · 2 pen · 0 h | 6/0 sh · 25 pt · 3 pen · 0 h | 8/0 sh · 58 pt · 4 pen · 0 h | 30/0 sh · 121 pt · 6 pen · 1 h | — |
| waste-bin | objects | 2/0 sh · 12 pt · 2 pen · 0 h | 7/0 sh · 28 pt · 3 pen · 0 h | 9/0 sh · 37 pt · 4 pen · 0 h | 21/0 sh · 93 pt · 6 pen · 0 h | — |
| date-palm | vegetation | 4/0 sh · 89 pt · 2 pen · 0 h | 30/0 sh · 209 pt · 3 pen · 0 h | 43/0 sh · 630 pt · 4 pen · 0 h | 107/0 sh · 2494 pt · 5 pen · 0 h | — |
| english-oak | vegetation | 4/0 sh · 181 pt · 2 pen · 0 h | 15/0 sh · 327 pt · 3 pen · 0 h | 37/0 sh · 565 pt · 4 pen · 1 h | 71/0 sh · 1325 pt · 5 pen · 1 h | — |
| hornbeam-hedge | vegetation | 2/0 sh · 68 pt · 2 pen · 0 h | 13/0 sh · 106 pt · 3 pen · 0 h | 37/0 sh · 208 pt · 4 pen · 1 h | 50/0 sh · 773 pt · 5 pen · 1 h | — |
| lombardy-poplar | vegetation | 2/0 sh · 108 pt · 2 pen · 0 h | 18/0 sh · 321 pt · 3 pen · 0 h | 38/0 sh · 524 pt · 4 pen · 0 h | 58/0 sh · 1097 pt · 5 pen · 0 h | — |
| miscanthus-grass | vegetation | 1/0 sh · 55 pt · 2 pen · 0 h | 21/0 sh · 180 pt · 3 pen · 0 h | 37/0 sh · 288 pt · 4 pen · 0 h | 160/0 sh · 1537 pt · 5 pen · 0 h | — |
| multi-stem-birch | vegetation | 2/0 sh · 113 pt · 2 pen · 0 h | 19/0 sh · 350 pt · 3 pen · 0 h | 55/0 sh · 665 pt · 4 pen · 0 h | 62/0 sh · 1229 pt · 5 pen · 0 h | — |
| scots-pine | vegetation | 3/0 sh · 122 pt · 2 pen · 0 h | 15/0 sh · 278 pt · 3 pen · 0 h | 43/0 sh · 477 pt · 4 pen · 1 h | 51/0 sh · 1020 pt · 5 pen · 1 h | — |
| weeping-willow | vegetation | 2/0 sh · 150 pt · 2 pen · 0 h | 13/0 sh · 332 pt · 3 pen · 0 h | 41/0 sh · 552 pt · 4 pen · 0 h | 64/0 sh · 1469 pt · 5 pen · 0 h | — |
| cargo-bike | vehicles | 9/0 sh · 32 pt · 3 pen · 0 h | 25/0 sh · 70 pt · 6 pen · 0 h | 34/0 sh · 93 pt · 7 pen · 0 h | 44/0 sh · 287 pt · 5 pen · 0 h | — |
| city-bus | vehicles | 14/0 sh · 71 pt · 3 pen · 0 h | 34/0 sh · 171 pt · 3 pen · 0 h | 46/0 sh · 309 pt · 4 pen · 0 h | 53/0 sh · 663 pt · 5 pen · 1 h | — |
| city-car | vehicles | 6/0 sh · 61 pt · 3 pen · 0 h | 19/0 sh · 125 pt · 3 pen · 0 h | 38/0 sh · 176 pt · 4 pen · 0 h | 62/0 sh · 335 pt · 5 pen · 1 h | — |

## Flagged assets

14 of 48 assets flagged, 192 asset-scale entries audited.

- **dairy-cow** (animals): (a) 200=100 same shape ids; (b) 200~100 identical; (c) 200 has 808 pts > 50 769 pts; (c) 100 has 808 pts > 50 769 pts
- **pigeon** (animals): (c) 100 has 448 pts > 50 412 pts
- **glass-balustrade** (components): (a) 200=100 same shape ids
- **glulam-beam** (components): (a) 200=100 same shape ids
- **cyclist** (humans): (c) 100 has 940 pts > 50 694 pts
- **elderly-with-cane** (humans): (c) 200 has 522 pts > 50 423 pts; (c) 100 has 655 pts > 50 423 pts
- **parent-with-stroller** (humans): (c) 200 has 764 pts > 50 677 pts; (c) 100 has 929 pts > 50 677 pts
- **running-child** (humans): (c) 200 has 646 pts > 50 456 pts; (c) 100 has 775 pts > 50 456 pts
- **seated-reader** (humans): (c) 200 has 527 pts > 50 441 pts; (c) 100 has 696 pts > 50 441 pts
- **tour-guide** (humans): (c) 500 has 465 pts > 200 462 pts
- **walking-commuter** (humans): (c) 200 has 512 pts > 50 465 pts; (c) 100 has 710 pts > 50 465 pts
- **wheelchair-user** (humans): (c) 100 has 743 pts > 50 614 pts
- **worker-carrying-box** (humans): (c) 100 has 704 pts > 50 659 pts
- **playground-swing** (objects): (a) 500=200 same shape ids

Flag counts by code: (a) 4, (b) 1, (c) 17

## Per-category totals

| Category | Assets | Flagged | Unavailable (500/200/100/50) | Points (500/200/100/50) | Shapes (500/200/100/50) |
|---|---|---|---|---|---|
| animals | 6 | 2 | 2/0/0/0 | 1117/3066/4049/4080 | 64/161/213/263 |
| components | 8 | 2 | 2/0/0/0 | 74/127/329/580 | 10/43/74/163 |
| furniture | 4 | 0 | 0/0/0/0 | 65/178/229/498 | 24/37/65/150 |
| humans | 12 | 9 | 0/0/0/0 | 4217/6764/8383/6822 | 165/332/479/739 |
| machinery | 3 | 0 | 0/0/0/0 | 247/381/564/836 | 50/78/114/144 |
| objects | 4 | 1 | 0/0/0/0 | 63/106/189/482 | 18/32/46/144 |
| vegetation | 8 | 0 | 0/0/0/0 | 886/2103/3909/10944 | 20/144/331/623 |
| vehicles | 3 | 0 | 0/0/0/0 | 164/366/578/1285 | 29/78/118/159 |
| **Total** | 48 | 14 | | | |

## Detail per asset and scale

| Asset | Scale | Kind | Elev | Plan | Points | Segments | Pens | Hatches | Decisions en+de | validRange |
|---|---|---|---|---|---|---|---|---|---|---|
| dairy-cow | 1:500 | authored | 14 | 0 | 257 | 91 | 4 | 0 | ok | ok |
| dairy-cow | 1:200 | authored | 40 | 0 | 808 | 281 | 3 | 0 | ok | ok |
| dairy-cow | 1:100 | authored | 40 | 0 | 808 | 281 | 3 | 0 | ok | ok |
| dairy-cow | 1:50 | authored | 44 | 0 | 769 | 250 | 4 | 0 | ok | ok |
| domestic-cat | 1:500 | unavailable | — | — | — | — | — | — | — | — |
| domestic-cat | 1:200 | authored | 21 | 0 | 266 | 93 | 4 | 0 | ok | ok |
| domestic-cat | 1:100 | authored | 38 | 0 | 715 | 228 | 3 | 0 | ok | ok |
| domestic-cat | 1:50 | authored | 49 | 0 | 745 | 248 | 4 | 0 | ok | ok |
| horse | 1:500 | authored | 20 | 0 | 380 | 128 | 3 | 0 | ok | ok |
| horse | 1:200 | authored | 40 | 0 | 770 | 248 | 3 | 0 | ok | ok |
| horse | 1:100 | authored | 43 | 0 | 800 | 257 | 3 | 0 | ok | ok |
| horse | 1:50 | authored | 57 | 0 | 833 | 276 | 4 | 0 | ok | ok |
| labrador-dog | 1:500 | authored | 17 | 0 | 245 | 86 | 4 | 0 | ok | ok |
| labrador-dog | 1:200 | authored | 19 | 0 | 457 | 146 | 3 | 0 | ok | ok |
| labrador-dog | 1:100 | authored | 37 | 0 | 728 | 235 | 3 | 0 | ok | ok |
| labrador-dog | 1:50 | authored | 46 | 0 | 755 | 253 | 4 | 0 | ok | ok |
| pigeon | 1:500 | unavailable | — | — | — | — | — | — | — | — |
| pigeon | 1:200 | authored | 13 | 0 | 217 | 72 | 2 | 0 | ok | ok |
| pigeon | 1:100 | authored | 25 | 0 | 448 | 143 | 3 | 0 | ok | ok |
| pigeon | 1:50 | authored | 31 | 0 | 412 | 138 | 4 | 0 | ok | ok |
| roe-deer | 1:500 | authored | 13 | 0 | 235 | 82 | 3 | 0 | ok | ok |
| roe-deer | 1:200 | authored | 28 | 0 | 548 | 176 | 3 | 0 | ok | ok |
| roe-deer | 1:100 | authored | 30 | 0 | 550 | 178 | 3 | 0 | ok | ok |
| roe-deer | 1:50 | authored | 36 | 0 | 566 | 188 | 4 | 0 | ok | ok |
| cantilever-balcony | 1:500 | authored | 2 | 0 | 8 | 7 | 2 | 0 | ok | ok |
| cantilever-balcony | 1:200 | authored | 5 | 0 | 18 | 16 | 3 | 0 | ok | ok |
| cantilever-balcony | 1:100 | authored | 7 | 0 | 44 | 43 | 4 | 2 | ok | ok |
| cantilever-balcony | 1:50 | authored | 20 | 0 | 83 | 74 | 7 | 2 | ok | ok |
| glass-balustrade | 1:500 | unavailable | — | — | — | — | — | — | — | — |
| glass-balustrade | 1:200 | authored | 3 | 0 | 7 | 6 | 3 | 0 | ok | ok |
| glass-balustrade | 1:100 | authored | 3 | 0 | 9 | 9 | 4 | 0 | ok | ok |
| glass-balustrade | 1:50 | authored | 8 | 0 | 25 | 24 | 5 | 1 | ok | ok |
| glulam-beam | 1:500 | authored | 1 | 0 | 4 | 4 | 2 | 0 | ok | ok |
| glulam-beam | 1:200 | authored | 3 | 0 | 8 | 6 | 3 | 0 | ok | ok |
| glulam-beam | 1:100 | authored | 3 | 0 | 12 | 12 | 4 | 1 | ok | ok |
| glulam-beam | 1:50 | authored | 23 | 0 | 70 | 50 | 5 | 0 | ok | ok |
| sliding-door | 1:500 | authored | 2 | 0 | 12 | 12 | 2 | 0 | ok | ok |
| sliding-door | 1:200 | authored | 7 | 0 | 24 | 22 | 3 | 0 | ok | ok |
| sliding-door | 1:100 | authored | 10 | 0 | 36 | 34 | 4 | 0 | ok | ok |
| sliding-door | 1:50 | authored | 21 | 0 | 56 | 51 | 5 | 0 | ok | ok |
| stair-flight | 1:500 | authored | 1 | 0 | 28 | 28 | 2 | 0 | ok | ok |
| stair-flight | 1:200 | authored | 13 | 0 | 28 | 16 | 3 | 0 | ok | ok |
| stair-flight | 1:100 | authored | 25 | 0 | 88 | 88 | 4 | 1 | ok | ok |
| stair-flight | 1:50 | authored | 39 | 0 | 116 | 102 | 5 | 0 | ok | ok |
| steel-column | 1:500 | authored | 1 | 0 | 12 | 12 | 2 | 0 | ok | ok |
| steel-column | 1:200 | authored | 3 | 0 | 12 | 12 | 3 | 0 | ok | ok |
| steel-column | 1:100 | authored | 6 | 0 | 20 | 19 | 4 | 0 | ok | ok |
| steel-column | 1:50 | authored | 16 | 0 | 50 | 44 | 5 | 0 | ok | ok |
| tied-canopy | 1:500 | authored | 3 | 0 | 10 | 9 | 2 | 0 | ok | ok |
| tied-canopy | 1:200 | authored | 5 | 0 | 16 | 14 | 4 | 0 | ok | ok |
| tied-canopy | 1:100 | authored | 13 | 0 | 95 | 94 | 4 | 1 | ok | ok |
| tied-canopy | 1:50 | authored | 23 | 0 | 137 | 130 | 6 | 1 | ok | ok |
| tilt-window | 1:500 | unavailable | — | — | — | — | — | — | — | — |
| tilt-window | 1:200 | authored | 4 | 0 | 14 | 13 | 4 | 0 | ok | ok |
| tilt-window | 1:100 | authored | 7 | 0 | 25 | 25 | 4 | 0 | ok | ok |
| tilt-window | 1:50 | authored | 13 | 0 | 43 | 36 | 5 | 1 | ok | ok |
| cafe-chair | 1:500 | authored | 4 | 0 | 10 | 7 | 2 | 0 | ok | ok |
| cafe-chair | 1:200 | authored | 6 | 0 | 14 | 9 | 3 | 0 | ok | ok |
| cafe-chair | 1:100 | authored | 8 | 0 | 18 | 11 | 4 | 0 | ok | ok |
| cafe-chair | 1:50 | authored | 21 | 0 | 69 | 42 | 7 | 0 | ok | ok |
| height-adjustable-desk | 1:500 | authored | 10 | 0 | 28 | 22 | 2 | 0 | ok | ok |
| height-adjustable-desk | 1:200 | authored | 15 | 0 | 52 | 48 | 3 | 0 | ok | ok |
| height-adjustable-desk | 1:100 | authored | 19 | 0 | 70 | 66 | 5 | 0 | ok | ok |
| height-adjustable-desk | 1:50 | authored | 37 | 0 | 136 | 113 | 8 | 1 | ok | ok |
| office-swivel-chair | 1:500 | authored | 7 | 0 | 15 | 13 | 2 | 0 | ok | ok |
| office-swivel-chair | 1:200 | authored | 10 | 0 | 36 | 27 | 3 | 0 | ok | ok |
| office-swivel-chair | 1:100 | authored | 12 | 0 | 65 | 43 | 4 | 0 | ok | ok |
| office-swivel-chair | 1:50 | authored | 39 | 0 | 149 | 85 | 7 | 0 | ok | ok |
| park-bench | 1:500 | authored | 3 | 0 | 12 | 12 | 2 | 0 | ok | ok |
| park-bench | 1:200 | authored | 6 | 0 | 76 | 64 | 3 | 0 | ok | ok |
| park-bench | 1:100 | authored | 26 | 0 | 76 | 56 | 4 | 0 | ok | ok |
| park-bench | 1:50 | authored | 53 | 0 | 144 | 103 | 6 | 1 | ok | ok |
| cyclist | 1:500 | authored | 19 | 0 | 379 | 130 | 1 | 0 | ok | ok |
| cyclist | 1:200 | authored | 41 | 0 | 680 | 227 | 3 | 0 | ok | ok |
| cyclist | 1:100 | authored | 58 | 0 | 940 | 314 | 13 | 2 | ok | ok |
| cyclist | 1:50 | authored | 108 | 0 | 694 | 282 | 4 | 1 | ok | ok |
| elderly-with-cane | 1:500 | authored | 14 | 0 | 279 | 91 | 1 | 0 | ok | ok |
| elderly-with-cane | 1:200 | authored | 21 | 0 | 522 | 169 | 2 | 0 | ok | ok |
| elderly-with-cane | 1:100 | authored | 31 | 0 | 655 | 215 | 3 | 2 | ok | ok |
| elderly-with-cane | 1:50 | authored | 46 | 0 | 423 | 166 | 3 | 1 | ok | ok |
| jogger | 1:500 | authored | 13 | 0 | 453 | 153 | 2 | 0 | ok | ok |
| jogger | 1:200 | authored | 22 | 0 | 531 | 179 | 3 | 0 | ok | ok |
| jogger | 1:100 | authored | 28 | 0 | 597 | 203 | 3 | 0 | ok | ok |
| jogger | 1:50 | authored | 55 | 0 | 639 | 245 | 4 | 0 | ok | ok |
| leaning-person | 1:500 | authored | 14 | 0 | 455 | 154 | 2 | 0 | ok | ok |
| leaning-person | 1:200 | authored | 22 | 0 | 458 | 157 | 3 | 0 | ok | ok |
| leaning-person | 1:100 | authored | 29 | 0 | 518 | 184 | 3 | 0 | ok | ok |
| leaning-person | 1:50 | authored | 51 | 0 | 559 | 219 | 4 | 0 | ok | ok |
| parent-with-stroller | 1:500 | authored | 19 | 0 | 462 | 153 | 1 | 0 | ok | ok |
| parent-with-stroller | 1:200 | authored | 35 | 0 | 764 | 251 | 4 | 0 | ok | ok |
| parent-with-stroller | 1:100 | authored | 49 | 0 | 929 | 308 | 10 | 3 | ok | ok |
| parent-with-stroller | 1:50 | authored | 69 | 0 | 677 | 282 | 3 | 0 | ok | ok |
| person-with-umbrella | 1:500 | authored | 16 | 0 | 499 | 168 | 2 | 0 | ok | ok |
| person-with-umbrella | 1:200 | authored | 30 | 0 | 549 | 189 | 3 | 0 | ok | ok |
| person-with-umbrella | 1:100 | authored | 40 | 0 | 604 | 213 | 3 | 0 | ok | ok |
| person-with-umbrella | 1:50 | authored | 63 | 0 | 643 | 251 | 4 | 1 | ok | ok |
| running-child | 1:500 | authored | 9 | 0 | 116 | 41 | 3 | 0 | ok | ok |
| running-child | 1:200 | authored | 29 | 0 | 646 | 209 | 3 | 0 | ok | ok |
| running-child | 1:100 | authored | 38 | 0 | 775 | 257 | 3 | 0 | ok | ok |
| running-child | 1:50 | authored | 47 | 0 | 456 | 179 | 4 | 0 | ok | ok |
| seated-reader | 1:500 | authored | 11 | 0 | 305 | 114 | 2 | 0 | ok | ok |
| seated-reader | 1:200 | authored | 28 | 0 | 527 | 189 | 3 | 0 | ok | ok |
| seated-reader | 1:100 | authored | 42 | 0 | 696 | 247 | 3 | 1 | ok | ok |
| seated-reader | 1:50 | authored | 56 | 0 | 441 | 192 | 4 | 1 | ok | ok |
| tour-guide | 1:500 | authored | 16 | 0 | 465 | 158 | 2 | 0 | ok | ok |
| tour-guide | 1:200 | authored | 23 | 0 | 462 | 160 | 3 | 0 | ok | ok |
| tour-guide | 1:100 | authored | 32 | 0 | 512 | 181 | 3 | 0 | ok | ok |
| tour-guide | 1:50 | authored | 53 | 0 | 552 | 216 | 4 | 0 | ok | ok |
| walking-commuter | 1:500 | authored | 8 | 0 | 233 | 75 | 2 | 0 | ok | ok |
| walking-commuter | 1:200 | authored | 22 | 0 | 512 | 168 | 3 | 0 | ok | ok |
| walking-commuter | 1:100 | authored | 36 | 0 | 710 | 236 | 3 | 1 | ok | ok |
| walking-commuter | 1:50 | authored | 60 | 0 | 465 | 200 | 4 | 1 | ok | ok |
| wheelchair-user | 1:500 | authored | 12 | 0 | 209 | 73 | 2 | 0 | ok | ok |
| wheelchair-user | 1:200 | authored | 38 | 0 | 534 | 184 | 5 | 0 | ok | ok |
| wheelchair-user | 1:100 | authored | 60 | 0 | 743 | 268 | 3 | 0 | ok | ok |
| wheelchair-user | 1:50 | authored | 75 | 0 | 614 | 264 | 4 | 0 | ok | ok |
| worker-carrying-box | 1:500 | authored | 14 | 0 | 362 | 118 | 1 | 0 | ok | ok |
| worker-carrying-box | 1:200 | authored | 21 | 0 | 579 | 188 | 2 | 0 | ok | ok |
| worker-carrying-box | 1:100 | authored | 36 | 0 | 704 | 237 | 3 | 1 | ok | ok |
| worker-carrying-box | 1:50 | authored | 56 | 0 | 659 | 269 | 3 | 0 | ok | ok |
| excavator | 1:500 | authored | 9 | 0 | 96 | 45 | 3 | 0 | ok | ok |
| excavator | 1:200 | authored | 18 | 0 | 116 | 58 | 3 | 0 | ok | ok |
| excavator | 1:100 | authored | 38 | 0 | 152 | 88 | 4 | 0 | ok | ok |
| excavator | 1:50 | authored | 57 | 0 | 299 | 167 | 5 | 0 | ok | ok |
| forklift | 1:500 | authored | 14 | 0 | 62 | 50 | 3 | 0 | ok | ok |
| forklift | 1:200 | authored | 25 | 0 | 93 | 75 | 3 | 0 | ok | ok |
| forklift | 1:100 | authored | 32 | 0 | 172 | 139 | 4 | 0 | ok | ok |
| forklift | 1:50 | authored | 41 | 0 | 289 | 198 | 5 | 0 | ok | ok |
| tower-crane | 1:500 | authored | 27 | 0 | 89 | 69 | 3 | 0 | ok | ok |
| tower-crane | 1:200 | authored | 35 | 0 | 172 | 137 | 6 | 0 | ok | ok |
| tower-crane | 1:100 | authored | 44 | 0 | 240 | 177 | 8 | 0 | ok | ok |
| tower-crane | 1:50 | authored | 46 | 0 | 248 | 186 | 5 | 0 | ok | ok |
| market-parasol | 1:500 | authored | 4 | 0 | 16 | 9 | 2 | 0 | ok | ok |
| market-parasol | 1:200 | authored | 11 | 0 | 32 | 19 | 3 | 0 | ok | ok |
| market-parasol | 1:100 | authored | 21 | 0 | 58 | 32 | 4 | 0 | ok | ok |
| market-parasol | 1:50 | authored | 55 | 0 | 149 | 77 | 6 | 0 | ok | ok |
| playground-swing | 1:500 | authored | 8 | 0 | 21 | 16 | 2 | 0 | ok | ok |
| playground-swing | 1:200 | authored | 8 | 0 | 21 | 16 | 3 | 0 | ok | ok |
| playground-swing | 1:100 | authored | 8 | 0 | 36 | 25 | 5 | 0 | ok | ok |
| playground-swing | 1:50 | authored | 38 | 0 | 119 | 97 | 7 | 0 | ok | ok |
| street-lamp | 1:500 | authored | 4 | 0 | 14 | 13 | 2 | 0 | ok | ok |
| street-lamp | 1:200 | authored | 6 | 0 | 25 | 14 | 3 | 0 | ok | ok |
| street-lamp | 1:100 | authored | 8 | 0 | 58 | 26 | 4 | 0 | ok | ok |
| street-lamp | 1:50 | authored | 30 | 0 | 121 | 66 | 6 | 1 | ok | ok |
| waste-bin | 1:500 | authored | 2 | 0 | 12 | 11 | 2 | 0 | ok | ok |
| waste-bin | 1:200 | authored | 7 | 0 | 28 | 19 | 3 | 0 | ok | ok |
| waste-bin | 1:100 | authored | 9 | 0 | 37 | 26 | 4 | 0 | ok | ok |
| waste-bin | 1:50 | authored | 21 | 0 | 93 | 53 | 6 | 0 | ok | ok |
| date-palm | 1:500 | authored | 4 | 0 | 89 | 41 | 2 | 0 | ok | ok |
| date-palm | 1:200 | authored | 30 | 0 | 209 | 119 | 3 | 0 | ok | ok |
| date-palm | 1:100 | authored | 43 | 0 | 630 | 495 | 4 | 0 | ok | ok |
| date-palm | 1:50 | authored | 107 | 0 | 2494 | 852 | 5 | 0 | ok | ok |
| english-oak | 1:500 | authored | 4 | 0 | 181 | 61 | 2 | 0 | ok | ok |
| english-oak | 1:200 | authored | 15 | 0 | 327 | 108 | 3 | 0 | ok | ok |
| english-oak | 1:100 | authored | 37 | 0 | 565 | 188 | 4 | 1 | ok | ok |
| english-oak | 1:50 | authored | 71 | 0 | 1325 | 487 | 5 | 1 | ok | ok |
| hornbeam-hedge | 1:500 | authored | 2 | 0 | 68 | 24 | 2 | 0 | ok | ok |
| hornbeam-hedge | 1:200 | authored | 13 | 0 | 106 | 57 | 3 | 0 | ok | ok |
| hornbeam-hedge | 1:100 | authored | 37 | 0 | 208 | 93 | 4 | 1 | ok | ok |
| hornbeam-hedge | 1:50 | authored | 50 | 0 | 773 | 286 | 5 | 1 | ok | ok |
| lombardy-poplar | 1:500 | authored | 2 | 0 | 108 | 36 | 2 | 0 | ok | ok |
| lombardy-poplar | 1:200 | authored | 18 | 0 | 321 | 107 | 3 | 0 | ok | ok |
| lombardy-poplar | 1:100 | authored | 38 | 0 | 524 | 174 | 4 | 0 | ok | ok |
| lombardy-poplar | 1:50 | authored | 58 | 0 | 1097 | 402 | 5 | 0 | ok | ok |
| miscanthus-grass | 1:500 | authored | 1 | 0 | 55 | 18 | 2 | 0 | ok | ok |
| miscanthus-grass | 1:200 | authored | 21 | 0 | 180 | 95 | 3 | 0 | ok | ok |
| miscanthus-grass | 1:100 | authored | 37 | 0 | 288 | 171 | 4 | 0 | ok | ok |
| miscanthus-grass | 1:50 | authored | 160 | 0 | 1537 | 552 | 5 | 0 | ok | ok |
| multi-stem-birch | 1:500 | authored | 2 | 0 | 113 | 51 | 2 | 0 | ok | ok |
| multi-stem-birch | 1:200 | authored | 19 | 0 | 350 | 117 | 3 | 0 | ok | ok |
| multi-stem-birch | 1:100 | authored | 55 | 0 | 665 | 216 | 4 | 0 | ok | ok |
| multi-stem-birch | 1:50 | authored | 62 | 0 | 1229 | 441 | 5 | 0 | ok | ok |
| scots-pine | 1:500 | authored | 3 | 0 | 122 | 41 | 2 | 0 | ok | ok |
| scots-pine | 1:200 | authored | 15 | 0 | 278 | 93 | 3 | 0 | ok | ok |
| scots-pine | 1:100 | authored | 43 | 0 | 477 | 165 | 4 | 1 | ok | ok |
| scots-pine | 1:50 | authored | 51 | 0 | 1020 | 369 | 5 | 1 | ok | ok |
| weeping-willow | 1:500 | authored | 2 | 0 | 150 | 50 | 2 | 0 | ok | ok |
| weeping-willow | 1:200 | authored | 13 | 0 | 332 | 151 | 3 | 0 | ok | ok |
| weeping-willow | 1:100 | authored | 41 | 0 | 552 | 227 | 4 | 0 | ok | ok |
| weeping-willow | 1:50 | authored | 64 | 0 | 1469 | 548 | 5 | 0 | ok | ok |
| cargo-bike | 1:500 | authored | 9 | 0 | 32 | 22 | 3 | 0 | ok | ok |
| cargo-bike | 1:200 | authored | 25 | 0 | 70 | 44 | 6 | 0 | ok | ok |
| cargo-bike | 1:100 | authored | 34 | 0 | 93 | 63 | 7 | 0 | ok | ok |
| cargo-bike | 1:50 | authored | 44 | 0 | 287 | 159 | 5 | 0 | ok | ok |
| city-bus | 1:500 | authored | 14 | 0 | 71 | 53 | 3 | 0 | ok | ok |
| city-bus | 1:200 | authored | 34 | 0 | 171 | 133 | 3 | 0 | ok | ok |
| city-bus | 1:100 | authored | 46 | 0 | 309 | 239 | 4 | 0 | ok | ok |
| city-bus | 1:50 | authored | 53 | 0 | 663 | 445 | 5 | 1 | ok | ok |
| city-car | 1:500 | authored | 6 | 0 | 61 | 33 | 3 | 0 | ok | ok |
| city-car | 1:200 | authored | 19 | 0 | 125 | 61 | 3 | 0 | ok | ok |
| city-car | 1:100 | authored | 38 | 0 | 176 | 89 | 4 | 0 | ok | ok |
| city-car | 1:50 | authored | 62 | 0 | 335 | 168 | 5 | 1 | ok | ok |
