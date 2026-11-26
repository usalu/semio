FINAL (2026-11-25 22:50: two assets repaired with all proofs; `🕳️fill-rule` 10 of 11 with one red case left that waits for lane MU (`tram`); `🧊️chain-state` 18 of 20 and `🧊️model-bounds` 42 of 82 are NOT green: every red case there is caused by the case, not by an asset, and is with MX-2 as three measured requests)

# Multiview ML-4 — The Red Cases of `🧊️model-bounds`, `🕳️fill-rule` and `🧊️chain-state`

Scope: the 40 red cases of `🧊️model-bounds` (lane VOL), widened by the coordinator to the catalog case of `🕳️fill-rule` and the two red cases of `🧊️chain-state` (MX-2, "Suites"). A result below counts only where its log ran to the end in this session. No test, fixture, engine file, `T/📊️matrix.ts` or manifest was edited.

## 1. Result

| Suite, run to its end (22:44 to 22:47, one grant) | Result | Peak (`time -l`, worker) | Open |
|---|---|---|---|
| `🕳️fill-rule` | 10 of 11 (15 overlaps before, 8 now) | 608 MB | the 8 entries of `tram`: repair ready, owner MU is IN WORK, request filed |
| `🧊️chain-state` | 18 of 20, as before | 1,193 MB | both cases are test-side, request for MX-2 |
| `🧊️model-bounds` | 42 of 82, as before (first run 158 s, peak 542 MB; second 605 MB) | 605 MB | all 40 cases are test-side (D52 kind), request for MX-2 |

Reservation measured for these suites as a whole: `🧊️model-bounds` and `🕳️fill-rule` 900 MB (peak x 1.1 x 1.3), `🧊️chain-state` 1750 MB.

## 2. Repaired Assets (`🕳️fill-rule`)

Cause in all three assets: one filled path holds subpaths that cover each other; under the even-odd rule the doubly covered area is painted as a hole. Script `🧰️matrix/ML-4/fill-rule.py` (idempotent; replaces the `d` strings and nothing else; second run `unchanged`).

| Asset (owner) | Before | Repair | After |
|---|---|---|---|
| `park-bench` (MB-1, ended) | 1:200 and 1:500 plan `three-back-slats`: 3 overlaps each, 0.013, 0.041, 0.054 m2 (three reclined slats seen from above) | the path is their outline, one ring 0.1815..0.2658 m | 0 overlaps; plan silhouette IoU 0.9313 to 0.9944, largest distance 56.3 to 14.1 mm at both scales |
| `seated-office-worker` (MH-5, FINAL) | 1:200 elevation `chair-tyre-front`: inner ring crosses the outer, 0.001051 m2 | inner ring is the outer pentagon at 0.85 about its centroid (the 3.75 mm tyre wall, as the ring of 1:100) | 0 overlaps; silhouette, validate, sweep identical to before |
| `tram` (MU, IN WORK) | 1:50 and 1:100 elevation `bogie-legs-f`, `-r`: 8 overlaps, 0.0002 and 0.0008 m2 (legs over cross bar) | one U-shaped ring; NOT written, request "multiview ML-4 — for MU" | trial copy: 0 overlaps, gate 0 hard, validate and sweep unchanged, elevation IoU 1:100 0.9987 to 0.9988, prints; matrix, check CLI and suites not run (they read the catalog) |

Proofs on the catalog documents (`🧰️matrix/ML-4/prove.sh`, one grant, 22:33 to 22:42):

| Proof | `park-bench` (`e92de8def49c252b`) | `seated-office-worker` (`a8bcdbb55d4507fc`) |
|---|---|---|
| Authors' gate `T/🔎️scan.ts` | clean, 0 hard findings | clean, 0 hard findings |
| Fixer's gate `MR/gate.ts --twins` | 0 hard | 0 hard |
| Silhouette guard, `validateAsset`, three-views sweep | OK, 0 issues, 0 findings | OK, 0 issues, 0 findings (same as the backup) |
| Prints | every authored scale prints | every authored scale prints |
| Check CLI | `✓` (1/1) | `✓` (1/1) |
| `🧊️catalog-silhouette` | 2 passed | 1 passed |
| `🧊️three-views` | 4 passed | 2 passed |
| `🌡️catalog-physics` | 4 passed | 3 passed |
| `🫁️breathing-catalog` | no case of the asset (290 skipped) | 3 passed |
| `✅️asset-validation` | 10 passed | 10 passed |
| `⚡️asset-power` | no case (24 skipped) | no case (24 skipped) |
| Matrix `📊️matrix.ts --fresh --jobs 1` | 252 cells, 252 ok, 0 fail | 252 cells, 28 ok, 224 `group-drift` (reported, as before), 0 fail |
| Sheets opened (before and after) | 1:200 and 1:500 `unloaded`: the back reads as one rail in the plan, elevation and front unchanged | 1:200 `typing`: unchanged to the eye (a 50 mm castor) |

The `tram` trial sheet at 1:50 (`running`) was opened too: the bogies read as before.

## 3. `🧊️model-bounds`: the 40 Red Cases

Every assertion measured with `🧰️matrix/ML-4/soft.ts` (the suite source with recording assertions) and `solids.ts`; lists and `decisions` read from the documents. Full table with numbers per case in `requests.md`, "multiview ML-4 — `🧊️model-bounds`: 40 of 82 cases …". No asset defect was found; no asset was changed for this suite.

| Cause | Cases | Assets |
|---|---|---|
| (a) The band declares in `volumes.<scale>.decisions` the simplification that `volumes-feature` asks for (7.5, 15, 30, 75 mm); the case asks the 1:50 detail of every band (the D52 verdict of the heater) | 33: vacuum 100, 200, 500; laminate, door gear, masonry 100, 200, 500; canopy, rollers, lintel 100, 200, 500; glass cavity 100, 200, 500; boiler spigot 100, 200, 500; ETICS 100, 200, 500; balcony 100, 200, 500; fin stack 100, 200, 500; radiator 200, 500; heat pump louvres 100, 200, 500; generator 500; street lamp 100, 200, 500 | `vacuum-cleaner`, `glass-balustrade`, `sliding-door`, `solid-brick-wall-1960`, `tied-canopy`, `sectional-garage-door`, `masonry-wall-with-lintel`, `curtain-wall-bay`, `gas-boiler`, `etics-brick-wall-12`, `-30`, `cantilever-balcony`, `finned-heat-exchanger`, `panel-radiator`, `heat-pump-outdoor-unit`, `diesel-generator`, `street-lamp` |
| (b) The case measures in the frame or under the node of an earlier rig; the part has its own bone now | 2 (and the blade count inside the three heat pump cases of (a)): heat pump 1:50 (blades on node `fan`, measured about y 0.8, z 0.15 of `unit`: 2 thirds for 3); trolley 1:500 (wheels on `wheel-*` nodes, the case reads `chassis`) | `heat-pump-outdoor-unit`, `shopping-trolley` |
| (c) The case pins solid ids the structure list does not use | 5: compressor 50, 100, 200, 500 (`/grommets|contact-pad|^feet$/` against `grommet-<corner>-pad`; the four bored rubber mounts exist); street lamp 1:50 (`joint-door-lock`, `joint-head-latch` are not part of the load path the structure list states; body and thermal pass) | `scroll-compressor`, `street-lamp` |

33 + 2 + 5 = 40. The three heat pump cases of (a) carry cause (b) as well.

## 4. `🧊️chain-state`: the 2 Red Cases

Measured with `chain.ts` and `chain-variant.ts`; request "multiview ML-4 — `🧊️chain-state` …". Both cases move a limb root sideways by its local `z`. In `person-with-umbrella` (MH-4) and `person-on-crutches` (MH-3) the limb roots hang on abducted and twisted clavicle and ilium bones (the anatomical rig), so the local `z` is not the lateral axis: depth difference 0.127 for more than 0.4 (0.756 with the offset on the clavicle); 22 moved edges, largest 0.1486 m (`tripod-stand` elevation minX), 0 in the elevation with the offset on clavicle and ilium. The second case also compensates revolved solids only, and the person is drawn with flat shapes now. Not an asset defect; no asset changed.

## 5. Requests Filed (`requests.md`)

1. `multiview ML-4 — 🧊️model-bounds: 40 of 82 cases …` (MX-2; D52 for the rest of the suite, or the dev's word that the 1:50 detail is wanted in every band).
2. `multiview ML-4 — for MU: vehicles/tram, the four overlaps …` (run `fill-rule.py tram`, then the proof list).
3. `multiview ML-4 — 🧊️chain-state: the two red cases …` (MX-2).

## 6. Open

- `🧊️model-bounds` 40 red, `🧊️chain-state` 2 red: wait for MX-2 (or the dev's decision on D52).
- `🕳️fill-rule` 1 red (8 entries, all `tram`): waits for MU.
- Note for the document owners: five 1:50 `decisions` still say a detail "is required at every scale" although their coarser bands leave it out by declaration (`curtain-wall-bay`, `masonry-wall-with-lintel`, `tied-canopy`, `etics-brick-wall-12`, `-30`); text only, not changed.

## 7. Files

Updated (catalog, `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/`): `🛋️furniture/park-bench/🔣️.json`, `👤️humans/seated-office-worker/🔣️.json`.
Updated (ticket): `requests.md` (three sections appended).
Created (ticket, `🧰️matrix/ML-4/`): `env.sh`, `q.sh`, `bounds.sh`, `suite.sh`, `soft.ts`, `soft-vitest.ts`, `solids.ts`, `overlaps.ts`, `chain.ts`, `chain-variant.ts`, `fill-rule.py`, `pre.sh`, `prove.sh`, `sheets.sh`; `multiview-ML-4.md`.
Removed: `🗑️generated/matrix/ML-4` (logs, backups, trial copies, sheets).
