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

## State after the finisher (final proofs, 2026-11-23 01:30 to 02:40)

All 27 planned plants of the scope were started by their workers (every worker folder had a generator), so all 27 are finished here; none was never started.

Hard gates, all 27: JSON schema 0, `validateAsset` 0 (with the print checks over every valid range), staffage check CLI `✓ <id> (1/1)` with every scale `authored, suitable under office-laser`, row file matching the asset. Catalog files of the scope: 27 of 27 (12 new). Row files of the scope: 27 of 27.

Volume rules (`checkVolumes`, the measured 3D rules): 9 of 27 clean, 18 with findings left (listed per plant below and with their cause in the worker reports).

| Plant | Whole-asset validation | Remaining volume findings | Check CLI |
|---|---|---|---|
| `norway-maple` | `INVALID vegetation/norway-maple (catalog file as it is): schema 0, validateAsset 0, checkVolumes 10` | connected 1:100 ×1, cross-scale 1:100 ×3, cross-scale 1:200 ×3, cross-scale 1:500 ×3 | `✓ norway-maple (1/1)` |
| `field-maple` | `INVALID vegetation/field-maple (catalog file as it is): schema 0, validateAsset 0, checkVolumes 8` | connected 1:100 ×1, cross-scale 1:100 ×3, cross-scale 1:200 ×3, cross-scale 1:500 ×1 | `✓ field-maple (1/1)` |
| `sycamore-maple` | `INVALID vegetation/sycamore-maple (catalog file as it is): schema 0, validateAsset 0, checkVolumes 9` | cross-scale 1:100 ×3, cross-scale 1:200 ×3, cross-scale 1:500 ×3 | `✓ sycamore-maple (1/1)` |
| `horse-chestnut` | `INVALID vegetation/horse-chestnut (catalog file as it is, two runs: with print without volumes, with volumes without print): schema 0, validateAsset 0, checkVolumes 7` | cross-scale 1:100 ×3, cross-scale 1:200 ×3, cross-scale 1:500 ×1 | `✓ horse-chestnut (1/1)` |
| `european-beech` | `INVALID vegetation/european-beech (catalog file as it is): schema 0, validateAsset 0, checkVolumes 9` | cross-scale 1:100 ×3, cross-scale 1:200 ×3, cross-scale 1:500 ×3 | `✓ european-beech (1/1)` |
| `common-ash` | `INVALID vegetation/common-ash (catalog file as it is, two runs: with print without volumes, with volumes without print): schema 0, validateAsset 0, checkVolumes 9` | cross-scale 1:100 ×3, cross-scale 1:200 ×3, cross-scale 1:500 ×3 | `✓ common-ash (1/1)` |
| `japanese-cherry` | `VALID vegetation/japanese-cherry (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` | none | `✓ japanese-cherry (1/1)` |
| `saucer-magnolia` | `VALID vegetation/saucer-magnolia (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` | none | `✓ saucer-magnolia (1/1)` |
| `crab-apple` | `VALID vegetation/crab-apple (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` | none | `✓ crab-apple (1/1)` |
| `ginkgo` | `VALID vegetation/ginkgo (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` | none | `✓ ginkgo (1/1)` |
| `honey-locust` | `INVALID vegetation/honey-locust (catalog file as it is): schema 0, validateAsset 0, checkVolumes 1` | cross-scale 1:500 ×1 | `✓ honey-locust (1/1)` |
| `sweetgum` | `VALID vegetation/sweetgum (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` | none | `✓ sweetgum (1/1)` |
| `tulip-tree` | `INVALID vegetation/tulip-tree (catalog file as it is): schema 0, validateAsset 0, checkVolumes 1` | cross-scale 1:500 ×1 | `✓ tulip-tree (1/1)` |
| `red-oak` | `VALID vegetation/red-oak (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` | none | `✓ red-oak (1/1)` |
| `black-locust` | `VALID vegetation/black-locust (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` | none | `✓ black-locust (1/1)` |
| `rowan` | `INVALID vegetation/rowan (catalog file as it is): schema 0, validateAsset 0, checkVolumes 1` | silhouette 1:50 ×1 | `✓ rowan (1/1)` |
| `hawthorn` | `INVALID vegetation/hawthorn (catalog file as it is): schema 0, validateAsset 0, checkVolumes 12` | connected 1:100 ×1, connected 1:200 ×1, connected 1:50 ×1, connected 1:500 ×1, cross-scale 1:100 ×2, cross-scale 1:200 ×2, silhouette 1:100 ×1, silhouette 1:200 ×1, silhouette 1:50 ×1, structure-coverage 1:500 ×1 | `✓ hawthorn (1/1)` |
| `black-alder` | `INVALID vegetation/black-alder (catalog file as it is): schema 0, validateAsset 0, checkVolumes 15` | connected 1:500 ×1, cross-scale 1:100 ×3, cross-scale 1:200 ×3, cross-scale 1:500 ×2, silhouette 1:50 ×1, structure-coverage 1:100 ×1, structure-coverage 1:200 ×1, structure-coverage 1:50 ×1, structure-coverage 1:500 ×2 | `✓ black-alder (1/1)` |
| `norway-spruce` | `INVALID vegetation/norway-spruce (catalog file as it is): schema 0, validateAsset 0, checkVolumes 4` | cross-scale 1:100 ×3, cross-scale 1:200 ×1 | `✓ norway-spruce (1/1)` |
| `european-larch` | `INVALID vegetation/european-larch (catalog file as it is): schema 0, validateAsset 0, checkVolumes 7` | cross-scale 1:100 ×2, cross-scale 1:200 ×3, cross-scale 1:500 ×2 | `✓ european-larch (1/1)` |
| `atlas-cedar` | `VALID vegetation/atlas-cedar (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` | none | `✓ atlas-cedar (1/1)` |
| `italian-cypress` | `VALID vegetation/italian-cypress (catalog file as it is): schema 0, validateAsset 0, checkVolumes 0` | none | `✓ italian-cypress (1/1)` |
| `english-yew` | `INVALID vegetation/english-yew (catalog file as it is): schema 0, validateAsset 0, checkVolumes 7` | cross-scale 1:100 ×3, structure-coverage 1:100 ×2, structure-coverage 1:50 ×2 | `✓ english-yew (1/1)` |
| `dawn-redwood` | `INVALID vegetation/dawn-redwood (catalog file as it is): schema 0, validateAsset 0, checkVolumes 8` | cross-scale 1:500 ×3, structure-coverage 1:100 ×2, structure-coverage 1:50 ×2, thermal-coverage 1:500 ×1 | `✓ dawn-redwood (1/1)` |
| `olive-tree` | `INVALID vegetation/olive-tree (catalog file as it is): schema 0, validateAsset 0, checkVolumes 6` | connected 1:100 ×1, connected 1:200 ×1, connected 1:50 ×1, structure-coverage 1:200 ×1, thermal-coverage 1:100 ×1, thermal-coverage 1:50 ×1 | `✓ olive-tree (1/1)` |
| `fig-tree` | `INVALID vegetation/fig-tree (catalog file as it is): schema 0, validateAsset 0, checkVolumes 10` | connected 1:100 ×1, connected 1:200 ×1, connected 1:50 ×1, connected 1:500 ×1, cross-scale 1:100 ×1, cross-scale 1:200 ×1, cross-scale 1:500 ×3, structure-coverage 1:100 ×1 | `✓ fig-tree (1/1)` |
| `stone-pine` | `INVALID vegetation/stone-pine (catalog file as it is): schema 0, validateAsset 0, checkVolumes 11` | connected 1:100 ×1, connected 1:200 ×2, connected 1:500 ×1, structure-coverage 1:200 ×2, structure-coverage 1:50 ×2, structure-coverage 1:500 ×2, thermal-coverage 1:50 ×1 | `✓ stone-pine (1/1)` |

`horse-chestnut` and `common-ash`: two runs each (with print without volume rules: `VALID … schema 0, validateAsset 0`; with volume rules without print: the findings above), because a combined run exceeded 15 minutes on the shared host. The check CLI of the nine plants shipped unchanged from a candidate was run on the candidate file and the shipped file was compared with it (identical); `field-maple`, `norway-maple`, `tulip-tree` and `fig-tree` were checked as catalog files.

## Per worker: before, after, what changed

| Worker | Before | After | Change |
|---|---|---|---|
| PW-01 maples | no catalog file, no row | 3 shipped, valid, check ✓; volume findings 10 / 8 / 9 | rotations renormalised in `asset.ts` (sycamore failed `volume-transform`), built and shipped |
| PW-02 park trees | shipped but invalid (`validateAsset` 4 / 10 / 5, check ✗) | valid, check ✓; volume findings 7 / 9 / 9 (were 7+ / unmeasured / 13) | coarse scales fitted to the 1:50 box in `kit.ts`, beech 1:500 top joined, rebuilt and shipped |
| PW-03 flowering trees | complete and clean | unchanged, clean | report placeholders answered |
| PW-04 street trees | complete; honey-locust 1 volume finding | unchanged | report written |
| PW-05 large broadleaves | no catalog file, no row | 3 shipped, valid, check ✓; volume findings 1 / 0 / 0 | symbol draws every crown ring (black-locust symbol extents), built and shipped |
| PW-06 small native trees | no catalog file, no row | 3 shipped, valid, check ✓; volume findings 1 / 12 / 15 | symbol rings, black-alder 1:500 rings overlap (print check at the range end), built and shipped |
| PW-07 conifers I | shipped, valid; volume findings 8 / 14 / 1 | valid, check ✓; volume findings 4 / 7 / 0 | spray members inside the body, 1:500 bark core and crown profile, cedar root area; rebuilt and shipped |
| PW-08 conifers II | no catalog file, no row | 3 shipped, valid, check ✓; volume findings 0 / 7 / 8 | height fit of the coarse scales (yew 1:500 extent), built and shipped |
| PW-09 mediterranean | shipped, valid; volume findings 6 / 24 / 11 | valid, check ✓; volume findings 6 / 10 / 11 | fig rebuilt from the changed template (`inflate` keeps slender leaves and twigs); olive and stone pine untouched |

## Preview sheets

For each of the 27 plants the 2D contact sheet (side, front, plan in line and solid at 1:500, 1:200, 1:100, 1:50) was rendered from the final file (or its identical candidate) and opened; the 3D sheets of rowan and english-yew were opened too. One line per plant is in the worker reports (section "Preview sheets opened and judged"). Exception: `black-alder` was looked at before its 1:500 rings were enlarged (the slivers between two rings that the change closes were visible on that sheet); the sheet was not rendered again.

## What stays imperfect

- `volumes-cross-scale` fails on 15 plants (all of PW-01 and PW-02, honey-locust, tulip-tree, hawthorn, black-alder, spruce, larch, yew, dawn-redwood, fig). The coarse scales of these generators are separate designs; the rule wants the same silhouette lobes at every scale. This is a redesign per generator and was not done. The catalog test suites do not run `checkVolumes` over the catalog (only on fixtures), so this does not turn a suite red by itself.
- Other volume findings left: connected bodies or load paths (norway-maple, field-maple, hawthorn, black-alder, olive, fig, stone pine), members or mass solids outside the body (hawthorn, black-alder, yew, dawn-redwood, olive, fig, stone pine), bark or total thermal areas (dawn-redwood, olive, stone pine), plan drawing against body (rowan, hawthorn, black-alder).
- `fig-tree` is the weakest asset: it still reads as the Japanese maple it is derived from, and its top tier floats above the crown at 1:200.
- `honey-locust` (single cluster on a bare leader), `dawn-redwood` (beads on a stick), `english-yew` (few coarse masses), `european-beech` (ragged top at 1:200 and 1:100; the 1:500 body text still says seven rings, there are eight) read weaker than the rest.
- `olive-tree` and `stone-pine` were not rebuilt although they have 6 and 11 volume findings: their templates (`apple-tree`, `scots-pine`) changed in the catalog after the worker's build, a rebuild takes the new template and needs generator work like the fig's.
- No vitest suite was run (reserved). The measures of the catalog silhouette test were computed for 15 plants with `finish-a/guard.ts`: every one of the wave's lobed trees and the three pre-existing plants measured for comparison (`apple-tree`, `small-leaved-lime`, `scots-pine`) exceed its stated limits (box within one smallest feature, Hausdorff distance at most two features), the three PW-07 conifers pass. See `requests.md`.

## Files

Created:

- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/{norway-maple,field-maple,sycamore-maple,tulip-tree,red-oak,black-locust,rowan,hawthorn,black-alder,italian-cypress,english-yew,dawn-redwood}/🔣️.json`
- `K/📥️manifest-rows/{norway-maple,field-maple,sycamore-maple,tulip-tree,red-oak,black-locust,rowan,hawthorn,black-alder,italian-cypress,english-yew,dawn-redwood}.json`
- `K/📝️PW-01.md`, `K/📝️PW-02.md`, `K/📝️PW-05.md`
- `K/🧰️scripts/finish-a/{sweep.sh,check.sh,val.sh,guard.sh,guard.ts,where.ts,measure.ts,look.sh,compose.ts,rows.py,facts.py,proofs.py,reports.py,notes.json}`
- `K/🧰️scripts/PW-09/candidate.sh`
- `U/plants-a.md`

Updated:

- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/{horse-chestnut,european-beech,common-ash,norway-spruce,european-larch,atlas-cedar,fig-tree}/🔣️.json`
- `K/📥️manifest-rows/{horse-chestnut,european-beech,common-ash,norway-spruce,european-larch,atlas-cedar,fig-tree}.json` (rewritten by the generators; same values except what the assets changed)
- `K/🧰️scripts/PW-01/asset.ts`
- `K/🧰️scripts/PW-02/{kit.ts,build.ts,european-beech.ts}`
- `K/🧰️scripts/PW-05/assemble.ts`
- `K/🧰️scripts/PW-06/{plant-kit.ts,black-alder.ts}`
- `K/🧰️scripts/PW-07/{whorled.py,atlas_cedar.py}`
- `K/🧰️scripts/PW-08/shared-geometry.ts`
- `K/🧰️scripts/PW-09/{morph.py,fig-floors.json}`
- `K/📝️PW-03.md`, `K/📝️PW-04.md`, `K/📝️PW-06.md`, `K/📝️PW-07.md`, `K/📝️PW-08.md`, `K/📝️PW-09.md` (section "Finisher completion" appended)
- `K/🗂️work-board.md` (rows PW-01 to PW-09)
- `U/requests.md` (one section)

Removed:

- `K/🗑️generated/{finish-a,PW-01,PW-02,PW-05,PW-06,PW-07,PW-08,PW-09}` (tool output of this session: candidates, logs, preview PNGs)

Not done by design: `merge-manifest.py` was not run, `🗂️catalog/🔣️.json` was not touched, no ticket or goal was opened or closed, no shared engine, schema, validator, test or site file was edited.

## For the umbrella coordinator

- The 27 rows are ready to merge. `merge-manifest.py` lists all 27 ids of this scope in its collections.
- Two interruptions cost time: the first proof sweep ran under a host load of 13 to 23 (5 to 16 minutes per asset), and the session restart at 22:20 killed a running validation.
- Shared-code requests still open from this scope: one, in `requests.md` (measures of the catalog silhouette test).
