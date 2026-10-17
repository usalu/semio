# 🧪️ Result RENDERER-WP2-TESTS (slice `rw2t`, 2026-10-16)

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`, `T` = `E/🎯️targets/🧊️wgpu`, `EL` = `E/🧱️elements`. Tools: `F/rw2t-tools/`. Logs and oracle output: `F/🗑️generated/rw2t-*` (the oracle writes `rw2t-rw2t-<label>-*` through `POS_PREFIX=rw2t`; the first three runs `b0`, `b1`, `c1` were started before I found that switch and are `pos-rw2t-*`). Pre-edit copies of every touched file: scratchpad `rw2t/pre/` (wiped on process restart; the list is `rw2t-tools/touched.txt`).

## 0. Verdict

**GREEN against the oracle, UNCOMPILED.** Every WP2 crate's test unit and the three bins that host WP2 laws now resolve and pass the privacy check in the privacy/type oracle (`pos.py`, canary reached: E0308, E0616 and E0624 raised in every run): **0 resolve rows and 0 privacy rows are attributable to the carve** in lib and test configurations. What remains in those units is listed in section 6 and is not the carve's: 72 + 74 rows that are `extern crate X as Y` alias heads or stub-layer gaps of the oracle (documented class since `RESULT-RENDERER-INTEGRATION-3.md` 2.4), and 28 + 38 rows that are the unimported free functions of the `infinite` board/dag API migration (commit `1bae6115bc5`, a different actor).

What this does NOT prove: nothing was compiled by cargo (compile admission is refused this wave), method resolution against the real external crates, borrowck and trait coherence are outside the oracle. Section 7 has the real-check commands.

Two facts about the numbers you will see elsewhere:
* The baseline revision `1bae6115bc5` already contains the WP2 crates, so a "NEW against the baseline" count (`ri2`, `ri3`: 5 lib privacy rows, 414 test resolve rows) hides everything the baseline shares. I measured **absolute** rows (no baseline) in the WP2 files and classified each (section 5); the official `NEW` form is in 4.2 as run `f1`.
* The oracle's `lib` configuration showed 270 privacy rows in the WP2 files, not 5. They are real cross-crate field and method reads of `pub(crate)` and private members, present in the baseline too.

## 1. Found state (before the first edit)

* Every WP2 crate's own `🧪️tests/🦀️.rs` was the empty skeleton line; the laws were mounted by the element-tree files (`#[cfg(test)] #[path] mod x_tests;`) and read `use super::*` against the old Scenes module. Oracle run `b0` (absolute, tree of 20:12, 92 crates): test resolve rows in the WP2 files **316**, test privacy rows **394**, lib privacy rows **270**, lib resolve rows 1 (carve) plus 71 oracle artifacts plus 28 foreign.
* Seven laws (and two tests of an eighth) named items of crates their host could not depend on, and the laws carried the monolith heads `crate::scenes::*`, `crate::interpreter::*`, `crate::element_contracts::*`, `crate::canvas2d_gumball::*`, `crate::collect_fixture_actions`, `crate::app_now_ms` and, in the context menu law, `engine_canvas::*_for_test`.
* Peers edited the same files during the slice: `pos2` applied privacy widenings (it made the `SceneListTransferSession`, `BlockListTarget` and `TextEditorTypingRun` fields `pub` while I ran), someone removed 34 lines of the `Scenes` facade `E/EL/🎞️Scenes/T/🦀️.rs`. Every edit of mine re-reads the file and asserts anchors (`ed.py`, `pos2widen.py apply` asserts the exact line).

## 2. Placement (rtb bin layout, plan 9.3 with the test-closure facts)

A law stays in its crate when every first-party name it uses comes from that crate's dependency closure; otherwise it moves to the lowest bin that can name everything. `F/rw2t-tools/plan.py` computes the owner of every free name of a law from the namespace of its old parent module (`ns.py`) and the export index of the 92 crates (`idx.py`); the numbers below are its output.

| law (file in `EL/🎞️Scenes/🧪️tests` unless noted) | was mounted by | now mounted by | why |
|---|---|---|---|
| `🔬️wgpu-canvas2d` (minus two tests) | scene-canvas2d | **scene-ink-tests** | names `canvas_*_into` (scene-render), `ui-engine::canvas_catalogue_drop`, `frame-actions::collect_fixture_actions` |
| `📷️wgpu-canvas2d-camera` (new file, two tests moved out of the law above, names unchanged) | n/a | **engine-attach-tests**, in its `scenes` module | the two tests drive the test-only emulator `handle_scene_wheel` / `handle_scene_pointer_move` of the Scenes standalone law |
| `🔬️wgpu-raster-frame-cost` | scene-ink | **scene-ink-tests** | names scene-tables (`Paint2dFlatLayer`) and scene-canvas2d |
| `🔬️wgpu-event-feed`, `🔬️wgpu-graph-timeline` | scene-feeds | **scene-ink-tests** | `collect_fixture_actions` lives in frame-actions (rank 60, above scene-feeds); graph-timeline also names `scene_list_hit` (scene-render) |
| `🔬️wgpu-table`, `🔬️wgpu-block-list`, `🔬️wgpu-virtual-file-system` | scene-tables, scene-tables, scene-text-vfs | **scene-render** (`T/🪞️scene-render/🧪️tests/🦀️.rs`) | name `passive_scene_pointer_*`, `cancel_scene_list_transfer*`, `scene_double_click_action` (scene-render) |
| `🔬️wgpu-ink-canvas`, `🔬️wgpu-diff-view`, `🔬️wgpu-text-editor`, `🔬️wgpu-gumball-dispatch`, `EL/🖼️IconRenderHost` 5 laws, `🔬️wgpu-icon-render`, `🔬️wgpu-surface-context-menu`, `🔬️wgpu-render-entry`, `EL/🗣️Interpreter/🧪️tests/🖼️wgpu-ui-image-loading` | unchanged | unchanged | closure-complete (the last one never used `super::*`) |

`scene-ink-tests` (the plan's companion, an empty skeleton until now) is the bin for the laws that need frame-actions and ui-engine; `engine-attach-tests` was at 0.92 of the byte gate and could not take 44 KB more, which is why only the two emulator tests moved there. rtb's other mounts (the EngineCanvas laws in engine-attach-tests and ui-engine-tests) are unchanged.

## 3. What changed

### 3.1 Explicit imports (task 1)

`use super::*;` is gone from **18 law files** (all of the above except the image-loading law, which already imported explicitly). Each file now starts with a generated block of `use` lines grouped by owning crate (`F/rw2t-tools/gen.py`, `apply_law.py`, `apply_all.py`): names of the law's old parent module become `super::name` (law stays) or `<crate>::name` (law moves); names the old Scenes root imported from outside become the same external paths (`ui_wgpu::wgpu::…`, `serde_json::{Value, json}`); the stale `crate::` heads are rewritten: `crate::scenes::X`, `crate::interpreter::X`, `crate::element_contracts::X` become imports of `X` from the owner (contracts, scene-state, host-base, frame-actions, ui-engine), `crate::canvas2d_gumball::paint(` becomes `semio_framework_os_renderer_wgpu_canvas2d_host::paint(`, `crate::app_now_ms` and `crate::collect_fixture_actions` are imported. 0 `crate::` heads remain in the 19 law files. The generator imports a name when the law mentions it as a token that the old module namespace, the export index or the crate roots' imports define; local bindings (`let`, closure parameters, `fn` parameters) and method names are not imported; the oracle is the check.

### 3.2 Fixtures moved to the crates that own what they read (rtb 2.1 rule)

Five test-only helpers of the Scenes standalone law that other laws also name became `#[cfg(any(test, feature = "testkit"))] pub fn` in their owners (`stepA.py`): `paint2d_navigator_fit_viewport`, `paint2d_navigator_overlay_rect` (scene-tables, `table-and-paint2d`, next to the doc comments that had lost them), `ink_items_at_point` (scene-ink, `ink-canvas-render`), `cursor_from_click`, `line_col_at` (scene-text-vfs, `text-editor`). The standalone law imports them (engine-attach-tests root import lists); `scene-text-vfs` got the `testkit` feature (`testkit = [contracts/testkit, scene-state/testkit]`) and engine-attach-tests enables it. `Paint2dCameraFields` (scene-tables) joined the testkit gate with its `Default` and `paint2d_default_one` and is `pub` (a `pub` field of a `pub` struct named it); the `Viewport` import for the two moved navigator fns is gated the same way.

`pick_target_hits_for_test` and `selection_domains_for_test` (engine-board `paint2d`) are `any(test, testkit)` now, engine-board has `testkit = []` and `semio-framework-ui-layout` moved from dev- to normal dependencies (the helper returns its `ContextMenuHit`); the surface context menu law imports them instead of the stale `engine_canvas::…` heads, and scene-render dev-depends on engine-board with `testkit`.

### 3.3 Per-thread scene cell for the companion bins

`SceneCell` (scene-state) was per-thread only under `cfg(test)` of scene-state itself; in a bin or in the tests of a sibling crate it is a process-wide mutex, so parallel laws would share `SCENE_STATE`. It now follows the testkit convention (`any(test, feature = "testkit")`, three spots, `stepC.py`). The 8 crates whose laws read scene state got `semio-framework-os-renderer-wgpu-scene-state = { …, features = ["testkit"] }` in `[dev-dependencies]` (`stepD1b.py`): canvas2d-host, scene-canvas2d, scene-ink, scene-feeds, scene-tables, scene-text-vfs, icon-host, scene-render. Side effect in the census (section 4.4): the oversized `semio-framework-ui` is also a normal dependency of those crates, and now shows as a dev-dependency row for them.

### 3.4 Privacy (task 3)

* **`SceneSurfaceState`**: 23 fields became `pub` (`viewport`, `canvas_framing_revision`, `drag`, `pointer_was_down`, `last_click_ms`, `last_click_target`, `hovered_control_id`, `paint_stroke_active`, `vfs_expanded_ids`, `map_marquee_points`, `map_marquee_active`, `map_last_hover_json`, `map_interaction_owner`, `map_hover_owner`, `touch_gesture`, `ink_camera`, `ink_overrides`, `ink_gesture_open`, `ink_marquee_points`, `ink_edit`, `last_pointer_pos`, `camera_dispatch_controller_id`, `canvas_click`); the five rows of `ri3`/`b1` (`ink_gesture_open`, `ink_overrides`, `ink_marquee_points`, `drag`, `pointer_was_down`) are among them. They are genuine API: scene-ink, scene-feeds, scene-render, scene-canvas2d and the tests of engine-attach read and write them in production code across the crate edges. The six fields no other crate names stay `pub(crate)` (`mount_owner`, `engine_token`, `scroll_offsets`, `canvas_camera_owner`, `selected_ids`, `vfs_selection_anchor`). I did not move any law to scene-state to avoid the widening: the laws exercise scene-ink, scene-render and scene-tables behaviour through that state, so the owner crate cannot host them.
* **Other cross-crate reads** found by the oracle (E0616, E0624, E0603) and widened with the planner `pos2widen.py plan|apply` on my own err files, restricted to WP2 files and the bins' hosts: 98 line edits in 12 files (`rw2t-widen-wp2.json`): the `GumballSlot` fields (canvas2d-host reads them), `SceneListTransferSession` / `SceneListTransferAuthority` fields, `CanvasStroke*` and `CanvasImage*` JSON fields (scene-canvas2d), the `SceneAccessibilityPresentation` members `stage` / `take` / `has_capacity_for`, `TextEditorDeliveryState` and `TextEditorTypingRun` fields (engine-text), `TextEditorRetainedActionBatch`, `EngineCanvasPacketReservation`, the board / table / vfs / event-feed layout records (scene-render), the `engine-surface-attach` progress fields, pending-raster owner fields and the `introspection` dump fields the `ui-engine-tests` law reads.
* **Items the moved laws name** (plan 9.2 rule 2: exactly the items they name, no `cfg(test) pub` shims): 38 private functions, types and consts of the Scenes tree became `pub` (`stepE`/`widen.py`, list `rw2t-widen-laws.json`), plus `table_editable_text_commit` (`pub(crate)` to `pub`).
* **Law moves instead of widening**: the two `*_for_test` helpers above (made testkit items), the five standalone helpers (3.2), and the two camera tests (section 2).

### 3.5 Mounts, manifests, registry

* Seven `//#region …Tests` blocks (the region markers and the mount) left the lib files of the Scenes tree; the new hosts mount the laws: `T/🧶️scene-ink-tests/🦀️.rs` (4 laws, `#![cfg(test)]`), `T/🪞️scene-render/🧪️tests/🦀️.rs` (3 laws), `T/🧷️engine-attach-tests/🦀️.rs` (`mod canvas2d_camera_tests;` in `scenes`). Module declarations are in rustfmt order. No law was copied.
* Manifests: scene-ink-tests gets 7 first-party edges (canvas2d-host, frame-actions[testkit], host-base, scene-feeds, scene-render, scene-tables[testkit], ui-engine), `testkit` features on contracts / scene-ink / scene-state, and the external lines its laws name (ui-action, ui-draw-contract, ui-scene-plane, ui-scene-views, ui-text, ui-theme, `ui_wgpu`, base64, image); scene-render dev-dependencies (ui-draw-contract, ui-text, `ui_contract`, engine-board[testkit]); scene-text-vfs `testkit`; engine-board `testkit`; the 8 dev edges of 3.3 (11 manifests in all). `F/rw2t-tools/mf.py` inserts lines in the file's own order.
* `Cargo.lock` (repo root): `locksync.py` (rtb `lock_sync`) rewrote the dependency lists of scene-ink-tests (+16 names) and scene-render (+3); the other entries were already equal.
* Taxonomy: the new law folder is `📷️wgpu-canvas2d-camera` (📷️ is a prefix used in the repo and unused among its siblings; it matches the `memberSlugPattern` of the `members-of-tests` kind, so no `🔣️taxonomy.json` edit). No crate, route, owner row or launch entry was added: the three bins already exist.
* Debug logs: none added.

## 4. Verification (every line names the command and its real result)

### 4.1 Oracle, absolute rows in the WP2 files (92 crates, lib and `--test`, canary reached in every run)

`cd F/pos-tools && POS_PREFIX=rw2t python3 pos.py run ../rw2t-tools/rw2t.json --cfgs lib,test --label <L> --no-base` (spec `rw2t.json` = the `ri3bins` family: the 76 `ri2` crates plus the 14 `shell-tests-*` bins, `frame-tests-1`, `fixtures`). Rows are the non-stub resolve and privacy diagnostics located in a WP2 crate or in `engine-attach-tests`, `scene-ink-tests`, `ui-engine-tests`; `F/rw2t-tools/table.py` classifies them (`oracle-artifact`: an `extern crate … as` alias head or a stub gap, see 6.1; `foreign`: 6.2; `carve`: everything else).

| run | tree | lib resolve carve / artifact / foreign | lib privacy | test resolve carve / artifact / foreign | test privacy |
|---|---|---|---|---|---|
| `b0` | before my first edit | 1 / 71 / 28 | **270** | **316** / 77 / 38 | **394** |
| `c1` | after the import rewrites and moves | 6 / 72 / 28 | 270 | 11 / 74 / 38 | 464 |
| `c2` | after widening round 1 | 0 / 72 / 28 | 10 | 0 / 74 / 38 | 22 |
| `c3` | after widening round 2 and the Viewport / helper fixes | **0** / 72 / 28 | **0** | **0** / 74 / 38 | **0** |

`c1` shows the usual order effect: once the laws resolve, the privacy errors of their bodies appear. Per crate (test configuration, carve rows only; the lib column is the same list without the laws):

| unit | test resolve before | after | test privacy before | after | lib privacy before | after |
|---|---|---|---|---|---|---|
| `engine-text` | 0 | 0 | 9 | 0 | 9 | 0 |
| `canvas2d-host` | 0 | 0 | 34 | 0 | 34 | 0 |
| `scene-canvas2d` | 185 | 0 | 42 | 0 | 25 | 0 |
| `scene-ink` | 64 | 0 | 73 | 0 | 64 | 0 |
| `scene-feeds` | 13 | 0 | 37 | 0 | 37 | 0 |
| `scene-tables` | 32 | 0 | 11 | 0 | 11 | 0 |
| `scene-text-vfs` | 10 | 0 | 3 | 0 | 3 | 0 |
| `icon-host` | 6 | 0 | 0 | 0 | 0 | 0 |
| `scene-render` | 6 | 0 | 91 | 0 | 87 | 0 |
| `engine-attach-tests` | 0 | 0 | 91 | 0 | 0 | 0 |
| `ui-engine-tests` | 0 | 0 | 3 | 0 | 0 | 0 |
| engine-registry, engine-board, engine-graph, engine-attach, scene-state, scene-ink-tests | 0 | 0 | 0 | 0 | 0 | 0 |
| **total** | **316** | **0** | **394** | **0** | **270** | **0** |

Controls: the oracle's own canary (E0616, E0624, E0308) is raised in `b0`, `b1`, `c1`, `c2`, `c3` and `f1`, so bodies were type-checked; the privacy rows disappear only after the field edits (`c1` to `c3`), which is the positive control. I ran no seeded mutation in this slice.

### 4.2 Oracle, official form (`NEW` against `1bae6115bc5`)

`cd F/pos-tools && POS_PREFIX=rw2t python3 pos.py run ../rw2t-tools/rw2t.json --cfgs lib,test --label f1` (with the baseline: `1bae6115bc5`, 90 of the 92 crates exist there; report `rw2t-rw2t-f1-report.txt`, result `rw2t-rw2t-f1-result.json`, 21:40 tree, started after the last source edit). Canary reached on both sides (E0308, E0616, E0624). Totals over all 92 crates: lib NEW 1,311 / GONE 633, test NEW 1,904 / GONE 898 (the rest of the NEW rows belong to the other slices' crates).

| NEW non-stub rows located in the 14 WP2 crates or in engine-attach-tests / scene-ink-tests / ui-engine-tests | lib | test |
|---|---|---|
| privacy (E0603, E0616, E0624) | **0** | **0** |
| resolve, attributable to the carve | **0** | **0** |
| resolve, oracle artifact (`extern crate` alias heads, stub gaps, 6.1) | 0 | 42 |
| resolve, foreign (infinite free functions, 6.2) | 0 | 38 |

(The foreign and artifact rows are present in the baseline run too in the lib configuration, which is why only the test configuration lists them as NEW: there the same names occur more often than in the baseline's single monolith unit.) The `ri2` / `ri3` figures this replaces were 5 lib privacy rows (`SceneSurfaceState`) and 414 test resolve rows.

### 4.3 Static gates

| check | command | result |
|---|---|---|
| test-name multiset | `python3 rw2t-tools/names.py snap` (before the first edit) and `compare` | 1,789 tests before and after; by name only-before 0, only-after 0 (`IDENTICAL BY NAME`); by (file, name) exactly the two camera tests changed file |
| duplicate mounts | `python3 rtb-tools/dupscan.py` | 97 packages, 755 files reached, **0 files reached by more than one package** |
| unmounted laws | `python3 rtb-tools/orphans.py` | 1 orphan (`NodeGraph/📡️scene/🧪️tests`), unmounted before; 3 files only the top crate reaches (the rtb kept ones) |
| paths | inline length scan of the 50 touched files (`touched.txt`) | longest touched path 195 B repo-relative (limit 256 incl. the 28 B prefix) |
| rustfmt parse | `rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --emit stdout` on the 39 touched `.rs` files (before the last tidy) | 0 parse errors |
| rustfmt style | `rw2t-tools/fmt_check.py` (`--style-edition 2024 --config-path rustfmt.toml`, temp copies) | new files clean (camera law); scene-ink-tests root, scene-render tests stub and the engine-attach-tests import lines of mine formatted; the remaining hunks are code that was unformatted before (law bodies, engine files) and rtb's mount order in the engine-attach-tests root, which its recipe stubs |
| census | `python3 rw2t-tools/census_run.py final` (`census.ts`, copy of `rtb-census.ts`, 3 labels) | 17 crates, 11,232 units, 0 error units, **0 test units over the gate** |
| manifests | `locksync.py` (reads the 11 changed manifests with the rtb dependency parser) | all 11 parse; lock lists synced |

### 4.4 Census of the WP2 test units after (native; files / bytes / active lines; ratio = worst of the 4000 / 32 / 262,144 gate)

| unit | files / bytes / lines | ratio |
|---|---|---|
| engine-registry | 6 / 103,262 / 2,305 | 0.576 |
| engine-text | 3 / 65,164 / 1,152 | 0.288 |
| engine-board | 5 / 93,534 / 1,785 | 0.446 |
| engine-graph | 3 / 69,826 / 1,322 | 0.331 |
| engine-attach | 4 / 61,217 / 1,160 | 0.290 |
| scene-state | 9 / 123,866 / 2,758 | 0.690 |
| canvas2d-host | 5 / 61,755 / 550 | 0.236 |
| scene-canvas2d | 5 / 82,598 / 1,634 | 0.408 |
| scene-ink | 8 / 143,236 / 2,828 | 0.707 |
| scene-feeds | 4 / 71,749 / 1,457 | 0.364 |
| scene-tables | 7 / 104,581 / 1,851 | 0.463 |
| scene-text-vfs | 5 / 98,148 / 1,816 | 0.454 |
| icon-host | 22 / 168,736 / 3,045 | 0.761 |
| scene-render | 16 / 199,088 / 3,035 | 0.759 |
| engine-attach-tests | 15 / 241,284 / 3,603 | 0.920 |
| scene-ink-tests | 12 / 126,172 / 1,903 | 0.481 |
| ui-engine-tests | 18 / 211,539 / 2,275 | 0.807 |

The census also reports **24 over-gate dev-dependency rows (8 crates x 3 labels)**, all `semio-framework-ui` (31 files, 1.1 MB, the peer UI carve's oversize) for the 8 crates of 3.3: it is also a normal dependency of each, and the rows exist because the dev edge to scene-state[testkit] now makes it reachable through a dev path (the same class as the `frame-job` / `host-winit` / `program-bridge` rows in `RESULT-RENDERER-TEST-BINS.md` 4). The baseline census of the same crates (`rw2t-census-base.log`) had 0 such rows. Removing the 8 edges removes the rows and the per-thread cell for those crates' own test units; I kept the cell.

## 5. Real-check commands (for the compile queue; not run, the ingress refuses compile admission)

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-renderer-wgpu-engine-registry -p semio-framework-os-renderer-wgpu-engine-text -p semio-framework-os-renderer-wgpu-engine-board -p semio-framework-os-renderer-wgpu-engine-graph -p semio-framework-os-renderer-wgpu-engine-attach -p semio-framework-os-renderer-wgpu-scene-state -p semio-framework-os-renderer-wgpu-canvas2d-host -p semio-framework-os-renderer-wgpu-scene-canvas2d -p semio-framework-os-renderer-wgpu-scene-ink -p semio-framework-os-renderer-wgpu-scene-feeds -p semio-framework-os-renderer-wgpu-scene-tables -p semio-framework-os-renderer-wgpu-scene-text-vfs -p semio-framework-os-renderer-wgpu-icon-host -p semio-framework-os-renderer-wgpu-scene-render --tests --keep-going --message-format=short
```
then the same with `-p semio-framework-os-renderer-wgpu-scene-ink-tests -p …-engine-attach-tests -p …-ui-engine-tests --tests`, once with `--lib` for the 14 crates, and once more with `--target wasm32-unknown-unknown` for the lib pass. Likeliest first faults: the foreign family of 6.2, then `unused import` warnings in the 18 rewritten laws (the generator imports a name when the law mentions it as a token), then the `testkit` feature set of `scene-ink-tests` (it enables `testkit` on contracts, scene-ink, scene-state, scene-tables, frame-actions) if a dependency of those lacks the forwarded feature.

## 6. What is not clean, and why it is not this slice's

### 6.1 Oracle artifacts (72 lib / 74 test rows, all in the baseline run too)
`unresolved import … element_contracts`, `cannot find element_contracts / host_base / asset_authority / canvas2d_paint in <crate>`: `extern crate X as Y;` heads at the library roots; `cannot find RestoreStep / Record in camera_storage`: `use semio_framework_os_renderer_camera as camera_storage;` of the scene-state root over an out-of-union crate; `cannot find module map_tiles`: `use semio_framework_surface::tiled_map::{tiles as map_tiles, …}` (the import exists at `engine-board/🗺️tiled-map` line 10 and in the `ui-engine-tests` law mod, the stub layer does not model the rename inside a brace group).

### 6.2 Foreign: the `infinite` API migration (28 lib / 38 test rows, E0425)
Commit `1bae6115bc5` (the same auto-commit that holds the WP2 carve) changed the renderer call sites from methods (`host.set_camera_silent(…)`, still methods one commit earlier) to the free functions of the split `infinite` sub-crates, without adding the `use` lines or the manifest edges: `semio-framework-os-infinite-board-normal-{gestures,host,focus}`, `-board-dag-{graph,host}`. Unresolved names, with file and line: engine-board `🎲️board2d`: `set_camera_silent` 66, `begin_pointer_commit` 418 / 447, `pointer_lane_is_direct` 770 / 800, `pointer_lane_in_flight` 843, `commit_wheel` 861; engine-graph `🕸️node-graph`: `fit_camera_to_content` 131, `connection_cursor_active_at_screen` 558 / 559, `screen_hit` 623, `screen_hit_json` 624, `hovered_node_id_ref` 678 / 685 / 841, `screen_geometry_census_json` 976 / 977, `hovered_node_id` 1076, `selection_union_bounds_screen_json` 1232; engine-attach `🧩️engine-surface-attach`: `set_hover_channel` 179, `set_dimmed` 268, `adopt_camera_or_fit` 294, `refit_camera_if_content_left_view` 307, `set_node_statuses` 333, `set_computing_progress` 347, `set_selection_ids_silent` 579, `set_camera_silent` 590, `set_transform_flags` 639; the laws `ui-engine-tests` `🔬️wgpu-board2d-engine` (8 uses of `set_camera_silent`, `set_selection_ids_silent`, `pointer_lane_*`, `begin_pointer_commit`) and `engine-attach-tests` `EngineCanvas/🧪️tests/🧊️wgpu-standalone` 402 / 406 (`defers_descriptor_sync_from_js`, defined in `…/🕹️gestures/🦀️.rs:164`). Owner: the infinite refactor; I did not guess the module paths of the sub-crates.

### 6.3 Outside the family
`semio-framework-os-renderer-wgpu-shell-view-paint` and `ui-engine` tuple-struct field rows, `frame-latency`, `presentation-raster`, `kernel-retained-state`, `host-base` `asset-decode-jobs` and the Shell `icon-export` fields appear in the planner's list but belong to other slices (WP1, WP3, WP4); I did not widen them.

## 7. Hand-offs and residual risks

1. The infinite call-site imports (6.2) block every real `cargo check` of engine-board, engine-graph, engine-attach and the two bins that mount the board laws; they are the first compile errors to expect.
2. `scene-ink-tests` now carries four laws and a long dependency list; its description in the manifest still says "ink canvas, raster frame cost and the standalone slice".
3. The generated import blocks list every name a law mentions as a token; a name used only as a method-resolution trait would be missed (the oracle reports E0599 for those; none was reported in the 18 files).
4. `SceneSurfaceState` now exposes 23 fields; the fields the engine-attach standalone law reads for assertions only (`ink_marquee_points`, `map_*`) could move to a `testkit` accessor if the API surface matters more than the test peek.
5. The census dev-dependency rows of 4.4.
6. `f1` (4.2) is the only run with the baseline made on the final tree; `b1` (`pos-rw2t-b1-report.txt`) ran while I started editing, so its tree is mixed.

## 8. Files

Created: `E/EL/🎞️Scenes/🧪️tests/📷️wgpu-canvas2d-camera/🦀️.rs`, `F/RESULT-RENDERER-WP2-TESTS.md`, `F/rw2t-tools/` (`wp2.py`, `idx.py`, `ns.py`, `plan.py`, `analyze.py`, `gen.py`, `apply_law.py`, `apply_all.py`, `needs.py`, `plan_deps.py`, `mf.py`, `ed.py`, `widen.py`, `names.py`, `rows.py`, `summ.py`, `table.py`, `newdiff.py` (abandoned: its parse diverged from the tool's), `census.ts`, `census_run.py`, `fmt_check.py`, `locksync.py`, `stepA.py`–`stepG.py`, `rw2t.json`, `touched.txt`).
Edited (50 repo files, list `F/rw2t-tools/touched.txt`): the 18 law files; the Scenes standalone law (5 helpers out); the Scenes lib files `table-and-paint2d`, `ink-canvas-render`, `text-editor`, `canvas2d`, `event-feed-and-timeline`, `board2d-and-vfs`, `block-list-and-diff`, `scene-state`, `pending-raster`; EngineCanvas `paint2d`, `text-editor`, `prepared-engine-canvas`, `engine-surface-registry`, `engine-surface-attach`; Interpreter `introspection`; `T/🎭️scene-state/🦀️.rs`, `T/🧶️scene-ink-tests/🦀️.rs`, `T/🪞️scene-render/🧪️tests/🦀️.rs`, `T/🧷️engine-attach-tests/🦀️.rs`; 11 `Cargo.toml` (scene-ink-tests, scene-render, scene-text-vfs, engine-board, engine-attach-tests, canvas2d-host, scene-canvas2d, scene-ink, scene-feeds, scene-tables, icon-host); repo-root `Cargo.lock`.
Deleted: none.
