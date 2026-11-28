# FEM laws that never ran: now run

Result: everything passes, no source fix was needed. The only defect was the runner route.

## Runner (root cause of "no vitest runner config")
The gesture test lives in the renderer engine package, not the FEM plugin: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🧪️tests/🖱️gesture-sample-lane/🟦️.ts`. Test files there are `🟦️.ts`/`🟦️.tsx` (no `*.test.*`), so plain `vitest run` finds none. The repo's route is the package `🎯️targets/⚛️react/📦️packages/🟦️typescript` with `--configLoader native --config ../../🧪️tests/🎚️config/🟦️.ts`, `SEMIO_VITEST_POLICY` and `SEMIO_TEST_LEVEL=standard` (same as `ix-vitest.sh`). A config already exists; none was added. New script `unrun-vitest.sh <paths relative to the engine dir>...` (multi-file, private cache `C:\t\tsv`).

## What ran
| Law | Route | Result |
|---|---|---|
| 2D `gesture-sample-lane` (9 tests, incl. "keeps a left drag marquee-ing instead of panning while a non-selective radio is on") | `unrun-vitest.sh` | 9/9 PASS |
| 3D `World3dHost` tests (engagement-pointer, modelling, projection-pane, multi-touch, component) + 2D `Canvas2dHost` input-contract and gumball-dispatch, as regression | `unrun-vitest.sh` | 7 files, 138/138 PASS |
| TS window-config contract 2D and 3D (incl. gumball valid/invalid fixture rows) | `bun run .../🧪️tests/🪟️window-config-contract/🟦️.ts` in each plugin root (`import.meta.main`) | both rc=0 |
| Rust `fem2d_window_config_model_matches_neutral_fixture_and_codecs` | `unrun-rust.sh wc` (crate `semio-s-artifact-fem-2d-editor-model-window --lib`) | 4/4 PASS |
| Rust `fem3d_window_config_model_matches_neutral_fixture_and_codecs` | same, 3D crate | 2/2 PASS |
| 3D `editor_app`: `focus_entity_from_a_panel_recentres_the_focused_pane_3d`, `a_selected_combination_renders_one_factor_input_per_term_3d` (Add Term), `the_analysis_row_selection_shows_the_analysis_settings_3d` (analysis granularity) | `unrun-rust.sh e3` | 3/3 PASS |

Cargo: private `C:\t\ts` and `C:\t\tstmp`, `-j 3`, detached launch. No `C:\t\wd*`, ports 6086 to 6089 or git writes touched.

## Fixed
Nothing in product code; no law failed. Note the new 2D marquee law was not shown to fail without the fix (not checked red first).

## Files
- Added: `unrun-vitest.sh`, `unrun-rust.sh` (in this ticket folder).
- Deleted generated output and `C:\t\ts*`, `C:\t\tsv`, `C:\t\ts-run.sh` when done.
