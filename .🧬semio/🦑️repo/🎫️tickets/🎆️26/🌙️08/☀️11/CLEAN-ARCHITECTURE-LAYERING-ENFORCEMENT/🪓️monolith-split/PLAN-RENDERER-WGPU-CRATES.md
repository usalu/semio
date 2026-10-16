# 🪓️ Plan: Crate Partition of `semio-framework-os-renderer-wgpu`

Planner: Sonnet 5.5, read-only, foreground, no sub-agents. Consumers: wave-4 executors. This is a plan, nothing was edited, compiled or committed. Every number below was measured on the live working tree on 2026-10-15 (15:20 to 16:40 CEST) with the canonical counter `readRustSourceClosureV1` and a token-level reference graph; section 12 lists the exact method so any executor can re-measure in minutes.

**WP0 executed 2026-10-15 (phase 0, `RESULT-RENDERER-WP0.md`).** Every place this plan changed against the live tree is marked **[WP0 change]**; the hand-off contract is section 10.4, the frozen crate, hub and lift tables are Appendices F, G and H. The numbers of sections 1 to 9 keep the planner's values and show the WP0 re-measure as `(was …)` where it moved. **Integration pass 1 (2026-10-16)** resolved the cross-package hand-offs and flipped the crates whose dependencies are real; its state table and the rules it added are section 10.5.

Notation. `E/` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/`. `T/` = `E/🎯️targets/🧊️wgpu/`. `R/` = `T/🧊️renderer/`. `PKG` = `T/📦️packages/🦀️rust/`. A crate named `x` below is `semio-framework-os-renderer-wgpu-x`. "Lines" are the gate's active lines (the cfg-selected text split on newlines, blank and comment lines included). "WP" is a work package (section 10).

## 0. Verdict in twelve lines

1. Today the library closure is **402 files / 5,966,814 bytes / 102,014 lines** (native; was 146 / 5,861,374 / 101,831), 386 files / 5,564,510 B / 91,472 lines (wasm32-unknown-unknown; was 130 / 5,459,031 / 91,304), **2 files / 9,943 B / 219 lines** (wasm32-wasip2: only `📮️runtime-mailbox-core`). The test closures are **756 files / 11,325,874 B / 157,346 lines** (native, 473 embedded files; was 505 / 11,217,952 / 157,068 / 223) and 626 files / 9,489,058 B / 123,883 lines (wasm32; was 375 / 9,383,249 / 123,640) as measured before the hubs (the test rows cannot be re-measured at the moment, 10.4.8). **[WP0 change]** The `OUT_DIR` include is gone, but the embedded icon SVGs are **not**: a peer replaced the build-script copies by a checked-in generated file `🖼️assets/🔣️icons/🤖️generated/🧩️icon-embeds/🦀️.rs` (255 lines) that `include_str!`s the 250 raw icon SVGs, mounted by `IconRenderHost`. That is +251 files and 261 embedded files in the lib closure and breaks the file budget of `icon-host` (risk 7, section 13).
2. The binding limits are **lines and bytes**, not files: 101,831 / 3,500 = 30 lib crates minimum, 5.86 MB / 230,000 B = 26 minimum, and the test closures need 157,068 / 3,500 = 45. The plan therefore proposes **46 non-Shell lib crates (45 new + the existing crate reduced to the thin top crate) + 24 Shell lib crates + 19 companion test crates (4 non-Shell, 14 Shell, 1 shared test-support) = 89 crates, 88 of them new** **[WP0 change]** (the planner counted 12 Shell bins and 87 crates; Appendix D lists 14 bins), every one at or below 3,500 lines / 28 files / 230,000 bytes for the lib closure and, with its unit tests, for the test closure (estimates, section 6; the tightest are called out).
3. At **module/file level** the reference graph is one strongly connected component of 101 of the 138 library files. At **item level** (4,585 items, impls bound to their type) there is no cycle larger than 988 lines (`ShellState` + `IconExportBatch` + `UiPrefsSnapshot` + `ShellChromePresentState`) except the frame-core knot (`AppRuntime` / `RuntimeMailbox` / `RuntimeApply` / `NativeReferenceDecodeJob`: 742 lines of types, about 3,000 with the impls that must stay beside them). So every cycle is cuttable; the file boundaries chosen by the earlier splits are simply the wrong cut surface.
4. After the cuts of section 5 the 46 non-Shell crates form a DAG with **3 residual back edges**, each with a prescribed inversion (section 5.3). 5,685 lines in 45 item groups move between today's files (section 5.2); four impl-ownership fixes are among them (a type and its inherent impl must share a crate: `KernelPoolState`, `AppFramePresentation`, `AppFrameBuild`, `AppFramePreparation`).
5. `ShellState` is the only inherent impl that cannot live in one crate: 248 fields, 701 methods, **21,518 lines in 20 files**. Section 7 decomposes it into 15 component structs in 13 crates (fields and methods together; two leaf crates `shell-vocab` and `shell-async` below them), a thin composer `ShellState` of components (`shell-state`, about 450 lines), 4 use-case crates that work on borrowed component bundles and 4 view crates (24 Shell crates). No extension traits. Ten ports (lowering moves or two callback seams) carry the upward calls that the call graph cannot lower.
6. The 5 other multi-hundred-line impls (`KernelPoolState` 1,898, `RuntimeMailbox` 1,602, `AppPresenter` 815, `FrameTransaction` 706, `AppRuntime` 696 in two files) each fit one crate; only `AppRuntime` and `OsHost` need their two impl files merged into one crate (done by the crate boundaries below).
7. The crate root today is `PKG/📚️library/🦀️.rs` which `include!`s `R/🦀️.rs` (so `R/🦀️.rs` is the real crate root: 921 lines of host statics, boot descriptor, logging and **a 123-name `use` block that every child inherits through `use super::*` globs**). Dissolving that root is the first job (WP0) and moves 123 imports into per-crate import lists (section 6.3).
8. The only external Rust API of the crate is `run_native_entrypoint`; the wasm component crate just links the library for its 47 `#[wasm_bindgen]` exports (section 8). The top crate stays thin: `pub use`, the wasm export wrappers, the native entry, `build.rs`.
9. Tests are the larger cost: 55,000+ test lines, 218 embedded JSON fixtures (1.36 MB), **95 `include_str!("….rs")` source-text embeds** (640 KB of other crates' `.rs` in 5 files) and 15 `*_SOURCE` concatenation consts. Source-text laws must read at run time (or move to TS) because an embed pulls another crate's source into this crate's budget (section 9).
10. Parallelism: WP0 (serial, first: skeletons for all 88 new crates, registries, root split into one mount file per WP; **executed**, section 10.4) then WP1..WP5 in parallel on **disjoint source files** (section 10), then WP0 again (top crate, TS readers, locks, gates).
11. Nothing here could be compiled: the compiler-ingress gate refuses the old crate (and `semio-framework-3d`) before rustc starts. Verification is the full-resolution differential from `RESULT-HARDEN-WGPU.md` per crate root plus census rows, definition scans and rustfmt (section 11).
12. Peers are changing the tree under this plan (the `🔤️compiler-fonts` to `🔤️runtime-assets` rename and the icon-pack runtime channel landed during the session; the old `📋️` / `🗑️generated` files are untouched). Re-run the census (section 12) at WP0 start and diff against section 1. **[WP0 change]** The re-run found: `runtime-assets` grew to 88 lines, the icon SVG embeds above, `IconRenderHost` root 185 to 179 lines, a one-line `use ui_gpu::{GpuContext};` in the root, and a handful of Shell/frame files that moved by 1 to 170 bytes (Appendix A). No file moved between packages.

## 1. Baseline, budgets, what binds

### 1.1 Closure measurements (gate counter, host cfg + `debug_assertions`)

| Closure | cfg | files | bytes | active lines | embedded files |
|---|---|---:|---:|---:|---:|
| lib | native (aarch64-apple-darwin, `not(wasm32)`) | 402 (was 146) | 5,966,814 (was 5,861,374) | 102,014 (was 101,831) | 261 (was 11) |
| lib | `wasm32-unknown-unknown` | 386 (was 130) | 5,564,510 (was 5,459,031) | 91,472 (was 91,304) | 258 (was 8) |
| lib | `wasm32-wasip2` (only `runtime_mailbox_core` mounts) | 2 | 9,943 (was 10,079) | 219 (was 221) | 0 |
| lib + tests | native, `--cfg test` | 756 (was 505) | 11,325,874 (was 11,217,952) | 157,346 (was 157,068) | 473 (was 223) |
| lib + tests | wasm32, `--cfg test` | 626 (was 375) | 9,489,058 (was 9,383,249) | 123,883 (was 123,640) | 405 (was 155) |

**[WP0 change]** Re-measured 2026-10-15 17:58 (lib rows, after the five mount hubs: +5 files, the renderer root keeps 665 active lines) and 17:30 (test rows, before the hubs; the test closure cannot be re-measured since a peer removed `🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs`). The 251 new files are the icon embeds (250 SVGs + the generated index) plus the five hubs; the lines moved by the hubs are neutral (about +25 lines for the banners, the five `include!` lines and the repeated region markers). `closure.ts` of the ticket folder reproduces every row.

The native test closure counts 29 fixture files that do not exist on disk (the counter is run with those `include_str!` fixtures stubbed empty, exactly like the hardening oracle did); the real byte count is a little higher.

### 1.2 Per-crate budget (`AGENTS` / wave-4 brief)

At most **3,500 active lines, 28 files, 230,000 bytes** for the lib closure **and** for the test closure (`cargo test` units are gated too). Embedded `include_str!` / `include_bytes!` files count as files and bytes. Hard ceiling is 4,000 / 32 / 262,144; the headroom is deliberate.

Consequences for the partition:

* A crate's test closure = its lib closure plus every test file it mounts plus the fixtures those embed. Tests are therefore sized with the lib: a 2,600-line lib leaves 900 test lines.
* Tests that need more than the crate has left go to a companion crate `x-tests` (dev-only, `publish = false`, lib root contains only `#[cfg(test)]` mounts). A companion's lib closure is empty; its test closure is its tests plus fixtures, and it depends on the production crates it exercises through their `pub` API.
* Source-text readers must not embed (section 9).
* Path length: keep every repo-relative path at most 256 characters and at most 240 UTF-8 bytes (the earlier splits used 240). A typical new crate manifest path is 146 characters / 214 bytes; keep folder names after `🧊️wgpu/` at most 28 bytes.

### 1.3 What is in flight (do not collide)

* `R/🦀️.rs`, `R/🔬️native-socket-probe/🦀️.rs`, `R/📦️kernel-runtime/🏊️kernel-pool/🦀️.rs`, `T/🏗️builder/🦀️.rs`, the crate `Cargo.toml`, `T/🔤️runtime-assets/` (new, was `🔤️compiler-fonts`), `E/🧪️tests/🔤️wgpu-compiler-fonts/🟦️.ts` (deleted) are staged or modified by a peer. They are included in every measurement here. **[WP0 change]** During phase 0 a peer also added `use ui_gpu::{GpuContext};` to the renderer root and removed `🖱️ui/…/🧊️gpu/🦀️.rs`; `🖼️IconRenderHost/…/🦀️.rs` is staged with the icon-embeds change.
* `semio-framework-os-kernel`, `plugin`, `replication`, `ui-contract`, `ui` aggregate producers, `job`, `2d`, `pack` are peer-owned: touch them only for consumer-side `Cargo.toml` / import edits.
* The distribution manifest `🧑‍💻dev/📤️distribution/🧾️manifest.json` is generated and stale for every earlier split; regenerate with its generator at the end (WP0).

## 2. Inventory

### 2.1 Crate layout and mounts

* `PKG/Cargo.toml` (`name = "semio-framework-os-renderer-wgpu"`, `build = "build.rs"`, lib path `📚️library/🦀️.rs`, `[[bin]] semio-wgpu-native` at `💾️binary/🦀️.rs` behind feature `native-bin`). Features: only `native-bin`.
* `PKG/📚️library/🦀️.rs` (23 lines; **[WP0 change]** 20 lines now): mounts `R/📇️registry` (non-wasi; stays here, 10.4.1), ~~`T/⌨️native-entrypoint/📦️modules` (native)~~ (moved into the WP1 hub), `T/📮️runtime-mailbox-core` (wasi), `T/⌨️native-entrypoint` (native, re-exports `run_native_entrypoint`), the paint-policy test (cfg test), **`include!("…/R/🦀️.rs")` (non-wasi)**, and a `#[global_allocator]` heap witness (cfg test, non-wasi).
* `R/🦀️.rs` (the included root, 921 lines; **[WP0 change]** 665 active lines after phase 0, the module mounts, the 12 partitions and the root-level test mounts now live in the five hubs `R/📥️mounts/<hub>/🦀️.rs`, `include!`d from the root, Appendix G): `extern crate` aliases (`infinite_canvas as infinite_world`, `semio_framework_os_kernel as dsl / protocol / store / store_sync / dsl_core`, `semio_framework_async as wasm_bindgen_futures`), 42 `#[path]` module mounts (`pub mod dock`, `agent_bridge`, …, `shell`, `scenes`, `interpreter`, `engine_canvas`, `program_bridge`, `os_host`, `winit_app`, …), 12 renderer partitions (`asset_authority` … `kernel_runtime`, each `pub(crate) use child::*` except the native-only ones), the 123-name import block, the host statics (`HostAppearance`, `HOST_PLATFORM*`, `HOST_LOCALE`, `HOST_INTRODUCTION_SUPPRESSED`), the boot descriptor (`WgpuBootDescriptor` and friends, `BOOT_DESCRIPTOR`), logging (`log_debug*`), `app_now_ms`, `actor_budget_from_turn_budget`, `drive_renderer_worker_task`, `spawn_app_task`, the `RENDERER_SOURCE` test const, 6 mounted root-level test modules and 2 `include!`d standalone tests.
* Mount depth: lib root, root, 42 element/target modules, their children (the five earlier splits created 16 + 15 + 20 + 9 + 10 = 70 child files), tests.
* cfg gates on mounts: `not(target_arch = "wasm32")` (native-only: kernel-runtime, scale-bench, native-temporal, native-accessibility, native-entrypoint, native-caller, parallel runtime, sqlite), `target_arch = "wasm32"` (browser-worker, 28 wasm exports), `not(target_os = "wasi")` (everything under the included root), `target_os = "wasi"` (only runtime-mailbox-core), `test`, `all(test, not(target_arch = "wasm32"))`.

### 2.2 Families (native lib closure, active lines; tests = lines of test modules attached to the family's files)

| family | files | lines | KB | wasm32 lines | test modules | test lines |
|---|---:|---:|---:|---:|---:|---:|
| Shell (20 split children + 4 pre-existing + parent) | 28 | 37071 | 2259 | 35791 | 56 | 24800 |
| Scenes (15 children + parent) | 16 | 12354 | 604 | 12339 | 15 | 5820 |
| EngineCanvas (9 children + parent) | 10 | 7785 | 401 | 7785 | 8 | 4044 |
| renderer partitions (frame-build, app-runtime, mailbox-impl, presentation-raster, frame-presentation, frame-actions) | 6 | 7727 | 434 | 6304 | 1 | 97 |
| Interpreter (10 children + parent) | 11 | 6263 | 374 | 6521 | 5 | 3528 |
| host plumbing (os-host, winit-app, frame-job, doors-adjacent files, native entry, deadlines, latency, browser worker, accessibility) | 17 | 5488 | 313 | 4958 | 15 | 2655 |
| kernel-runtime (4 partitions + root) | 5 | 5476 | 290 | 0 | 2 | 1527 |
| renderer assets (glb-decode, asset-authority, decode-jobs) | 3 | 4142 | 206 | 3797 | 1 | 221 |
| hub elements (+ canvas-presence) | 5 | 2483 | 130 | 2483 | 5 | 1695 |
| Dock | 1 | 2149 | 109 | 2149 | 4 | 1780 |
| agent elements + bridge door | 5 | 2086 | 100 | 1989 | 3 | 941 |
| IconRenderHost export tree | 7 | 1973 | 84 | 1973 | 5 | 489 |
| probes (scale-bench, native-socket-probe, sqlite) | 3 | 1377 | 86 | 126 | 1 | 152 |
| ProgramBridge | 1 | 1309 | 119 | 1164 | 5 | 299 |
| Canvas2dHost (gumball + paint) | 2 | 1229 | 53 | 1229 | 2 | 279 |
| renderer root `R/🦀️.rs` (included crate root) | 1 | 921 | 49 | 910 | 8 | 3602 |
| socket/directory doors (+ runtime-assets) | 3 | 686 | 47 | 947 | 3 | 457 |
| native-temporal | 4 | 470 | 26 | 0 | 1 | 286 |
| media-slots | 4 | 372 | 62 | 372 | 4 | 281 |
| outside E/ (mesh, theme geometry) | 2 | 276 | 13 | 276 | 1 | 57 |
| element-contracts | 1 | 135 | 7 | 135 | 0 | 0 |
| catalog/registry | 2 | 36 | 3 | 36 | 0 | 0 |
| lib root | 1 | 23 | 0 | 20 | 2 | 166 |
| **total** | 138 | 101831 | 5778 | 91304 | 147 | 53176 |


The per-file table (138 library files with lines, bytes, cfg gate, attached test lines and target crate) is Appendix A.

### 2.3 Cross-cutting facts that shape the plan

* **Imports through globs.** Every child begins `use super::*;`, so each file silently consumes the root's 123 imports (`infinite_world::world::{…}` 100+ names, `ui_wgpu::wgpu::{…}` 60+ names, `std`, `winit`, `semio_framework_job`, `program_bridge::…`, `shell::…`). Each new crate lists its own imports. The token-level usage per crate (names from the old root block) is in section 6.3.
* **Re-export globs.** Parents `pub use child::*` / `pub(crate) use child::*`. Item-level analysis ignores re-exports entirely; paths such as `crate::scenes::X` become `semio_framework_os_renderer_wgpu_scene_state::X` etc. (rename table in section 6.4).
* **Embedded assets left in the lib closure** (**[WP0 change]** 261 now: the 11 below plus the 250 raw icon SVGs of `🖼️assets/🔣️icons`, embedded through the generated `🧩️icon-embeds/🦀️.rs`; the 11 were): `🖼️assets/🥽️mesh` catalogue + representation catalogue JSON, the Anta outline font (`include_bytes!`, 26,772 B, `R/🧠️app-runtime`), styling JSON (31,584 B), theme geometry JSON ×2, plugin shard configuration JSON, store creation-polling JSON, replay-refusal vocabulary JSON, sqlite DDL, the engine-root logo SVG (682 B). All are small; they travel with their owning crate. The Anta font is the one asset the brief's rule 2 would normally move to a runtime channel: leave it (26 KB) unless `app-runtime` overruns its byte budget (it is the tightest crate, section 6).
* **Files mounted from outside `E/`**: `🖼️assets/🥽️mesh`, `🖱️ui/🎨️styling/🌓️theme/📐️geometry` (+ its two JSON fixtures), `🖱️ui/🧪️testing/🎨️paint-policy`, `🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/📥️enqueue/🧪️tests`. Three of the four have exactly one mounting crate today (verified with `git grep`) and become members of exactly one new crate (mesh-assets → `host-base`, theme geometry → `shell-prefs`, enqueue tests → `runtime-mailbox`); `paint-policy` is a test-only file that three crates already mount (this one, the `ui` wgpu target and the theme component): it stays owned by `🖱️ui`, and `entrypoint` keeps its `cfg(test)` mount. Keep the `#[path]` form; do not copy.
* **wasm exports** (47): 19 free functions in 7 files and the `BrowserRendererWorker` / `BrowserRendererBootstrap` classes (28 `#[wasm_bindgen]` attributes) in `T/🌐️browser-worker`. Table in section 8.
* **Statics and thread-locals:** 107 (66 `static` + 32 `thread_local!` blocks + OnceLocks), of which 29 are referenced from more than one future crate; they must live in the lowest crate that names them (section 6.5).
* **Macros:** one `macro_rules!` (`R/📇️registry`, renderer registration). `#[global_allocator]` appears once (lib root, `cfg(test)`), and only one may exist per test binary: keep it in the top crate and in each companion test crate that wants the heap witness.
* **Peer-visible externals:** only `run_native_entrypoint` (used by `✏️s/🧑‍💻dev/💡️services/🦀️.rs` and `T/⌨️native-entrypoint/💾️binary/🦀️.rs`) and the wasm component crate `T/🧩️component/🦀️.rs` (`extern crate semio_framework_os_renderer_wgpu as _;`).

## 3. Method (what the numbers mean)

1. **Closure**: `readRustSourceClosureV1` from the repo's own counter on `PKG/📚️library/🦀️.rs` with the host cfg, with `wasm32-unknown-unknown`, `wasm32-wasip2`, each also with `test`; missing fixture files stubbed as empty (29 native, 23 wasm); results in section 1.1.
2. **Module tree**: regex mount parser over `#[path]`, plain `mod`, and `include!` with the cfg attributes of each mount (285 nodes, 138 library files, 147 test files).
3. **Item graph**: every column-0 item of the 138 library files (fn, struct, enum, trait, type, const, static, `thread_local!`, `macro_rules!`, impl blocks; 4,585 items, 99,446 of the 101,831 lines) gets the set of top-level names it mentions. A name counts as a reference to an item only if that name is defined by exactly one crate-visible item (multi-defined names resolve to the lowest-ranked definition), is not a generic word (an ignore list of 40 words such as `paint`, `begin`, `label`, `reject`, `publish`), is not preceded by `.` and, for functions, is called, path-qualified or imported. Inherent impl blocks are bound to their type. Inherent method calls (`x.m()`) are not tracked by this graph; they are tracked separately for `ShellState` (section 7).
4. **Partition**: crates are file lists plus **item moves** (names or line ranges). Crate-level edges are the item references that cross a crate boundary; a **back edge** is a reference to a higher-ranked crate. The reported "down" cost is the line count of the referenced items and of everything they drag down (transitive closure into higher-ranked crates), "up" is the line count of the referencing items plus everything in the crate that depends on them. The cheaper side wins; if both are large the edge is inverted (section 5.3).
5. **Tests**: each test file is placed in the highest-ranked crate that supplies at least 60 lines of the names it mentions (or 15 percent of them) and its own host crate, then packed (section 9).
6. **Limits of this evidence** (carry into verification): references through method syntax, trait dispatch, macros and `Deref` are invisible; names in generic position that equal a generic word were ignored; the all-cfg item totals (104.9k lines) exceed the per-cfg closure (101.8k) because cfg-excluded alternatives are counted once per item. Treat every size as plus or minus 8 percent and re-measure per crate with the census before and after each carve.

## 4. Layers and the crate DAG

### 4.1 Rules the partition obeys

1. **Lines and bytes decide the cut, cohesion decides the shape.** A crate is a domain (one element family, one kernel subsystem, one shell component), never a size bucket; a domain larger than the budget is split along its item-level DAG, never by file count.
2. **Dependencies point down only.** The rank order of section 4.3 is the topological order of the crate DAG. A reference to a higher-ranked crate is a defect and appears in section 5.
3. **Contracts are crates of types and cells, not of behaviour.** `contracts` holds what two or more elements must both name and cannot own: the admitted-surface map and tokens, `Viewport`, `WorkerCell`, `ScenePointerTarget`, `RasterUploadRefusal`, `FrameworkWidgetContext`, `EngineSurfaceToken`, `ProgramFault`, the shell find-item vocabulary, `ShellHubRemoteV1` and the retained-UI cell (`UI_ENGINE`, close queue, scene pointer owners).
4. **Statics live in the lowest crate that names them** (section 6.5), so a crate boundary never creates a second instance.
5. **A type and every inherent impl of it share a crate.** `OsHost` (impl split over `🏠️os-host` 129 lines and `🪟️winit-app` 216) and `AppRuntime` (impl split over `🧾️frame-build` 100 lines and `🧠️app-runtime` 596) therefore each end up in one crate.
6. **Inversion only where lowering fails**: three generic or callback seams outside the Shell (section 5.3), ten ports inside it (section 7.5). Never an extension trait.
7. **Native-only code stays native-only by crate, not by `cfg` soup**: `native-platform`, `kernel-*`, `native-probe`, `entrypoint` are whole-crate gated by target-specific dependencies in their consumers; `browser-worker` is wasm-only; `mailbox-core` builds everywhere including wasip2 with no dependencies.

### 4.2 Layer sketch (low to high)

```
L0  contracts  host-base  native-platform  media-slots  canvas2d-paint  hub-contracts  mailbox-core
L1  asset-authority  doors  kernel-protocol  dock  hub-workspace*  agent-elements*
L2  asset-decode  kernel-retained  engine-registry
L3  kernel-pool  kernel-runtime  engine-text  engine-board  program-bridge
L4  engine-graph  scene-state
L5  engine-attach  canvas2d-host  scene-canvas2d  icon-host
L6  scene-ink  scene-feeds
L7  scene-tables  scene-text-vfs
L8  scene-render
L9  ui-engine-focus  ->  ui-engine-dump  ->  ui-engine
L10 SHELL TIER 1: shell-vocab shell-async shell-chrome-state shell-surfaces shell-plugins shell-history shell-prefs
                  shell-directory shell-sync shell-docs shell-doc-open shell-windows shell-dock shell-input shell-commands
L11 SHELL TIER 2: shell-state (composer) -> shell-session -> shell-window-actions -> shell-actions -> shell-pointer
L12 SHELL TIER 3: shell-view-paint -> shell-view-chrome -> shell-view-overlays -> shell-view-panels
L13 frame-actions  host-plumbing
L14 runtime-mailbox (generic over the runtime state)
L15 app-runtime  ->  frame-presentation  ->  frame-transaction  ->  frame-job
L16 native-probe  host-winit  browser-worker
L17 entrypoint (top crate: wasm export wrappers, run_native_entrypoint, build.rs)
(* depends only on contracts, media-slots, doors; placed with its element family for work-package locality)
```

The authoritative edge list (direct first-party dependencies per crate) is the `deps` column of section 6.1; the machine-computed DAG is reproducible with the tool of section 12.

### 4.3 Rank order (the topological order used for back-edge accounting)

`contracts, host-base, native-platform, media-slots, canvas2d-paint, asset-authority, asset-decode, doors, kernel-protocol, kernel-retained, kernel-pool, kernel-runtime, engine-registry, engine-text, engine-board, engine-graph, engine-attach, scene-state, canvas2d-host, scene-canvas2d, scene-ink, scene-feeds, scene-tables, scene-text-vfs, icon-host, scene-render, ui-engine-focus, ui-engine-dump, ui-engine, hub-contracts, hub-workspace, agent-elements, dock, program-bridge, [Shell tier 1..3 in the order of section 7.3], mailbox-core, host-plumbing, frame-actions, runtime-mailbox, app-runtime, frame-presentation, frame-transaction, frame-job, native-probe, host-winit, browser-worker, entrypoint`.

## 5. Cycle cuts

### 5.1 What the cycles are

* File level: one SCC of **101 files** (every element, the renderer partitions, the Shell children, the host files). Seams: `Interpreter` ⇄ `Scenes` (6 names), `Scenes` ⇄ `EngineCanvas` (the `puzzle_board_*` forwarders and `Viewport`), `scene-state` ⇄ `os-host` (component surface close), the kernel quartet (`KernelClient`, `KernelRequestQueue`, `ResponseSlot`, `MountedProductReplayRequest`), and the frame cluster.
* Item level: **one genuine knot in the Shell** (988 lines: `ShellState`, `IconExportBatch`, `UiPrefsSnapshot`, `ShellChromePresentState`: the three helpers take `&ShellState`) and **one in the frame core** (742 lines of types, about 3,000 with their impls: `AppRuntime`, `AppHandle`, `RuntimeMailbox(Inner)`, `RuntimeApply`, `NativeReferenceDecodeJob`, `RuntimeCompletion*`). Everything else is acyclic once items move.

### 5.2 Item moves (apply before or while carving; destination crate is the one to place the item in; counts are active lines)

Executors extract an item with its doc comment, attributes and region markers intact. A moved item keeps its name; the old path disappears (no aliases). Items are named, not line-numbered, because peers edit these files; use the outline listing of section 12 to find them.

| destination crate | from file | default crate of that file | lines | what |
|---|---|---|---:|---|
| `contracts` | `🗃️scene-state` | `scene-state` | 307 | SCENE_SURFACE_CAPACITY, SCENE_SURFACE_ID_BYTE_CAPACITY, AdmittedSurfaceToken, AdmittedSurfaceFault, AdmittedSurfaceRejected, AdmittedSurfaceCloseOwner, AdmittedSurfaceEntry, AdmittedSurfaceMap, impl AdmittedSurfaceMap, impl Adm… |
| `contracts` | `🔖️identity-and-types` | `shell-*` | 135 | ShellFindItem, SHELL_FIND_ITEM_CAPACITY, SHELL_FIND_PAYLOAD_BYTES, ShellFindItems, impl ShellFindItems, impl ShellFindItems, ActiveShellFindItems, ACTIVE_FIND_ITEMS, ShellFindItemBinding, impl ShellFindItems, impl ShellFindItem… |
| `contracts` | `🧩️retained-ui-engine` | `ui-engine` | 117 | UI_ENGINE, UI_ENGINE_LOCALE_UNRESOLVED, TEST_UI_ENGINE_LOCALE, UiEngineCell, UiEngineGuard, impl UiEngineGuard, impl UiEngineGuard, initial_ui_engine, impl UiEngineCell, install_ui_engine_locale, UiDocumentCloseOwner, UiDocumen… |
| `contracts` | `🌉️ProgramBridge` | `program-bridge` | 41 | ProgramFault, impl ProgramFault, impl ProgramFault, impl ProgramFault, impl ProgramFault, impl ProgramFault |
| `contracts` | `🖼️ui-image-loading` | `scene-canvas2d` | 19 | framework_widget_context |
| `contracts` | `🧱️chrome-paint-and-navbar` | `shell-*` | 14 | ShellHubRemoteV1 |
| `contracts` | `🗂️engine-surface-registry` | `engine-registry` | 5 | EngineSurfaceToken |
| `host-base` | `🔁️asset-decode-jobs` | `asset-decode` | 356 | renderer_worker_pool, renderer_worker_pool, RendererIoHandle, impl RendererIoHandle, impl RendererIoHandle, impl RendererIoHandle, RENDERER_IO_SESSION_SLOTS, RENDERER_IO_VACANT, RENDERER_IO_LIVE, RENDERER_IO_CHECKED_OUT, RENDER… |
| `host-base` | `🎨️theme-file-import-and-accessibility` | `shell-*` | 89 | HOST_IO_REQUEST_TRACE, HostIoRequestTraceGuard, impl HostIoRequestTraceGuard, impl HostIoRequestTraceGuard, host_io_request_trace_label, host_io_request_trace_label, host_io_call |
| `host-base` | `🏊️kernel-pool` | `kernel-pool` | 48 | KernelPoolFuture, impl KernelPoolFuture, impl KernelPoolFuture |
| `host-base` | `🌐️browser-worker` | `browser-worker` | 8 | declare_boot_subphase |
| `host-base` | `🎨️presentation-raster` | `runtime-mailbox` | 4 | RuntimeHostWaker, RuntimeHostWaker |
| `native-platform` | `🎞️Scenes` | `scene-state` | 16 | NativeHostTemporalFormatter, NATIVE_HOST_TEMPORAL_FORMATTER, install_native_host_temporal_formatter, native_host_temporal_formatter |
| `asset-authority` | `🔁️asset-decode-jobs` | `asset-decode` | 22 | renderer_asset_svg_prefix_is_valid, renderer_asset_ui_image_prefix_is_valid, renderer_asset_reference_prefix_is_valid |
| `kernel-protocol` | `📬️kernel-replay-client` | `kernel-pool` | 1089 | PRODUCT_REPLAY_KIND_BYTES, PRODUCT_REPLAY_INPUT_PAGE_CAPACITY, PRODUCT_REPLAY_INPUT_BYTES, PRODUCT_REPLAY_PROFILE_COUNT, RawSpawnJobDisposition, RawSpawnJobClosePhase, RawSpawnJobOwner, impl RawSpawnJobOwner, MountedProductRepl… |
| `kernel-protocol` | `🏊️kernel-pool` | `kernel-pool` | 247 | KERNEL_REQUEST_QUEUE_CAPACITY, KernelRequestQueue, KernelRequestQueueState, impl KernelRequestQueue, impl KernelRequestQueue |
| `kernel-protocol` | `📦️kernel-runtime` | `kernel-runtime` | 31 | read_native_component |
| `kernel-pool` | `🗃️kernel-retained-state` | `kernel-retained` | 54 | KernelPoolState |
| `engine-registry` | `✍️text-editor` | `engine-text` | 218 | TEXT_EDITOR_PENDING_ECHO_CAPACITY, TextEditorDeliverySnapshot, TextEditorTypingRun, TextEditorTypingEnd, impl TextEditorTypingEnd, TextEditorDeliveryInFlight, TextEditorDraftPublication, TextEditorDeliveryPublication, TextEdito… |
| `engine-registry` | `🕸️node-graph` | `engine-graph` | 26 | engine_canvas_fault_log, engine_canvas_debug_log, PublishedGraphInteraction, published_graph_interaction |
| `engine-registry` | `🧩️engine-surface-attach` | `engine-attach` | 9 | engine_camera_from_json |
| `scene-state` | `📐️Canvas2dHost` | `canvas2d-host` | 203 | HANDLE_LENGTH, ROTATE_RADIUS, HIT_RADIUS, RING_HIT_BAND, MOVE_EPSILON, TURN_EPSILON, GumballConfig, ModelToLayer, IDENTITY_MAP, GumballMeta, MetaLayer, GumballView, impl GumballView, GumballHandle, impl GumballHandle, GumballMo… |
| `scene-state` | `🖌️canvas2d` | `scene-canvas2d` | 125 | CanvasFillJson, CanvasGradientStopJson, CanvasStrokeJson, CanvasImageFieldJson, CanvasTextFieldJson, CanvasLayer |
| `scene-state` | `⌨️scene-input` | `scene-render` | 88 | MAP_MARQUEE_THRESHOLD_PX, canvas_active_utility, request_canvas_pointer_gesture_cancel_for_host, write_scene_action_batch, BLOCK_LIST_PALETTE_DRAG_MIME, SceneListTransferStart, SceneListTransferRelease, SceneModifiers, impl Sce… |
| `scene-state` | `🏠️os-host` | `host-winit` | 69 | ComponentSurfaceCloseToken, ComponentSurfaceCloseRequest, ComponentSurfaceCloseBridgeState, ComponentSurfaceCloseBridge, impl ComponentSurfaceCloseBridge, COMPONENT_SURFACE_CLOSE_BRIDGE, COMPONENT_SURFACE_CLOSE_BRIDGE, with_com… |
| `scene-state` | `🎨️scene-render-entry` | `scene-render` | 57 | render_placeholder, VirtualFileSystemChromeLabels, BlockListChromeLabels, IconRenderChromeLabels, SceneChromeLabels, impl SceneChromeLabels, SceneEngineHosts |
| `scene-state` | `🧮️table-and-paint2d` | `scene-tables` | 43 | TableStepperFocusTarget, TableEditableTextFocusTarget, impl TableEditableTextFocusTarget, TableStepperAccessibilityCell, TableEditableTextAccessibilityCell, TableButtonAccessibilityCell |
| `scene-state` | `🧭️node-graph-and-tiled-map` | `scene-feeds` | 26 | retire_tiled_map_scene_identity |
| `scene-state` | `✍️text-editor` | `scene-text-vfs` | 17 | TextEditorUiState, TextEditorRenameDraft |
| `scene-state` | `✒️ink-canvas-model-state` | `scene-ink` | 16 | InkBoundsF, impl InkBoundsF |
| `scene-state` | `🎯️chrome-ledger` | `ui-engine-dump` | 15 | WORLD_CAMERA_LEDGER, note_world3d_live_camera, take_world_camera_diagnostic |
| `scene-state` | `🖋️ink-canvas-render` | `scene-ink` | 6 | draw_ink_rect_outline |
| `scene-state` | `🧩️board2d-and-vfs` | `scene-text-vfs` | 5 | VfsAccessibilityFocusTarget |
| `ui-engine-focus` | `🧩️retained-ui-engine` | `ui-engine` | 288 | SCENE_INTENT_CAPACITY, SCENE_INTENT_ID_BYTES, SceneIntentEvent, SceneInteractionIntent, SceneIntentQueue, impl SceneIntentQueue, impl SceneIntentQueue, impl SceneInteractionIntent, SCENE_INTENTS, apply_presented_scene_caret, wr… |
| `host-plumbing` | `🪟️winit-app` | `host-winit` | 37 | pointer_button_to_i16, event_modifiers_to_pointer, key_action_from_dispatch |
| `frame-actions` | `🧵️frame-job` | `frame-job` | 11 | FrameBuildInputs, FrameDirectives |
| `frame-actions` | `🎨️presentation-raster` | `runtime-mailbox` | 1 | WORLD3D_DEADLINE_CAPACITY |
| `runtime-mailbox` | `🛎️frame-actions` | `frame-actions` | 370 | FrameMaintenanceOwner, impl FrameMaintenanceOwner, FrameDeferredExecutionOwner, impl FrameDeferredExecutionOwner, FrameMaintenanceRefusal, impl FrameMaintenanceRefusal, RuntimeApply, impl RuntimeApply |
| `runtime-mailbox` | `🧠️app-runtime` | `app-runtime` | 134 | native_renderer_asset_path, stream_native_renderer_asset, push_renderer_asset_page, stream_native_renderer_http_asset, world_rejection_fault |
| `runtime-mailbox` | `🧾️frame-build` | `app-runtime` | 2 | AppHandle |
| `app-runtime` | `🖥️frame-presentation` | `frame-presentation` | 262 | FramePreparationStep, AppFramePreparation, impl AppFramePreparation, impl AppFramePresentation |
| `app-runtime` | `🪟️winit-app` | `host-winit` | 137 | dispatch_normalized_event |
| `app-runtime` | `🔬️native-socket-probe` | `native-probe` | 3 | ICON_ATLAS_RUNTIME |
| `frame-transaction` | `🧾️frame-build` | `app-runtime` | 781 | FrameTransaction, AppFrameTransactionStep, FrameTransactionStage, impl FrameTransactionStage, AppFrameTransactionPhase, impl FrameTransaction |
| `frame-transaction` | `🧠️app-runtime` | `app-runtime` | 134 | boot_runtime |
| | | **total** | 5685 | 45 groups |


Additional moves implied by whole-file membership (not repeated above): `IconRenderHost/📤️export/📥️asset` (138 lines) goes to `asset-authority` because `close_renderer_asset_step` calls `scenes::icon_export::asset::close_all_step()` (the one low-to-high edge that was not a name collision); the 123-name root `use` block is dissolved into the crates' own imports; `RENDERER_SOURCE`, `SHELL_SOURCE`, `SCENES_SOURCE`, `ENGINE_CANVAS_SOURCE`, `INTERPRETER_SOURCE` are deleted (section 9).

Name collisions in the graph that are **not** dependencies (verified by reading the definitions; do not cut them): `label` (EngineCanvas node-graph vs shell labels), `paint`/`begin` (Canvas2dHost gumball vs unrelated locals), `reject`, `publish`, `cancel`, `release`, `CAPACITY`, `scalar`, `visible`, `row`, `run`, `rects_overlap` (two private copies), `WorkerCell` (contracts vs the private Scenes cell, which is renamed `SceneCell` and stays in `scene-state`).

### 5.3 Inversions (the three residual back edges; none is inside an element family)

| # | edge | evidence | prescribed cut |
|---|---|---|---|
| I1 | `kernel-protocol` → `kernel-pool` (`KernelClient`) | `JobProgressPresentationLease::drop` calls `KernelClient::get().try_acknowledge_job_progress(token)` (`🧮️kernel-core` line 227) | `JobProgressPresentationBridge::new(ack: fn(JobProgressPresentationToken) -> bool)`; `kernel-pool` installs the function when the client starts. `kernel-protocol` stops naming the client. |
| I2 | `runtime-mailbox` → `app-runtime` (`AppRuntime`, `AppInteractionState`, `dispatch_normalized_event`, + `AppHandle`) | `RuntimeMailbox::new(runtime: AppRuntime)`, `try_lock() -> MutexGuard<AppRuntime>`, `update_frame_inputs(&AppRuntime)`, `spawn_dispatch_reserved(cursor, interaction: AppInteractionState)`, `runtime.interaction.as_ref().map(AppInteractionState::has_pending_text_work)`, and `winit_app::dispatch_normalized_event(interaction, event)` at `🎨️presentation-raster:754` | `RuntimeMailbox<R: MailboxRuntime>` / `RuntimeMailboxInner<R>` / `AppHandle<R>`. `trait MailboxRuntime { type Interaction: InteractionDispatch; fn interaction_available(&self)->bool; fn check_out_interaction(&mut self,..); fn return_interaction(&mut self,..); fn submit_interaction(&mut self,..); fn resize(&mut self,..); fn has_pending_text_work(i:&Self::Interaction)->bool; }` and `trait InteractionDispatch { fn dispatch(&mut self, ev:&mut DispatchEvent)->impl Future<Output=()>; }`, both defined in `runtime-mailbox`, implemented for `AppRuntime` / `AppInteractionState` in `app-runtime`. These are the observed call sites (frame-actions: 19 `return_interaction`, 2 `check_out_interaction`, 1 `submit_interaction`, 1 `resize`, 1 `interaction_available`; mailbox: 28 `interaction.as_mut`, 7 `as_ref`, 3 `interaction_available`, 1 `try_lock`, 1 `open_scope_now`): the trait is exactly this set. `RuntimeApply` and its three owners (370 lines, today in `🛎️frame-actions`) move into `runtime-mailbox` (they need `RuntimeMailbox` and the runtime trait, and the mailbox needs them). The five native asset-stream functions (`stream_native_renderer_asset` & co, 160 lines) move from `🧠️app-runtime` down into `runtime-mailbox` (they are mailbox transport). |
| I3 | `runtime-mailbox` → `frame-presentation` (`AppPresenter`) | one method, `RuntimeMailbox::close_world3d_dynamic_step(&self, presenter:&mut AppPresenter, cx)` (`📮️mailbox-impl:137`) | move the method to `impl AppPresenter` (`frame-presentation`), taking `&RuntimeMailbox<AppRuntime>`. |

Four ownership fixes that are moves, not inversions (a type and every inherent impl of it must share a crate, E0116): (1) `struct KernelPoolState` (54 lines, 30 fields) is defined in `🗃️kernel-retained-state` but its 1,898-line impl is in `🏊️kernel-pool`: the struct moves to `kernel-pool`; (2) `AppFramePreparation`, `FramePreparationStep` and `AppFramePresentation` (types plus their impls, about 100 lines, today in `🖥️frame-presentation` / `🧾️frame-build`) move down into `app-runtime` because the build cursors hold them; (3) `impl AppFrameBuild` (44 lines) stays with its struct in `app-runtime`; (4) `FrameTransaction` and `boot_runtime`, the only users of `AppPresenter`, form the new `frame-transaction` crate above `frame-presentation`. Everything in the frame cluster not named above sorts by itself. Resulting crates (sizes in section 6.1): `frame-actions` (carriers generic in the interaction type, no `AppRuntime`), `runtime-mailbox` (+ `RuntimeApply`, asset stream), `app-runtime` (`AppRuntime` 696 lines in one impl, `AppInteractionState` 481, build cursors, `dispatch_normalized_event` + its three tiny helpers), `frame-presentation` (`AppPresenter` 815), `frame-transaction` (`FrameTransaction` 706, `boot_runtime`), `frame-job`.

### 5.4 Residual risk list for section 5

* Method syntax is not in the graph: after each move run the differential oracle (section 11) before trusting a cut.
* `scene-state` is the busiest landing site (670 lines of moves in, 16 source files touched). Its executor must keep `SCENE_STATE`, `CANVAS_GESTURE`, `CANVAS_CATALOGUE_HOVER`, `PENDING_RASTER_STATE`, `SCENE_LIST_TRANSFER` and `GUMBALL` as single statics.
* `contracts` receives the retained-UI cell. `UiEngineCell`, `UiEngineGuard`, `UI_ENGINE` and the close queue must keep their exact layout (they carry the reentrancy law the `🧵️worker-cell` test pins); move them as a unit.

## 6. The non-Shell crates

### 6.1 Catalogue (sizes are the all-cfg item totals plus a proportional share of each file's header lines; lib closure = lib lines; test closure = lib + unit tests; "unit tests" are the tests that stay in the crate, section 9)

"Direct deps" lists every first-party crate the code names (Cargo needs each declared, not the transitive reduction). `shell-*` means a component or composer crate of section 7; the frame cluster sits above the Shell. The 24 Shell crates have their own table in 7.7.

| # | crate (`semio-framework-os-renderer-wgpu-…`) | WP | lib lines | lib KB | lib src files | unit tests: lines / KB / files | direct first-party deps (non-Shell) | cfg |
|---:|---|---:|---:|---:|---:|---|---|---|
| 1 | `contracts` | 1 | 820 | 35 | 8 | 0 / 0 / 0 |  | native + wasm32 |
| 2 | `host-base` | 1 | 2039 (was 2015) | 114 (was 112) | 11 | 425 / 30 / 6 |  | native + wasm32 |
| 3 | `native-platform` | 1 | 1131 | 61 | 6 | 495 / 35 / 3 |  | native only |
| 4 | `media-slots` | 1 | 417 | 62 | 4 | 254 / 58 / 8 |  | native + wasm32 |
| 5 | `canvas2d-paint` | 1 | 617 | 25 | 1 | 180 / 40 / 2 |  | native + wasm32 |
| 6 | `asset-authority` | 1 | 918 | 39 | 3 | 54 / 2 / 1 | `contracts` | native + wasm32 |
| 7 | `asset-decode` | 1 | 2993 | 155 | 2 | 0 / 0 / 0 | `contracts`, `host-base`, `asset-authority` | native + wasm32 |
| 8 | `doors` | 1 | 794 | 43 | 2 | 218 / 8 / 1 | `host-base` | native + wasm32 |
| 9 | `kernel-protocol` | 1 | 2527 | 120 | 5 | 124 / 10 / 2 | `contracts`, `host-base` | native only |
| 10 | `kernel-retained` | 1 | 879 | 38 | 1 | 0 / 0 / 0 | `kernel-protocol` | native only |
| 11 | `kernel-pool` | 1 | 2234 | 140 | 3 | 0 / 0 / 0 | `contracts`, `host-base`, `kernel-protocol`, `kernel-retained` | native only |
| 12 | `kernel-runtime` | 1 | 913 | 57 | 2 | 1403 / 89 / 3 | `contracts`, `host-base`, `kernel-protocol` | native only |
| 13 | `engine-registry` | 2 | 2442 | 115 | 7 | 0 / 0 / 0 | `contracts`, `asset-authority` | native + wasm32 |
| 14 | `engine-text` | 2 | 1137 | 64 | 1 | 0 / 0 / 0 | `contracts`, `engine-registry` | native + wasm32 |
| 15 | `engine-board` | 2 | 1798 | 94 | 3 | 0 / 0 / 0 | `contracts`, `asset-authority`, `engine-registry` | native + wasm32 |
| 16 | `engine-graph` | 2 | 1297 | 68 | 1 | 601 / 33 / 2 | `contracts`, `host-base`, `media-slots`, `engine-registry`, `engine-board` | native + wasm32 |
| 17 | `engine-attach` | 2 | 1128 | 58 | 1 | 2264 / 231 / 17 | `contracts`, `media-slots`, `canvas2d-paint`, `engine-registry`, `engine-text`, `engine-board`, `engine-graph` | native + wasm32 |
| 18 | `scene-state` | 2 | 2629 | 115 | 16 | 159 / 12 / 3 | `contracts`, `host-base`, `canvas2d-paint`, `engine-board` | native + wasm32 |
| 19 | `canvas2d-host` | 2 | 383 | 19 | 1 | 99 / 39 / 2 | `contracts`, `scene-state` | native + wasm32 |
| 20 | `scene-canvas2d` | 2 | 1377 | 69 | 2 | 0 / 0 / 0 | `contracts`, `media-slots`, `canvas2d-paint`, `asset-authority`, `scene-state` | native + wasm32 |
| 21 | `scene-ink` | 2 | 2393 | 108 | 2 | 1890 / 112 / 7 | `contracts`, `scene-state`, `scene-canvas2d` | native + wasm32 |
| 22 | `scene-feeds` | 2 | 1416 | 68 | 2 | 862 / 52 / 7 | `contracts`, `host-base`, `native-platform`, `engine-board`, `scene-state`, `scene-canvas2d` | native + wasm32 |
| 23 | `scene-tables` | 2 | 1500 | 84 | 2 | 236 / 14 / 3 | `contracts`, `media-slots`, `scene-state`, `scene-feeds` | native + wasm32 |
| 24 | `scene-text-vfs` | 2 | 1281 | 67 | 2 | 719 / 40 / 4 | `contracts`, `engine-text`, `engine-board`, `scene-state`, `scene-canvas2d`, `scene-feeds` | native + wasm32 |
| 25 | `icon-host` | 2 | 2538 (was 2289) | 121 (was 101) | 8 (was 7) + 251 embedded | 675 / 62 / 12 | `contracts`, `host-base`, `media-slots`, `scene-state` | native + wasm32 |
| 26 | `scene-render` | 2 | 1618 | 82 | 2 | 1714 / 115 / 12 | `contracts`, `engine-registry`, `engine-board`, `engine-graph`, `engine-attach`, `scene-state`, `canvas2d-host`, `scene-canvas2d`, `scene-ink`, `scene-feeds`, `scene-tables`, `scene-text-vfs`, `icon-host` | native + wasm32 |
| 27 | `ui-engine-focus` | 3 | 1682 | 95 | 3 | 0 / 0 / 0 | `contracts`, `engine-registry`, `engine-text`, `engine-graph`, `scene-state`, `scene-ink`, `scene-tables`, `scene-text-vfs` | native + wasm32 |
| 28 | `ui-engine-dump` | 3 | 2175 | 121 | 3 | 444 / 29 / 2 | `contracts`, `host-base`, `native-platform`, `media-slots`, `engine-text`, `scene-state`, `scene-feeds`, `ui-engine-focus` | native + wasm32 |
| 29 | `ui-engine` | 3 | 2306 | 132 | 5 | 1717 / 142 / 12 | `contracts`, `host-base`, `engine-text`, `engine-board`, `scene-state`, `scene-canvas2d`, `scene-ink`, `scene-feeds`, `scene-tables`, `scene-text-vfs`, `scene-render`, `ui-engine-focus`, `ui-engine-dump` | native + wasm32 |
| 30 | `hub-contracts` | 1 | 1298 | 63 | 2 | 633 / 154 / 5 |  | native + wasm32 |
| 31 | `hub-workspace` | 1 | 1185 | 67 | 3 | 1062 / 71 / 6 | `contracts`, `media-slots`, `hub-contracts` | native + wasm32 |
| 32 | `agent-elements` | 1 | 2086 | 100 | 5 | 941 / 72 / 8 | `host-base`, `media-slots`, `doors` | native + wasm32 |
| 33 | `dock` | 1 | 2149 | 109 | 1 | 402 / 53 / 6 | `contracts` | native + wasm32 |
| 34 | `program-bridge` | 1 | 1820 | 120 | 2 | 299 / 37 / 10 | `contracts`, `host-base`, `media-slots`, `kernel-protocol`, `kernel-pool` | native + wasm32 |
| 35 | `mailbox-core` | 1 | 214 | 9 | 1 | 264 / 31 / 3 |  | every target incl. wasm32-wasip2, zero dependencies |
| 36 | `host-plumbing` | 3 | 1200 | 71 | 7 | 522 / 96 / 15 | `contracts`, `host-base`, `media-slots` | native + wasm32 |
| 37 | `frame-actions` | 3 | 702 | 33 | 3 | 301 / 36 / 5 | `host-base`, `engine-text`, `ui-engine`, `shell-*` | native + wasm32 |
| 38 | `runtime-mailbox` | 3 | 2931 | 155 | 5 | 2709 / 970 / 21 | `contracts`, `host-base`, `asset-authority`, `asset-decode`, `engine-registry`, `engine-text`, `engine-board`, `scene-canvas2d`, `ui-engine-focus`, `program-bridge`, `mailbox-core`, `host-plumbing`, `frame-actions` | native + wasm32 (the native asset-stream half is `cfg(not(wasm32))`) |
| 39 | `app-runtime` | 3 | 2229 | 161 | 5 | 0 / 0 / 0 | `contracts`, `host-base`, `kernel-protocol`, `engine-registry`, `engine-text`, `engine-board`, `engine-graph`, `scene-feeds`, `scene-text-vfs`, `icon-host`, `ui-engine-focus`, `ui-engine`, `program-bridge`, `mailbox-core`, `host-plumbing`, `frame-actions`, `runtime-mailbox`, `shell-*` | native + wasm32 |
| 40 | `frame-presentation` | 3 | 1139 | 62 | 1 | 110 / 16 / 2 | `contracts`, `host-base`, `media-slots`, `engine-registry`, `engine-attach`, `ui-engine-dump`, `ui-engine`, `host-plumbing`, `runtime-mailbox`, `app-runtime` | native + wasm32 |
| 41 | `frame-transaction` | 3 | 945 | 55 | 2 | 0 / 0 / 0 | `host-base`, `engine-registry`, `engine-text`, `engine-board`, `scene-state`, `scene-canvas2d`, `icon-host`, `ui-engine-focus`, `ui-engine`, `program-bridge`, `mailbox-core`, `frame-actions`, `runtime-mailbox`, `app-runtime`, `frame-presentation`, `shell-*` | native + wasm32 |
| 42 | `frame-job` | 3 | 925 | 65 | 2 | 449 / 46 / 7 | `host-base`, `frame-actions`, `runtime-mailbox`, `app-runtime`, `frame-transaction` | native + wasm32 |
| 43 | `native-probe` | 3 | 478 | 28 | 2 | 464 / 81 / 13 | `host-base`, `native-platform`, `canvas2d-host`, `program-bridge`, `host-plumbing`, `app-runtime`, `shell-*` | mostly native (125 of 477 lines also build for wasm32) |
| 44 | `host-winit` | 3 | 1827 | 100 | 2 | 1115 / 87 / 10 | `contracts`, `host-base`, `native-platform`, `kernel-protocol`, `kernel-pool`, `engine-registry`, `scene-state`, `host-plumbing`, `runtime-mailbox`, `frame-presentation`, `frame-transaction`, `frame-job`, `shell-*` | native + wasm32 |
| 45 | `browser-worker` | 3 | 879 | 57 | 1 | 0 / 0 / 0 | `contracts`, `host-base`, `asset-authority`, `engine-registry`, `icon-host`, `program-bridge`, `mailbox-core`, `host-plumbing`, `frame-actions`, `runtime-mailbox`, `app-runtime`, `frame-presentation`, `frame-job`, `host-winit`, `shell-*` | wasm32-unknown-unknown only |
| 46 | `entrypoint` | 3 | 186 | 8 | 2 | 166 / 7 / 2 | `host-base`, `canvas2d-host`, `native-probe` | native (+ wasm export wrappers) |


**[WP0 change]** `icon-host` satisfies lines and bytes (2,538 lines, 121 KB) but not files: 8 sources + 251 embedded files = 259 against 28 (hard ceiling 32), see 6.2 and risk 7. Every other row keeps the planner's value; the measured per-file drift is below 0.3 percent (Appendix A).

Every row satisfies the budget with its unit tests except those flagged in 9.3 (`engine-attach`, `scene-ink`, `ui-engine`, `runtime-mailbox` go to companions) and the tight ones (`scene-render` 3,332 / 198 KB, `hub-contracts` 218 KB, `agent-elements` 3,027 lines, `engine-attach` bytes, `app-runtime` 142 KB of lib bytes).

### 6.2 Per-crate specifications

Each entry: members (source file and the lines it contributes; `T/` = `E/🎯️targets/🧊️wgpu/`, `R` = `T/🧊️renderer`, `FW/` = `🧰️framework/🔨️modules/`, element files are written `<element>/<child>` for `E/🧱️elements/<element>/🎯️targets/🧊️wgpu/<child>`), direct crate dependencies, external crates named in the code (the Cargo dependency list to start from; keep the feature sets of the old `Cargo.toml`: `ui_wgpu` with `wgpu-engine`, `semio-framework-os-kernel` with `sync`+`ureq` on native and `sync` on wasm32, `testkit`/`mutation-testing` for dev), cfg, and the cuts and shared items specific to the crate.

#### `contracts` (WP1, 820 lines, 35 KB)

* members (source file, lines contributed): `🎞️Scenes/🗃️scene-state` 310, `T/🧬️element-contracts` 139, `🐚️Shell/🔖️identity-and-types` 135, `🗣️Interpreter/🧩️retained-ui-engine` 117, `🌉️ProgramBridge` 41, `🗣️Interpreter/🖼️ui-image-loading` 19, `🐚️Shell/🧱️chrome-paint-and-navbar` 14, `⚙️EngineCanvas/🗂️engine-surface-registry` 5
* direct crate deps: none
* external crates named in the code: `ui_wgpu`, `semio_framework_actor`, `semio_framework`, `serde_json`, `semio_framework_ui_locale`, `ui_contract`, `semio_framework_plugin_host`, `semio_framework_async`, `semio_framework_os_kernel`, `ui_render`
* cfg: native + wasm32
* The cycle breaker. Members: `element_contracts` (`WorkerCell`, `ScenePointerTarget`, `RasterUploadRefusal`, `FrameworkWidgetContext`), the admitted-surface family and `Viewport` out of `scene-state` (`AdmittedSurface{Token,Fault,Rejected,CloseOwner,Entry,Map}`, 223-line impl), the shell find-item vocabulary out of `identity-and-types`, `ProgramFault`, `ShellHubRemoteV1`, `EngineSurfaceToken`, `framework_widget_context`, and the retained-UI cell out of `retained-ui-engine` (`UiEngineCell`, `UiEngineGuard`, `UI_ENGINE`, `UiDocumentCloseQueue` + `ui_document_close_pending(_for)`, `ScenePointerOwner(s)`, `component_scene_is_presented`, `merge_action_args`). Statics: `UI_ENGINE`, `UI_DOCUMENT_CLOSE_QUEUE`, `SCENE_POINTER_OWNERS`, `ACTIVE_FIND_ITEMS`. Tests: `admitted-surface-map` (159), the `🧵️worker-cell` law (68) stay as unit tests. Rule: types, constants, statics and pure accessors only. `request_ui_document_close`, `close_ui_document_one` (the closers) stay up in `ui-engine`.

#### `host-base` (WP1, 2039 lines, 114 KB; was 2015, 112) **[WP0 change]**

* members (source file, lines contributed): `R` 699, `R/🔁️asset-decode-jobs` 359, `T/⏱️frame-latency` 332, `FW/🖼️assets/🥽️mesh` 104, `🐚️Shell/🎨️theme-file-import-and-accessibility` 89, `R/📦️kernel-runtime/🏊️kernel-pool` 48, `T/🔤️runtime-assets` 41, `T/📇️catalog` 20, `T/🌐️browser-worker` 8, `R/📇️registry` 7, `R/🎨️presentation-raster` 4
* direct crate deps: none
* external crates named in the code: `semio_framework_async`, `semio_framework_job_worker`, `serde_json`, `semio_framework`, `wasm_bindgen`, `serde`, `js_sys`, `semio_framework_os_services`, `semio_framework_job`, `semio_framework_trace`, `semio_framework_plugin_host`, `ui_wgpu`, `web_sys`, `semio_framework_ui_locale`, `semio_framework_canvas` **[WP0 change]** (`T/🔤️runtime-assets` installs the Canvas runtime assets)
* cfg: native + wasm32
* The dissolved crate root plus four leaf libraries. (a) host statics: `HostAppearance`, `HOST_PLATFORM(_USES_META)`, `HOST_LOCALE`, `HOST_INTRODUCTION_SUPPRESSED`, `resolve_theme`, `appearance_is_dark`, `prefers_dark_scheme`; (b) boot descriptor: `WgpuBoot*`, `BOOT_DESCRIPTOR`, `boot_*` readers, `resolve_environment_boot_descriptor`; (c) `log_debug*`, `app_now_ms`, `actor_budget_from_turn_budget`, `host_io_call` + trace guard, `declare_boot_subphase`, plugin catalog (`PluginHostConfig`, `install_plugin_catalog*`), `runtime-assets`, the registration macro; (d) `mesh_assets` (+ `CATALOG`), `frame_latency`, and `renderer-io` (`renderer_worker_pool`, `RendererIoHandle`/`Slot`/`Node`, the I/O slot statics and pump, `RuntimeHostWaker`, `KernelPoolFuture`, `drive_renderer_worker_task`, `spawn_app_task`; 480 lines, 289 of them also on wasm). The seven wasm setters stay here as plain `pub fn`; their `#[wasm_bindgen]` wrappers are in the top crate. Statics shared outward: `RENDERER_IO_SLOTS`, `RENDERER_IO_WAKE`, `CATALOG`.

#### `native-platform` (WP1, 1131 lines, 61 KB)

* members (source file, lines contributed): `T/♿️native-accessibility` 353, `T/🕰️native-temporal/🍎️macos` 245, `T/🕰️native-temporal` 236, `T/🕰️native-temporal/🪟️windows` 115, `T/🕰️native-temporal/🐧️unix` 66, `🎞️Scenes` 16
* direct crate deps: none
* external crates named in the code: `accesskit`, `ui_render`, `ui_contract`, `serde_json`, `winit`, `accesskit_winit`
* cfg: native only
* `native-accessibility` (AccessKit bridge, `NativeAccessibility*`, 395 lines) + `native-temporal` (platform adapters for macOS/unix/windows, ICU compile test, 736) and the `NativeHostTemporalFormatter` trait with its `OnceLock` installer (moved down out of the Scenes parent so `scene-state`/`scene-feeds` depend on this crate, not the reverse; confirm the trait is `cfg(not(wasm32))` and keep `scene-state`'s use of it gated). Whole crate native-only; target dependencies (`accesskit`, `accesskit_winit`, `winit`, platform FFI) move with it.

#### `media-slots` (WP1, 417 lines, 62 KB)

* members (source file, lines contributed): `T/🎬️media-slots` 187, `T/🎬️media-slots/🎟️collect` 95, `T/🎬️media-slots/📤️slot` 56, `T/🎬️media-slots/📥️props` 52
* direct crate deps: none
* external crates named in the code: `semio_framework_job`, `semio_framework_value`, `serde_json`, `ui_wgpu`, `ui_contract`, `semio_framework_hash`, `semio_framework_pack_json`, `serde`, `replication`
* cfg: native + wasm32
* `🎬️media-slots` root + `📥️props`, `📤️slot`, `🎟️collect` (417 lines). Pure bounded descriptors; fixtures are JSON (58 KB in the test closure, the byte-heavy part). No moves.

#### `canvas2d-paint` (WP1, 617 lines, 25 KB)

* members (source file, lines contributed): `📐️Canvas2dHost/🎨️paint` 560
* direct crate deps: none
* external crates named in the code: none
* cfg: native + wasm32
* `📐️Canvas2dHost/🎨️paint` (path geometry: `Affine`, `FillRule`, `LineCap`, `ScenePathSegment`, `Contour`; 617 lines) split away from the gumball host because EngineCanvas and Scenes use the geometry while the gumball uses scene state. Tests: `path-paint` (180 lines, 30 KB fixture).

#### `asset-authority` (WP1, 918 lines, 39 KB)

* members (source file, lines contributed): `R/🗂️asset-authority` 714, `🖼️IconRenderHost/📤️export/📥️asset` 123, `R/🔁️asset-decode-jobs` 22
* direct crate deps: `contracts`
* external crates named in the code: none
* cfg: native + wasm32
* `🗂️asset-authority` (statics `RENDERER_ASSET_IO`, `RENDERER_ASSET_GENERATION`, fetch owner, page/format structure cursors) + the three `renderer_asset_*_prefix_is_valid` functions (moved up from decode jobs) + `IconRenderHost/📤️export/📥️asset` (exact-request GLB delivery slot, moved here because `close_renderer_asset_step` calls it). Everything else that names these statics is above this crate.

#### `asset-decode` (WP1, 2993 lines, 155 KB)

* members (source file, lines contributed): `R/🦴️glb-decode` 2356, `R/🔁️asset-decode-jobs` 557
* direct crate deps: `contracts`, `host-base`, `asset-authority`
* external crates named in the code: `semio_framework_job`, `ui_wgpu`, `semio_framework_async`, `semio_framework_value`, `semio_framework_job_worker`, `ui_contract`, `semio_framework_pixels`, `infinite_world`, `serde`, `semio_framework_job_worker_authority`
* cfg: native + wasm32
* `🦴️glb-decode` (2,419 lines: JSON lexer, schema cursors, materialize cursors) + decode jobs (`RendererAssetProbe`, `RendererAssetDecodeJob/Session`; ~570 lines). 155 KB: if a later change pushes it over 230 KB split `glb-decode` (125 KB, depends only on `asset-authority`) into its own crate. The worker pool and I/O slots that used to sit at the bottom of this file are in `host-base` (`renderer-io`).

#### `doors` (WP1, 794 lines, 43 KB)

* members (source file, lines contributed): `T/🔌️socket-door` 610, `T/📇️directory-door` 184
* direct crate deps: `host-base`
* external crates named in the code: `serde_json`, `store_sync`, `semio_framework_os_kernel`, `semio_framework_async`, `semio_framework_io_base64`, `semio_framework_os_services`
* cfg: native + wasm32
* `🔌️socket-door` (duplex socket: verbs, frames, pages) + `📇️directory-door` (the one directory transport seam). `host_io_call` and `spawn_app_task` come from `host-base`. `agent-bridge-door` stays with `agent-elements`, which it serves.

#### `kernel-protocol` (WP1, 2527 lines, 120 KB)

* members (source file, lines contributed): `R/📦️kernel-runtime/📬️kernel-replay-client` 1092, `R/📦️kernel-runtime/🧮️kernel-core` 810, `R/📦️kernel-runtime/🏊️kernel-pool` 247, `T/🎠️runtime` 167, `R/📦️kernel-runtime` 31
* direct crate deps: `contracts`, `host-base`
* external crates named in the code: `semio_framework_actor`, `semio_framework`, `semio_framework_async`, `semio_framework_plugin_host`, `semio_framework_os_services`, `serde_json`, `semio_framework_plugin_guest_contract`, `semio_framework_job`, `semio_framework_value`, `protocol`, `serde`, `semio_framework_plugin`, `store_sync`
* cfg: native only
* Native-only. Bottom of the actor-kernel group: `kernel-core` (sequence authority, job-progress presentation, turn budget, extension index, `CreateAppRequestOwner`, `QueuedKernelEvent`, `KernelRequest`, close submissions) + the whole mounted-product-replay subsystem lifted from `kernel-replay-client` lines 1 to 935 (raw spawn owner, recovery/refusal registries, `MountedProductReplay*`, `RefusedProductReplay`) + `ResponseSlot`/`ResponseState`/`KernelOutcome`/`ExchangeSurfaceDocument`/`ExchangeOutcome`/`TypedOperationResultPage` + `KernelRequestQueue` (+ state, capacity; 250 lines) + `ParallelRuntime` (`🎠️runtime`) + `read_native_component`. Inversion I1 (job-progress acknowledgement callback). Test: `typed-result-page` (124).

#### `kernel-retained` (WP1, 879 lines, 38 KB)

* members (source file, lines contributed): `R/📦️kernel-runtime/🗃️kernel-retained-state` 843
* direct crate deps: `kernel-protocol`
* external crates named in the code: `semio_framework_actor`, `ui_contract`, `semio_framework_job`, `semio_framework_async`, `semio_framework`
* cfg: native only
* Native-only. `🗃️kernel-retained-state` unchanged (retained document exchange, surfaces, pending rejections, mounted replay routes, command-document retirement; 935 lines). The 1,403-line `semantic-document` law that used to be `include!`d into the pool is not here: it names items of all four kernel crates and lives in the facade `kernel-runtime` (make the named items `pub`).

#### `kernel-pool` (WP1, 2234 lines, 140 KB)

* members (source file, lines contributed): `R/📦️kernel-runtime/🏊️kernel-pool` 1941, `R/📦️kernel-runtime/📬️kernel-replay-client` 224, `R/📦️kernel-runtime/🗃️kernel-retained-state` 54
* direct crate deps: `contracts`, `host-base`, `kernel-protocol`, `kernel-retained`
* external crates named in the code: `semio_framework_job`, `semio_framework`, `semio_framework_actor`, `semio_framework_plugin_host`, `ui_contract`, `serde_json`, `protocol`, `semio_framework_hash`, `semio_framework_os_services`, `semio_framework_async`, `semio_framework_pack_json`, `semio_framework_os_kernel`
* cfg: native only
* Native-only. `🏊️kernel-pool` (`struct KernelPoolState` with its 30 fields, moved here from `🗃️kernel-retained-state` so it shares a crate with its 1,898-line impl, `run_kernel_pool`, yield turns) + `KernelClient`, `KernelFuture`, `global_client`, `persistent_command_completion_port_ready` (lifted from the replay-client file; they spawn the pool). Installs the job-progress acknowledgement (I1).

#### `kernel-runtime` (WP1, 913 lines, 57 KB)

* members (source file, lines contributed): `R/📊️scale-bench` 827, `R/📦️kernel-runtime` 8
* direct crate deps: `contracts`, `host-base`, `kernel-protocol`
* external crates named in the code: `serde_json`, `semio_framework_actor`, `semio_framework_os_services`, `semio_framework`, `semio_framework_async`, `semio_framework_hash`, `semio_framework_pack_json`, `semio_framework_job`, `semio_framework_plugin_host`
* cfg: native only
* Native-only facade: `📦️kernel-runtime/🦀️.rs` (declares/re-exports the four kernel crates) + `📊️scale-bench` (896-line measured scenario driver). Hosts the `semantic-document` law (1,403 lines + 89 KB of fixtures) as its test. Re-exports exactly what `program-bridge`/`app-runtime` name.

#### `engine-registry` (WP2, 2442 lines, 115 KB)

* members (source file, lines contributed): `⚙️EngineCanvas/🗂️engine-surface-registry` 1058, `⚙️EngineCanvas/📦️prepared-engine-canvas` 724, `⚙️EngineCanvas/✍️text-editor` 218, `⚙️EngineCanvas/🎓️tutorial-surface-geometry` 159, `⚙️EngineCanvas` 44, `⚙️EngineCanvas/🕸️node-graph` 26, `⚙️EngineCanvas/🧩️engine-surface-attach` 9
* direct crate deps: `contracts`, `asset-authority`
* external crates named in the code: `semio_framework_job`, `ui_wgpu`, `semio_framework_value`, `infinite_canvas`, `semio_framework_surface`, `semio_framework_async`, `serde_json`, `framework_editor`, `semio_framework`, `web_sys`, `wasm_bindgen`, `semio_framework_os_node_graph`, `flow`, `semio_framework_os_kernel`, `semio_framework_canvas` **[WP0 change]** (`use semio_framework_canvas as canvas;` in the surface registry, prepared canvas and tutorial geometry files)
* cfg: native + wasm32
* Bottom of EngineCanvas: surface registry (`ENGINE_SURFACES`, `EngineSurface*`, retirement 408-line impl), prepared engine canvas (`EngineCanvasPacket`, staging, `STAGED_ENGINE_SCENES`, presenter), tutorial surface geometry, plus moves: text-editor delivery state (`TextEditorDeliveryState` & 9 siblings), `PublishedGraphInteraction`, the two engine log fns, `engine_camera_from_json`. Statics: `ENGINE_SURFACES` (used by 4 crates), `MAP_TILE_ASSET_FAULT`, `STAGED_ENGINE_SCENES`.

#### `engine-text` (WP2, 1137 lines, 64 KB)

* members (source file, lines contributed): `⚙️EngineCanvas/✍️text-editor` 1040
* direct crate deps: `contracts`, `engine-registry`
* external crates named in the code: `ui_wgpu`, `serde`, `serde_json`, `semio_framework`, `semio_framework_os_kernel`, `semio_framework_value`, `framework_editor`, `js_sys`, `wasm_bindgen`
* cfg: native + wasm32
* `⚙️EngineCanvas/✍️text-editor` (editor scene document, delivery, interaction; 1,137 lines after moves). Owns the `semioWgpuHostPageHidden` logic.

#### `engine-board` (WP2, 1798 lines, 94 KB)

* members (source file, lines contributed): `⚙️EngineCanvas/🎲️board2d` 813, `⚙️EngineCanvas/🖌️paint2d` 528, `⚙️EngineCanvas/🗺️tiled-map` 271
* direct crate deps: `contracts`, `asset-authority`, `engine-registry`
* external crates named in the code: `ui_wgpu`, `infinite_canvas`, `serde_json`, `semio_framework_job`, `semio_framework_surface`, `semio_framework_canvas` **[WP0 change]**
* cfg: native + wasm32
* `🎲️board2d` + `🖌️paint2d` + `🗺️tiled-map` (1,798). The `puzzle_board_*_into` family is defined here; `scene-text-vfs` keeps its five one-line forwarders.

#### `engine-graph` (WP2, 1297 lines, 68 KB)

* members (source file, lines contributed): `⚙️EngineCanvas/🕸️node-graph` 1166
* direct crate deps: `contracts`, `host-base`, `media-slots`, `engine-registry`, `engine-board`
* external crates named in the code: `ui_wgpu`, `flow`, `serde_json`, `semio_framework_os_kernel`, `semio_framework`, `semio_framework_os_node_graph`, `ui_styling`, `semio_framework_canvas` **[WP0 change]**
* cfg: native + wasm32
* `🕸️node-graph` (1,297). Tests: `catalogue-workflow-drop` (80) and one of the node-graph tests (601 lines total in-crate).

#### `engine-attach` (WP2, 1128 lines, 58 KB)

* members (source file, lines contributed): `⚙️EngineCanvas/🧩️engine-surface-attach` 1046
* direct crate deps: `contracts`, `media-slots`, `canvas2d-paint`, `engine-registry`, `engine-text`, `engine-board`, `engine-graph`
* external crates named in the code: `serde_json`, `ui_wgpu`, `infinite_canvas`, `semio_framework_value`, `flow`, `ui_styling`, `semio_framework_tool_run`, `semio_framework_io_base64`, `semio_framework_canvas`, `framework_editor`, `semio_framework_os_node_graph`, `semio_framework_surface` **[WP0 change]** (named by the live source)
* cfg: native + wasm32
* `🧩️engine-surface-attach` (`AttachedSurfaceRegistry`, per-kind scene sync; 1,128) + the parent `EngineCanvas` facade items it needs. Its tests are the byte-heaviest in the plan (2,264 lines / 231 KB): move `engine-surfaces` (1,179), `paint2d-engine` (1,087) and the gestures test to `engine-attach-tests`.

#### `scene-state` (WP2, 2629 lines, 115 KB)

* members (source file, lines contributed): `🎞️Scenes/🗃️scene-state` 773, `🎞️Scenes/🪣️pending-raster` 372, `🎞️Scenes` 314, `🎞️Scenes/📷️scene-camera-dispatch` 288, `📐️Canvas2dHost` 203, `🎞️Scenes/🖌️canvas2d` 125, `🎞️Scenes/⌨️scene-input` 88, `T/🏠️os-host` 69, `🎞️Scenes/🎨️scene-render-entry` 57, `🎞️Scenes/🧮️table-and-paint2d` 43, `🎞️Scenes/🧭️node-graph-and-tiled-map` 26, `🎞️Scenes/✍️text-editor` 17 … (+4 more)
* direct crate deps: `contracts`, `host-base`, `canvas2d-paint`, `engine-board`
* external crates named in the code: `ui_wgpu`, `ui_contract`, `semio_framework_job`, `semio_framework_value`, `ui_render`, `semio_framework`, `serde_json`, `infinite_world`, `web_sys`, `semio_framework_async`, `semio_framework_trace`, `semio_framework_os_renderer_camera` (`camera_storage`) **[WP0 change]**
* cfg: native + wasm32
* Bottom of Scenes and the busiest landing site: Scenes parent (`SceneCell`, statics, `scene_host_retiring`, `mount_scene_identity`, `retire_scene_identity`, list transfer), `scene-state`, `pending-raster`, `scene-camera-dispatch`, plus the shared kit lifted from higher scene files: `SceneListHit`, `SceneModifiers`, `SceneListTransfer{Start,Release}`, `BLOCK_LIST_PALETTE_DRAG_MIME`, `now_ms`, `render_placeholder`, the four `*ChromeLabels`, `SceneEngineHosts`, the three `Table*AccessibilityCell`, `Table{EditableText,Stepper}FocusTarget`, `TextEditorUiState`, `TextEditorRenameDraft`, `InkBoundsF`, `retire_tiled_map_scene_identity`, `VfsAccessibilityFocusTarget`, `request_canvas_pointer_gesture_cancel_for_host`, `CanvasLayer` + the five Canvas JSON types, `draw_ink_rect_outline`, `queue_canvas_image_upload`, gumball types (`GUMBALL`, `GumballSlot/Gesture/View/Handle/Motion/Dispatch/Config/Meta`, `ModelToLayer`, constants) and the component-surface-close bridge (`ComponentSurfaceClose{Token,Request,Bridge}`, `COMPONENT_SURFACE_CLOSE_BRIDGE`, `request/acknowledge/..._close`) out of `os-host`, `note_world3d_live_camera`, `take_world_camera_diagnostic`, `WORLD_CAMERA_LEDGER`, `canvas_active_utility`, `write_scene_action_batch`. Statics: `SCENE_STATE`, `SCENE_SURFACE_RETIREMENT`, `CANVAS_GESTURE`, `CANVAS_CATALOGUE_HOVER`, `PENDING_RASTER_STATE`, `SCENE_LIST_TRANSFER`, `SCENE_CAMERA_DISPATCH_*`, `GUMBALL`. Source files touched: 16 (largest fan-in of the plan), so the executor must land it before `scene-*` leaves compile.

#### `canvas2d-host` (WP2, 383 lines, 19 KB)

* members (source file, lines contributed): `📐️Canvas2dHost` 340
* direct crate deps: `contracts`, `scene-state`
* external crates named in the code: `ui_wgpu`, `ui_render`, `serde_json`, `semio_framework`
* cfg: native + wasm32
* `📐️Canvas2dHost` gumball dispatch after its types moved down (383 lines): `pointer_button_into`, `pointer_move_into`, `cancel_into`, `retire_window`, `paint`, circle/ring geometry. Test `gumball-dispatch` (99, 34 KB fixture).

#### `scene-canvas2d` (WP2, 1377 lines, 69 KB)

* members (source file, lines contributed): `🎞️Scenes/🖌️canvas2d` 940, `🗣️Interpreter/🖼️ui-image-loading` 301
* direct crate deps: `contracts`, `media-slots`, `canvas2d-paint`, `asset-authority`, `scene-state`
* external crates named in the code: `ui_wgpu`, `image`, `usvg`, `base64`, `serde_json`, `semio_framework_2d`, `tiny_skia`, `resvg`
* cfg: native + wasm32
* `🖌️canvas2d` scene (paint, fill blend, shapes) + `🖼️ui-image-loading` (image cache statics `UI_IMAGE_*`, `current_ui_image_key`, svg rasterisation, `render_ui_image_step`; 337) lifted out of the Interpreter because Scenes call it. 1,377 lines.

#### `scene-ink` (WP2, 2393 lines, 108 KB)

* members (source file, lines contributed): `🎞️Scenes/✒️ink-canvas-model-state` 1936, `🎞️Scenes/🖋️ink-canvas-render` 319
* direct crate deps: `contracts`, `scene-state`, `scene-canvas2d`
* external crates named in the code: `ui_wgpu`, `serde_json`, `ui_render`
* cfg: native + wasm32
* `✒️ink-canvas-model-state` (2,054: `InkInteractionJob` 568-line impl, camera/documents/events) + `🖋️ink-canvas-render`. 2,393 lines; tests 1,890 lines (raster-frame-cost, ink-canvas, the 1,121-line standalone slice) go to `scene-ink-tests`.

#### `scene-feeds` (WP2, 1416 lines, 68 KB)

* members (source file, lines contributed): `🎞️Scenes/📰️event-feed-and-timeline` 896, `🎞️Scenes/🧭️node-graph-and-tiled-map` 400
* direct crate deps: `contracts`, `host-base`, `native-platform`, `engine-board`, `scene-state`, `scene-canvas2d`
* external crates named in the code: `ui_wgpu`, `ui_contract`, `serde_json`, `semio_framework`, `semio_framework_surface`, `ui_render`, `serde`, `ui_styling`
* cfg: native + wasm32
* `📰️event-feed-and-timeline` + `🧭️node-graph-and-tiled-map` scenes (1,416). Uses `native-platform` for host temporal formatting and `current_ui_image_key` from `scene-canvas2d`.

#### `scene-tables` (WP2, 1500 lines, 84 KB)

* members (source file, lines contributed): `🎞️Scenes/🧱️block-list-and-diff` 705, `🎞️Scenes/🧮️table-and-paint2d` 688
* direct crate deps: `contracts`, `media-slots`, `scene-state`, `scene-feeds`
* external crates named in the code: `serde_json`, `ui_wgpu`, `semio_framework`
* cfg: native + wasm32
* `🧮️table-and-paint2d` + `🧱️block-list-and-diff` (1,500). The table accessibility cells it defines moved down.

#### `scene-text-vfs` (WP2, 1281 lines, 67 KB)

* members (source file, lines contributed): `🎞️Scenes/🧩️board2d-and-vfs` 658, `🎞️Scenes/✍️text-editor` 444
* direct crate deps: `contracts`, `engine-text`, `engine-board`, `scene-state`, `scene-canvas2d`, `scene-feeds`
* external crates named in the code: `ui_wgpu`, `ui_contract`, `serde_json`, `semio_framework`, `ui_render`, `serde`, `infinite_canvas`
* cfg: native + wasm32
* `🧩️board2d-and-vfs` + `✍️text-editor` scenes (1,281); forwarders to `engine-board`/`engine-text`.

#### `icon-host` (WP2, 2538 lines, 121 KB; was 2289, 101) **[WP0 change]**

* members (source file, lines contributed): `🖼️IconRenderHost/📤️export/📐️svg` 692, `🖼️IconRenderHost/📤️export/🎬️scene` 504, `🎞️Scenes/🖼️icon-render` 413, `🖼️IconRenderHost/📤️export/🧊️gpu` 252, `🖼️IconRenderHost` 156, `🖼️IconRenderHost/📤️export/📷️png` 74, `🖼️IconRenderHost/📤️export` 31
* direct crate deps: `contracts`, `host-base`, `media-slots`, `scene-state`
* external crates named in the code: `infinite_world`, `semio_framework_job`, `ui_wgpu`, `semio_framework_value`, `semio_framework_plugin`, `serde_json`, `usvg`, `tiny_skia`, `resvg`, `semio_framework_pixels`
* cfg: native + wasm32
* **[WP0 change]** **File budget.** The lib closure of this crate today is 8 sources (7 plus the generated `🖼️assets/🔣️icons/🤖️generated/🧩️icon-embeds/🦀️.rs`, 255 lines, 20,119 B) and 251 embedded files (the 250 raw icon SVGs and the 682-byte logo), 259 files against 28 (ceiling 32) and 164 KB of embeds. The embeds must leave the crate before `icon-host` can pass the gate: feed the atlas from the Canvas icon pack installed through the runtime asset port (`semio_framework_canvas::RuntimeAsset::IconPack`, installed by `host-base` `runtime-assets`) or add a raw-icon asset to that port; the 250 `include_str!` list is a build-time index of checked-in sources, not data that has to be compiled in. Decision for WP2 with the owner of the Canvas assets.
* `🖼️IconRenderHost` atlas root (**[WP0 change]** the plan assumed the catalogue comes from the Canvas icon pack installed through the runtime asset port; the live tree still embeds the 250 SVGs through `ICON_SVGS`, see the file budget above) + the whole `📤️export` tree (png, gpu, scene, svg; the `asset` slot moved to `asset-authority`) + `🎞️Scenes/🖼️icon-render` scene. 2,289 lines. `IconExportBatch` (Shell, 305-line impl) is in `shell-doc-open`.

#### `scene-render` (WP2, 1618 lines, 82 KB)

* members (source file, lines contributed): `🎞️Scenes/⌨️scene-input` 898, `🎞️Scenes/🎨️scene-render-entry` 549
* direct crate deps: `contracts`, `engine-registry`, `engine-board`, `engine-graph`, `engine-attach`, `scene-state`, `canvas2d-host`, `scene-canvas2d`, `scene-ink`, `scene-feeds`, `scene-tables`, `scene-text-vfs`, `icon-host`
* external crates named in the code: `ui_wgpu`, `serde_json`, `ui_render`, `semio_framework`, `infinite_world`, `semio_framework_trace`, `semio_framework_job`, `web_sys`, `wasm_bindgen`
* cfg: native + wasm32
* `⌨️scene-input` (canvas pointer wire, cancellation, catalogue drag, list hits) + `🎨️scene-render-entry` (surface context menu, component scene render entry). The top of Scenes (1,618 lines); everything it shares with the scene leaves was lifted into `scene-state`. 3,332 lines with its tests: the byte-tight neighbour is `engine-attach` (see there).

#### `ui-engine-focus` (WP3, 1682 lines, 95 KB)

* members (source file, lines contributed): `🗣️Interpreter/🖋️ink-and-vfs-focus` 699, `🗣️Interpreter/✍️text-and-table-focus` 513, `🗣️Interpreter/🧩️retained-ui-engine` 288
* direct crate deps: `contracts`, `engine-registry`, `engine-text`, `engine-graph`, `scene-state`, `scene-ink`, `scene-tables`, `scene-text-vfs`
* external crates named in the code: `ui_wgpu`, `semio_framework_job`, `semio_framework_value`, `semio_framework_job_worker`, `semio_framework_async`, `image`, `ui_render`, `semio_framework_job_worker_authority`, `base64`, `ui_contract`
* cfg: native + wasm32
* `✍️text-and-table-focus` + `🖋️ink-and-vfs-focus` focus statics and key handlers + from `retained-ui-engine`: scene intent queue (`SCENE_INTENTS`, `SceneIntentQueue/Event`, `SceneInteractionIntent`), clipboard IO (`MountedClipboardIo`, `submit_clipboard_io`, `pump_clipboard_io_one`, `deliver_clipboard_recipient`, `validate_clipboard_source`, `write_owned_os_clipboard`, `write_os_clipboard`), `apply_presented_scene_caret`. 1,682 lines. Statics: ten `FOCUSED_*` / `INK_CLIPBOARD_STREAMS` / `PENDING_NATIVE_*` thread-locals read by `ui-engine-dump` and `ui-engine`.

#### `ui-engine-dump` (WP3, 2175 lines, 121 KB)

* members (source file, lines contributed): `🗣️Interpreter/🔬️introspection` 1260, `🗣️Interpreter/🎯️chrome-ledger` 654, `🗣️Interpreter/🧱️mesh-and-board2d-stats` 199
* direct crate deps: `contracts`, `host-base`, `native-platform`, `media-slots`, `engine-text`, `scene-state`, `scene-feeds`, `ui-engine-focus`
* external crates named in the code: `ui_wgpu`, `ui_contract`, `serde`, `serde_json`, `semio_framework_job`, `ui_render`, `semio_framework_trace`, `js_sys`, `semio_framework_pack_json`, `semio_framework`, `web_sys`
* cfg: native + wasm32
* `🔬️introspection` (1,012) + `🧱️mesh-and-board2d-stats` + `🎯️chrome-ledger` (6 dump structs, accessibility builders, ledger). The six `dump*` wasm exports' logic is here. 2,175 lines; test `introspection` (444).

#### `ui-engine` (WP3, 2306 lines, 132 KB)

* members (source file, lines contributed): `🗣️Interpreter/🧩️retained-ui-engine` 1018, `🗣️Interpreter/🖱️scene-interaction` 425, `🗣️Interpreter/📄️retained-document-consumer` 331, `🗣️Interpreter/🧮️render-plan-validator` 191, `🗣️Interpreter` 38
* direct crate deps: `contracts`, `host-base`, `engine-text`, `engine-board`, `scene-state`, `scene-canvas2d`, `scene-ink`, `scene-feeds`, `scene-tables`, `scene-text-vfs`, `scene-render`, `ui-engine-focus`, `ui-engine-dump`
* external crates named in the code: `ui_wgpu`, `ui_render`, `serde_json`, `semio_framework_job`, `infinite_world`, `semio_framework`, `ui_contract`, `web_sys`, `wasm_bindgen`, `semio_framework_async`
* cfg: native + wasm32
* `🧩️retained-ui-engine` remainder (dispatch, `dispatch_accessibility_event` 252 lines, retained input receipts, `apply_ui_commands`, `request_ui_document_close`, `close_ui_document_one`, `apply_scene_ui_command`) + `📄️retained-document-consumer` + `🖱️scene-interaction` + `🧮️render-plan-validator` + the Interpreter parent. 2,306 lines; tests (ui-command-wiring 2,594, app-catalogue 159, validator 263, worker-cell 68) sit in the Shell test bins and `ui-engine-tests`.

#### `hub-contracts` (WP1, 1298 lines, 63 KB)

* members (source file, lines contributed): `🔐️HubSignIn` 669, `🏘️SpaceBrowser` 475
* direct crate deps: none
* external crates named in the code: `serde_json`
* cfg: native + wasm32
* `🔐️HubSignIn` + `🏘️SpaceBrowser` (pure contract twins; 1,298). Tests 633 lines / 154 KB (fixtures): byte-tight at 218 KB.

#### `hub-workspace` (WP1, 1185 lines, 67 KB)

* members (source file, lines contributed): `🔗️HubConnection` 722, `🛂️SpaceAdministration` 171, `👕️canvas-presence` 129
* direct crate deps: `contracts`, `media-slots`, `hub-contracts`
* external crates named in the code: `serde_json`, `semio_framework_value`
* cfg: native + wasm32
* `🔗️HubConnection` + `🛂️SpaceAdministration` + `👕️canvas-presence` (1,185). Uses `Viewport` from `contracts`.

#### `agent-elements` (WP1, 2086 lines, 100 KB)

* members (source file, lines contributed): `🔗️AgentBridge` 1121, `T/🌉️agent-bridge-door` 316, `🤖️AgentApprovals` 297, `🚦️AgentPresence` 78, `💬️AgentChatPanel` 44
* direct crate deps: `host-base`, `media-slots`, `doors`
* external crates named in the code: `serde_json`, `semio_framework_async`, `semio_framework_os_kernel`
* cfg: native + wasm32
* `🔗️AgentBridge` (1,214) + `🚦️AgentPresence` + `🤖️AgentApprovals` + `💬️AgentChatPanel` + `🌉️agent-bridge-door` (329, the transport). 2,086 lines; tests 941.

#### `dock` (WP1, 2149 lines, 109 KB)

* members (source file, lines contributed): `🛰️Dock` 2017
* direct crate deps: `contracts`
* external crates named in the code: `ui_wgpu`
* cfg: native + wasm32
* `🛰️Dock` (`DockState` 630-line impl, layout, stack chrome, split resize; 2,149). In-crate tests: app-mode-layouts, window-template-drag, stack-drop-destination (402); the 1,378-line `wgpu-unit` names `ShellState` and goes to a Shell test bin.

#### `program-bridge` (WP1, 1820 lines, 120 KB)

* members (source file, lines contributed): `🌉️ProgramBridge` 1742, `T/⌨️native-entrypoint/📦️modules` 68
* direct crate deps: `contracts`, `host-base`, `media-slots`, `kernel-protocol`, `kernel-pool`
* external crates named in the code: `semio_framework`, `protocol`, `serde_json`, `semio_framework_pack_json`, `js_sys`, `ui_contract`, `semio_framework_value`, `semio_framework_plugin`, `store`, `semio_framework_os_kernel`, `dsl`, `serde`, `ui_wgpu`, `semio_framework_job`
* cfg: native + wasm32
* `🌉️ProgramBridge` (`ProgramBridgeEntry` 428-line impl, plugin filtering/loading, fixtures; 1,820, 120 KB) + `⌨️native-entrypoint/📦️modules` (`NativeRuntimeModule`, `NativeRuntimeManifest`, `NativeJsonPages`; 78). `ProgramFault` is in `contracts`.

#### `mailbox-core` (WP1, 214 lines, 9 KB)

* members (source file, lines contributed): `T/📮️runtime-mailbox-core` 192
* direct crate deps: none
* external crates named in the code: none
* cfg: every target incl. wasm32-wasip2, zero dependencies
* `📮️runtime-mailbox-core` (target-neutral bounded completion queue and interaction-checkout ledger; 214 lines; the only module of the wasip2 build). Zero dependencies. Tests `runtime-mailbox-core-unit` + `runtime-mailbox-admission` (264 lines, 31 KB).

#### `host-plumbing` (WP3, 1200 lines, 71 KB)

* members (source file, lines contributed): `T/📐️surface-lane` 450, `T/🎮️input-wire` 227, `T/📸️render-snapshot` 136, `T/⏰️deadlines` 85, `T/🎟️native-caller` 78, `T/🌐️browser-worker/📤️output` 48, `T/🪟️winit-app` 37
* direct crate deps: `contracts`, `host-base`, `media-slots`
* external crates named in the code: `semio_framework_job`, `semio_framework_value`, `ui_render`, `semio_framework_pack_json`, `serde`, `ui_wgpu`, `semio_framework_trace`, `semio_framework_job_worker`
* cfg: native + wasm32
* Small host adapters: `📐️surface-lane`, `📸️render-snapshot`, `🎟️native-caller`, `⏰️deadlines`, `🎮️input-wire`, `🌐️browser-worker/📤️output`, `🪙️authority/🖼️frame`, plus the three input-normalisation helpers from `winit-app` (`event_modifiers_to_pointer`, `key_action_from_dispatch`, `pointer_button_to_i16`). 1,200 lines; 7 unit tests, 96 KB of fixtures.

#### `frame-actions` (WP3, 702 lines, 33 KB)

* members (source file, lines contributed): `R/🛎️frame-actions` 689, `T/🧵️frame-job` 11, `R/🎨️presentation-raster` 1
* direct crate deps: `host-base`, `engine-text`, `ui-engine` + `shell-*`
* external crates named in the code: `ui_wgpu`, `semio_framework_job`, `semio_framework_value`, `ui_render`, `serde`, `ui_host`, `semio_framework_async`
* cfg: native + wasm32
* Frame action machinery without the runtime: `RuntimeDispatchCursor`, `FrameActionEnvelope/ReceiptOwner/BatchOwner/Owners`, deferred cursors, maintenance authority, `FrameBuildInputs`/`FrameDirectives` (from `frame-job`), `WORLD3D_DEADLINE_CAPACITY`. `RuntimeApply` and its owners moved to `runtime-mailbox` (I2). 702 lines.

#### `runtime-mailbox` (WP3, 2931 lines, 155 KB)

* members (source file, lines contributed): `R/📮️mailbox-impl` 1602, `R/🎨️presentation-raster` 790, `R/🛎️frame-actions` 370, `R/🧠️app-runtime` 134, `R/🧾️frame-build` 2
* direct crate deps: `contracts`, `host-base`, `asset-authority`, `asset-decode`, `engine-registry`, `engine-text`, `engine-board`, `scene-canvas2d`, `ui-engine-focus`, `program-bridge`, `mailbox-core`, `host-plumbing`, `frame-actions`
* external crates named in the code: `semio_framework_job`, `infinite_world`, `semio_framework_os_services`, `semio_framework_async`, `semio_framework_value`, `ui_wgpu`, `semio_framework_trace`, `ui_host`, `semio_framework_actor`, `semio_framework_os_kernel`, `ui_render`
* cfg: native + wasm32 (the native asset-stream half is `cfg(not(wasm32))`)
* Generic over the runtime state (I2): `RuntimeMailbox<R>` (1,602-line impl), `RuntimeMailboxInner<R>`, presentation/raster authorities, completion queue, staged reference images, `NativeReferenceDecodeJob`, native asset transport and the five asset-stream functions, `RuntimeApply` + `FrameMaintenanceOwner` + `FrameDeferredExecutionOwner` + `FrameMaintenanceRefusal`, `AppHandle<R>`. 2,931 lines / 155 KB. The async-boundary law (2,612 lines, 26 embedded sources = 1,068 KB today) goes to `runtime-mailbox-tests` with run-time source reads.

#### `app-runtime` (WP3, 2229 lines, 161 KB)

* members (source file, lines contributed): `R/🧠️app-runtime` 1164, `R/🧾️frame-build` 628, `R/🖥️frame-presentation` 262, `T/🪟️winit-app` 137, `R/🔬️native-socket-probe` 3
* direct crate deps: `contracts`, `host-base`, `kernel-protocol`, `engine-registry`, `engine-text`, `engine-board`, `engine-graph`, `scene-feeds`, `scene-text-vfs`, `icon-host`, `ui-engine-focus`, `ui-engine`, `program-bridge`, `mailbox-core`, `host-plumbing`, `frame-actions`, `runtime-mailbox` + `shell-*`
* external crates named in the code: `semio_framework_job`, `ui_wgpu`, `semio_framework_value`, `infinite_world`, `semio_framework_job_worker`, `ui_render`, `semio_framework_async`, `semio_framework_trace`, `semio_framework_job_worker_authority`, `semio_framework_os_services`, `semio_framework_actor`, `ui_contract`
* cfg: native + wasm32
* `AppRuntime` (one 696-line impl), `AppInteractionState` (481), `AppTextStream`, frame engine packets, the build cursors (`FrameBuildCursor`, `AppFrameAfterChrome`, `FrameFinishCursor`, `AppFrameBuild` + its impl …), the `frame_*_child_step` helpers, world3d traces, `AppFramePreparation` / `FramePreparationStep` / `AppFramePresentation` (moved down from `frame-presentation` / `frame-build` so type and impl share a crate), `dispatch_normalized_event`, `ICON_ATLAS_RUNTIME`, the Anta outline font embed (26 KB). Implements `MailboxRuntime`/`InteractionDispatch`. 1,921 lines but **142 KB**: the tightest byte budget among lib crates; do not add embeds.

#### `frame-presentation` (WP3, 1139 lines, 62 KB)

* members (source file, lines contributed): `R/🖥️frame-presentation` 1139
* direct crate deps: `contracts`, `host-base`, `media-slots`, `engine-registry`, `engine-attach`, `ui-engine-dump`, `ui-engine`, `host-plumbing`, `runtime-mailbox`, `app-runtime`
* external crates named in the code: `ui_wgpu`, `semio_framework_job`, `semio_framework_value`, `semio_framework_trace`, `infinite_world`, `semio_framework_job_worker`, `winit`, `web_sys`
* cfg: native + wasm32
* `AppPresenter` (815-line impl), `AppPresentStallWatch`, `AppPresentInputWait`, `AppSurfaceResizePhase`, surface context (1,401). Gains `close_world3d_dynamic_step` (I3).

#### `frame-transaction` (WP3, 945 lines, 55 KB)

* members (source file, lines contributed): `R/🧾️frame-build` 781, `R/🧠️app-runtime` 134
* direct crate deps: `host-base`, `engine-registry`, `engine-text`, `engine-board`, `scene-state`, `scene-canvas2d`, `icon-host`, `ui-engine-focus`, `ui-engine`, `program-bridge`, `mailbox-core`, `frame-actions`, `runtime-mailbox`, `app-runtime`, `frame-presentation` + `shell-*`
* external crates named in the code: `ui_wgpu`, `semio_framework_job`, `semio_framework_trace`, `infinite_canvas`, `infinite_world`, `semio_framework_os_kernel`, `winit`, `web_sys`, `semio_framework_value`
* cfg: native + wasm32
* `FrameTransaction` (706-line impl), `AppFrameTransactionStep/Phase`, `FrameTransactionStage`, `boot_runtime` (134). About 950 lines.

#### `frame-job` (WP3, 925 lines, 65 KB)

* members (source file, lines contributed): `T/🧵️frame-job` 920, `🪙️authority/🖼️frame` 2
* direct crate deps: `host-base`, `frame-actions`, `runtime-mailbox`, `app-runtime`, `frame-transaction`
* external crates named in the code: `semio_framework_job_worker`, `semio_framework_value`, `semio_framework_async`, `semio_framework_job`, `semio_framework_trace`, `web_sys`
* cfg: native + wasm32
* `🧵️frame-job` (`FrameBuildJob`, `FrameBuildHandle`, `FrameCompletionWake`, `ActiveFramePhase`; `FrameBuildInputs`/`FrameDirectives` moved down) + `🪙️authority/🖼️frame`. 925 lines; tests 449.

#### `native-probe` (WP3, 478 lines, 28 KB)

* members (source file, lines contributed): `R/🔬️native-socket-probe` 401, `R/🪶️sqlite` 47
* direct crate deps: `host-base`, `native-platform`, `canvas2d-host`, `program-bridge`, `host-plumbing`, `app-runtime` + `shell-*`
* external crates named in the code: `semio_framework_value`, `store`, `serde`, `serde_json`, `semio_framework_dsl_record`, `dsl`, `tokio`, `semio_framework_os_kernel`, `semio_framework_ui_locale`, `semio_framework_artifact_reference`, `semio_framework_actor`, `semio_framework_async`, `semio_framework_os_services`, `semio_framework_job`
* cfg: mostly native (125 of 477 lines also build for wasm32)
* `🔬️native-socket-probe` (probe snapshots, native runners, smoke runner, `upload_icon_atlas` logic) + `🪶️sqlite`. `run_smoke_with_axes` takes `ShellState` by value (above the composer, legitimate). `ICON_ATLAS_RUNTIME` moved down to `app-runtime`.

#### `host-winit` (WP3, 1827 lines, 100 KB)

* members (source file, lines contributed): `T/🪟️winit-app` 948, `T/🏠️os-host` 837
* direct crate deps: `contracts`, `host-base`, `native-platform`, `kernel-protocol`, `kernel-pool`, `engine-registry`, `scene-state`, `host-plumbing`, `runtime-mailbox`, `frame-presentation`, `frame-transaction`, `frame-job` + `shell-*`
* external crates named in the code: `winit`, `semio_framework_job`, `ui_host`, `semio_framework_value`, `semio_framework_trace`, `infinite_world`, `ui_render`, `wasm_bindgen`, `ui_wgpu`, `web_sys`, `js_sys`, `semio_framework_os_kernel`
* cfg: native + wasm32
* `🏠️os-host` (`OsHost` impl merged: 129 + 161 lines) + `🪟️winit-app` (event loop, `FrameGenerationHold`) minus `dispatch_normalized_event` and the helpers. 1,827 lines; tests 1,115 (os-host standalone 674).

#### `browser-worker` (WP3, 879 lines, 57 KB)

* members (source file, lines contributed): `T/🌐️browser-worker` 879
* direct crate deps: `contracts`, `host-base`, `asset-authority`, `engine-registry`, `icon-host`, `program-bridge`, `mailbox-core`, `host-plumbing`, `frame-actions`, `runtime-mailbox`, `app-runtime`, `frame-presentation`, `frame-job`, `host-winit` + `shell-*`
* external crates named in the code: `semio_framework_value`, `semio_framework_job`, `semio_framework_plugin`, `wasm_bindgen`, `web_sys`, `js_sys`, `semio_framework_actor`, `serde_json`, `ui_render`, `ui_host`, `ui_wgpu`
* cfg: wasm32-unknown-unknown only
* `🌐️browser-worker` (938 lines; `BrowserRendererWorker` 491-line wasm-bindgen impl, bootstrap, frame close). wasm32-unknown-unknown only; takes `ShellState` by value.

#### `entrypoint` (WP3, 186 lines, 8 KB)

* members (source file, lines contributed): `T/⌨️native-entrypoint` 163, `T/📦️packages/🦀️rust/📚️library` 18
* direct crate deps: `host-base`, `canvas2d-host`, `native-probe`
* external crates named in the code: `semio_framework_os_kernel`, `semio_framework_trace`, `semio_framework_async`, `semio_framework_job`
* cfg: native (+ wasm export wrappers)
* The thin top crate `semio-framework-os-renderer-wgpu`: `PKG/Cargo.toml`, `build.rs` (+ `🏗️builder`), lib root, `⌨️native-entrypoint` (`run_native_entrypoint`), the `wasm_exports` module (19 one-line wrappers), `extern crate` links for exporting crates, the `[[bin]] semio-wgpu-native`, the paint-policy test mount (`🖱️ui/🧪️testing/🎨️paint-policy`), `#[global_allocator]` (cfg test). Budget: under 600 lines.



### 6.3 Crate-level imports the dissolved root block must be replaced by (token usage of the 123 old root imports)

Each crate gets an explicit `use` list for exactly these names (the lists are the intersection of the old block with the identifiers each crate's items mention; the full per-name lists are mechanical to regenerate, section 12). The `extern crate` aliases of the old root become Cargo renames: `infinite_world` and `infinite_canvas` = package `semio-framework-os-infinite`, `dsl`, `dsl_core`, `protocol`, `store`, `store_sync` = package `semio-framework-os-kernel` (use `use semio_framework_os_kernel as dsl;` per crate, not a crate-root alias), `wasm_bindgen_futures` = `semio_framework_async` (wasm32 only), `camera_storage` = `semio-framework-os-renderer-camera`.

| crate | `infinite_world::world` names | `ui_wgpu::wgpu` names | `std` | other root imports |
|---|---:|---:|---:|---|
| `contracts` | 0 | 6 | 4 | semio_framework_os_kernel |
| `host-base` | 0 | 1 | 8 | plugin_catalog, semio_framework_async, semio_framework_job |
| `native-platform` | 0 | 0 | 4 | winit |
| `media-slots` | 0 | 0 | 1 | semio_framework_job |
| `canvas2d-paint` | 0 | 0 | 0 |  |
| `asset-authority` | 10 | 0 | 4 |  |
| `asset-decode` | 6 | 25 | 6 | infinite_canvas, semio_framework_job |
| `doors` | 0 | 0 | 1 | semio_framework_os_kernel |
| `kernel-protocol` | 0 | 0 | 7 | semio_framework_job, semio_framework_os_kernel |
| `kernel-retained` | 0 | 0 | 4 |  |
| `kernel-pool` | 0 | 0 | 6 | semio_framework_os_kernel |
| `kernel-runtime` | 0 | 0 | 2 |  |
| `engine-registry` | 1 | 3 | 2 | semio_framework_job |
| `engine-text` | 0 | 4 | 0 |  |
| `engine-board` | 3 | 4 | 1 |  |
| `engine-graph` | 0 | 6 | 2 |  |
| `engine-attach` | 0 | 1 | 2 |  |
| `scene-state` | 0 | 4 | 3 | infinite_canvas, semio_framework_job, semio_framework_os_renderer_camera |
| `canvas2d-host` | 0 | 4 | 0 |  |
| `scene-canvas2d` | 3 | 2 | 2 |  |
| `scene-ink` | 0 | 6 | 2 |  |
| `scene-feeds` | 0 | 4 | 1 |  |
| `scene-tables` | 0 | 4 | 1 | semio_framework_os_kernel |
| `scene-text-vfs` | 0 | 5 | 1 |  |
| `icon-host` | 14 | 9 | 2 | infinite_canvas, semio_framework_job |
| `scene-render` | 0 | 3 | 0 | infinite_canvas |
| `ui-engine-focus` | 0 | 4 | 1 | semio_framework_async, semio_framework_job, winit |
| `ui-engine-dump` | 0 | 2 | 1 |  |
| `ui-engine` | 0 | 6 | 3 | infinite_canvas, semio_framework_async, winit |
| `hub-contracts` | 0 | 0 | 0 |  |
| `hub-workspace` | 0 | 2 | 1 |  |
| `agent-elements` | 0 | 1 | 4 |  |
| `dock` | 0 | 6 | 1 | winit |
| `program-bridge` | 0 | 0 | 7 | program_bridge, semio_framework_job, semio_framework_os_kernel |
| `shell-* (all 24)` | 6 | 10 | 9 | infinite_canvas, program_bridge, semio_framework_job, semio_framework_os_kernel, shell, winit |
| `mailbox-core` | 0 | 0 | 0 |  |
| `host-plumbing` | 0 | 3 | 5 | semio_framework_job |
| `frame-actions` | 0 | 2 | 3 | semio_framework_job |
| `runtime-mailbox` | 24 | 7 | 7 | infinite_canvas, semio_framework_job |
| `app-runtime` | 5 | 11 | 5 | infinite_canvas, program_bridge, semio_framework_job, shell, winit |
| `frame-presentation` | 0 | 3 | 2 | infinite_canvas, semio_framework_job, winit |
| `frame-transaction` | 14 | 7 | 3 | infinite_canvas, program_bridge, semio_framework_job, shell, winit |
| `frame-job` | 0 | 0 | 1 | semio_framework_job |
| `native-probe` | 0 | 1 | 2 | program_bridge, semio_framework_job, semio_framework_os_kernel, shell, winit |
| `host-winit` | 1 | 3 | 5 | infinite_canvas, semio_framework_job, winit |
| `browser-worker` | 4 | 8 | 0 | program_bridge, semio_framework_job, shell |
| `entrypoint` | 0 | 0 | 2 | plugin_catalog |


### 6.4 Module path rename table (old `crate::…` root module → crate)

| old root module | new crate(s) |
|---|---|
| `dock` | `dock` |
| `mesh_assets` | `host-base` |
| `agent_bridge`, `agent_presence`, `agent_approvals`, `agent_chat_panel`, `agent_bridge_door` | `agent-elements` |
| `compiler_fonts` (now `runtime_assets`), `plugin_catalog`, `renderer_registration` | `host-base` |
| `space_administration`, `hub_connection`, `canvas_presence` | `hub-workspace` |
| `hub_sign_in`, `space_browser` | `hub-contracts` |
| `element_contracts` | `contracts` |
| `engine_canvas` | `engine-registry`, `engine-text`, `engine-board`, `engine-graph`, `engine-attach` |
| `interpreter` | `ui-engine`, `ui-engine-focus`, `ui-engine-dump` (+ cell in `contracts`, images in `scene-canvas2d`) |
| `media_slots` | `media-slots` |
| `program_bridge` | `program-bridge` |
| `scenes` | `scene-state`, `scene-canvas2d`, `scene-ink`, `scene-feeds`, `scene-tables`, `scene-text-vfs`, `scene-render`, `icon-host` (`scenes::icon_export`) |
| `canvas2d_gumball` / `canvas2d_paint` | `canvas2d-host` (+ gumball types in `scene-state`) / `canvas2d-paint` |
| `native_temporal`, `native_accessibility` | `native-platform` |
| `shell` | `shell-*` (7.7) |
| `icon_atlas` | `icon-host` |
| `deadlines`, `render_snapshot`, `native_caller`, `surface_lane`, `input_wire`, `browser_original_output`, `frame_authority` | `host-plumbing` |
| `frame_latency` | `host-base` |
| `os_host`, `winit_app` | `host-winit` |
| `runtime_mailbox_core` | `mailbox-core` |
| `frame_job` | `frame-job` |
| `socket_door`, `directory_door` | `doors` |
| `browser_worker` | `browser-worker` |
| `parallel_runtime`, `scale_bench`, `kernel_runtime` | `kernel-protocol` (parallel runtime), `kernel-runtime` (scale bench + facade) |
| renderer partitions: `asset_authority`; `glb_decode`, `asset_decode_jobs`; `frame_actions`; `presentation_raster`, `mailbox_impl`; `frame_build`, `app_runtime`; `frame_presentation`; `native_socket_probe`, `native_socket_sqlite` | `asset-authority`; `asset-decode` (+ io slots in `host-base`); `frame-actions`; `runtime-mailbox`; `app-runtime` + `frame-transaction`; `frame-presentation`; `native-probe` |
| root items (`HostAppearance*`, `host_*`, `boot_*`, `Wgpu*`, `log_debug*`, `app_now_ms`, `spawn_app_task`, `actor_budget_from_turn_budget`) | `host-base` |

### 6.5 Statics and thread-locals referenced across crate boundaries (29 of 107)

Each lives in the crate shown (the lowest that names it). The other 78 are used only inside their own crate. A crate that wants such a static takes a normal dependency; nothing is duplicated.

| owning crate | static / thread-local | referenced from (crate: items) |
|---|---|---|
| `contracts` | `SCENE_POINTER_OWNERS` | `ui-engine-dump` 1, `ui-engine` 5 |
| `contracts` | `UI_DOCUMENT_CLOSE_QUEUE` | `ui-engine` 3 |
| `contracts` | `UI_ENGINE` | `ui-engine-focus` 17, `ui-engine-dump` 18, `ui-engine` 40 |
| `host-base` | `RENDERER_IO_SLOTS` | `entrypoint` 1 |
| `host-base` | `RENDERER_IO_WAKE` | `frame-presentation` 1 |
| `kernel-protocol` | `CANCELLED_KERNEL_TURNS` | `kernel-pool` 1 |
| `engine-registry` | `ENGINE_SURFACES` | `engine-text` 23, `engine-board` 30, `engine-graph` 23, `engine-attach` 9 |
| `engine-registry` | `MAP_TILE_ASSET_FAULT` | `engine-board` 2 |
| `engine-registry` | `STAGED_ENGINE_SCENES` | `engine-attach` 1 |
| `scene-state` | `CANVAS_CATALOGUE_HOVER` | `scene-render` 8 |
| `scene-state` | `CANVAS_GESTURE` | `scene-render` 8 |
| `scene-state` | `GUMBALL` | `canvas2d-host` 8 |
| `scene-state` | `PENDING_RASTER_STATE` | `scene-canvas2d` 3 |
| `scene-state` | `SCENE_LIST_TRANSFER` | `scene-render` 8 |
| `scene-state` | `SCENE_STATE` | `scene-ink` 4, `scene-feeds` 43, `scene-text-vfs` 3, `scene-render` 2 |
| `scene-state` | `WORLD_CAMERA_LEDGER` | `ui-engine-dump` 1 |
| `ui-engine-focus` | `FOCUSED_INK_EDITOR` | `ui-engine-dump` 2 |
| `ui-engine-focus` | `FOCUSED_INK_SURFACE` | `ui-engine-dump` 2 |
| `ui-engine-focus` | `FOCUSED_NODE_GRAPH_CARET` | `ui-engine-dump` 2 |
| `ui-engine-focus` | `FOCUSED_TABLE_EDITABLE_TEXT` | `ui-engine-dump` 2 |
| `ui-engine-focus` | `FOCUSED_TABLE_STEPPER` | `ui-engine-dump` 3, `ui-engine` 2 |
| `ui-engine-focus` | `FOCUSED_TEXT_EDITOR` | `ui-engine-dump` 2, `ui-engine` 1 |
| `ui-engine-focus` | `FOCUSED_VFS_CONTROL` | `ui-engine-dump` 2, `ui-engine` 2 |
| `ui-engine-focus` | `INK_CLIPBOARD_STREAMS` | `ui-engine-dump` 2 |
| `ui-engine-focus` | `PENDING_NATIVE_INK_CLIPBOARD` | `ui-engine-dump` 2 |
| `ui-engine-focus` | `PENDING_NATIVE_TEXT_EDITOR_CLIPBOARD` | `ui-engine-dump` 2 |
| `ui-engine-focus` | `SCENE_INTENTS` | `ui-engine-dump` 1, `ui-engine` 7 |
| `ui-engine-focus` | `TEXT_EDITOR_INPUT_STREAMS` | `ui-engine-dump` 2 |
| `app-runtime` | `ICON_ATLAS_RUNTIME` | `native-probe` 1 |


Singleton hazard to test after carving: `UI_ENGINE` (contracts), `SCENE_STATE` (scene-state), `ENGINE_SURFACES` (engine-registry), `RENDERER_IO_SLOTS` (host-base) must each exist once; a `thread_local!` duplicated by a re-mounted file would silently fork state. The oracle's duplicate-name scan finds it.

## 7. `ShellState` and the other large impls

### 7.1 What exists

`T/../🐚️Shell/…/🦀️.rs` defines `pub struct ShellState` with **248 fields** (595 lines with docs; 179 `pub` / `pub(crate)`, 69 private, 13 carrying a `cfg`) and **20 inherent `impl ShellState` blocks, one per child file, 701 methods, 21,518 lines** (command-dispatch 2,361, keyboard-and-palette 2,256, settings-and-dock-persistence 2,028, shell-actions 1,895 …). A further 893 lines in 21 free items take `&ShellState` (list in 7.6). Inherent impls cannot span crates, so this struct has to be taken apart; the brief's god-object rule applies: components own their fields and impl, the old struct becomes a thin composer, no extension traits.

Measured coupling (field level, `self.field` inside the impls): 316 of the 696 methods that touch fields touch exactly one component of the table in 7.3, 231 touch two or more, 149 touch none (they call other methods or are helpers). Call level: each method's `self.m()` callees were resolved by name (unique names only). That graph is what forces some methods up (7.4).

### 7.2 Design rules (binding for WP4 and WP5)

1. **One component struct per field group** (7.3). A component owns its fields, an invariant-preserving API, and every method whose touched components are all at or below it in rank. Field names do not change (`state.dock.dock_tabs`, not `state.dock.tabs`); this keeps the rewrite mechanical (`self.dock_tabs` → `self.dock_tabs` inside the component, `self.dock.dock_tabs` elsewhere) and lets WP5 code against the names before WP4 lands.
2. **A method that needs fields of lower components takes them as parameters**, in rank order after `&self` / `&mut self`: `&mut Lower` if it mutates, `&Lower` otherwise. Of the 490 methods that end in tier-1 components, 283 need no foreign parameter, 136 need one, 66 need two and 5 need three.
3. **Methods that touch four or more components, or that call a higher-ranked method, are use-case methods** and live in the use-case or view crates of tier 2 and 3, as methods of a borrowed bundle (`struct PointerCtx<'a> { pub dock: &'a mut ShellDockState, pub input: &'a mut ShellInputState, … }`), never with a `&mut ShellState` parameter. A bundle is defined in the use-case crate that uses it and has `fn of(shell: &'a mut ShellState) -> Self` (split borrows through the composer's `pub` component fields). Use-case crates may depend on the composer `shell-state`; no component crate may.
4. **The composer `ShellState`** (crate `shell-state`, about 450 lines) is exactly: the 15 component structs as `pub` fields, `ShellState::new`/`boot` assembly that only calls component constructors, `impl Drop` (21 lines), `impl Default` if needed. It has no other behaviour.
5. **Upward calls go through the ports of 7.5 or move the callee down.** A method may be lifted to a higher crate (always dependency-safe, together with its callers); it may not be pushed down past a component whose fields it touches.
6. **No behaviour-only trait is introduced to split an impl.** The ten ports are inversion points for upward calls (refresh, dispatch, host effects); each has exactly one implementor.

### 7.3 Components (field groups) and where they live

Rank order (low to high) is the order of the table; it is also the crate dependency order of tier 1.

| struct | crate | fields | field names |
|---|---|---:|---|
| `ChromeState` | `shell-chrome-state` | 11 | `error`, `screen_w`, `screen_h`, `fullscreen_toggle_requested`, `fullscreen_active`, `presented_chrome_accessibility`, `presented_chrome_accessibility_generation`, `chrome_floor_scope`, `dispatch_origin`, `chrome_build`, `chrome_present` |
| `SurfaceState` | `shell-surfaces` | 14 | `world3d_states`, `world3d_window_ids`, `retired_world3d_states`, `world3d_close_cancels`, `component_world3d_retirement`, `world3d_retirement_epoch`, `world3d_retirement_sequence`, `world3d_retirement_fault`, `world3d_status`, `world3d_status_pill_trace`, `node_graph_states`, `tiled_map_states`, `icon_render_states`, `board2d_states` |
| `TutorialState` | `shell-surfaces` | 2 | `tutorial`, `tutorial_pending_document_ops` |
| `SettleState` | `shell-surfaces` | 5 | `deferred_actions`, `settling`, `owed_refresh_scope`, `settle_pump`, `shell_io_pending` |
| `PluginState` | `shell-plugins` | 14 | `plugins`, `extensions`, `plugin_filter`, `host_capability`, `space_mode`, `tool_run_panel_runs`, `plugin_faults`, `surface_faults`, `app_catalogue_json`, `app_catalogue_instance`, `active_example_id`, `pending_extension_invocations`, `default_app_pins`, `plugin_install` |
| `HistoryState` | `shell-history` | 11 | `history_cursor`, `history_entries`, `history_current_checkpoint_id`, `history_time_travel`, `history_reprojection`, `time_travel_focus`, `time_travel_polled_at_ms`, `local_folder_bindings`, `local_folder_reattach_tried`, `local_folder_direct_reattach`, `host_route_folder` |
| `PrefsState` | `shell-prefs` | 13 | `appearance_id`, `locale_id`, `terminology_id`, `driver_id`, `driver_draft`, `driver_save_label`, `theme_draft`, `theme_save_label`, `merge_policy`, `pending_theme_import`, `open_conflicts`, `selected_conflict_id`, `conflicts_seeded` |
| `DirectoryState` | `shell-directory` | 28 | `agent_chat_draft`, `identity`, `verified_session_authority`, `identity_offline`, `identity_env`, `shell_session_id`, `identity_retry_at_ms`, `identity_bootstrap_rx`, `identity_bootstrap_task`, `directory_client`, `agent_bridge`, `agent_bridge_offer_scope`, `inference_port`, `inference_port_status`, `directory_home`, `directory_transport`, `directory_cancel`, `directory_commands`, `hub_workspace`, `hub_sign_in_task`, `hub_spaces_task`, `hub_workspace_open`, `space_administration`, `space_administration_epoch`, `presence_peers`, `presence_surface`, `presence_self`, `presence_pointer` |
| `SyncState` | `shell-sync` | 18 | `sync_backbone_uri`, `sync_card_kind`, `sync_card_draft`, `sync_card_anchor`, `document_host`, `sync_channel`, `next_sync_binding_generation`, `sync_terminal_fault`, `sync_status`, `sync_link`, `hub_documents`, `sync_bootstrap_progress`, `sync_reseed`, `last_uncommitted_edit_at_ms`, `auto_checkin_pending`, `checkpoint_dispatched`, `checkin_dialog_draft`, `hub_check_in` |
| `DocumentState` | `shell-docs` | 11 | `session`, `window_ui`, `panel_documents`, `spawned_ui`, `spawned_session`, `app_document_identities`, `closing_documents`, `window_actions_documents`, `window_search_documents`, `window_projection_documents`, `window_measures_documents` |
| `OpeningState` | `shell-doc-open` | 12 | `uri_history`, `uri_index`, `open_space_id`, `pending_shell_uri_apply`, `pending_file_opens`, `import_transfers`, `import_transfer_next`, `last_envelope_dsl`, `document_services`, `document_opening`, `icon_export`, `document_execution_target_lease` |
| `WindowState` | `shell-windows` | 32 | `active_window_id`, `noted_active_window`, `host_event_active_window`, `engagement_expanded`, `engagement_activated`, `measures_folded`, `measures_expanded`, `measures_width`, `measures_resize_origin_width`, `engagement_inputs`, `measures_resize_window_id`, `window_topology_actions`, `window_topology_publications`, `window_topology_action_tokens`, `next_window_topology_action_token`, `window_topology_refresh_owed`, `window_topology_journal_dispatch_owed`, `active_utilities`, `active_utility_by_window`, `action_panel_folded`, `search_panel_folded`, `utility_bar_folded`, `projection_pane_folded`, `world_projection_template`, `window_icon_overrides`, `window_title_overrides`, `action_panel_expanded`, `tool_measures`, `window_engagements`, `window_measures_minted`, `utility_collection_expanded`, `contributor_instances` |
| `ShellDockState` | `shell-dock` | 36 | `panel_anchors`, `dock_tabs`, `dock_override`, `dock_collapsed_branches`, `dock_tab_drag`, `dock_loaded_app_id`, `panel_resize_origin_width`, `dock`, `split_resize_path`, `split_resize_index`, `split_resize_axis_total`, `dock_drag`, `pending_dock_drag`, `dock_view`, `dock_instance_owner`, `dock_input_identity`, `tree_windows`, `tree_window_served`, `dock_canvas_bounds`, `dock_axis_separator`, `dock_drop_tab_bars`, `dock_drop_bodies`, `dock_window_plan`, `dock_plan_trace`, `layout_override`, `split_resize_origin`, `split_resize_secondary_path`, `split_resize_secondary_index`, `split_resize_secondary_axis_total`, `split_resize_secondary_origin`, `layout_save_label`, `user_layouts`, `mobile_panel_visible`, `mobile_panel_path`, `window_content_rects`, `window_silhouettes` |
| `ShellInputState` | `shell-input` | 30 | `scroll_offsets`, `overlay_state`, `collapsed_sections`, `tree_open_states`, `accessibility_focused_control_id`, `interaction_selection`, `open_selects`, `context_menu`, `right_click`, `tree_drag`, `tree_hovered_id`, `widget_maps`, `widget_maps_staging`, `pending_tree_drag`, `tree_drag_origin`, `pending_chrome_release`, `chrome_pointer_owner`, `keybinding_capture_control_id`, `retained_hit_windows`, `retained_hit_windows_staging`, `retained_scene_hits`, `retained_scene_hits_staging`, `presented_input_epoch`, `next_presented_input_epoch`, `presented_input_candidate`, `presented_input_geometry`, `presented_input_geometry_staging`, `retained_body_rects_staging`, `pane_overlay_hits`, `retained_hover_window` |
| `CommandState` | `shell-commands` | 11 | `search_open`, `find_open`, `search_query`, `search_selected`, `find_query`, `find_items`, `find_selected`, `staged_action_args`, `expanded_command_id`, `staged_command_args`, `search_possibles_open` |


Names of structs that already exist in the tree are kept distinct on purpose (`ShellDockState` not `DockState`, `ShellInputState` not `InputState`).

### 7.4 Method homing (algorithm, so WP4/WP5 reproduce it)

1. For each of the 701 methods compute `C(m)` = components whose fields `m` reads or writes (`self.f` with `f` in the table) plus the method's majority component (call-graph majority for methods with no field access).
2. If the method name begins `render_`, `paint_`, `draw_` or is `chrome_accessibility_*`, `footer_*`, `navbar_*`, or ends `_paint_ops` / `_accessibility_nodes` / `_chrome_layout`, it is a **view** method: it goes to the view crate of its source file's theme (frame/navbar/main window: `shell-view-chrome`; notices, dialogs, tour, tooltips, history/folder bands: `shell-view-overlays`; settings/display/command panels, theme editor, window panes: `shell-view-panels`).
3. Else if `|C(m)| >= 4` or `C(m)` is empty, it is a **use-case** method in the use-case crate of its source file (`session-and-topology`, `settings-and-dock-persistence`, `time-travel`, `tutorial-runtime`, `local-folders`, `icon-export`, `dock-model-and-measures`, `tree-windows`, `storage-prefs-and-i18n` → `shell-session`; `shell-actions`, `command-dispatch` → `shell-actions`; `window-actions-and-commands` → `shell-window-actions`; `pointer-input`, `keyboard-and-palette` → `shell-pointer`).
4. Else it goes to the **highest-ranked component in `C(m)`**; the other components of `C(m)` become parameters.
5. Fixed point: while a method calls (by `self.name(…)`, unique name, not a port) a method of a higher-ranked crate, lift the caller to that crate.
6. Ports (7.5) are not edges in step 5.

Result with the table order of 7.3 (methods / lines), tier 1 components: chrome-state 18 / 326, surfaces 29 / 582, plugins 18 / 356, history 6 / 45, prefs 37 / 695, directory 42 / 797, sync 17 / 434, docs 73 / 1,570, doc-open 23 / 505, windows 35 / 549, dock 103 / 1,791, input 70 / 1,463, commands 19 / 577; tier 2: session 35 / 1,215, window-actions 33 / 1,353, actions 30 / 2,229, pointer 30 / 2,229; tier 3 views: chrome 34 / 1,758, overlays 24 / 1,657, panels 20 / 1,387. Lifted by step 5: 1,781 lines of the tier-1 total (mostly `dock` 681, `docs` 420, `prefs` 199), 2,334 lines of tier 2 and 1,366 of the views; **the full machine assignment of every method is Appendix B**. If a crate overflows its budget while carving, its highest-fan-in lifted methods spill to the next crate up (always safe).

### 7.5 Ports (the only upward seams)

| port | what the lower code does today | replacement | callers |
|---|---|---|---:|
| P1 `refresh_ui(scope)` / `refresh_ui_rendered` (34 + 194 lines) | components `.await` a full document refresh after mutating state | `SettleState::owe_refresh(scope)` (already exists, 10 callers); the settle driver (`advance_owed_refresh`, `settle_ui_chain`, `drain_deferred_actions`; homed in `shell-pointer`) performs the refresh by calling `refresh_ui_rendered` (homed in `shell-window-actions`; the 34-line wrapper `refresh_ui` moves with it, the tool had homed it in `shell-plugins` only because it touches no field). Callers that read the refreshed documents immediately move up with it. | 20 |
| P2 `dispatch_action` (622 lines, `shell-actions`) | components dispatch an action descriptor directly | push onto `SettleState::deferred_actions` (field exists); `drain_deferred_actions` in `shell-pointer` dispatches. | 27 |
| P3 `queue_host_effects` (94) | write `pending_extension_invocations`, `deferred_actions`, chrome notices | parameters: `&mut PluginState`, `&mut SettleState`, `&mut ChromeState`; callers up to use-case | 9 |
| P4 `sync_dock` (77) | rebuilds dock from session | moves to `shell-dock` with `&DocumentState`, `&WindowState` params | 9 |
| P5 `retire_one_surface_document` (11) | retire a surface's UI document | `shell-docs` method, called with `&mut DocumentState` | 9 |
| P6 `note_refused_open` / `show_transient_notice` / `note_dispatch_fault` / `band_tongue` | write `chrome_build` | `ChromeState` methods (rank 0) | 18 + 9 + 5 + 8 |
| P7 `active_locale` / `active_terminology` | read `locale_id` / `terminology_id` | `PrefsState` methods; every higher component gets `&PrefsState` | 59 + 46 |
| P8 `anchor_state` / `anchor_open` / `anchor_state_mut` | read `panel_anchors` / `dock_tabs` | `ShellDockState` methods | 25 + 21 + 9 |
| P9 `directory_ctx` / `directory_command_ctx` | directory handles | `DirectoryState` methods | 12 + 9 |
| P10 `ensure_local_folder_bindings` | `local_folder_bindings` | `HistoryState` | 5 |

P1 and P2 are the real inversions (behavioural change: the call becomes deferred). The rest are ordinary lowering: the callee already touches one component and is simply homed there (they are listed because their fan-in decides the rank order).

### 7.6 Free items that name `ShellState` (must take narrow parameters)

Today these 21 items (893 lines) take `&ShellState`/`&mut ShellState` and are the knot of 5.1. Replace the parameter by the components they read:

| item | lines | narrow parameters |
|---|---:|---|
| `IconExportBatch` impl (`📤️icon-export`) | 305 | `&mut OpeningState` (owns `icon_export`), `&DocumentState`, `&SettleState` |
| `UiPrefsSnapshot` impl, `persist_ui_preferences`, `commit_ui_preferences_write` | 16 + 63 + 9 | `&PrefsState` |
| `host_storage_write_through`, `semio_wgpu_set_host_storage`, `semio_wgpu_set_agent_bridge_config` (wasm exports) | 17 + 9 + 22 | `&PrefsState` / `&mut DirectoryState` (the agent bridge offer scope) |
| `download_media_export`, `download_media_export_worker`, `request_file_open` | 26 + 13 + 25 | `&mut OpeningState` (import transfers), `&mut ChromeState` (notices) |
| `chrome_tour_layout` (126), `chrome_tour_anchor_rect` (16), `impl ShellChromeBuildState` (45) | 187 | `&ChromeState` + `&WindowState` + `&ShellDockState` |
| `tutorial_*` (8 fns) | 201 | `&mut TutorialState`, `&SurfaceState`, `&DocumentState` |
| `run_smoke_with_axes` (`native-probe`), `boot_runtime`, `BrowserRendererBootstrap` | 50 + 134 + 162 | take `ShellState` by value as the composer, which is legitimate: they sit above `shell-state` |

### 7.7 Shell crates, sizes and order

Estimated lines = method lines + free-item lines (types, constants, helpers that stay with the component) + 2.4 lines per field. Free-item lines are assigned by source file as listed in Appendix C; treat the totals as plus or minus 12 percent and re-measure.

| # | crate (`semio-framework-os-renderer-wgpu-…`) | tier | fields | methods (count / lines) | free items + types (lines, est.) | est. total lines | WP |
|---:|---|---|---:|---:|---:|---:|---:|
| 1 | `shell-vocab` | 1 leaf | 0 | 0 / 0 | 1030 | 1030 | 4 |
| 2 | `shell-async` | 1 leaf | 0 | 0 / 0 | 1327 | 1327 | 4 |
| 3 | `shell-chrome-state` | 1 component | 11 | 18 / 326 | 1522 | 1848 | 4 |
| 4 | `shell-surfaces` | 1 component (3 structs) | 21 | 29 / 582 | 639 | 1221 | 4 |
| 5 | `shell-plugins` | 1 component | 14 | 18 / 356 | 1560 | 1916 | 4 |
| 6 | `shell-history` | 1 component | 11 | 6 / 45 | 1026 | 1071 | 4 |
| 7 | `shell-prefs` | 1 component | 13 | 37 / 695 | 1361 | 2056 | 4 |
| 8 | `shell-directory` | 1 component | 28 | 42 / 797 | 837 | 1634 | 4 |
| 9 | `shell-sync` | 1 component | 18 | 17 / 434 | 103 | 537 | 4 |
| 10 | `shell-docs` | 1 component | 11 | 73 / 1570 | 426 | 1996 | 4 |
| 11 | `shell-doc-open` | 1 component | 12 | 23 / 505 | 242 | 747 | 4 |
| 12 | `shell-windows` | 1 component | 32 | 35 / 549 | 110 | 659 | 4 |
| 13 | `shell-dock` | 1 component | 36 | 103 / 1791 | 1120 | 2911 | 4 |
| 14 | `shell-input` | 1 component | 30 | 70 / 1463 | 525 | 1988 | 4 |
| 15 | `shell-commands` | 1 component | 11 | 19 / 577 | 1061 | 1638 | 4 |
| 16 | `shell-state` | 2 composer | 0 | 0 / 0 | 450 | 450 | 4 |
| 17 | `shell-session` | 2 use-case | 0 | 35 / 1215 | 0 | 1215 | 5 |
| 18 | `shell-window-actions` | 2 use-case | 0 | 33 / 1353 | 0 | 1353 | 5 |
| 19 | `shell-actions` | 2 use-case | 0 | 30 / 2229 | 0 | 2229 | 5 |
| 20 | `shell-pointer` | 2 use-case | 0 | 30 / 2229 | 123 | 2352 | 5 |
| 21 | `shell-view-paint` | 3 view | 0 | 0 / 0 | 1919 | 1919 | 5 |
| 22 | `shell-view-chrome` | 3 view | 0 | 34 / 1758 | 190 | 1948 | 5 |
| 23 | `shell-view-overlays` | 3 view | 0 | 24 / 1657 | 0 | 1657 | 5 |
| 24 | `shell-view-panels` | 3 view | 0 | 20 / 1387 | 0 | 1387 | 5 |
| | **total** | | 248 | 701 / 21518 | | 37089 | |


Order of crates inside tier 1 is the rank order of 7.3; `shell-vocab` and `shell-async` sit below all of them (no `ShellState` knowledge). `shell-state` depends on all 13 components; tier 2/3 crates depend on it and on the tier-1 crates they name. Tier-2/3 internal order (call-graph evidence): `shell-session` < `shell-window-actions` < `shell-actions` < `shell-pointer` < `shell-view-paint` < `shell-view-chrome` < `shell-view-overlays` < `shell-view-panels`. The one call that disagrees (`shell-actions` → `shell-window-actions`, 15 calls versus 8 in the other direction) is resolved by moving 7 callees (`apply_mutations`, `observe_invocation_history`, `attach_sync_backbone`, `open_service_port`, `dispatch_command`, `switch_to_session_role`, `apply_shell_uri`, 246 lines) down into `shell-window-actions`; `dispatch_action` is port P2.

### 7.8 Other large impls (explicit)

| type | impl lines (files) | decision |
|---|---|---|
| `KernelPoolState` | 1,898 + 69 (1 file) | the struct (54 lines, **30 fields**) is defined in `🗃️kernel-retained-state` but the impl is in `🏊️kernel-pool`: the struct moves to `kernel-pool`, which then holds struct and impl whole (about 2,230 lib lines). Its fields group into six state machines that can become component structs inside the crate if it ever needs to split: request (`request_queue`, `serving`, `interleaving`), runtime (`guest_runtime`, `runtime`, `worker_count`, `now_ms`, `compiled_components`), instances (`plugin_ordinals`, `instances`, `next_instance_id`, `closing_apps`, `fault_closing_actors`, `realm_progress_close_started`), retained documents (`retained`, `pending_rejections`, `retained_command_closes`, `queued_command_closes`, `command_document_closes`, `rejected_command_builds`, `rejected_events`, `semantic_close_document_lane`), job progress (`job_progress`, `rejected_job_progress`, `pending_job_progress_presentations`), replay (`replay_routes`, `job_replays`, `reserved_jobs`, `product_replay_claims`, `product_replay_authorities`). No cross-crate split is needed. |
| `RuntimeMailbox` | 1,602 (1) | stays whole in `runtime-mailbox` (made generic by I2). |
| `AppPresenter` | 815 (1) | whole in `frame-presentation`; `close_world3d_dynamic_step` joins it (I3). |
| `FrameTransaction` | 706 (1) | whole in `frame-transaction` (its struct and impl are both in the `🧾️frame-build` file). |
| `AppRuntime` | 696 (2: `🧾️frame-build` 100 in three blocks, `🧠️app-runtime` 596) | all blocks merge into `app-runtime`. |
| `AppInteractionState` | 481 (1) | `app-runtime`; holds a `ShellState` (composer), legitimate. |
| `OsHost` | 345 (2 files, 4 blocks: `🏠️os-host` 129, `🪟️winit-app` 55 + 69 + 92) | both blocks merge into `host-winit`; `os-host` and `winit-app` are one crate for this reason. |
| `DockState` | 630 (1) | `dock` crate. |
| `InkInteractionJob` | 568 (1) | `scene-ink`. |
| `PanelProjection` | 509 (1) | moves with `PanelRecord` into `shell-plugins`. |
| `BrowserRendererWorker` | 491 (1) | `browser-worker`. |
| `ChromeDialogRequest` | 430 (1) | `shell-chrome-state`. |
| `ProgramBridgeEntry` / `EngineSurfaceRetirement` | 428 / 408 (1) | stay in their element crates. |
| `ShellChromeBuildState` | 175 (2 files: `🗨️chrome-overlays-and-tour` 130 in three blocks, `🖱️pointer-input` 45) | the 45-line block in pointer-input takes `ShellState`; rewrite onto `ChromeState` + params (7.6) and merge into `shell-chrome-state`. |

Everything else with a multi-file impl does not exist: the scan lists no other type whose inherent impl spans files or crates after the five earlier splits.

## 8. Public surface and wasm exports

* **Rust**: `pub use native_entrypoint::run_native_entrypoint` (native). That is all the repository consumes (`✏️s/🧑‍💻dev/💡️services/🦀️.rs`, `T/⌨️native-entrypoint/💾️binary/🦀️.rs`). `T/🧩️component/🦀️.rs` does `extern crate semio_framework_os_renderer_wgpu as _;` to link the exports.
* **wasm exports (47)**. Free functions: the logic stays in the owning crate as a plain `pub fn`; the `#[wasm_bindgen(js_name = …)]` wrapper is a one-line function in the top crate (`wasm_exports` module) so lower crates do not depend on `wasm_bindgen`'s attribute. The two classes keep their `#[wasm_bindgen] impl` in `browser-worker` (an inherent impl cannot move) and the top crate does `pub use` them.

| js_name | wrapper calls | owner crate |
|---|---|---|
| `semioWgpuSetHostAppearance`, `…SetHostIntroductionSuppressed`, `…SetHostPlatform`, `…SetHostLocale`, `…SetRuntimeDiagnostics`, `…SetBootDescriptor`, `…SetPluginCatalog` | `set_host_appearance`, … | `host-base` |
| `semioWgpuSetHubEnv` | `shell_hub_env` | `shell-directory` |
| `semioWgpuSetHostStorage`, `semioWgpuSetAgentBridgeConfig` | storage door / offer scope | `shell-prefs` / `shell-directory` |
| `semioWgpuHostWindowBlur` | `note_host_window_blur` | `shell-pointer` |
| `semioWgpuHostPageHidden` | text editor page-hidden | `engine-text` |
| `dumpStructure`, `dumpFrameStats`, `dumpMeshStats`, `dumpBoard2d`, `dumpChrome`, `dumpAccessibility` | `dump_*` | `ui-engine-dump` |
| `uploadIconAtlas` | `upload_icon_atlas` | `native-probe` |
| `BrowserRendererWorker` (24 methods: `admitOriginalFrame`, `originalFrame*`, `acknowledgeOriginalFrameOutput`, `pollAssetRequest`, `assetResponseCurrent`, `begin/push/sealReferenceImage`, `reserve/push/seal/abort/rejectAssetResponse`, `enqueueBatch`, `closeOriginalFrame(Context)`, `assetDecodeStep`, `closeStep`), `BrowserRendererBootstrap` (`bootShell`), `semioWgpuWorkerBootstrap` | in place | `browser-worker` |

Verification of export parity (WP0): extract the exported name list from the built wasm module's `.d.ts` before and after; the 47 attributes' names must be identical. Linker-pruning risk: exports defined in a dependency rlib survive only if the crate is linked; the top crate keeps `extern crate` / `pub use` lines for every crate that owns an export, and the component crate keeps its `extern crate … as _`.

## 9. Tests

### 9.1 What there is

147 test files, 55,000+ active lines in the native test closure (157,068 − 101,831), 218 embedded JSON fixtures (1.36 MB), 5 embedded `.rs` of other crates (640 KB), 95 `include_str!("….rs")` source-text embeds in total, and five `*_SOURCE` concatenation consts (`RENDERER_SOURCE` 9 readers, `SHELL_SOURCE` 12, `SCENES_SOURCE` 2, `ENGINE_CANVAS_SOURCE` 3, `INTERPRETER_SOURCE` 2). Shell tests are 56 files / 29,000 lines (the biggest: shell-input 2,727, ui-command-wiring 2,594, async-boundary 2,612, time-travel 2,033, hub-projection 1,382, dock unit 1,378, shell-chrome-parity 1,271).

### 9.2 Rules

1. **Placement**: a test lives in the highest-ranked crate whose names it uses heavily (60+ lines or 15 percent of its references), together with its host crate. If that crate cannot hold it inside its budget, it moves to a companion `x-tests` crate placed right above the crates it exercises.
2. **Visibility**: tests that used `super::*` private access get `pub(crate)`→`pub` on exactly the items they name (the oracle's definition scan finds them); never `#[cfg(test)] pub` shims.
3. **Source-text laws do not embed.** A test that pins where code calls what (`RENDERER_SOURCE` & co, `include_str!("….rs")`) reads the files at run time through one helper, `read_module_tree_source(root_rs)`, in a dev-only crate `semio-framework-os-renderer-wgpu-test-support` (about 150 lines, no dependencies; it follows `#[path]` / `mod` like the TS helper `readModuleTreeSource`). Laws whose subject is another crate's source (`draw`, `prepared`, `gpu`, `directory schema`, `draw-unit`: 640 KB) move to the TS test of the owning module. The five `*_SOURCE` consts are deleted. This removes 95 embeds and about 1.1 MB of embedded source from test closures (the async-boundary test alone embeds 1,068 KB through 26 files).
4. **Fixtures** stay beside their tests; count them (each is a file and its bytes). Tests with more than 10 fixtures or 100 KB of fixtures are the byte-tight ones: `canvas2d-host` (path-paint / gumball, 190 KB), `engine-attach` (230 KB with its tests), `hub-contracts` (218 KB), `frame-job` / `runtime-mailbox` before rule 3.
5. **Global allocator**: `#[global_allocator] RENDERER_HEAP_WITNESS` stays in the top crate and in each companion that wants the witness.

### 9.3 Placement (computed; unit tests stay in the crate, listed counts include helpers/fixtures)

| crate | in-crate unit tests (test module / lines) | fits? |
|---|---|---|
| `host-base` | 🧭️boot-axis-parity 205, ⏱️frame-latency-diagnostics 123, 🔬️unit 57, 🧪️tests 40 | yes |
| `native-platform` | 🌐️icu 286, ♿️native-accessibility 209 | yes |
| `media-slots` | 🔬️wgpu-media-slots-unit 213, 🧪️tests 22, 🧪️tests 19 | yes |
| `canvas2d-paint` | 🧪️path-paint 180 | yes |
| `asset-authority` | 📤️asset-delivery 54 | yes |
| `doors` | 🔌️wgpu-socket-door 218 | yes |
| `kernel-protocol` | 🗞️typed-result-page 124 | yes |
| `kernel-runtime` | 🔬️wgpu-renderer-kernel-runti 1403 | yes |
| `engine-graph` | 🧊️wgpu-standalone 521, 🔬️wgpu-catalogue-workflow-dr 80 | yes |
| `engine-attach` | 🖌️wgpu-paint2d-engine 1087, 🔬️wgpu-board2d-engine 391, 🕸️wgpu-node-graph 388, 🫳️node-graph-gestures 355, 🩺️metrics-invalidation-drain 43 | NO -> companion |
| `scene-state` | 🔬️wgpu-admitted-surface-map 159 | yes |
| `canvas2d-host` | 🧪️wgpu-gumball-dispatch 99 | yes |
| `scene-ink` | 🧊️wgpu-standalone 1121, 🔬️wgpu-raster-frame-cost 388, 🔬️wgpu-ink-canvas 381 | NO -> companion |
| `scene-feeds` | 🔬️wgpu-event-feed 477, 🔬️wgpu-graph-timeline 385 | yes |
| `scene-tables` | 🔬️wgpu-diff-view 236 | yes |
| `scene-text-vfs` | 🔬️wgpu-text-editor 450, 🔬️wgpu-virtual-file-system 269 | yes |
| `icon-host` | 🔬️wgpu-icon-render 240, 📤️svg-export 184, 📤️gpu-export 107, 🔬️wgpu-unit 85, 📤️png-export 59 | yes |
| `scene-render` | 🔬️wgpu-canvas2d 656, 🔬️wgpu-table 525, 🔬️wgpu-block-list 305, 🔬️wgpu-surface-context-menu 216, 🔬️wgpu-render-entry 12 | tight (3332 l / 198 KB) |
| `ui-engine-dump` | 🔬️wgpu-introspection 444 | yes |
| `ui-engine` | 🧩️wgpu-engine-surfaces 1179, 🔬️wgpu-render-plan-validator 263, 🛍️wgpu-app-catalogue 159, 🧪️tests 68, 🎟️original-turn 48 | NO -> companion |
| `hub-contracts` | 🔬️wgpu-unit 335, 🔬️wgpu-unit 298 | tight (1931 l / 218 KB) |
| `hub-workspace` | 🔬️wgpu-unit 796, 🔬️wgpu-unit 138, 🔬️wgpu-unit 128 | yes |
| `agent-elements` | 🔬️wgpu-unit 653, 🔬️wgpu-unit 197, 🔬️wgpu-unit 91 | tight (3027 l / 172 KB) |
| `dock` | 🪟️app-mode-layouts 303, 🎯️stack-drop-destination 50, 🪟️window-template-drag 49 | yes |
| `program-bridge` | 🕹️wgpu-reserved-verb-answer 105, 📏️wgpu-window-measures-secti 70, 📦️frame 50, 📡️wgpu-document-backbone-eff 41, 🎬️wgpu-window-engagements-se 33 | yes |
| `mailbox-core` | 🎮️wgpu-runtime-mailbox-admis 212, 🔬️wgpu-runtime-mailbox-core- 52 | yes |
| `host-plumbing` | 🎮️wgpu-browser-input-wire 196, 🔬️wgpu-deadlines-unit 100, 🔬️wgpu-render-snapshot-unit 89, 🔬️wgpu-surface-lane-unit 84, 🧪️tests 27, 🧪️tests 26 | yes |
| `frame-actions` | 🧾️frame-action-ledger 256, 🖱️wheel-application-point 45 | yes |
| `runtime-mailbox` | 🔬️wgpu-renderer-async-bounda 2612, 📥️enqueue 97 | NO -> companion |
| `frame-presentation` | 🐕️wgpu-present-stall-watch 110 | yes |
| `frame-job` | 🔬️wgpu-frame-job-unit 435, 🧪️tests 14 | yes |
| `native-probe` | 🔬️wgpu-renderer-renderer-io- 221, 🧪️tests 152, 🧊️wgpu-renderer-standalone 91 | yes |
| `host-winit` | 🧊️wgpu-os-host-standalone 674, 🔬️wgpu-winit-app-p3c 187, 🔢️frame-generation-hold 172, 🔬️wgpu-winit-app-callback-la 82 | yes |
| `entrypoint` | 🎨️paint-policy 113, 🎨️wgpu-paint-policy 53 | yes |


### 9.4 Companion test crates

Non-Shell (4): `engine-attach-tests` (paint2d-engine 1,087, board2d-engine 391, node-graph 388, gestures 355, metrics-drain 43), `scene-ink-tests` (the 1,121-line EngineCanvas/Scenes standalone slice, raster-frame-cost 388, ink-canvas 381), `ui-engine-tests` (engine-surfaces 1,179, render-plan-validator 263, app-catalogue 159, worker-cell 68, original-turn 48), `runtime-mailbox-tests` (async-boundary 2,612 with run-time source reads, enqueue 97). Plus the `kernel-retained` law moves into `kernel-runtime`'s own test mount (1,403 lines + 89 KB; 2,316 lines / 146 KB with the facade and scale bench), and `ui-command-wiring` (2,594), `shell-input` (2,727) and `time-travel` (2,033) are Shell bins because they name `ShellState`. Shell (14), packed by theme **[WP0 change]** (the planner wrote 12; the groups listed below and Appendix D are 14) to at most 3,300 lines / 215 KB / 26 files (Appendix D): input (3 bins), documents/sync/directory (3 bins), chrome (4 bins plus 2 for the Dock unit test and `ui-command-wiring`), windows (2 bins).

## 10. Work packages and hand-off protocol

### 10.1 Packages (disjoint source file sets; one executor each)

| WP | scope | crates | owns (source files) | est. lib lines | unit-test lines |
|---|---|---|---|---:|---|
| 0 | WP0 infrastructure (serial, first and last) | all 87 skeletons, registries, mount hub, test-support crate; later: top crate `entrypoint`, TS readers, locks, manifest, census, export parity | `PKG/**`, `.vscode/*`, `📋️build-owners.json`, workspace + lock files, `R/🦀️.rs`, `T/⌨️native-entrypoint/🦀️.rs`, `T/🧩️component`, TS readers | - | - |
| 1 | WP1 foundation, kernel, small elements | contracts, host-base, native-platform, media-slots, canvas2d-paint, asset-authority, asset-decode, doors, kernel-protocol, kernel-retained, kernel-pool, kernel-runtime, hub-contracts, hub-workspace, agent-elements, dock, program-bridge, mailbox-core | `R/🦀️.rs` (dissolved into `host-base`), `R/📇️registry`, `T/📇️catalog`, `T/🔤️runtime-assets`, `T/⏱️frame-latency`, `🖼️assets/🥽️mesh`, `T/🧬️element-contracts`, `T/🎬️media-slots/**`, `T/🕰️native-temporal/**`, `T/♿️native-accessibility`, `T/📮️runtime-mailbox-core`, `R/🗂️asset-authority`, `R/🦴️glb-decode`, `R/🔁️asset-decode-jobs`, `T/🔌️socket-door`, `T/📇️directory-door`, `R/📦️kernel-runtime/**`, `R/📊️scale-bench`, `T/🎠️runtime`, `E/🧱️elements/{🛰️Dock,🔐️HubSignIn,🏘️SpaceBrowser,🔗️HubConnection,🛂️SpaceAdministration,👕️canvas-presence,🔗️AgentBridge,🚦️AgentPresence,🤖️AgentApprovals,💬️AgentChatPanel,🌉️ProgramBridge}`, `T/🌉️agent-bridge-door`, `T/⌨️native-entrypoint/📦️modules` | 25010 | 6754 |
| 2 | WP2 EngineCanvas, Scenes, Icon, Canvas2d | engine-registry, engine-text, engine-board, engine-graph, engine-attach, scene-state, canvas2d-host, scene-canvas2d, scene-ink, scene-feeds, scene-tables, scene-text-vfs, icon-host, scene-render | `E/🧱️elements/{⚙️EngineCanvas,🎞️Scenes,🖼️IconRenderHost,📐️Canvas2dHost}/**` and their `🧪️tests` | 22688 | 9219 |
| 3 | WP3 UI engine, frame core, host | ui-engine-focus, ui-engine-dump, ui-engine, host-plumbing, frame-actions, runtime-mailbox, app-runtime, frame-presentation, frame-transaction, frame-job, native-probe, host-winit, browser-worker | `E/🧱️elements/🗣️Interpreter/**`, `R/{🛎️frame-actions,🎨️presentation-raster,📮️mailbox-impl,🧾️frame-build,🖥️frame-presentation,🧠️app-runtime,🔬️native-socket-probe,🪶️sqlite}`, `T/{🧵️frame-job,🏠️os-host,🪟️winit-app,🌐️browser-worker,📐️surface-lane,📸️render-snapshot,🎟️native-caller,⏰️deadlines,🎮️input-wire,🪙️authority}`, root-level `E/🧪️tests` renderer laws | 19604 | 7997 |
| 4 | WP4 Shell components | shell-vocab, shell-async, shell-chrome-state, shell-surfaces, shell-plugins, shell-history, shell-prefs, shell-directory, shell-sync, shell-docs, shell-doc-open, shell-windows, shell-dock, shell-input, shell-commands, shell-state | `E/🧱️elements/{🐚️Shell,🏛️ShellHost}/**` until step S0 (below) hands the tier-2/3 staging files to WP5; `🖱️ui/…/🌓️theme/📐️geometry` | 23029 | component-level Shell test bins |
| 5 | WP5 Shell use-cases and views | shell-session, shell-window-actions, shell-actions, shell-pointer, shell-view-paint, shell-view-chrome, shell-view-overlays, shell-view-panels | tier-2/3 staging files produced by S0 | 14060 | input and chrome Shell test bins |


### 10.1b Order of work inside each package

* **WP1.** (1) `contracts` first (it receives lifted items from WP2, WP3, WP4 and its own `element-contracts`; keep `UiEngineCell` and the close queue as one unit). (2) `host-base`: dissolve `R/🦀️.rs` (host statics, boot descriptor, logging, catalog, runtime assets, mesh assets, frame latency, `renderer-io`), replacing the 123-name import block crate by crate as the other packages need names. (3) `native-platform`, `media-slots`, `canvas2d-paint`, `mailbox-core`, `doors`, `asset-authority`, `asset-decode`. (4) The kernel group in order `kernel-protocol` (inversion I1), `kernel-retained`, `kernel-pool` (take `struct KernelPoolState` from the retained-state file), `kernel-runtime` (+ semantic-document law). (5) The element families `hub-*`, `agent-elements`, `dock`, `program-bridge`. Tight: `hub-contracts` (218 KB with tests).
* **WP2.** (1) `scene-state` (largest landing site: 16 source files, 670 moved-in lines; land it before any `scene-*` leaf is compiled). (2) `engine-registry`, `engine-text`, `engine-board`, `engine-graph`, `engine-attach`. (3) `canvas2d-host`, `scene-canvas2d` (+ ui-image loader), `scene-ink`, `scene-feeds`, `scene-tables`, `scene-text-vfs`, `icon-host`, `scene-render`. (4) The four companion test crates for `engine-attach`, `scene-ink` (and the `ui-engine-tests` set with WP3). Tight: `engine-attach` bytes, `scene-render` (3,332 lines with tests).
* **WP3.** (1) `ui-engine-focus`, `ui-engine-dump`, `ui-engine` (needs `contracts` and the scene crates' APIs only). (2) The frame core in dependency order: `runtime-mailbox` first and generic (inversion I2, plus `RuntimeApply` and the asset stream moved in), then `frame-actions`, `host-plumbing`, `app-runtime` (implements the two mailbox traits; absorbs `AppFramePreparation`, `AppFramePresentation`, `impl AppFrameBuild`), `frame-presentation` (gains `close_world3d_dynamic_step`, I3), `frame-transaction`, `frame-job`. (3) `host-winit` (merge the `OsHost` impl blocks), `native-probe`, `browser-worker`. (4) `runtime-mailbox-tests` and `ui-engine-tests`, and the `test-support` source-reading helper (used by every package's source-text laws). Tight: `app-runtime` (142 KB lib bytes), the async-boundary law.
* **WP4.** (1) Step S0 (below). (2) Tier 1 in rank order `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`; each crate lands with its component struct, constructor and invariants before its methods. (3) The composer `shell-state` last. (4) The `docs-*` and `windows-*` test bins. Tight: `shell-dock` (2,911 estimated lines); apply the spill valve of 7.4 before squeezing.
* **WP5.** (1) The four bundles (`SessionCtx`, `WindowActionsCtx`, `ActionsCtx`, `PointerCtx`) and ports P1/P2 against the frozen component API. (2) `shell-session`, `shell-window-actions`, `shell-actions`, `shell-pointer`. (3) `shell-view-paint`, `shell-view-chrome`, `shell-view-overlays`, `shell-view-panels`. (4) The `input-*` and `chrome-*` test bins (the largest rewrite: `shell-input` 2,727, `ui-command-wiring` 2,594, shell-chrome-parity 1,271 lines).

### 10.2 Protocol (nobody edits another package's files; registries are edited once)

**Phase 0 (WP0, serial, before anyone else starts)** **[WP0 change]** **executed 2026-10-15; what it created and the rules that follow are in 10.4:**
1. Generate all 88 new crate skeletons (89 crates with the top crate) in one pass: `Cargo.toml` (name, workspace depth, `[lib] path = "../../🦀️.rs"`, `[package.metadata.semio.rust-source-boundary]` with the recipe's exact three keys at the ceiling values `maxSourceBytes = 262144`, `maxSourceFiles = 32`, `maxActiveLines = 4000`; the internal target 3,500 / 28 / 230,000 is enforced by the census, not by the manifest), `📋️project.json`, stub `🦀️.rs` with its docstring, `🧪️tests/🦀️.rs` stub, workspace member (byte-sorted), `📋️build-owners.json` row (lexical), six launch routes in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`, `Cargo.lock` refresh through the sanctioned launcher. All crates declare *no* dependencies yet except those listed in this plan's `deps` column (so the DAG is already cycle-checked by `cargo metadata`).
2. Rewrite `R/🦀️.rs` into a mount hub: the 42 module mounts and 12 renderer partitions move into five files `R/📥️mounts/<hub>/🦀️.rs` (hub folders `🌱️wp1-foundation`, `🎞️wp2-scenes`, `🧵️wp3-frame-core`, `🐚️wp4-shell-state`, `🎛️wp5-shell-views`), each owned by one package and `include!`d from the root (the module-tree helper of the TS readers follows `include!` since phase 0); the root keeps only host statics until WP1 moves them. This is the only way five executors can remove their own mounts without editing the same file.
3. Create the 18 empty companion test crates (14 Shell bins, 4 others) and the `test-support` crate **[WP0 change]** (16 and 12 were the planner's counts).
4. Freeze the interface: component struct names, field names, crate names, and the tier-2 bundle names of 7.2. Publish them in the ticket (`📌️important.md` stays empty; this plan is the contract).

**Phase 1 (WP1..WP5 in parallel):** each package carves *its* crates in this order and never touches anything outside its file set:
1. Extract moved-out items from its own source files into new files placed **in the destination crate's directory** under `📥️lifted/<source-slug>/🦀️.rs` (new unique paths, so no edit conflicts); the destination package wires them with one `mod` line. Moves *into* its crates are therefore picked up from `📥️lifted/` by the owner.
2. Move whole files to their crate directory with `#[path]`-free `mod` declarations where the file is the only member; keep `#[path]` only for the four files that live outside `E/` (section 2.3).
3. Widen visibility only as the definition scan demands (`pub(crate)` → `pub` for cross-crate), keep region markers balanced, add the explicit imports replacing the old root `use` block (section 6.3).
4. Run the per-crate gates of section 11 in the foreground and write `RESULT-W4-WGPU-WPn.md` with the census rows.

**Step S0 (WP4, first, before WP5 starts; mechanical, about one hour of scripting):** apply Appendix B to the 20 Shell files. For every (source file, target crate) pair write one staging file `T/../🐚️Shell/📥️staging/<crate>/<source-slug>/🦀️.rs` containing the assigned methods still in their original `impl ShellState { … }` wrapper plus the free items of Appendix C, and cut the same text out of the original file; the old module tree keeps compiling as one crate (the staging files are mounted by the old `shell` parent), so the oracle must report an identical multiset before and after S0 (losslessness check: the line multiset of the staging files equals that of the 20 originals). From here the staging directories are disjoint: WP4 owns those of tier 1 and the composer, WP5 those of tiers 2 and 3. Items bound for `contracts` / `host-base` / `scene-state` go to `📥️lifted/` of those crates in the same pass.

**Cross-package order constraints** (compile-level, not edit-level): WP1's `contracts` and `host-base` are imported by everyone — they are created *first inside* WP1 and merged into the skeleton by WP0 as soon as they pass the differential. WP5 codes against WP4's frozen component API and compiles after WP4 lands each crate. WP3 depends on WP2's `ui-engine` and WP4's `shell-state`; its crates compile last.

**Phase 2 (WP0, serial):** top crate (`Cargo.toml` deps, thin lib root with `pub use run_native_entrypoint`, `wasm_exports`, `extern crate` links), delete the dissolved modules and the mount hub, re-point TS readers (`POLICY_SPLIT_SOURCE_ROOTS` in `🔍️discovery/📖️source-access/🟦️.ts`, the root `📜️script.ts` readers, `readModuleTreeSource` call sites listed in `RESULT-WGPU-*` files), regenerate the distribution manifest, run census for all 87 crates, the export-parity check, the full differential on the top crate, `cargo metadata --offline`, and the lock refresh.

### 10.3 Executor rules that matter here

No `git commit/stash/checkout`, no worktrees, no sub-agents, re-read before every edit, foreground builds only, no compiling around the ingress gate, `[DEBUG] ` prefix for temporary logs, one RESULT file per package, no `ticket_close` from executors, no generated `🗑️generated` sweep.

### 10.4 WP0 hand-off contract (phase 0 executed 2026-10-15; evidence in `RESULT-RENDERER-WP0.md`)

This section replaces nothing above; it records what phase 0 actually created and freezes the rules that WP1..WP5 work under. Where it differs from 10.2 the difference is marked **[WP0 change]**. The tables of Appendix F (crates), G (hub units) and H (cross-package lifts) are generated from the live tree and are part of the contract.

#### 10.4.1 State of the tree after phase 0

* The tree is resolution-equivalent to the tree the plan was written against: still **one** compilation unit `semio-framework-os-renderer-wgpu`, nothing was moved between crates, only mounts were moved between files. Evidence: the patched full-resolution differential (native lib, `--cfg test`, `wasm32-unknown-unknown` lib and test), the mount-equivalence check (`verify_hub.py`: 68 mounts and includes, identical module name, absolute target file and cfg attributes before and after), the TS module-tree comparison and the static registry gate (`verify_static.ts`).
* **88 new crates exist** as skeletons: 45 non-Shell, 24 Shell, 4 non-Shell companions, **14** Shell test bins and `test-support`. **[WP0 change]** The plan said 87 crates and 12 Shell bins in places; Appendix D lists 14 bins and 46 + 24 + 4 + 14 + 1 = 89 crates including the existing top crate `entrypoint`. Appendix D is authoritative.
* **[WP0 change]** The renderer root is split into **five mount hubs** (`T/🧊️renderer/📥️mounts/<hub>/🦀️.rs`), spliced into the root by `include!`; the root keeps the imports, aliases, statics, boot descriptor, logging and the `RENDERER_SOURCE` const. The hub folders are `🌱️wp1-foundation`, `🎞️wp2-scenes`, `🧵️wp3-frame-core`, `🐚️wp4-shell-state`, `🎛️wp5-shell-views`. The 68 mount units (module mounts, `include!`d tests, `cfg(test)` test modules) were distributed by the work package that owns the crate the module goes to (Appendix G). A region that spans two packages (`🔖️OsHostDecomposition`) is repeated, balanced, in each hub that holds a piece (the established rule of `RESULT-REGION-RESTORE.md`).
* **[WP0 change]** The mount of `R/📇️registry` (`renderer_registration`) **stays in the library root**: a `#[macro_export]` macro defined in a module declared inside an `include!`d file is macro-expanded, and rustc then refuses `crate::action_args_json!` (error `macro_expanded_macro_exports_accessed_by_absolute_paths`, 73 occurrences). The differential found it on the first run. The `native_runtime_modules` mount moved into the WP1 hub (no macro).
* Registries are written and frozen (10.4.6): 88 workspace members, 88 owner rows, 528 launch routes in each of the two launch files, 88 names in `members-of-wgpu-target`, one transitional dependency block of 69 lines in the top crate.
* The TypeScript module-tree helper (`📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts`) now follows `include!("literal")` items (an included file stays inside the directory of the file that includes it) so `listModuleTree` and `readModuleTreeSource` keep seeing every partition below the renderer root through the hubs; new function `inspectRustModuleChildren`; tested against the installed tree-sitter grammar and against rustc dep-info (`rust-included-hub-resolves-like-the-compiler`).

#### 10.4.2 Crate folders and skeleton layout

Every crate has the same layout; the folder is `<emoji>️<crate suffix>` directly below `T/` (at most 28 bytes; the manifest path is at most 214 bytes):

```
T/<folder>/🦀️.rs                     library root (docstring; production crates also mount the test stub under cfg(test))
T/<folder>/🧪️tests/🦀️.rs            test stub (production crates only; companions mount existing test files with #[path])
T/<folder>/📦️packages/🦀️rust/Cargo.toml
T/<folder>/📦️packages/🦀️rust/📋️project.json
```

Package name `semio-framework-os-renderer-wgpu-<suffix>`, id `os-renderer-wgpu-<suffix>`, Nx project `@semio-tech/os-renderer-wgpu-<suffix>-rs`, `role = "framework"`, `[lib] path = "../../🦀️.rs"`, `workspace` = the repository root, the exact three `rust-source-boundary` keys at the ceiling values, `[lints] workspace = true`. Companion test crates and `test-support` add `publish = false`. Folder emoji are RGI, unique among the siblings and registered in the taxonomy (10.4.6). Three new folders share a slug with an old source folder (`🌍️browser-worker`/`🌐️browser-worker`, `🎥️media-slots`/`🎬️media-slots`, `👷️frame-job`/`🧵️frame-job`): the old folder is the source tree of the module, the new one the crate; the old folder disappears when its owner has moved the files in.

#### 10.4.3 Ownership (disjoint; nobody edits another package's files)

| WP | hub it owns | crate folders it owns | source trees (10.1) and the reconciliations phase 0 found |
|---|---|---|---|
| 0 | none after phase 0 | `🧯️test-support` skeleton until WP3 fills it | `PKG/**` (top crate `Cargo.toml`, `build.rs`, library root), `T/🧩️component`, registries (members, owner rows, launch routes, taxonomy names, locks), the renderer root `R/🦀️.rs` outside the statics WP1 dissolves, TS readers |
| 1 | `🌱️wp1-foundation` | contracts, host-base, native-platform, media-slots, canvas2d-paint, asset-authority, asset-decode, doors, kernel-protocol, kernel-retained, kernel-pool, kernel-runtime, hub-contracts, hub-workspace, agent-elements, dock, program-bridge, mailbox-core | 10.1 list. **[WP0 change]** also `E/🧱️elements/📐️Canvas2dHost/🎨️paint/` (canvas2d-paint is a WP1 crate whose one source file sits in WP2's element tree; WP2 must not edit that folder) and `T/🔤️runtime-assets/` (grew from 64 to 88 lines, uses `semio_framework_canvas`). WP1 may delete the single `renderer_registration` unit in the library root when `host-base` owns the macro (the only line of a WP0 file a package may edit) |
| 2 | `🎞️wp2-scenes` | engine-registry, engine-text, engine-board, engine-graph, engine-attach, scene-state, canvas2d-host, scene-canvas2d, scene-ink, scene-feeds, scene-tables, scene-text-vfs, icon-host, scene-render, engine-attach-tests, scene-ink-tests | `E/🧱️elements/{⚙️EngineCanvas,🎞️Scenes,🖼️IconRenderHost,📐️Canvas2dHost}/**` except the canvas2d-paint folder above |
| 3 | `🧵️wp3-frame-core` | ui-engine-focus, ui-engine-dump, ui-engine, host-plumbing, frame-actions, runtime-mailbox, app-runtime, frame-presentation, frame-transaction, frame-job, native-probe, host-winit, browser-worker, ui-engine-tests, runtime-mailbox-tests, test-support | 10.1 list and the root-level test mounts in the hub (`fixture_original_turn`, `async_boundary_tests`, `frame_action_ledger_tests`, `wheel_application_point_tests`, the two `include!`d standalone tests) |
| 4 | `🐚️wp4-shell-state` | the 16 Shell crates of tier 1 and the composer, plus the Shell test bins `docs-1..3` and `windows-1..2` | `E/🧱️elements/{🐚️Shell,🏛️ShellHost}/**` until step S0 hands the tier-2/3 staging files to WP5 |
| 5 | `🎛️wp5-shell-views` | the 8 Shell crates of tiers 2 and 3, plus the Shell test bins `input-1..3` and `chrome-1..6` | the staging files of step S0; the hub holds the `settle_pump_tests` mount |

Rules that follow:

1. A package edits only its own hub, its own crate folders (manifest, library root, tests, moved sources) and the source trees above. A needed edit in a file another package owns is made by that owner (or recorded in the RESULT file as a request); the one tolerated exception is the WP1 library-root line above.
2. **Moves between packages are lifts.** The package that owns the source file cuts the item into `T/<destination folder>/📥️lifted/<source slug>/🦀️.rs` (a new unique path, so there is no edit conflict) and the destination package wires it with one `mod` line in its library root. Appendix H lists the 45 item-move groups of 5.2 with their source and destination package; the 12 that cross packages are marked with the lift file to create. Lifting does not change the module the item is visible in until the destination crate exists: re-export it from the source module with `pub use` in the meantime.
3. No package creates a crate, a folder with an emoji, a workspace member, an owner row or a launch route. A package that needs one asks WP0 (rename the row in Appendix F by editing this table, not by editing the registries).

#### 10.4.4 The carve protocol with the hubs

Each hub holds the mount units of its package. **Carving a crate means:**

1. Move the module's source files into `T/<folder>/` (library root `T/<folder>/🦀️.rs`, children as `#[path]` mounts or plain files below it; keep `#[path]` out of `E/`-relative literals except for the four files that live outside `E/`, section 2.3). Keep every `//#region` marker, balanced.
2. In your hub, replace the module's unit by `extern crate semio_framework_os_renderer_wgpu_<suffix> as <old module name>;` (keep the `cfg` attributes of the unit). The remaining monolith code keeps compiling against `crate::<old module name>::…` and `<old module name>::…` unchanged, because an `extern crate … as name;` at the crate root defines `name` in the root namespace. `pub(crate) use <name>::*;` lines next to a unit stay as they are. Appendix G gives the exact replacement line for every unit that maps to one crate.
3. For a module that maps to several crates (`engine_canvas`, `interpreter`, `scenes`, `shell`, `kernel_runtime`, `asset_decode_jobs`, the frame partitions) keep the mount; the parent file of the module, which you own, becomes a thin facade that `pub use`s the new crates explicitly. Phase 2 deletes the facade after the consumers import the crates directly.
4. Everything the monolith still names from the carved crate must be `pub` there (the definition scan lists it); widen only to the narrowest visibility that works, never with a qualified path.
5. Crate-root aliases of the old renderer root (`infinite_canvas as infinite_world`, `semio_framework_os_kernel as dsl / dsl_core / protocol / store / store_sync`, `semio_framework_os_renderer_camera as camera_storage`) are declared per crate in its own library root with `extern crate … as …;` for exactly the aliases it names. The 123-name import block and the aliases of the renderer root stay in the root until phase 2: other packages' modules still reach them through `use super::*`.
6. A `#[macro_export]` macro that moves into a crate (`action_args_json!`, 71 uses) is re-exported from the root with `pub use <crate>::action_args_json;` so `crate::action_args_json!` keeps resolving.

The top crate already declares every production crate as a dependency (10.4.5), so no package edits the top crate manifest.

#### 10.4.5 Dependencies

* Each skeleton manifest declares exactly the first-party edges of the plan (6.1 `deps` column; for the Shell the field-touch evidence below) and the external crates its source names (6.2 lists plus the drift additions of 6.2, with the specs and target tables of the old manifest: `ui_wgpu` with `wgpu-engine`, `semio-framework-os-kernel` with `sync`+`ureq` on native and `sync` on wasm32, native-only crates under `cfg(not(target_arch = "wasm32"))`, the browser crates under `cfg(all(target_arch = "wasm32", target_os = "unknown"))`, `winit`/`accesskit_winit` with the Linux table). Dev-dependencies (`testkit`, `mutation-testing`) are present when the crate names the crate and has tests.
* **The rank order is the contract**: a crate may depend on any crate of a strictly lower rank (column `rank` of Appendix F, the order of 4.3 with the Shell crates in the order of 7.3/7.7). A package may add a downward edge to its own manifest. An upward edge, a cycle, or a dependency of a production crate on a companion is a defect; `verify_static.ts` fails on it. Native-only crates are reached from other crates only through the native target table; `mailbox-core` has no dependencies.
* **Shell edges.** Tier-1 components declare `contracts`, `host-base`, `shell-vocab`, `shell-async` and the lower components their methods touch. The touch sets were computed from the live Shell sources (619 of the 701 `impl ShellState` methods, homed by Appendix B, fields by 7.3): every edge points down the rank order, which confirms the rank of 7.3; the declared edges are `shell-surfaces` → `shell-chrome-state`; `shell-plugins` → `surfaces`; `shell-prefs` → `chrome-state`, `history`, `plugins`; `shell-directory` → `chrome-state`, `history`, `prefs`, `surfaces`; `shell-sync` → `chrome-state`, `directory`, `history`, `plugins`, `prefs`; `shell-docs` → `chrome-state`, `directory`, `history`, `plugins`, `prefs`, `surfaces`; `shell-doc-open` → `directory`, `docs`, `plugins`, `prefs`, `surfaces`; `shell-windows` → `directory`, `docs`, `plugins`, `surfaces`; `shell-dock` → `chrome-state`, `directory`, `doc-open`, `docs`, `history`, `plugins`, `prefs`, `surfaces`, `sync`, `windows`; `shell-input` → `chrome-state`, `dock`, `docs`, `history`, `prefs`, `surfaces`, `windows`; `shell-commands` → `doc-open`, `dock`, `docs`, `input`, `prefs`, `windows`. Free items (types, constants) may add further downward edges when the staging files of S0 are assigned; add them to your own manifest. The Shell crates declare no non-Shell edge other than `contracts` and `host-base`; the Shell names `interpreter`, `scenes`, `program_bridge`, `engine_canvas`, `dock`, the hub and agent elements heavily (`crate::interpreter` 75, `scenes` 55, `program_bridge` 46, `engine_canvas` 33 references), so WP4 and WP5 add the lower-ranked non-Shell edges they actually use (every non-Shell crate up to `program-bridge` ranks below the Shell).
* The six WP3 crates that name the shell (`frame-actions`, `app-runtime`, `frame-transaction`, `native-probe`, `host-winit`, `browser-worker`) declare `shell-state`; `app-runtime`, `frame-transaction` and `browser-worker` also `shell-input` (they name `PointerHitOwner` / `PointerCapture`). `ShellState` is taken by value there, legitimately above the composer.
* **Top crate.** `semio-framework-os-renderer-wgpu/Cargo.toml` carries one transitional block of 69 dependency lines (the 45 non-Shell and 24 Shell crates; no companion and no `test-support`): 62 under `cfg(not(target_os = "wasi"))`, 5 native-only under `cfg(not(target_arch = "wasm32"))`, `browser-worker` under the browser table, `mailbox-core` under `[dependencies]`. Phase 2 prunes the block to the entrypoint's real dependencies. `Cargo.lock` already lists all 88 packages (refreshed through the launcher by a peer and by `update --workspace --offline` at the end of phase 0); it goes stale again whenever a peer adds a crate, so run `bun repo cargo --store=shared --configuration=prototype -- update --workspace --offline` before a check that fails with `cargo fetch --locked … cannot update the lock file`.

#### 10.4.6 Registries (frozen; edited once by WP0)

* `Cargo.toml` (root workspace) `members`: the wgpu block grew from 2 to 90 rows, byte-sorted.
* `📋️build-owners.json`: 88 rows `@semio-tech/os-renderer-wgpu-<suffix>-rs`, `root` = package folder, `manifest` = its `📋️project.json`, `packages: []`, inserted as one sorted block before `@semio-tech/os-services-rs`.
* `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: six routes per crate (`🦀️ <package> 🏗️build|🔎️check|🧪️test 🛠️prototype|🚀️release`, groups `4_build`, `4_gate`, `4_test`, command `bun repo run @semio-tech/os-renderer-wgpu-<suffix>-rs:<target> --configuration=<mode> --parallel=1 -- --lib --offline`), inserted after the `semio-framework-canvas-scene` routes (a normalizer of the peers has since appended an `"env"` object to every Rust route of both files, ours included; keep it). Order keys are unused ones, identical in both files: `build` from 900.24401, `check` from 922.052, `test` from 903.592, key = base + 2 × (position in Appendix F) + (0 prototype, 1 release), in steps of 0.00001.
* `🔣️taxonomy.json`, `semanticDirectoryMemberKinds["members-of-wgpu-target"].memberNames`: the 88 folder names appended (without them the taxonomy verifier reports `directory-kind-unresolved` for each folder; with them each crate scope is clean apart from the existing `generator-activation-invalid` for the frame-worker catalogue digest). The five hub folders below `🧊️renderer` have no registration mechanism: the children of `🧊️renderer` are unregistered by every earlier split.
* A package never adds, renames or removes a row. The static gate (`verify_static.ts`) checks all of it in about 20 seconds.

#### 10.4.7 Tests

* Production crates have `🧪️tests/🦀️.rs` mounted under `cfg(test)`; put the crate's own unit tests and the language-agnostic vectors there (`🧪️tests/🧫️fixtures/`). The 9.3 placement table decides which tests stay and which go to a companion.
* Companions and the 14 Shell bins have an empty library root; the owner mounts the existing test files with `#[cfg(test)] #[path = "…"] mod …;` from where they live today (no copies, no moves). A test that names `ShellState` is rewritten against the component API of 7.2/7.3 in the bin of Appendix D.
* **`test-support`** is a skeleton. WP3 implements `pub fn read_module_tree_source(root: &std::path::Path) -> Result<String, String>` (follows `#[path]` mounts and `include!("literal")` of a `.rs` root in declaration order, below the root's directory, `🧪️tests` excluded, each file once, parent first, newline-terminated pieces) and `pub fn read_module_tree_files(root: &std::path::Path) -> Result<Vec<std::path::PathBuf>, String>`; the TS helper `readModuleTreeSource` (and its fixtures `📚️compiler/📖️syntax/🌳️module-tree/🧪️tests/🧫️fixtures/🔣️.json`, which include the hub shapes) is the independent oracle. Until it exists, source-text laws keep their `include_str!` embeds and the five `*_SOURCE` consts stay (`RENDERER_SOURCE` already lists the five hubs).
* The TS readers keep reading through the module-tree helper. Ten vitest readers (`🧾️frame-action-ledger`, `🫀️settle-pump`, `🖱️wheel-application-point`, `🎮️wgpu-browser-input-wire`, `🎮️wgpu-runtime-mailbox-admission`, `🎯️presented-input-authority`, `📮️wgpu-frame-deferred-ownership`, `🔢️frame-generation-hold`, `🛍️app-catalogue-attempt`, `🪟️action-window-scope`) fail with `ENOENT` on `E/🧫️fixtures/<name>/🔣️.json` (the fixture folders are empty in the working tree) and three (`🌳️wgpu-document-reconcile`, `🔗️hub-projection`, `🚗️driver-editor`) fail resolving `@semio-tech/framework-os-node-graph-rs`; all thirteen fail identically before and after phase 0 (24 vitest files and 20 bun test files compared), so they do not witness a carve until those environment faults are fixed.

#### 10.4.8 Toolbox, gates and what is blocked

Tools (ticket folder `rwp0-tools/`, all read-only against the repo unless named `gen_*`/`hubsplit`): `closure.ts` (gate counter for the five closure rows), `appendix_a_drift.py`, `extscan.py`, `shelledges.py`, `crates.py` (the frozen crate table, rank check), `verify_static.ts` (manifests, DAG and ranks, members, owners, launch routes, emoji, path lengths, top block), `verify_hub.py` (mount equivalence against the original, regions, rustfmt parse), `oracle.py` (full-resolution differential, `--baseline snapshot|inverse`), `ts_compare.py`, `ts_vitest.py`, `tree_compare.ts`, `taxonomy-scope.ts` (taxonomy verifier over one folder, about 10 s).

* **Census**: `cd 🗑️generated && ONLY=semio-framework-os-renderer-wgpu-<suffix> bun census.ts`. Every skeleton is 1 file, at most 342 bytes, at most 6 lines. Only the five leaves `canvas2d-paint`, `hub-contracts`, `mailbox-core`, `shell-vocab`, `test-support` have a closure inside the gate today; every other crate sits behind the oversized frontier crates (`semio-framework`, `os-kernel`, `plugin`, `canvas`, …).
* **After the first crate leaves the monolith** the WP0 differential (`oracle.py`) turns `extern crate … as …;` into an empty module and can no longer answer; use the union oracle of the earlier carves (`scene-carve-tools/union.py`, `canvas-carve-tools/union.py`) per crate root, plus the definition scan.
* **Blocked, not fixed by phase 0** (recorded in the RESULT file): (1) the launcher was down (`Finite original maintenance authority required`) until the end of phase 0 and then worked: `check -p` of the five closure-admitted leaves (`mailbox-core`, `canvas2d-paint`, `test-support`, `shell-vocab`, `hub-contracts`) is green, every other skeleton is refused by the gate before the compiler (`Rust crate Source refused before compiler: semio-framework-os-kernel` / `semio-framework-os-infinite`, the oversized frontier); (2) the live test closure cannot be re-measured because a peer removed `🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs` (counter error `Missing Rust source input`); the 17:30 test rows stand; (3) the renderer root gained `use ui_gpu::{GpuContext};` from a peer during phase 0 (the sandbox treats it as an unresolved external crate); (4) `icon-host` cannot meet its file budget as the tree stands: see 6.2 `icon-host` and risk 7 in section 13.

### 10.5 Integration pass 1 (`RESULT-RENDERER-INTEGRATION-1.md`, 2026-10-16)

The closing pass of the non-Shell packages. It resolved every cross-package hand-off that does not touch a Shell view file and flipped every crate whose dependencies are real crates. State of the 88 crates afterwards (`ri1-tools/real.json` lists the real ones):

| state | crates |
|---|---|
| real (own their sources, mounted by `extern crate … as …;`, `use`/`pub use` of the crate, or a facade) | WP1: all 18 (`asset-authority` and `asset-decode` carved in this pass, `native_temporal` joined `native-platform`); WP2: all 14; WP3: `ui-engine-focus`, `ui-engine-dump`, `ui-engine` (facade `interpreter`), `host-plumbing` (flipped in this pass), `test-support`; WP4: `shell-vocab` |
| still skeleton, blocked on `shell-state` (the frame core holds `ShellState` by value: `AppInteractionState` lives in `frame-actions`) | `frame-actions`, `runtime-mailbox`, `app-runtime`, `frame-presentation`, `frame-transaction`, `frame-job`, `native-probe`, `host-winit`, `browser-worker` |
| still skeleton, Shell (WP4 staging, WP5 views) and the companion test crates | 23 Shell crates, 14 Shell test bins, `engine-attach-tests`, `scene-ink-tests`, `ui-engine-tests`, `runtime-mailbox-tests` |

Rules this pass added to the contract:

* **`contracts` owns the cell and vocabulary family exactly once**: the `AdmittedSurface*` map, the shell find items (`ShellFindItem`, `ShellFindItems`, `ShellFindItemBinding`, `try_push_find_item`; the binding thread-local exists only there), `ShellHubRemoteV1`, the retained UI engine cell with `component_scene_is_presented`, and the framework widget context. The Shell, the Interpreter and the Scenes facades re-export them with `pub use`; a lift file is mounted by its destination crate only (`dupmounts.py`: no file has two owners).
* **A facade over several crates names them with `use <crate>::<module>;`**, keeps its test mounts and `*_SOURCE` consts, and the children stay where they are (the crate root mounts them with `#[path]`). The module-tree helper now follows a facade, hub or root file into the crates it names (`inspectRustCrateReferences`; members of a crate never do), so the TypeScript source-text readers see the same files as before.
* **`testkit` carries `cfg(test)` across a crate boundary.** A carved crate that has a mock, per-thread or test-only variant behind `cfg(test)` declares `[features] testkit = [<its first-party dependencies>/testkit]` and writes the predicate `any(test, feature = "testkit")` (`not(any(test, feature = "testkit"))` for the production variant); the top crate lists the crate in its `[dev-dependencies]` with `features = ["testkit"]`. Applied to `contracts`, `ui-engine-focus`, `ui-engine-dump`, `ui-engine`; the other real crates still carry plain `cfg(test)` variants (clipboard mock, per-thread cells, …) that are off in the monolith test unit (risk 1 of the result file).
* **Tests that name the monolith stay in the monolith**: a test host file in the hub folder (`📥️mounts/<hub>/🧪️test-hosts/<slug>/🦀️.rs`) re-exports the module namespace the law was written against and mounts the law file; the crate no longer mounts it (`🔬️wgpu-ui-command-wiring`, `🔬️wgpu-introspection`).
* The frozen crate table (`🗑️generated/rwp0-crates.json`, Appendix F) now records the first-party edges the live manifests declare (12 crates changed, all downward; `ri1-tools/freeze.py`); `rwp0-tools/verify_static.ts` reports 0 failures.

### 10.6 Integration pass 2 (`RESULT-RENDERER-INTEGRATION-2.md`, 2026-10-16)

The Shell and the frame core became crates: the 15 tier-1 crates, `shell-state`, the 8 WP5 crates (flipped before the frame core, which calls their bundles) and the 9 frame-core crates now own their sources (the staging and WP3 files stay in place and are mounted by the crate roots); the `shell` parent and the WP3 hub only `pub use` them. Rules this pass added: (1) the direction of **items**, not only of method calls, decides the crate: 148 free items and types moved down to the lowest crate that can see all they name (`ri2-tools/moves*.json`); (2) format-argument identifiers count as references; (3) `testkit` is declared by 18 more crates (the top crate and the test bins enable it); (4) a test that shares helper modules with monolith-only fixtures (`collect_fixture_actions`, `camera_fixture_lease`, `engine_surface_law_guard`, `RENDERER_SOURCE`) stays mounted in the Shell tree; 23 of 55 moved into seven bins; (5) the union oracle never rewrites a file outside the sandbox. Open: the three standalone fixture files and a `renderer_source()` reader (the remaining 32 tests), a real compile (the launcher refuses for headroom), narrowing of the `pub` widening, residue wasm exports in the old chunk files.

## 11. Verification and gates

Per crate (WP-level, before its mount is removed from the old hub):
1. `cd <monolith-split>/🗑️generated && ONLY=<crate> bun census.ts` — lib and test rows must satisfy ≤ 3,500 / 28 / 230,000 (the table of section 6.1 predicts them; a miss of more than 8 percent means re-cut, not squeeze).
2. Full-resolution differential from the crate root (`RESULT-HARDEN-WGPU.md` method: `rustc +nightly-2026-07-20 --emit=metadata`, native lib, `--cfg test`, `wasm32-unknown-unknown` lib, wasm32 `--test`; baseline sandbox = the old tree) — error multisets must be identical. Seeded sensitivity check (remove one `pub`, one import) must be caught.
3. Definition scan for fields, methods and items that cross the new boundary (tools in `🗑️generated/harden-wgpu/`, `defscan.py`).
4. `rustfmt --edition 2021 --emit stdout` on every new file; path-literal resolver (all `#[path]`, `include*` resolve); TOML parse of the manifest; both JSON files parse.
5. Real compile is attempted at start and end of each package with `bun repo run @semio-tech/<project>-rs:check --configuration=prototype --parallel=1 -- --lib --offline`; today the ingress gate refuses the old crate and `semio-framework-3d`, so a green real compile is expected only for the leaf crates (`contracts`, `host-base`, `mailbox-core`, `canvas2d-paint`, `media-slots`, `native-platform`, `hub-contracts`). Report actual output.
6. Types of regressions to look for after a carve: duplicate singleton statics, a lost `#[cfg]` on an import, a `pub use` glob accidentally widening a name, `Deref`/trait-method scoping of types that moved (the oracle cannot see trait scoping), and the Shell methods whose callers did not move together.

Top-level (WP0, phase 2): export parity (47 names), wasm32 + native + wasip2 `cargo metadata`, all crate-boundary corpora, the TS suites that read these sources (list: `♻️frame-close`, `🔬️interactivity-mounted-frame-transaction`, `🛑️scene-pointer-cancellation`, `⏱️wgpu-worker-step-budget`, `🧾️frame-action-ledger`, `🖼️wgpu-raster-residency`, `📨️browser-frame-transport`, UI `♻️physical-job-close`, and the `🔬️rust-policy-source-evidence` registry guard; each must read through the module-tree helper).

## 12. Reproducing the analysis

Everything above was produced from five small programs (no repo files were written):

* **closure**: the repo counter, called as `readRustSourceClosureV1([PKG/📚️library/🦀️.rs], ROOT, cfg, read, {manifestDirectory: PKG, generated: () => undefined})` with `cfg = rustc --print cfg [--target T] + debug_assertions [+ test]` and a `read` that returns an empty buffer for non-`.rs` ENOENT (29 stubbed fixtures).
* **tree**: regex mount parser (`#[path = "…"]`, `mod x;`, `include!("…")` with the attached `#[cfg(…)]`), 285 nodes.
* **outline**: column-0 item scanner over comment/string-stripped text with brace matching; items = `fn struct enum union trait type static const mod impl macro_rules! thread_local!`; an impl is bound to its type; item text yields the name set.
* **graph**: item `defs` / `refs` (rules of section 3), edges between assigned crates, Tarjan SCC, linear-order feedback minimiser (greedy insertion), back-edge closure (down) and dependent closure (up).
* **shell**: field table → component; `self.f` / `self.m()` extraction over the 701 methods; homing algorithm of 7.4.

**[WP0 change]** The WP0 programs are in the ticket folder `🪓️monolith-split/rwp0-tools/` (`closure.ts`, `appendix_a_drift.py`, `extscan.py`, `shelledges.py`, `crates.py`, `gen_files.py`, `gen_registry.py`, `hubsplit.py`, `verify_static.ts`, `verify_hub.py`, `oracle.py`, `ts_compare.py`, `ts_vitest.py`, `tree_compare.ts`, `taxonomy-scope.ts`, `plan_appendix.py`, `plan_edit.py`).

The planner's programs fit in about 600 lines of Python and live in the planner's session scratchpad (ephemeral); recreate them from this description. The outline tool of Appendix E is the only piece executors need verbatim. Parameters worth re-running at WP0: the census (section 11.1), `ONLY=`-scoped, and the item-level back-edge listing after each package merges its moves.

## 13. Risks and open decisions

1. **Frame core inversion (I2)** is the one design decision with runtime consequences (a generic `RuntimeMailbox<R>` changes monomorphisation: expect larger code in `app-runtime`). Alternative if rejected: keep `AppRuntime`, `RuntimeMailbox(Inner)`, `RuntimeApply`, `NativeReferenceDecodeJob` and the completion types together with their impls in one `runtime-core` crate (about 3,100 lines) and layer `frame-actions` and the rest of `app-runtime` around it. Not chosen because it leaves a 3,100-line cycle crate, no seam, and no room for tests.
2. **P1/P2 make refresh and dispatch deferred** inside components. The tutorial, the settle pump and `drain_deferred_actions` already assume deferral (`owed_refresh_scope`, `deferred_actions`), but every one of the 47 call sites needs a read of what follows the call. WP4/WP5 should list them in their RESULT files.
3. **The Shell numbers are estimates** (method homing is name-based; free items are assigned by file). The spill valve of 7.4 keeps any overflow dependency-safe, but `shell-dock` (2,911 estimated lines) and `shell-actions` (2,229) are within 25 percent of the limit and `app-runtime` is at 142 KB of 230 KB before tests.
4. **Peer churn**: the working tree moved twice while this plan was produced. WP0 re-measures first; if a package's file set changed, the owner of the moved file transfers it by renaming its row in section 10, not by editing foreign files.
5. **88 new crates is a lot of registry** (88 × 6 launch routes × 2 JSON files, owner rows, locks). That cost is mechanical and entirely WP0; phase 0 paid it except the lock files.
6. **Decision requested** (assumed yes by phase 0: the skeleton companions depend on production crates through `pub` API only): whether the companion test crates may depend on production crates through `pub` API only (assumed yes; the alternative, `pub(crate)` + in-crate tests, is what blows `engine-attach`, `ui-engine`, `scene-ink`, `runtime-mailbox`, `app-runtime` past 3,500 lines).
7. **[WP0 change]** **`icon-host` and the embedded icon SVGs.** The tree embeds 250 raw icon SVGs plus the logo through a generated index (+251 files in every lib and test closure); with them `icon-host` is at 259 files against a 28 file budget and cannot pass the gate (6.2). Needs the runtime asset channel or a raw-icon asset; WP2 with the Canvas owner.
8. **[WP0 change]** **Hubs are `include!`d.** Every macro-expanded `#[macro_export]` macro stays reachable by `crate::` paths only if its module is declared outside an `include!`d file (found: `renderer_registration`, kept in the library root). Any new `#[macro_export]` macro in a hub-mounted module has the same defect; declare such modules in the library root or re-export the macro from a real crate.
9. **[WP0 change]** **The renderer root is still edited by peers** (`use ui_gpu::{GpuContext};` appeared while phase 0 ran). The WP0 differential supports `--baseline inverse` (the current root and hubs un-hubbed against the current tree) for exactly that reason; WP1 owns the root, so it reconciles such lines.
10. **[WP0 change]** **Cargo evidence is partial.** The launcher worked only at the end of phase 0: the five admitted leaf skeletons check green; the other 83 are refused by the gate through peer crates (`os-kernel`, `os-infinite`, …) and stay behind the static gate, the census and the differentials until those frontier crates are carved.

## Appendix A. Library files (138; `T/` = `E/🎯️targets/🧊️wgpu/`; a path without prefix is `E/🧱️elements/<element>/…` with its `🎯️targets/🧊️wgpu` segment omitted; `T/🧊️renderer/…` is the text's `R/`)

Active lines and bytes are the native-closure values (0 = not in the native cfg); the cfg gate is the chain of `#[cfg]` attributes of the mounts that reach the file (the `not(wasi)` of the included root is omitted); "tests lines" are the lines of test modules mounted under the file; "target crate(s)" lists the crates the file's items go to (percentages when a file is split).

| # | file (E/ = engine root, T/ = targets/wgpu) | active lines | bytes | cfg gate | tests lines | target crate(s) |
|---:|---|---:|---:|---|---:|---|
| 1 | `T/⌨️native-entrypoint` | 159 | 7247 | not(target_arch = "wasm32") | 0 | entrypoint |
| 2 | `T/⌨️native-entrypoint/📦️modules` | 78 | 3358 | not(target_arch = "wasm32") | 0 | program-bridge |
| 3 | `T/⏰️deadlines` | 119 | 5035 |  | 100 | host-plumbing |
| 4 | `T/⏱️frame-latency` | 360 | 13634 |  | 123 | host-base |
| 5 | `T/♿️native-accessibility` | 395 | 19215 | not(target_arch = "wasm32") | 209 | native-platform |
| 6 | `T/🌉️agent-bridge-door` | 329 | 16205 |  | 0 | agent-elements |
| 7 | `T/🌐️browser-worker` | 0 (wasm32: 935, 59962 B) | 0 | target_arch = "wasm32" | 0 | browser-worker 99% |
| 8 | `T/🌐️browser-worker/📤️output` | 57 | 5847 |  | 26 | host-plumbing |
| 9 | `T/🎟️native-caller` | 88 | 14473 | not(target_arch="wasm32") | 0 | host-plumbing |
| 10 | `T/🎠️runtime` | 181 | 11925 | not(target_arch = "wasm32") | 0 | kernel-protocol |
| 11 | `T/🎬️media-slots` | 142 | 10501 |  | 213 | media-slots |
| 12 | `T/🎬️media-slots/🎟️collect` | 107 | 29284 |  | 27 | media-slots |
| 13 | `T/🎬️media-slots/📤️slot` | 64 | 14430 |  | 19 | media-slots |
| 14 | `T/🎬️media-slots/📥️props` | 59 | 8460 |  | 22 | media-slots |
| 15 | `T/🎮️input-wire` | 260 | 11522 |  | 196 | host-plumbing |
| 16 | `T/🏠️os-host` | 951 | 45039 |  | 674 | host-winit 92%, scene-state 8% |
| 17 | `T/📇️catalog` | 26 | 2909 |  | 0 | host-base |
| 18 | `T/📇️directory-door` | 147 | 12946 |  | 199 | doors |
| 19 | `T/📐️surface-lane` | 479 | 19127 |  | 84 | host-plumbing |
| 20 | `T/📦️packages/🦀️rust/📚️library` | 20 (was 23) | 854 (was 990) |  | 166 | entrypoint |
| 21 | `T/📮️runtime-mailbox-core` | 214 | 9089 | target_os = "wasi" | 264 | mailbox-core |
| 22 | `T/📸️render-snapshot` | 160 | 14284 |  | 89 | host-plumbing |
| 23 | `T/🔌️socket-door` | 475 | 30535 |  | 218 | doors |
| 24 | `T/🔤️runtime-assets` | 88 (was 64) | 5219 (was 3647) |  | 40 | host-base |
| 25 | `T/🕰️native-temporal` | 169 | 12445 | not(target_arch = "wasm32") | 0 | native-platform |
| 26 | `T/🕰️native-temporal/🍎️macos` | 301 | 14267 | target_os = "macos" & not(target_arch = "wasm32") | 286 | native-platform |
| 27 | `T/🕰️native-temporal/🐧️unix` | 0 | 0 | all(unix, not(target_os = "macos")) & not(target_arch = "wasm32") | 0 | native-platform |
| 28 | `T/🕰️native-temporal/🪟️windows` | 0 | 0 | windows & not(target_arch = "wasm32") | 0 | native-platform |
| 29 | `T/🧊️renderer` | 665 (was 921; the mounts moved to the hubs, rows 140 to 144) | 38042 (was 49580) |  | 3602 | host-base |
| 30 | `T/🧊️renderer/🎨️presentation-raster` | 826 | 40232 |  | 97 | runtime-mailbox 99% |
| 31 | `T/🧊️renderer/📇️registry` | 10 | 234 | not(target_os = "wasi") | 0 | host-base |
| 32 | `T/🧊️renderer/📊️scale-bench` | 896 | 57098 | not(target_arch = "wasm32") | 0 | kernel-runtime |
| 33 | `T/🧊️renderer/📦️kernel-runtime` | 84 | 5233 | not(target_arch = "wasm32") | 0 | kernel-protocol 79%, kernel-runtime 21% |
| 34 | `T/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool` | 2188 | 136783 | not(target_arch = "wasm32") | 1403 | kernel-pool 87%, kernel-protocol 11% |
| 35 | `T/🧊️renderer/📦️kernel-runtime/📬️kernel-replay-client` | 1390 | 64800 | not(target_arch = "wasm32") | 124 | kernel-protocol 83%, kernel-pool 17% |
| 36 | `T/🧊️renderer/📦️kernel-runtime/🗃️kernel-retained-state` | 935 | 43159 | not(target_arch = "wasm32") | 0 | kernel-retained 94%, kernel-pool 6% |
| 37 | `T/🧊️renderer/📦️kernel-runtime/🧮️kernel-core` | 879 | 40114 | not(target_arch = "wasm32") | 0 | kernel-protocol |
| 38 | `T/🧊️renderer/📮️mailbox-impl` | 1535 (was 1533) | 89359 (was 88653) |  | 0 | runtime-mailbox |
| 39 | `T/🧊️renderer/🔁️asset-decode-jobs` | 966 | 47277 |  | 221 | asset-decode 59%, host-base 38% |
| 40 | `T/🧊️renderer/🔬️native-socket-probe` | 423 | 23778 |  | 152 | native-probe 99% |
| 41 | `T/🧊️renderer/🖥️frame-presentation` | 1398 | 80299 |  | 0 | frame-presentation 81%, app-runtime 19% |
| 42 | `T/🧊️renderer/🗂️asset-authority` | 757 | 33638 |  | 0 | asset-authority |
| 43 | `T/🧊️renderer/🛎️frame-actions` | 1061 | 51907 |  | 0 | frame-actions 65%, runtime-mailbox 35% |
| 44 | `T/🧊️renderer/🦴️glb-decode` | 2419 | 125719 |  | 0 | asset-decode |
| 45 | `T/🧊️renderer/🧠️app-runtime` | 1446 | 89296 (was 89096) |  | 0 | app-runtime 81%, runtime-mailbox 9%, frame-transaction 9% |
| 46 | `T/🧊️renderer/🧾️frame-build` | 1463 | 84433 (was 84103) |  | 0 | frame-transaction 55%, app-runtime 45% |
| 47 | `T/🧊️renderer/🪶️sqlite` | 58 | 5130 | not(target_arch = "wasm32") | 0 | native-probe |
| 48 | `T/🧬️element-contracts` | 135 | 7001 |  | 0 | contracts |
| 49 | `T/🧵️frame-job` | 903 | 65204 |  | 449 | frame-job 99% |
| 50 | `T/🪟️winit-app` | 1079 | 67857 |  | 441 | host-winit 84%, app-runtime 12%, host-plumbing 3% |
| 51 | `⚙️EngineCanvas/T` | 145 | 9520 |  | 3530 | engine-registry |
| 52 | `⚙️EngineCanvas/✍️text-editor` | 1375 | 73693 |  | 0 | engine-text 83%, engine-registry 17% |
| 53 | `⚙️EngineCanvas/🎓️tutorial-surface-geometry` | 176 | 8696 |  | 0 | engine-registry |
| 54 | `⚙️EngineCanvas/🎲️board2d` | 883 | 45776 |  | 391 | engine-board |
| 55 | `⚙️EngineCanvas/📦️prepared-engine-canvas` | 702 | 38396 |  | 0 | engine-registry |
| 56 | `⚙️EngineCanvas/🕸️node-graph` | 1326 | 69740 |  | 80 | engine-graph 98% |
| 57 | `⚙️EngineCanvas/🖌️paint2d` | 604 | 33231 |  | 0 | engine-board |
| 58 | `⚙️EngineCanvas/🗂️engine-surface-registry` | 1125 | 48317 |  | 0 | engine-registry 100% |
| 59 | `⚙️EngineCanvas/🗺️tiled-map` | 311 | 15032 |  | 0 | engine-board |
| 60 | `⚙️EngineCanvas/🧩️engine-surface-attach` | 1138 | 59357 |  | 43 | engine-attach 99% |
| 61 | `🌉️ProgramBridge/T` | 1309 | 119361 |  | 299 | program-bridge 98% |
| 62 | `🎞️Scenes/T` | 381 | 18568 |  | 0 | scene-state 95%, native-platform 5% |
| 63 | `🎞️Scenes/⌨️scene-input` | 1076 | 51825 |  | 0 | scene-render 91%, scene-state 9% |
| 64 | `🎞️Scenes/✍️text-editor` | 584 | 31631 |  | 450 | scene-text-vfs 96%, scene-state 4% |
| 65 | `🎞️Scenes/✒️ink-canvas-model-state` | 2054 | 92808 |  | 0 | scene-ink 99% |
| 66 | `🎞️Scenes/🎨️scene-render-entry` | 704 | 38936 (was 38787) |  | 228 | scene-render 91%, scene-state 9% |
| 67 | `🎞️Scenes/📰️event-feed-and-timeline` | 980 | 47024 |  | 862 | scene-feeds |
| 68 | `🎞️Scenes/📷️scene-camera-dispatch` | 329 | 20480 |  | 1121 | scene-state |
| 69 | `🎞️Scenes/🖋️ink-canvas-render` | 363 | 16763 |  | 769 | scene-ink 98% |
| 70 | `🎞️Scenes/🖌️canvas2d` | 1179 | 57007 |  | 656 | scene-canvas2d 88%, scene-state 12% |
| 71 | `🎞️Scenes/🖼️icon-render` (the planner's row said `icon-host`, a name slip) | 454 | 21079 (was 21071) |  | 240 | icon-host |
| 72 | `🎞️Scenes/🗃️scene-state` | 1126 | 46025 |  | 159 | scene-state 71%, contracts 29% |
| 73 | `🎞️Scenes/🧩️board2d-and-vfs` | 724 | 37374 |  | 269 | scene-text-vfs 99% |
| 74 | `🎞️Scenes/🧭️node-graph-and-tiled-map` | 464 | 22939 |  | 0 | scene-feeds 94%, scene-state 6% |
| 75 | `🎞️Scenes/🧮️table-and-paint2d` | 789 | 43613 |  | 525 | scene-tables 94%, scene-state 6% |
| 76 | `🎞️Scenes/🧱️block-list-and-diff` | 757 | 42532 |  | 541 | scene-tables |
| 77 | `🎞️Scenes/🪣️pending-raster` | 390 | 16001 |  | 0 | scene-state |
| 78 | `🏘️SpaceBrowser/T` | 539 | 26289 |  | 298 | hub-contracts |
| 79 | `🏛️ShellHost/🔀️surface-switch/📄️document` | 48 | 1933 |  | 73 | shell-* (App. B, C) |
| 80 | `🐚️Shell/T` | 961 | 63753 |  | 7056 | shell-* (App. B, C) |
| 81 | `🐚️Shell/⌨️keyboard-and-palette` | 2246 | 131728 (was 131583) |  | 4757 | shell-* (App. B, C) |
| 82 | `🐚️Shell/⏪️time-travel` | 1183 | 75276 |  | 2033 | shell-* (App. B, C) |
| 83 | `🐚️Shell/⏪️time-travel/🗣️labels` | 73 | 12731 |  | 0 | shell-* (App. B, C) |
| 84 | `🐚️Shell/⏪️time-travel/🚦️refusals` | 32 | 3841 |  | 0 | shell-* (App. B, C) |
| 85 | `🐚️Shell/⚓️dock-model-and-measures` | 1160 | 60471 |  | 215 | shell-* (App. B, C) |
| 86 | `🐚️Shell/⚙️settings-and-dock-persistence` | 2030 | 124053 (was 123883) |  | 716 | shell-* (App. B, C) |
| 87 | `🐚️Shell/🎓️tutorial-runtime` | 824 (was 823) | 45797 (was 45642) |  | 397 | shell-* (App. B, C) |
| 88 | `🐚️Shell/🎨️theme-file-import-and-accessibility` | 1938 | 119954 |  | 2925 | shell-* 96%, host-base 4% |
| 89 | `🐚️Shell/🎬️shell-actions` | 1901 | 124150 |  | 0 | shell-* (App. B, C) |
| 90 | `🐚️Shell/🏛️space-admin-and-settle` | 959 | 51544 |  | 0 | shell-* (App. B, C) |
| 91 | `🐚️Shell/💬️chrome-dialog-and-tooltips` | 1074 | 80431 |  | 0 | shell-* (App. B, C) |
| 92 | `🐚️Shell/📎️local-folders` | 439 | 34426 |  | 361 | shell-* (App. B, C) |
| 93 | `🐚️Shell/📤️icon-export` | 440 | 21396 |  | 147 | shell-* (App. B, C) |
| 94 | `🐚️Shell/📨️command-dispatch` | 2242 | 143210 |  | 0 | shell-* (App. B, C) |
| 95 | `🐚️Shell/🔑️storage-prefs-and-i18n` | 1680 | 104100 |  | 2933 | shell-* (App. B, C) |
| 96 | `🐚️Shell/🔖️identity-and-types` | 1652 | 86383 |  | 320 | shell-* 91%, contracts 9% |
| 97 | `🐚️Shell/🖱️pointer-input` | 2083 | 130853 (was 130723) |  | 0 | shell-* (App. B, C) |
| 98 | `🐚️Shell/🖼️chrome-frame-and-main-window` | 2052 | 119543 (was 119476) |  | 0 | shell-* (App. B, C) |
| 99 | `🐚️Shell/🗨️chrome-overlays-and-tour` | 1682 | 104975 |  | 0 | shell-* (App. B, C) |
| 100 | `🐚️Shell/🧩️session-and-topology` | 1764 | 109274 |  | 0 | shell-* (App. B, C) |
| 101 | `🐚️Shell/🧭️navbar-footer-and-overlays` | 1564 | 92638 |  | 0 | shell-* (App. B, C) |
| 102 | `🐚️Shell/🧰️window-actions-and-commands` | 2247 | 137344 (was 137329) |  | 0 | shell-* (App. B, C) |
| 103 | `🐚️Shell/🧱️chrome-paint-and-navbar` | 2278 | 138118 (was 137976) |  | 2247 | shell-* 99% |
| 104 | `🐚️Shell/🧵️shell-async-runners` | 826 | 40587 |  | 206 | shell-* (App. B, C) |
| 105 | `🐚️Shell/🧾️panel-records` | 1493 | 90683 |  | 357 | shell-* (App. B, C) |
| 106 | `🐚️Shell/🪟️tree-windows` | 201 | 11542 |  | 57 | shell-* (App. B, C) |
| 107 | `👕️canvas-presence/T` | 148 | 7494 |  | 138 | hub-workspace |
| 108 | `💬️AgentChatPanel/T` | 76 | 4525 |  | 0 | agent-elements |
| 109 | `📐️Canvas2dHost/T` | 612 | 27746 |  | 99 | canvas2d-host 63%, scene-state 37% |
| 110 | `📐️Canvas2dHost/🎨️paint` | 617 | 25587 |  | 180 | canvas2d-paint |
| 111 | `🔐️HubSignIn/T` | 759 | 37652 |  | 335 | hub-contracts |
| 112 | `🔗️AgentBridge/T` | 1214 | 55376 |  | 653 | agent-elements |
| 113 | `🔗️HubConnection/T` | 821 | 47936 |  | 796 | hub-workspace |
| 114 | `🖼️IconRenderHost/T` | 179 (was 185) | 7898 (was 8161) |  | 85 | icon-host |
| 115 | `🖼️IconRenderHost/📤️export` | 41 | 1459 |  | 0 | icon-host |
| 116 | `🖼️IconRenderHost/📤️export/🎬️scene` | 530 | 24917 (was 24534) |  | 0 | icon-host |
| 117 | `🖼️IconRenderHost/📤️export/📐️svg` | 739 | 30671 |  | 184 | icon-host |
| 118 | `🖼️IconRenderHost/📤️export/📥️asset` | 138 | 5004 |  | 54 | asset-authority |
| 119 | `🖼️IconRenderHost/📤️export/📷️png` | 80 | 3322 |  | 59 | icon-host |
| 120 | `🖼️IconRenderHost/📤️export/🧊️gpu` | 260 | 11467 |  | 107 | icon-host |
| 121 | `🗣️Interpreter/T` | 124 | 7511 |  | 68 | ui-engine |
| 122 | `🗣️Interpreter/✍️text-and-table-focus` | 586 | 28909 |  | 0 | ui-engine-focus |
| 123 | `🗣️Interpreter/🎯️chrome-ledger` | 732 | 41905 |  | 0 | ui-engine-dump 98% |
| 124 | `🗣️Interpreter/📄️retained-document-consumer` | 358 | 19598 |  | 0 | ui-engine |
| 125 | `🗣️Interpreter/🔬️introspection` | 1012 | 68453 |  | 444 | ui-engine-dump |
| 126 | `🗣️Interpreter/🖋️ink-and-vfs-focus` | 774 | 37349 |  | 0 | ui-engine-focus |
| 127 | `🗣️Interpreter/🖱️scene-interaction` | 496 | 28390 |  | 0 | ui-engine |
| 128 | `🗣️Interpreter/🖼️ui-image-loading` | 358 | 18058 |  | 0 | scene-canvas2d 94%, contracts 6% |
| 129 | `🗣️Interpreter/🧩️retained-ui-engine` | 1589 | 97854 |  | 2753 | ui-engine 72%, ui-engine-focus 20%, contracts 8% |
| 130 | `🗣️Interpreter/🧮️render-plan-validator` | 180 | 14102 |  | 263 | ui-engine |
| 131 | `🗣️Interpreter/🧱️mesh-and-board2d-stats` | 54 | 11994 |  | 0 | ui-engine-dump |
| 132 | `🚦️AgentPresence/T` | 104 | 5862 |  | 91 | agent-elements |
| 133 | `🛂️SpaceAdministration/T` | 216 | 11348 |  | 128 | hub-workspace |
| 134 | `🛰️Dock/T` | 2149 | 109350 |  | 1780 | dock |
| 135 | `🤖️AgentApprovals/T` | 363 | 18226 |  | 197 | agent-elements |
| 136 | `E/🪙️authority/🖼️frame` | 5 | 472 |  | 0 | frame-job |
| 137 | `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/📐️geometry` | 161 | 7581 |  | 0 | shell-* (App. B, C) |
| 138 | `🧰️framework/🔨️modules/🖼️assets/🥽️mesh` | 115 | 5726 |  | 57 | host-base |
| 139 | `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🧩️icon-embeds` **[WP0 change]** | 255 | 20119 |  | 0 | icon-host (+ 250 embedded SVGs, 164,569 B) |
| 140 | `T/🧊️renderer/📥️mounts/🌱️wp1-foundation` **[WP0 change]** | 150 | 8195 |  | 0 | hub of WP1 |
| 141 | `T/🧊️renderer/📥️mounts/🎞️wp2-scenes` **[WP0 change]** | 17 | 1058 |  | 0 | hub of WP2 |
| 142 | `T/🧊️renderer/📥️mounts/🧵️wp3-frame-core` **[WP0 change]** | 98 | 5304 |  | 0 | hub of WP3 |
| 143 | `T/🧊️renderer/📥️mounts/🐚️wp4-shell-state` **[WP0 change]** | 8 | 677 |  | 0 | hub of WP4 |
| 144 | `T/🧊️renderer/📥️mounts/🎛️wp5-shell-views` **[WP0 change]** | 9 | 747 |  | 0 | hub of WP5 |


## Appendix B. Method assignment for the 701 `ShellState` methods (algorithm 7.4, threshold 4 components, ports P1/P2 excluded, 7 callees moved into `shell-window-actions`)

Format: `name` or `name`(lines) for methods of 40 lines or more. This is the mechanical first cut of the field-touch rule: a name that looks out of place (for example `dispatch_gesture_action` in `shell-chrome-state`, 148 lines, because it only touches `chrome_build`) is an artefact of homing by fields; if its callees force it, lift it to the nearest use-case crate by the spill rule of 7.4. Same-named methods in different files (`directory_now_ms`, `pump_directory_events`, `plugin_install_precondition`, `acquire_plugin_program`, `drain_gesture_bound_work`) appear once per file.

**shell-chrome-state** (18 methods, 326 lines): `active_theme_id_for_frame`, `advance_chrome_preferences_persist_step`, `agent_presence_tone`, `apply_agent_bridge_frame`, `cancel_agent_tool_call`, `chrome_control_label`, `dispatch_gesture_action`(148), `mobile_panel_active`, `persist_ui_prefs_if_changed`, `request_chrome_preferences_load`, `request_chrome_preferences_persist`, `resolve_agent_approval_control`, `retained_text_field_has_focus`, `shortcut_table`, `show_transient_notice`, `take_agent_bridge_outbox`, `time_travel_focus_is_held`, `transient_notice`

**shell-surfaces** (29 methods, 582 lines): `advance_world3d_retirement_step`, `chrome_maintenance_pending`, `clear_world3d_catalogue_drop_previews`, `close_component_world_step`, `engine_host_id_for_window`, `handle_world3d_input`, `live_compute_surfaces`, `next_chrome_deadline`, `next_tutorial_deadline_ms`, `owe_refresh`, `owe_settle`, `scene_surface_contains`, `schedule_sync_path_pick`, `submit_shell_io`, `submit_shell_io_future`, `sync_world3d_catalogue_drop_preview`, `tutorial_bar_reserve`, `tutorial_note_real_dispatch`, `tutorial_seek`, `tutorial_stop`, `tutorial_tick`(73), `tutorial_toggle_play_pause`(48), `world3d_catalogue_drop_action`, `world3d_close_cancel_settled`, `world3d_host_id_for_window`, `world3d_retirement_fault`, `world3d_state_for_window`, `world3d_state_for_window_mut`, `world3d_status_rows`

**shell-plugins** (18 methods, 356 lines): `acquire_plugin_program`, `cancel_plugin_install`, `default_app_rows`(45), `default_host_panel_state`, `host_app`, `host_catalogue_tab_id`, `host_config`, `host_controller_id`, `install_plugin`(48), `installed_document_http_port`, `load_extension_record`, `marketplace_roster`, `plugin_install_precondition`, `project_extension_record`, `publish_app_catalogue`, `refresh_extension_store`, `refresh_ui`, `resolve_activation_owner_app`

**shell-history** (6 methods, 45 lines): `commit_local_folders_mutation`, `ensure_local_folder_bindings`, `forget_local_folder`, `history_time_travel`, `observe_history_reprojection`, `remember_local_folder`

**shell-prefs** (37 methods, 695 lines): `active_locale`, `active_terminology`, `apply_theme_editor_commit`(68), `band_tongue`, `build_command_panel_row`, `build_settings_conflicts_ui`(72), `build_settings_default_apps_ui`(42), `build_settings_driver_editor`(85), `build_space_workflows`, `clear_driver_draft`, `clear_theme_draft`, `driver_document`, `driver_rows`, `fault_status`, `folder_reconnect_band_plan_for`, `framework_settings_tabs`, `fullscreen_item_width`, `history_reprojection_band_plan_for`, `history_reprojection_status`, `load_ui_prefs_once`, `note_dispatch_fault`, `note_refused_open`, `open_boot_instance`, `plugin_fault_status`, `record_plugin_fault`, `record_surface_fault`, `refuse_replay`, `refused_guest_call`, `run_theme_import_request`, `session_action_invocation`, `set_driver_field`, `settle_surface_faults`, `surface_fault_status`, `theme_document`, `theme_draft_patch`, `theme_geometry_draft_patch`, `time_travel_band_plan_for`

**shell-directory** (42 methods, 797 lines): `acknowledge_space_administration_capability`, `begin_hub_artifact_creation`, `board_peer_overlays`, `board_presence_paint_items`, `board_presence_views`, `bootstrap_identity`, `build_agent_chat_ui`(65), `cancel_inference_port`, `clear_hub_session_owner`, `close_space_administration`, `current_shell_actor`, `directory_command_ctx`, `directory_ctx`, `dispatch_directory_command`, `flush_pending_directory_commands`, `hub_online_user_ids`, `hub_workspace_settled`, `intend_document_service`, `open_acknowledged_directory_stream`, `open_hub_artifact_creation`, `open_space_administration`, `peer_time_travel_presence`, `persist_hub_connection_book`, `poll_browser_identity`, `poll_directory_home_publication`, `poll_directory_page_result`, `project_verified_hub_authority`, `publish_peer_time_travel_notes`, `pump_inference_port`, `pump_space_administration`(47), `reload_hub_members`, `resolve_space_administration_control`, `run_hub_create_invite_turn`, `run_hub_redeem_turn`, `run_hub_sign_out_turn`, `send_agent_chat_draft`, `session_identity_view`, `set_agent_bridge_config`, `space_administration_plan`, `start_directory_event_page_fetch`, `start_hub_sign_in`, `start_hub_spaces_reload`

**shell-sync** (17 methods, 434 lines): `active_sync_owner`, `advance_sync_reseed`(52), `apply_hub_sign_in`, `attached_document_id`, `build_sync_attach_ui`(118), `hub_connection_state`, `hub_footer_label`, `hub_projection`, `install_document_seed`, `mint_sync_binding_generation`, `poll_hub_workspace_tasks`, `poll_identity_bootstrap`(42), `publish_hub_document_status`, `request_hub_check_in`, `request_presence_preview`, `retire_hub_document_status`, `sync_pill`

**shell-docs** (73 methods, 1570 lines): `action_session`, `active_plugin_manifest`, `active_terminologies`, `advance_chrome_preferences_load_step`(40), `announce_session_example`, `arm_control_command`, `arm_tool_for_panel_tab`, `auto_start_introduction`, `body_rect`, `build_command_panel_ui`, `build_display_windows_ui`(46), `build_os_commands`(60), `build_search_items`(65), `build_settings_general_ui`(98), `checkpoint_before_detach`, `chrome_dialog_request`, `chrome_tour_active_step`, `chrome_tour_advance_current_step`, `chrome_tour_complete_interaction`, `chrome_tour_note_action_performed`, `chrome_tour_note_utility_performed`, `close_document_one`(42), `command_category_tabs`, `command_search_items`(56), `command_session`, `default_dock`(50), `detach_host_route_folder`, `direct_reattach_candidate`, `dispatch_checkpoint`, `dock_store_app_id`, `document_controller_id`, `drain_retained_document_arenas`, `flush_host_route_folder`, `folder_reconnect_band_offer`, `folder_reconnect_offer`, `handle_checkin_action`, `has_display_tabs`, `host_effect_earned_scope`, `introduction_seen_key`, `match_app_keybinding`, `media_owner_for_session`, `mobile_panel_tabs`, `note_control_command`, `note_shell_setting_command`, `poll_auto_checkin`, `prepare_hot_reload`, `reconcile_host_introduction_policy`, `refresh_app_catalogue`, `refresh_app_document_identity`, `request_introduction_read`, `resolve_open_conflict`, `resolved_commands`, `restore_window_document`, `retain_document_for_close`, `retire_documents_outside`, `retire_one_surface_document`, `seed_open_conflicts`, `session_example_rows`, `shell_command_controller_id`, `shell_command_label_for_setting`, `shell_owned_panel_leaves`, `spawn_plugin`, `start_directory_home_publication`(51), `subfooter_height`, `sync_program_identity`, `take_window_document`, `teardown_spawned_window`, `time_travel_shortcut_action`, `tool_panel_tabs`, `trace_resolved_locale`, `uninstall_plugin`, `window_document_session`, `write_host_route_archive`

**shell-doc-open** (23 methods, 505 lines): `advance_icon_export`, `begin_document_resolution`(60), `begin_import_transfer`, `begin_media_frames_transfer`, `bind_document_execution_target`(46), `build_task_manager_ui`, `cancel_document_opening`, `cancel_icon_export`, `cancel_import_transfer`, `close_icon_export_step`, `default_bindings_for_current_session`, `enqueue_icon_export`, `fail_document_opening`, `import_task_nodes`, `install_document_services`, `poll_shell_io`(56), `pump_agent_bridge_offer_scope`, `pump_hub_check_in`(58), `push_uri`, `queue_transfer`, `run_file_open_request`, `shell_uri`, `step_import_transfers`

**shell-windows** (35 methods, 549 lines): `actions_enabled_for_window`, `active_utility_for_window`, `apply_initial_world_projection_template`, `arm_window_blur_host_event`, `build_tool_panel_ui`, `build_window_projection_ui`, `commit_window_topology_publication`, `discard_window_topology_journal`, `install_app_session`, `install_window_engagements_section`, `measures_rail_folded`, `owe_window_topology_refresh`, `projection_pane_folded`, `publish_surface_records`, `publish_window_measures`, `refresh_tool_measures`, `refresh_window_engagements`, `refresh_window_measures`(50), `refresh_window_projection_document`, `reserve_window_topology_action`, `set_active_utility`, `switch_to_app`, `take_window_topology_action`, `toggle_active_utility`, `utility_bar_folded`, `window_actions_folded`, `window_has_engagement_spec`, `window_has_search_spec`, `window_icon_override`, `window_measures_rect`, `window_pane_chip_icon_id`, `window_search_folded`, `window_title_override`, `window_topology_publication_has_credit`, `world_projection_template_id`

**shell-dock** (103 methods, 1791 lines): `active_utility_bar_window_kind`, `advance_panel_layout_persist_step`, `allocate_window_instance_id`, `anchor_at_point`, `anchor_content_height`, `anchor_content_rect`, `anchor_open`, `anchor_panel_tab_rows`, `anchor_rect`, `anchor_state`, `anchor_state_mut`, `anchor_tab_bar_height`, `anchor_tab_row_item`, `anchor_tab_rows`, `apply_dock_ui`, `apply_host_panel_selection`(50), `apply_inbound_agent_shell_commands`, `apply_window_icon_host_command`, `apply_window_title_host_command`, `attach_host_route_backbone`, `build_display_layout_ui`(88), `chrome_tab_band_width`, `chrome_tour_frame_begin`(43), `chrome_tour_reveal_panel_tab`, `close_active_window`, `close_dock_window`, `close_dock_window_and_arm`, `current_named_layout`, `deactivate_active_window`, `derive_utility_nodes`, `derive_window_utility_nodes`, `dock_branch_collapsed`, `dock_chrome_maps`(46), `dock_stack_path_for`, `dock_tab_drop_index`, `dock_tab_insert_preview`, `dock_ui_snapshot`, `document_body_key`, `drain_gesture_bound_work`, `finish_dock_tab_drag`, `focus_active_window`, `journal_panel_selection`, `keyboard_ring`, `left_panel_open`, `live_window_ids`, `live_window_kind_id`, `load_persisted_dock`, `load_persisted_named_layouts`, `move_dock_tab`, `named_layout_by_id`, `note_panel_toggle_command`, `note_retained_body_painted`, `observe_history_time_travel`, `observe_tree_windows`, `open_active_window_in_new_window`, `open_anchors`, `open_display_window`, `open_floating_panel_rects`, `panel_anchor_snapshot`, `panel_tab_anchor`, `persist_dock_layout`, `persist_dock_ui`, `persist_dock_ui_if_changed`, `persist_named_layouts`, `poll_time_travel_progress`, `pump_agent_bridge`, `record_document_paint_fault`, `refresh_history_snapshot`, `relabel_sync_tab`, `request_panel_layout_persist`, `reread_history_projection`, `restore_document_archive_bytes`, `restore_folder_archive`, `retained_keyboard_focus`, `retained_select_owns_keyboard`, `retained_surface_is_panel`, `reveal_dock_tab`, `reveal_mobile_panel_tab`, `right_panel_open`, `route_retained_pointer_press`(44), `seed_history_snapshot`, `seed_new_window_projection`, `select_panel_tab`, `select_panel_tab_step`, `select_world_projection_template`, `settle_pump_cross`(56), `surface_overlay_controls`, `surface_status_pills`, `sync_dock_tabs`, `sync_session_chrome`, `sync_world3d_declared_actions`, `toggle_anchor`, `toggle_anchor_tab`, `toggle_column`, `toggle_dock_branch`, `toggle_panel_anchor`, `toggle_window_pane_chip`, `tree_window_body_key`, `window_body_key`, `window_has_action_pane`, `window_kind_of`, `window_pane_chip_target`, `window_pane_chips`

**shell-input** (70 methods, 1463 lines): `activate_window_under_pointer`, `begin_pending_dock_drag`, `begin_window_template_drag_from_hit`, `build_settings_keybindings_ui`(63), `build_settings_theme_editor_tree`(88), `build_settings_theme_ui`, `candidate_pointer_input_is_modal`, `canonical_tree_open`, `capture_keybinding_chord`, `chrome_surface_census`, `claim_chrome_pointer`, `claim_text_editor_alt_completions`, `claim_text_editor_menu_action`, `close_open_selects`, `commit_focused_input`, `context_menu_handle_key`(97), `context_window_instance_id`, `discard_presented_input_candidate`, `dismiss_dismissable_layers`(50), `dispatch_tree_selection`, `dispatch_widget_drag_values`, `drive_presented_media_slots`, `finish_canvas2d_catalogue_drop`, `finish_tree_drag`, `focus_retained_accessibility_target`, `handle_chrome_dialog_accessibility_event`(55), `handle_chrome_dialog_key`(93), `handle_pointer_wheel`, `keyboard_focus_ring_rect`, `keyboard_focused_chrome_target`, `observe_interaction_output`, `pointer_input_is_modal`, `pointer_is_over_open_panel`, `pointer_owner_at`, `presented_input_candidate_matches`, `presented_retained_clock_deadline`, `progress_presented_input_candidate`, `queue_tree_selection`, `register_retained_body_hits`, `register_retained_panel_scroll_region`, `resolve_chrome_dialog_click`(48), `resolve_context_menu_surface`, `resolve_time_travel_focus`, `retained_canvas2d_hit_window`, `retained_clock_surface`, `retained_context_menu_window`, `retained_hit_window`, `retained_scene_hit_is_live`, `retained_scene_point`, `retained_window_body_rect`, `reveal_panel_tab_row`, `route_retained_pointer_move`(62), `scene_pointer_target_at`, `scene_pointer_target_is_published`, `set_canonical_tree_open`, `set_interaction_selection_projection`, `sync_canvas2d_catalogue_drag`, `sync_context_menu_hover`, `sync_world3d_catalogue_drop_preview_from_retained_ui`, `theme_appearances_section`(47), `theme_item_open`, `theme_leaf_section`, `theme_metrics_section`, `theme_section_open`, `theme_visible_rows`, `theme_window_rows`, `toggle_immediate_tree_item`, `update_tree_hover`, `utility_cursor_override`, `wheel_reaches_scene_surface`(49)

**shell-commands** (19 methods, 577 lines): `activate_find_item`, `apply_sync_shell_shortcut`(75), `build_command_category_ui`(84), `build_window_search_ui`(71), `close_command_palette`, `command_palette_plan`(96), `dismiss_overlays`, `dispatch_app_keybinding`(45), `effective_arg_value`, `execute_staged_action`, `filtered_find_items`, `filtered_search_items`, `palette_query`, `publish_command_panel_document`, `reset_staged_args`, `set_palette_query`, `set_search_open`, `stage_arg`, `staged_map_for`

**shell-session** (35 methods, 1215 lines): `app_owns_command`, `apply_boot_mode`, `apply_mode_step`, `apply_named_layout`, `apply_navbar_mode`, `boot_app_role`, `build_marketplace_ui`(135), `column_anchors`, `complete_window_topology_refresh`(54), `conflict_rows`, `debug_log`, `declare_boot_subphase`, `dispatch_fold_directory_events`, `flatten_panel_tab_leaves`, `framework_display_tabs`, `instant_now_ms`, `live_view_state`, `mirror_engine_surface_states`(83), `new`(301), `panel_identifier`, `panel_json`, `panel_state_from_view`, `publish_shell_panel_document`, `pump_tree_windows`, `republish_shell_panel_document`, `resident_census`, `retire_closed_world3d_windows`(50), `same_session`, `session_window_instances`, `sync_dock`(77), `sync_engine_surface_states`, `trace_engine_surface_sync`, `tutorial_start`, `tutorial_start_recording`(41), `validate_panel_state`

**shell-window-actions** (33 methods, 1353 lines): `activate_search_item`(52), `apply_async_shell_shortcut`, `apply_mutations`, `apply_os_command`(61), `apply_shell_uri`(69), `arm_host_window_blur`, `arm_window_activation_note`, `attach_sync_backbone`(53), `boot`(95), `build_window_actions_ui`(179), `dispatch_command`, `drain_operation_publications`, `hot_reload_plugins`, `note_shell_command_action`, `observe_invocation_history`(41), `open_service_port`, `parse_dock_tab_action_id`, `push_contributions`(56), `queue_host_effects`(94), `reattach_local_folder`, `reattach_remembered_local_folder`, `refresh_ui_rendered`(194), `refresh_window_action_panes`(43), `republish_window_action_panes`, `resolved_execute_args`, `run_extension_invocation`, `settle_boot`, `shell_command_for_control`, `staged_key`, `switch_to_session_role`, `tutorial_flush_pending_document_ops`, `window_actions_rect`, `window_search_rect`

**shell-actions** (30 methods, 2229 lines): `advance_document_opening`(91), `apply_ops_inner`(98), `apply_pending_shell_uri`, `begin_document_seed`, `continue_resolved_open`, `detach_sync_backbone_internal`(78), `directory_now_ms`, `dispatch_action`(622), `dispatch_session_command`(41), `extension_store_answer`, `finish_document_open`(54), `handle_hub_workspace_action`(110), `handle_open_artifact_relay`(58), `handle_replay_shell_command`, `handle_sync_action`(115), `install_extension_from_store`, `open_created_hub_artifact`, `open_document`, `parse_persistence_binding`, `prepare_document_open`, `pump_directory_events`(159), `pump_hub_artifact_creation`(76), `pump_sync_events`(181), `replay_create_space_artifact`, `run_session_app_switch`(50), `set_extension_enabled`, `switch_to_managed_app`(70), `touch_space_index_artifact`(97), `uninstall_extension`(43), `validate_extension_record`

**shell-pointer** (30 methods, 2229 lines): `advance_keyboard_ring`, `advance_owed_refresh`(52), `candidate_input_geometry`, `content_is_editing`, `dock_window_order`, `drag_captures_pointer`, `drain_deferred_actions`(58), `finish_dock_drag`(56), `flush_deferred_actions`, `handle_accessibility_event`(253), `handle_keyboard`(261), `handle_keyboard_async`(153), `handle_pointer_button`, `handle_pointer_button_for`(318), `handle_pointer_cancel`, `handle_pointer_cancel_for`(71), `handle_pointer_move`, `handle_pointer_move_for`(135), `handle_shell_hit`(366), `open_context_menu`(104), `pointer_hit_owner`, `pointer_press_belongs_to_shell_chrome`, `retained_event_modifiers`, `retained_pointer_button`, `seal_presented_input_candidate`, `settle_pump_pending`, `settle_pump_step`, `settle_pump_step_inner`(40), `settle_ui_chain`, `take_spawned_entry`

**shell-view-chrome** (34 methods, 1758 lines): `advance_chrome_maintenance_step`, `advance_presence_preview_step`, `chrome_hosted_panel_overhang`, `footer_band_width`, `footer_tab_row_item`, `navbar_center_tab_row_item`, `navbar_center_width`, `navbar_chrome_layout`, `navbar_control_band_width`, `navbar_control_width`, `navbar_leading_tab_row_item`, `navbar_leading_tab_row_rect`, `navbar_trailing_tab_row_item`, `paint_anchor_tab_bar`(70), `paint_mobile_tab_bar`, `paint_window_measures_step`(61), `paint_window_projection_step`(75), `paint_window_utilities_step`(75), `panel_band`, `panel_tab_scroll_id`, `plan_dock_windows`(60), `render_agent_chat_header_step`(52), `render_chrome_tooltip_step`(72), `render_context_menu_step`(83), `render_document_opening_step`(63), `render_mobile_panel_step`(83), `render_navbar_cluster_step`(43), `render_navbar_step`(333), `render_panel_document_opportunity`(47), `render_panel_step`(148), `render_plugin_install_step`(49), `render_transient_notice_step`(53), `render_tutorial_bar_step`(153), `window_projection_rect`

**shell-view-overlays** (24 methods, 1657 lines): `agent_approvals_paint_ops`(67), `band_accessibility_nodes`, `chrome_dialog_paint_ops`(198), `context_menu_level_width`, `folder_reconnect_accessibility_node`, `history_reprojection_accessibility_node`, `history_reprojection_progress_accessibility_node`, `intersect_content_rect`, `paint_window_pane_chips_step`(55), `paint_window_time_travel_indicator_step`, `render_agent_approvals_step`(47), `render_chrome_dialog`(62), `render_chrome_dialog_step`(48), `render_chrome_tooltip`(49), `render_chrome_tour_step`(171), `render_context_menu_level`(89), `render_folder_reconnect_band_step`(58), `render_history_reprojection_band_step`(50), `render_hub_workspace_step`(86), `render_overlay_step`(429), `render_space_administration_step`(48), `render_time_travel_band_step`(57), `time_travel_progress_accessibility_node`, `time_travel_status_accessibility_node`

**shell-view-panels** (20 methods, 1387 lines): `acknowledge_presented_input`(47), `chrome_accessibility_checked`, `chrome_accessibility_nodes`(163), `chrome_accessibility_pressed`, `chrome_accessibility_selected`, `dialog_accessibility_nodes`(55), `footer_chrome_layout`, `footer_presence_rows`, `footer_status_chips`, `footer_tab_row_rect`, `paint_window_pane_body_step`(58), `palette_accessibility_nodes`(112), `publish_retained_hit_registry`, `publish_retained_input_for_test`, `render_chrome_step`(343), `render_footer_step`(128), `render_icon_export_step`(46), `render_main_window_step`(255), `render_tree_drag_overlay_node`, `transient_notice_accessibility_node`


## Appendix C. Shell free items (types, constants, helpers outside `impl ShellState`) by source file

Counts are the outline's item lines (docs and attributes included). Methods are in Appendix B.

| source file (`E/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/…`) | free lines | goes to |
|---|---:|---|
| `🦀️.rs` (parent) | 714 | `ShellState` struct (595) dissolves into the 15 component structs (their field lists are in 7.3) and the composer (`shell-state`, ≈120 lines of fields + 21-line `Drop`); `ShellExtension{StoreRecord,LoadStatus,Projection}` + ledger capacity (26) → `shell-plugins`; `WindowIconOverride`, `WindowTitleOverride` (14) → `shell-windows`; `DockInputIdentity`, `dock_mode_layout_identity` (32) → `shell-dock`; `SHELL_SOURCE` (25) deleted |
| `🔖️identity-and-types` | 1,513 | find-item family (135) → `contracts`; `FRAMEWORK_*` ids, `WindowTemplateDragPayload`, `SearchPaletteItem`, `ReplayRefusal*` vocabulary (≈80), `COMPOSE_WINDOW_TEMPLATE_MIME` (≈250) → `shell-vocab`; `ContextMenu*` types and fns, `OverlayState`, `RightClickState`, `SHELL_OVERLAY_INPUT_IDS` (≈220) → `shell-input`; `ShellPaletteKind` (+68-line impl), `ShellPalettePlan*` (≈110) → `shell-commands`; `ShellPluginInstall*`, `ShellDocumentOpen*`, `ShellPreparedDocumentOpen`, `RemoteBackboneUri`, `document_execution_target_admitted`, `bind_wgpu_document_socket_surface`, `SpawnedAppEntry`, `SpaceProgramEntry`, `SpacePanelState`, `ActiveSession` (≈300) → `shell-doc-open` / `shell-docs`; `ShellSync{Channel,Owner,Reseed}`, `route_document_backbone_effects`, `SYNC_RESEED_RETRY_MS` (≈60) → `shell-sync`; `mint_shell_session_id`, `resolve_identity_env`, `shell_directory_*`, `shell_hub_*`, `shell_actor`, `directory_command_from_action`, `OpenArtifactRelayTarget`, `presence_peer_rows_for_surface`, `shell_presence_label` (≈160) → `shell-directory`; `AUTO_CHECKIN_*`, `fold_history_patch`, `history_rows_oldest_first`, `uncommitted_edit_count`, `can_check_in`, `auto_checkin_should_fire`, `should_checkpoint_before_detach`, `checkpoint_landed` (≈100) → `shell-history`; `SHELL_DRIVER_AXES`, `builtin_ui_driver_document`, `resolve_ui_driver_*`, `custom_ui_driver_id`, `ShellConflictRow` (≈80) → `shell-prefs`; `RetainedContentFocus`, `ShellChromeBuildState` (46), `ShellChromePresentState`, `ShellChromeMaintenance` (≈80) → `shell-chrome-state` |
| `🔑️storage-prefs-and-i18n` | 1,689 | `shell_chrome_string` (629), `scene_chrome_labels`, `shell_language_axes`, `resolve_language_axes`, `resolve_locale_id` (≈700) → `shell-vocab`; `PrefsStore`, `FilePrefsStore`, native pref files, storage door (`HostStorageKeyKind`, `StorageDoor*`, `encode/decode_*`), `ChromePrefsState`, theme registry fns, `UiPrefsSnapshot`, ui-preferences event log and `persist_ui_preferences` (≈900) → `shell-prefs`; `AgentBridgeOfferScopeV1/OwnerV1` (≈90) → `shell-directory` |
| `🎨️theme-file-import-and-accessibility` | 1,164 | download/pick/open/import transfers, host-io trace/call (call itself → `host-base`), ffmpeg frame extraction, `request_media_frames` (≈520) → `shell-async`; `ThemeNumber`, `ThemePaintRef`, `ThemeIcons`, `ThemeDocument`, theme resolve/metrics fns, theme editor constants (≈330) + `🖱️ui/…/🌓️theme/📐️geometry` (161) → `shell-prefs`; `ChromeControlPresentation`, `note_chrome_control_*`, `chrome_accessibility_role`, `chrome_status_accessibility_*` (≈180) → `shell-view-paint` |
| `🧰️window-actions-and-commands` | 935 | window scope V1 types and `mode_layout_stacks_v1`, `dock_seed_active_window_id_v1`, `engagement_control_rows` (150), `window_*_surface_id` (≈220) → `shell-windows`; shortcuts, keybinding rows, edit verbs, key matching, command registry, categories, fuzzy search (`rank_fuzzy_items`, `fuzzy_token_score`, `bounded_damerau_levenshtein`) (≈715) → `shell-commands` |
| `🏛️space-admin-and-settle` | 909 | `ShellSpaceAdministration*` (≈300), `ShellDirectoryCommandQueueV1`, `ShellHubTask` (≈170) → `shell-directory`; `ShellPluginFault`, `ShellSurfaceFault`, `select_boot_program`, `BootSelectionRequest`, `project_boot_app_role` (≈72) → `shell-plugins`; `WindowTopologyPublication` (9) → `shell-windows`; settle constants, `ShellSettle{Step,Watch,Verdict,Pump}`, `settle_*` (≈190) → `shell-surfaces`; `PendingExtensionInvocation`, `PendingFileOpen` (23) → `shell-doc-open`; `PointerHitOwner`, `PointerCapture`, `PresentedInputGeometry` (≈120) → `shell-input` |
| `🧵️shell-async-runners` | 787 | `ShellDetached`, `ShellPoolFuture`, `ShellDirectoryRunner`, `ShellInferenceRunner`, `DirectoryHomeProjection`, `ShellIoCompletion` → `shell-async`; `ShellDocumentRetirementRegistry` and slots (≈130) → `shell-docs` |
| `🧾️panel-records` | 1,429 | all (`PanelRecord`, `PanelProjection` 509-line impl, tool-run panel, app-catalogue attempt) → `shell-plugins` |
| `⏪️time-travel` (+ `🗣️labels` 70, `🚦️refusals` 29) | 650 | → `shell-history`; labels/refusals tables → `shell-vocab` |
| `📎️local-folders` | 250 | → `shell-history` |
| `🎓️tutorial-runtime` | 399 | → `shell-surfaces` |
| `⚓️dock-model-and-measures` | 865 | anchors, dock tabs, skeleton, tab moves → `shell-dock`; `WindowMeasure*` (≈100) → `shell-windows` |
| `🪟️tree-windows` | 137 | → `shell-dock` |
| `📤️icon-export` | 341 | `IconExportItems`, `Phase`, `IconExportBatch` → `shell-doc-open` (impl takes `&mut OpeningState` + params, 7.6) |
| `🗨️chrome-overlays-and-tour` | 1,462 | notice/band/dialog-request/tour/tooltip/agent-approval types and their impls (`ChromeDialogRequest` 430-line impl) → `shell-chrome-state`; `chrome_tour_layout`, `chrome_tour_anchor_rect` (142) → `shell-view-overlays` |
| `🧱️chrome-paint-and-navbar` | 1,739 | chrome group items, retained steps, navbar/footer layouts, `World3dComputeStatus`, `ShellHubRemoteV1` (→ `contracts`) → `shell-view-paint` |
| `🖼️chrome-frame-and-main-window` | 190 | frame cursors, `ShellUtilityPath`, phases → `shell-view-chrome` |
| `🖱️pointer-input` | 123 | `pointer_hit_owner` & friends → `shell-input` |

## Appendix D. Shell test bins (computed packing, ≤ 3,300 lines / 215 KB / 26 files)

| bin | tests | lines | KB | files | WP |
|---|---|---:|---:|---:|---:|
| `shell-tests-windows-1` | 🔬️wgpu-panel-anchor-model, 🪟️wgpu-window-pane-chrome | 2091 | 176 | 11 | 4 |
| `shell-tests-windows-2` | 🎬️wgpu-window-actions-search-panes, 🔬️wgpu-tutorial, 🛰️wgpu-dock-close-reopen-journals, 📏️wgpu-window-measures, 🧲️engine-surface-retention, 🛟️panel-window-reservation, 🪟️action-window-scope, 🪟️tree-windows | 2185 | 178 | 16 | 4 |
| `shell-tests-chrome-1` | 🔬️wgpu-ui-command-wiring | 2594 | 215 | 19 | 5 |
| `shell-tests-chrome-2` | 🔬️wgpu-unit | 1378 | 82 | 2 | 5 |
| `shell-tests-chrome-3` | 🔬️wgpu-shell-chrome-parity | 1271 | 176 | 11 | 5 |
| `shell-tests-chrome-4` | 🎨️wgpu-theme-editor-and-accessibility, 🔬️wgpu-chrome-overlays-tour | 1960 | 194 | 13 | 5 |
| `shell-tests-chrome-5` | 🧭️wgpu-navbar-footer-parity, 🖥️wgpu-display-conflicts-marketplace, ⚙️settings-general-layout, 🗄️browser-prefs-persistence | 2469 | 202 | 15 | 5 |
| `shell-tests-chrome-6` | 🔬️wgpu-agent-overlays, 🌓️appearance-tour-and-footer-pills, 🚪️wgpu-host-door-remainder, 🔬️wgpu-ui-prefs-themes-i18n, 🔬️wgpu-retained-chrome-text-laws, 🫀️settle-pump, 📐️wgpu-dpi-logical-units, 🚗️driver-editor, 🧪️wgpu-fault-notices, 🔬️unit, 💓️chrome-maintenance-pressure | 2416 | 196 | 20 | 5 |
| `shell-tests-input-1` | 🔬️wgpu-shell-input | 2727 | 242 | 26 | 5 |
| `shell-tests-input-2` | 🔬️wgpu-command-registry, ⌨️wgpu-shell-shortcuts-palette, 🎯️presented-input-authority | 2160 | 180 | 12 | 5 |
| `shell-tests-input-3` | 🎓️tour-overlay-and-chord-glyphs, ⌨️wgpu-chord-example-role-parity, 🎡️wgpu-wheel-and-escape-routing, 🪟️window-lifecycle-template-drag, 🎯️wgpu-pointer-hit-ownership, ⌨️native-keyboard-ring, 🔬️wgpu-context-menu-keyboard | 1786 | 145 | 14 | 5 |
| `shell-tests-docs-1` | 🧪️wgpu-time-travel | 2033 | 272 | 11 | 4 |
| `shell-tests-docs-2` | 🔗️hub-projection-workspace, 🧪️wgpu-local-folders, 🔬️wgpu-identity-directory-presence, 🔀️wgpu-document-relay, 🔬️wgpu-shell-boot-isolation, 📇️wgpu-directory-door | 2743 | 214 | 15 | 4 |
| `shell-tests-docs-3` | 📤️wgpu-file-open-import, ⏯️wgpu-tool-run-panel, 🔬️wgpu-shell-document-retirement, 🎬️wgpu-plugin-install, 📤️export-batch, 👕️board-presence, 🔬️wgpu-media-frames, 🔄️detached-sync-task-manager, 🛍️app-catalogue-attempt, ⬇️wgpu-media-export-encoding, 🔄️sync-owner-lifetime, 🔬️wgpu-shell-pool-future | 1393 | 160 | 23 | 4 |

`chrome-1` is the Interpreter's `ui-command-wiring` and `chrome-2` the Dock's `wgpu-unit` (both name `ShellState`); `docs-1` is `time-travel`, `input-1` is `shell-input`. Each bin is a companion crate `shell-tests-<bin>` depending on the tier crates it exercises. WP4 owns the `docs-*` and `windows-*` bins, WP5 the `input-*` and `chrome-*` bins. The bin names are theme labels, not ranks. Each test is rewritten against the new component / bundle API (`state.dock.dock_tabs`, `PointerCtx::of(&mut state)`), which is mechanical but not small: about 29,000 lines of test text change mostly in their receivers.

## Appendix E. Item outline tool (self-contained, 52 lines)

Run `python3 outline.py <file.rs>` to list every column-0 item with its line span; use it to find the items named in 5.2 on today's text.

```python
#!/usr/bin/env python3
# usage: python3 outline.py <file.rs>   -> "start-end lines kind name" for every column-0 item
import re, sys
def strip(t):
    o=[];i=0;n=len(t)
    while i<n:
        c=t[i]
        if t.startswith('//',i):
            j=t.find('\n',i);i=n if j<0 else j;continue
        if t.startswith('/*',i):
            d=1;i+=2
            while i<n and d:
                if t.startswith('/*',i):d+=1;i+=2
                elif t.startswith('*/',i):d-=1;i+=2
                else:
                    if t[i]=='\n':o.append('\n')
                    i+=1
            continue
        m=re.match(r'(?:br|rb|r|cr)(#*)"',t[i:i+12]) if c in 'rbc' and (i==0 or not(t[i-1].isalnum() or t[i-1]=='_')) else None
        if m:
            e='"'+m.group(1);j=t.find(e,i+m.end());j=n if j<0 else j+len(e)
            o.append('""'+'\n'*t[i:j].count('\n'));i=j;continue
        if c=='"':
            j=i+1
            while j<n and t[j]!='"':j+=2 if t[j]=='\\' else 1
            o.append('""'+'\n'*t[i:j].count('\n'));i=j+1;continue
        if c=="'":
            m=re.match(r"'(?:\\.[^']*|[^\\'])'",t[i:i+12])
            if m:o.append("''");i+=m.end();continue
            m=re.match(r"'[A-Za-z_]\w*",t[i:])
            if m:o.append(' ');i+=m.end();continue
        o.append(c);i+=1
    return ''.join(o)
HEAD=re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:(?:async|const|unsafe|extern\s+"[^"]*")\s+)*(fn|struct|enum|union|trait|type|static|const|mod|impl|macro_rules!|thread_local!|use)(?:\b|(?<=!))\s*([A-Za-z_<][\w:<>, ]*)?')
raw=open(sys.argv[1],encoding='utf-8').read();L=strip(raw).split('\n');R=raw.split('\n');n=len(L)
depth=0;starts=[]
for l in L:
    starts.append(depth);depth+=l.count('{')-l.count('}')
ln=0
while ln<n:
    l=L[ln];m=HEAD.match(l) if starts[ln]==0 and l and not l[0].isspace() and l[0] not in '}#' else None
    if m:
        d=0;seen=False;j=ln
        while j<n:
            d+=L[j].count('{')-L[j].count('}');seen=seen or '{' in L[j]
            if (seen and d<=0) or (not seen and L[j].rstrip().endswith(';')):break
            j+=1
        s=ln
        while s>0 and R[s-1].lstrip().startswith(('///','#[','//!')):s-=1
        if m.group(1)!='use':print(f"{s+1:5d}-{j+1:5d} {j-s+1:4d} {m.group(1):12s} {(l.strip()[:70] if m.group(1)=='impl' else (m.group(2) or '').strip())[:80]}")
        ln=j+1
    else: ln+=1
```

## Appendix F. Crate registry (frozen by WP0, generated by `rwp0-tools/crates.py` and `plan_appendix.py`)

`T/` = `E/🎯️targets/🧊️wgpu/`. The package folder of a crate is `T/<folder>/📦️packages/🦀️rust`, its library root `T/<folder>/🦀️.rs`. `rank` is the position in the dependency order of 4.3 (shell crates in the order of 7.3/7.7; companions last). Dependencies are the direct first-party edges declared in the manifest; native-only dependents are reached through the native target table.

| # | crate (`semio-framework-os-renderer-wgpu-…`) | folder under `T/` | WP | kind | cfg | rank | direct first-party dependencies (frozen; downward only) |
|---:|---|---|---:|---|---|---:|---|
| 1 | `contracts` | `🤝️contracts` | 1 | lib | native + wasm32 | 0 | - |
| 2 | `host-base` | `🏛️host-base` | 1 | lib | native + wasm32 | 1 | - |
| 3 | `native-platform` | `🖥️native-platform` | 1 | lib | native only | 2 | - |
| 4 | `media-slots` | `🎥️media-slots` | 1 | lib | native + wasm32 | 3 | - |
| 5 | `canvas2d-paint` | `🖌️canvas2d-paint` | 1 | lib | native + wasm32 | 4 | - |
| 6 | `asset-authority` | `🗂️asset-authority` | 1 | lib | native + wasm32 | 5 | `contracts` |
| 7 | `asset-decode` | `🦴️asset-decode` | 1 | lib | native + wasm32 | 6 | `contracts`, `host-base`, `asset-authority` |
| 8 | `doors` | `🛂️doors` | 1 | lib | native + wasm32 | 7 | `host-base` |
| 9 | `kernel-protocol` | `📡️kernel-protocol` | 1 | lib | native only | 8 | `contracts`, `host-base` |
| 10 | `kernel-retained` | `🗃️kernel-retained` | 1 | lib | native only | 9 | `kernel-protocol` |
| 11 | `kernel-pool` | `🏊️kernel-pool` | 1 | lib | native only | 10 | `contracts`, `host-base`, `kernel-protocol`, `kernel-retained` |
| 12 | `kernel-runtime` | `🧠️kernel-runtime` | 1 | lib | native only | 11 | `contracts`, `host-base`, `kernel-protocol`, `kernel-retained`, `kernel-pool` |
| 13 | `engine-registry` | `🗺️engine-registry` | 2 | lib | native + wasm32 | 12 | `contracts`, `asset-authority` |
| 14 | `engine-text` | `✍️engine-text` | 2 | lib | native + wasm32 | 13 | `contracts`, `engine-registry` |
| 15 | `engine-board` | `🎲️engine-board` | 2 | lib | native + wasm32 | 14 | `contracts`, `asset-authority`, `engine-registry` |
| 16 | `engine-graph` | `🕸️engine-graph` | 2 | lib | native + wasm32 | 15 | `contracts`, `host-base`, `media-slots`, `engine-registry`, `engine-board` |
| 17 | `engine-attach` | `🔗️engine-attach` | 2 | lib | native + wasm32 | 16 | `contracts`, `media-slots`, `canvas2d-paint`, `engine-registry`, `engine-text`, `engine-board`, `engine-graph` |
| 18 | `scene-state` | `🎭️scene-state` | 2 | lib | native + wasm32 | 17 | `contracts`, `host-base`, `native-platform`, `canvas2d-paint`, `engine-registry`, `engine-board` |
| 19 | `canvas2d-host` | `🔄️canvas2d-host` | 2 | lib | native + wasm32 | 18 | `contracts`, `scene-state` |
| 20 | `scene-canvas2d` | `🎨️scene-canvas2d` | 2 | lib | native + wasm32 | 19 | `contracts`, `media-slots`, `canvas2d-paint`, `asset-authority`, `scene-state`, `canvas2d-host` |
| 21 | `scene-ink` | `✒️scene-ink` | 2 | lib | native + wasm32 | 20 | `contracts`, `scene-state`, `scene-canvas2d` |
| 22 | `scene-feeds` | `📰️scene-feeds` | 2 | lib | native + wasm32 | 21 | `contracts`, `host-base`, `native-platform`, `engine-board`, `scene-state`, `scene-canvas2d` |
| 23 | `scene-tables` | `🧮️scene-tables` | 2 | lib | native + wasm32 | 22 | `contracts`, `media-slots`, `scene-state`, `scene-feeds` |
| 24 | `scene-text-vfs` | `📁️scene-text-vfs` | 2 | lib | native + wasm32 | 23 | `contracts`, `engine-text`, `engine-board`, `scene-state`, `scene-canvas2d`, `scene-feeds` |
| 25 | `icon-host` | `💠️icon-host` | 2 | lib | native + wasm32 | 24 | `contracts`, `host-base`, `media-slots`, `asset-authority`, `scene-state` |
| 26 | `scene-render` | `🪞️scene-render` | 2 | lib | native + wasm32 | 25 | `contracts`, `host-base`, `engine-registry`, `engine-board`, `engine-graph`, `engine-attach`, `scene-state`, `canvas2d-host`, `scene-canvas2d`, `scene-ink`, `scene-feeds`, `scene-tables`, `scene-text-vfs`, `icon-host` |
| 27 | `ui-engine-focus` | `👆️ui-engine-focus` | 3 | lib | native + wasm32 | 26 | `contracts`, `engine-registry`, `engine-text`, `engine-graph`, `scene-state`, `scene-ink`, `scene-tables`, `scene-text-vfs` |
| 28 | `ui-engine-dump` | `🔎️ui-engine-dump` | 3 | lib | native + wasm32 | 27 | `contracts`, `host-base`, `native-platform`, `media-slots`, `engine-text`, `scene-state`, `scene-feeds`, `ui-engine-focus` |
| 29 | `ui-engine` | `🖱️ui-engine` | 3 | lib | native + wasm32 | 28 | `contracts`, `host-base`, `engine-text`, `engine-board`, `scene-state`, `scene-canvas2d`, `scene-ink`, `scene-feeds`, `scene-tables`, `scene-text-vfs`, `scene-render`, `ui-engine-focus`, `ui-engine-dump` |
| 30 | `hub-contracts` | `🔐️hub-contracts` | 1 | lib | native + wasm32 | 29 | - |
| 31 | `hub-workspace` | `🏘️hub-workspace` | 1 | lib | native + wasm32 | 30 | `contracts`, `media-slots`, `hub-contracts` |
| 32 | `agent-elements` | `🙋️agent-elements` | 1 | lib | native + wasm32 | 31 | `host-base`, `media-slots`, `doors` |
| 33 | `dock` | `🛰️dock` | 1 | lib | native + wasm32 | 32 | `contracts` |
| 34 | `program-bridge` | `🪢️program-bridge` | 1 | lib | native + wasm32 | 33 | `contracts`, `host-base`, `media-slots`, `kernel-protocol`, `kernel-pool` |
| 35 | `shell-vocab` | `📖️shell-vocab` | 4 | shell | native + wasm32 | 34 | - |
| 36 | `shell-async` | `⏳️shell-async` | 4 | shell | native + wasm32 | 35 | `host-base` |
| 37 | `shell-chrome-state` | `🎚️shell-chrome-state` | 4 | shell | native + wasm32 | 36 | `contracts`, `host-base`, `ui-engine`, `agent-elements`, `dock`, `shell-vocab`, `shell-async` |
| 38 | `shell-surfaces` | `🫧️shell-surfaces` | 4 | shell | native + wasm32 | 37 | `contracts`, `host-base`, `engine-graph`, `scene-feeds`, `scene-text-vfs`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state` |
| 39 | `shell-plugins` | `🧲️shell-plugins` | 4 | shell | native + wasm32 | 38 | `contracts`, `host-base`, `engine-graph`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-surfaces` |
| 40 | `shell-history` | `⏪️shell-history` | 4 | shell | native + wasm32 | 39 | `contracts`, `host-base`, `agent-elements`, `shell-vocab`, `shell-async`, `shell-chrome-state` |
| 41 | `shell-prefs` | `🎛️shell-prefs` | 4 | shell | native + wasm32 | 40 | `contracts`, `host-base`, `ui-engine`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-plugins`, `shell-history` |
| 42 | `shell-directory` | `📒️shell-directory` | 4 | shell | native + wasm32 | 41 | `contracts`, `host-base`, `doors`, `engine-board`, `ui-engine`, `hub-contracts`, `hub-workspace`, `agent-elements`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-history`, `shell-prefs` |
| 43 | `shell-sync` | `🔃️shell-sync` | 4 | shell | native + wasm32 | 42 | `contracts`, `host-base`, `doors`, `hub-contracts`, `hub-workspace`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory` |
| 44 | `shell-docs` | `📄️shell-docs` | 4 | shell | native + wasm32 | 43 | `contracts`, `host-base`, `media-slots`, `ui-engine`, `hub-workspace`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync` |
| 45 | `shell-doc-open` | `📂️shell-doc-open` | 4 | shell | native + wasm32 | 44 | `contracts`, `host-base`, `icon-host`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs` |
| 46 | `shell-windows` | `🔲️shell-windows` | 4 | shell | native + wasm32 | 45 | `contracts`, `host-base`, `ui-engine`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-prefs`, `shell-directory`, `shell-docs` |
| 47 | `shell-dock` | `⚓️shell-dock` | 4 | shell | native + wasm32 | 46 | `contracts`, `host-base`, `engine-registry`, `ui-engine`, `agent-elements`, `dock`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows` |
| 48 | `shell-input` | `🕹️shell-input` | 4 | shell | native + wasm32 | 47 | `contracts`, `host-base`, `media-slots`, `scene-feeds`, `scene-render`, `ui-engine-dump`, `ui-engine`, `dock`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-history`, `shell-prefs`, `shell-docs`, `shell-windows`, `shell-dock` |
| 49 | `shell-commands` | `🎹️shell-commands` | 4 | shell | native + wasm32 | 48 | `contracts`, `host-base`, `shell-vocab`, `shell-async`, `shell-prefs`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input` |
| 50 | `shell-state` | `🐙️shell-state` | 4 | shell | native + wasm32 | 49 | `contracts`, `host-base`, `media-slots`, `doors`, `ui-engine`, `hub-contracts`, `hub-workspace`, `agent-elements`, `dock`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands` |
| 51 | `shell-session` | `🚶️shell-session` | 5 | shell | native + wasm32 | 50 | `contracts`, `host-base`, `engine-graph`, `engine-attach`, `scene-feeds`, `scene-text-vfs`, `scene-render`, `ui-engine`, `hub-workspace`, `dock`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state` |
| 52 | `shell-window-actions` | `🪝️shell-window-actions` | 5 | shell | native + wasm32 | 51 | `contracts`, `host-base`, `engine-registry`, `engine-text`, `dock`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state`, `shell-session` |
| 53 | `shell-actions` | `🔀️shell-actions` | 5 | shell | native + wasm32 | 52 | `contracts`, `host-base`, `ui-engine-dump`, `ui-engine`, `hub-contracts`, `hub-workspace`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state`, `shell-session`, `shell-window-actions` |
| 54 | `shell-pointer` | `🖐️shell-pointer` | 5 | shell | native + wasm32 | 53 | `contracts`, `host-base`, `engine-board`, `engine-graph`, `scene-state`, `scene-feeds`, `scene-render`, `ui-engine-dump`, `ui-engine`, `dock`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state`, `shell-session`, `shell-window-actions`, `shell-actions` |
| 55 | `shell-view-paint` | `🖍️shell-view-paint` | 5 | shell | native + wasm32 | 54 | `contracts`, `host-base`, `scene-state`, `scene-canvas2d`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state`, `shell-pointer` |
| 56 | `shell-view-chrome` | `🍱️shell-view-chrome` | 5 | shell | native + wasm32 | 55 | `contracts`, `host-base`, `scene-state`, `scene-canvas2d`, `scene-feeds`, `scene-render`, `ui-engine-dump`, `ui-engine`, `agent-elements`, `dock`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state`, `shell-session`, `shell-window-actions`, `shell-actions`, `shell-pointer`, `shell-view-paint` |
| 57 | `shell-view-overlays` | `🗨️shell-view-overlays` | 5 | shell | native + wasm32 | 56 | `contracts`, `host-base`, `scene-state`, `scene-canvas2d`, `ui-engine`, `hub-workspace`, `agent-elements`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state`, `shell-session`, `shell-pointer`, `shell-view-paint`, `shell-view-chrome` |
| 58 | `shell-view-panels` | `🪀️shell-view-panels` | 5 | shell | native + wasm32 | 57 | `contracts`, `host-base`, `native-platform`, `scene-state`, `scene-canvas2d`, `scene-feeds`, `icon-host`, `ui-engine-dump`, `ui-engine`, `hub-workspace`, `dock`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state`, `shell-session`, `shell-window-actions`, `shell-pointer`, `shell-view-paint`, `shell-view-chrome`, `shell-view-overlays` |
| 59 | `mailbox-core` | `📬️mailbox-core` | 1 | lib | every target | 58 | - |
| 60 | `host-plumbing` | `🔧️host-plumbing` | 3 | lib | native + wasm32 | 59 | `contracts`, `host-base`, `media-slots` |
| 61 | `frame-actions` | `🛎️frame-actions` | 3 | lib | native + wasm32 | 60 | `contracts`, `host-base`, `engine-text`, `engine-board`, `engine-graph`, `scene-feeds`, `scene-text-vfs`, `ui-engine-focus`, `ui-engine`, `shell-prefs`, `shell-input`, `shell-state`, `host-plumbing` |
| 62 | `runtime-mailbox` | `📨️runtime-mailbox` | 3 | lib | native + wasm32 | 61 | `contracts`, `host-base`, `asset-authority`, `asset-decode`, `engine-registry`, `engine-text`, `engine-board`, `scene-canvas2d`, `icon-host`, `ui-engine-focus`, `program-bridge`, `shell-pointer`, `shell-view-chrome`, `mailbox-core`, `host-plumbing`, `frame-actions` |
| 63 | `app-runtime` | `🏃️app-runtime` | 3 | lib | native + wasm32 | 62 | `contracts`, `host-base`, `kernel-protocol`, `kernel-runtime`, `engine-registry`, `engine-text`, `engine-board`, `engine-graph`, `scene-feeds`, `scene-text-vfs`, `icon-host`, `ui-engine-focus`, `ui-engine-dump`, `ui-engine`, `program-bridge`, `shell-docs`, `shell-input`, `shell-state`, `shell-session`, `shell-window-actions`, `shell-pointer`, `shell-view-chrome`, `shell-view-panels`, `mailbox-core`, `host-plumbing`, `frame-actions`, `runtime-mailbox` |
| 64 | `frame-presentation` | `📺️frame-presentation` | 3 | lib | native + wasm32 | 63 | `contracts`, `host-base`, `media-slots`, `engine-registry`, `engine-attach`, `ui-engine-dump`, `ui-engine`, `shell-view-panels`, `host-plumbing`, `frame-actions`, `runtime-mailbox`, `app-runtime` |
| 65 | `frame-transaction` | `🧾️frame-transaction` | 3 | lib | native + wasm32 | 64 | `host-base`, `engine-registry`, `engine-text`, `engine-board`, `scene-state`, `scene-canvas2d`, `icon-host`, `ui-engine-focus`, `ui-engine-dump`, `ui-engine`, `program-bridge`, `shell-prefs`, `shell-input`, `shell-state`, `mailbox-core`, `frame-actions`, `runtime-mailbox`, `app-runtime`, `frame-presentation` |
| 66 | `frame-job` | `👷️frame-job` | 3 | lib | native + wasm32 | 65 | `host-base`, `ui-engine-dump`, `frame-actions`, `runtime-mailbox`, `app-runtime`, `frame-transaction` |
| 67 | `native-probe` | `🔬️native-probe` | 3 | lib | native + wasm32 | 66 | `host-base`, `native-platform`, `canvas2d-host`, `program-bridge`, `shell-prefs`, `shell-sync`, `shell-state`, `shell-window-actions`, `shell-actions`, `shell-pointer`, `host-plumbing`, `app-runtime` |
| 68 | `host-winit` | `🎪️host-winit` | 3 | lib | native + wasm32 | 67 | `contracts`, `host-base`, `native-platform`, `media-slots`, `asset-authority`, `kernel-protocol`, `kernel-pool`, `kernel-runtime`, `engine-registry`, `scene-state`, `shell-state`, `shell-window-actions`, `host-plumbing`, `frame-actions`, `runtime-mailbox`, `app-runtime`, `frame-presentation`, `frame-transaction`, `frame-job` |
| 69 | `browser-worker` | `🌍️browser-worker` | 3 | lib | wasm32-unknown-unknown only | 68 | `contracts`, `host-base`, `asset-authority`, `engine-registry`, `icon-host`, `program-bridge`, `shell-prefs`, `shell-input`, `shell-state`, `shell-session`, `shell-actions`, `mailbox-core`, `host-plumbing`, `frame-actions`, `runtime-mailbox`, `app-runtime`, `frame-presentation`, `frame-job`, `native-probe`, `host-winit` |
| 70 | `test-support` | `🧯️test-support` | 0 | support | native + wasm32 | 140 | - |
| 71 | `engine-attach-tests` | `🧷️engine-attach-tests` | 2 | companion | native + wasm32 | 141 | `engine-attach`, `engine-registry`, `engine-text`, `engine-board`, `engine-graph`, `contracts`, `media-slots`, `canvas2d-paint`, `host-base` |
| 72 | `scene-ink-tests` | `🧶️scene-ink-tests` | 2 | companion | native + wasm32 | 142 | `scene-ink`, `scene-state`, `scene-canvas2d`, `contracts`, `engine-registry`, `engine-board` |
| 73 | `ui-engine-tests` | `🧿️ui-engine-tests` | 3 | companion | native + wasm32 | 143 | `ui-engine`, `ui-engine-focus`, `ui-engine-dump`, `scene-render`, `scene-state`, `engine-text`, `engine-board`, `contracts`, `host-base` |
| 74 | `runtime-mailbox-tests` | `🪤️runtime-mailbox-tests` | 3 | companion | native + wasm32 | 144 | `runtime-mailbox`, `mailbox-core`, `frame-actions`, `host-plumbing`, `host-base`, `contracts`, `asset-authority`, `asset-decode`, `engine-registry`, `engine-text`, `engine-board`, `scene-canvas2d`, `ui-engine-focus`, `program-bridge`, `test-support` |
| 75 | `shell-tests-windows-1` | `🏙️shell-tests-windows-1` | 4 | shell-tests | native + wasm32 | 145 | `contracts`, `host-base`, `hub-workspace`, `agent-elements`, `dock`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state`, `shell-session`, `shell-window-actions`, `shell-actions`, `shell-pointer`, `shell-view-paint`, `shell-view-chrome`, `shell-view-overlays`, `shell-view-panels`, `test-support`, `ui-engine-tests` |
| 76 | `shell-tests-windows-2` | `🌆️shell-tests-windows-2` | 4 | shell-tests | native + wasm32 | 146 | `contracts`, `host-base`, `dock`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state`, `shell-session`, `shell-window-actions`, `shell-pointer`, `shell-view-paint`, `shell-view-chrome`, `shell-view-panels`, `test-support`, `ui-engine-tests` |
| 77 | `shell-tests-chrome-1` | `🌈️shell-tests-chrome-1` | 5 | shell-tests | native + wasm32 | 147 | `contracts`, `host-base`, `shell-vocab`, `shell-async`, `shell-state`, `test-support`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-session`, `shell-window-actions`, `shell-actions`, `shell-pointer`, `shell-view-paint`, `shell-view-chrome`, `shell-view-overlays`, `shell-view-panels` |
| 78 | `shell-tests-chrome-2` | `🌌️shell-tests-chrome-2` | 5 | shell-tests | native + wasm32 | 148 | `contracts`, `host-base`, `shell-vocab`, `shell-async`, `shell-state`, `test-support`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-session`, `shell-window-actions`, `shell-actions`, `shell-pointer`, `shell-view-paint`, `shell-view-chrome`, `shell-view-overlays`, `shell-view-panels` |
| 79 | `shell-tests-chrome-3` | `🌅️shell-tests-chrome-3` | 5 | shell-tests | native + wasm32 | 149 | `contracts`, `host-base`, `dock`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state`, `shell-session`, `shell-window-actions`, `shell-actions`, `shell-pointer`, `shell-view-paint`, `shell-view-chrome`, `shell-view-overlays`, `shell-view-panels`, `test-support` |
| 80 | `shell-tests-chrome-4` | `🌴️shell-tests-chrome-4` | 5 | shell-tests | native + wasm32 | 150 | `contracts`, `host-base`, `ui-engine-dump`, `ui-engine`, `dock`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state`, `shell-session`, `shell-window-actions`, `shell-actions`, `shell-pointer`, `shell-view-paint`, `shell-view-chrome`, `shell-view-overlays`, `shell-view-panels`, `test-support`, `ui-engine-tests` |
| 81 | `shell-tests-chrome-5` | `🌲️shell-tests-chrome-5` | 5 | shell-tests | native + wasm32 | 151 | `contracts`, `host-base`, `ui-engine-dump`, `ui-engine`, `dock`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state`, `shell-session`, `shell-window-actions`, `shell-actions`, `shell-pointer`, `shell-view-paint`, `shell-view-chrome`, `shell-view-overlays`, `shell-view-panels`, `test-support`, `ui-engine-tests` |
| 82 | `shell-tests-chrome-6` | `🍃️shell-tests-chrome-6` | 5 | shell-tests | native + wasm32 | 152 | `contracts`, `host-base`, `hub-workspace`, `agent-elements`, `dock`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state`, `shell-session`, `shell-window-actions`, `shell-actions`, `shell-pointer`, `shell-view-paint`, `shell-view-chrome`, `shell-view-overlays`, `shell-view-panels`, `test-support` |
| 83 | `shell-tests-input-1` | `🍎️shell-tests-input-1` | 5 | shell-tests | native + wasm32 | 153 | `contracts`, `host-base`, `shell-vocab`, `shell-async`, `shell-state`, `test-support`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-session`, `shell-window-actions`, `shell-actions`, `shell-pointer`, `shell-view-paint`, `shell-view-chrome`, `shell-view-overlays`, `shell-view-panels` |
| 84 | `shell-tests-input-2` | `🍯️shell-tests-input-2` | 5 | shell-tests | native + wasm32 | 154 | `contracts`, `host-base`, `doors`, `ui-engine-dump`, `dock`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state`, `shell-session`, `shell-window-actions`, `shell-actions`, `shell-pointer`, `shell-view-paint`, `shell-view-chrome`, `shell-view-overlays`, `shell-view-panels`, `test-support`, `ui-engine-tests` |
| 85 | `shell-tests-input-3` | `🍰️shell-tests-input-3` | 5 | shell-tests | native + wasm32 | 155 | `contracts`, `host-base`, `native-platform`, `ui-engine-dump`, `dock`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state`, `shell-session`, `shell-window-actions`, `shell-actions`, `shell-pointer`, `shell-view-paint`, `shell-view-chrome`, `shell-view-overlays`, `shell-view-panels`, `test-support`, `ui-engine-tests` |
| 86 | `shell-tests-docs-1` | `🥧️shell-tests-docs-1` | 4 | shell-tests | native + wasm32 | 156 | `contracts`, `host-base`, `shell-vocab`, `shell-async`, `shell-state`, `test-support`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands` |
| 87 | `shell-tests-docs-2` | `🥪️shell-tests-docs-2` | 4 | shell-tests | native + wasm32 | 157 | `contracts`, `host-base`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state`, `shell-session`, `shell-window-actions`, `shell-actions`, `shell-view-chrome`, `shell-view-overlays`, `shell-view-panels`, `test-support` |
| 88 | `shell-tests-docs-3` | `🥣️shell-tests-docs-3` | 4 | shell-tests | native + wasm32 | 158 | `contracts`, `host-base`, `icon-host`, `ui-engine-dump`, `dock`, `program-bridge`, `shell-vocab`, `shell-async`, `shell-chrome-state`, `shell-surfaces`, `shell-plugins`, `shell-history`, `shell-prefs`, `shell-directory`, `shell-sync`, `shell-docs`, `shell-doc-open`, `shell-windows`, `shell-dock`, `shell-input`, `shell-commands`, `shell-state`, `shell-session`, `shell-window-actions`, `shell-actions`, `shell-pointer`, `shell-view-chrome`, `shell-view-panels`, `test-support`, `ui-engine-tests` |

## Appendix G. Hub units (the 61 `mod` mounts of the five hubs, test modules included)

State at the end of WP0. Section 10.5 records which of these units were replaced by `extern crate`, `use` or `pub use` lines of a crate by the integration pass (`asset_authority`, `glb_decode`, `asset_decode_jobs`, `native_temporal`, `deadlines`, `native_caller`, `render_snapshot`, `browser_original_output`, `surface_lane`, `input_wire` leave the hubs as `mod` units; `interpreter` stays a facade).

The two `include!`d standalone test files of the WP3 hub (`🧪️tests/🧊️wgpu-renderer-standalone`, `🧪️tests/🐕️wgpu-present-stall-watch`) are no modules and are not listed; they go to `native-probe` and `frame-presentation`. A unit that maps to one crate is replaced by the line in the last column when the crate is carved (10.4.4).

| hub (WP) | module | cfg | target crate(s) | after a single-crate carve |
|---|---|---|---|---|
| WP1 | `dock` | - | dock | `extern crate semio_framework_os_renderer_wgpu_dock as dock;` |
| WP1 | `mesh_assets` | - | host-base | `extern crate semio_framework_os_renderer_wgpu_host_base as mesh_assets;` |
| WP1 | `agent_bridge` | - | agent-elements | `extern crate semio_framework_os_renderer_wgpu_agent_elements as agent_bridge;` |
| WP1 | `agent_presence` | - | agent-elements | `extern crate semio_framework_os_renderer_wgpu_agent_elements as agent_presence;` |
| WP1 | `agent_approvals` | - | agent-elements | `extern crate semio_framework_os_renderer_wgpu_agent_elements as agent_approvals;` |
| WP1 | `agent_chat_panel` | - | agent-elements | `extern crate semio_framework_os_renderer_wgpu_agent_elements as agent_chat_panel;` |
| WP1 | `agent_bridge_door` | - | agent-elements | `extern crate semio_framework_os_renderer_wgpu_agent_elements as agent_bridge_door;` |
| WP1 | `runtime_assets` | - | host-base | `extern crate semio_framework_os_renderer_wgpu_host_base as runtime_assets;` |
| WP1 | `space_administration` | - | hub-workspace | `extern crate semio_framework_os_renderer_wgpu_hub_workspace as space_administration;` |
| WP1 | `hub_sign_in` | - | hub-contracts | `extern crate semio_framework_os_renderer_wgpu_hub_contracts as hub_sign_in;` |
| WP1 | `space_browser` | - | hub-contracts | `extern crate semio_framework_os_renderer_wgpu_hub_contracts as space_browser;` |
| WP1 | `hub_connection` | - | hub-workspace | `extern crate semio_framework_os_renderer_wgpu_hub_workspace as hub_connection;` |
| WP1 | `canvas_presence` | - | hub-workspace | `extern crate semio_framework_os_renderer_wgpu_hub_workspace as canvas_presence;` |
| WP1 | `element_contracts` | - | contracts | `extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;` |
| WP1 | `media_slots` | - | media-slots | `extern crate semio_framework_os_renderer_wgpu_media_slots as media_slots;` |
| WP1 | `program_bridge` | - | program-bridge | `extern crate semio_framework_os_renderer_wgpu_program_bridge as program_bridge;` |
| WP1 | `plugin_catalog` | - | host-base | `extern crate semio_framework_os_renderer_wgpu_host_base as plugin_catalog;` |
| WP1 | `canvas2d_paint` | - | canvas2d-paint | `extern crate semio_framework_os_renderer_wgpu_canvas2d_paint as canvas2d_paint;` |
| WP1 | `native_temporal` | `not(target_arch = "wasm32")` | native-platform | `#[cfg(not(target_arch = "wasm32"))] extern crate semio_framework_os_renderer_wgpu_native_platform as native_temporal;` |
| WP1 | `parallel_runtime` | `not(target_arch = "wasm32")` | kernel-protocol | `#[cfg(not(target_arch = "wasm32"))] extern crate semio_framework_os_renderer_wgpu_kernel_protocol as parallel_runtime;` |
| WP1 | `asset_authority` | - | asset-authority | `extern crate semio_framework_os_renderer_wgpu_asset_authority as asset_authority;` |
| WP1 | `glb_decode` | - | asset-decode | `extern crate semio_framework_os_renderer_wgpu_asset_decode as glb_decode;` |
| WP1 | `asset_decode_jobs` | - | asset-decode (renderer I/O slots: host-base) | keep the mount; the parent file in your tree becomes the facade |
| WP1 | `scale_bench` | `not(target_arch = "wasm32")` | kernel-runtime | `#[cfg(not(target_arch = "wasm32"))] extern crate semio_framework_os_renderer_wgpu_kernel_runtime as scale_bench;` |
| WP1 | `kernel_runtime` | `not(target_arch = "wasm32")` | kernel-protocol, kernel-retained, kernel-pool, kernel-runtime (facade module) | keep the mount; the parent file in your tree becomes the facade |
| WP1 | `boot_axis_parity_tests` | `test` | host-base | keep the mount; the parent file in your tree becomes the facade |
| WP1 | `frame_latency` | - | host-base | `extern crate semio_framework_os_renderer_wgpu_host_base as frame_latency;` |
| WP1 | `runtime_mailbox_core` | - | mailbox-core | `extern crate semio_framework_os_renderer_wgpu_mailbox_core as runtime_mailbox_core;` |
| WP1 | `socket_door` | - | doors | `extern crate semio_framework_os_renderer_wgpu_doors as socket_door;` |
| WP1 | `directory_door` | - | doors | `extern crate semio_framework_os_renderer_wgpu_doors as directory_door;` |
| WP1 | `native_accessibility` | `not(target_arch = "wasm32")` | native-platform | `#[cfg(not(target_arch = "wasm32"))] extern crate semio_framework_os_renderer_wgpu_native_platform as native_accessibility;` |
| WP1 | `native_runtime_modules` | `not(target_arch = "wasm32")` | program-bridge | `#[cfg(not(target_arch = "wasm32"))] extern crate semio_framework_os_renderer_wgpu_program_bridge as native_runtime_modules;` |
| WP2 | `engine_canvas` | - | engine-registry, engine-text, engine-board, engine-graph, engine-attach (facade module) | keep the mount; the parent file in your tree becomes the facade |
| WP2 | `scenes` | - | scene-state, scene-canvas2d, scene-ink, scene-feeds, scene-tables, scene-text-vfs, scene-render, icon-host (facade module) | keep the mount; the parent file in your tree becomes the facade |
| WP2 | `canvas2d_gumball` | - | canvas2d-host (gumball types: scene-state) | `extern crate semio_framework_os_renderer_wgpu_canvas2d_host as canvas2d_gumball;` |
| WP2 | `icon_atlas` | - | icon-host | `extern crate semio_framework_os_renderer_wgpu_icon_host as icon_atlas;` |
| WP3 | `fixture_original_turn` | `test` | ui-engine-tests | keep the mount; the parent file in your tree becomes the facade |
| WP3 | `interpreter` | - | ui-engine-focus, ui-engine-dump, ui-engine (facade module) | keep the mount; the parent file in your tree becomes the facade |
| WP3 | `frame_actions` | - | frame-actions (RuntimeApply: runtime-mailbox) | keep the mount; the parent file in your tree becomes the facade |
| WP3 | `presentation_raster` | - | runtime-mailbox | `extern crate semio_framework_os_renderer_wgpu_runtime_mailbox as presentation_raster;` |
| WP3 | `mailbox_impl` | - | runtime-mailbox | `extern crate semio_framework_os_renderer_wgpu_runtime_mailbox as mailbox_impl;` |
| WP3 | `frame_build` | - | app-runtime, frame-transaction | keep the mount; the parent file in your tree becomes the facade |
| WP3 | `frame_presentation` | - | frame-presentation (AppFramePreparation: app-runtime) | keep the mount; the parent file in your tree becomes the facade |
| WP3 | `app_runtime` | - | app-runtime (boot_runtime: frame-transaction) | keep the mount; the parent file in your tree becomes the facade |
| WP3 | `native_socket_probe` | - | native-probe | `extern crate semio_framework_os_renderer_wgpu_native_probe as native_socket_probe;` |
| WP3 | `async_boundary_tests` | `test` | runtime-mailbox-tests | keep the mount; the parent file in your tree becomes the facade |
| WP3 | `frame_action_ledger_tests` | `test` | frame-actions | keep the mount; the parent file in your tree becomes the facade |
| WP3 | `wheel_application_point_tests` | `test` | frame-actions | keep the mount; the parent file in your tree becomes the facade |
| WP3 | `deadlines` | - | host-plumbing | `extern crate semio_framework_os_renderer_wgpu_host_plumbing as deadlines;` |
| WP3 | `os_host` | - | host-winit | `extern crate semio_framework_os_renderer_wgpu_host_winit as os_host;` |
| WP3 | `native_caller` | `not(target_arch="wasm32")` | host-plumbing | `#[cfg(not(target_arch="wasm32"))] extern crate semio_framework_os_renderer_wgpu_host_plumbing as native_caller;` |
| WP3 | `render_snapshot` | - | host-plumbing | `extern crate semio_framework_os_renderer_wgpu_host_plumbing as render_snapshot;` |
| WP3 | `browser_original_output` | - | host-plumbing | `extern crate semio_framework_os_renderer_wgpu_host_plumbing as browser_original_output;` |
| WP3 | `frame_job` | - | frame-job | `extern crate semio_framework_os_renderer_wgpu_frame_job as frame_job;` |
| WP3 | `frame_authority` | - | frame-job | `extern crate semio_framework_os_renderer_wgpu_frame_job as frame_authority;` |
| WP3 | `surface_lane` | - | host-plumbing | `extern crate semio_framework_os_renderer_wgpu_host_plumbing as surface_lane;` |
| WP3 | `input_wire` | - | host-plumbing | `extern crate semio_framework_os_renderer_wgpu_host_plumbing as input_wire;` |
| WP3 | `browser_worker` | `target_arch = "wasm32"` | browser-worker | `#[cfg(target_arch = "wasm32")] extern crate semio_framework_os_renderer_wgpu_browser_worker as browser_worker;` |
| WP3 | `winit_app` | - | host-winit | `extern crate semio_framework_os_renderer_wgpu_host_winit as winit_app;` |
| WP4 | `shell` | - | shell-* (facade module until S0 and WP5 land) | keep the mount; the parent file in your tree becomes the facade |
| WP5 | `settle_pump_tests` | `test` | shell-tests-chrome-6 | keep the mount; the parent file in your tree becomes the facade |

## Appendix H. Item-move groups of 5.2 by package (the 12 groups that cross packages are lifted through `📥️lifted`)

| destination crate (WP) | from file | default crate of that file (WP) | lines | lift file to create (source WP) |
|---|---|---|---:|---|
| `contracts` (WP1) | `🗃️scene-state` | `scene-state` (WP2) | 307 | `🤝️contracts/📥️lifted/🗃️scene-state/🦀️.rs` (WP2) |
| `contracts` (WP1) | `🔖️identity-and-types` | `shell-*` (WP4) | 135 | `🤝️contracts/📥️lifted/🔖️identity-and-types/🦀️.rs` (WP4) |
| `contracts` (WP1) | `🧩️retained-ui-engine` | `ui-engine` (WP3) | 117 | `🤝️contracts/📥️lifted/🧩️retained-ui-engine/🦀️.rs` (WP3) |
| `contracts` (WP1) | `🌉️ProgramBridge` | `program-bridge` (WP1) | 41 | same WP: move inside your tree |
| `contracts` (WP1) | `🖼️ui-image-loading` | `scene-canvas2d` (WP2) | 19 | `🤝️contracts/📥️lifted/🖼️ui-image-loading/🦀️.rs` (WP2) |
| `contracts` (WP1) | `🧱️chrome-paint-and-navbar` | `shell-*` (WP4) | 14 | `🤝️contracts/📥️lifted/🧱️chrome-paint-and-navbar/🦀️.rs` (WP4) |
| `contracts` (WP1) | `🗂️engine-surface-registry` | `engine-registry` (WP2) | 5 | `🤝️contracts/📥️lifted/🗂️engine-surface-registry/🦀️.rs` (WP2) |
| `host-base` (WP1) | `🔁️asset-decode-jobs` | `asset-decode` (WP1) | 356 | same WP: move inside your tree |
| `host-base` (WP1) | `🎨️theme-file-import-and-accessibility` | `shell-*` (WP4) | 89 | `🏛️host-base/📥️lifted/🎨️theme-file-import-and-accessibility/🦀️.rs` (WP4) |
| `host-base` (WP1) | `🏊️kernel-pool` | `kernel-pool` (WP1) | 48 | same WP: move inside your tree |
| `host-base` (WP1) | `🌐️browser-worker` | `browser-worker` (WP3) | 8 | `🏛️host-base/📥️lifted/🌐️browser-worker/🦀️.rs` (WP3) |
| `host-base` (WP1) | `🎨️presentation-raster` | `runtime-mailbox` (WP3) | 4 | `🏛️host-base/📥️lifted/🎨️presentation-raster/🦀️.rs` (WP3) |
| `native-platform` (WP1) | `🎞️Scenes` | `scene-state` (WP2) | 16 | `🖥️native-platform/📥️lifted/🎞️Scenes/🦀️.rs` (WP2) |
| `asset-authority` (WP1) | `🔁️asset-decode-jobs` | `asset-decode` (WP1) | 22 | same WP: move inside your tree |
| `kernel-protocol` (WP1) | `📬️kernel-replay-client` | `kernel-pool` (WP1) | 1089 | same WP: move inside your tree |
| `kernel-protocol` (WP1) | `🏊️kernel-pool` | `kernel-pool` (WP1) | 247 | same WP: move inside your tree |
| `kernel-protocol` (WP1) | `📦️kernel-runtime` | `kernel-runtime` (WP1) | 31 | same WP: move inside your tree |
| `kernel-pool` (WP1) | `🗃️kernel-retained-state` | `kernel-retained` (WP1) | 54 | same WP: move inside your tree |
| `engine-registry` (WP2) | `✍️text-editor` | `engine-text` (WP2) | 218 | same WP: move inside your tree |
| `engine-registry` (WP2) | `🕸️node-graph` | `engine-graph` (WP2) | 26 | same WP: move inside your tree |
| `engine-registry` (WP2) | `🧩️engine-surface-attach` | `engine-attach` (WP2) | 9 | same WP: move inside your tree |
| `scene-state` (WP2) | `📐️Canvas2dHost` | `canvas2d-host` (WP2) | 203 | same WP: move inside your tree |
| `scene-state` (WP2) | `🖌️canvas2d` | `scene-canvas2d` (WP2) | 125 | same WP: move inside your tree |
| `scene-state` (WP2) | `⌨️scene-input` | `scene-render` (WP2) | 88 | same WP: move inside your tree |
| `scene-state` (WP2) | `🏠️os-host` | `host-winit` (WP3) | 69 | `🎭️scene-state/📥️lifted/🏠️os-host/🦀️.rs` (WP3) |
| `scene-state` (WP2) | `🎨️scene-render-entry` | `scene-render` (WP2) | 57 | same WP: move inside your tree |
| `scene-state` (WP2) | `🧮️table-and-paint2d` | `scene-tables` (WP2) | 43 | same WP: move inside your tree |
| `scene-state` (WP2) | `🧭️node-graph-and-tiled-map` | `scene-feeds` (WP2) | 26 | same WP: move inside your tree |
| `scene-state` (WP2) | `✍️text-editor` | `scene-text-vfs` (WP2) | 17 | same WP: move inside your tree |
| `scene-state` (WP2) | `✒️ink-canvas-model-state` | `scene-ink` (WP2) | 16 | same WP: move inside your tree |
| `scene-state` (WP2) | `🎯️chrome-ledger` | `ui-engine-dump` (WP3) | 15 | `🎭️scene-state/📥️lifted/🎯️chrome-ledger/🦀️.rs` (WP3) |
| `scene-state` (WP2) | `🖋️ink-canvas-render` | `scene-ink` (WP2) | 6 | same WP: move inside your tree |
| `scene-state` (WP2) | `🧩️board2d-and-vfs` | `scene-text-vfs` (WP2) | 5 | same WP: move inside your tree |
| `ui-engine-focus` (WP3) | `🧩️retained-ui-engine` | `ui-engine` (WP3) | 288 | same WP: move inside your tree |
| `host-plumbing` (WP3) | `🪟️winit-app` | `host-winit` (WP3) | 37 | same WP: move inside your tree |
| `frame-actions` (WP3) | `🧵️frame-job` | `frame-job` (WP3) | 11 | same WP: move inside your tree |
| `frame-actions` (WP3) | `🎨️presentation-raster` | `runtime-mailbox` (WP3) | 1 | same WP: move inside your tree |
| `runtime-mailbox` (WP3) | `🛎️frame-actions` | `frame-actions` (WP3) | 370 | same WP: move inside your tree |
| `runtime-mailbox` (WP3) | `🧠️app-runtime` | `app-runtime` (WP3) | 134 | same WP: move inside your tree |
| `runtime-mailbox` (WP3) | `🧾️frame-build` | `app-runtime` (WP3) | 2 | same WP: move inside your tree |
| `app-runtime` (WP3) | `🖥️frame-presentation` | `frame-presentation` (WP3) | 262 | same WP: move inside your tree |
| `app-runtime` (WP3) | `🪟️winit-app` | `host-winit` (WP3) | 137 | same WP: move inside your tree |
| `app-runtime` (WP3) | `🔬️native-socket-probe` | `native-probe` (WP3) | 3 | same WP: move inside your tree |
| `frame-transaction` (WP3) | `🧾️frame-build` | `app-runtime` (WP3) | 781 | same WP: move inside your tree |
| `frame-transaction` (WP3) | `🧠️app-runtime` | `app-runtime` (WP3) | 134 | same WP: move inside your tree |
