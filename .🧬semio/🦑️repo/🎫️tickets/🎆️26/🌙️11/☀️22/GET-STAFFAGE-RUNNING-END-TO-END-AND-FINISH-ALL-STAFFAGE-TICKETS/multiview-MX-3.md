FINAL (2026-11-26 09:10; Job 1 done 2026-11-25 23:35, Job 2 done 2026-11-26 08:41)

# 🔭️ Multiview: Engine and Suite Owner MX-3 (2026-11-25 to 26, Sonnet 5.5)

Paths: `R` repo root, `S` = `R/🎓️teaching/🏛️architecture/🧍️staffage`, `U` this ticket folder, `K` the multiview ticket of 2026-11-02. Scripts: `U/🧰️matrix/MX-3/`. Every run through `repo capacity run` (one request at a time, 900 to 1800 MB), logs in `U/🗑️generated/matrix/MX-3/` (kept until Job 2 ends).

## Job 1

| Item | Result | Evidence (logs read to their end) |
|---|---|---|
| `🧊️model-bounds` | 82 of 82 (was 42), test-side, D62 to D64 | `vt-bounds-1.log`, 23:29, peak 661 MB |
| `🧊️chain-state` | 20 of 20 (was 18), test-side, D65 | `vt-chain-1.log`, 23:26, peak 1,139 MB |
| `🕳️fill-rule` | 10 of 11: the one red case is the catalog case with the 8 `tram` entries (MU has not landed `tram`) | `vt-bounds-2.log`, 23:29 |
| MV-L1 (coordinator): loft of the side elevation as depth twin | engine repaired, D66, new suite `🧊️loft-depth` 5 of 5 (4 of 5 red on the module before) | `vt-engine-*.log` 23:24 (`🧊️loft-shape` 29 of 29, `🧊️loft-depth` 5 of 5), `vt-loftold-1.log` (red before) |
| MH-2 (coordinator): 15 mm shorter lofts at 1:1000 | expected band behaviour of the paper simplification, D67; the repair that keeps the extremes is prepared and proven but not landed (changes the bytes of most drawings) | `U/🧰️matrix/MX-3/extremes/` |

### `🧊️model-bounds`

The 40 red cases were all caused by the cases (ML-4). The 33 band cases state per band what the body resolves and name the declaration that says so (`said` reads `volumes.<scale>.decisions` and fails when the band no longer states it); 1:50 keeps the old assertion. Absent parts are asserted absent, every selection has a count or existence guard. Measured with `ids.ts`, `world.ts`, `ray.ts`, `repeats.ts` against the catalog documents; reviewed with `ML-4/soft.ts` (recording assertions): the first run of the rewritten suite had 2 red cases (the brick wall bands that touch, the through rollers of the door at 1:200: both measured, corrected in the tables), then 82 of 82. (b) 2 cases measured in the old frame: heat pump fan about its own bone axis; trolley wheels in the model frame through the node chain. (c) 5 pinned ids: compressor mounts by role (`part-support`, exactly four; `grommets`, exactly one) with a guard on the foot plate; street lamp lock and catch at 1:50 in the body and thermal list only. Details and reasons: D62, D63, D64 in `K/🗂️work-board.md`.

### `🧊️chain-state`

`sagittalRoot` (the first abducted or twisted bone from the pelvis down) is the bone whose `z` moves a limb sideways. Case 1: clavicles, 0.756 m for 0.4. Case 2: side elevation unchanged in every pose (1e-6), height of the front and ahead extent of the plan unchanged, lateral extents move (more than 0.05 m): a flat shape cannot be compensated for a lateral offset of its bone, only a revolved solid. D65. Two scenarios were added to `🥒️.feature`.

### MV-L1, the engine change

`🔨️modules/🧮️evaluation/🟦️.ts`: `restElevation` and the twin map of `projection` (written 23:10 to 23:15 on 2026-11-25). A `loft` of the elevation is a depth twin of its bone; a `solid` is not (it would have moved 34 of 58 candidate assets by up to 17 cm). Five assets move with the loft: `mallard-duck` 0.9 mm, `office-worker-carrying-laptop` 24 mm, `person-with-rollator` 13 mm, `seated-office-worker` 21 mm, `surveyor-with-total-station` 28 mm; the other 53 candidates are identical ring for ring, the 317 others cannot be reached. Suite `🧪️tests/🧊️loft-depth` (+ `🥒️.feature`): carries a flat front/plan stroke without a twin as the flat rest outline of the same loft does (1e-9, every pose), and moves it away from the bone-origin depth (so the twin is read). Announced in `requests.md` (answer under "multiview MV-L1").

### MH-2 (paper box of lofts at 1:1000)

Not a loft defect: the evaluation is identical at both denominators; the paper stage simplifies every ring to 0.01 mm of paper (10 mm of the object at 1:1000) and cuts the tip. D67. The extreme-keeping pass (and its suite `✂️paper-simplify`, 10 of 10, shapely as oracle) is in `U/🧰️matrix/MX-3/extremes/`; the engine and tests in the repo are as before.

### MH-4 hands (coordinator, 2026-11-26)

Asset side, D68: the elevation draws one hand plate to u 0.176 m where front and plan draw palm and fingers on separate bones; recipe measured on a trial copy (`🧰️matrix/MX-3/guide-hands.py`): gate 0 hard, validate 0, front 15.0 / plan 13.5 mm unchanged, elevation 18.5 mm against 15 (3.5 mm open for ML-5; leaving out one finger: index, middle or ring 18.5 mm, little finger 23.5 mm, so the open point is not at those three). Answer in `requests.md`.

### Polygon union at small bead spacing (coordinator, 2026-11-26)

No defect in `unionPolygons` (D69): shapely agrees in area and extent on the real bodies of `hawthorn` at 4 degrees (126 evaluations) and on 332 synthetic and random cases; the short extent is in the sections of the loft tool. Guard added to `🧪️tests/🔺️polygon-union` (44 of 44). No engine change; answer in `requests.md` ("multiview MX-3 — polygon union at small bead spacing").

### `black-alder` / `field-maple` silhouette guard (MV-L1 request, coordinator 2026-11-26)

Located (`🧰️matrix/MX-3/guard-where.ts`): two sub-pen holes (15 x 7 cm, 0.75 x 0.35 mm on paper at 1:200) of the guard's reference that the loft drawing covers; no channel and no hole in the drawing. Cured by the pocket rule (trial module: 16.8 mm against 1207.5 mm on the `fol-8` copy). Whole-tree copies at 1:50 and 1:100 not proven. D70; answer in `requests.md`. Landing the pocket rule is therefore also the cure the two trees wait for.

## Order to land the pocket rule (prepared; waits for the coordinator's go)

1. Preconditions: no catalog `🔣️.json` written in the last 10 minutes and no live lane process with `--write`; nobody editing `S/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`.
2. Before: `🧊️catalog-silhouette` whole (385 cases, 1600 to 2500 MB, `vt.sh before 2500 🧊️catalog-silhouette`) and `python3 🧰️matrix/MX-2/pocket/test.ts` equivalent (`bun pocket/test.ts` on the live module: 6 pass); keep both logs.
3. Land: `python3 🧰️matrix/MX-2/pocket/patch.py --land` (refuses unless every anchor is found exactly once); run it a second time (must change nothing); note the time in `requests.md`.
4. After: `bun pocket/test.ts` on the live module (14 of 14), `🧊️catalog-silhouette` whole again; per-case diff against step 2: wanted 0 pass to fail, and the fail to pass cases listed (white cane 1:500 front, bentwood chairs 1:500, the cases of MH-4 and MV-2).
5. Dependents: `tsc` of the staffage package; `✅️asset-validation`, `🧊️three-views` and `🌡️catalog-physics` are the other readers of `measureAsset`: run `✅️asset-validation` whole.
6. `person-with-white-cane` (56 cells): scan, `MR/measure.ts`, check CLI, then `📊️matrix.ts --fresh --jobs 1 --assets person-with-white-cane` (800 MB, `timeout` 3600 s).
7. Answer the requests that wait on it (MH-4 small hole, MV-2 pocket, MU slender solid, ML-3 `fol-6`) with the numbers of steps 2 to 4.

## Job 2 (done)

Landed 2026-11-26 **08:34:13** (`patch.py --land`, second run "already landed", md5 `57764b63…`; module before: `d8fa2805…`, kept as `🧰️matrix/MX-3/extremes/silhouette-before-landing.ts.txt`). One patch of three changes: pocket rule (`agreedPockets`), `sealRings`, body-following rule (`drawsBody`, MU's chairs at 1:500).

| Proof | Result |
|---|---|
| `🧊️catalog-silhouette` whole, before | 385 of 385 (08:30, peak 2,918 MB; the first attempt was cut by the 3000 s timeout after 126 cases) |
| same, after (trial copy of the module, identical to the landed one but for imports) | 385 of 385 (08:32, peak 2,738 MB): no verdict changed, nothing of the eight assets of MV-L2/MV-L3 to exclude |
| `pocket/test.ts` on the live module | 14 of 14 (6 of 14 before) |
| tsc, staffage package | exit 0, 0 errors |
| `✅️asset-validation` | 85 of 85 |
| matrix `person-with-white-cane` | 252 cells, 175 ok, 0 fail |
| matrix `used-bentwood-dining-chairs` | 252 cells, 230 ok, 22 fail (the 22 of MU's report: 1:500 gate findings; its cure now passes the guard in the trial, 0.9464 against 0.92) |

Requests answered: the announcement `## multiview MX-3 — pocket rule live 08:34`, `## multiview MV-L1 — silhouette guard reads 1207 mm …` (located: sub-pen holes of the reference, cured by the rule), `## multiview ML-3 — loft silhouette fol-6`. Open for others: the chairs (run the 1:500 cure on a copy and the guard first), whole-tree copies of `black-alder`/`field-maple` and `rowan` 1:50 plan on a copy with the rule live.

## Decisions (K/🗂️work-board.md)

D62 to D70: band-resolution tables of `🧊️model-bounds`; frames; compressor and lamp selection; chain-state roots; loft depth twin; paper simplification as expected behaviour; guide-dog hands recipe (3.5 mm open for ML-5); union exact at small bead spacing; sub-pen reference holes.

## Files

Updated: `S/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts` (the landing), `S/🧪️tests/🔺️polygon-union/🟦️.ts`, `🐍️.py`, `🥒️.feature` (bead masses), `S/🧪️tests/🧊️model-bounds/🟦️.ts`, `S/🧪️tests/🧊️chain-state/🟦️.ts`, `🥒️.feature` (chain-state), `S/🔨️modules/🧮️evaluation/🟦️.ts`, `K/🗂️work-board.md` (D62 to D70), `U/requests.md` (answers and the announcement).
Created: `S/🧪️tests/🧊️loft-depth/` (`🟦️.ts`, `🥒️.feature`), `U/🧰️matrix/MX-3/` (`env.sh`, `q.sh`, `vt.sh`, `soft.sh`, `ids.ts`, `world.ts`, `ray.ts`, `repeats.ts`, `affected.py`, `compare.ts`, `lofts-edge.ts`, `simplify-compare.ts`, `evaluation-before-mvl1.ts.txt`, `extremes/`), `U/multiview-MX-3.md`.
