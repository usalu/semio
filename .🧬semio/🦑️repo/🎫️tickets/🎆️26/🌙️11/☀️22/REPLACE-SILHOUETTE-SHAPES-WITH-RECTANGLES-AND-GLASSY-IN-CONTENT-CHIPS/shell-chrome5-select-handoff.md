# shell-tests-chrome-5: Select popup commit, fixture poisoning, fixture rows, wasm seam

## Result

- `semio-renderer-wgpu-shell-tests-chrome-5`: 57 passed / 0 failed (plain `repo cargo test`, `--test-threads=2`, run twice after the final edits; log `scratchpad/chrome5b/r9.txt`). Before: 52/5.
- `semio-ui-wgpu-engine-laws`: 72 passed / 0 failed, including `ui_surface_slot_table_is_heap_first_and_fits_a_bounded_thread_stack` against the 60 464 B row (log `scratchpad/chrome5b/r10.txt`).
- `semio-renderer-wgpu-ui-engine-tests`: 43 passed / 16 failed, no hang (`r13.txt`). `engine_canvas_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack` passes against the corrected rows. The 16 failures are the pre-existing, fixture-independent ones listed under Open (checked: each of the sampled ones fails alone in under 1 s with the same message).
- wasm: `repo cargo check --target wasm32-unknown-unknown -p semio-renderer-wgpu-host-base` exit 0 (target installed), so `scope_plugin_catalog_v1` compiles for wasm (log `scratchpad/chrome5b/r11.txt`).

## Task 1: root cause of `appearance_option_commit_uses_the_retained_router_and_retires_its_popup`

Product bug, not a stale test, and not a regression of the frame conversion: the press on the General Appearance `Select` never opened the popup.

- Instrumented the engine (`[DEBUG]` prints, removed): the press hit the Select node (correct rect), capture and release matched the same node, `overlays=0` afterwards.
- The release branch chose `tree_row=true`. `EventRouter` (`⚡️events/🖱️accessibility-and-pointer-dispatch`) decides `tree_row = ... tree_row_owner(tree, active_id).is_some()` before the `is_select` branch, so the click was routed as a Tree row press (`route_tree_press`) and `toggle_select_popup` never ran.
- Why `tree_row_owner` answered Some for a Select: a General Tree item and its inline control share one key by design (control id == row id, fixture `⚙️settings-general-layout` `tree.controls`: `rowId == controlId`; printed: node key == parent key == `<surface>/framework.settings.appearance`). `UiTree::authored_tree_item` and `find_tree_item_spec` identified an item by key alone, so the inline control child was mistaken for its own row. Same mistake affected any inline Button/Input/Toggle/Select in a Tree row (tab stops, drag arming, focus).
- Fix (product): a node is an authored tree item only if it is the row `Stack` (rows mount as `UiNode::Stack`).
  - `🧰️framework/🔨️modules/🖱️ui/🌳️retained/🌳️tree/🍃️nodes/🌿️ui-tree-core-and-layout/🦀️.rs`: `authored_tree_item` returns None for a non-`Stack` node (docstring added).
  - `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🧭️event-model-and-focus/🦀️.rs`: `find_tree_item_spec` same guard.
- Verified by running: the single test passes (`r5.txt`), then the full crate 57/57.

## Task 1b: one failure no longer cascades

Cause: a failing law left its retained documents in the process-wide document arena / handback table; the next law on the thread then hit `stale UI value owner` in `close_ui_document_page_one`, and its arena close failed with `UI arena descendants remain borrowed`. `UiArenaFixture::abandon` only resets value handbacks, and `FixtureTurnV1::drop` skipped its surface close while panicking.

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧰️fixtures/🐚️shell/🦀️.rs`: new `RetiringShell` (Deref/DerefMut to `ShellState`); when the law panics its `Drop` runs `retire_shell_documents` under `catch_unwind`. It is declared after the fixture caller so it drops first (documents are handed back before the caller and arena close).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎟️original-turn/🦀️.rs`: `FixtureTurnV1::drop` now also closes its owned surfaces while panicking (best effort under `catch_unwind`, bounded to 256 rounds instead of 65 536).
- `🧱️elements/🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs`: the seven document-owning laws build their shell through `RetiringShell::new`; one disjoint-borrow line became `let state = &mut *shell`.
- Verified by running a deliberate probe (`panic!("[DEBUG] probe")` at the end of the Appearance law, since removed): before the guard 5 failed (probe + the four cascaded), with the guard exactly 1 failed / 56 passed (`r6.txt` vs `r8.txt`). Probe removed afterwards (`grep` clean).

## Task 2: fixture rows

- UI row (60 464 B per `UiSurfaceSlot`): `semio-ui-wgpu-engine-laws` passes (72/72).
- Renderer rows (`📺️renderer/🧪️tests/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json`), law in `⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🗺️tiled-map-and-board-attach-laws/🦀️.rs`:
  - The previous agent's claim was wrong: `EngineSurfaceSlot` still embeds `Option<EngineSurfaceId>` (258 inline bytes), so one slot is 288 B and the 256-slot table is 73 728 B, above the 65 536 B conversion threshold. The law `ENGINE_SURFACE_CAPACITY * size_of::<EngineSurfaceSlot>() <= conversionThresholdBytes` failed. Restored the measured `EngineSurfaceRegistry` entry in the law and the fixture row with the true numbers (capacity 256, element 288, owner 32; confirmed by the law's own left/right diff).
  - `EngineCanvasBuildContext` row was stale independently (committed 384 / 72, measured 1 168 / 312): updated to the measured values.
  - The law file also did not compile against current `BoardHost` (`host.nodes` -> `host.scene.nodes`, `host.edges` -> `host.scene.edges`, `host.selection` -> `host.focus.selection`): fixed.
- Lock-order inversion (deadlock, scheduling dependent, found because the crate hung in a full run): `retained_map_same_host_sibling_rebase_retires_engine_interaction_owner` took the arena serial lock (`FixtureTurnV1::new`) before `engine_surface_law_guard`, every other law takes the guard first. Swapped the two lines. Evidence: `sample` of the hung process showed both laws parked on those two mutexes; run alone or with `--test-threads=1` they do not hang.

## wasm seam

`scope_plugin_catalog_v1` lives in `🎯️targets/🧊️wgpu/📇️catalog/🦀️.rs`, a `#[path]` module of `semio-renderer-wgpu-host-base` declared without a cfg gate, so it is compiled for wasm. Uses only `std` (`thread_local!` const `Cell`, `Box::leak`, `OnceLock`). The wasm check above passed.
Note (not changed): `resolve_plugin_host_config` caches its host table in a process `OnceLock` from the first catalogue it sees, so under a thread scope it can serve a stale host table; the scoped catalogue test only uses `on-artifact-kind:` activation, which does not hit that cache.

## Open (not mine, pre-existing, listed so they are not lost)

`semio-renderer-wgpu-ui-engine-tests` has 16 failures independent of the two fixture rows (each fails alone in under 1 s). Counted causes in the full run: 9x work-loan, 5x world resources, 1x `layout never settled` (`a_painted_chrome_window_reports_draw_calls_quads_and_glyphs`), 1x float/int compare. Not investigated further, outside this task:
- four laws fail with `original work loan callback replaced its initiating context or refilled its work` (`a_board_window_paints_its_tool_run_trace_lane_and_echoes_the_cursor`, `board_and_map_two_touch_gestures_share_camera_math_but_keep_distinct_transfer_rules`, `retained_map_key_replacement_and_removal_retire_the_old_gesture_without_input`, `retained_map_same_host_sibling_rebase_retires_engine_interaction_owner`): a job/fixture-caller accounting invariant in `semio_framework_job`, not in UI code.
- `a_revision_bump_while_the_bridge_lease_is_in_flight_still_publishes_the_draws`: `original world resources dropped before paid close` (`♾️infinite/🌍️world/🗃️model/.../🎒️prepared-resources/🦀️.rs:448`).
- `map_device_loss_recovery_restores_the_live_camera_on_retry`: compares `Number(40.0)` with `Number(40)` (JSON float vs integer) in `♻️retained-map-gesture-lifecycle-and-bridge/🦀️.rs:229`.
