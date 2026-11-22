# Shell panels parity: TaskManager, UtilityTree, ChromePanels

Base `B` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`.

## Verified against the Rust first
- Settings-unavailable (`settings.unavailable`, en+de) already existed (default-apps and conflicts leaves); audit was stale there. Route-not-found, the actor table, task model and utility grouping were genuinely missing.

## TaskManager
- New `B/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/📂️doc-open/🧵️task-manager-runtime/🦀️.rs`: lane/status/stage/task-lane/task-state enums with en/de labels, `runtime_metrics_rows` (= `runtimeMetricsRowsV1`), `task_manager_columns/rows/build_task_manager_table_scene` (same JSON as React's TableScene), `spawned_job_tasks`, `install_tasks`, `tool_call_tasks`, `tool_run_tasks`, elapsed/progress text/determinate percent, request queue.
- `OpeningState` gained `runtime_actors`, `runtime_tasks`, `runtime_requests`, `route_not_found`. `build_task_manager_ui` now = running tasks (transfers + runtime tasks with Suspend/Resume/Cancel) above the actors section (table rows with 11 cells + 3 actions, or distinct `no-runtime` / `no-actors`).
- Actions on controller `os.task-manager` (same descriptors as React) are queued in `OpeningState` (`take_task_manager_requests`) from `dispatch_action`.
- Labels: `B/🎯️targets/🧊️wgpu/📖️shell-vocab/🧵️task-manager-and-route-vocab/🦀️.rs` (key for key with React `os.taskManager.*`, plus `shell_chrome_format` for `{{x}}`).
- Fixtures: new `B/🧱️elements/🧵️TaskManager/🧪️tests/🧫️fixtures/🧵️actor-table.json`; existing `🏃️running-tasks.json` now replayed by Rust too. React test added to `🧩️component/🟦️.tsx`.

## UtilityTree
- New `B/🧱️elements/🎛️UtilityTree/🎯️targets/🧊️wgpu/🦀️.rs` (mounted in shell-view-chrome): `group_utility_nodes_by_category`, `UTILITY_CATEGORIES`, icons, sort, dedupe, ribbon segments + keys, path reconcile, pressed leaf / group path, selection (row-selection bucket) and `utility_ribbon_row_keys` (depth rows, `row-utility-options`, `row-selection-options`).
- Fixture `🎛️UtilityTree/🧪️tests/🧫️fixtures/🌳️utility-tree/🔣️.json`; React replay `🧪️tests/🌳️fixture-law/🟦️.tsx`; Rust replay in `🍱️shell-view-chrome/🧪️tests/🦀️.rs`.

## ChromePanels
- New `📂️doc-open/🧭️route-not-found/🦀️.rs`: `parse_shell_route` (= `parseShellRoute`), `note_shell_route`, `build_route_not_found_ui` (message `common.routeNotFound` en/de + Home button -> `framework navigateShellUri "/"`). `apply_shell_uri` records an unknown path (never `/hub`) and returns before switching apps; new `navigateShellUri` framework action.
- Route cases live in `actor-table.json`.

## Honest gaps
- Nothing yet feeds `set_runtime_actors/tasks` or drains requests (no wgpu actor runtime bridge); unattached shell still shows `no-runtime`.
- Route-not-found page and grouped ribbon are builders/models; painting them over the canvas / swapping the paint path (view-chrome/overlays, other agents' files) is not wired.
- Actor table is stack rows (no Table widget node exists yet).

## Tests to run (status: NOT YET COMPILED OR RUN)
My only `check -p semio-renderer-wgpu-shell-doc-open` was still compiling dependencies (about 608 units) when the no-new-cargo policy arrived; it never reached my crates, so no compile error or pass is verified. Earlier attempts were blocked by "source boundary exceeded" in `semio-ui-wgpu-events` and `semio-ui-wgpu-paint` (other agents' crates, 4032 and 4209 of 4000 active lines). My crates are well under the limit (doc-open about 1626, view-chrome about 3207).

Integration agent, in order:
1. `check -p semio-renderer-wgpu-shell-vocab`, then `-p semio-renderer-wgpu-shell-doc-open`, `-p semio-renderer-wgpu-shell-actions`, `-p semio-renderer-wgpu-shell-view-chrome` (expect small fixes: untried imports, `use super::*` visibility of `Deserialize`/`IconName`/`ActionDescriptor` in the new doc-open staging modules).
2. `test -p semio-renderer-wgpu-shell-doc-open`: actor table, running tasks, progress line, route and route-not-found-message laws.
3. `test -p semio-renderer-wgpu-shell-view-chrome`: utility category grouping, dedupe, ribbon/path/row-bucket laws.
4. `test -p semio-renderer-wgpu-shell-tests-docs-3`: the existing `task_manager_leaf_body_and_command_report_the_unattached_runtime_truthfully` must stay green, plus three new tests (runtime actors listed with three actions, request queue, unknown route recorded). `apply_shell_uri` via `ActionsCtx` is an assumption; adjust the call if the method is not reachable there. Existing wgpu tests that call `apply_shell_uri` with trailing-slash paths (for example `/spaces/x/studio/`) now count as not found, as in React.
5. React (not run): vitest on `🧵️TaskManager/🧪️tests/🧩️component` and `🎛️UtilityTree/🧪️tests/🌳️fixture-law`.

## Follow-up round (code only, still no cargo)
Note: another agent overwrote `🎬️shell-actions/🎯️dispatch-action/🦀️.rs`, `📨️dispatch-app-switch/🦀️.rs` and `🐙️state/🧩️session-and-topology/🦀️.rs` at 02:17 and dropped my edits; I re-applied them. Re-check these three still carry `TASK_MANAGER_CONTROLLER_ID`/`NAVIGATE_SHELL_URI_ACTION` arms, `note_shell_route` and the four new `OpeningState` field initialisers.

### Wired
- Feed: React's `TaskManagerWindow` reads `sources.tasks()` plus `ActivationRegistry.runtimeMetricsSnapshot()`/`metricsBus`. The wgpu shell has no activation registry or kernel handle (only `semio-renderer-wgpu-kernel-runtime`, not reachable from shell state), so there is no actor snapshot to read. The one real runtime source it owns is the lazy plugin install (`plugins.plugin_install`, phases Resolving/Loading): `publish_shell_panel_document` now calls `OpeningState::sync_runtime_install_tasks` before building the Task Manager body, so the install shows as an `install:<plugin>` task (elapsed time kept across rebuilds) with Cancel.
- Drain: `dispatch_action` on controller `os.task-manager` queues the press, then `take_install_cancel_requests` hands an install Cancel to `PluginState::cancel_plugin_install` (the same call the install banner's cancel uses). Actor suspend/resume/cancel and task suspend/resume stay queued in `OpeningState::runtime_requests` (`take_task_manager_requests`) because no runtime exists to receive them.
- Actor table: now a `ComponentScene` of kind Table (`build_table_scene` + `TableScene::base(columnsJson, rowsJson)`), byte-identical to React's TableScene, replacing the stack rows. NOT the retained `Component::Table`/`TableRow`: that needs a new `UiNode::Table` variant (about 19 files across other agents' ui crates) and a projection arm in `🔲️windows/🧾️panel-records`; I judged it too risky blind. Follow-up for whoever owns the table upgrade.
- Route-not-found: `apply_shell_uri` records the path and also sets `chrome.error` to the localized `Route not found: {path}` (cleared again when a known route is applied and the error is still that text). This paints through the existing Error phase text line, not as a replacement canvas page with a Home button; `build_route_not_found_ui` (Home button) still has no mount.
- Grouped ribbon: the UtilityTree model moved to the shell-dock crate (mounted there, `pub use`); `derive_window_utility_nodes` now returns `group_utility_nodes_by_category(.., UTILITY_CATEGORIES)` like React's window utility bar, so the roster that paints and the roster that hit-routing/keybindings address (`windows.active_utilities`) stay identical. Behaviour change to expect: history/sync-category utilities no longer appear in the window bar, and every utility path is one collection deeper (`selection`/`utilities` first). Existing tests that assert window utility rosters or collection paths may need updating.

### Run list (all UNRUN; the integration agent compiles)
1. `check` then `test -p semio-renderer-wgpu-shell-vocab`, `-shell-dock` (new `utility_tree` module; if source boundary 4000 lines trips, split it), `-shell-doc-open`, `-shell-actions`, `-shell-session`, `-shell-view-chrome`, `-shell-tests-docs-3`.
2. Laws: doc-open `actor_table_replays_the_fixture_in_both_locales`, `actor_row_cells_print_the_em_dash_for_every_unobserved_counter`, `running_tasks_replay_the_fixture`, `task_progress_line_and_percent_follow_the_fixture_words`, `shell_routes_replay_the_fixture`, `route_not_found_message_is_the_shared_en_de_label`; view-chrome `utility_category_grouping_replays_the_fixture`, `utility_nodes_dedupe_by_first_id`, `ribbon_segments_paths_and_row_buckets_replay_the_fixture`; docs-3 `task_manager_leaf_lists_the_runtime_actors_with_all_three_row_actions` (now asserts the table scene JSON), `task_manager_actor_and_task_presses_queue_requests_for_the_runtime_bridge`, `unknown_shell_route_is_recorded_and_known_routes_clear_it`, plus the pre-existing `task_manager_leaf_body_and_command_report_the_unattached_runtime_truthfully`.
3. Watch for: window-utility tests broken by the grouping; `semio_ui_declaration::ui::{build_table_scene, TableScene}` paths; `semio_renderer_wgpu_shell_plugins::ShellPluginInstallPhase` path in the session crate.
4. React (not run): `🧵️TaskManager/🧪️tests/🧩️component/🟦️.tsx`, `🎛️UtilityTree/🧪️tests/🌳️fixture-law/🟦️.tsx`.

## Third round (code only, no cargo)
### (3) Window bar grouping corrected to match React
React's `utilityBarNode` groups with `UTILITY_CATEGORIES` (selection, utilities only, so history and sync are NOT in the window bar) and then unwraps each bucket (and any `Select` group) into its children, so the bar's top level is the utilities themselves. My previous round wrongly put the category collections one level deeper. Now `window_utility_bar_nodes` (UtilityTree wgpu module) ports exactly that and `derive_window_utility_nodes` calls it; paths therefore do not gain a level. History and sync utilities still drop out of the window bar, as in React. New fixture key `windowBar` plus test `window_utility_bar_replays_the_fixture` (view-chrome). React has no exported function for this unwrap, so the fixture is replayed by Rust only.

### (1) Real actor snapshot and controls
React: `TaskManagerWindow` reads `ActivationRegistry.runtimeMetricsSnapshot()` rows (actorId, pluginId, resident, shard) and sends `suspend/resume/cancel` back through the registry. Wgpu twin:
- Port in `🎯️targets/🧊️wgpu/🏛️host-base/🎛️actor-observation-port/🦀️.rs` (host-base is the existing host port crate both kernel-pool and the shell depend on): `ObservedActor`, `publish_observed_actors`/`observed_actors`, `ActorControl{Suspend,Resume,Cancel}`, `request_actor_control`/`take_actor_controls`.
- Kernel side: new `🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🎛️runtime-observation/🦀️.rs`; `run_kernel_pool` calls `serve_runtime_observation` at the top of every loop turn. It applies queued controls to the pool's `ParallelRuntime` (`kernel_mut().suspend(actor, None)`, `.resume(actor)`, cancel as an `Envelope` with `Payload::Cancel(0)`, the shard-loop contract React's native counterpart names) and republishes `kernel().runtime_metrics_snapshot` as `ObservedActor` rows every 500 ms (`resident` = not `Suspended`, `shard` from `actor_record`). I avoided new `KernelRequest`/`KernelOutcome` variants on purpose: they would break every exhaustive `KernelOutcome` match in the protocol crates.
- Shell side: `OpeningState::sync_runtime_actors` (called when the Task Manager panel is published) maps the snapshot to `RuntimeActorSample`; `None` stays the no-runtime state (wasm, or no kernel yet). `forward_actor_requests` drains the actor presses from `runtime_requests` into the port from `dispatch_action`. Task presses (tool-run suspend/resume) remain queued: no wgpu source lists tool runs as tasks yet.
- Limit: the kernel loop only turns when a request arrives or work is pending, so a control or snapshot can lag until the next kernel request; actor ids are the kernel's numeric ids as strings.

### (2) Route-not-found page
React shows only the page (no error line), so the earlier `chrome.error` line is removed. The page is now a retained panel document: `ROUTE_NOT_FOUND_PANEL_ID` (defined in shell-docs next to the other panel ids, re-exported by doc-open) is a shell-owned panel leaf, `publish_shell_panel_document` builds `build_route_not_found_ui` for it while `opening.route_not_found` is set (else nothing), and the hub-workspace overlay step now paints that document (centered card with the message and the Home button, scrim id `os.route-not-found.scrim`) when the hub is closed and the route is not found. Home dispatches `framework navigateShellUri "/"`. It is a centered overlay card rather than a full-bleed canvas replacement, and it reuses the hub overlay's frame phase to avoid a new phase in the frame state machine.

### Additional run list (all UNRUN)
- `check` then `test -p semio-renderer-wgpu-host-base`, `-kernel-pool` (native), `-shell-docs`, `-shell-session`, `-shell-view-overlays`, `-shell-doc-open`, `-shell-actions`, `-shell-dock`, `-shell-view-chrome`, `-shell-tests-docs-3`.
- Updated test `task_manager_actor_and_task_presses_queue_requests_for_the_runtime_bridge` (actor press now reaches `take_actor_controls`; only the task press stays queued).
- Compile risks to look at first: `Backpressure`/`KernelError` used without `Debug`; `semio_framework_actor::ActorStatus` path in the kernel-pool crate; `ParallelRuntime::kernel_mut()` borrow across `.await` in `serve_runtime_observation`; `ROUTE_NOT_FOUND_PANEL_ID` used as a match-arm pattern in the session crate; hub scrim `control_id` expression in the overlay step.
- Not covered by any test: kernel-pool observation loop, the host-base port round trip beyond the docs-3 test, the overlay paint of the route page.
