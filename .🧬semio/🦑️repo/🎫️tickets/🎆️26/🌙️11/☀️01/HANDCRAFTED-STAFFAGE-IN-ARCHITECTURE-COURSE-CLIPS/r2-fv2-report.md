# r2-fv2: final Python-side verification

Runner: `T/m9_tests.sh` one module (or class, or method) at a time, 1500 s timeout, `SEMIO_TEACHING_LANGUAGE` set per run. Scripts: `T/🧰️fv2/run.sh`, `nx.sh`, `nxv.sh`, `nxg.sh`, `rerun.sh`, `hydronic_check.py`. No source file was edited by this agent.

## Summary
- Everything is green except: (1) `visuals.🧍️staffage.🧪️tests.🐍️` (24 errors, caused by the concurrent TOP-FRONT ticket, not this one), (2) `fonts...📑️heading` in en (fixture pins the German instruction; passes in de), (3) three heavy tests that exceed 1500 s or the memory limit (deferred).
- Modules without tests (rc 5, `NO TESTS RAN`, same as earlier rounds): `modules.txt` lines 27, 33, 34, 35.

## Table 1: modules.txt, guard, labels (V = `🎨️visuals`)

| module (line) | lang | count | verdict |
|---|---|---|---|
| V tests `🐍️staffage_only` (guard, rules A, B, C, thermal span; line 13) | en, de | 25, 25 | OK |
| V tests `🐍️labels` | en, de | 5, 5 | OK |
| 1 cooling internal-gains | en, de | 3, 3 | OK |
| 2 cooling clip | en, de | 4, 4 | OK |
| 3 ventilation volume | en, de | 2, 2 | OK |
| 4 ventilation architecture | en, de | 5, 5 | OK |
| 5 V staffage tests | en, de | 50, 50 (24 errors each) | FAIL, not this ticket (see below) |
| 6 semio-style | en, de | 9, 9 | OK |
| 7 V tests | en, de | 7, 7 | OK |
| 8 icons | en, de | 8, 8 | OK |
| 9 integration | en (re-run), de | 9, 9 | OK (first en run: transient gas-boiler export error, see below) |
| 10 scenes | en (re-run), de | 13, 13 | OK (same transient) |
| 11 section_consumers | en, de | 9, 9 | OK |
| 12 sections | en, de | 9, 9 | OK |
| 14 transforms | en, de | 2, 2 | OK |
| 15 readout | en, de | 5, 5 | OK |
| 16 audience | en | 7 | OK |
| 17 output | en | 9 | OK |
| 18 authoring clip tests | en, de | 31 (1 skipped), 31 (1 skipped) | OK |
| 19 browser | en | 9 | OK |
| 20 persisted | en | 4 | OK |
| 21 closure | en | 1 | OK |
| 22 geometry | en | 8 | OK |
| 23 interaction | en | 21 | OK |
| 24 hierarchy | en | 8 | OK |
| 25 chapters | en | 12 | OK |
| 26 pointer | en | 1 | OK |
| 27 process | en | 0 (no tests) | n/a |
| 28 presentation | en | 15 | OK |
| 29 stream | en | 3 | OK |
| 30 fonts | en | 13 | OK |
| 31 fonts heading | en / de | 3 (1 failure) / 3 | en FAIL (fixture), de OK |
| 32 demand sun | en | 2 | OK |
| 33, 34, 35 demand practice (composition, practice, speech) | en | 0 | no tests |
| hydronic-coupling oracle (`T/🧰️fv2/hydronic_check.py`, regenerates with scipy LSODA and CoolProp and compares to `🧫️fixtures/🚰️hydronic-coupling/🔣️.json`, 1e-9 relative) | n/a | 5 plates, 7 coupled rows | OK, fixture fresh |

## Table 2: Nakagin, Excel, Grasshopper (en, per class; 77 class runs)

| module | count | verdict |
|---|---|---|
| nakagin `❄️frozen` (LeanFaces, Orders, FrozenTower) | 2, 1, 2 | OK |
| nakagin `🌉️bridge` | 7 | OK |
| nakagin `🎨️visuals/✂️cutaway` | 4 | OK |
| nakagin `🎨️visuals` VisualContracts, per method | 21 of 22 OK | `test_tower_depth_selections_and_cached_transform` timed out at 1507 s (single method, cannot be split); deferred. The whole class timed out at 1504 s |
| nakagin `🏬️podium`, `🏭️roof` | 10, 10 | OK |
| nakagin `🧪️tests/🐍️` (Data, Capsule, Tower, ProgramEnergy, Variant, Assembly) | 3, 6, 11, 5, 5, 29 | OK |
| nakagin `👁️occlusion` (+ fragments 14, grouping 1) | 28 | OK |
| nakagin `💊️detail`, `💡️light`, `💾️order-cache`, `📍️survey`, `📏️dimensions`, `📦️casing`, `🔧️mounts`, `🗼️core`, `🚶️circulation`, `🧭️poses`, and the remaining nakagin test dirs | 9, 2, 4, 6, 13, 22, 4, 12, 18, 2, ... | OK |
| excel `🐍️` WorkbookToolkit and unit classes | 12 and all others | OK |
| excel `🎭️presentation` SceneContract, per method | 7 OK | `test_native_lifecycle` excluded and deferred per coordinator (memory, `T/r2-xl4-report.md`) |
| excel `⚡️anchors`, `📇️registry`, `🧍️occupants` | 3, 5, 4 | OK |
| grasshopper `🎭️presentation` GrasshopperPresentationContract, per method | 7 OK (narration, dispatch, chapters, imports, provenance, language, native play) | class timed out at 1505 s, so `test_native_lifecycle` is deferred (same memory family as xl4) |
| grasshopper `🎭️presentation` CanvasWidgets | 5 | OK |
| grasshopper `🐍️`, `📇️registry`, `📐️layout`, `🗼️ghosts` (TestPaths, Trees, Casting, Synthetic, Canvases, Geometry, Nakagin, TowerSlots, Catalogue, PanelDisplay, FittedDrawings, Legends, TowerGhosts) | 2, 7, 8, 5, 10, 12, 5, 10, 5, 2, 1, 1, 3 | OK |
| helper classes picked up by the per-class split (`ExactCamera`, `FakeWorkbook`, `Probe`) | n/a | not test cases, rc 1 is an artifact of the split, ignore |

## Failures and attribution

1. `V/🧍️staffage/🧪️tests/🐍️.py` (module line 5): 24 errors `KeyError: 'viewpoint'` at `V/🧍️staffage/🐍️.py:468` (`_index` requires `viewpoint` in the cast index). The `viewpoint` key and check were introduced by the concurrent `TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE` ticket (its `report-FW.md`, `report-TS.md`); the test's `synthetic()` index in `V/🧍️staffage/🧪️tests/🐍️.py` has no `viewpoint`. Not in `T/🧰️cf3/files.json`, not touched (instructed). Persisted in the last re-run. Fix belongs to that ticket: add `"viewpoint": "elevation"` to the synthetic index.
2. `fonts/🧪️tests/📑️heading/🐍️.py::test_actual_source_header_has_instruction_but_no_video_title` fails under `SEMIO_TEACHING_LANGUAGE=en`: the fixture `🔣️.json:25` pins `"instruction": "Was Lüftung leisten muss"` (German) while the English scene renders English. Passes in de (3/3). Pre-existing (last touched in commit 704), not this ticket; the fixture should carry the instruction per language.
3. Transient: first en runs of integration and scenes failed with `asset "gas-boiler" has no suitable representation at 1:50` while `🗂️catalog/🔌️appliances/gas-boiler/🔣️.json` was being edited. Both runs of de and the en re-runs pass (9 and 13 tests).

## Deferred
- BIM modules (`🏗️bim/🎬️clip/🧪️tests`: ModelCase, BimConsistencyContract, BimStepExchangeContract): 4 h geometry builds each, see `T/r2-bv-report.md`, `T/r2-bw-report.md`.
- `test_native_lifecycle` of `📋️program` (Excel, and the Grasshopper contract that timed out): memory, see `T/r2-xl4-report.md`.
- `test_tower_depth_selections_and_cached_transform` (nakagin VisualContracts): over 1500 s.
- vitest, catalog check and the de runs for modules 16 to 30 were not part of this pass.

## Cleanup
`T/🗑️generated/fv2` deleted; scripts and this report kept.
