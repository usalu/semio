# 🧱️ Result RENDERER-WP1: foundation, kernel and small elements of the wgpu renderer carved into producer crates

Executor: Sonnet 5.5, foreground, no sub-agents, no git-modifying command, no ticket tool. Plan: `PLAN-RENDERER-WGPU-CRATES.md` (6.2 `contracts` to `mailbox-core`, 5.2, 5.3, 10.1b, 10.4, Appendices F, G, H). Tools: `rwp1-tools/` (see section 9). Logs and evidence: `🗑️generated/rwp1-*`. `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`, `T` = `E/🎯️targets/🧊️wgpu`, `R` = `T/🧊️renderer`.

## 0. Verdict

**Sixteen of the eighteen WP1 crates are carved and wired; two are blocked on a lift that belongs to WP2.** `contracts`, `host-base`, `native-platform` (accessibility half), `media-slots`, `canvas2d-paint`, `doors`, `kernel-protocol`, `kernel-retained`, `kernel-pool`, `kernel-runtime`, `hub-contracts`, `hub-workspace`, `agent-elements`, `dock`, `program-bridge` and `mailbox-core` own their sources; the renderer hub `🌱️wp1-foundation` replaces their mounts by `extern crate … as …;` or `pub use <crate>::{…};`. `asset-authority` and `asset-decode` are analysed but not carved because they name `AdmittedSurfaceToken`, which only exists inside the scenes tree (the lift into `contracts` that the plan assigns to WP2 has not been delivered, section 4). `native-platform` holds `native_accessibility` only; `native_temporal` joins when WP2 lifts `NativeHostTemporalFormatter`.

Every carved crate is inside the internal budget (3,500 lines, 28 files, 230,000 bytes) for the lib **and** the test closure (section 3); the renderer top crate's lib closure dropped from 102,014 to 73,158 active lines with my carves alone (measured when the last one landed; the other packages' carves and staging moved it further and in both directions afterwards, section 3).

The evidence is static (the launcher refuses before any compile, section 6.1): the patched full-resolution union differential of the whole monolith plus the carved crates against the pre-carve tree, per crate resolution probes in all four configurations, census, definition scan, losslessness, rustfmt, path and TOML checks, and the renderer TS readers. **No `cargo check`, no `cargo test` and no executed unit test exists for any of these crates.**

## 1. Decisions that differ from the hand-off contract

1. **Module files stay where they are; every crate root mounts them with `#[path]`.** 10.4.4 step 1 asks to move source files into the crate folder. I did not, for the reason `RESULT-CARVE-PRINT-SERVER.md` and WP2 (`🎭️scene-state/🦀️.rs`) gave: element TS readers, the taxonomy, the os-infinite source-text test and the nested-cargo projection fixture all name the element paths, and the module bodies stay byte-identical (history, error locations). A crate root is therefore `docstring + imports + #[path] mounts + pub use <module>::*; + test stub`; the monolith keeps resolving `crate::<old module>::…` because the hub line is `extern crate <crate> as <module>;` (single-module crates, the crate root re-exports the module's items with a glob) or `pub use <crate>::{modules};` (crates that own several modules).
2. **`use super::*` is replaced by explicit imports in the crate root** (the children keep their `use super::*;`, which now sees the crate root). The import lists are computed from the old root's 123-name block, the old kernel parent's block and the names the earlier carved crates provide (`rwp1-tools/needs.py`, `provide.py`, `aliases.py`), then pruned by identifier use. Extern-prelude aliases of the old root (`dsl`, `protocol`, `store`, `store_sync`) are re-declared in the crate roots that name them.
3. **`pub(crate)`/`pub(super)` became `pub`** in the carved files, because every file now has a crate boundary between it and its former siblings. The few members that were private (not crate-visible) and are named across a crate boundary were widened one by one, driven by the oracle and the definition scan: `RendererSequenceReservation::generation`, the request queue methods, the `ResponseState` fields, the column-0 items of the three lifted pool and I/O files, … Widening whole files mechanically leaves some items wider than needed (section 8, risk 4).
4. **One `📥️lifted` pre-copy.** The plan wants the source owner to create `📥️lifted/<slug>/🦀️.rs`; WP3 delivered four, WP4 and WP2 none yet. To unblock `doors` I created `🏛️host-base/📥️lifted/🎨️theme-file-import-and-accessibility/🦀️.rs` as a **verbatim copy** of the four wasm trace/door items of the shell (`HOST_IO_REQUEST_TRACE`, `HostIoRequestTraceGuard`, `host_io_request_trace_label`, `host_io_call`) at exactly the path WP4 will write, so its cut overwrites the file with identical text. Until the shell removes its own copy there are two `HOST_IO_REQUEST_TRACE` statics (diagnostic label only, wasm only; `rwp1-tools/dupstatics.py` lists it).
5. **Tests that name the monolith moved to test hosts in the hub**: a `#[cfg(test)] #[path = "🧪️test-hosts/<slug>/🦀️.rs"] mod <x>_host;` in the hub, whose file is `pub(crate) use crate::<module>::*;` plus `#[path] mod tests;`, keeps the law's `use super::*;` and module path while the law's file stays the property of its element (region `HostBaseTestHosts` of the hub). One law of `media-slots` (the nonempty roster law) needs `render_snapshot`/`browser_original_output` of WP3 and is gated by the empty cargo feature `host-plumbing-law` until it moves into `host-plumbing` (request to WP3).

6. **Appendix G lines**: seven units use the `extern crate <crate> as <module>;` line of the table (`dock`, `media_slots`, `element_contracts`, `program_bridge`, `canvas2d_paint`, `runtime_mailbox_core`, `kernel_runtime`); the units whose crate owns a real module of that name (`hub_sign_in`, `space_browser`, `hub_connection`, `space_administration`, `canvas_presence`, the five agent modules, `socket_door`, `directory_door`, `parallel_runtime`, `scale_bench`, `native_accessibility`, `native_runtime_modules`) use `pub use <crate>::<module>;` with the visibility the old `mod` line had (`pub`, `pub(crate)` or private), and the root re-exports `frame_latency`, `mesh_assets`, `runtime_assets`, `plugin_catalog` and `action_args_json` of `host-base`. `asset_authority`, `glb_decode`, `asset_decode_jobs`, `native_temporal` stay mounted.

## 2. What was carved

Crate graph (rank order of Appendix F; every edge points down; `verify_static.ts` reports no rank, cycle, member or owner failure for them (one frozen-table difference, section 5); `depcheck.py`: every first-party crate named in the code is declared):

```
contracts   host-base   native-platform   media-slots   canvas2d-paint   mailbox-core   hub-contracts      (no first-party deps)
doors -> host-base                       dock -> contracts                  hub-workspace -> hub-contracts (+ dev: doors)
kernel-protocol -> contracts, host-base
kernel-retained -> kernel-protocol
kernel-pool     -> contracts, host-base, kernel-protocol, kernel-retained
kernel-runtime  -> contracts, host-base, kernel-protocol, kernel-retained, kernel-pool      (facade + scale-bench)
program-bridge  -> contracts, host-base, kernel-protocol, kernel-pool
agent-elements  -> host-base, doors
```

Longest chain: `kernel-runtime -> kernel-pool -> kernel-retained -> kernel-protocol -> host-base` (5 crates).

| crate | what it owns | source of the code |
|---|---|---|
| `mailbox-core` | bounded completion queue and checkout ledger (the wasip2 build) | `📮️runtime-mailbox-core` |
| `canvas2d-paint` | path geometry | `📐️Canvas2dHost/🎨️paint` |
| `hub-contracts` | `hub_sign_in`, `space_browser` | `🔐️HubSignIn`, `🏘️SpaceBrowser` |
| `hub-workspace` | `hub_connection`, `space_administration`, `canvas_presence` | three elements |
| `media-slots` | media slot descriptors, registry, props/slot/collect cursors | `🎬️media-slots` |
| `host-base` | the **dissolved renderer root** (host appearance/introduction/platform/locale, boot descriptor, logging, `app_now_ms`, `spawn_app_task`, `drive_renderer_worker_task`, `actor_budget_from_turn_budget`), `frame_latency`, `mesh_assets`, `runtime_assets`, `plugin_catalog`, the `action_args_json!` macro, **renderer I/O** (worker pool, slots, pump; 356 lines out of the asset decode jobs), `KernelPoolFuture` (48 lines out of the pool), the lifts `host_io`, `boot_subphase`, `host_waker` | `R/🦀️.rs` lines 107 to 737 (the 7 wasm_bindgen wrappers stay in the root, region markers repeated), registry, catalog, assets, latency, mesh |
| `contracts` | `element_contracts` (worker cell, scene pointer target, raster refusal, widget context), `ProgramFault` (41 lines out of the bridge), `framework_widget_context`, the retained UI engine cell | element-contracts + three lifts |
| `doors` | `socket_door`, `directory_door` | two files |
| `kernel-protocol` | kernel core (sequence authority, job-progress presentation, requests, close submissions), the replay vocabulary (1,157 lines out of the replay client), the request queue (247 lines out of the pool), `read_native_component`, `parallel_runtime` | five sources |
| `kernel-retained` | retained surfaces, pending rejections, replay routes, command document retirement | retained state minus `KernelPoolState` |
| `kernel-pool` | pool state (the 54-line struct moved here, with its 1,900-line impl), runner, kernel client and future | pool + client part of the replay client |
| `kernel-runtime` | facade (explicit `pub use` of the ten names consumers use) + `scale_bench` + the semantic-document law | scale bench, the old `kernel_runtime` root is retired |
| `program-bridge` | program bridge, native runtime module readers | two files |
| `agent-elements` | agent bridge, presence, approvals, chat panel, bridge door | five files |
| `dock` | dock tree, stack chrome, layouts | `🛰️Dock` |
| `native-platform` | `native_accessibility` | one file |

Item moves of plan 5.2 performed: `ProgramFault` to `contracts`; renderer I/O (356) and `KernelPoolFuture` (48) to `host-base`; replay vocabulary (1,157; the plan said 1,089), request queue (247) and `read_native_component` (31) to `kernel-protocol`; `KernelPoolState` (54) to `kernel-pool`. The inversion I1 (job-progress acknowledgement callback, `JobProgressAcknowledge` in the kernel core) landed in the tree during this session through an edit of someone else in the kernel files; the kernel core therefore no longer names `KernelClient` and `kernel-protocol` needs nothing from `kernel-pool`. The 22-line prefix validators and the 123-line `IconRenderHost/📤️export/📥️asset` stay with the blocked `asset-authority`.

## 3. Census (canonical counter, host cfg and `wasm32-unknown-unknown`, `debug_assertions`; limits 4000 / 32 / 262,144, target 3500 / 28 / 230,000)

Lib and test closures, native (`rwp1-census-final.tsv` has the `wasm32-unknown-unknown` rows too; they differ only by the cfg-selected code and stay inside the budget). Before = the crate was a module of the monolith (102,014 lines / 402 files / 5,966,814 bytes lib closure at WP0).

| crate | module files owned | first-party deps (declared) | lib files / bytes / lines | test files / bytes / lines |
|---|---:|---|---|---|
| `mailbox-core` | 2 | - | 2 / 9,367 / 224 | 6 / 40,998 / 493 |
| `canvas2d-paint` | 2 | - | 2 / 25,888 / 627 | 5 / 66,711 / 811 |
| `hub-contracts` | 3 | - | 3 / 64,573 / 1,310 | 9 / 219,274 / 1,948 |
| `hub-workspace` | 4 | `contracts`, `doors`, `hub-contracts`, `media-slots` | 5 / 68,484 / 1,203 | 12 / 140,142 / 2,272 |
| `media-slots` | 5 | - | 5 / 62,800 / 382 | 15 / 133,156 / 720 |
| `host-base` | 11 | - | 12 / 94,900 / 1,686 | 16 / 105,576 / 1,807 |
| `contracts` | 5 | - | 5 / 19,594 / 386 | 6 / 19,685 / 412 |
| `kernel-protocol` | 6 | `contracts`, `host-base` | 6 / 119,306 / 2,507 | 9 / 130,281 / 2,707 |
| `kernel-retained` | 2 | `kernel-protocol` | 2 / 39,304 / 899 | 3 / 39,401 / 909 |
| `kernel-pool` | 4 | `contracts`, `host-base`, `kernel-protocol`, `kernel-retained` | 5 / 143,878 / 2,311 | 6 / 143,971 / 2,316 |
| `kernel-runtime` | 2 | `contracts`, `host-base`, `kernel-pool`, `kernel-protocol`, `kernel-retained` | 3 / 62,759 / 951 | 7 / 152,514 / 2,388 |
| `program-bridge` | 3 | `contracts`, `host-base`, `kernel-pool`, `kernel-protocol`, `media-slots` | 3 / 121,043 / 1,362 | 14 / 158,705 / 1,740 |
| `doors` | 3 | `host-base` | 3 / 44,122 / 637 | 5 / 53,094 / 859 |
| `agent-elements` | 6 | `doors`, `host-base`, `media-slots` | 6 / 101,365 / 2,110 | 15 / 173,726 / 3,057 |
| `dock` | 2 | `contracts` | 2 / 109,829 / 2,157 | 9 / 163,309 / 2,565 |
| `native-platform` | 2 | - | 2 / 19,803 / 404 | 5 / 41,835 / 617 |

Top crate `semio-framework-os-renderer-wgpu` lib closure: **111 files / 4,298,558 bytes / 73,158 lines** when my last crate landed (was 402 / 5,966,814 / 102,014); at the end of the session peers' carves and staging make it 344 files / 3,925,929 B / 64,299 lines (`rwp1-census-final.tsv`); still over the gate until WP2..WP5 land. Its native test closure cannot be counted (peers removed `🖱️ui/🧬️contract/🕰️host-temporal-format/🧪️tests/🧫️fixtures/🔣️.json`: `Missing Rust source input`), as at WP0.

## 4. Lifts: what arrived, what was mounted, what is missing

| item (plan 5.2) | owner | state |
|---|---|---|
| `framework_widget_context` to `contracts` | WP3 | delivered, **mounted** in `contracts`; WP3's `contracts_lift` mount in `🗣️Interpreter/…/🖼️ui-image-loading` replaced by `pub use semio_framework_os_renderer_wgpu_contracts::framework_widget_context;` (edit in WP3's file, requested by the coordinator) |
| retained UI engine cell, close queue, scene pointer owners to `contracts` | WP3 | delivered, **mounted** in `contracts`; WP3's `contracts_lift` mount in `🗣️Interpreter/…/🧩️retained-ui-engine` replaced by an explicit `pub use semio_framework_os_renderer_wgpu_contracts::{UI_ENGINE, UiEngineCell, …}` (13 names, `TEST_UI_ENGINE_LOCALE` under `cfg(test)`), so `UI_ENGINE`, `UI_DOCUMENT_CLOSE_QUEUE` and `SCENE_POINTER_OWNERS` are mounted exactly once (in `contracts`) |
| `declare_boot_subphase`, `RuntimeHostWaker` to `host-base` | WP3 | delivered, **mounted** in `host-base`; WP3 mounts them too (stateless) |
| `host_io_call` and trace (89) to `host-base` | WP4 | **not delivered; verbatim copy mounted** (section 1.4) |
| `AdmittedSurface*` family (307), `EngineSurfaceToken` (5) to `contracts` | WP2 | **not delivered** (`scene-state` keeps them): blocks `asset-authority` and `asset-decode` |
| `NativeHostTemporalFormatter` (16) to `native-platform` | WP2 | **not delivered**: blocks `native_temporal` |
| shell find items (135), `ShellHubRemoteV1` (14) to `contracts` | WP4 | not delivered (no WP1 crate needs them yet) |

## 4a. Requests to the other packages (nobody's file was edited)

* **WP2**: write `🤝️contracts/📥️lifted/🗃️scene-state/🦀️.rs` (AdmittedSurface family) and `🖥️native-platform/📥️lifted/🎞️Scenes/🦀️.rs`; then I (or WP0) add one `mod` line each to `contracts`, `native-platform` and carve `asset-authority`, `asset-decode` (analysed with `needs.py`: apart from `AdmittedSurfaceToken`, one enum variant and one field, everything they name comes from `host-base`, `std` or external crates). The scenes tree must then re-export from the crates.
* **WP3**: replace the mounts of `host_base_lift` (browser worker) and `host_waker` (presentation raster) by `pub use semio_framework_os_renderer_wgpu_host_base::{declare_boot_subphase, RuntimeHostWaker};` (the two `contracts_lift` mounts are already replaced); move the `media-slots` roster law (`🎬️media-slots/🎟️collect/🧪️tests`) into `host-plumbing` and drop the `host-plumbing-law` gate; the `RENDERER_SOURCE` laws of `frame-close`, `physical-job-close`, … read through the module-tree helper, which does not follow `extern crate` (section 6.4).
* **WP4**: cut the four host-io items and re-export them from `host-base`; deliver the two `contracts` lifts. **Four Shell test files reach into `cfg(test)` helper modules of carved crates**, which do not exist in a dependency's non-test build (the only real signal of the `--cfg test` differential, 5 rows): `🐚️Shell/🧪️tests/📏️wgpu-window-measures` and `🎬️wgpu-window-actions-search-panes` and `🔬️wgpu-shell-input` use `crate::program_bridge::{window_measures_section_tests, window_engagements_section_tests}::{fixture, measures_section_document, engagements_section_document, section_document}`, and `👕️board-presence` uses `crate::canvas_presence::tests::{numeric, peer}`. They need either a `testkit` feature of `program-bridge`/`hub-workspace` enabled by the top crate's dev-dependencies, or the fixtures moved into `test-support`/the Shell test bins (WP4/WP5 re-home those tests anyway). I did not guess between the two.
* **WP0**: `Cargo.lock` refresh (section 6.1), taxonomy registration of `📥️lifted` (section 6.5), `verify_static.ts` frozen table (dev edge `hub-workspace -> doors`, `kernel-runtime -> kernel-pool, kernel-retained`), the nested-cargo projection fixture (a historical record of the old library root, left as it was).


How the missing pieces plug in (each is one mount in a crate root, nothing else changes in the crates that already exist):

```
// 🤝️contracts/🦀️.rs                        after WP2 writes 📥️lifted/🗃️scene-state/🦀️.rs
#[path = "📥️lifted/🗃️scene-state/🦀️.rs"]
mod admitted_surfaces;
pub use admitted_surfaces::*;
// 🖥️native-platform/🦀️.rs                   after WP2 writes 📥️lifted/🎞️Scenes/🦀️.rs, plus `native_temporal` (T/🕰️native-temporal) as `pub mod`
// asset-authority, asset-decode             then: crate roots = `mod asset_authority; pub use asset_authority::*;` etc. exactly like the kernel crates,
//                                           the 22 prefix validators (asset-decode-jobs lines 576 to 599) and IconRenderHost/📤️export/📥️asset move into asset-authority
```

## 5. Verification (commands run and their real results)

**5.1 Full-resolution union differential of the whole monolith** (`uo.py <tag> --mode full`, log `🗑️generated/rwp1-uo-f8.log`; the raw `rwp1-oracle-f8-*.jsonl` (15 MB each) were deleted after the rows below were read; `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --error-format=json --emit=metadata -o /dev/null`).

* **A** = a sandbox copy of the live engine tree where the 16 carved crates are mounted as modules of ONE crate (the library root plus `#[path] pub mod semio_framework_os_renderer_wgpu_<crate>;`), every `crate::` in a crate's files rewritten to `crate::<crate>::`, `pub(crate)` to `pub(in crate::<crate>)` (so a cross-crate use of a crate-private item is E0603), crate-name paths to module paths and `extern crate <crate> as <alias>;` to `use`. The sandbox patches of `📓️RESOLUTION-ORACLE.md` apply (extern crates become empty modules, external glob imports `const _: () = ();`).
* **B** = the live tree with every WP1-owned directory and the hub, the renderer root and the library root restored from a snapshot taken before my first edit (`SNAP`), so edits of the other packages that happen during the runs cancel; the crate folders keep the files other packages wrote into them.
* The error multisets are compared on (code, message, span text, label) without the file path, because code moved between files. A difference is **noise** when it is an unresolved external crate or attribute (the count differs because the same `use` now exists in more files) or an error in an extern-prelude alias module (`protocol`, `store_sync`, … are empty modules in the sandbox); **signal** is everything else.

| config | errors A | errors B | only in A | signal rows | only in B |
|---|---:|---:|---|---:|---:|
| `lib` | 4524 | 4499 | 32 (32 external-crate or extern-alias count noise) | **0** | 7 |
| `wasm` | 4054 | 4017 | 41 (41 external-crate or extern-alias count noise) | **0** | 4 |
| `test` | 1360 | 1341 | 22 (16 external-crate or extern-alias count noise) | **5** | 3 |
| `wasm-test` | 6765 | 6743 | 44 (39 external-crate or extern-alias count noise) | **3** | 22 |

Signal rows of the final run `f8` (all in the two test configurations):

```
  +SIGNAL 2 E0603 function `fixture` is private | fixture | {'E/el/🐚️Shell/🧪️tests/📏️wgpu-window-measures/🦀️.rs': 1, 'E/el/🐚️Shell/🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs': 1}
  +SIGNAL 1 E0603 function `measures_section_document` is private | measures_section_document | {'E/el/🐚️Shell/🧪️tests/📏️wgpu-window-measures/🦀️.rs': 1}
  +SIGNAL 1 E0603 module `tests` is private | tests | {'E/el/🐚️Shell/🧪️tests/👕️board-presence/🦀️.rs': 1}
  +SIGNAL 1 E0603 function `engagements_section_document` is private | engagements_section_document | {'E/el/🐚️Shell/🧪️tests/🎬️wgpu-window-actions-search-panes/🦀️.rs': 1}
  +SIGNAL 1 E0603 function `section_document` is private | section_document | {'E/el/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs': 1}
  +SIGNAL 3 E0422 cannot find struct, variant or union type `FrameLatencyObservation` in this scope | FrameLatencyObservation | {'E/🧪️tests/⏱️frame-latency-diagnostics/🦀️.rs': 3}
  +SIGNAL 1 E0603 module `tests` is private | tests | {'E/el/🐚️Shell/🧪️tests/👕️board-presence/🦀️.rs': 1}
  +SIGNAL 1 E0603 function `section_document` is private | section_document | {'E/el/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs': 1}
```

Reading of the signal rows: the lib and wasm configurations are clean. The test configurations show two causes, both outside what the carve can fix alone. (a) Four Shell test files use `cfg(test)` helper modules of `program-bridge` and `hub-workspace` (`window_measures_section_tests::{fixture, measures_section_document, section_document}`, `window_engagements_section_tests::{fixture, engagements_section_document}`, `canvas_presence::tests::{numeric, peer}`): the helpers cannot exist in a dependency's non-test build (request to WP4 in section 4a). (b) `FrameLatencyObservation` (3 rows, wasm test only) was a private struct that the frame latency law of the hub test host builds; it and the private fields and methods that law touches were made `pub` after the run (`⏱️frame-latency`), checked by the crate probe, not re-run through the 25-minute full test differential. The `f6` run before it had caught the inline-module test hosts (fatal `couldn't find file`), the first version of which WP3 reported as well.

**5.2 Sensitivity.** The same differential with one planted defect each: `pub` removed from `renderer_worker_pool` (3 signal rows: E0425 in `host-base`, unresolved imports in `agent-elements` and `kernel-pool`), the `std::sync::{Arc, Mutex, OnceLock}` import removed from the `kernel-protocol` root (6 signal rows, 20 + 17 + 10 missing names), a broken `#[path]` literal in the `doors` root (fatal `couldn't find file`, 1 error against 5,388). Logs `rwp1-uo-seedpub.log`, `rwp1-uo-seedimp.log`, `rwp1-uo-seed1.log`.

**5.3 Per-crate resolution probes** (`uo.py <tag> --mode crates`, only the carved crates mounted, all four configurations `lib`, `--test`, `wasm32-unknown-unknown` lib and `--test`): after the final fixes every resolution-class error (E0412, E0425, E0422, E0423, E0432, E0433, E0603, E0616, E0624, E0252, …) that is left names an external crate or the empty extern-alias modules; the only exception is `JsValue` (87 uses in `host-base` and the program bridge: the `wasm_bindgen::prelude::*` glob is neutralised by the sandbox and the baseline has the same errors).

| check | result |
|---|---|
| census (`census.py`, canonical `readRustSourceClosureV1`) | section 3: 16 crates, lib and test, native and wasm: all `ok` (at most 2,507 lines, 8 files … see table; test closures at most 3,057 lines / 219,274 bytes) |
| real `rustc` on `mailbox-core` (the one crate with no dependency: `rustc +nightly-2026-07-20 --edition 2021 --crate-type lib --emit=metadata -o /dev/null <crate root>` for the host, `--target wasm32-wasip2` and `--target wasm32-unknown-unknown`, also with `-W unused -W rust_2018_idioms`) | complete analysis, no diagnostic on any of the three targets: the only crate of the 16 whose lib type-checks for real without the gate (its tests need `serde_json`) |
| definition scan (`defscan.py`: private fields, impl members and free items of each crate that sources outside it mention; names also defined `pub` elsewhere and short names are skipped) | 75 name-level mentions over 16 crates, all read: coincidences (a local variable or another struct's field of the same name, trait-impl methods such as `send_text`), child test files of the same module (`🧪️tests` below the element), and one real finding, fixed: `UiDocumentCloseQueue` and `ScenePointerOwners` of the retained UI engine lift were private types behind `pub static`s, now `pub`. No other private member of a carved crate is used across a crate boundary. Output `rwp1-defscan.txt` |
| losslessness (`lossless.py`, line multiset before/after with `pub(crate)`/`pub(super)` folded into `pub`) | 27,412 lines before, 27,965 after (+553: crate roots, test hosts, headers, repeated region markers); 162 lines lost, all explained: 48 replaced mounts, 6 replaced `use` lists, 108 lines whose only change is `pub` added to a private item that a sibling crate names or a path shortened (`crate::program_bridge::ProgramFault` to `ProgramFault`, `crate::shell::host_io_call` to `crate::host_io_call`), and the kernel core's `KernelClient::get().try_acknowledge_job_progress(self.token)` that a peer replaced by the callback (not my edit). List `rwp1-lossless.txt` |
| region markers (inline script over the owned trees before/after) | 100 region names before, 103 after, none lost, no name with fewer markers, every one of the 288 files of the trees balanced; the wasm wrappers that stay in the root repeat `HostAppearance`, `HostIntroduction`, `HostPlatform`, `HostLocale`, `BootDescriptor`, the four kernel cuts repeat the regions open at the cut |
| duplicate statics (`dupstatics.py`) | 1 of mine: `HOST_IO_REQUEST_TRACE` (section 1.4; `COMPONENT_SURFACE_CLOSE_BRIDGE` is a WP2/WP3 lift); no source file is mounted by two WP1 crates; four files are mounted by a WP1 crate **and** by a WP3 module (section 4) |
| rustfmt (`rustfmt +nightly-2026-07-20 --check --config skip_children=true,unstable_features=true`) | the 16 crate roots, the 12 lifted files and the hub parse and are formatted (the roots with generated import lists were formatted in place); touched module files keep their pre-existing formatting |
| path literals (`pathcheck.py`: `#[path]`, `include!`, `include_str!`, `include_bytes!` over 549 files of the 16 trees, the hubs, the root and the library root) | 1,181 literals, 4 unresolved, none caused by this work: `native-temporal` names `🕰️host-temporal-format/🧪️tests/🧫️fixtures/🔣️.json` (twice) and the async-boundary test names two `🖍️draw` files that peers removed |
| manifests (`tomlcheck.ts`, Bun TOML) | 16 manifests parse, 117 path dependencies resolve to a manifest of the declared package, `launch.json` parses |
| static registry gate (`rwp0-tools/verify_static.ts`: DAG, rank order, members, owner rows, launch routes, 1,765 path dependencies) | 11 failures, all of the check "first-party dependency set equals the frozen table": 10 belong to crates of WP2/WP3, 1 is mine and intended: `kernel-runtime` declares `kernel-pool` and `kernel-retained` because its facade re-exports them (plan 6.2 says the facade re-exports the four kernel crates but freezes three edges; both new edges point down). No rank, cycle, member, owner or launch-route failure. `depcheck.py`: every first-party crate a crate names is declared; 4 declared edges are not named yet |
| TS readers, `bun test` (20 files, live tree against the baseline with the WP1 directories restored, `ts_diff.py`, final run `rwp1-ts-t3-*.log`) | 19 identical (pass/fail counts and failing test names); the 20th, `browser-frame-transport`, passes one more law in the live tree (46 pass / 16 fail against 45 / 17): the boot descriptor law that read the renderer root now reads the `host-base` root, and the baseline's restored `host-base` is a skeleton. The first run (`t1`) also showed `engine-surface-retention` once, caused by a shell edit of WP4 between the two runs; it is identical in `t3` |
| TS readers, vitest (17 files) | 16 identical (13 fail identically before and after: empty fixture folders, unresolved package); the 17th is the `browser-frame-transport` law above (passes in the live tree) |
| launcher | `bun repo cargo --store=shared --configuration=prototype --parallelism=4 -- check -p semio-framework-os-renderer-wgpu-mailbox-core --all-targets --message-format=short` with the `SEMIO_SCRIPT_POLICY` the coordinator gave, at the start and at the end: `error: Insufficient physical compiler initial headroom` (`Physical compiler admission refused … availableBytes 4291510272, requiredBytes 4294967296`; the fixed image under `⚡️cache/rust/fixed/m` is 3.4 MB short). `bun repo run @semio-tech/os-renderer-wgpu-mailbox-core-rs:check …`: `Owned Nx invocation failed (1)`. The gate census (`census.ts`, `rwp1-census-gate.out`) says only `mailbox-core`, `canvas2d-paint`, `media-slots` and `native-platform` have a closure with no oversized dependency (`closureCompilable: true`); the other 12 sit behind `semio-framework-os-kernel`/`semio-framework`/`plugin` (1 to 3 oversized dependencies each), exactly the frontier WP0 recorded. None was compiled |


## 6. Details of the evidence

**6.1 Real compile.** Tried first thing and last thing with the `SEMIO_SCRIPT_POLICY` of the coordinator: the launcher passes the lease stage and then refuses with `Insufficient physical compiler initial headroom` (the fixed compiler image has 4,291,510,272 of the required 4,294,967,296 bytes free; "Bounded inactive Cargo retirement plan units 0"), so no crate was compiled and `Cargo.lock` was not refreshed. `lockdiff.py` lists what the next `update --workspace --offline` must change: 50 dependency-list differences over 15 crate entries (only additions; e.g. `kernel-runtime` gains `kernel-pool`, `kernel-retained`, `tokio`; `program-bridge` gains `semio-framework-actor`, `-hash`, `-os-services`, `-plugin-host`, `-trace`, `wasm-bindgen`; `hub-workspace` gains `doors` as a dev edge). Four crates have a closure without an oversized dependency and are the first to try when the launcher works: `mailbox-core`, `canvas2d-paint`, `media-slots`, `native-platform`.

**6.2 Why the union differential is the right size of evidence here.** It resolves every path the carve changed, including the glob re-exports (`pub use <module>::*`), the root re-exports of the monolith (`pub use <crate>::{…}`), the extern aliases, the cross-crate visibility (every `pub(crate)` of a carved file is `pub(in crate::<crate>)` in A) and the macro (`action_args_json!` is a `#[macro_export]` of `host-base`; in the union the macro lands at the union root, which is where the 71 call sites look). It does **not** type-check bodies whose types come from an unresolved external crate, does not see trait scoping through glob imports (I checked the traits the old root block supplied: only `Future`, imported wherever `poll` or `Future` appears), and cannot see private fields or methods used across a crate boundary where the receiver type is unresolved (the definition scan covers those by name).

**6.3 Monolith consequences handled in the same change.**
* `R/🦀️.rs`: lines 107 to 737 left for `host-base` (578 lines); 7 `#[wasm_bindgen]` wrappers stay (export names identical before and after: `semioWgpuSetHostAppearance`, `…IntroductionSuppressed`, `…HostPlatform`, `…HostLocale`, `…RuntimeDiagnostics`, `…BootDescriptor`, `…PluginCatalog`); explicit `pub use`/`pub(crate) use` lines replace the definitions so `crate::log_debug`, `crate::boot_descriptor()` and every `use super::*` consumer keep resolving; `RENDERER_SOURCE` lists the new files.
* `PKG/📚️library/🦀️.rs` (WP0 file, two minimal edits): the wasi mount of the mailbox core file became `pub extern crate …_mailbox_core as runtime_mailbox_core;` (a second owner of the same file is not allowed), and the `renderer_registration` mount is gone (the macro is owned by `host-base`, the root re-exports `action_args_json`).
* The old kernel parent `R/📦️kernel-runtime/🦀️.rs` is deleted (its docstring is the facade crate root's); `kernel_runtime::KernelPoolFuture` now lives in `host-base` (re-exported by the facade); the renderer-I/O laws and the semantic-document law moved with their code (`kernel-runtime` mounts the semantic-document law with `include!`).
* Visibility of the monolith's own aliases was kept: `pub(crate) use` for what was `pub(crate) mod`, plain `use` for the private `native_runtime_modules`.

**6.4 Source-text readers.** The module-tree helper (`📚️compiler/📖️syntax/🌳️module-tree`) follows `#[path]` and `include!`, not `extern crate`; a reader that asks for the renderer root no longer sees the text of a carved module. I fixed the two readers that read WP1 text: the os-infinite `RENDERER_SOURCE()` list (`♾️infinite/🌍️world/🗃️model/🧪️tests/🔬️unit`: the deleted kernel parent was replaced by the kernel crate roots, the lifted files and `host-base`) and the boot descriptor law of `📨️browser-frame-transport` (reads the `host-base` root). The other readers of `🧊️renderer/🦀️.rs` (`frame-close`, `physical-job-close`, …) look for frame core text that is still in the monolith; they will need the same treatment as WP3 carves (a helper that resolves `extern crate semio_framework_os_renderer_wgpu_*` to the sibling crate root would fix all of them at once; not done, it is WP0's helper).

**6.5 Taxonomy.** `taxonomy-scope.ts` on a crate folder with a `📥️lifted` child reports `directory-kind-unresolved` for `📥️lifted` and each slug below it (for `host-base`: 4 errors plus the pre-existing frame-worker catalogue digest). The hand-off contract creates these folders, so the kind has to be registered once by WP0 (a member name of the crate-folder kind); I did not edit the taxonomy.

**6.6 Things deliberately left alone.** The nested-cargo projection fixture (`🖼️assets/📽️nested-cargo-package-projection/🔣️.json`) embeds the old library root text; it records a past migration and is not read as live state. `Cargo.lock`, the distribution manifest, launch routes, owner rows and workspace members were not touched (frozen by WP0).


## 7. Consumers re-pointed and files changed

Edited outside my crate folders (minimal consumer-side edits):

| file | edit |
|---|---|
| `T/📦️packages/🦀️rust/📚️library/🦀️.rs` (WP0) | wasi mount of mailbox-core -> `pub extern crate`; `renderer_registration` mount removed |
| `R/🦀️.rs` (renderer root, WP1 by contract) | 578 lines to `host-base`, re-export lines, `RENDERER_SOURCE` entries |
| `R/📥️mounts/🌱️wp1-foundation/🦀️.rs` (my hub) and four new files `R/📥️mounts/🌱️wp1-foundation/🧪️test-hosts/{⏱️frame-latency,🔬️renderer-io,📇️directory-door,🛰️dock-unit}/🦀️.rs` | extern crate / `pub use` lines, test hosts |
| `♾️infinite/🌍️world/🗃️model/🧪️tests/🔬️unit/🦀️.rs` (peer-owned crate, test only) | `RENDERER_SOURCE()` file list |
| `T/📦️packages/🟦️typescript/📜️script.ts` | media-slot contract laws run on the `media-slots` package |
| `E/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/{🧩️retained-ui-engine,🖼️ui-image-loading}/🦀️.rs` (WP3, at the coordinator's request) | the `contracts_lift` mounts became `pub use` of the `contracts` crate |
| `E/🧪️tests/📨️browser-frame-transport/🟦️.ts` | boot descriptor law reads the `host-base` root |
| `E/🧱️elements/🌉️ProgramBridge/🧪️tests/🕹️wgpu-reserved-verb-answer/🦀️.rs` | `ExchangeOutcome` from `kernel-protocol` |

In my trees: 16 crate folders (`🦀️.rs` root and `📦️packages/🦀️rust/Cargo.toml` each; 12 `📥️lifted/<slug>/🦀️.rs` files below `contracts`, `host-base`, `kernel-protocol`, `kernel-pool`), the module files of the trees listed in section 2 (visibility, path shortening, `use` lists cut out; full list `🗑️generated/rwp1-files-changed.txt`: 60 changed, 16 added, 1 removed in the engine trees, plus the os-infinite test outside them), `rwp1-tools/` and this file. Deleted: `R/📦️kernel-runtime/🦀️.rs`. Nothing was committed, staged, stashed or checked out. Longest new path: `T/🏘️hub-workspace/📦️packages/🦀️rust/Cargo.toml` (214 bytes, from WP0); longest path I created: `…/🧊️renderer/📥️mounts/🌱️wp1-foundation/🧪️test-hosts/📇️directory-door/🦀️.rs` (below 240 bytes).


## 8. Residual risks

1. **Nothing was compiled or executed** (section 6.1). The union differential proves name resolution and visibility, not types, borrowing or behaviour.
2. **Transitional duplicates until the other packages switch**: two stateless lifted files that WP3 mounts in `browser-worker` and `presentation-raster` (`declare_boot_subphase`, `RuntimeHostWaker`) are also mounted in `host-base`; `HOST_IO_REQUEST_TRACE` exists twice on wasm (a diagnostic label). The two stateful `contracts` lifts (`UI_ENGINE` and the pointer owners) are mounted once.
3. **Two crates and one half crate are missing** (`asset-authority`, `asset-decode`, `native_temporal` of `native-platform`) and `contracts` lacks the AdmittedSurface family, the shell find items and `ShellHubRemoteV1`; every crate that ranks above them (`engine-registry`, `runtime-mailbox`, `browser-worker`, `scene-*`) must keep importing those from the monolith until the lifts land.
4. **Visibility was widened mechanically** in whole files (`pub(crate)`/`pub(super)` to `pub`, and column-0 private items of the three lifted pool/io files). The definition scan and the oracle found no unused exposure that matters, but the crate APIs are wider than the minimum; narrowing is a clean-up that should follow a real compile (a `pub` that nobody names shows as `unreachable_pub` only under that lint).
5. **API of the monolith library**: `pub mod dock`, `pub mod media_slots`, … became private `extern crate` aliases; only `run_native_entrypoint` and the wasm exports are external API (plan 2.3), but a downstream user of `semio_framework_os_renderer_wgpu::dock::…` would break.
6. **Test coverage that moved or is gated**: the media-slots roster law (`host-plumbing-law`), four laws now run from hub test hosts (`🧊️renderer/📥️mounts/🌱️wp1-foundation/🧪️test-hosts/<slug>/🦀️.rs`, file modules because a `#[path]` below an inline `mod` goes through a directory that does not exist; the first version of these hosts used inline modules and broke the `--cfg test` differential, found by WP3 and fixed); they compile only in the monolith's test build, not in `cargo test -p <crate>`, and every unit test that stays in a crate was only name-resolved.
7. **Registry gates**: the frozen dependency table of `verify_static.ts` does not know the edges I added (`hub-workspace` dev `doors`, `kernel-runtime` -> `kernel-pool`, `kernel-retained`); declared-but-unused edges of the frozen table remain (`hub-workspace` -> `contracts`, `media-slots`; `program-bridge` and `agent-elements` -> `media-slots`).
8. **`action_args_json!`** expands to `semio_framework::optional_json_to_dsl(Some(serde_json::json!(…)))` with plain crate names: every crate that invokes it (71 uses today, in the shell, scenes and frame core) must declare `semio-framework` and `serde-json` even if it names them nowhere else. A `$crate`-based re-export in `host-base` would remove that coupling; not done because it changes the macro that 71 call sites share.
9. **Peers edit these trees**: the kernel files carry unstaged edits of other agents; my cuts re-read the files and asserted their text before writing, and the baseline of the differential is a snapshot from before my first edit, so a peer edit to a WP1 file during the runs would show as a difference in A against B. None was found (the differential shows no row that a peer file could explain).


## 9. Tools

`rwp1-tools/` (all read-only against the repo unless named `carve*`, `kernel*`, `hostbase*`, `widen`, `aliases`, `manifest --apply`, `depcheck --apply`): `common.py` (paths, WP1 crate table, owned directories), `rs.py`, `items.py`, `outline.py`, `modtree.py` (lexer, item table, module tree), `needs.py`, `provide.py`, `aliases.py` (explicit import lists from the old root namespace), `carve.py`, `widen.py`, `hostbase1.py`, `hostbase2.py`, `kernel1.py`, `kernel2.py`, `kernel3.py` (the actual cuts), `manifest.py`, `depcheck.py`, `lockdiff.py`, `tomlcheck.ts`, `pathcheck.py`, `lossless.py`, `defscan.py`, `dupstatics.py`, `census.py`/`census-wp1.ts`, `uo.py` (union oracle: `--mode crates|full`, `--seed`), `uo_reparse.py`, `ts_diff.py`, `report_tables.py`, `finalize_result.py`, `LEDGER.md`. Re-run the whole verification: `python3 uo.py <tag> --mode full --configs lib,test,wasm,wasm-test` then `python3 uo_reparse.py <tag> <config>`; `python3 census.py --top`; `python3 depcheck.py`; `python3 pathcheck.py`; `python3 defscan.py`; `python3 ts_diff.py <tag> both`. The pre-carve snapshot used as baseline lives in the session scratchpad (`rwp1/snap0`, wiped on process restart); to rebuild a baseline later, restore the WP1-owned directories from `HEAD`/the index.

