# normtest-b - norm test drifts and Python oracles

Status: source fixes written, NOT yet compiled or run. Two cargo test runs (`normtest-b-r2`, `-r3`) hit the 1 h `cargo:test` limit while starved by the capacity gate (no test executed); per coordinator no further cargo until `TEST COMPILES RESUME` is in `coordination-plan.md`. Re-run script: `T/normtest-b-test.sh <tag> -p <crate>...`.

## Verdicts on the 17 drifts (root = which side was wrong)
| test | verdict | fix |
| --- | --- | --- |
| outliner `unsupported_list_controls...` | test wrong: one `TreeWindows` reused across two locale renders (per-body ledger claims keys) | windows built per render |
| panels `a_viewport_action...`, `the_keyboard_help...`, `the_toolbar_lists...` | test wrong: `{:?}` of `BuiltNode` prints only child counts | project via `project_and_retire_fixture_tree` (`projected` helper) |
| app-surface `an_absent_row...` | test wrong: `address_pending` creates only the LAST row; a missing parent row is rightly refused | expectation `is_none()` |
| edit-rules `declared_companion_arguments...` | test wrong: neutral fixture holds 15 families, Rust test covers en1999 | filter by `family` |
| live-core `a_pointer_drag...` | test wrong: after release the hover parameter is the snapped committed value 4.25 (Rust+TS twin agree) | expect 4.25 |
| live-core `evaluation_is_debounced...` | test wrong: debounce fires once per burst | second poll empty |
| panels-inspection `a_multi_selection...` (+ properties cascade) | not diagnosed: arena fixture is now serial and panic-safe, so rerun first | pending run |
| visual-chrome slider `value_text` | test wrong: text is "value unit, utilisation, status" by design | assert German combined text |
| diagrams2d `table_twin` | rule is four significant digits (doc); test used 5 digits | data 1502.75 -> "1503" |
| diagrams2d golden | fixture was `{}\n`; regenerate with `SEMIO_UPDATE_GOLDEN=1` after the run and review | pending run |
| fibre linear law | midpoint strips miss own inertia: exact discrete value is closed form x (1 - 1/n^2) (4e-4 at n=50) | assert exact discrete value 1e-12 plus closed form within 1/n^2 |
| fibre bisection moment 3e-6 | not diagnosed (needs a run to see which random section/kink differs) | pending |
| case-kit OP form dimensions (2 tests) | fixture wrong: opening size keyed to `h` (0.5 m) exceeds slab width 0.3 m so width is hidden by design | opening keyed to `a2`; wide-opening test asserts the opening still follows its input (0.625) |
| meshing shell/weld volumes | test wrong: `MeshData` stores f32, thin shell/weld cancellation bound is eps32*extent/thickness | tolerance = that bound (verify by run) |

Files: outliner/viewport-panels/app-surface unit/edit-rules/direction-manipulation/lane-runtime/accessibility/diagrams2d/forms/tessellation/fibre test files.

## Python oracles (23 tests)
Not run. Findings: `SEMIO_PYTHON` is set by the repo command layer for test targets; fallback is `.venv`. The system python is 3.9 (needs 3.14), no interpreter under `.cache/b/t/m/python/interpreters`. Planned env: `.cache/b/d/m/python/environments/workspace-3b97251b6599d245` (groups dev+test, `uv sync --locked`). `repo tools exec uv` forces python-downloads=never; `repo dependencies sync --scope python` waited 20 min for capacity, then failed with "Actual declared requested Source scope required" (the stale bun receipt for `@semio-tech/course`). Needs the dependency owner to repair the receipt; then rerun the 23 oracle tests.
