# 🔗️ Result RENDERER-INTEGRATION-3: stale Shell-parent heads, the testkit field and the E0603 rows

Executor: Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. Slice `ri3`. `F` = this folder. `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`, `T` = `E/🎯️targets/🧊️wgpu`, `SH` = `E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu`, `STG` = `SH/📥️staging`. Tools: `F/ri3-tools/`. Logs and oracle output: `F/🗑️generated/ri3-*` and `F/🗑️generated/pos-ri2-ri3{a,b,c}-*`, `pos-ri3bins-ri3{d,e}-*`. Pre-edit copies of every file I touched: scratchpad `ri3/pre/` (wiped on process restart).

## 0. Verdict

**All three targets are done and re-measured with the privacy/type oracle; nothing is compiled by cargo** (launcher headroom gate, as for the whole wave; section 6 has the exact real-check commands for the compile queue).

1. **Stale Shell-parent heads: fixed.** All 28 occurrences of `time_travel::`, `local_folders::`, `icon_export::` and `tree_windows::` (23 code uses, 5 doc mentions or links) in 15 staging files are gone. Every name already had an explicit import in the owning crate root (the ri2 flip computed it from the tokens) or is defined in the consuming crate itself, so the fix is the bare name; no manifest edge was missing and no owner sits above its consumer (section 2.1). The two non-Shell names of the sweep were real as well: `winit_app::{HostUserEvent, WinitApp}` in `native-probe` was a **back edge** (host-winit ranks above native-probe) and is resolved by lifting `run_native` into `host-winit` (2.2); `runtime_publication_tests::…` in the `runtime-mailbox` lib was an upward reference to its test companion and is now a hook port (2.3). `host_base`/`element_contracts` are oracle artefacts (2.4).
2. **testkit field: fixed.** `HistoryState.host_route_folder` (and `HostRouteFolder`) are ungated in `shell-history`; the `shell-state` initialiser lost its gate. The feature forwarding of the four gated consumers was already consistent and is checked mechanically (section 3).
3. **12 E0603 rows: real, fixed.** Not the oracle's alias rewrite. Seven call sites in six test laws named a sibling test module's *private* `use` import (`panel_anchor_model_tests::host_test_shell()`); they call the name they already import from the fixtures crate (section 4).

Oracle result on the final tree (`ri2` family, 76 crates, lib and `--test`, canary reached in every run: E0308, E0616 and E0624 all raised): **0 of the sweep's carve rows remain** (section 5). The rows still reported belong to peers in flight (Scenes/EngineCanvas carve and test relocation) and to documented oracle artefacts (section 7); none is in a Shell crate.

## 1. Found state

* `RESULT-PRIVACY-ORACLE-SWEEP.md` section 5 and the `ri2` family were the input. The sweep's `f3` report hid the sibling-head rows (stub flag read rustc's help text); `pos.py` was edited by a peer while I worked (19:01 to 19:13), so I read the rows from the result JSON with my own printer (`ri3-tools/rows.py`, stub-flagged rows included) instead of the report text.
* Baseline taken before my first edit: oracle run `ri3b` (label), tree of 19:00: lib 46 privacy/resolve rows (14 of them the sibling heads below, hidden by the old stub flag), `F/🗑️generated/ri3-before-lib-rows.txt`.
* The 12 E0603 rows of `f3` came from test files mounted by the top crate hub. By the time I started, those laws were mounted by the test bins (`shell-tests-docs-1/2/3`, `-windows-2`), which are **not** in the `ri2` family, so a plain re-run could no longer see them. I built the family `ri3-tools/ri3bins.json` (the 76 crates plus the 14 `shell-tests-*` bins, `frame-tests-1` and `fixtures`) to keep them under the oracle (section 4).
* Peer activity seen: `rtb` (RENDERER-TEST-BINS) had written only its baseline section; at 19:00 someone fixed the same defect in `🚪️wgpu-host-door-remainder` (one line); `pos-tools/pos.py`/`postrait.py` were rewritten while I worked; the Scenes/EngineCanvas roots and the bins were being edited (section 7). Every file was re-read immediately before its edit (`ri3-tools/edit.py` asserts every anchor count and that the file did not change between read and write).

## 2. Target 1: stale heads

### 2.1 Shell staging heads (owner table)

Owner = crate that defines the name (`ri3-tools/owners.py` indexes the definitions of every crate through its `#[path]` mounts). Rank order of 7.7 (dependencies point left to right): history < prefs < directory < sync < docs < doc-open < windows < dock < input < commands < state < session < window-actions < actions < pointer.

| stale head | names | consumer crate | owner crate | edge | fix |
| --- | --- | --- | --- | --- | --- |
| `time_travel::` | `TimeTravelFocus` | shell-history | shell-history (own `⏪️time-travel`) | own | bare name |
| `time_travel::` | `ChromeBandLine`, `chrome_band_layout`, `history_refusal_of_fault`, `history_refusal_is_silent`, `history_refusal_discards_input_draft` (+1 doc) | shell-prefs | shell-prefs (own) | own | bare name |
| `time_travel::` | `history_refusal_notice` | shell-actions | shell-prefs | actions to prefs exists, name already in the root import | bare name |
| `time_travel::` | `history_refusal_notice`, `history_refusal_is_silent` | shell-pointer | shell-prefs | exists, already imported | bare name |
| `time_travel::` | `time_travel_peer_presence` (doc only) | shell-view-panels | shell-directory | n/a | bare backticked name |
| `local_folders::` | `HostRouteFolder` | shell-history | shell-history (own) | own | bare name (+ ungated, section 3) |
| `local_folders::` | `LOCAL_FOLDER_DIRECT_REATTACH` | shell-state | shell-state (own `📎️local-folders`) | own | bare name |
| `local_folders::` | `local_folder_text`, `LocalFolderText`, `FOLDER_RECONNECT_ACTION`, `FOLDER_FORGET_ACTION` | shell-actions | shell-prefs | exists, already imported by the root | bare names |
| `local_folders::` | `SYNC_DOCUMENT_UNIDENTIFIED_CODE`, `is_host_route_uri` | shell-actions | shell-actions (own) | own | bare names |
| `icon_export::` | `IconExportBatch` | shell-doc-open | shell-doc-open (own) | own | bare name |
| `icon_export::` | `IconExportBatch::running` | shell-pointer | shell-doc-open | exists, already imported | bare name |
| `icon_export::` | `CONTROL_ID` | shell-pointer | shell-pointer (own `📤️icon-export`) | own | bare name |
| `tree_windows::` | `TreeWindowScheduler` | shell-dock | shell-dock (own) | own | bare name |
| `tree_windows::` | `TreeWindowScheduler` | shell-state | shell-dock | exists, already imported | bare name |
| `tree_windows::` | `TreeWindowScheduler::{window_of, view_state_fields}` (2 doc links) | shell-session | shell-dock | exists | link written with the crate path `semio_framework_os_renderer_wgpu_shell_dock::TreeWindowScheduler::…` |

No owner is above its consumer, so no back edge and no port was needed in the Shell crates. The `match` arms `FOLDER_RECONNECT_ACTION =>` / `FOLDER_FORGET_ACTION =>` are constant patterns; a missing import would have turned them silently into catch-all bindings, so I verified the explicit import in the `shell-actions` root (line 20) before the edit. Edits: 28 occurrences in 15 staging files, applied by `ri3-tools/apply1.py` (regex `(?<![\w:$.])head::` with an asserted count per file); a scan afterwards (`ri3-tools/stale_heads.py`, any lower-case path head that no module, import, macro or dependency of its crate names) finds no remaining `time_travel`, `local_folders`, `icon_export` or `tree_windows` head anywhere in the 92 crates.

### 2.2 `winit_app` in native-probe (back edge, lifted)

`run_native` (13 lines, in `T/🧊️renderer/🔬️native-socket-probe`, crate `native-probe`, frozen rank 66) names `winit_app::{HostUserEvent, WinitApp}` of `host-winit` (rank 67). The only caller is the top crate's `run_native_entrypoint` (`use crate::run_native`), which reaches both crates through the hub globs. Following plan 7.2 rule 5 (a method may be lifted to a higher crate together with its callers) I moved the function to `T/🪟️winit-app/🦀️.rs` (module `winit_app`, below `pub use native::{HostUserEvent, WinitApp}`), fully qualified, and re-exported it at the `host-winit` root (`#[cfg(not(target_arch = "wasm32"))] pub use winit_app::run_native;`), so `crate::run_native` still resolves through `pub(crate) use host_winit::*` and the TS source-text test (`run_native(&plugin_filter, modules_root, services)`) still matches. `native-probe` lost the three imports only `run_native` used (`native_caller`, `native_temporal`, `EventLoop`) and four manifest lines that became unused (`host-plumbing`, `native-platform`, `winit` twice, with the Linux-only `winit` table). `host-winit` already declared everything the function needs (`host-base`, `host-plumbing`, `native-platform`, `winit`, `semio-framework-job`, `semio-framework-os-kernel`). Script: `ri3-tools/apply3.py` plus a manifest edit (pre-edit copy in the scratchpad).

### 2.3 `runtime_publication_tests` in the runtime-mailbox lib (upward reference, now a port)

`enqueue_runtime_completion` (lib `T/🧊️renderer/🎨️presentation-raster`, `#[cfg(all(any(test, feature = "testkit"), not(wasm32)))]`) called `runtime_publication_tests::pause_after_completion_publication()`, a module of the test companion `runtime-mailbox-tests` (which depends on the lib): an upward name that does not exist inside the lib. Port: the lib owns a per-thread hook slot and `pub fn set_after_completion_publication_hook(Option<Box<dyn Fn()>>)` (same cfg), runs the hook where it called the module, and `enqueue_runtime_completion` is `pub` so the companion law can drive it. The law (`🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/📥️enqueue/🧪️tests/📥️enqueue`) imports the two names from the runtime-mailbox crate, keeps its own interlock, installs `pause_after_completion_publication` as the hook around the call and clears it. The companion manifest enables `testkit` on its runtime-mailbox dependency (one line). Script: `ri3-tools/apply4.py`.

### 2.4 Not defects (oracle artefacts, verified by reading the code)

* `host_base` at `🪟️winit-app:688`: the path is `..semio_framework_os_renderer_wgpu_host_base::host_appearance()` (struct-update syntax); the tool's head regex refuses a head preceded by `.`. The crate is a declared dependency.
* `element_contracts` in `ui-engine-dump`/`ui-engine-focus` and every other element crate: `extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;` at the crate root makes `crate::element_contracts::…` valid Rust; the union tool neutralises `extern crate … as` aliases (sweep section 5). Same for `pack`/`store` in `native-probe/🪶️sqlite`, `serde_json::json!` and `NativeJsonPages`.

## 3. Target 2: the testkit-gated field

Scan `ri3-tools/testkit_fields.py` (every `#[cfg(…testkit…)]` that precedes a field declaration or initialiser, all crates): the only field defined in one crate and initialised in another is `host_route_folder` (shell-history to shell-state).

* The manifests already forward: `shell-state`, `shell-sync`, `shell-session` and `shell-window-actions` (the four crates whose code is gated on `testkit` for the host route) all list `shell-history/testkit` (`ri3-tools/testkit_forward.py`, closure check `ri3-tools/testkit_forward.py::closure_check`: no crate whose `testkit` closure reaches `shell-history/testkit` has `shell-state` in its graph without reaching `shell-state/testkit`; dev-dependency rows checked with a grep over all 77 manifests).
* The remaining hazards were `cfg(test)` (shell-state's own unit tests build shell-history *without* `test` or `testkit`) and a workspace build where something else enables only `shell-history/testkit`. Forwarding cannot remove those, so the gate is removed from the data: `HostRouteFolder` (five `String`/`u64` fields) and `HistoryState.host_route_folder: Option<HostRouteFolder>` are unconditional (`STG/⏪️history/🧬️component`, `STG/⏪️history/📎️local-folders`), and the initialiser in `STG/🐙️state/🧩️session-and-topology` lost its `#[cfg]`. The consumers' gated *code* keeps its gate: it is browser-only behaviour, and its own `testkit` forwards the owner's. Script: `ri3-tools/apply2.py`.
* The other forwarding gaps the manifest scan lists (47 consumer-to-owner pairs without `testkit` forwarding, for example `shell-prefs -> program-bridge`, `ui-engine-focus -> scene-state`) are items for a stricter real `test` build; the brief's criterion (the code, not a failing run) found no cross-crate use that needs them. List: `F/🗑️generated/ri3-testkit-forward.json`.

## 4. Target 3: the 12 E0603 rows

Triage: **real**, not the `use <alias>;` rewrite. In the oracle note of each row the "unresolved item import `X` is defined here" points at a private `use semio_framework_os_renderer_wgpu_fixtures::shell::{…}` line of a sibling test module, and the consumer reads it as `sibling_tests::X(...)`. A `use` declaration is private to its module and its descendants; a sibling may not path through it, in the oracle or in rustc. In the monolith the helper was a `fn` of that module; the fixtures extraction replaced it by an import.

Rows and fix (every consumer already imports the name from `semio_framework_os_renderer_wgpu_fixtures::shell` at the top of its file, so the call site drops the qualifier):

| law (`SH/../🧪️tests/…`) | stale call | fixed to |
| --- | --- | --- |
| `🧪️wgpu-time-travel` (3) | `command_registry_tests::test_app`, `panel_anchor_model_tests::host_test_shell` ×2 | `test_app`, `host_test_shell` |
| `🧪️wgpu-local-folders` (1) | `panel_anchor_model_tests::host_test_shell` | `host_test_shell` |
| `🛟️panel-window-reservation` (1) | `window_pane_chrome_tests::split_pane_shell` | `split_pane_shell` |
| `👕️board-presence` (4) | `shell_input_tests::{tree_pointer_record, paint_component_pointer_documents, pointer_interaction, mouse_pointer}` | bare names |
| `🔗️hub-projection-workspace` (1) | `shell_input_tests::paint_component_pointer_documents` | bare name |
| `🚪️wgpu-host-door-remainder` (2: `tour_shell`, `tour_introduction`) | fixed by a peer at 19:00 before I reached it | (verified) |

A grep of the whole Shell test tree for `<x>_tests::<fixture name>` finds none left. After the laws moved into the bins the same references would have been E0433 ("cannot find module `command_registry_tests`") in the bin crates, so the fix matters beyond the hub.

## 5. Verification

All oracle runs: `python3 F/pos-tools/pos.py run <family.json> --cfgs lib,test --label <L> [--no-base]` (rustc `+nightly-2026-07-20 --emit=metadata`, single-crate union with the stub layer, baseline `1bae6115bc5`). Canary (E0308, E0616, E0624 raised by the probe module) was reached in **every** run, so bodies were type-checked.

| run | tree | family | lib NEW (non-stub) privacy / resolve | test NEW (non-stub) privacy / resolve |
| --- | --- | --- | --- | --- |
| `ri3a` (19:00, absolute) | before | `ri2` | absolute mode (all non-stub rows) | absolute |
| `ri3b` (19:00) | before my edits | `ri2` (with base) | 5 / 1 (+14 sibling-head rows hidden by the stub flag, read from the JSON) | 0 / 414 |
| `ri3c` | final | `ri2` (with base) | **5 / 1, all foreign (section 7), sibling-head rows 0** | **0 / 414, identical set, none in a Shell crate** |
| `ri3d` | final | `ri3bins` (92 crates, absolute) | n/a | 0 E0603; 0 rows that name a `*_tests` module; the 6 laws of section 4 show only stub-typing noise (`&mut ()`, `usize` for an error-typed stub) |
| `ri3e` | final + sandbox-only mutation | `ri3bins`, `--mutate` | n/a | seeded control: reintroducing `command_registry_tests::test_app` at `🧪️wgpu-time-travel:72` in the sandbox gives `E0433 cannot find module or crate command_registry_tests`, non-stub; absent in `ri3d` |

Row accounting of the lib configuration, `ri3b` to `ri3c` (`F/🗑️generated/ri3-before-lib-rows.txt`, `ri3-after-lib-rows.txt`): the 15 rows naming `time_travel`, `local_folders`, `icon_export`, `tree_windows`, `winit_app` and `runtime_publication_tests` are gone (46 listed rows become 31; the 31 are the artefacts and foreign rows of section 7). Totals: lib errors 21,031 to 21,011 (NEW against the baseline 614 to 593), test 28,755 to 29,154 (the test total moves with the peers' relocation edits, not with mine: 1,304 to 1,346 NEW, all in Scene/Engine test laws).

Other gates:
* rustfmt parse (`rustfmt +nightly-2026-07-20 --edition 2021 --config skip_children=true --emit stdout`) of the 26 changed `.rs` files: 0 errors. The two changed manifests parse (`Bun.TOML.parse`).
* Paths: longest changed path 224 bytes repo-relative (limit 240; the absolute path adds 28, as for every existing file of this tree).
* Duplicate mounts: `ri2-tools/dupmounts.py` and a second scan over `#[path]` mounts (`ri3-tools/owners.py`): **0** files mounted by more than one compilation unit; none of my edits adds a mount.
* Losslessness: every edit is an anchored replacement with an asserted count (`edit.py`); `git diff --stat` per file is proportional (1 to 5 lines for the heads and the E0603 call sites; 16 lines for the hook; the lift is 13 added lines in `winit-app` against 12 removed from the probe).

## 6. Real-check commands (for the compile queue; not run)

```
cd /Users/ueli/Documents/semio && SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' perl -e 'alarm 3500; exec @ARGV' -- bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-renderer-wgpu-shell-history -p semio-framework-os-renderer-wgpu-shell-prefs -p semio-framework-os-renderer-wgpu-shell-state -p semio-framework-os-renderer-wgpu-shell-dock -p semio-framework-os-renderer-wgpu-shell-doc-open -p semio-framework-os-renderer-wgpu-shell-actions -p semio-framework-os-renderer-wgpu-shell-pointer -p semio-framework-os-renderer-wgpu-shell-session -p semio-framework-os-renderer-wgpu-runtime-mailbox -p semio-framework-os-renderer-wgpu-native-probe -p semio-framework-os-renderer-wgpu-host-winit -p semio-framework-os-renderer-wgpu --lib --keep-going --message-format=short --manifest-path 🧰️framework/Cargo.toml
```
then the same with `--tests` and the packages `semio-framework-os-renderer-wgpu-runtime-mailbox-tests`, `-shell-tests-docs-1`, `-shell-tests-docs-2`, `-shell-tests-docs-3`, `-shell-tests-windows-2`, and a `--target wasm32-unknown-unknown` lib pass of `shell-history` and `host-winit` (the ungated field and the cfg-gated `run_native`). The `native-probe` manifest lost four dependency lines, so the framework `Cargo.lock` needs `update --workspace --offline --manifest-path 🧰️framework/Cargo.toml` first (the launcher reports `cannot update the lock file` otherwise).

## 7. Not fixed, foreign errors and hand-offs

* **Scenes/EngineCanvas carve in flight (peer).** Lib: `E0616` ×5 `SceneSurfaceState.{ink_gesture_open, ink_overrides, ink_marquee_points, drag, pointer_was_down}` are `pub(crate)` in `scene-state` and read by `scene-ink`/`scene-render`; `E0425 Paint2dCameraFields` in `scene-tables/🧮️table-and-paint2d`. Test: 414 resolve rows, 0 in a Shell crate, the same set before and after my edits: the Scenes laws mounted by `scene-canvas2d` (187), `scene-ink` (64), `scene-tables` (32), `scene-state` (28), `ui-engine-tests` (22), `engine-attach-tests` (14) and others still name monolith heads (`scenes::`, `interpreter::`, `canvas2d_gumball`, `engine_surface_attach_tests`, `host_base`, `media_slots`, `asset_authority`). These appeared between the sweep (18:49) and 19:00 when the Scenes roots changed; I did not touch them. Hand-off to the Scenes/test-relocation owner (`rtb` or the scene slice): widen the five fields, then re-point the laws with the recipe of section 2.1.
* **`cfg(test)` items used across crates (hand-off to `rtb`).** `ri3-tools/cfgtest_cross.py` lists `pub` items gated by a bare `#[cfg(test)]` that another crate's code names: `selection_domains_for_test`, `pick_target_hits_for_test` (engine-board, used by the scene-render law `🔬️wgpu-surface-context-menu`, which also still writes `engine_canvas::…`), `install_fixture_*` (program-bridge, used by `shell-tests-*`), `paint2d_navigator_*`, `cursor_from_click`, `line_col_at`, `ink_items_at_point`, `handle_scene_*` (defined in `engine-attach-tests`, used by Scenes laws: companion crates used as owners, a back edge in the making), `publish_reserved`/`try_reserve_fresh_packet` (engine-registry), `batch_is_empty`, `begin_shutdown`, `shutdown_step` (kernel). `cfg(test)` is invisible to every other crate; the oracle cannot see it (it turns `test` on for all crates). Each needs `any(test, feature = "testkit")` plus the consumer's `testkit` forwarding, decided together with the test relocation.
* **`runtime-mailbox-tests` is not finished as a companion**: besides my hook, its async-boundary law uses `testkit`-gated lib items (`worker_barriers`, `native_asset_page_barrier`) and its law root does not import everything `🔬️wgpu-renderer-async-boundary` needs (`RuntimeHostWaker`, `E0425` in the test run). I only added `features = ["testkit"]` to its runtime-mailbox dependency.
* **Coordination with `rtb`.** `F/RESULT-RENDERER-TEST-BINS.md` had only its baseline section while I worked. I edited no bin root, no bin manifest and no `test-support` file. I edited call sites in five Shell law files (they are mounted by bins), one UI-host law file and one companion manifest line (list in section 8); `rtb` should re-read those before moving laws. Concurrent peer edits seen during my work: `🧪️wgpu-time-travel` (19:00, before my edit; my anchors still matched), `🚪️wgpu-host-door-remainder`.
* **Tool artefacts that stay in the lists** (not defects): `pack`, `NativeJsonPages`, `serde_json::json`, `ToolRunTraceVisibility` in `tool_run_trace` (engine-attach, in the baseline), `element_contracts`/`host_base` heads (2.4), `E0407` on stub traits, `E0532` unit-variant rows of the stubbed `UiNode`/`Value`.
* **Pre-existing foreign row** (from the sweep, untouched): `S_SPACE_INDEX_DOCUMENT_SCHEMA` in `shell-actions/🎬️shell-actions` (E0425, needs its author).
* **Not covered**: real type-check of any crate (method resolution against the real crates, borrowck, `testkit` builds), `--target wasm32` oracle runs (no change of mine is wasm-only except the `cfg(not(wasm32))` lift), the 47 optional `testkit` forwards of section 3.

## 8. Files

Repo, edited (28 per `F/🗑️generated/ri3-touched.txt`, of which 26 are `.rs`):
* Staging (`STG/…/🦀️.rs`): `⏪️history/🧬️component`, `⏪️history/📎️local-folders`, `⚓️dock/🧬️component`, `📂️doc-open/🧬️component`, `🎛️prefs/📎️local-folders`, `🎛️prefs/🗨️chrome-overlays-and-tour`, `🐙️state/🧩️session-and-topology`, `🔀️actions/🗨️chrome-overlays-and-tour`, `🔀️actions/🎬️shell-actions`, `🖐️pointer/🎬️shell-actions`, `🖐️pointer/⌨️keyboard-and-palette`, `🖐️pointer/🖱️pointer-input`, `🪀️view-panels/🎬️shell-actions`, `🚶️session/⚙️settings-and-dock-persistence`, `🚶️session/🧩️session-and-topology`.
* Probe lift: `T/🧊️renderer/🔬️native-socket-probe`, `T/🔬️native-probe/🦀️.rs` and its `Cargo.toml`, `T/🪟️winit-app`, `T/🎪️host-winit/🦀️.rs`.
* Hook port: `T/🧊️renderer/🎨️presentation-raster`, the UI-host law `…/📥️enqueue/🧪️tests/📥️enqueue/🦀️.rs`, `T/🪤️runtime-mailbox-tests/📦️packages/🦀️rust/Cargo.toml`.
* E0603 call sites: `SH/../🧪️tests/{🧪️wgpu-time-travel, 🧪️wgpu-local-folders, 🛟️panel-window-reservation, 👕️board-presence, 🔗️hub-projection-workspace}/🦀️.rs`.

Tools (`F/ri3-tools/`): `owners.py` (crate to files, definition index), `stale_heads.py`, `rows.py`, `edit.py`, `apply1.py` to `apply4.py`, `testkit_fields.py`, `testkit_forward.py`, `testkit_users.py`, `cfgtest_cross.py`, `common_ri3.py`, `ri3bins.json`. Logs: `F/🗑️generated/ri3-*` (nothing deleted).
