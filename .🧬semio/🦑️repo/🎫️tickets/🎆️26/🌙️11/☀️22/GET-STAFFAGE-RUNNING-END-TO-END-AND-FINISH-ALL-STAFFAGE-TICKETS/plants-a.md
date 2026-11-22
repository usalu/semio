# 🌳️ Finisher Report: Plant Wave, Workers PW-01 … PW-09

Ticket `K` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/STAFFAGE-PLANT-COLLECTION-WAVE`, scope: 27 planned plants of the workers `PW-01` … `PW-09`. Written by the finisher of this scope on 2026-11-22; every number below was measured in this session.

## State found (measured 20:27 to 22:10)

Tools: whole-asset validation `r3-validate.ts vegetation/<id> --catalog-only --volumes` (schema, `validateAsset`, print, `checkVolumes`) and the staffage check CLI `🩺️check --asset <id>`, one process at a time (`K/🧰️scripts/finish-a/sweep.sh`).

| Worker | Plant | Catalog file | Row | schema | validateAsset | checkVolumes | check CLI |
|---|---|---|---|---|---|---|---|
| PW-01 | norway-maple, field-maple, sycamore-maple | none (generator only, candidates of the worker lost with `🗑️generated`) | none | – | – | – | – |
| PW-02 | horse-chestnut | yes | yes | 0 | 4 (extents of 1:500 and symbol) | 7 | ✗ |
| PW-02 | european-beech | yes | yes | 0 | 10 (extents of 1:500 and symbol; run without print) | not measured (run stopped after 16 minutes, the asset is rebuilt) | not run |
| PW-02 | common-ash | yes | yes | 0 | 5 (extents of 1:100, 1:200 and symbol) | 13 | ✗ |
| PW-03 | japanese-cherry | yes | yes | 0 | 0 | 0 | ✓ |
| PW-03 | saucer-magnolia | yes | yes | 0 | 0 | 0 | ✓ |
| PW-03 | crab-apple | yes | yes | 0 | 0 | 0 | ✓ |
| PW-04 | ginkgo | yes | yes | 0 | 0 | 0 | ✓ |
| PW-04 | honey-locust | yes | yes | 0 | 0 | 1 (cross-scale 1:500 against 1:200, elevation) | ✓ |
| PW-04 | sweetgum | yes | yes | 0 | 0 | 0 | ✓ |
| PW-05 | tulip-tree, red-oak, black-locust | none (generator only) | none | – | – | – | – |
| PW-06 | rowan, hawthorn, black-alder | none (generator only) | none | – | – | – | – |
| PW-07 | norway-spruce | yes | yes | 0 | 0 | 8 | ✓ |
| PW-07 | european-larch | yes | yes | 0 | 0 | 14 | ✓ |
| PW-07 | atlas-cedar | yes | yes | 0 | 0 | 1 (bark-lower area 1:50) | ✓ |
| PW-08 | italian-cypress, english-yew, dawn-redwood | none (generator only) | none | – | – | – | – |
| PW-09 | olive-tree | yes | yes | 0 | 0 | 6 | ✓ |
| PW-09 | fig-tree | yes | yes | 0 | 0 (run without print) | not yet measured | not yet run |
| PW-09 | stone-pine | yes | yes | 0 | 0 | 11 | ✓ |

All 15 row files matched their catalog asset by the rules of `validateManifest` (`K/🧰️scripts/finish-a/rows.py`).
