# Finisher Report — Used Park Objects (`USED-PARK-OBJECTS-STAFFAGE-COLLECTION`)

Finishing session of 2026-11-22, 20:23 to about 21:55. Scope: the 29 started assets (rows 01–29). Rows 30–40 were never started and stay so.

## Result

- All 29 started assets: full proof `VALID` (schema 0, validateAsset 0, checkVolumes 0), check CLI `✓`, catalog copy byte-equal to the candidate, manifest row equal to the recomputed row, member of `used-park-objects` (29 members); every catalog copy `VALID` with `--catalog-only`.
- `used-bicycle-shelter` (row 29, interrupted at publish) is published: manifest row inserted (manifest 276 → 277 entries).
- 14 assets corrected in the review pass and re-published through `publish.ts`; 15 unchanged.
- No author report existed anywhere (`K/reports/` was never created; the authors' sessions ended before writing them). 29 reports were written from the documents and from this session's measurements: `K/reports/<slug>.md`.
- All 29 author scripts regenerate their candidate byte-identically (checked in a scratch copy before any edit); every correction was made in the author script and the document regenerated.

## Per asset

Before (20:27–20:55, measured): every row 01–28 `VALID` / `✓` / catalog = candidate / row present; row 29 `VALID` / `✓` / catalog = candidate / no row. After:

| row | slug | final proof line | check | changed |
|---|---|---|---|---|
| 01 | used-backrest-park-bench | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-backrest-park-bench (1/1) | no |
| 02 | used-concrete-bench-block | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-concrete-bench-block (1/1) | no |
| 03 | used-tree-surround-bench | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-tree-surround-bench (1/1) | yes |
| 04 | used-park-armchair | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-park-armchair (1/1) | no |
| 05 | used-sun-lounger-bench | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-sun-lounger-bench (1/1) | no |
| 06 | used-beer-garden-set | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-beer-garden-set (1/1) | no |
| 07 | used-concrete-table-tennis | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-concrete-table-tennis (1/1) | yes |
| 08 | used-chess-table | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-chess-table (1/1) | yes |
| 09 | used-litter-bin-post | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-litter-bin-post (1/1) | no |
| 10 | used-dog-waste-station | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-dog-waste-station (1/1) | yes |
| 11 | used-recycling-bin-cluster | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-recycling-bin-cluster (1/1) | no |
| 12 | used-park-lantern | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-park-lantern (1/1) | yes |
| 13 | used-bollard-light | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-bollard-light (1/1) | no |
| 14 | used-double-swing | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-double-swing (1/1) | yes |
| 15 | used-slide-tower | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-slide-tower (1/1) | yes |
| 16 | used-seesaw | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-seesaw (1/1) | yes |
| 17 | used-spring-rocker | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-spring-rocker (1/1) | yes |
| 18 | used-playground-roundabout | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-playground-roundabout (1/1) | no |
| 19 | used-sandpit-digger | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-sandpit-digger (1/1) | yes |
| 20 | used-climbing-net-pyramid | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-climbing-net-pyramid (1/1) | no |
| 21 | used-outdoor-fitness-station | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-outdoor-fitness-station (1/1) | yes |
| 22 | used-basketball-hoop | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-basketball-hoop (1/1) | no |
| 23 | used-football-goal | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-football-goal (1/1) | no |
| 24 | used-pump-drinking-fountain | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-pump-drinking-fountain (1/1) | yes |
| 25 | used-water-play-table | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-water-play-table (1/1) | no |
| 26 | used-bubbler-stone-fountain | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-bubbler-stone-fountain (1/1) | no |
| 27 | used-park-noticeboard | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-park-noticeboard (1/1) | no |
| 28 | used-wayfinding-signpost | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-wayfinding-signpost (1/1) | yes |
| 29 | used-bicycle-shelter | VALID schema 0, validateAsset 0, checkVolumes 0 | ✓ used-bicycle-shelter (1/1) | yes |

## What was changed

**03 used-tree-surround-bench**
- Lifting pose corrected: the pair of hands at the outer slat ends of segment 1 is an applied force of the statics of the lifted part alone instead of a support that shared the weight of the whole rig (179 N per hand before, 63.6 N per hand after; `lift_statics.py`, `reactions.ts`). Key pose `hands-on` added to the clip `segment-lift-out`.

**07 used-concrete-table-tennis**
- Footing treatment made the same as rows 01 and 05: the two strip footings are massless bones `footing-a`, `footing-b` with body, thermal (`footings` region, 7.52 m2) and structure solids at all four scales and filled cut shapes in elevation and front (before: thin ground outlines in 2D only at 1:50 and 1:100, nothing in 3D); they stay behind in `maintenance`. A straight ground line was added at 1:200 and 1:500; the front hollow line that would cross the footing was removed.
- Footing depth set to the frost-free 0.80 m (author: 0.40 m). `dimensions.heightM` is now 1.7125 m (0.9125 m above ground plus 0.80 m of footing), as the product's extent rules demand; the true-extent symbol and `distinction.json` follow.
- With 0.40 m depth the full proof reported `volumes-silhouette` 1:50 front p95M 48.8 mm; the finding is an artefact of the banded raster (it disappears at 0.30, 0.50, 0.60 and 0.80 m with nothing else changed; request filed).

**08 used-chess-table**
- Lifting pose corrected: the pair of round slings is an applied force of the statics of the lifted part alone instead of a support that shared the weight of the whole rig (3269.7 N on each of two slings and two soil contacts before, 1222.4 N per sling, soil 2 x 5316.9 N after; `lift_statics.py`, `reactions.ts`). Key pose `slings-taut` added to the clip `relevel`; lifting anchors are `load` anchors; reference EN 1492-2 added.

**10 used-dog-waste-station**
- Bag slot lowered from 1.16 m to 1.05 m (bag tab gripped at 1.01 m) so that it lies inside the reach range of 0.85 to 1.05 m of DIN 18040-1 that the asset itself cites; the whole dispenser (housing, mount, clamps, lid, roll, tab, lock, strokes, anchors, bones) moved down by 0.11 m at all four scales.
- Consequence: the rain hood of the bin can open only 45 degrees under the dispenser (20 mm clear) instead of 85; joint limit, the poses `maintenance` and `lids-open` and their texts changed accordingly.

**12 used-park-lantern**
- Lifting pose corrected: the lawn mat under the eave of the lowered column is an applied force of the statics of the lifted part alone instead of a support that shared the weight of the whole rig (224.7 N on the mat before, 197.4 N on the mat by moments about the hinge pin, foundation 607.4 N after; `lift_statics.py`, `reactions.ts`).

**14 used-double-swing**
- Lifting pose corrected: the resting point of the unshackled flat seat is an applied force of the statics of the lifted part alone instead of a support that shared the weight of the whole rig (293.2 N under the seat before, 38.7 N under the seat, the weight of the seat after; `lift_statics.py`, `reactions.ts`).

**15 used-slide-tower**
- Lifting pose corrected: the four tie beam ends of the dismantled roof (crane sling, then lawn) is an applied force of the statics of the lifted part alone instead of a support that shared the weight of the whole rig (1155 to 1344 N per rest before, 230.4 N per rest after; `lift_statics.py`, `reactions.ts`). Key pose `roof-on-crane` added: in the clip `dismantle-roof` the crane now carries the roof from 1 s on instead of the tower supports.

**16 used-seesaw**
- Lifting pose corrected: the pair of round slings on the beam is an applied force of the statics of the lifted part alone instead of a support that shared the weight of the whole rig (233.0 N on each of five supports before, 380.6 N per sling, 134.6 N on each of footing and tyres after; `lift_statics.py`, `reactions.ts`).

**17 used-spring-rocker**
- Lifting pose corrected: the four-leg sling is an applied force of the statics of the lifted part alone instead of a support that shared the weight of the whole rig (510 to 546 N per leg before, 50.1 N per handle leg and 82.5 N per rear leg, 265.2 N in all, footing 2424.8 N after; `lift_statics.py`, `reactions.ts`).
- Real defect found by the statics: both sling pairs (handle ends x 0.27 m, footrest ends x 0.13 m) lay ahead of the centre of mass of the lifted part (x 0.027 m), so the rocker would have tipped backwards on the hook. The rear pair is now choked round the rear spacer under the seat (x -0.1205 m); anchors `lift-foot-*` renamed `lift-rear-*`; key pose `slings-taut` added.

**19 used-sandpit-digger**
- Lifting pose corrected: the crane hook in the eye nut is an applied force of the statics of the lifted part alone instead of a support that shared the weight of the whole rig (4132.6 N on the hook before, 276.1 N on the hook after; `lift_statics.py`, `reactions.ts`). Key pose `hook-taut` added to the clip `maintenance-lift`.

**21 used-outdoor-fitness-station**
- Lifting pose corrected: the pair of workers' hands under the high bar is an applied force of the statics of the lifted part alone instead of a support that shared the weight of the whole rig (2926.2 N and 3092.5 N on the hands before, 14.0 N per worker after; `lift_statics.py`, `reactions.ts`). Key pose `hands-on` added to the clip `bar-change`.

**24 used-pump-drinking-fountain**
- Lifting pose corrected: the crane hook on the pump head is an applied force of the statics of the lifted part alone instead of a support that shared the weight of the whole rig (5263.5 N on the hook before, 399.6 N on the hook, slab 10590.5 N after; `lift_statics.py`, `reactions.ts`). Key pose `sling-taut` added to the clip `maintenance`.

**28 used-wayfinding-signpost**
- Lifting pose corrected: the pair of hands under finger 2 is an applied force of the statics of the lifted part alone instead of a support that shared the weight of the whole rig (840.5 N and 807.6 N on the hands before, 6.6 N per hand, footing 2543.4 N after; `lift_statics.py`, `reactions.ts`). The hands were moved to lie either side of the finger's centre of mass (local x 0.20 and 0.413 m instead of 0.30 and 0.52 m, which put 97 % on one hand); key pose `hands-under` added.

**29 used-bicycle-shelter**
- Published for the first time (manifest row inserted, collection now 29 assets); the author had stopped after writing `distinction.json`.
- Lifting pose corrected: the hold of the two workers on the slid-out sheet is an applied force of the statics of the lifted part alone instead of a support that shared the weight of the whole rig (147 N per hand before, 17.7 N per hand, a quarter of the 7.2 kg sheet each, its rear edge on the glazing bars after; `lift_statics.py`, `reactions.ts`).

Decisions taken:

- The ledger's pattern "crane hook as applied lifting force — reuse for 01/05/07/08/17" was checked by measurement first (`reactions.ts`): rows 01, 05 and 07 (and 02, 04, 13, 22) were already exact because their footings are massless or their lifted part is the whole rig, so they were left alone; the pattern was needed for 08 and 17 and for ten more rows whose ledger lines named the same solver effect or showed it in the measurement (03, 12, 14, 15, 16, 19, 21, 24, 28, 29).
- Row 17: the statics showed that all four sling points lay ahead of the centre of mass; the rear pair was moved (a real defect, not only a solver effect).
- Footings 05 against 07: 07 now models its strip footings like 01 and 05 (massless bones, solids at every scale, cut shapes). Rows 14 and 29 keep their deep foundations out of the model (below 300 mm of gravel, under paving: neither visible nor cut), as their assumptions state.
- Row 10: the dispenser was lowered rather than the claim reworded; the price is a hood that opens 45 degrees.
- Sheets: every final document was rendered (about 20 three-quarter renders and four 2D drawings per asset) and looked at as contact sheets; the 3D contact sheet of all 29 was opened, the 2D contact sheet of 12 (rows 01, 02, 06, 07, 08, 10, 13, 14, 20, 22, 26, 29, among them every asset whose 2D changed). The 2D sheets of the other 17 were rendered but not opened; six-view sheets at full resolution were not opened one by one. Each report says which.
- The proof sweeps ran as one background shell loop with one tool process at a time (a foreground call is cut after ten minutes); nothing else was run beside them.

## Files

Created (repo-relative to the ticket folder `K` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/USED-PARK-OBJECTS-STAFFAGE-COLLECTION`):

- `K/reports/<slug>.md` for the 29 started slugs
- `K/verify.ts` (candidate / catalog / manifest comparison, read-only), `K/prove.sh` (full proof and check CLI in sequence), `K/reactions.ts` (support reactions per pose), `K/lift-loads.ts` and `K/lift_statics.py` (statics of a lifted part), `K/sheets.sh` and `K/contact.ts` (review contact sheets), `K/review-notes.py` and `K/review-notes.json`, `K/report.py`, `K/reactions-measured.txt`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json`: one row inserted and 14 rows replaced, only through `K/publish.ts`
- this report and the sections `## USED-PARK-OBJECTS-STAFFAGE-COLLECTION — …` appended to `U/requests.md`

Updated:

- `K/wave.md` (one true row per asset), `K/requests.md` (carry-over note)
- `K/assets/used-tree-surround-bench/` author script(s) and `🔣️.json`; `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/used-tree-surround-bench/🔣️.json`
- `K/assets/used-concrete-table-tennis/` author script(s) and `🔣️.json`; `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/used-concrete-table-tennis/🔣️.json`
- `K/assets/used-chess-table/` author script(s) and `🔣️.json`; `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🛋️furniture/used-chess-table/🔣️.json`
- `K/assets/used-dog-waste-station/` author script(s) and `🔣️.json`; `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/used-dog-waste-station/🔣️.json`
- `K/assets/used-park-lantern/` author script(s) and `🔣️.json`; `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/used-park-lantern/🔣️.json`
- `K/assets/used-double-swing/` author script(s) and `🔣️.json`; `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/used-double-swing/🔣️.json`
- `K/assets/used-slide-tower/` author script(s) and `🔣️.json`; `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/used-slide-tower/🔣️.json`
- `K/assets/used-seesaw/` author script(s) and `🔣️.json`; `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/used-seesaw/🔣️.json`
- `K/assets/used-spring-rocker/` author script(s) and `🔣️.json`; `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/used-spring-rocker/🔣️.json`
- `K/assets/used-sandpit-digger/` author script(s) and `🔣️.json`; `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/used-sandpit-digger/🔣️.json`
- `K/assets/used-outdoor-fitness-station/` author script(s) and `🔣️.json`; `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/used-outdoor-fitness-station/🔣️.json`
- `K/assets/used-pump-drinking-fountain/` author script(s) and `🔣️.json`; `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/used-pump-drinking-fountain/🔣️.json`
- `K/assets/used-wayfinding-signpost/` author script(s) and `🔣️.json`; `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/used-wayfinding-signpost/🔣️.json`
- `K/assets/used-bicycle-shelter/` author script(s) and `🔣️.json`; `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🪑️objects/used-bicycle-shelter/🔣️.json`
- `K/assets/used-concrete-table-tennis/distinction.json`; changed scripts in detail: `author-chess-table.py`, `author-spring-rocker.py`, `author-pump.py`, `author-wayfinding-signpost.py`, `author-fitness-station.py`, `author-sandpit.py`, `author-seesaw.py`, `author-tree-bench.py`, `author-bicycle-shelter.py` + `author_text.py`, `author-double-swing.py` + `author_text.py`, `author-slide-tower.py`, `author-lantern.py`, `author-used-dog-waste-station.py` + `author-texts.py` + `author_scales.py` + `author_drawings.py`, `author-build.py` (table tennis)

Removed: `K/🗑️generated/` (proof, check, publish and reaction outputs, contact sheets) and the session's renders `used-*.png` under the tool ticket's `🗑️generated/r3-sheet/file` and `r3-2d/file`. Nothing else was deleted.

## What stays imperfect

- Lifting gear, workers and users are not drawn in any asset; lifted parts hang in the air (schema: no per-pose presence of solids; request filed).
- The applied-force pattern makes the reactions exact but shows the lift as a load, not as a support; bodies that only lie beside each other (bicycle under the shelter, sand and edging and digger socket of the sandpit) are still solved as one rigid body on springs of equal stiffness (request filed).
- No reference table of an author exists; the reports state the document's own numbers and could not check them against outside sources (the 3 % rule of the wave brief is unverified for all 29).
- `used-concrete-table-tennis`: `heightM` 1.7125 m includes 0.80 m of footing; the footing depth 0.40 m of the author could not be kept because of a raster artefact of `volumes-silhouette` (request filed), 0.80 m is the frost-free depth.
- `used-dog-waste-station`: hood opening limited to 45 degrees. `used-sandpit-digger`: eye nut 0.12 m beside the centre of mass. `used-pump-drinking-fountain`: lifting point 32 mm beside it.
- Ledger points of the wave that were not re-worked (approximate front views of tilted poses in rows 09 and 12, rigid nets in 20 and 22, static water in 25, dense 1:500 net in 20, text-only lawn hollows in 16) are listed per row in `K/wave.md` and in the reports, marked as taken from the ledger where the finisher did not re-measure them.
- 17 of the 29 2D contact sheets were not opened by the finisher.

## Never started

30 `used-ebike-charging-pillar`, 31 `used-pram-gate-barrier`, 32 `used-chain-post-fence`, 33 `used-picket-playground-fence`, 34 `used-pergola`, 35 `used-park-pavilion`, 36 `used-raised-garden-bed`, 37 `used-compost-bins`, 38 `used-insect-hotel`, 39 `used-wheelbarrow`, 40 `used-leaf-collection-cart`.

## Shared-code requests still open (in `U/requests.md`)

- structure solver: a lifted, carried or set-down part shares the weight with the footing supports (measured on twelve assets)
- volumes-silhouette: banded raster reports the top bin when only the frame height changes (measured on `used-concrete-table-tennis`)
- no per-pose presence of solids and shapes
- volumes-silhouette with slender curved tubes (`followedWidth`), carried over, not re-measured
- open detail strokes on a part that a pose turns over, carried over, not re-measured
- manifest rows can be lost under the replica (one derived manifest writer)
