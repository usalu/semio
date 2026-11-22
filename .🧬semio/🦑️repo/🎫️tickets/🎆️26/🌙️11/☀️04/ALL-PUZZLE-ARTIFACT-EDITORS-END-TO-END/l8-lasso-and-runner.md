# L8: 2D lasso failure and canonical browser runner flags

## Diagnosis: engine bug, not stale wasm and not story/test
- The shipped pkg wasm is from 2026-11-19 19:38; the relevant engine files (`pointer-planners`, `pointer-commit-steps`, `scene-state-impl`, `pointer-plan`) have mtimes 2026-10-21 / 2026-10-28, so they predate it. A rebuild will NOT fix the failure by itself.
- Fixture `🧪️tests/🧫️fixtures/📸️board-session/🔣️.json` lasso is a closed polygon: last point equals first `(-80,-80)`. The test releases on the start point.
- `plan_selection_pointer` (`🧭️host/🗺️pointer-planners/🦀️.rs`) on Up computed `click_only = distance(start_screen, end_screen) < 4px`; a closed loop ends at its start, so it was treated as a click: `next = {}` and the commit cleared the selection (mode none, selectionCount 0). During the drag preselect worked (preselectCount 5 is the preview path, which has no click check). The legacy `Interaction::Selection` branch in `🧭️host/🖱️pointer-screen-events/🦀️.rs` had the same flaw.
- Commit steps (`step_selection_commit`) are correct: they insert exactly the plan deltas.

## Changes
- `🗺️pointer-planners/🦀️.rs` and `🖱️pointer-screen-events/🦀️.rs` (under `🧰️framework/.../♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧭️host/`): `click_only` is now "every recorded screen point is within the click distance of the start" instead of "the end is".
- New Rust test `closed_lasso_released_at_its_start_commits_the_enclosed_nodes` in `🧭️host/🧪️tests/🔬️board-host/🪝️pointer-plans-and-commit-turns/🦀️.rs` (node-a enclosed, node-b excluded; asserts commit plan events). Not compiled or run (no build allowed); its imports (`load_board_snapshot_json`, `serde_json`) were assumed to be in scope as in the sibling delete-plan fixture, so check them on first build.
- Canonical runner flags: `.storybook/🧪️tests/🧪️browser-runner/🟦️.ts` already takes `args` from `browserLaunchOptionsV1()` (policy in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🧬️schema/🔣️.json`, schema-first). That policy had only `--use-angle=swiftshader --enable-unsafe-swiftshader --enable-unsafe-webgpu`. Added `--enable-features=Vulkan,WebGPU --use-vulkan=swiftshader --use-webgpu-adapter=swiftshader --ignore-gpu-blocklist` to the schema and to the test corpus `🧪️tests/🧫️fixtures/🔣️.json` (the test `every browser runner uses the portable software GPU policy` compares them). `l3-pw.config.ts` is now redundant for flags (its 60 s expect timeout is already covered by the runner's `expect` of up to 120 s).

## Verify after a wasm rebuild
1. Rebuild `@semio-tech/puzzle-wasm`, restage, rerun `🧪️tests/◻️storybook-2d` on port 6010 (expect 12/12; lasso gives `["alpha"]`).
2. Run the new Rust test plus the existing `selection_move_and_up_are_revisioned_fixed_page_plans` and `a_click_without_motion_publishes_only_its_selection` (clicks must still clear).
3. Run `🧪️tests/🟦️.test.ts` of the browser library (policy corpus) and one puzzle suite via the root `.storybook` runner.
