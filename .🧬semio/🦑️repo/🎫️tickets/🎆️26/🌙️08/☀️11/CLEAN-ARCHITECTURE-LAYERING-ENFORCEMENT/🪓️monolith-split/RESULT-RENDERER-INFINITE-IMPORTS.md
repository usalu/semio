# 📥️ Result RENDERER-INFINITE-IMPORTS (slice `rii`, 2026-10-16)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`, `T` = `E/🎯️targets/🧊️wgpu`, `B` = `E/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu`, `INF` = `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed`. Tools: `F/rii-tools/`. Logs and oracle output: `F/🗑️generated/rii-*` (the oracle writes `rii-rii-<label>-*` through `POS_PREFIX=rii`). Pre-edit copies of every touched file: scratchpad `rii/pre/` (wiped on process restart; the list is `rii-tools/touched.txt`).

## 0. Verdict

**GREEN against the oracle, UNCOMPILED.**

1. **Target 1: done.** The 66 unimported free functions (28 lib rows, 38 test rows) resolve now, plus 1 more test row the original list missed (`wheel_screen`, section 2.3): every one is imported explicitly from the `semio-framework-os-infinite-board-*` crate that owns it, and each of the five renderer manifests got the matching edges (direction renderer -> infinite only; no infinite crate depends on a renderer crate, section 2.4). Oracle with the 11 board crates **real** in the union: own rows 28 + 39 -> **0 + 0**, **0 new rows** against the baseline (section 4.1), canary (E0308, E0616, E0624) reached in every run.
2. **Target 2: done, with a different mechanism than the hint.** The 24 over-gate dev rows (8 crates x 3 labels) are gone: `rii-census.ts` over the 17 crates of the WP2 family says **0 over-gate dev rows, 0 over-gate test units, 0 error units** (baseline of the same tool and crate set: 24 rows). Gating a part of `scene-state` behind a feature cannot do it (section 3.1: the census follows the normal `ui` edge of `scene-state` itself); a dependency-free leaf crate that carries the isolation switch does (section 3.2).
3. While proving the imports I found and fixed 6 defects the oracle cannot see or that sat in the same files (section 2.5): a bare `use dag::…` head, five self-crate-name imports in laws, one stale duplicate import.
4. **Not proven by a compiler.** The sanctioned launcher was tried once on the new crate and sat in its `running` stage until my 900 s alarm (section 5). Method resolution, borrowck and trait coherence remain unverified.

## 1. Found state (before the first edit)

* Oracle run `b0` (family `rii.json` = the 92 crates of `rw2t.json` + the 11 board crates, lib and `--test`, `--no-base`): **28 lib / 39 test** `E0425` rows for the names below in the five WP2 files, canary reached. The slice brief says 38 test rows: the 39th is `wheel_screen` in the `EngineCanvas/🧪️tests/🧊️wgpu-standalone` helper `board2d_wheel` (a `#[cfg(test)]` fn, line 495), which the earlier list did not name. It sits in the EngineCanvas standalone law that `engine-attach-tests` mounts.
* Census (`rii-census.ts`, 17 crates): **24 over-gate dev-dependency rows** (8 native, 3 labels), all `semio-framework-ui` (31 files, 1,109,244 B, 19,372 lines), `direct=false`, `alsoNormal=true`, for canvas2d-host, scene-canvas2d, scene-ink, scene-feeds, scene-tables, scene-text-vfs, icon-host and scene-render.
* No `RESULT-RENDERER-PRIVACY.md` existed at any time while I worked; slice `rpv` was active (`rpv-*` logs, `rpv-a1-table.md` lists the `scene_state` members it widens). I did not touch a field, a visibility or a `pub(...)` outside the two `pick_target_hits` / `selection_domains` signatures (section 3.3) and the seven items of 2.5; I edited only `use` lines elsewhere. If `rpv` changes `paint2d` it must keep those two `pub`.
* Intent confirmed by read-only history: `git show 1bae6115bc5~1:<file>` has the methods (`host.set_camera_silent(…)`, `host.begin_pointer_commit(plan)`, `host.commit_wheel(&plan)`, `host.fit_camera_to_content()`, `host.set_hover_channel(…)`, `host.set_selection_ids_silent(&ids)`, `host.set_transform_flags(…)` in board2d, node-graph and engine-surface-attach); the commit turned them into the component free functions (`oinf` board carve, `RESULT-W4-OS-INFINITE.md` "Board carve") and `HEAD` has 0 imports of any `semio_framework_os_infinite_board_*` crate in the three files. So nothing was removed or renamed: the call sites already have the new shape, the `use` lines and manifest edges are what was missing.

## 2. Target 1: imports and manifest edges

### 2.1 Owners (index `rii-tools/idx.py` over the 11 board crates; every name is unique; `pub fn` with the arity the call sites use)

| name | owner crate (`semio-framework-os-infinite-board-…`) | call sites fixed |
|---|---|---|
| `set_camera_silent`, `commit_wheel`, `set_selection_ids_silent`, `wheel_screen` | `normal-focus` | board2d (`set_camera_silent`, `commit_wheel`), engine-surface-attach (`set_camera_silent`, `set_selection_ids_silent`), law `wgpu-board2d-engine` (`set_camera_silent` x2, `set_selection_ids_silent`), law `wgpu-standalone` (`wheel_screen`) |
| `begin_pointer_commit` | `normal-host` | board2d (2), law `wgpu-board2d-engine` (1) |
| `pointer_lane_is_direct`, `pointer_lane_in_flight`, `set_transform_flags`, `defers_descriptor_sync_from_js` | `normal-gestures` | board2d (2 + 1), law `wgpu-board2d-engine` (2 + 2), engine-surface-attach (`set_transform_flags`), law `wgpu-standalone` (`defers_descriptor_sync_from_js` x2) |
| `fit_camera_to_content`, `hovered_node_id`, `hovered_node_id_ref`, `selection_union_bounds_screen_json`, `set_hover_channel`, `adopt_camera_or_fit`, `refit_camera_if_content_left_view`, `dag_set_wheel_zoom_active` | `dag-graph` | node-graph (1 + 1 + 3 + 1 + 4), engine-surface-attach (1 + 1 + 1) |
| `connection_cursor_active_at_screen`, `screen_hit`, `screen_hit_json`, `screen_geometry_census_json`, `set_dimmed`, `set_node_statuses`, `set_computing_progress` | `dag-host` | node-graph (2 + 1 + 1 + 2), engine-surface-attach (3) |

Totals of the baseline rows by file (lib / test): `B/🎲️board2d` 7 / 7, `B/🕸️node-graph` 12 / 12, `B/🧩️engine-surface-attach` 9 / 9, law `🔬️wgpu-board2d-engine` 0 / 8 (mounted by `ui-engine-tests`), law `🧊️wgpu-standalone` 0 / 3 (mounted by `engine-attach-tests`): 28 / 39.

### 2.2 Edits

* Imports (sorted names, one line per owner crate): `B/🎲️board2d` 3 lines, `B/🕸️node-graph` 2 lines (replacing the dangling `use dag::dag_set_wheel_zoom_active;`), `B/🧩️engine-surface-attach` 4 lines, `T/🧿️ui-engine-tests/🦀️.rs` 3 lines in `mod laws` (the law `wgpu-board2d-engine` does `use super::*`), `T/🧷️engine-attach-tests/🦀️.rs` 2 lines in `mod engine_canvas` (`wheel_screen`, `defers_descriptor_sync_from_js`). No facade (`infinite_canvas::…`) export was added: the facade stays a path replica, the owners are named directly (`RESULT-W4-OS-INFINITE.md`: a later pass retires the `board::` facade paths per consumer).
* Manifests (`[dependencies]`, path edges, first-party order kept; `rii-tools/addep.py` inserts without reordering): engine-board `normal-focus`, `normal-gestures`, `normal-host`; engine-graph `dag-graph`, `dag-host`; engine-attach `dag-graph`, `dag-host`, `normal-focus`, `normal-gestures`; ui-engine-tests `normal-focus`, `normal-gestures`, `normal-host`; engine-attach-tests `normal-focus`, `normal-gestures`.
* `Cargo.lock` (repo root): `rii-tools/locksync.py` (the rtb `lock_sync`) rewrote the dependency lists of these five crates and of the nine crates of target 2 (lists in 3.4); the new package entry was created.

### 2.3 What the first list missed

`wheel_screen` (1 test row, above). Everything else in the original list is in the baseline of `b0` one to one (28 lib, 38 test).

### 2.4 Direction

`rii-tools` greps every `Cargo.toml` under `INF/🎲️board`: the only `ui*` edges are `semio-framework-ui-styling` and, in the three dag crates, `semio-framework-ui` (a lower layer); no board crate names a renderer, `wgpu` shell or `📺️renderer` path. The new edges go renderer -> infinite only, and no cycle is possible (no infinite crate depends on `semio-framework-os-renderer-*`).

### 2.5 Defects found next to the 66 (hidden from the oracle because it stubs unknown crate heads) and fixed

Found with `rii-tools/heads.py` / `reach.py` (every first segment of a `use` path in the files a crate reaches through `#[path]` / `include!` must be a manifest dependency, an `extern crate … as` alias, a std crate or a local name; read-only):

| file | defect | fix |
|---|---|---|
| `B/🕸️node-graph/🦀️.rs:20` | `use dag::dag_set_wheel_zoom_active;`: there is no crate `dag` (the facade `flow::dag` does not export it) | import from `…-board-dag-graph` (part of 2.1) |
| `T/🧿️ui-engine-tests/🦀️.rs:70` and `E/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs:15` | `use semio_framework_os_renderer_wgpu_ui_engine_tests::…` inside the crate itself (E0432) | `use crate::fixture_original_turn;`, `use crate::surface_fixtures::*;` |
| `E/🧱️elements/🎞️Scenes/🧪️tests/{🔬️wgpu-table,🔬️wgpu-block-list,🔬️wgpu-virtual-file-system}/🦀️.rs` | `use semio_framework_os_renderer_wgpu_scene_render::…` inside scene-render's own unit tests (E0432) | `use crate::…` (the lib root `pub use`s `scene_input` and `scene_render_entry`) |
| `E/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-text-editor/🦀️.rs:7` | stale `use framework_widget_context;` next to the real import on line 1 (E0252, the oracle reported it) | line removed |

## 3. Target 2: the 24 over-gate dev rows

### 3.1 Why a feature gate inside `scene-state` cannot work

`tcc-census.ts` (copy `rii-tools/rii-census.ts`, outputs `rii-census-*`) marks a dependency `viaDev` when any dev edge reaches it, then measures it on its own: `semio-framework-ui` is a **normal** dependency of `scene-state`, of `contracts` and of `engine-board`, so every dev edge to `scene-state` (whatever features it requests) makes `ui` a dev row; `alsoNormal=true` on all 24 rows says the aggregate was already in the normal closure. The rows could only vanish if no dev path reached a crate that depends on `ui`, i.e. if the 8 crates stopped dev-depending on any renderer crate. What the dev edge was for: enabling `scene-state/testkit` so that `SceneCell` (native scene state) is per thread in the 8 crates' tests (`RESULT-RENDERER-WP2-TESTS.md` 3.3). Checked: `scene-state`'s other `testkit` items (`camera_fixture_*`, `CameraFixtureLease`, `sweep_expired_scene_camera_dispatches`) are used only by `engine-attach-tests`, `shell-tests-input-1` and the Scenes laws mounted by `scene-ink-tests` / `engine-attach-tests`; `contracts/testkit` and `engine-registry/testkit` (forwarded by `scene-state/testkit`) are used by none of the 8 crates' laws (`WorkerCell`, `UI_ENGINE`, `engine_surface_law_guard`, `fixture_scene_pointer_target` appear only in ui-engine-tests, scene-ink-tests, engine-attach-tests and shell bins).

### 3.2 Mechanism: a dependency-free leaf crate

New crate `semio-framework-os-renderer-wgpu-thread-isolation` (`T/🔒️thread-isolation`, project `@semio-tech/os-renderer-wgpu-thread-isolation-rs`, no dependencies, 32 + 39 lines, `publish = false`):

* `pub const ISOLATED: bool = cfg!(feature = "isolated");` and `pub fn isolated_cell<T: 'static>(key, initialize) -> &'static Mutex<T>` (the per-thread leaked store that `element_contracts::test_worker_cell` held).
* `scene-state` depends on it; `SceneCell::with` takes `isolated_cell` when `cfg!(test) || ISOLATED` and the process-wide `OnceLock<Mutex<…>>` otherwise (the `cfg(any(test, feature = "testkit"))` struct and `PhantomData` variants of `rw2t` are gone: one struct, one branch). `scene-state`'s `testkit` feature forwards `…-thread-isolation/isolated`, so the two bins that enable `scene-state/testkit` (`scene-ink-tests`, `engine-attach-tests`) keep the per-thread cell (`ui-engine-tests` never enabled it and does not now).
* `contracts` now re-exports the same function as `test_worker_cell` under its existing `cfg(any(test, feature = "testkit"))` (the private copy of that function and its `thread_local` in `T/🧬️element-contracts/🦀️.rs` is deleted; repeated code stays in one place). `contracts` gets the normal dependency.
* The 8 crates replace `scene-state = { …, features = ["testkit"] }` in `[dev-dependencies]` by `semio-framework-os-renderer-wgpu-thread-isolation = { path = "../../../🔒️thread-isolation/📦️packages/🦀️rust", features = ["isolated"] }`. The dev edge reaches one crate without dependencies; cargo's feature unification enables `isolated` for the whole test build, so `scene-state` (a normal dependency of each) sees it.

### 3.3 The second source of rows: engine-board `testkit`

`scene-render` also dev-depended on `engine-board[testkit]` for the two `*_for_test` wrappers of `paint2d` (another dev path into `ui`). `pick_target_hits` and `selection_domains` are plain JSON parsers; they are `pub` now, the two wrappers and the `engine-board` `testkit` feature are deleted, `ui-layout` is back in `[dev-dependencies]` of engine-board as at `HEAD`, the law `🔬️wgpu-surface-context-menu` calls the functions directly and scene-render lost the dev edge (`B/🖌️paint2d/🦀️.rs`, the law, two manifests).

### 3.4 Registration of the new crate (files)

Root `Cargo.toml` (`members`, byte-sorted position between `🔐️hub-contracts` and `🔗️engine-attach`), `📋️build-owners.json` (1 row), `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (six routes each, cloned from the `test-support` routes so each file keeps its layout, orders `900.24581/2`, `922.0538/05381`, `903.5938/59381`, unused before and equal in both files), `🔣️taxonomy.json` (`members-of-wgpu-target`: `🔒️thread-isolation`; 🔒 is an RGI emoji used as a folder prefix elsewhere and unused among the siblings of `T`), `Cargo.lock` (new package, `thread-isolation` added to the lists of `contracts`, `scene-state` and the 8 crates). `rii-tools/step3.py` is the registrar (built on `rtb-tools/register.py`).

## 4. Verification (every line names the command and its real result)

### 4.1 Oracle (privacy/type oracle `pos.py`, rustc `--emit=metadata` over the stub layer; family `rii-tools/rii.json` = the 92 crates of `rw2t` + the 11 board crates + the new crate = 104; lib and `--test`; absolute rows, `--no-base`)

`cd F/pos-tools && POS_PREFIX=rii python3 pos.py run ../rii-tools/rii.json --cfgs lib,test --label <L> --no-base` (the manifest index was rebuilt with `pos.py index --force` before `e1` because the new crate was missing from it). Canary E0308, E0616 and E0624 reached in every run (`b0`, `c1`, `d2`, `e1`; bodies were type-checked). `rii-tools/rows.py` counts the `E0425` rows of the 23 names; `rii-tools/diffrows.py` compares the other rows of the 19 family units (key: code, message, source line, file).

| run | tree | own rows lib / test | family rows lib / test | new vs `b0` |
|---|---|---|---|---|
| `b0` | before my first edit | **28 / 39** | 160 / 194 | - |
| `c1` | after step 1 (imports + edges) | 0 / 0 | 132 / 156 | 0 / 0 |
| `d2` | after targets 1 and 2 | 0 / 0 | 132 / 154 | 1 / 1 (see below) |
| `e1` | final tree, new crate in the union | **0 / 0** | **131 / 153** | **0 / 0** |

`d2` showed the one row the stub layer makes when the new crate is outside the union (`E0308 if/else incompatible` in `SceneCell::with` because the stub of `isolated_cell` has no real return type); with the crate in the union (`e1`) it is gone, the control that the oracle looks at the new code. The rows that left between `b0` and `e1` are exactly: 29 lib / 41 test = the 28 / 38 originals + `wheel_screen` (1 test) + the `E0252` duplicate import of the text editor law (1 test) + the `element_contracts::test_worker_cell` extern-alias artifact of `scene-state` (1 lib, 1 test; the call is gone). Privacy rows in the 19 family units: **0 / 0** in `b0` and in `e1`. The 131 / 153 remaining family rows are the baseline (section 6).

### 4.2 Static gates

| check | command | result |
|---|---|---|
| unresolved heads | `python3 rii-tools/reach.py <17 manifest dirs>` (+ `heads.py`) | 0 unresolved heads in all 17 crates after the fixes of 2.5 (before: 7 in 4 crates) |
| imported names exist | `rii-tools/idx.py` index vs. every imported name | all 23 names are `pub fn` of the crate they are imported from, 0 mismatches |
| call arity | `python3 rii-tools/arity.py` over board2d, node-graph, engine-surface-attach and the two laws | 42 calls checked, 0 argument-count mismatches against the definitions |
| test-name multiset | `python3 rii-tools/names.py` (pre-edit copy vs live, every touched `.rs`) | 11 tests before, 11 after, 0 files differ: IDENTICAL; the new crate adds 4 tests (3 laws + the feature-off law; the feature-on twin is cfg-gated) |
| duplicate mounts | `python3 rtb-tools/dupscan.py` | 98 packages, 757 files reached, **0 files reached by more than one package** |
| unmounted laws | `python3 rtb-tools/orphans.py` | the same 1 orphan as before (`NodeGraph/📡️scene/🧪️tests`) |
| paths | length of the 33 touched paths (`touched.txt`) | longest 193 bytes repo-relative (limit 240 / 256 incl. the checkout prefix) |
| rustfmt parse | `rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --emit stdout` on the 16 touched `.rs` | 0 parse errors |
| rustfmt style | `rustfmt … --style-edition 2024 --config-path rustfmt.toml --check` | the two new files are clean; my import lines in the other files follow the file's own ordering (those files were not rustfmt-clean before, `RESULT-RENDERER-WP2-TESTS.md` 4.3), the `SceneCell::with` body is written in the form rustfmt picks |
| manifests / registries | `Bun.TOML.parse` of the 16 edited manifests and the root `Cargo.toml`; `json.load` of `📋️build-owners.json`, `🔣️taxonomy.json`, `.vscode/launch.json` | all parse; `launch.json` names unique for the new routes, orders unique per group |
| census | `bun rii-tools/rii-census.ts` with `ONLY=<17 crates>` (`rii-census-base.log` before, `rii-census-final.log` after) | **before 24 over-gate dev rows (8 native) -> after 0**, 0 over-gate test units, 0 error units; worst test-unit ratio 0.921 (engine-attach-tests, was 0.920) |

Positive control of the census: the baseline run of the same tool on the same crates over the tree of `rw2t` reports the 24 rows, so the tool does see this class; the removal of exactly the dev edge to a crate that reaches `ui` is what removes them (not a shrunken crate set: `ONLY` is identical, units 11,232 -> 3,206 because the transitive dev-dependency units are gone). A census of `contracts` on top (not in the 24) shows 3 rows (1 native) from its own old dev edge `infinite_canvas`; that edge is not mine and `contracts` is outside the 8 crates.

## 5. Real-check commands (for the compile queue)

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-renderer-wgpu-thread-isolation -p semio-framework-os-renderer-wgpu-contracts -p semio-framework-os-renderer-wgpu-scene-state -p semio-framework-os-renderer-wgpu-engine-board -p semio-framework-os-renderer-wgpu-engine-graph -p semio-framework-os-renderer-wgpu-engine-attach --lib --keep-going --message-format=short
```
then the same with `--tests` and the further `-p` set: `…-engine-attach-tests`, `…-ui-engine-tests`, `…-scene-render`, `…-canvas2d-host`, `…-scene-canvas2d`, `…-scene-ink`, `…-scene-feeds`, `…-scene-tables`, `…-scene-text-vfs`, `…-icon-host`; then `test -p semio-framework-os-renderer-wgpu-thread-isolation --lib` (4 laws; with `--features isolated` the fifth). A wasm pass (`--target wasm32-unknown-unknown --lib`) covers `contracts` and `scene-state` (the new crate is plain std).

Tried once under the fleet mutex (`cargo-mutex.lock`, owner `rii`, released): the same command for `-p semio-framework-os-renderer-wgpu-thread-isolation --lib --tests` (`F/🗑️generated/rii-cargo-isolation.log`) printed `stage: loading`, `stage: running` and then nothing until my own `alarm 900` ended it (`error: script "repo" was terminated by signal SIGALRM`, exit 142); the launcher process sat at ~85 % CPU. No cargo output, no rustc, so this is **not** a compile result. The root `Cargo.lock` was edited by hand (`locksync.py`, the same structure the rtb tool wrote for earlier crates); if the launcher says the lock is stale, run `-- update --workspace --offline` first.

Likeliest first compile faults, in order: (1) method resolution / borrowck at the rewritten call sites, which now borrow disjoint components (`&mut host.dag.viewport, &mut host.dag.graph`) of `GraphHost` / `FlowHost` values the oracle sees only as stubs (`flow::dag::dag_screen_to_world` at node-graph line 257 already shows a stub-layer `E0308`); (2) `unused import` warnings if a law mentions a name only in a comment; (3) the `thread-isolation` `isolated` feature must reach `scene-state` through the dev edge of the 8 crates: if a crate's laws show shared scene state, check `cargo tree -e features -p <crate> --edges dev`.

## 6. What is not clean, and why it is not this slice's

* Baseline oracle rows of the family units (131 lib / 153 test), unchanged: `extern crate X as Y` alias heads (`element_contracts`, `asset_authority`), the `map_tiles` rename inside a brace group, the `Self::Error` trait stub in node-graph, `__ext::serde_json` stub types, and `UI_ENGINE` method rows of the `🧪️tests` module of ui-engine-tests (`apply_tree`, `surface_token`, `tree_revision`, `tree`: the stub layer cannot see the `Ui` type) are the documented oracle-artifact class (`RESULT-RENDERER-WP2-TESTS.md` 6.1).
* One row looks genuine and is not an import: `E0560 struct EngineCanvasPacket has no field scene_retirement_faulted` in the law `EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs:110` (engine-attach-tests, test configuration; present in `b0`). It is an API drift of the engine-registry packet, owner `rw2t` / the registry slice; I did not guess its new shape.
* `NodeGraphEngine::Flow(host)` arms use `host.dag.viewport` etc. and `Flow(host).fit_camera_to_content()` keeps its method: `FlowHost` still exposes both shapes, as at the parent commit; I checked arity and names only.
* The mechanism of 3.2 covers `SceneCell`. `WorkerCell` (contracts) still switches on its own `testkit`; the same leaf could drive it (`cfg!(test) || ISOLATED`) once a crate other than the 8 needs it. Not done here to keep the contracts edit to the deleted duplicate.
* AGENTS rule "same output of a test with a third-party library": the new crate's laws are properties of a thread-local map; no third-party crate does the same job offline, so there is no second implementation to compare.

## 7. Hand-offs

1. Compile queue: section 5. The first run also tells whether the hand-written lock entries are accepted.
2. `rpv`: no overlap in members; if it widens or narrows `pick_target_hits` / `selection_domains` (`B/🖌️paint2d`) the law `🔬️wgpu-surface-context-menu` of scene-render needs both `pub`.
3. Registry slice: `EngineCanvasPacket.scene_retirement_faulted` (section 6).
4. The `semio-framework-os-infinite` facade (`infinite_canvas`) can lose the `board::` replica paths per consumer; the renderer no longer needs the 23 functions from it.

## 8. Files

Created: `T/🔒️thread-isolation/🦀️.rs`, `T/🔒️thread-isolation/🧪️tests/🦀️.rs`, `T/🔒️thread-isolation/📦️packages/🦀️rust/Cargo.toml`, `…/📋️project.json`; `F/RESULT-RENDERER-INFINITE-IMPORTS.md`; `F/rii-tools/` (`idx.py`, `heads.py`, `reach.py`, `arity.py`, `rows.py`, `diffrows.py`, `names.py`, `ed.py`, `addep.py`, `locksync.py`, `step1.py`, `step2.py`, `step3.py`, `rii-census.ts`, `rii.json`, `touched.txt`).
Edited (repo): `B/🎲️board2d`, `B/🕸️node-graph`, `B/🧩️engine-surface-attach`, `B/🖌️paint2d` (lib roots `🦀️.rs`); `T/🧿️ui-engine-tests/🦀️.rs`, `T/🧷️engine-attach-tests/🦀️.rs`; `T/🎭️scene-state/🦀️.rs`; `T/🧬️element-contracts/🦀️.rs`; the laws `E/🧱️elements/🎞️Scenes/🧪️tests/{🔬️wgpu-surface-context-menu,🔬️wgpu-table,🔬️wgpu-block-list,🔬️wgpu-virtual-file-system,🔬️wgpu-text-editor}/🦀️.rs` and `B/../🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs`; manifests of engine-board, engine-graph, engine-attach, engine-attach-tests, ui-engine-tests, scene-state, contracts, canvas2d-host, scene-canvas2d, scene-ink, scene-feeds, scene-tables, scene-text-vfs, icon-host, scene-render; root `Cargo.toml`, `Cargo.lock`, `📋️build-owners.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`.
Deleted: none (the two `*_for_test` wrappers, the private `TEST_WORKER_CELLS` copy and the `engine-board` `testkit` feature were removed from files).
