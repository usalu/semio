# 🧪️ Result RENDERER-TEST-BINS (slice `rtb`, 2026-10-16)

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no modifying git command. `F` = this folder. Tools: `F/rtb-tools/` (every script named below). Logs and census outputs: `F/🗑️generated/rtb-*`. Scratch (pre-edit snapshot `snap0`, first-edit copies `pre/`, oracle sandboxes): the session scratchpad `rtb/`.

## 0. Verdict

**GREEN against the gate, UNCOMPILED.** The `semio-framework-os-renderer-wgpu` test unit went from 592 files / 9.3 MB / 38,108 lines (35.5x the gate) to 22 files / 113 KB / 1,397 lines (0.69), and every new or edited test unit of the slice (37 crates, native + both wasm labels) is inside the gate. Every renderer and Shell law that the monolith unit ran is now mounted, in place and under its own name, by exactly one of 15 `shell-tests-*` bins, `engine-attach-tests`, `ui-engine-tests`, `runtime-mailbox-tests`, `frame-tests-1`, or in the crate whose private items it reads (`host-winit`, `frame-job`, `dock`); the test-name multiset is unchanged (+2 new `test-support` tests); 0 duplicate mounts; the two new crates (`fixtures`, `frame-tests-1`) are registered everywhere.

What this does NOT prove: nothing was compiled (the cargo ingress refuses on this machine) and the stub-layer typeck oracle could not be made to reach typeck at this scale, so cross-crate privacy (E0616/E0624) of the moved laws is unverified; sections 6.2 and 6.3 list the candidates, section 7.1 the first real-compile commands. The union resolution oracle (native + wasm) shows no row that the pre-edit baseline did not have except stale names that HEAD's laws already used.

## 1. Task, as executed

The `semio-framework-os-renderer-wgpu` (top crate) test unit was the one unit still over the Rust source gate (4,000 active lines / 32 files / 262,144 bytes): it compiled 93 source files (lib adapter, renderer root, five hubs, the Shell/EngineCanvas/Scenes/Interpreter facades, 66+ law files) and 499 embedded inputs (356 `.rs` source texts, 140 JSON, 2 GLB, 1 TOML). The slice moves every law out of it into test bins and companion crates (Appendix D packing), moves the shared fixtures into the lowest crate that can name what they need, turns the source-text laws into run-time reads, and registers everything.

### 1.1 Baseline (before the first edit)

`bun F/rtb-tools/rtb-census.ts` (copy of `otcp-census.ts`, output prefix `rtb-census-`; `MANIFESTS=<top manifest> DUMP_INPUTS=1`), top crate `semio-framework-os-renderer-wgpu`, saved in `F/🗑️generated/rtb-baseline/`:

| unit | native | wasm32-unknown-unknown | wasm32-wasip2 |
|---|---|---|---|
| lib (normal) | 13 files / 115,612 B / 1,169 lines | n/a | n/a |
| lib-test | **592 / 9,312,360 / 38,108 (35.5x)** | **521 / 8,074,999 / 23,678 (30.8x)** | 2 / 3,619 / 68 |

The 592 files are 93 source files plus 499 embedded inputs (356 `.rs` source texts of the source-text laws, 140 JSON fixtures, 2 GLB, 1 TOML). The dev-dependency rows `semio-framework-ui` (31 files / 1,109,244 B / 19,483 lines, ratio 4.87, direct and normal) and `semio-framework-plugin` (125 / 1,979,150 B, ratio 7.55, transitive and normal) are over the gate on every label; they are normal dependencies of the top crate, not test closures (foreign: the peer UI and plugin carves).

## 2. What changed

### 2.1 Shared fixtures, each in the lowest crate that can name what it needs

| fixture | new home (module, all behind `#[cfg(any(test, feature = "testkit"))]`) |
|---|---|
| `collect_fixture_actions` | `🛎️frame-actions/🧰️fixtures` (`frame-actions`, re-exported from the crate root) |
| `ENGINE_SURFACE_LAW_LOCK`, `engine_surface_law_guard`, `with_engine_close_context` | `🗺️engine-registry/🧰️fixtures` |
| `camera_fixture_grant/lease/clear/sweep_expired_scene_camera_dispatches`, `CameraFixtureLease` | `🎭️scene-state/🧰️fixtures` |
| `drop_engine_surface`, `retire_engine_surface_fixture` | `🔗️engine-attach/🧰️fixtures` |
| `finish_*` close fixtures, `runtime_with_shell` | `🏠️os-host/🧰️fixtures` (module of `host-winit`) |
| Shell helpers (`host_test_shell`, `host_test_apps`, `test_app`, `split_pane_*`, `engaged_pane_shell`, `actions_shell`, `tree_pointer_record`, `paint_*_pointer_documents`, `retained_world_sequence_probe`, `world3d_pointer_record`, `mouse_pointer`, `pointer_interaction`, `boot_*`, `tour_*`, `partition_utilities_by_category`) | new crate `semio-framework-os-renderer-wgpu-fixtures` (`🧰️fixtures`, module `shell`): needs `ShellState`, so it sits above `shell-state`; every shell bin depends on it |
| retained-surface helpers shared by the engine-surfaces, node-graph and paint2d laws (`paint_retained_surface_scene/_in_window`, `close_retained_surface_fixture`, `RetainedSurfacePaint`, `retained_document`, `retained_surface_record`, `drop_world3d_states`, `with_world_step`) | `ui-engine-tests/🧰️surface-fixtures` (module `surface_fixtures`); `engine-attach-tests` now depends on `ui-engine-tests` (they live in two bins) |
| Dock internals read by the Dock unit laws | the 50 laws that only need the dock crate are mounted inside the Dock element module (`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs`, private items stay private); the 10 laws that need `ShellState`/shell crates moved with their helpers to the sibling file `🛰️Dock/🧪️tests/🔬️wgpu-shell-bound`, mounted by `shell-tests-chrome-2` (test names unchanged, file identity of those 10 changed) |

`testkit` feature convention: a bin builds its dependencies without `cfg(test)`, so every cross-crate `cfg(test)` item the laws name became `cfg(any(test, feature = "testkit"))` and the feature is propagated through the dependency chain (ledger 2.5).

### 2.2 Laws reading source text

`test-support` (`🧯️test-support/📜️repo-text`, module `repo_reader`) now carries `repo_root`, `engine_root`, `repo_text`, `repo_texts`, `repo_bytes` and the five source sets (`renderer_source()`, `scenes_source()`, `engine_canvas_source()`, `interpreter_source()`, `shell_source()`, built from the Rust-side module-tree reader). All 66 test files that embedded `.rs`/`.toml` sources through `include_str!`, `concat!(include_str!…)` or the `*_SOURCE` consts (`RENDERER_SOURCE`, `SHELL_SOURCE`, …) read them at run time instead (consts became `fn NAME() -> &'static str`); the `CARGO_MANIFEST_DIR` idiom became file-relative or `engine_root()`. The largest embedded JSON/GLB inputs of the three over-budget bins (time-travel band 66 KB, text-splice 47 KB, world3d modelling 62 KB, capsule GLB 129 KB, mask-stroke, text-input, local-echo, node-graph-gestures) are read with `repo_text`/`repo_bytes` as well. Two new test-support tests (`repo_texts_read_what_the_standard_reader_reads`, `every_source_set_file_exists_below_the_engine_folder`) pin the readers against `std::fs` and the source-set lists.

### 2.3 Laws naming `AppInteractionState`/`ShellState`

They are mounted by the 15 `shell-tests-*` bins (depending on the flipped `shell-state` and `fixtures`) and by the companions below; no test file was copied (every file stays at its place and is mounted with `#[path]`, so the test names and the file identities are unchanged).

### 2.4 In-crate placements (private access)

Laws that read private items of one crate are mounted inside that crate instead of widening it: the os-host standalone law, the winit-app p3c / callback-latency / frame-generation-hold laws (`host-winit`), the frame-job unit and outcome laws (`frame-job`), the Dock unit laws (`dock`, 2.1).

### 2.5 Ledgers

Testkit gating (`cfg(test)` → `cfg(any(test, feature = "testkit"))`): `board2d_tool_run_trace_state` (engine-attach), `component_scene_begin_note_edit_fixture` (engine-graph), `EngineCanvasPacketDestination` (engine-registry), `CANCELLED_KERNEL_TURNS` (kernel-protocol, re-export in kernel-runtime, use and increment in kernel-pool), `ProgramFixtureDocument` (program-bridge), `ink_pointer_state_is_clear` (scene-ink), `PAINT2D_NAVIGATOR_PADDING`, `Paint2dDocSyncJson`, `Paint2dFlatLayer` (scene-tables); `testkit` features added or propagated in 12 manifests (engine-attach, engine-graph, engine-registry, host-winit, kernel-pool, kernel-protocol, kernel-runtime, program-bridge, scene-ink, scene-state, scene-tables and the first-party members they pull in).

Visibility widenings (only these): `native_socket_probe::native_socket_sqlite` private → `pub mod` (sqlite law in `frame-tests-1`); `GraphInteractionDispatch` private → `pub struct` with `publish_select`, `publish_hover` and `item_count()` `pub(super)` → `pub` (node-graph law in `engine-attach-tests`).

Dependency hygiene: `host-winit` test-only crates are `optional` normal dependencies switched on by its `testkit` feature (the os-host fixtures need `shell-input`, `ui-declaration`, `ui-locale`) plus `dev-dependencies` for the in-crate laws; no production build gains a dependency.

### 2.6 The monolith

Removed from the top crate unit (`cleanup.py`, verified by `chunkrefs.py` that nothing else mounts them): every `cfg(test)` statement of the Shell, EngineCanvas, Scenes and Interpreter facade parents; the test statements of hubs wp1/wp3/wp5 (kept: the renderer-io retained host, the paint-policy law, the heap witness, the doc-surface law); `RENDERER_SOURCE`/`Mesh3dItem` of the renderer root; the five `🧪️test-hosts` directories; the 9 emptied Shell chunk files (`⏪️time-travel` keeps its `🗣️labels` and `🚦️refusals` children, which production staging uses). The top crate keeps 3 test files (paint-policy, renderer-io retained host, doc-surface).

### 2.7 Registration

`Cargo.toml` members (byte-sorted, 2 new), `📋️build-owners.json` (2 rows), `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (12 routes each, after the `shell-tests-docs-3` block, same naming/grouping/env, order series 900.245xx / 922.053xx / 903.593xx), `Cargo.lock` (manifest dependency sync for every touched manifest, 2 entries created), taxonomy (`🧰️fixtures`, `🍇️frame-tests-1` folder members; module names `🧰️fixtures`, `🐚️shell`, `📜️repo-text`, `🧰️surface-fixtures`), `🔒️dependencies.json` users, `📋️project.json` of both crates, the frozen crate table `🗑️generated/rwp0-crates.json` (first-party sets refreshed from the manifests, topological re-rank of the test side, 2 rows).

## 3. The bins

| bin | mounted test files |
|---|---|
| `engine-attach-tests` | `wgpu-standalone`, `wgpu-standalone`, `🕸️wgpu-node-graph`, `🖌️wgpu-paint2d-engine`, `🫳️node-graph-gestures`, `🩺️metrics-invalidation-drain`, `🔬️wgpu-catalogue-workflow-drop` |
| `frame-tests-1` | `🧾️frame-action-ledger`, `🖱️wheel-application-point`, `⏱️frame-latency-diagnostics`, `🧭️boot-axis-parity`, `🧪️tests`, `🐕️wgpu-present-stall-watch`, `🧊️wgpu-renderer-standalone` |
| `runtime-mailbox-tests` | `🔬️wgpu-renderer-async-boundary`, `📥️enqueue` |
| `scene-ink-tests` | (skeleton) |
| `shell-tests-chrome-1` | `🔬️wgpu-ui-command-wiring` |
| `shell-tests-chrome-2` | `🔬️wgpu-shell-bound` |
| `shell-tests-chrome-3` | `🔬️wgpu-shell-chrome-parity` |
| `shell-tests-chrome-4` | `🔬️wgpu-chrome-overlays-tour`, `🎨️wgpu-theme-editor-and-accessibility` |
| `shell-tests-chrome-5` | `🗄️browser-prefs-persistence`, `🧭️wgpu-navbar-footer-parity`, `🖥️wgpu-display-conflicts-marketplace`, `⚙️settings-general-layout` |
| `shell-tests-chrome-6` | `🚗️driver-editor`, `🔬️wgpu-retained-chrome-text-laws`, `🔬️wgpu-ui-prefs-themes-i18n`, `🔬️wgpu-agent-overlays`, `🧪️wgpu-fault-notices`, `📐️wgpu-dpi-logical-units`, `💓️chrome-maintenance-pressure`, `🌓️appearance-tour-and-footer-pills`, `🚪️wgpu-host-door-remainder`, `🫀️settle-pump` |
| `shell-tests-docs-1` | `🧪️wgpu-time-travel` |
| `shell-tests-docs-2` | `🔬️wgpu-identity-directory-presence`, `🔗️hub-projection-workspace`, `🧪️wgpu-local-folders`, `🔀️wgpu-document-relay`, `🔬️wgpu-shell-boot-isolation`, `📇️wgpu-directory-door` |
| `shell-tests-docs-3` | `📤️export-batch`, `🔬️wgpu-shell-pool-future`, `🔬️wgpu-shell-document-retirement`, `🛍️app-catalogue-attempt`, `📤️wgpu-file-open-import`, `🔬️wgpu-media-frames`, `⬇️wgpu-media-export-encoding`, `🎬️wgpu-plugin-install`, `👕️board-presence`, `🔄️sync-owner-lifetime`, `🔄️detached-sync-task-manager` |
| `shell-tests-input-1` | `🔬️wgpu-shell-input` |
| `shell-tests-input-2` | `🔬️wgpu-command-registry`, `⌨️wgpu-shell-shortcuts-palette`, `🎯️presented-input-authority` |
| `shell-tests-input-3` | `🔬️wgpu-context-menu-keyboard`, `⌨️native-keyboard-ring`, `🎓️tour-overlay-and-chord-glyphs`, `⌨️wgpu-chord-example-role-parity`, `🎡️wgpu-wheel-and-escape-routing`, `🪟️window-lifecycle-template-drag`, `🎯️wgpu-pointer-hit-ownership` |
| `shell-tests-windows-1` | `🔬️wgpu-panel-anchor-model`, `🪟️wgpu-window-pane-chrome`, `⏯️wgpu-tool-run-panel` |
| `shell-tests-windows-2` | `🪟️tree-windows`, `🪟️action-window-scope`, `🔬️wgpu-tutorial`, `🎬️wgpu-window-actions-search-panes`, `📏️wgpu-window-measures`, `🧲️engine-surface-retention`, `🛟️panel-window-reservation`, `🛰️wgpu-dock-close-reopen-journals` |
| `ui-engine-tests` | `🎟️original-turn`, `🧰️surface-fixtures`, `🧩️wgpu-engine-surfaces`, `🔬️wgpu-board2d-engine`, `🧪️tests`, `🔬️wgpu-introspection` |

`fixtures` and `test-support` are support crates; `scene-ink-tests` stays the skeleton it was (its only law mount moved to `engine-attach-tests`/`ui-engine-tests` with the helpers it shares).

## 4. Census after (`bun F/rtb-tools/rtb-census.ts`, lib-test unit of every crate the slice created or edited; files / bytes / lines (worst ratio of the gate))

| crate | native | wasm32-unknown-unknown | wasm32-wasip2 |
|---|---|---|---|
| `~` | 22 / 113,494 B / 1,397 L (0.69) | 15 / 76,064 B / 1,036 L (0.47) | 2 / 3,619 B / 68 L (0.06) |
| `~-dock` | 11 / 237,019 B / 3,774 L (0.94) | 3 / 110,037 B / 2,161 L (0.54) | 3 / 110,037 B / 2,161 L (0.54) |
| `~-engine-attach` | 4 / 61,231 B / 1,160 L (0.29) | 4 / 61,231 B / 1,160 L (0.29) | 4 / 61,231 B / 1,160 L (0.29) |
| `~-engine-attach-tests` | 13 / 237,281 B / 3,632 L (0.91) | 13 / 237,281 B / 3,632 L (0.91) | 13 / 237,281 B / 3,632 L (0.91) |
| `~-engine-graph` | 3 / 69,826 B / 1,322 L (0.33) | 3 / 69,826 B / 1,322 L (0.33) | 3 / 69,826 B / 1,322 L (0.33) |
| `~-engine-registry` | 6 / 103,263 B / 2,305 L (0.58) | 6 / 103,263 B / 2,301 L (0.57) | 6 / 103,263 B / 2,301 L (0.57) |
| `~-fixtures` | 3 / 40,326 B / 633 L (0.16) | 3 / 40,326 B / 632 L (0.16) | 3 / 40,326 B / 632 L (0.16) |
| `~-frame-actions` | 5 / 76,107 B / 1,465 L (0.37) | 5 / 76,107 B / 1,304 L (0.33) | 5 / 76,107 B / 1,304 L (0.33) |
| `~-frame-job` | 11 / 118,723 B / 1,387 L (0.45) | 10 / 115,403 B / 1,126 L (0.44) | 10 / 115,403 B / 1,126 L (0.44) |
| `~-frame-tests-1` | 19 / 129,262 B / 1,022 L (0.59) | 13 / 85,573 B / 840 L (0.41) | 13 / 85,573 B / 840 L (0.41) |
| `~-host-winit` | 15 / 198,671 B / 2,981 L (0.76) | 10 / 181,228 B / 1,869 L (0.69) | 10 / 181,228 B / 1,869 L (0.69) |
| `~-kernel-pool` | 6 / 144,023 B / 2,316 L (0.58) | 6 / 144,023 B / 2,316 L (0.58) | 6 / 144,023 B / 2,316 L (0.58) |
| `~-kernel-protocol` | 9 / 130,307 B / 2,707 L (0.68) | 9 / 130,307 B / 2,707 L (0.68) | 9 / 130,307 B / 2,707 L (0.68) |
| `~-kernel-runtime` | 7 / 152,540 B / 2,388 L (0.60) | 7 / 152,540 B / 2,388 L (0.60) | 7 / 152,540 B / 2,388 L (0.60) |
| `~-native-probe` | 5 / 29,670 B / 497 L (0.16) | 3 / 24,467 B / 150 L (0.09) | 3 / 24,467 B / 150 L (0.09) |
| `~-program-bridge` | 13 / 155,466 B / 1,662 L (0.59) | 9 / 139,332 B / 1,341 L (0.53) | 9 / 139,332 B / 1,341 L (0.53) |
| `~-runtime-mailbox-tests` | 15 / 218,233 B / 2,799 L (0.83) | 5 / 192,910 B / 1,715 L (0.74) | 5 / 192,910 B / 1,713 L (0.74) |
| `~-scene-ink` | 9 / 162,100 B / 3,209 L (0.80) | 9 / 162,100 B / 3,209 L (0.80) | 9 / 162,100 B / 3,209 L (0.80) |
| `~-scene-ink-tests` | 1 / 228 B / 4 L (0.03) | 1 / 228 B / 4 L (0.03) | 1 / 228 B / 4 L (0.03) |
| `~-scene-state` | 9 / 123,919 B / 2,758 L (0.69) | 9 / 123,919 B / 2,744 L (0.69) | 9 / 123,919 B / 2,744 L (0.69) |
| `~-scene-tables` | 14 / 172,345 B / 2,645 L (0.66) | 14 / 172,345 B / 2,645 L (0.66) | 14 / 172,345 B / 2,645 L (0.66) |
| `~-shell-tests-chrome-1` | 12 / 213,241 B / 2,671 L (0.81) | 1 / 7,165 B / 59 L (0.03) | 1 / 7,165 B / 58 L (0.03) |
| `~-shell-tests-chrome-2` | 2 / 16,533 B / 291 L (0.07) | 1 / 1,994 B / 29 L (0.03) | 1 / 1,994 B / 29 L (0.03) |
| `~-shell-tests-chrome-3` | 11 / 176,318 B / 1,309 L (0.67) | 11 / 176,318 B / 1,309 L (0.67) | 11 / 176,318 B / 1,309 L (0.67) |
| `~-shell-tests-chrome-4` | 11 / 196,706 B / 2,013 L (0.75) | 11 / 196,706 B / 1,981 L (0.75) | 11 / 196,706 B / 1,981 L (0.75) |
| `~-shell-tests-chrome-5` | 13 / 218,303 B / 2,547 L (0.83) | 4 / 62,996 B / 671 L (0.24) | 4 / 62,996 B / 670 L (0.24) |
| `~-shell-tests-chrome-6` | 19 / 201,381 B / 2,365 L (0.77) | 6 / 63,931 B / 691 L (0.24) | 6 / 63,931 B / 691 L (0.24) |
| `~-shell-tests-docs-1` | 8 / 183,186 B / 2,095 L (0.70) | 1 / 5,191 B / 61 L (0.03) | 1 / 5,191 B / 60 L (0.03) |
| `~-shell-tests-docs-2` | 16 / 226,336 B / 2,776 L (0.86) | 13 / 199,547 B / 1,547 L (0.76) | 13 / 199,547 B / 1,546 L (0.76) |
| `~-shell-tests-docs-3` | 21 / 129,197 B / 1,306 L (0.66) | 12 / 62,533 B / 736 L (0.38) | 12 / 62,533 B / 735 L (0.38) |
| `~-shell-tests-input-1` | 14 / 236,823 B / 2,558 L (0.90) | 14 / 236,823 B / 2,558 L (0.90) | 14 / 236,823 B / 2,557 L (0.90) |
| `~-shell-tests-input-2` | 8 / 166,330 B / 2,192 L (0.63) | 8 / 166,330 B / 1,743 L (0.63) | 8 / 166,330 B / 1,742 L (0.63) |
| `~-shell-tests-input-3` | 14 / 152,025 B / 1,859 L (0.58) | 12 / 110,540 B / 1,285 L (0.42) | 12 / 110,540 B / 1,285 L (0.42) |
| `~-shell-tests-windows-1` | 13 / 198,770 B / 2,236 L (0.76) | 9 / 128,644 B / 1,501 L (0.49) | 9 / 128,644 B / 1,500 L (0.49) |
| `~-shell-tests-windows-2` | 17 / 188,645 B / 2,240 L (0.72) | 9 / 99,794 B / 1,147 L (0.38) | 9 / 99,794 B / 1,146 L (0.38) |
| `~-test-support` | 6 / 46,661 B / 840 L (0.21) | 6 / 46,661 B / 840 L (0.21) | 6 / 46,661 B / 840 L (0.21) |
| `~-ui-engine-tests` | 18 / 211,539 B / 2,275 L (0.81) | 18 / 211,539 B / 2,274 L (0.81) | 18 / 211,539 B / 2,274 L (0.81) |

Test units over the gate on any label: **0** (35 crates, 3 labels, 0 error rows). The top crate went from 592 files / 9,312,360 B / 38,108 lines (35.5x) to 22 files / 113,494 B / 1,397 lines (0.69). The over-limit dev-dependency rows that remain (`semio-framework-ui`, `semio-framework-plugin`; top crate, `frame-job`, `host-winit`, `program-bridge`) are normal dependencies of those crates that existed before this slice: foreign (peer UI and plugin carves).

## 5. Verification (every line names the command and what it returned)

| check | command (from `F`) | result |
|---|---|---|
| census of the top crate, 36 bins/companions/support/edited crates, native + `wasm32-unknown-unknown` + `wasm32-wasip2` | `python3 rtb-tools/census_run.py final` (`bun rtb-tools/rtb-census.ts`, copy of `otcp-census.ts`) | 37 crates, 16,895 units, 0 error units, **0 test units over the gate on any label**; worst rows `dock` native 3,774 lines (0.94), `engine-attach-tests` and `shell-tests-input-1` 0.90; `F/🗑️generated/rtb-census-final.log` |
| test-name multiset | `python3 rtb-tools/testnames.py compare` | 1,787 baseline names, 1,789 now; by name the multiset is identical except the 2 new `test-support` tests (the 10 Dock shell-bound laws changed file, not name) |
| duplicate mounts / unmounted tests | `python3 rtb-tools/dupscan.py`, `python3 rtb-tools/orphans.py` | 97 packages, 754 files reached, **0 files reached by more than one package**; 1 test file reached by no package (`🧱️elements/🕸️NodeGraph/📡️scene/🧪️tests`), unmounted in the pre-edit snapshot too; 3 files only the top crate reaches (the kept ones) |
| module edges deleted with the chunks | `python3 rtb-tools/chunkrefs.py` | only the production `📥️staging/🎛️prefs/⏪️time-travel` still reaches `⏪️time-travel/🚦️refusals`/`🗣️labels`, which are kept |
| rustfmt | `rustfmt +nightly-2026-07-20 --check` on every new file; `python3 rtb-tools/fmt_roots.py` | the new `.rs` files and the 20 bin roots are formatted (roots idempotent); the 115 edited or new `.rs` files parse (`--config skip_children=true`, 0 errors); pre-existing files were not reformatted |
| path lengths | `python3 rtb-tools/pathscan.py` | longest path of a new or changed file 229 bytes (limit 256) |
| registration | `bun rwp0-tools/verify_static.ts` after `python3 rtb-tools/freeze_rows.py` | 90 crates, 2,261 path dependencies resolved, 5 failures, all caused by the peer crate `native-runtime-modules` / `⌨️native-entrypoint/📦️modules` (section 6.4) |
| union resolution oracle (the ri2 oracle generalised: 89 crates mounted as modules of one crate, `pub(crate)` → `pub(in crate::<crate>)`, `--test` and `--test --target wasm32-unknown-unknown`) | `python3 rtb-tools/uo.py rtb5 --mode crates --configs test,wasm-test --crates real` | native: 2,201 errors, signal rows after removing the 95 `action_args_json` rows and the include-path rows: 4, all foreign or oracle artifacts (6.1); wasm: no new row except HEAD-stale names (6.2). Earlier runs found and I fixed: stray `use test_app;` (4 files), `appearance_tour_and_footer_pill_tests::tour_*` in host-door-remainder, the private `native_socket_sqlite`, the missing `fixtures` mount, the cross-bin `engine_surface_attach_tests::*` helpers, the Dock privacy, root imports without wasm gating (`native_socket_sqlite`, `CANCELLED_KERNEL_TURNS`) |
| stub-layer typeck oracle (`frs-tools/stubgen.py` over the union) | `python3 rtb-tools/tc.py tc2 --configs test --only-a` | **not achieved**: the stubbed union still has 2,400 resolution errors (serde derives and `json!`, tuple-vs-unit enum variants, trait stubs, `infinite_world::world`), so rustc never reaches typeck; the otcp precedent needs a per-crate seed configuration for ~80 external crates. Replaced by the restricted-visibility scan (6.3) |
| cargo | `bun repo cargo --store=shared --configuration=prototype -- metadata --offline --no-deps` | refused by the ingress: `Exact canonical persistent Native publication required`; nothing was compiled |

## 6. What is not clean, and why it is not this slice's

### 6.1 Foreign or oracle-artifact rows (present in the pre-edit baseline run `rtb-uo-base0.log` too)
`host_base::action_args_json` (named by 95 sites, defined nowhere), `::semio_framework_os_kernel` global paths in the sqlite law, the `include_str!(concat!(env!("CARGO_MANIFEST_DIR"), …))` rows of the Scenes laws (the oracle sets another manifest dir), `scene_text_vfs::interpreter` in the text-editor law, `native_runtime_modules` (peer crate), and the enqueue law outside the sandbox (mounted read-only, so its crate imports stay unresolved in the oracle).

### 6.2 HEAD-stale names (the laws did not compile in the monolith either)
Found by the wasm config because the monolith run stops before late resolution: `defers_descriptor_sync_from_js` and a free `wheel_screen` (defined nowhere), `caller` in `frame_deferred_cursor_advances_one_owned_operation_in_order` and 11 more uses (no `let caller`), `UiComponentSceneNode` in the Scenes standalone (the HEAD facade does not import it either), native-only items used by ungated laws (`DirectoryHomeProjection`, `pump_renderer_io_sessions`, `ComponentSurfaceCloseToken` construction). The p3c law is now gated `not(wasm32)` because it names the native-only `ime_event_from_winit`. I did not repair the others: they need the real compiler to fix them for good.

### 6.3 Restricted visibility across crates (E0616/E0624/E0603 candidates for the first real compile)
`python3 rtb-tools/restrict_scan.py` lists 6 method names and 92 field names with a restricted visibility (`pub(crate)`, `pub(super)`, private) that a test file mounted by another crate names; most are name collisions (`read`, `send`, `first`, `view`, …) and the Json-schema structs read through `serde_json`. The same accesses existed at HEAD (the top crate was a different crate than `scene-state`, `engine-text`, `agent-elements`, `asset-decode` there too): `SceneSurfaceState` `pub(crate)` fields (`ink_camera`, `ink_marquee_points`, `last_click_*`, `map_*`, `paint_stroke_active`, `pointer_was_down`, `camera_dispatch_controller_id`, `drag`, `selected_ids`) and `SceneListTransferSession.start_x/y` read by the Scenes standalone law (`engine-attach-tests`) and by the in-crate ink/canvas2d laws; `EditorSceneDocument.buffer` (`pub(super)`), `AgentBridgeState.outbox`, `EngineCanvasPacketReservation.destination`. Output: `F/🗑️generated/rtb-restrict-scan.txt`. I widened only what the scan proved real for a placement I made (`GraphInteractionDispatch`, `native_socket_sqlite`) and moved the Dock laws in-crate instead of widening 10 Dock items.

### 6.4 Registry failures of `verify_static.ts` that are the peer's
`program-bridge` depends on the peer crate `native-runtime-modules` (not in the frozen table), the wgpu member block has the peer's `⌨️native-entrypoint/📦️modules/…` entry out of byte order, and both launch files carry 6 routes of that crate (546 = 91 × 6, the table says 90).

## 7. Hand-offs

1. First real compile (when the ingress allows it), per crate so a failure names its owner: `cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-renderer-wgpu --tests --keep-going --message-format=short`, then the same with `-p semio-framework-os-renderer-wgpu-<bin>` for the 15 `shell-tests-*` bins, `engine-attach-tests`, `ui-engine-tests`, `runtime-mailbox-tests`, `frame-tests-1`, `fixtures`, `test-support`, `host-winit`, `frame-job`, `dock`, and once more with `--target wasm32-unknown-unknown` and `--target wasm32-wasip2`. Expect the 6.2 and 6.3 families first.
2. `dock` native lib-test is 3,774 of 4,000 lines: the next Dock law needs a bin (move the pure-dock laws of `🔬️wgpu-unit` that need only `DockState` first).
3. `engine-attach-tests` (0.90 bytes) and `shell-tests-input-1` (0.90 bytes) have the least headroom; their biggest remaining embeds are the `paint2d` and `shell-input` laws' JSON fixtures (use `EMBED_LIMIT=8000 python3 rtb-tools/srcread.py <file> --apply`).
4. Renderer TS (slice `rhu2`) was not touched.
5. Registry: the frozen table `🗑️generated/rwp0-crates.json` was refreshed from the manifests by `freeze_rows.py` (it also corrects the stale first-party sets of the earlier bins and companions); `crates.py` was not changed, so regenerating the table from the plan would drop my rows.

