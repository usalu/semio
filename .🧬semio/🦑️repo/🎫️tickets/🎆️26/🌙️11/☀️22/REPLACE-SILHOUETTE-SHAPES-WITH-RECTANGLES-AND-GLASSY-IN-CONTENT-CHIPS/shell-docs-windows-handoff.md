# 🧯️ Shell docs-1/2/3 and windows-1/2 test crates: handoff

Start (g6): docs-1 34/7, docs-2 80/5 (+11 ignored), docs-3 50/9, windows-1 62/9, windows-2 44/16. Several clusters were already cured by the arena-fixture batch and the ingress-generation product fix of the input-1 pass before my first fresh run (g7).

## Results (last measured run per crate, `repo cargo test -j 3 --no-fail-fast`, `--test-threads=2`)
| crate | result | run |
| --- | --- | --- |
| docs-1 | 41 passed / 0 failed | f2 |
| docs-2 | 85 passed / 0 failed / 11 ignored | f2 |
| docs-3 | 59 passed / 0 failed | f2 |
| windows-1 | 71 passed / 0 failed | f1 |
| windows-2 | 60 passed / 0 failed | f1 |

Final runs: `scratchpad/shell-dw/f1.txt` (windows-1+2) and `f2.txt` (docs-1+2+3), both after all edits.

## The 11 ignored tests (docs-2): kept ignored, not hiding failures
`hub-projection-workspace/*`: they need a live hub (`SEMIO_HUB_LIVE_ORIGIN`, principals), a staged native guest runtime (`SEMIO_PLUGIN_MODULES`, built by the renderer package's `native-guest-journey-check` verb) or a React `s serve` plus browser driver. Each carries its reason in `#[ignore = ...]` and in its doc comment, and the attribute is present in the first commit that contains the file. They are integration-tier laws, not hermetic ones.

## Clusters
| # | symptom | root cause | kind | files |
| --- | --- | --- | --- | --- |
| 1 | `original renderer fixture accepts the exact cumulative receipt` (docs-1 history body, windows tool-run, measures) | `close_ui_document_one` pays one fuel unit per node; the tests drove it with the default 1-unit `turn` | fixture | `wgpu-time-travel/🧰️harness-and-recorders`, `wgpu-tool-run-panel`, `wgpu-window-measures` (`turn_funded(1 << 20, ..)`) |
| 2 | paint "parked in layout" / overlay painted nothing | layout copies parameter records; `FixtureTurnV1::new(UI_WORKER_RETIREMENT_POLICY)` (1 item, 64 KiB) cannot lay out the History, ToolRun and Measures documents | fixture | same three harnesses now use `FixtureTurnV1::wide(.., 1)` |
| 3 | caller finished while its mounted surface was still open (`root closure never reached terminal`, `parent_empty Some(false)`) | helpers created a caller per paint and finished it with the surface mounted; a surface can only be closed by the caller that mounted it | fixture | time-travel harness (one thread-local caller, closed by `close_retained_body`), tool-run (`turns()`, caller passed in), measures (`paint_overlay`/`close_overlay` take the law's caller), dense Actions chrome (`pane_turns()` owns the pane surfaces) |
| 4 | `UI arena descendants remain borrowed at lifetime closure` / `stale UI value owner` | laws left shell documents (panel, window, aux, projection, spawned) unretired when the arena scope closed | fixture/test | new additive `fixtures::shell::retire_shell_documents`; called at the end of the affected laws (sync card, keyboard router, route, chip press, history body, local folders, band layout) |
| 5 | `UI arena has no accepting lifetime registration` | laws without `UiArenaFixture` | test | local-folders restart law, band layout focus law, measures, retention, actions pane |
| 6 | board/engine-retention tests: fixture close never finished | a retired CPU-only scene (Board2d) hands the host a component close request that nobody served | fixture | `ui-engine-tests/surface-fixtures::serve_cpu_only_component_close` (new), `PointerFixtureTurns::finish` now serves it; board-presence and engine-surface-retention retire their documents then `finish` |
| 7 | `/hub` left the route-not-found page showing | **product**: the `/hub` branch of `apply_shell_uri` returned before `note_shell_route`, which is what clears the recorded route | product | `staging/actions/command-dispatch/dispatch-app-switch` (`route_not_found = None`) |
| 8 | task-manager law: queued `toolRun:` press missing | stale test: tool-run presses are consumed by `take_tool_run_control_actions` and answered with the run's panel action; the law now queues a `kernel:` task and asserts the tool-run press is not left queued | test | `detached-sync-task-manager` |
| 9 | directory-lane law `left the shared directory lane`; sign-in law `SIGN_IN arms and returns` | stale source scans after the WP4 component split: `region_of` read only the first same-named region (it now concatenates all), the entry points are asserted un-gated wherever they are declared, the verbs are now `self.directory.start_hub_*` | test | `wgpu-directory-door`, `cross-shell-collaboration-and-sign-in-task` |
| 10 | export-batch: `copied_items` 8 vs 1 | step receipt compared with the turn total; now compared with `fresh_retained_progress_since` | test | `IconRenderHost/tests/export-batch` |
| 11 | keybinding override law `DEFAULT table is untouched` | test assumed `mod` is ctrl; on Apple hosts it is meta (`host_platform_uses_meta`) | test (cross-platform) | `settings-keybindings-hug-and-mobile` |
| 12 | leaf-closure law: `os.route-not-found publishes a body but no anchor declares it` | route-not-found and plugin-recovery are route-owned overlays exactly like Hub | test | same file |
| 13 | peer roster announcement `(Bearbeitet, ...)` | not a bug: the role word is React's `ui.presence.role.author` (a peer with role author/owner/member), documented on `presence_bar_chip_accessible_text` and unit-pinned; it names the role, not an activity. The law's expectation was stale | test | `wgpu-time-travel/notices-peer-edits-and-finalize` |
| 15 | measures keyboard gesture: typed text went nowhere | a tree has one roving Tab stop, its row (`is_tab_stop`, React's `applyRovingTabindexV1`); the oracle's single Tab now needs a second to reach the entry control. The law sends the first Tab twice | test | `wgpu-window-measures` |
| 14 | measures hug law: values right of overlay centre | not RTL: `retained-tree-node-painting` deliberately forces `FlowInline::Ltr` for any tree that is not a table (committed in 752), so the value column is at the inline end in LTR even in a TopEnd overlay. Law updated: value column on the right, checkbox at the column's inline start | test | `wgpu-window-measures` |

## Product changes
- `apply_shell_uri` clears `route_not_found` on `/hub`.
- None else. A trial edit of `EventRouter::transfer_interaction_from` (flow transfer) and `[DEBUG]` traces in the engine were reverted (verified clean by grep).

## Fixture additions (additive)
`fixtures::shell::retire_shell_documents`, `ui_engine_tests::surface_fixtures::serve_cpu_only_component_close`, `PointerFixtureTurns::finish` serving CPU-only closes.

## Still failing
Nothing in these five crates. Ignored: the 11 live-infrastructure laws above.

## Verified by running vs reasoned
Verified: every number in the table (final runs), each cluster fix by the crate going green. Reasoned only: that the `Bearbeitet` role word and the LTR tree are deliberate (read from code docs and the committed `retained-tree-node-painting` line), and the ignore history (git log of the first commit).
