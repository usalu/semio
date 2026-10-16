# 🐚️ Result RENDERER-WP4: `ShellState` decomposition (S0 staging, 15 component structs, composer, narrow-parameter free items)

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool, no cargo compile (refused for physical compiler headroom, see section 8). Plan: `PLAN-RENDERER-WGPU-CRATES.md` (7.2 to 7.7, Appendix B/C/F, 10.4). Tools: `rwp4-tools/` (this ticket folder); logs `🗑️generated/rwp4-*`. `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`, `SH` = `E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu`, `T` = `E/🎯️targets/🧊️wgpu`.

## 0. Verdict

__VERDICT__

## 1. What changed

1. **Step S0 (mechanical split, plan 10.2)**: the 701 `impl ShellState` members and the free items of the 20 chunk files were cut into **__NSTAGING__ staging modules** `SH/📥️staging/<crate>/<source-slug>/🦀️.rs` (one per crate and source chunk), mounted by the old `shell` parent with `#[path]` and re-exported with globs where they hold free items. Bodies were moved verbatim; a private item that crossed a module boundary was widened to `pub(super)` (`rwp4-s0-widened.json`: __NWIDEN__ definitions, found by a name scan over all modules, tests included); relative `include_str!` paths were rebased; the extra `use` lines of the four pre-split modules travel with the code; every `//#region` marker is kept and balanced in each staging file (186 distinct marker lines before, 188 after: the pair for the staging block).
2. **Re-homing (7.4 step 5)**: the Appendix B first cut has 120 upward calls. A fixed point lowered pure helpers to their lowest caller and raised callers over upward calls: __NREHOME__ of 701 members changed crate (__NUP__ up, __NDOWN__ down, `rwp4-t1-plan.json`, table `RESULT-RENDERER-WP4-ASSIGNMENT.md`). Members whose transitive need exceeds three foreign components (__NFORCED__) became use-case methods. Free items were homed by gravity (lowest-ranked crate among members, other free items and component field types that name them; `gravity.py`), not by the file table of Appendix C, because the file table creates upward edges (`ShellSyncPill`, `World3dComputeStatus`).
3. **Step T1 (the decomposition)**: `ShellState` is now exactly the **fifteen component structs** of 7.3 as `pub` fields (`state.dock.dock_tabs`, `state.prefs.locale_id`, ...). Each component lives in `SH/📥️staging/<crate>/🧬️component/🦀️.rs` with its fields (cfg attributes and docs travel, field names unchanged, all `pub`). **__NT1__ tier-1 members** are methods of their component struct; the lower components they touch are parameters in rank order (`&mut` where the body or a callee may mutate, otherwise `&`): __PARAMDIST__. **__NUSE__ use-case members** stay `impl ShellState` (WP5) and address the components (`self.<component>.<field>`, `self.<component>.method(args, &mut self.<k>)`). __NASSOC__ pure associated fns became free functions. No extension trait, no wrapper, no forwarding method on the composer.
4. **Narrow-parameter free items (7.6)**: `tutorial_capture_camera_pose`, `tutorial_apply_camera_pose`, `tutorial_resolve_gesture_point`, `persist_ui_preferences` and `UiPrefsSnapshot::capture` take components now; the other tutorial functions (`*_ui_snapshot`, `*_ui_change_to_shell`, `render_tutorial_gesture_overlay_node`, `tutorial_recorder_sample`) touch four or more components and were lifted with their callers `tutorial_tick/stop/seek/toggle_play_pause` into `shell-session` (use-case tier, still `&ShellState`), as do `chrome_tour_layout` and `chrome_tour_anchor_rect` in `shell-view-overlays`. `Drop for ShellState` and `ShellState::new` (components built from per-component literals, in the `shell-state` staging module) address the components directly.
5. **External consumers** (55 files, rewritten with the same tables, `ext_overlay.py`/`apply.py`): 43 Shell test files, 4 root tests (`🔬️wgpu-frame-job-unit`, `🔬️wgpu-renderer-async-boundary`, `🔬️wgpu-winit-app-p3c`, `🧊️wgpu-os-host-standalone`), the `Dock` and `IconRenderHost` tests, and six renderer files owned by WP3 (`browser-worker`, `mailbox-impl`, `native-socket-probe`, `frame-presentation`, `app-runtime`, `frame-build`: only `shell.<field>` to `shell.<component>.<field>` and the receiver/argument changes of moved methods). Variables are found by binding (`let x = ShellState::new(..)`, helper fns returning `ShellState`, typed parameters), never by name.
6. **First physical Shell crate**: `shell_chrome_string` (the 629-line EN/DE chrome string table, a pure lookup) moved out of the monolith into the real crate `semio-framework-os-renderer-wgpu-shell-vocab` (`T/📖️shell-vocab/🦀️.rs`, the hub carries `extern crate … as shell_vocab;`, the parent `pub(crate) use shell_vocab::shell_chrome_string;`). The crate has no first-party dependency; its lib and test roots pass `rustc -Zno-codegen` (native), a seeded type error is caught.
7. **Cross-package requests of the coordinator, done**: (a) the four host-io items (`HOST_IO_REQUEST_TRACE`, `HostIoRequestTraceGuard` with its impls, both `host_io_request_trace_label`, `host_io_call`) are cut from the Shell and re-exported from `host-base` (`pub(crate) use …::host_io_call;` in the staging block); the 11 wasm-only errors they produced in the wasm differential are gone. (b) The cfg(test) helpers of `program-bridge`/`hub-workspace` that four Shell tests used (`window_measures_section_tests`, `window_engagements_section_tests`, `canvas_presence::tests::{numeric, peer}`) now live in `SH/../🧪️tests/🧰️section-fixtures/🦀️.rs` (mounted `cfg(test)` as `shell::section_fixtures`, imports re-pointed in `📏️wgpu-window-measures`, `🎬️wgpu-window-actions-search-panes`, `🔬️wgpu-shell-input`, `👕️board-presence`); no `testkit` feature and no edit in WP1's crates were needed, and WP1's two `E0432` signals of the test differential are gone; WP1 may delete its helpers or keep them for its own tests. (c) The 16 calls `crate::winit_app::dispatch_normalized_event(` of `🔬️wgpu-shell-input` call `crate::dispatch_normalized_event(`; the alias line in `winit-app` can go. (d) The two `contracts` lifts are written as verbatim `📥️lifted` files: `T/🤝️contracts/📥️lifted/🔖️identity-and-types/🦀️.rs` (the find-item family, 154 lines, standalone-clean under `rustc -Zno-codegen`, no import needed) and `T/🤝️contracts/📥️lifted/🧱️chrome-paint-and-navbar/🦀️.rs` (`ShellHubRemoteV1`, needs `serde` in the `contracts` manifest). The Shell originals stay until `contracts` mounts them, then the Shell cuts them and re-exports (`ActiveShellFindItems` is a thread-local singleton: never mount it twice).
8. **Language-agnostic test** `E/🧪️tests/🧬️shell-component-decomposition/🟦️.ts` with oracle `E/🧫️fixtures/🧬️shell-components/🔣️.json` (the 7.3 table): composer fields, component fields and the absence of direct legacy-field access in `impl ShellState`, read by two independent parsers (shipped tree-sitter Rust grammar, repository tokenizer). Red on the pre-WP4 tree (3 of 3 fail), green on the candidate tree (3 of 3 pass). Registered in the wgpu vitest config.

## 2. Published component interface (frozen; WP5 codes against this)

__INTERFACE__

Bundle contract for WP5 (plan 7.2.3, unchanged): a use-case bundle is defined in the use-case crate that uses it and has `fn of(shell: &'a mut ShellState) -> Self` splitting the borrows through the composer's `pub` component fields. The per-member assignment (crate, struct, receiver, extra components, Appendix B first cut) is `RESULT-RENDERER-WP4-ASSIGNMENT.md`.

## 3. Hand-off to WP5 (tier-2/3 staging, read before starting)

* Your files: `SH/📥️staging/{🚶️session,🪝️window-actions,🔀️actions,🖐️pointer,🖍️view-paint,🍱️view-chrome,🗨️view-overlays,🪀️view-panels}/**`: __WP5MEMBERS__ `impl ShellState` members and the free items gravity gave them. They already compile against the component paths (resolution differential), so you can convert them to bundle methods one file at a time.
* `shell-view-paint` ended with no free items of its own: its Appendix C items are used by `shell-view-chrome` and lower and were homed there (gravity). Re-split when you carve.
* Over budget (3,500 lines / 28 files / 230,000 bytes) as staged: __OVER__. Spill rule 7.4: move the highest-fan-in members to the next crate up; the two knots are `dispatch_action` (622 lines) and `pump_sync_events`/`pump_directory_events`.
* **Ports P1/P2 were not implemented as inversions.** `dispatch_action` and `refresh_ui(_rendered)` stay use-case methods; every tier-1 member that called them (and every transitive caller) was lifted into your crates instead (the 113 upward re-homings of the assignment table). If you want the deferred inversion, the callers are exactly the members whose `from_appendix_b` differs and whose crate is `shell-session` or above.
* Tier-1 callees you call from `impl ShellState` are `self.<component>.method(args, extras)`; the extras are `&self.<k>`/`&mut self.<k>` as the table says. Free fns with narrow parameters: see section 1.4.

## 4. Hand-off to WP3 / WP1 / WP0

* **WP3**: the six renderer files above were edited in place (field paths and receivers only; list in `🗑️generated/rwp4-ext-applied.json`). `AppInteractionState.shell` is still a `ShellState` by value (legitimate above the composer). `native-socket-probe`, `browser-worker` call `ShellState::new` unchanged.
* **WP1**: mount the two lifted files of item 1.7(d) in `contracts` (one `mod` line each, `serde` in the manifest for the second), tell WP4 and the Shell originals are cut; the other Appendix C items for `contracts` were not lifted (gravity kept them with their Shell users). `host-base` is now the only owner of the four host-io items.
* **WP0**: no registry was touched (no crate, member, owner row or launch route added). The `shell-vocab` manifest still lists `serde_json` and `semio-framework-ui-locale`, unused by the first carve. The top crate manifest is unchanged. `Cargo.lock` untouched.

## 5. Census (staging trees, lib part: members, free items, component structs; tests stay mounted by the old chunks)

__CENSUS__

The tier-1 crates are all at most 2,351 lines, 128,666 bytes, 16 files. Before: `ShellState` impl 21,518 lines in 20 files inside one 37,071-line module.

## 6. Verification (commands run, real results)

__VERIFY__

## 7. Residual risks

* **Typeck is blind in the oracle.** In this crate rustc reaches `type_check_crate` and stops (the `-Ztime-passes` run ends at `coherence_checking`, 0.4 s): seeded field and arity errors in the candidate tree are not reported. The differential therefore proves name resolution, imports, item visibility and paths, not field/method existence, arity, privacy of methods/fields or borrowck. The compensating static checks (`verify_t1.py`, `verify_calls.py`, `verify_ext.py`, `verify_vis.py`) cover existence, arity, mutability of holders, leftovers of the old API and cross-file private definitions; borrowck and exact types are covered by nothing until the first compile.
* **Mutability of extras** is derived syntactically (assignments, `&mut`, mutating method names from a scan of all `&mut self` fns of the renderer tree plus std mutators). An over-approximation compiles when the caller holds `&mut`; a `&self` caller of an over-approximated method would fail. The scan found no `&self` member with a mutating need.
* **Behaviour**: unchanged by construction except argument evaluation order (component arguments are evaluated last) and borrow granularity (callees now borrow components, not the whole state, which only relaxes borrows).
* The tier-1 staging trees are not crates yet: they depend on `contracts`, `host-base`, `engine-*`, `scene-*`, `ui-engine`, `program-bridge` (WP1..WP3), which are still skeletons. Physical carving is possible crate by crate as those land; `shell-vocab` shows the shape.
* The 14 Shell test bins (`shell-tests-docs-*`, `windows-*`, ...) are untouched skeletons; the tests themselves run as before inside the monolith (rewritten against the component API).
* Peers edit files I rewrote (`E/🧪️tests`, WP3 files); `apply.py` is re-appliable from the pre-WP4 snapshots and refuses when a live file differs from what it last wrote.

## 8. Real compile attempts

__COMPILE__

## 9. Files

__FILES__
