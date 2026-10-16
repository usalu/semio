# 🐚️ Result RENDERER-WP4: `ShellState` decomposition (S0 staging, 15 component structs, composer, narrow-parameter free items)

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool, no cargo compile (refused for physical compiler headroom, see section 8). Plan: `PLAN-RENDERER-WGPU-CRATES.md` (7.2 to 7.7, Appendix B/C/F, 10.4). Tools: `rwp4-tools/` (this ticket folder); logs `🗑️generated/rwp4-*`. `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`, `SH` = `E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu`, `T` = `E/🎯️targets/🧊️wgpu`.

## 0. Verdict

The decomposition is done in the monolith and verified statically; it is **not compiled**, because compiles are refused and the oracle cannot see types. `ShellState` is a composer of the fifteen planned components; 411 component methods and 23 free functions (18 former associated fns, 5 narrow-parameter fns) live in component impls or crate-level modules with explicit component parameters, 272 use-case members stay `impl ShellState` for WP5, every file of the 20 chunks is split into crate-shaped staging trees, 55 consumers were rewritten, and the first Shell crate (`shell-vocab`) is carved for real. Against the pre-WP4 snapshot the patched full-resolution differential shows **0 real differences in all four configurations** (native lib, `--cfg test`, wasm32 lib, wasm32 test; the only differences are sandbox duplicates of unresolved external imports and the errors this work removed). What is **not** proven: types, method/field existence in bodies the oracle skips, privacy of methods, borrowck. Those are covered by four static checkers that I wrote and seeded (section 6), not by rustc. Open design debts: ports P1/P2 were not made deferred (callers were lifted instead), three WP5 crates are over budget as staged, and the tier-1 staging trees are not crates yet because their dependencies (WP1..WP3 crates) are still skeletons.

## 1. What changed

1. **Step S0 (mechanical split, plan 10.2)**: the 701 `impl ShellState` members and the free items of the 20 chunk files were cut into **243 staging modules** `SH/📥️staging/<crate>/<source-slug>/🦀️.rs` (one per crate and source chunk), mounted by the old `shell` parent with `#[path]` and re-exported with globs where they hold free items. Bodies were moved verbatim; a private item that crossed a module boundary was widened to `pub(super)` (`rwp4-s0-widened.json`: 250 definitions, found by a name scan over all modules, tests included); relative `include_str!` paths were rebased; the extra `use` lines of the four pre-split modules travel with the code; every `//#region` marker is kept and balanced in each staging file (186 distinct marker lines before, 188 after: the pair for the staging block).
2. **Re-homing (7.4 step 5)**: the Appendix B first cut has 120 upward calls. A fixed point lowered pure helpers to their lowest caller and raised callers over upward calls: 132 of 701 members changed crate (113 up, 19 down, `rwp4-t1-plan.json`, table `RESULT-RENDERER-WP4-ASSIGNMENT.md`). Members whose transitive need exceeds three foreign components (46 members) became use-case methods. Free items were homed by gravity (lowest-ranked crate among members, other free items and component field types that name them; `gravity.py`), not by the file table of Appendix C, because the file table creates upward edges (`ShellSyncPill`, `World3dComputeStatus`).
3. **Step T1 (the decomposition)**: `ShellState` is now exactly the **fifteen component structs** of 7.3 as `pub` fields (`state.dock.dock_tabs`, `state.prefs.locale_id`, ...). Each component lives in `SH/📥️staging/<crate>/🧬️component/🦀️.rs` with its fields (cfg attributes and docs travel, field names unchanged, all `pub`). **411 tier-1 members** are methods of their component struct; the lower components they touch are parameters in rank order (`&mut` where the body or a callee may mutate, otherwise `&`): 235 need no foreign component, 116 one, 61 two, 19 three (the plan estimated 283/136/66/5 of 490). **272 use-case members** stay `impl ShellState` (WP5) and address the components (`self.<component>.<field>`, `self.<component>.method(args, &mut self.<k>)`). 18 pure associated fns became free functions. No extension trait, no wrapper, no forwarding method on the composer.
4. **Narrow-parameter free items (7.6)**: `tutorial_capture_camera_pose`, `tutorial_apply_camera_pose`, `tutorial_resolve_gesture_point`, `persist_ui_preferences` and `UiPrefsSnapshot::capture` take components now; the other tutorial functions (`*_ui_snapshot`, `*_ui_change_to_shell`, `render_tutorial_gesture_overlay_node`, `tutorial_recorder_sample`) touch four or more components and were lifted with their callers `tutorial_tick/stop/seek/toggle_play_pause` into `shell-session` (use-case tier, still `&ShellState`), as do `chrome_tour_layout` and `chrome_tour_anchor_rect` in `shell-view-overlays`. `Drop for ShellState` and `ShellState::new` (components built from per-component literals, in the `shell-state` staging module) address the components directly.
5. **External consumers** (55 files, rewritten with the same tables, `ext_overlay.py`/`apply.py`): 43 Shell test files, 4 root tests (`🔬️wgpu-frame-job-unit`, `🔬️wgpu-renderer-async-boundary`, `🔬️wgpu-winit-app-p3c`, `🧊️wgpu-os-host-standalone`), the `Dock` and `IconRenderHost` tests, and six renderer files owned by WP3 (`browser-worker`, `mailbox-impl`, `native-socket-probe`, `frame-presentation`, `app-runtime`, `frame-build`: only `shell.<field>` to `shell.<component>.<field>` and the receiver/argument changes of moved methods). Variables are found by binding (`let x = ShellState::new(..)`, helper fns returning `ShellState`, typed parameters), never by name.
6. **First physical Shell crate**: `shell_chrome_string` (the 629-line EN/DE chrome string table, a pure lookup) moved out of the monolith into the real crate `semio-framework-os-renderer-wgpu-shell-vocab` (`T/📖️shell-vocab/🦀️.rs`, the hub carries `extern crate … as shell_vocab;`, the parent `pub(crate) use shell_vocab::shell_chrome_string;`). The crate has no first-party dependency; its lib and test roots pass `rustc -Zno-codegen` (native), a seeded type error is caught.
7. **Cross-package requests of the coordinator, done**: (a) the four host-io items (`HOST_IO_REQUEST_TRACE`, `HostIoRequestTraceGuard` with its impls, both `host_io_request_trace_label`, `host_io_call`) are cut from the Shell and re-exported from `host-base` (`pub(crate) use …::host_io_call;` in the staging block); the 11 wasm-only errors they produced in the wasm differential are gone. (b) The cfg(test) helpers of `program-bridge`/`hub-workspace` that four Shell tests used (`window_measures_section_tests`, `window_engagements_section_tests`, `canvas_presence::tests::{numeric, peer}`) now live in `SH/../🧪️tests/🧰️section-fixtures/🦀️.rs` (mounted `cfg(test)` as `shell::section_fixtures`, imports re-pointed in `📏️wgpu-window-measures`, `🎬️wgpu-window-actions-search-panes`, `🔬️wgpu-shell-input`, `👕️board-presence`); no `testkit` feature and no edit in WP1's crates were needed, and WP1's two `E0432` signals of the test differential are gone; WP1 may delete its helpers or keep them for its own tests. (c) The 16 calls `crate::winit_app::dispatch_normalized_event(` of `🔬️wgpu-shell-input` call `crate::dispatch_normalized_event(`; the alias line in `winit-app` can go. (d) The two `contracts` lifts are written as verbatim `📥️lifted` files: `T/🤝️contracts/📥️lifted/🔖️identity-and-types/🦀️.rs` (the find-item family, 154 lines, standalone-clean under `rustc -Zno-codegen`, no import needed) and `T/🤝️contracts/📥️lifted/🧱️chrome-paint-and-navbar/🦀️.rs` (`ShellHubRemoteV1`, needs `serde` in the `contracts` manifest). The Shell originals stay until `contracts` mounts them, then the Shell cuts them and re-exports (`ActiveShellFindItems` is a thread-local singleton: never mount it twice).
8. **Language-agnostic test** `E/🧪️tests/🧬️shell-component-decomposition/🟦️.ts` with oracle `E/🧫️fixtures/🧬️shell-components/🔣️.json` (the 7.3 table): composer fields, component fields and the absence of direct legacy-field access in `impl ShellState`, read by two independent parsers (shipped tree-sitter Rust grammar, repository tokenizer). Red on the pre-WP4 tree (3 of 3 fail), green on the candidate tree (3 of 3 pass). Registered in the wgpu vitest config.

## 2. Published component interface (frozen; WP5 codes against this)

Source of truth: plan section 7.3 (fields, unchanged names), Appendix B (method homing), Appendix F (crate DAG). Field names do not change: `state.dock.dock_tabs`, `state.prefs.locale_id`. The composer `ShellState` (crate `shell-state`) is exactly the 15 component structs as `pub` fields, named by the snake-case field below, plus `Drop`.

| composer field | struct | crate | rank | fields | methods of the struct |
|---|---|---|---:|---:|---:|
| `chrome` | `ChromeState` | `shell-chrome-state` | 0 | 11 | 14 |
| `surfaces` | `SurfaceState` | `shell-surfaces` | 1 | 14 | 18 |
| `tutorial` | `TutorialState` | `shell-surfaces` | 1 | 2 | 3 |
| `settle` | `SettleState` | `shell-surfaces` | 1 | 5 | 4 |
| `plugins` | `PluginState` | `shell-plugins` | 2 | 14 | 16 |
| `history` | `HistoryState` | `shell-history` | 3 | 11 | 6 |
| `prefs` | `PrefsState` | `shell-prefs` | 4 | 13 | 39 |
| `directory` | `DirectoryState` | `shell-directory` | 5 | 28 | 42 |
| `sync` | `SyncState` | `shell-sync` | 6 | 18 | 16 |
| `docs` | `DocumentState` | `shell-docs` | 7 | 11 | 54 |
| `opening` | `OpeningState` | `shell-doc-open` | 8 | 12 | 20 |
| `windows` | `WindowState` | `shell-windows` | 9 | 32 | 34 |
| `dock` | `ShellDockState` | `shell-dock` | 10 | 36 | 65 |
| `input` | `ShellInputState` | `shell-input` | 11 | 30 | 67 |
| `commands` | `CommandState` | `shell-commands` | 12 | 11 | 11 |

Fields per component:

* `chrome: ChromeState` (shell-chrome-state): `error`, `screen_w`, `screen_h`, `fullscreen_toggle_requested`, `fullscreen_active`, `presented_chrome_accessibility`, `presented_chrome_accessibility_generation`, `chrome_floor_scope`, `dispatch_origin`, `chrome_build`, `chrome_present`
* `surfaces: SurfaceState` (shell-surfaces): `world3d_states`, `world3d_window_ids`, `retired_world3d_states`, `world3d_close_cancels`, `component_world3d_retirement`, `world3d_retirement_epoch`, `world3d_retirement_sequence`, `world3d_retirement_fault`, `world3d_status`, `world3d_status_pill_trace`, `node_graph_states`, `tiled_map_states`, `icon_render_states`, `board2d_states`
* `tutorial: TutorialState` (shell-surfaces): `tutorial`, `tutorial_pending_document_ops`
* `settle: SettleState` (shell-surfaces): `deferred_actions`, `settling`, `owed_refresh_scope`, `settle_pump`, `shell_io_pending`
* `plugins: PluginState` (shell-plugins): `plugins`, `extensions`, `plugin_filter`, `host_capability`, `space_mode`, `tool_run_panel_runs`, `plugin_faults`, `surface_faults`, `app_catalogue_json`, `app_catalogue_instance`, `active_example_id`, `pending_extension_invocations`, `default_app_pins`, `plugin_install`
* `history: HistoryState` (shell-history): `history_cursor`, `history_entries`, `history_current_checkpoint_id`, `history_time_travel`, `history_reprojection`, `time_travel_focus`, `time_travel_polled_at_ms`, `local_folder_bindings`, `local_folder_reattach_tried`, `local_folder_direct_reattach`, `host_route_folder`
* `prefs: PrefsState` (shell-prefs): `appearance_id`, `locale_id`, `terminology_id`, `driver_id`, `driver_draft`, `driver_save_label`, `theme_draft`, `theme_save_label`, `merge_policy`, `pending_theme_import`, `open_conflicts`, `selected_conflict_id`, `conflicts_seeded`
* `directory: DirectoryState` (shell-directory): `agent_chat_draft`, `identity`, `verified_session_authority`, `identity_offline`, `identity_env`, `shell_session_id`, `identity_retry_at_ms`, `identity_bootstrap_rx`, `identity_bootstrap_task`, `directory_client`, `agent_bridge`, `agent_bridge_offer_scope`, `inference_port`, `inference_port_status`, `directory_home`, `directory_transport`, `directory_cancel`, `directory_commands`, `hub_workspace`, `hub_sign_in_task`, `hub_spaces_task`, `hub_workspace_open`, `space_administration`, `space_administration_epoch`, `presence_peers`, `presence_surface`, `presence_self`, `presence_pointer`
* `sync: SyncState` (shell-sync): `sync_backbone_uri`, `sync_card_kind`, `sync_card_draft`, `sync_card_anchor`, `document_host`, `sync_channel`, `next_sync_binding_generation`, `sync_terminal_fault`, `sync_status`, `sync_link`, `hub_documents`, `sync_bootstrap_progress`, `sync_reseed`, `last_uncommitted_edit_at_ms`, `auto_checkin_pending`, `checkpoint_dispatched`, `checkin_dialog_draft`, `hub_check_in`
* `docs: DocumentState` (shell-docs): `session`, `window_ui`, `panel_documents`, `spawned_ui`, `spawned_session`, `app_document_identities`, `closing_documents`, `window_actions_documents`, `window_search_documents`, `window_projection_documents`, `window_measures_documents`
* `opening: OpeningState` (shell-doc-open): `uri_history`, `uri_index`, `open_space_id`, `pending_shell_uri_apply`, `pending_file_opens`, `import_transfers`, `import_transfer_next`, `last_envelope_dsl`, `document_services`, `document_opening`, `icon_export`, `document_execution_target_lease`
* `windows: WindowState` (shell-windows): `active_window_id`, `noted_active_window`, `host_event_active_window`, `engagement_expanded`, `engagement_activated`, `measures_folded`, `measures_expanded`, `measures_width`, `measures_resize_origin_width`, `engagement_inputs`, `measures_resize_window_id`, `window_topology_actions`, `window_topology_publications`, `window_topology_action_tokens`, `next_window_topology_action_token`, `window_topology_refresh_owed`, `window_topology_journal_dispatch_owed`, `active_utilities`, `active_utility_by_window`, `action_panel_folded`, `search_panel_folded`, `utility_bar_folded`, `projection_pane_folded`, `world_projection_template`, `window_icon_overrides`, `window_title_overrides`, `action_panel_expanded`, `tool_measures`, `window_engagements`, `window_measures_minted`, `utility_collection_expanded`, `contributor_instances`
* `dock: ShellDockState` (shell-dock): `panel_anchors`, `dock_tabs`, `dock_override`, `dock_collapsed_branches`, `dock_tab_drag`, `dock_loaded_app_id`, `panel_resize_origin_width`, `dock`, `split_resize_path`, `split_resize_index`, `split_resize_axis_total`, `dock_drag`, `pending_dock_drag`, `dock_view`, `dock_instance_owner`, `dock_input_identity`, `tree_windows`, `tree_window_served`, `dock_canvas_bounds`, `dock_axis_separator`, `dock_drop_tab_bars`, `dock_drop_bodies`, `dock_window_plan`, `dock_plan_trace`, `layout_override`, `split_resize_origin`, `split_resize_secondary_path`, `split_resize_secondary_index`, `split_resize_secondary_axis_total`, `split_resize_secondary_origin`, `layout_save_label`, `user_layouts`, `mobile_panel_visible`, `mobile_panel_path`, `window_content_rects`, `window_silhouettes`
* `input: ShellInputState` (shell-input): `scroll_offsets`, `overlay_state`, `collapsed_sections`, `tree_open_states`, `accessibility_focused_control_id`, `interaction_selection`, `open_selects`, `context_menu`, `right_click`, `tree_drag`, `tree_hovered_id`, `widget_maps`, `widget_maps_staging`, `pending_tree_drag`, `tree_drag_origin`, `pending_chrome_release`, `chrome_pointer_owner`, `keybinding_capture_control_id`, `retained_hit_windows`, `retained_hit_windows_staging`, `retained_scene_hits`, `retained_scene_hits_staging`, `presented_input_epoch`, `next_presented_input_epoch`, `presented_input_candidate`, `presented_input_geometry`, `presented_input_geometry_staging`, `retained_body_rects_staging`, `pane_overlay_hits`, `retained_hover_window`
* `commands: CommandState` (shell-commands): `search_open`, `find_open`, `search_query`, `search_selected`, `find_query`, `find_items`, `find_selected`, `staged_action_args`, `expanded_command_id`, `staged_command_args`, `search_possibles_open`


Bundle contract for WP5 (plan 7.2.3, unchanged): a use-case bundle is defined in the use-case crate that uses it and has `fn of(shell: &'a mut ShellState) -> Self` splitting the borrows through the composer's `pub` component fields. The per-member assignment (crate, struct, receiver, extra components, Appendix B first cut) is `RESULT-RENDERER-WP4-ASSIGNMENT.md`.

## 3. Hand-off to WP5 (tier-2/3 staging, read before starting)

* Your files: `SH/📥️staging/{🚶️session,🪝️window-actions,🔀️actions,🖐️pointer,🖍️view-paint,🍱️view-chrome,🗨️view-overlays,🪀️view-panels}/**`: 272 `impl ShellState` members and the free items gravity gave them. They already compile against the component paths (resolution differential), so you can convert them to bundle methods one file at a time.
* `shell-view-paint` ended with no free items of its own: its Appendix C items are used by `shell-view-chrome` and lower and were homed there (gravity). Re-split when you carve.
* Over budget (3,500 lines / 28 files / 230,000 bytes) as staged: `shell-actions` 4,639 lines / 294,141 bytes / 18 files, `shell-session` 3,628 lines, `shell-view-chrome` 3,503 lines / 212,599 bytes (limits 3,500 / 230,000 / 28). Spill rule 7.4: move the highest-fan-in members to the next crate up; the two knots are `dispatch_action` (622 lines) and `pump_sync_events`/`pump_directory_events`.
* **Ports P1/P2 were not implemented as inversions.** `dispatch_action` and `refresh_ui(_rendered)` stay use-case methods; every tier-1 member that called them (and every transitive caller) was lifted into your crates instead (the 113 upward re-homings of the assignment table). If you want the deferred inversion, the callers are exactly the members whose `from_appendix_b` differs and whose crate is `shell-session` or above.
* Tier-1 callees you call from `impl ShellState` are `self.<component>.method(args, extras)`; the extras are `&self.<k>`/`&mut self.<k>` as the table says. Free fns with narrow parameters: see section 1.4.

## 4. Hand-off to WP3 / WP1 / WP0

* **WP3**: the six renderer files above were edited in place (field paths and receivers only; list in `🗑️generated/rwp4-ext-applied.json`). `AppInteractionState.shell` is still a `ShellState` by value (legitimate above the composer). `native-socket-probe`, `browser-worker` call `ShellState::new` unchanged.
* **WP1**: mount the two lifted files of item 1.7(d) in `contracts` (one `mod` line each, `serde` in the manifest for the second), tell WP4 and the Shell originals are cut; the other Appendix C items for `contracts` were not lifted (gravity kept them with their Shell users). `host-base` is now the only owner of the four host-io items.
* **WP0**: no registry was touched (no crate, member, owner row or launch route added). The `shell-vocab` manifest still lists `serde_json` and `semio-framework-ui-locale`, unused by the first carve. The top crate manifest is unchanged. `Cargo.lock` untouched.

## 5. Census (staging trees, lib part: members, free items, component structs; tests stay mounted by the old chunks)

| staging folder | files | bytes | lines | within 3500 / 28 / 230000 |
|---|---:|---:|---:|---|
| `🔀️actions` | 18 | 294,141 | 4,639 | NO |
| `🚶️session` | 21 | 218,795 | 3,628 | NO |
| `🍱️view-chrome` | 11 | 212,599 | 3,503 | NO |
| `🗨️view-overlays` | 8 | 148,932 | 2,360 | yes |
| `📒️directory` | 16 | 128,666 | 2,351 | yes |
| `🖐️pointer` | 8 | 132,221 | 2,191 | yes |
| `🕹️input` | 16 | 117,075 | 2,068 | yes |
| `🎛️prefs` | 15 | 119,786 | 2,045 | yes |
| `📄️docs` | 18 | 124,870 | 1,966 | yes |
| `⚓️dock` | 18 | 103,783 | 1,806 | yes |
| `🪀️view-panels` | 11 | 99,432 | 1,755 | yes |
| `🎚️chrome-state` | 14 | 89,862 | 1,654 | yes |
| `🔲️windows` | 11 | 101,405 | 1,645 | yes |
| `🪝️window-actions` | 12 | 98,541 | 1,518 | yes |
| `🫧️surfaces` | 14 | 55,899 | 1,015 | yes |
| `📂️doc-open` | 13 | 52,814 | 868 | yes |
| `🔃️sync` | 9 | 37,228 | 688 | yes |
| `🧲️plugins` | 8 | 30,468 | 542 | yes |
| `⏪️history` | 5 | 27,836 | 538 | yes |
| `🎹️commands` | 4 | 26,941 | 514 | yes |
| `🐙️state` | 3 | 21,721 | 410 | yes |
| `⏳️async` | 3 | 4,292 | 88 | yes |


The tier-1 crates are all at most 2,351 lines, 128,666 bytes, 16 files. Before: `ShellState` impl 21,518 lines in 20 files inside one 37,071-line module.

## 6. Verification (commands run, real results)

| check | command / tool | real result |
|---|---|---|
| item scanner, losslessness | `rwp4-tools/lossless.py` (line multiset of the 25 chunk files against the new tree, widening normalised) | 2 lines differ: the `SHELL_SOURCE` list line (now followed by the staging includes) and one rebased `include_str!` path; every region marker line survives (186 before, 188 after) |
| rustfmt parse | `rustfmt +nightly-2026-07-20 --emit stdout --config skip_children=true` on all 289 changed or new Rust files of the live tree | 0 failures |
| path resolution | every `#[path]`, `include_str!`, `include!` literal of the live Shell tree, the WP4 hub and `shell-vocab` | 625 literals, 0 missing |
| full-resolution differential, native lib | `oracle4.py k1` (rustc +nightly-2026-07-20 `--emit=metadata -o /dev/null`, extern crates and external globs patched, B = pre-WP4 snapshot with the external files, the hub and the vocab root restored) | A 3,644 / B 3,626 errors; **0 real new**, 0 gone; 18 new are repeated unresolved external `use` lines (sandbox only) |
| `--cfg test` | same | A 1,006 / B 988; **0 real new**; 2 gone: WP1's `E0432` signals for the cfg(test) helper imports (fixed by item 1.7b); 20 sandbox duplicates |
| wasm32-unknown-unknown lib | same | A 3,453 / B 3,446; **0 real new**; 11 gone (the host-io items now in `host-base`, item 1.7a) |
| wasm32-unknown-unknown `--test` | same | A 6,049 / B 6,019; **0 real new**; 12 gone; 42 sandbox duplicates |
| differential sensitivity | `seed1`: one glob re-export of a staging module removed from the parent, one `pub(super) fn` made private again | **caught**: 43 new errors (`cannot find type BandTongue`, `band_label`, `ChromeBandLayout`, …) |
| earlier iterations of the same differential | `s0b`, `t1a`, `g2`, `h1`, `i1`, `j1` | found and fixed: `use` lists of the four pre-split chunks, `include_str!` depth, qualified paths into pinned modules (`icon_export::`, `time_travel::`, `local_folders::`, `tree_windows::`), child modules (`theme_geometry`), non-glob `use super::{…}` imports, the parent import block copied into staging files |
| type-level blind spot | `-Ztime-passes` of the candidate sandbox ends at `coherence_checking` / `type_check_crate` 0.4 s; seeded `s.nonexistent_field` and a wrong-arity call (`t1c`) were not reported | the differential does not typecheck bodies: see section 7 |
| component/arity/visibility checkers | `verify_t1.py` (every `self.x`, `<param>.x`, arity, in component impls), `verify_calls.py` (every `<recv>.<component>.x(..)` in staging, free fns, external text; `&self` holders of `&mut` methods), `verify_ext.py` (no old field/method on a ShellState-typed variable, by binding), `verify_vis.py` (no private definition or field named from another file) | all four **0 findings** on the live tree; a seeded arity error is reported by `verify_calls.py` |
| TS readers, vitest (wgpu config, 25 files) | `ts_compare4.py k1` (candidate sandbox against baseline sandbox, per-test status) | 24 identical; 1 differs by design (the new law); `wgpu-document-owner-move` and `engine-surface-retention` readers were adapted (below) |
| TS readers, bun test (20 files) | same | identical except `engine-surface-retention` and `wgpu-document-owner-move`, which read the Shell as one file or by `\n    fn ` boundaries; fixed: the first reads `readModuleTreeSource`, the second finds the method end with a visibility-aware boundary. Live tree: the owner-move law `releases the chrome step…` passes, its other 4 laws fail as in the baseline; `engine-surface-retention`: the Shell law passes, the remaining failure is the `EngineCanvas` registration struct (WP2's carve) |
| language-agnostic law | `vitest run --config E/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts E/🧪️tests/🧬️shell-component-decomposition/🟦️.ts` | baseline sandbox: 3 of 3 **fail**; live tree: 3 of 3 **pass** |
| `shell-vocab` | `rustc +nightly-2026-07-20 --crate-type lib [--test] -Zno-codegen` on its root (native), seeded type error | clean for lib and test; seeded error caught; census row `files 1, bytes 39,378, active lines 638, closureCompilable true` (`🗑️generated/rwp4-census-vocab.log`) |
| contracts lifts | `rustc -Zno-codegen` on the find-item file alone | clean; the `ShellHubRemoteV1` file needs `serde` |

## 7. Residual risks

* **Typeck is blind in the oracle.** In this crate rustc reaches `type_check_crate` and stops (the `-Ztime-passes` run ends at `coherence_checking`, 0.4 s): seeded field and arity errors in the candidate tree are not reported. The differential therefore proves name resolution, imports, item visibility and paths, not field/method existence, arity, privacy of methods/fields or borrowck. The compensating static checks (`verify_t1.py`, `verify_calls.py`, `verify_ext.py`, `verify_vis.py`) cover existence, arity, mutability of holders, leftovers of the old API and cross-file private definitions; borrowck and exact types are covered by nothing until the first compile.
* **Mutability of extras** is derived syntactically (assignments, `&mut`, mutating method names from a scan of all `&mut self` fns of the renderer tree plus std mutators). An over-approximation compiles when the caller holds `&mut`; a `&self` caller of an over-approximated method would fail. The scan found no `&self` member with a mutating need.
* **Behaviour**: unchanged by construction except argument evaluation order (component arguments are evaluated last) and borrow granularity (callees now borrow components, not the whole state, which only relaxes borrows).
* The tier-1 staging trees are not crates yet: they depend on `contracts`, `host-base`, `engine-*`, `scene-*`, `ui-engine`, `program-bridge` (WP1..WP3), which are still skeletons. Physical carving is possible crate by crate as those land; `shell-vocab` shows the shape.
* The 14 Shell test bins (`shell-tests-docs-*`, `windows-*`, ...) are untouched skeletons; the tests themselves run as before inside the monolith (rewritten against the component API).
* Peers edit files I rewrote (`E/🧪️tests`, WP3 files); `apply.py` is re-appliable from the pre-WP4 snapshots and refuses when a live file differs from what it last wrote.

## 8. Real compile attempts

* Start: `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-renderer-wgpu --lib` with `SEMIO_SCRIPT_POLICY` → `Insufficient physical compiler initial headroom` (4,291,510,272 available against 4,294,967,296 required); same for `-p …-shell-vocab` later (`🗑️generated/rwp4-cargo-start.log`, `rwp4-cargo-vocab0.log`).
* Later and at the end: `-p semio-framework-os-renderer-wgpu-shell-vocab --lib` → `Original queue claim path identity required` (`readResourceQueueClaimIdentityV1`) and a resource-control error (`rwp4-cargo-vocab1.log`, `rwp4-cargo-end.log`): infrastructure, not the crate (`shell-vocab`'s closure is gate-compilable per the census). No other crate can pass the gate: every other Shell crate sits behind the oversized frontier.
* One cargo at a time, no raw cargo, no private target dir, no scratch workspace; the only compilation was the analysis-only `rustc … -o /dev/null` oracle runs and `-Zno-codegen` on gate-compilable standalone roots.

## 9. Files

Created: `SH/📥️staging/**` (243 staging modules + 13 `🧬️component` modules, 22 crate folders), `SH/../🧪️tests/🧰️section-fixtures/🦀️.rs`, `E/🧪️tests/🧬️shell-component-decomposition/🟦️.ts`, `E/🧫️fixtures/🧬️shell-components/🔣️.json`, `T/🤝️contracts/📥️lifted/{🔖️identity-and-types,🧱️chrome-paint-and-navbar}/🦀️.rs`, `rwp4-tools/*` (this ticket), `RESULT-RENDERER-WP4-ASSIGNMENT.md`, this file; logs `🗑️generated/rwp4-*`.
Edited: the 20 chunk files and the parent `SH/🦀️.rs` (composer, mounts, `SHELL_SOURCE`, staging block), `T/📖️shell-vocab/🦀️.rs` (the carve), the WP4 hub `T/🧊️renderer/📥️mounts/🐚️wp4-shell-state/🦀️.rs` (one `extern crate … as shell_vocab;`), 43 Shell test files, 4 root tests, the `Dock` and `IconRenderHost` tests, six WP3 renderer files (list `🗑️generated/rwp4-ext-applied.json`), `E/🧪️tests/🖌️wgpu-document-owner-move/🟦️.ts`, `E/🧪️tests/🧲️engine-surface-retention/🟦️.ts`, the wgpu vitest config (one include).
Not touched: any registry (workspace, owner rows, launch routes, taxonomy, locks), any peer-owned crate, the top crate manifest.
Re-apply: `rwp4-tools/pipeline.sh <tag>` rebuilds everything from the pre-WP4 snapshot in the scratch directory, `apply.py` copies it and refuses when a live file differs from what it last wrote (peers added `Shell/🎬️activation`, `🚀️boot` and fixture folders while I worked; they are not mine and untouched).
