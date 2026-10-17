# 🧪️ Result RENDERER-TEST-BINS (slice `rtb`, 2026-10-16)

Executor: Sonnet 5.5, foreground, no sub-agents, no ticket tool, no modifying git command. `F` = this folder. Tools: `F/rtb-tools/` (every script named below). Logs and census outputs: `F/🗑️generated/rtb-*`. Scratch (pre-edit snapshot `snap0`, first-edit copies `pre/`, oracle sandboxes): the session scratchpad `rtb/`.

## 0. Verdict

{{VERDICT}}

## 1. Task, as executed

The `semio-framework-os-renderer-wgpu` (top crate) test unit was the one unit still over the Rust source gate (4,000 active lines / 32 files / 262,144 bytes): it compiled 93 source files (lib adapter, renderer root, five hubs, the Shell/EngineCanvas/Scenes/Interpreter facades, 66+ law files) and 499 embedded inputs (356 `.rs` source texts, 140 JSON, 2 GLB, 1 TOML). The slice moves every law out of it into test bins and companion crates (Appendix D packing), moves the shared fixtures into the lowest crate that can name what they need, turns the source-text laws into run-time reads, and registers everything.

{{BASELINE}}

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

{{BINS}}

`fixtures` and `test-support` are support crates; `scene-ink-tests` stays the skeleton it was (its only law mount moved to `engine-attach-tests`/`ui-engine-tests` with the helpers it shares).

## 4. Census after (`bun F/rtb-tools/rtb-census.ts`, lib-test unit of every crate the slice created or edited; files / bytes / lines (worst ratio of the gate))

{{CENSUS}}

Test units over the gate on any label: **0** (35 crates, 3 labels, 0 error rows). The top crate went from 592 files / 9,312,360 B / 38,108 lines (35.5x) to 22 files / 113,494 B / 1,397 lines (0.69). The over-limit dev-dependency rows that remain (`semio-framework-ui`, `semio-framework-plugin`; top crate, `frame-job`, `host-winit`, `program-bridge`) are normal dependencies of those crates that existed before this slice: foreign (peer UI and plugin carves).

{{EXTRA}}
