# 🪓️ Plan: Crate Partition of `semio-framework-plugin`

Planner: Sonnet 5.5, read-only, foreground, no sub-agents. Consumers: wave-4 executors. This is a plan: nothing was edited, compiled or committed. Every number was measured on the live working tree on 2026-10-15 with the canonical counter `readRustSourceClosureV1` and a token-level item graph; Appendices F and G contain the programs, so any executor can re-measure in minutes. Base state: the plan sits on top of the earlier splits (`RESULT-PLUGIN-INSTANCE-HOST-SPLIT.md`, instance and host already cut into band files) and on top of `RESULT-RELOCATE-PLUGIN-SDK.md` (time-travel and tool-run turns already moved into their domain modules behind `InstanceExtension`; `os_vcs` and `ui_wgpu` already gone from the host). None of those splits has ever been compiled (the compiler-ingress gate refuses `semio-framework`, a dependency of every carved crate, and this crate is 23 times over the byte ceiling by itself), so every claim of "this edge exists" below is a token-level claim that the compiler must confirm.

Notation. `PLUG/` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`. `PKG` = `PLUG/📦️packages/🦀️rust/`. A crate named `x` below is `semio-framework-plugin-x` (Nx project `@semio-tech/plugin-x-rs`, `[package.metadata.semio] id = "plugin-x"`). "Lines" are the gate's active lines (blank and comment lines included). A path with a leading `†` is a new directory that WP0 must register in `🔣️taxonomy.json` first. "WP" is a work package (section 12). The old crate keeps its name and becomes the **facade** (section 10).

**WP0 executed 2026-10-15 (Sonnet 5.5, foreground; evidence in `RESULT-PLUGIN-WP0.md`).** The census was re-run and the inventory reconciled (Appendix A rows marked `[WP0 change]`), 80 skeleton crates, seven mount hubs, three facade macro mounts, 26 lift files and all registries exist, and the hand-off contract is section 12.5 with Appendices H to K. The tree is still **one** compilation unit and resolution-equivalent to the tree this plan was written against. Where a statement below differs from what phase 0 created, the difference is marked `[WP0 change]`. Tools: `pwp0-tools/` of this ticket folder.


## 0. Verdict in twelve lines

1. Today the lib closure is **185 files / 6,007,520 bytes / 83,182 lines** (native), 189 / 6,075,698 / 84,493 (wasm32-wasip2: four more wasip2-only files), and 211 / 6,512,773 / 88,660 with the four testing features (10 embedded fixtures). The test closure is **729 files / 12,793,280 bytes / 145,762 lines** (270 embedded files). The manifest declares `maxSourceBytes = 1048576, maxSourceFiles = 128, maxActiveLines = 12000`, which is not the ceiling triple of the recipe; a real compile is not possible before the gate admits the closure.
2. The binding limits are **bytes and lines**: 6.01 MB / 230,000 B = 27 crates minimum for the lib closure, 83,182 / 3,500 = 24; the test closure needs 56 by bytes and 42 by lines. The plan proposes **58 lib crates** (41 "lower" crates, 16 instance-family / builder / testing crates, and the existing crate as thin facade) plus **21 companion test crates** (14 packed bins plus 7 bins for the two oversize test files). Every lib crate is at or below 3,500 lines / 28 files / 230,000 bytes (estimates, section 6; the largest lower crates are `decl-views` 2,832 item lines / 182 KB and `window-config` 2,600 / 182 KB; the tightest instance crate is `instance-jobs` at about 2,300 lines).
3. At **file level** the production graph is one strongly connected component of 82 of the 176 files that hold production items (about 55,000 lines, 66 percent of the code). At **item level** (3,540 items, 2,919 production units after dropping 174 `cfg(test)` items; inherent impls bound to their type) it is almost acyclic: besides the god object (a pair with a 5-line struct) and the `PluginBuilder` / `Plugin` pair (976 lines, inversion I2) there are exactly **two real knots**, the declaration contract knot (26 units, 1,384 lines: `ArtifactApp`, the tool-job contract, `EphemeralEmit`, `CompletionCell`, `ToolRunJobRequest`, ...) and the reactor lifecycle knot (29 units, 1,131 lines: `PendingPatchAuthority`, `NativeLifecycleRegistry`, `PluginRuntime`, `PluginInstanceCloseLease`, `PatchTrackerState`, ...); each stays whole inside one crate. The other 19 cycles are at most 6 units / 670 lines (23 non-trivial SCCs in all). The file boundaries chosen by the earlier splits are the wrong cut surface, not the code.
4. After the cuts of section 5 the 41 lower crates form a DAG of **16 layers with zero back edges**. The cuts are **196 item moves of 3,352 lines (4.0 percent of the code)** plus 38 knot-member relocations (692 lines), and eight explicit inversions or decoupling moves (5.3). Four of the moves are cuts of single big files: the 5,465-line runtime file into five crates, the 3,444-line `time-travel` file into three, `reactor/🦀️.rs` into four, and the 594-line inline `wit_bridge` module into its own wasip2-only crate.
5. **God object 1, the artifact instance.** `VcsArtifactApp<A, M>` has **227 fields, 26 inherent impl blocks + `Drop` + `PluginApp`, 674 methods, 14,971 lines in 21 files**. Section 7 decomposes it into 19 component structs (fields and methods together) in five tier-1 crates (plus the small hook crate `instance-extension`), four use-case crates and two feature-turn crates over borrowed bundles, and a composer `instance-app` that holds the struct and the `PluginApp` impl (a trait impl must live in the crate that owns the type or the trait; the trait is below, so the composer is the top). No extension traits. The one existing trait seam, `InstanceExtension`, becomes a type parameter `X` of the use-case functions so that feature crates (time-travel, tool-run) sit above the instance, not below.
6. **God object 2, reactor runtime state.** It is not an impl problem: `PluginRuntime<PA>` has 14 fields and a 24-line impl. The 391 KB runtime file is a bag of 176 free functions, 24 structs, 10 enums, 5 macros over it, plus a 16-static `component_persistent_local!` block in `reactor/🦀️.rs`. Section 7.8 cuts the functions into five function-family crates and gives every static one owning crate (the three cross-crate statics are listed).
7. **Wrong-direction domains.** `tool-run` / `time-travel`: the generic instance still names them at 30 call sites, 2 fields, 4 `PluginApp` contract methods and 2 ladder rungs; Phase 1 (this plan) makes them lower crates that the composer depends on and puts their *turns* above the generic use-case crates, Phase 2 (optional, section 8) moves the two ledgers behind `InstanceExtension::State` and removes the last names. `ui_wgpu`: the dependency is **dead** (no code mention left in the crate; remove it from the manifest, section 8). `os_vcs` in `GuestRuntime` belongs to `semio-framework-plugin-host`, a separate crate that does not depend on this one; nothing to do here.
8. **cfg targets.** No crate is native-only. wasm32-wasip2-only code is 1,311 lines in three places: the `abi` wire bindings and owned ABI, the `reactor-context` guest exports file, and the new `reactor-wit-bridge` crate (whole-crate gate by a target-specific dependency of the facade). 67 `cfg(all(wasm32, p2))` sites, 12 `cfg(wasm32)` sites and 6 `not(p2)` fallbacks are all inside single files that stay in one crate (section 9).
9. **Facade.** 4,830 Rust files name `semio_framework_plugin`; they all keep working: the facade keeps every root name and every `pub mod` path (`app`, `retained_command`, `plugin_runtime`, `reactor`, `describe`, `host`, `declarations`, ...) through explicit `pub use` lists, keeps the 18 `#[macro_export]` macros (15 live in the facade, 3 stay with the crate that uses them), forwards the 5 Cargo features, and loses exactly one inherent method (`Plugin::builder`, two callers, both in the crate's own testing module).
10. **Tests are the larger cost.** 274 test-only files (60,546 lines, 4.98 MB) + 269 embedded fixtures (1.73 MB; six fixtures of 40 KB to 859 KB are 1.26 MB and must be read at run time) + **25 `include_str!("....rs")` source embeds in 10 Rust test files** + **532 `🦀️.rs` path literals in 104 TS and Rust readers (283 distinct targets)**. 34 of the 41 lower crates keep their tests in-crate; seven lower crates, the instance family, the builder and the facade use 21 companion test crates (section 11).
11. **Parallelism.** WP0 (serial, first) → WP1..WP5 (lower crates, disjoint files) and WP6/WP7 (instance family, after WP0's freeze of component and bundle names) in parallel → WP0 again (facade, registries, readers, gates). Staging files (`📥️lifted/`) carry every cross-package item move, so no executor edits another's file.
12. **Nothing here could be compiled**: verification is the full-resolution differential per crate root plus census, definition scan, rustfmt and path resolution (section 13). The token graph is blind to method-call edges and ignores about two dozen generic names; the three risks that matter most are the orphan-rule placement of the `PluginApp` impl, the `dyn_enum` / `$crate` macro closures, and the 2,676 `crate::` rewrites (section 15).

## 1. Baseline, budgets, what binds

### 1.1 Closure measurements (gate counter, host cfg + `debug_assertions`)

| Closure | cfg | files | bytes | active lines | embedded |
|---|---|---:|---:|---:|---:|
| lib | native (aarch64-apple-darwin) | 185 | 6,007,520 | 83,182 | 0 |
| lib | wasm32-wasip2 | 189 | 6,075,698 | 84,493 | 0 |
| lib | wasm32-unknown-unknown | 185 | 6,007,520 | 83,256 | 0 |
| lib | native + `canvas-runtime-assets` | 185 | 6,007,520 | 83,194 | 0 |
| lib | native + 4 testing features | 211 | 6,512,773 | 88,660 | 10 |
| lib + tests | native, `--cfg test` | 729 | 12,793,280 | 145,762 | 270 |
| lib + tests | wasm32-wasip2, `--cfg test` | 733 | 12,861,458 | 147,043 | 270 |
| lib + tests | native, `--cfg test`, all features | 731 | 12,806,193 | 145,841 | 271 |

The peer note `rust-remaining-giants-current-refresh.md` reports 187 files / 6,064,270 bytes for the same crate (a path-literal parser that skips cfg); the two views agree to 1 percent. Plugin's two biggest implementations in that note are the reactor runtime (391,525 bytes) and `time-travel` (209,942 bytes).

**[WP0 change] Re-measure (files / bytes / active lines), `pwp0-tools/closure.ts`, 2026-10-15; "start" is before the first WP0 edit, "end" after the hubs and the facade macro mounts.**

| closure | plan | start | end |
|---|---|---|---|
| lib native | 185 / 6,007,520 / 83,182 | 185 / 6,010,310 / 83,203 | 195 / 6,018,889 / 83,307 |
| lib wasm32-wasip2 | 189 / 6,075,698 / 84,493 | 189 / 6,078,488 / 84,514 | 199 / 6,087,067 / 84,618 |
| lib wasm32-unknown-unknown | 185 / 6,007,520 / 83,256 | 185 / 6,010,310 / 83,277 | 195 / 6,018,889 / 83,381 |
| lib native canvas | 185 / 6,007,520 / 83,194 | 185 / 6,010,310 / 83,215 | 195 / 6,018,889 / 83,319 |
| lib native testing-features | 211 / 6,512,773 / 88,660 / 10 embedded | 211 / 6,515,563 / 88,681 / 10 embedded | 221 / 6,524,142 / 88,785 / 10 embedded |
| test native | 729 / 12,793,280 / 145,762 / 270 embedded | 729 / 12,796,350 / 145,783 / 270 embedded | 739 / 12,804,099 / 145,885 / 270 embedded |
| test wasm32-wasip2 | 733 / 12,861,458 / 147,043 / 270 embedded | 733 / 12,864,528 / 147,064 / 270 embedded | 743 / 12,872,277 / 147,166 / 270 embedded |
| test native all-features | 731 / 12,806,193 / 145,841 / 271 embedded | 731 / 12,809,263 / 145,862 / 271 embedded | 741 / 12,817,012 / 145,964 / 271 embedded |

Drift start against plan: +21 active lines and +2,790 bytes in the lib closure, +21 lines and +3,070 bytes in the test closure, same file counts. By file (`appendix_a_drift.py`): the root `🦀️.rs` 940 to 960 lines (+3,312 bytes: the `InstanceExtensions` alias and the `semio-framework-ui-layout` / `-ui-declaration` / `-ui-scene` re-exports that replaced `pub use ui_wgpu::wgpu::*`), `🪟️window-kits/🌍️world3d` +1 line, `🏗️builder/🧩️app` +151 bytes. Appendix A omitted three files that the closure already contained: `🧪️tests/🧬️generated-test-contracts/🦀️.rs` (100 lines, an unconditional root mount that exports four macros), `🧵️retained-command/♻️metadata/🦀️.rs` (8) and `🪟️window/🎚️config/🧬️preparation/✏️apply/🦀️.rs` (3); two rows (`🪆️child/👁️read/🧪️test-view`, `⚖️authoring-law`) belong to the test and feature closures only. The item graph is unchanged: `plan_tools.py census` prints the same crate sizes as 6.1 for all 41 crates and 196 moves of 3,351 lines (plan 3,352), `home` the same 227 fields / 674 methods / 14,971 lines and the same homing totals.


### 1.2 Per-crate budget

At most **3,500 active lines, 28 files, 230,000 bytes** for the lib closure **and** for the test closure (`cargo test` units are gated too). Hard ceiling 4,000 / 32 / 262,144. A crate's test closure is its lib closure plus the test files it mounts plus the fixtures they embed. Tests that do not fit move to a companion crate (`x-tests`, dev-only, lib root with `#[cfg(test)]` mounts only). Embedded fixtures and source-text embeds must be read at run time (section 11). The manifest of every new crate declares exactly the recipe's ceiling triple (`maxSourceBytes = 262144`, `maxSourceFiles = 32`, `maxActiveLines = 4000`); the 3,500 / 28 / 230,000 internal target is enforced by the census. Path lengths: at most 256 characters and, for new paths, 240 UTF-8 bytes (a new package manifest path is 140 to 160 bytes; the longest existing path in the test closure is 270 bytes / 187 characters and predates this plan).

### 1.3 What is in flight (do not collide)

* Peer-owned crates (touch only for consumer-side `Cargo.toml` / import edits): `semio-framework-os-kernel` (3.9 MB; every carved crate depends on it, 27 of 41 lower crates import it directly), `plugin-host`, `replication`, `ui-contract`, `ui` aggregates, `job`, `2d`, `pack`, `os-renderer-wgpu` (its kernel pool links the facade and calls `plugin_runtime`, `package_asset`).
* The relocation slice finished `InstanceExtension`, the `ui-layout` re-points and the `os_vcs` re-points shortly before this plan; its residual list (`RESULT-RELOCATE-PLUGIN-SDK.md` section 6) is part of the baseline here: the three `include!`s of the instance parent remain; the domain names remain in instance state, the ladders and four `PluginApp` contract methods.
* `PKG/Cargo.toml` and `PLUG/🦀️.rs` carry peer edits (`InstanceExtensions` alias, `semio-framework-ui-layout`); `🗑️generated` files are untouched. WP0 re-runs the census first and diffs against section 1.

* **[WP0 change] State at WP0 start.** The tree had moved since planning: the relocation session had replaced the `ui_wgpu` dependency by `semio-framework-ui-layout` and `semio-framework-ui-declaration` and added the `InstanceExtensions` alias and the `🧵️instance/🧩️extensions` trait file; `PKG/Cargo.toml` and the root were staged by a peer. The trace-testkit change (`RESULT-W4-TRACE-TESTKIT.md`) adds a dev-dependency to 34 crates but not to this one; the root test heap witness still names `semio_framework_trace::HeapWitness`, which still exists. Peers keep editing plugin files (`⚛️reactor`, `🧪️testing`, the instance band files): every package re-reads a file before each edit (12.3), and the root is WP0-owned.


## 2. Inventory

### 2.1 Layout and mounts

* `PKG/Cargo.toml`: lib path `🦀️.rs`, which mounts `PLUG/🦀️.rs` as `pub mod component` and re-exports it (`pub use component::*`); features `artifact-app-testing`, `media-turn-testing`, `reserved-producer-testing`, `original-actor-context-testing`, `canvas-runtime-assets` (optional `semio-framework-canvas-assets`); three `[[test]]` targets with `required-features`; 44 nominal dependencies. Four `extern crate semio_framework_os_kernel as dsl / protocol / store / vcs;` at the crate root put four names into the extern prelude for every module (`store::` is used by 36 of the crate groups below, `protocol::` by 31, `vcs::` by 6, `dsl::` by 2): every carved crate re-declares the aliases it uses.
* `PLUG/🦀️.rs` (940 lines) mounts 96 modules in six facets (`WindowFacet`, `DeclarationFacet`, `PublicationFacet`, `JobsFacet`, `ReactorFacet`, `InstanceFacet`), defines the macros `with_authoring_identity`, `component_persistent_local`, `register_plugin`, `derive_artifact_facets`, `subset`, the wasip2-only `component_persistent_local` module and `owned_abi` (140 lines), the `pub mod app` re-export hub (97 lines, 74 `pub use`), `plugin_app_close_prelude`, and about 300 lines of explicit root `pub use app::{...}`.
* Mount depth: lib root, root, 96 facet modules, their children (the earlier instance split added 6 band files to the 9 existing instance children). Composition by text: three `include!` in the instance parent (`🪟️window/🫧️transient/🔁️document-replacement`, `🎮️command/🎟️lease`, `♻️lifecycle/🏪️stores`; all three are `impl VcsArtifactApp` bodies), one `include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🚪️lifetime/🦀️.rs"))` in the runtime (`PluginInstanceCloseLease`), and five test `include!`s.

### 2.2 Families (native lib closure; test lines are the test-only `.rs` files mounted under that family)

| family | files | KB | lines | what it is |
|---|---:|---:|---:|---|
| `⚛️reactor` | 28 | 1,198 | 15,668 | guest turn loop, pending patches, lifetime registry, jobs, runtime (5,465-line function bag), executor, context, exports |
| `🧵️instance` | 12 | 935 | 11,772 | `VcsArtifactApp` struct + 7 impl bands + `PluginApp` impl + extension trait |
| `📜️declaration` | 16 | 724 | 11,510 | app contract, tool-job contract, emission, views, surfaces, registration, tree |
| `🧩️composition` | 19 | 365 | 3,876 | child emit preparation, publication groups, mounted receipts, metadata |
| `⏪️time-travel` | 4 | 351 | 5,083 | history editing: ledger, store state, panel, instance turns |
| `🪟️window` | 11 | 282 | 3,926 | window config (2.7k), transient owners (0.8k), tree assembly |
| `🧵️retained-command` | 9 | 216 | 2,280 | typed command full operations and retained command jobs |
| `🪟️window-kits` | 9 | 210 | 3,725 | window kit contract, tree/table/world3d/document/text/image/media/mesh |
| `⏯️tool-run` (+ `🛠️tool-machine`) | 7 | 214 | 3,298 | tool-run ledger/driver, continuous controls, instance turns |
| `🏗️builder` | 4 | 158 | 2,714 | `PluginBuilder`, `AppBuilder`, program, gumball audience |
| `👷️job` | 5 | 150 | 2,094 | tool jobs, reserved jobs, outcomes |
| `🗃️document` | 1 | 136 | 1,861 | envelope ingress, store replacement, archive loads |
| `🪜️` ladders (close, maintenance, envelope) | 3 | 163 | 2,214 | `impl VcsArtifactApp` retirement ladders |
| `🎮️command` | 5 | 110 | 1,218 | command log, history views, admission |
| `🕹️interaction` | 11 | 90 | 1,780 | local interaction ownership, query, topology |
| `💡️inference` + `🧩️extension` + `🛂️describe` | 5 | 159 | 2,182 | inference gateway, extension bundle, describe |
| `🪆️child`, `👥️peer`, `🌐host`, `♻️cancellation`, `🎞️media`, `🚪️io`, `🪪️identity`, `📬️completion`, `🖱️context-menu`, `🧬️schema`, `📡️backbone`, ... | 36 | 600 | 9,900 | the rest |
| root `🦀️.rs` | 1 | 57 | 940 | facade candidate |

Test closure: 274 test-only files, 60,546 lines / 4.98 MB: 84 files (17,356 lines, 1.22 MB) mounted by the root, 24 files (17,059 lines, 1.34 MB) mounted by the runtime file (among them `🧪️tests/🔬️plugin-runtime-plugin-builder-contract`, one file of 10,660 lines / 869 KB), then 3,460 lines under `👷️job`, 2,927 under `🧵️retained-command`, 2,355 under `⚛️reactor/🦀️.rs`, ... .

### 2.3 Cross-cutting facts that shape the plan

1. `use super::*` appears in 54 files (73 sites); in 22 of them the parent ends up in another crate, so the glob must become an explicit import list (section 6.3). `crate::` appears 2,676 times in 114 files.
2. 18 `#[macro_export]` macros (table in 10.2) with `$crate::` paths into the old root; 13 `component_persistent_local!` invocations in the reactor; about 90 statics and thread-locals (section 7.8).
3. 227 fields of one struct; 26 inherent impl blocks of it spread over 21 files; 3,540 items of which 174 (1,746 lines) are `cfg(test)`.
4. The only embedded data in the lib closure are 10 JSON fixtures of the testing features; there are no assets to move.
5. 247 of the 571 units that cross a new crate boundary are `pub(crate)`, `pub(super)` or private today and need `pub` (`#[doc(hidden)]` where the facade must not re-export them). Methods and fields are not counted here (they are invisible to the token graph); the definition scan of section 13 finds them.
6. 532 path literals in 104 TS and Rust readers point at 283 distinct `🦀️.rs` targets (most read: root 56, `runtime` 12, `turn` 12, `time-travel` 10, `reactor context call` 9, `tool-run` 8, the mounted receipt files 7+6+6+5).

## 3. Method (what the numbers mean)

1. **Closure**: `readRustSourceClosureV1` on `PKG/🦀️.rs`, per cfg (script in Appendix F). Per-file lines and bytes below come from it.
2. **Module tree**: mount parser over `#[path]`, plain `mod`, `include!`, `include!(concat!(env!("CARGO_MANIFEST_DIR"), ...))` and `include_str!`; 464 nodes (lib and test mounts) resolved, 0 missing; its `.rs` set equals the closure's.
3. **Items**: column-0 outline over comment/string-stripped text (3,540 items; 79,350 of the 86,717 file lines of the 189 files are inside items; the rest are `use` blocks, attributes, regions). An inherent impl is bound to its type (443 impls bound; trait impls stay separate nodes because the orphan rule lets them live beside either the trait or the type).
4. **Edges**: an item refers to an item when it mentions a name defined by 1 to 3 items. Filters that removed false edges (each one found by reading a wrong edge in an earlier run): tokens preceded by `Capitalized::` (enum variants), preceded by `.` (method or field access), followed by a single `:` (field, binding, parameter), lowercase tokens not followed by `(`, `::`, `!` or `<`, lowercase single words of up to 13 letters as definitions (`bytes`, `fault`, `kind`, `progress`), the defs `_` and `tests`, and about two dozen generic names (`GENERIC` in Appendix G: `Cell`, `Inner`, `Node`, `Frame`, `Cursor`, `Source`, `Slot`, `Lane`, `Phase`, `Reserve`, `Admission`, `Vacant`, `Step`, `Ready`, ...). `mod` items are not definitions. `cfg(test)` items are not part of the production graph.
5. **Partition**: crates are file prefixes (Appendix G `MAP`) plus cuts of three big files by line region plus item moves. A crate-level edge is an item reference across crates; a **back edge** references a higher-ranked crate. Fix = the cheaper of "down" (move the referenced unit and everything it drags into the referencing crate) or "up" (move the referencing unit and all its dependents above), with cost = lines, doubled when the move crosses a subsystem (foundation, window, declaration, owners, reactor, features). Knots (unit SCCs) are contracted first and live in the crate that holds most of their lines.
6. **Rank order** (section 4.3): found by insertion local search over the proxy cost (sum over crate pairs of `min(lines, 400)` of every back reference) with the subsystem order F < W < D < O < R < T held fixed, then re-checked by the exact move simulation. 
7. **Instance decomposition**: field → component by name pattern (19 components, 227 fields), method → component by the fields it reads (`self.f`), lifted to the highest component it calls (fixed point), 674 methods (algorithm in 7.3).
8. **Limits of this evidence** (carry into verification): method-call edges and trait dispatch are invisible; references through macros, `Deref` and trait scoping are invisible; the generic names and all single-word lowercase function names are ignored, so a few real edges may be missing; sizes are item lines, add 6 to 9 percent for `use` blocks and region markers; every size is ±10 percent. Re-measure per crate with the census before and after each carve.

## 4. Layers and the crate DAG

### 4.1 Rules the partition obeys

1. **Lines and bytes decide the cut, cohesion decides the shape.** A crate is a taxonomy node or a union of sibling nodes, never a size bucket. The three big files are the only ones cut inside a file, along their own `//#region` / function-family boundaries.
2. **Dependencies point down only.** The rank order of 4.3 is the topological order; a reference to a higher-ranked crate is a defect and appears in section 5.
3. **A type and every inherent impl of it share a crate.** 4,419 lines of inherent impl sit today in a different crate region than their type under the crate map of Appendix G (the ladders and the two feature-turn files of `VcsArtifactApp`, section 7, plus a few small impls moved by 5.2).
4. **Statics live in the lowest crate that names them** and are reached by `pub` accessor functions from above (7.8).
5. **Trait impls** (for example `impl PluginApp for VcsArtifactApp`, `impl WindowKit for ...`) live beside the type or the trait, whichever is higher.
6. **No extension traits to split an impl**; the one existing trait seam (`InstanceExtension`) is an inversion point with exactly one implementor list.
7. **No compatibility layer**: moved items keep their names; the facade re-exports them explicitly; `Plugin::builder` is removed, not aliased.
8. **Native vs guest**: no crate is native-only. wasip2-only content is gated by `cfg` inside the owning file (as today); one crate (`reactor-wit-bridge`) is gated as a whole by the facade's target-specific dependency.

### 4.2 Layer sketch (low to high; generated by `plan_tools.py`)

```
L0  abi                      -> 
L0  actor-host               -> 
L0  identity                 -> 
L0  interaction              -> 
L0  window-config            -> 
L1  operation                -> interaction
L1  window-transient         -> interaction, window-config
L2  window-kits              -> interaction, operation, window-transient
L2  decl-views               -> interaction, operation, window-config
L3  window-kits-rich         -> interaction, window-transient, window-kits
L3  decl-emission            -> actor-host, operation, window-config, decl-views
L3  child                    -> decl-views
L4  decl-app                 -> interaction, operation, window-config, window-transient, window-kits, decl-views, decl-emission
L4  composition-emission     -> decl-views, decl-emission, child
L5  app-builder              -> window-transient, decl-views, decl-app
L5  composition-publication  -> window-config, window-transient, decl-views, decl-emission, decl-app, child, composition-emission
L5  document-media           -> operation, decl-views, decl-app, child, composition-emission
L5  job                      -> interaction, operation, window-config, window-transient, window-kits, decl-views, decl-emission, decl-app
L5  peer                     -> decl-views, decl-app
L5  tool-run                 -> interaction, window-config, window-transient, window-kits, decl-views, decl-emission, decl-app
L6  decl-surface             -> abi, interaction, window-config, window-transient, window-kits, decl-views, decl-emission, decl-app, app-builder
L6  command                  -> actor-host, interaction, operation, window-transient, window-kits, window-kits-rich, decl-views, decl-emission, decl-app, job
L6  publication-mounted      -> decl-views, child, composition-publication
L6  retained-command         -> operation, window-config, window-transient, decl-views, decl-emission, decl-app, composition-publication, job
L6  reactor-app              -> operation, window-config, decl-views, decl-emission, decl-app, composition-publication, document-media, job, peer
L7  decl-registry            -> identity, window-kits, decl-app, app-builder, decl-surface, reactor-app
L7  time-travel-panel        -> interaction, window-transient, window-kits, window-kits-rich, decl-views, command
L8  plugin-program           -> identity, decl-views, decl-emission, app-builder, decl-surface, job, reactor-app, decl-registry
L8  time-travel              -> decl-views, decl-app, child, command, time-travel-panel, tool-run
L9  reactor-lifecycle        -> actor-host, interaction, job, reactor-app, plugin-program
L10 runtime-instances        -> job, reactor-app, decl-registry, plugin-program, reactor-lifecycle
L10 reactor-patches          -> reactor-lifecycle
L11 runtime-ops              -> abi, actor-host, operation, window-transient, decl-views, decl-emission, decl-app, decl-surface, document-media, job, peer, reactor-app, plugin-program, reactor-lifecycle, runtime-instances
L11 extension                -> abi, identity, decl-surface, reactor-app, decl-registry, plugin-program, reactor-lifecycle, runtime-instances
L12 reactor-surfaces         -> reactor-app, reactor-lifecycle, runtime-instances, runtime-ops
L12 reactor-jobs             -> abi, actor-host, job, decl-registry, plugin-program, extension
L12 runtime-exchange         -> window-config, decl-views, decl-emission, decl-app, job, reactor-app, plugin-program, reactor-lifecycle, runtime-instances, runtime-ops
L13 reactor-core             -> actor-host, decl-emission, reactor-app, reactor-lifecycle, runtime-instances, reactor-surfaces
L14 reactor-turn             -> abi, actor-host, operation, window-kits-rich, job, command, peer, reactor-app, reactor-lifecycle, runtime-instances, runtime-ops, extension, reactor-patches, reactor-surfaces, reactor-core, runtime-exchange
L15 reactor-wit-bridge       -> reactor-app, reactor-lifecycle, reactor-turn
L15 reactor-context          -> abi, reactor-app, reactor-lifecycle, runtime-ops, reactor-jobs, reactor-turn
```

Instance family and above (not part of the item-move graph; their lower dependencies are the crates above):

```
T1  instance-stores -> instance-history -> instance-children -> instance-jobs -> instance-docs      (component crates, rank order of 7.3)
T1x instance-extension                                                                              (InstanceExtension trait + bundle types, after the components)
T2  instance-commands  instance-typed-ops  instance-render  instance-ladders                        (use-case crates over bundles)
    time-travel-turns  tool-run-turns                                                               (feature turns; implement InstanceExtension)
T3  instance-app (composer VcsArtifactApp + PluginApp impl + constructors + InstanceExtensions alias)
T4  builder -> facade (semio-framework-plugin)  ->  testing-laws, testing-acceptance (feature-gated)
    companion test crates (dev-only) above their hosts
```

The instance crates depend on `reactor-core` only for `spawn_task` / `queue_extension_invocation` (I6, Option A); with Option B they depend on no reactor crate except `reactor-app`.

### 4.3 Rank order (the order used for back-edge accounting)

`abi, actor-host, identity, interaction, operation, window-config, window-transient, window-kits, window-kits-rich, decl-views, decl-emission, decl-app, app-builder, decl-surface, child, composition-emission, composition-publication, document-media, job, command, peer, publication-mounted, retained-command, reactor-app, decl-registry, plugin-program, reactor-lifecycle, runtime-instances, runtime-ops, extension, reactor-patches, reactor-surfaces, reactor-core, reactor-jobs, runtime-exchange, reactor-turn, reactor-wit-bridge, reactor-context, time-travel-panel, tool-run, time-travel`, then the instance family of section 7.5, `builder`, the facade.

The shape in words. **L0..L2**: wire/ABI (`abi`), host API and executors (`actor-host`), identity, interaction state, window config and window transient owners, cancellation/operation vocabulary. **L2..L6**: the declaration contracts (`decl-views` < `decl-emission` < `decl-app` < `decl-surface`), the app builder DSL (`app-builder`: `AppBuilder`, `App`, `ExampleSource`, which the surface declarations name), the owners (child, composition, document/media, job, command, peer, retained-command). **L6..L10**: `reactor-app` (the `PluginApp` trait), `decl-registry`, `plugin-program` (`Plugin`, `AppInstance`, `SurfaceContexts`), `reactor-lifecycle` (the knot). **L10..L15**: runtime function families, patches, surfaces, core registries, jobs, turn, context. **features**: `tool-run` and `time-travel` crates sit beside the owners (`time-travel` depends on `command`, `tool-run`, `child`, the declaration crates) and below the instance.

## 5. Cycle cuts

### 5.1 What the cycles are

* File level: one SCC of 82 files / about 55,000 lines (`🧵️instance` and its ladders, `⏪️time-travel`, `⏯️tool-run`, `📜️declaration/*`, `⚛️reactor/*`, `🗃️document`, `🪆️child`, `🪟️window*`, `🧩️composition/*`, `🧵️retained-command`, `👷️job`, `🏗️builder`, `🎮️command`, `🌐host`, `👥️peer`, `💡️inference`, `🧩️extension`). A few further 2-file cycles.
* Item level: 2,919 production units. SCCs larger than one unit: the pair {`VcsArtifactApp` with its 13,795 lines of inherent impls, `DocumentWindowResetAdmission` (5 lines whose impl names the app)}, the declaration knot (26 units, 1,384 lines), the lifecycle knot (29 units, 1,131 lines), and 19 small ones (2 to 6 units, 14 to 670 lines: `ArtifactEditor` + `NaturalFileImportJob`, `ChildContentView` + owners, `TimeTravelStoreState`, `Emit` + `AsyncTask` + `cancel` helpers, `RuntimeAppCell` cluster, `ChildContentRetirement`, `MountedMemberReceipt`, `CompletedInput`/`FaultCursor`, `PluginCommandIngress`, `PagedCommandLog`, ...).
* **The declaration knot** is `ArtifactApp` (729 lines, with its associated types) together with the tool-job contract items, because the trait's methods name the job requests and the job requests are generic over the trait. It stays whole in `decl-app` (2,414 lines after moves); the members that live in other directories today (`ToolRunJobRequest` 36 and `ArtifactMediaExportJobRequest` 15 from the views, `EphemeralEmit` 40 from emission, `CompletionCell` 19 and the retirement cursors from completion) move into `decl-app`.
* **The lifecycle knot**: `NativeCloseKey` names `PluginInstanceCloseLease`, which closes through `plugin_begin_instance_close` and `PluginRuntime`, which owns the `NativeLifecycleRegistry` of `NativeLifetimeOwner<PA>`s, which hold leases; the patch tracker slots and `PendingPatchAuthority` are keyed by `NativeCloseKey`. It stays whole in `reactor-lifecycle` (2,446 lines with the registries the runtime file and `reactor/🦀️.rs` hand down). Optional cut, not planned: make `NativeLifecycleRegistry` generic over the owner `O: GuestLifetimeOwner` (the cell already is) so the registry no longer names the lease.

### 5.2 Item moves (apply with the staging protocol of 12.2; counts are active lines and include the 692 lines of knot members that sit in another directory than their knot; `plan_tools.py moves` prints the optimizer's part of the list, Appendix G)

| lines | from | to | units | largest items |
|---:|---|---|---:|---|
| 570 | `time-travel-panel` | `command` | 20 | `HistoryPanelText` 167, `time_travel_band_section` 98, `time_travel_reprojection_section` 47, `history_mutation_description` 38, `history_mutation_entry` 37, `history_code_text` 26, `TimeTravelPanel` 21, +13 more |
| 565 | `runtime-instances` | `reactor-lifecycle` | 11 | `HostInstanceGuard` 231, `RuntimeCloseWorkerState` 162, `PluginInstanceCloseLease` 78, `plugin_begin_instance_close` 65, `runtime_close_pending_authority` 10, `debug_runtime_line` 5, `PendingNativeCreation` 5, +4 more |
| 369 | `extension` | `plugin-program` | 31 | `wire_artifact_infer_with_service_cold` 66, `validate_wire_request_resources` 44, `WireArtifactInferenceResult` 31, `WireArtifactInferenceRequest` 24, `validate_inference_execution` 22, `validate_wire_request_metadata` 21, `ArtifactInferenceExecutionError` 19, +24 more |
| 304 | `composition-emission` | `decl-emission` | 10 | `TypedChildEmitPreparation` 147, `ChildEmitPreparation` 72, `OwnedChildEmit` 50, `ChildEmitPreparationOwner` 19, `ChildEmitPreparationStep` 9, `retirement_refusal_fault` 3, `owned_message_bytes` 1, +3 more |
| 235 | `decl-app` | `decl-views` | 9 | `ArtifactInstanceOperationOwnerHandle` 158, `InstanceInferenceSessions` 32, `ArtifactInferenceSessionAccess` 19, `ArtifactInstanceOperationOwner` 9, `OriginalInferenceSessionOwner` 7, `original_inference_metadata` 5, `original_inference_close_step` 2, +2 more |
| 218 | `command` | `time-travel-panel` | 1 | `ui_history_panel` 218 |
| 199 | `reactor-core` | `reactor-lifecycle` | 17 | `ReactorFixedSlots` 73, `reserve_reactor_close` 28, `ReactorCloseRegistry` 21, `ReactorCloseState` 19, `preflight_reactor_close` 12, `release_reactor_close` 11, `activate_reactor_close` 8, +10 more |
| 180 | `reactor-surfaces` | `plugin-program` | 6 | `SurfaceContexts` 132, `SurfaceRole` 27, `SurfaceBinding` 8, `synthetic_window_surface` 7, `SurfaceContext` 4, `SYNTHETIC_WINDOW_SURFACE` 2 |
| 178 | `extension` | `decl-registry` | 18 | `ArtifactInferenceServiceRegistry` 35, `ArtifactInferenceService` 20, `ArtifactInferenceServiceMetadata` 17, `ArtifactInferenceExecutionRequest` 14, `ArtifactInferenceExecution` 14, `ArtifactInferencePayloadContract` 12, `WireArtifactInferenceDiagnostic` 8, +11 more |
| 141 | `reactor-lifecycle` | `runtime-instances` | 4 | `RuntimeAppCell` 98, `RuntimeCloseWorkerState` 27, `HostInstanceGuard` 10, `BackgroundInstanceGuard` 6 |
| 137 | `reactor-patches` | `reactor-lifecycle` | 11 | `MountedTreeTerminal` 34, `PatchTrackerState` 28, `ReadySlot` 20, `SurfaceSlot` 13, `has_work` 10, `MountedTreeProducer` 7, `ClosingInstance` 7, +4 more |
| 135 | `window-kits` | `window-kits-rich` | 2 | `DocumentWindowKit` 115, `impl WindowKit for DocumentWindowKit {` 20 |
| 112 | `operation` | `decl-app` | 19 | `CompletionCell` 19, `impl<A:ArtifactApp> RetirementCursor for` 18, `ArtifactCompletionFault` 11, `derived_admit_completion_retirement` 10, `cancellation_result_lane` 7, `impl<A:ArtifactApp> From<ArtifactToolCom` 6, `borrowed_report` 6, +12 more |
| 106 | `decl-registry` | `extension` | 2 | `preflight_artifact_declarations` 57, `commit_artifact_declarations` 49 |
| 72 | `runtime-instances` | `plugin-program` | 6 | `wire_artifact_mutation_plan` 25, `key_ordered_wire_value` 16, `PluginDescriptorExtras` 11, `wire_list_artifact_mutations` 9, `encode_wire_serialized` 6, `decode_wire_serialized` 5 |
| 71 | `decl-views` | `decl-app` | 4 | `ToolRunJobRequest` 36, `ArtifactMediaExportRequestRetirement` 19, `ArtifactMediaExportJobRequest` 15, `impl<A:ArtifactApp> Drop for ArtifactMed` 1 |
| 51 | `decl-emission` | `decl-app` | 3 | `EphemeralEmit` 40, `CompletionPayload` 6, `impl<A: ArtifactApp> Default for Ephemer` 5 |
| 45 | `decl-app` | `document-media` | 3 | `MediaArtifact` 27, `whole_document_media` 10, `impl semio_framework_job::OriginalDomain` 8 |
| 42 | `operation` | `decl-emission` | 8 | `dispose_cold_task_run` 9, `AsyncTaskRun` 8, `impl<T:AsyncTaskSource<M,C,D>,M: ,C: ,D:` 7, `ColdTaskRun` 6, `impl<M:Send+ ,C:Send+ ,D:Send+ > semio_f` 5, `AsyncTaskSource` 5, `impl<M,C,D> Drop for ColdTaskRun<M,C,D>{` 1, +1 more |
| 36 | `reactor-core` | `reactor-turn` | 1 | `step_reactor_close` 36 |
| 28 | `plugin-program` | `decl-registry` | 2 | `ArtifactInferenceExecutionError` 19, `AppFactory` 9 |
| 25 | `window-kits-rich` | `window-kits` | 4 | `WorldSunConfig` 11, `tree_item` 6, `world3d_scene` 5, `world3d_environment_json` 3 |
| 24 | `decl-surface` | `decl-emission` | 1 | `ViewEmit` 24 |
| 22 | `runtime-instances` | `extension` | 2 | `plugin_wire_list_artifact_inference_serv` 11, `plugin_wire_artifact_infer_cold` 11 |
| 22 | `retained-command` | `decl-app` | 6 | `ArtifactOwnedContextHandle` 13, `ContextOwnedFields` 4, `ContextToolRunMetadata` 2, `ContextToolRunSources` 1, `ContextRetirementParts` 1, `ContextOriginalResidual` 1 |
| 20 | `job` | `retained-command` | 1 | `FRAMEWORK_RESERVED_ROUTE_CAPACITY` 20 |
| 19 | `window-transient` | `window-config` | 5 | `ErasedWindowMutationValue` 7, `refresh_transfer` 5, `refresh_demands` 3, `WindowAuthorityRefreshStep` 3, `WindowRefreshSnapshot` 1 |
| 17 | `time-travel-panel` | `time-travel` | 1 | `time_travel_selection_value` 17 |
| 16 | `retained-command` | `job` | 1 | `TypedCommandFullOperationJobFactory` 16 |
| 12 | `reactor-core` | `runtime-instances` | 1 | `take_original_instance_metadata_app_id` 12 |
| 9 | `reactor-jobs` | `actor-host` | 1 | `cancel_job` 9 |
| 9 | `decl-views` | `operation` | 2 | `ArtifactOperationProgress` 8, `CANCEL_TYPED_OPERATION_ACTION_ID` 1 |
| 6 | `operation` | `decl-views` | 1 | `CompletionPayload` 6 |
| 6 | `decl-views` | `decl-emission` | 1 | `CompletionPayload` 6 |
| 5 | `window-transient` | `operation` | 2 | `ui_assembly_error` 3, `UiAssemblyResult` 2 |
| 5 | `operation` | `interaction` | 2 | `ui_assembly_error` 3, `UiAssemblyResult` 2 |
| 4 | `tool-run` | `runtime-ops` | 1 | `is_tool_run_action_id` 4 |
| 4 | `time-travel` | `time-travel-panel` | 2 | `TIME_TRAVEL_SHORT_ID_CHARS` 2, `TIME_TRAVEL_ENTITY_NAME_VISITS` 2 |
| 4 | `runtime-ops` | `job` | 1 | `is_tool_run_action_id` 4 |
| 4 | `command` | `tool-run` | 1 | `time_travel_arg_generation` 4 |
| 3 | `reactor-turn` | `reactor-core` | 3 | `response_key_empty` 1, `original_receiving_sources_empty` 1, `intent_key_empty` 1 |
| 3 | `reactor-lifecycle` | `plugin-program` | 1 | `plugin_internal_fault` 3 |
| 3 | `document-media` | `decl-app` | 2 | `key_media_error` 2, `media_projection_error` 1 |
| 2 | `time-travel` | `command` | 1 | `HISTORY_ROW_MUTATION_ROWS` 2 |
| 2 | `runtime-ops` | `runtime-instances` | 1 | `LAST_MAINTENANCE_STAGE` 2 |
| 2 | `reactor-turn` | `command` | 1 | `Inputs` 2 |
| 2 | `operation` | `window-transient` | 1 | `EphemeralParts` 2 |

Notes on the moves that are not mechanical:

1. **History helpers (`time-travel-panel → command` 570, `command → time-travel-panel` 218).** The `Driver` region of the time-travel file (`MutationPages`, `MemberEditHistory`, `HistoryViewStamps`, `history_mutation_*`, 500 lines) and the history panel text (`HistoryPanelText`, band sections, 570 lines in total) are the history *view* vocabulary that `command` and the instance dispatch need; they land in `command` (`RESULT-RELOCATE-PLUGIN-SDK.md` section 6 named this as a follow-up). `ui_history_panel` (218 lines) in `command` is the time-travel panel itself and goes up to `time-travel-panel`.
2. **Inference** (`extension → plugin-program` 369, `extension → decl-registry` 178): the wire request/result types and the service registry are used by the registration plan and the program; the gateway and the cold entry points stay in `extension`.
3. **`runtime-instances → reactor-lifecycle` 422 / `reactor-lifecycle → runtime-instances` 141**: the knot's registry types (`HostInstanceGuard`, `RuntimeCloseWorkerState`, `PluginInstanceCloseLease`, `plugin_begin_instance_close`) move into the knot crate; `RuntimeAppCell` (98) and the cleanup pump jobs stay with the instance registry. If WP4 prefers, merge `runtime-instances` into `reactor-lifecycle` (3,225 lines / 185 KB, still inside the limit); the move list then shrinks by 563 lines.
4. **`reactor-core → reactor-lifecycle` 199** (close registry, fixed slots), **`reactor-core → reactor-turn` 36** (`step_reactor_close`), **`reactor-core → reactor-wit-bridge` 594** (cut of the inline `wit_bridge`, listed separately in 5.3).
5. **`composition-emission → decl-emission` 304**: `ChildEmitPreparation` and its owners are what `Emit`/`ChildEmit` name; this also moves `🧩️composition/📨️emission/📦️preparation` into the `decl-emission` crate.
6. **`decl-app → decl-views` 235**: `ArtifactInstanceOperationOwnerHandle` (158) and the inference session owners are named by `ArtifactView` (the view layer is below the app contract).

### 5.3 Inversions and decoupling moves (the residual cross-layer edges that are not item moves)

| id | what the lower code does today | replacement | sites |
|---|---|---|---:|
| I1 | the instance dispatch head, emit freezes, maintenance pump and render call `InstanceExtensions::{claims_action, dispatch_action, reserved_action_refusal, mutation_refusal, document_refusal, freezes_document, transaction_refusal, window_config_published, has_pending_work, holds_receipt, terminal_is_empty, drive_turn, render_body, after_app_render}` on the concrete alias `(ToolRunExtension, (TimeTravelExtension, ()))` | the use-case functions take a type parameter `X: InstanceExtension`; the alias is defined once in `instance-app`; `time-travel-turns` and `tool-run-turns` implement the trait over the same bundles. Static dispatch, no `dyn Future` (the relocation slice's reason stays valid) | 22 call sites in 9 files |
| I2 | `impl Plugin<PA> { pub fn builder(..) -> PluginBuilder<NeedsLabel, PA> }` sits in the builder file, but `Plugin` is a runtime type; the pair `PluginBuilder` / `Plugin` is a 976-line SCC | delete `Plugin::builder`; the 13 `PluginBuilder::new(` users are the live API; the two callers are `super::Plugin::builder(plugin_id)` in `🧪️testing/🦀️.rs` (lines 2700, 2729); a ticket fixture and 6 doc comments mention it | 2 |
| I3 | `editor_surface`, `viewer_surface`, `EditorSurfaceApp`, `ViewerSurfaceApp` (63 lines) in the declaration tree name `VcsArtifactApp` | move up to `instance-app` and re-export from the facade at the same path (`declarations::editor_surface`) | 4 items |
| I4 | `DocumentWindowResetAdmission` impl (5 lines) names the app | the impl takes the stores bundle (`instance-stores`) | 1 |
| I5 | `component_persistent_local!` (macro + wasip2 module) is defined in the root and used 13 times by the reactor | define in `abi` (`#[macro_export]`), facade re-exports; its `$crate::component_persistent_local::ComponentPersistentLocal` path is then local to `abi` | 13 uses |
| I6 | the instance family names 11 runtime helpers: `original_plugin_turn_grant` (x4), `remaining_plugin_turn_grant` (x2), `original_render_section`, `admit_addressed_action_view`, `debug_runtime_line` (x5), `LAST_MAINTENANCE_STAGE`, `drive_framework_reserved_registry_job(_counted)`, `PluginRuntimeRegistry`, and the two reactor calls `spawn_task` (typed-operations line 1279) and `queue_extension_invocation` (dispatch line 987) | move the six small helpers (2 to 12 lines each; 2 for the static) into `reactor-app`; `drive_framework_reserved_registry_job` (the 145-line `⚛️reactor/💼️jobs/🏛️reserved` file, whose own docstring calls it "the registry driver the instance runs it through") into `job`; `PluginRuntimeRegistry` already lives in `decl-registry` (L7). `spawn_task` and `queue_extension_invocation` stay in `reactor-core`, so the instance family depends on `reactor-core` (default, Option A); Option B (decision D8) cuts a `reactor-tasks` crate (349 lines: those two functions, the task records and resumes and their statics; an experiment with the tool gives dependencies `decl-emission` and `actor-host` only, but the statics are invisible to the graph) so that no instance crate depends on any reactor crate except `reactor-app` | 13 items |
| I7 | `wit_bridge` (594 lines, `cfg(all(wasm32, p2))`) is an inline module of `reactor/🦀️.rs` | its own crate `reactor-wit-bridge` above `reactor-turn`; the three statics it touches (`INTENTS`, `RESPONSES`, `TURN_EVENTS`) get `pub` accessors in `reactor-turn` | 3 statics |
| I8 | `extension_exports!`, `__semio_actor_exports!`, `__semio_owned_core_exports!`, `__semio_plugin_actor_exports!`, `plugin_exports!` name facade paths (`$crate::{reactor, plugin_runtime, component, io, sqlite_wire, operation_authority, describe, app, ...}`) | the five macros live in the facade (625 lines); the runtime file's `extension` region keeps `ExtensionBundle` | 5 macros |

### 5.4 Residual risk list for section 5

1. The graph is blind to method-call edges: a lower crate may call an inherent method of a unit that the plan places higher. The compiler finds it as `E0599` (no method) and the cure is always the same: move the method with its owner or lower the owner. 
2. Moves of `impl Trait for Type` items are free but change trait scoping (`use` of the trait) at every call site in the destination crate; the definition scan lists trait-method uses of every moved trait.
3. I1 changes monomorphisation of the dispatch head (one instantiation per `X`); there is exactly one `X`.
4. The two knots are kept whole on purpose; if either grows past 3,000 lines (lifecycle is at 2,446 plus 8 percent for `use` blocks), cut it with the optional generic-registry cut of 5.1.

## 6. The crates

### 6.1 Catalogue of the 41 lower crates (lines = production item lines after moves; KB = bytes of those items; WP = work package; deps = direct first-party dependencies from the item graph, in rank order; layer = longest path)

| crate | layer | lines | KB | WP | root directory (package at `<root>/📦️packages/🦀️rust`) | member directories mounted by the root's `#[path]` | deps |
|---|---|---:|---:|---:|---|---|---|
| `abi` | L0 | 932 | 74 | 1 | `🚪️io` | 🧬️schema, † 🧬️schema/🔁️persistent-local (root: component_persistent_local, owned_abi) | - |
| `actor-host` | L0 | 1,968 | 98 | 1 | `🌐host` | ⚛️reactor/🧵️executor, 📡️backbone | - |
| `identity` | L0 | 671 | 30 | 1 | `🪪️identity` |  | - |
| `interaction` | L0 | 1,537 | 79 | 1 | `🕹️interaction` |  | - |
| `window-config` | L0 | 2,600 | 182 | 1 | `🪟️window/🎚️config` |  | - |
| `operation` | L1 | 767 | 80 | 1 | `♻️cancellation` | ⏳️operation-progress, 🌱️initialization, 📬️completion, 🧵️task (cold-task part stays in decl-emission) | `interaction` |
| `window-transient` | L1 | 942 | 61 | 1 | `🪟️window` | 🫧️transient, 🎛️engagement, 📬️mutation | `interaction`, `window-config` |
| `window-kits` | L2 | 1,487 | 78 | 1 | `🪟️window-kits` | 🖱️context-menu; kits 📃️document 🖼️image 🎬️media 🧊️mesh 📝️text | `interaction`, `operation`, `window-transient` |
| `decl-views` | L2 | 2,832 | 182 | 2 | `📜️declaration/👁️view` | 🪆️child, 🎮️command, 🛠️tool-machine, ⏯️tool-run, 🎞️media, 👥️presence/♻️retirement, 📝️draft/🚫️none, 🫧️transient/♻️retirement, 🎚️config/🚫️none | `interaction`, `operation`, `window-config` |
| `window-kits-rich` | L3 | 2,018 | 113 | 1 | `† 🪟️window-kits/🧱️rich` | 🌳️tree, 📊️table, 🌍️world3d | `interaction`, `window-transient`, `window-kits` |
| `decl-emission` | L3 | 1,457 | 101 | 2 | `📜️declaration/📨️emission` | 🧩️composition/📨️emission/📦️preparation (moved in), 🧵️task/🛬️source | `actor-host`, `operation`, `window-config`, `decl-views` |
| `child` | L3 | 981 | 62 | 3 | `🪆️child` |  | `decl-views` |
| `decl-app` | L4 | 2,414 | 168 | 2 | `📜️declaration/🧩️app` | 👷️job (tool-job contract), 📬️completion/♻️retirement | `interaction`, `operation`, `window-config`, `window-transient`, `window-kits`, `decl-views`, `decl-emission` |
| `composition-emission` | L4 | 730 | 67 | 3 | `† 🧩️composition` | 🪪️metadata, ♻️root, 📨️emission/🌱️genesis | `decl-views`, `decl-emission`, `child` |
| `app-builder` | L5 | 1,464 | 82 | 2 | `🏗️builder/🧩️app` | 🖐️gumball-audience | `window-transient`, `decl-views`, `decl-app` |
| `composition-publication` | L5 | 1,323 | 117 | 3 | `🧩️composition/📬️publication` | 🤝️group (non-mounted: 📦️owner, 🪆️child) | `window-config`, `window-transient`, `decl-views`, `decl-emission`, `decl-app`, `child`, `composition-emission` |
| `document-media` | L5 | 2,344 | 179 | 3 | `🗃️document` | 🎞️media | `operation`, `decl-views`, `decl-app`, `child`, `composition-emission` |
| `job` | L5 | 1,694 | 120 | 3 | `👷️job` | 🏛️reserved, 📤️outcomes, 🧾️bytes | `interaction`, `operation`, `window-config`, `window-transient`, `window-kits`, `decl-views`, `decl-emission`, `decl-app` |
| `peer` | L5 | 510 | 32 | 3 | `👥️peer` |  | `decl-views`, `decl-app` |
| `tool-run` | L5 | 1,923 | 120 | 5 | `⏯️tool-run` | 🛠️tool-machine | `interaction`, `window-config`, `window-transient`, `window-kits`, `decl-views`, `decl-emission`, `decl-app` |
| `decl-surface` | L6 | 2,148 | 133 | 2 | `📜️declaration/🪟️surface` | 📥️natural-file | `abi`, `interaction`, `window-config`, `window-transient`, `window-kits`, `decl-views`, `decl-emission`, `decl-app`, `app-builder` |
| `command` | L6 | 1,638 | 111 | 3 | `🎮️command` | 🗝️latest, 📥️admission; history-view helpers from ⏪️time-travel | `actor-host`, `interaction`, `operation`, `window-transient`, `window-kits`, `window-kits-rich`, `decl-views`, `decl-emission`, `decl-app`, `job` |
| `publication-mounted` | L6 | 1,291 | 139 | 3 | `🧩️composition/📬️publication/🤝️group/🪟️mounted` |  | `decl-views`, `child`, `composition-publication` |
| `retained-command` | L6 | 1,912 | 143 | 3 | `🧵️retained-command` |  | `operation`, `window-config`, `window-transient`, `decl-views`, `decl-emission`, `decl-app`, `composition-publication`, `job` |
| `reactor-app` | L6 | 463 | 43 | 4 | `⚛️reactor/🔌️app` |  | `operation`, `window-config`, `decl-views`, `decl-emission`, `decl-app`, `composition-publication`, `document-media`, `job`, `peer` |
| `decl-registry` | L7 | 2,364 | 139 | 2 | `📜️declaration` | 📇️registration, 🌳️tree | `identity`, `window-kits`, `decl-app`, `app-builder`, `decl-surface`, `reactor-app` |
| `time-travel-panel` | L7 | 1,139 | 75 | 5 | `† ⏪️time-travel/🪟️panel` | regions PanelView, Pointer, Panel | `interaction`, `window-transient`, `window-kits`, `window-kits-rich`, `decl-views`, `command` |
| `plugin-program` | L8 | 860 | 49 | 2 | `🏗️builder/🔌️program` | ⚛️reactor/🪟️surfaces (SurfaceContexts), wire helpers from runtime | `identity`, `decl-views`, `decl-emission`, `app-builder`, `decl-surface`, `job`, `reactor-app`, `decl-registry` |
| `time-travel` | L8 | 1,482 | 109 | 5 | `⏪️time-travel` | (regions Limits..StoreState, Supersessions) | `decl-views`, `decl-app`, `child`, `command`, `time-travel-panel`, `tool-run` |
| `reactor-lifecycle` | L9 | 2,446 | 125 | 4 | `⚛️reactor/📨️pending` | ⚛️reactor/🚪️lifetime, 🚪️lifetime (lease), registries moved from ⚛️reactor/🦀️.rs and runtime lines 39-600 | `actor-host`, `interaction`, `job`, `reactor-app`, `plugin-program` |
| `runtime-instances` | L10 | 779 | 60 | 4 | `† ⚛️reactor/🏃️runtime/📦️instances` | runtime file lines 600-1880 | `job`, `reactor-app`, `decl-registry`, `plugin-program`, `reactor-lifecycle` |
| `reactor-patches` | L10 | 1,159 | 68 | 4 | `⚛️reactor/🩹️patches` |  | `reactor-lifecycle` |
| `runtime-ops` | L11 | 2,082 | 172 | 4 | `† ⚛️reactor/🏃️runtime/🧰️ops` | runtime file lines 1881-3837 | `abi`, `actor-host`, `operation`, `window-transient`, `decl-views`, `decl-emission`, `decl-app`, `decl-surface`, `document-media`, `job`, `peer`, `reactor-app`, `plugin-program`, `reactor-lifecycle`, `runtime-instances` |
| `extension` | L11 | 1,917 | 149 | 3 | `🧩️extension` | 💡️inference, 🛂️describe, runtime lines 5189-5587 | `abi`, `identity`, `decl-surface`, `reactor-app`, `decl-registry`, `plugin-program`, `reactor-lifecycle`, `runtime-instances` |
| `reactor-surfaces` | L12 | 541 | 26 | 4 | `† ⚛️reactor/📥️cold-pair` | ⚛️reactor/📸️checkpoint | `reactor-app`, `reactor-lifecycle`, `runtime-instances`, `runtime-ops` |
| `reactor-jobs` | L12 | 1,033 | 86 | 4 | `⚛️reactor/💼️jobs` |  | `abi`, `actor-host`, `job`, `decl-registry`, `plugin-program`, `extension` |
| `runtime-exchange` | L12 | 812 | 52 | 4 | `† ⚛️reactor/🏃️runtime/🔄️exchange` | runtime file lines 3838-4649 (plugin_exchange) | `window-config`, `decl-views`, `decl-emission`, `decl-app`, `job`, `reactor-app`, `plugin-program`, `reactor-lifecycle`, `runtime-instances`, `runtime-ops` |
| `reactor-core` | L13 | 748 | 39 | 4 | `⚛️reactor` | (root file after cuts) | `actor-host`, `decl-emission`, `reactor-app`, `reactor-lifecycle`, `runtime-instances`, `reactor-surfaces` |
| `reactor-turn` | L14 | 2,459 | 198 | 4 | `⚛️reactor/🔄️turn` |  | `abi`, `actor-host`, `operation`, `window-kits-rich`, `job`, `command`, `peer`, `reactor-app`, `reactor-lifecycle`, `runtime-instances`, `runtime-ops`, `extension`, `reactor-patches`, `reactor-surfaces`, `reactor-core`, `runtime-exchange` |
| `reactor-wit-bridge` | L15 | 594 | 49 | 4 | `† ⚛️reactor/🧬️wit-bridge` | inline mod wit_bridge, reactor root lines 1236-1829 | `reactor-app`, `reactor-lifecycle`, `reactor-turn` |
| `reactor-context` | L15 | 462 | 102 | 4 | `⚛️reactor/🎟️context` |  | `abi`, `reactor-app`, `reactor-lifecycle`, `runtime-ops`, `reactor-jobs`, `reactor-turn` |

All 41 are at or below 2,832 item lines and 198 KB before the 6 to 9 percent of `use`/region lines; the tightest are `reactor-turn` (198 KB), `window-config` (182), `decl-views` (182), `document-media` (179), `runtime-ops` (172). Test lines per crate are in 11.2; crates marked "COMPANION" there do not carry their tests.

**[WP0 change] 6.1 as frozen.** A 42nd lower crate, `reactor-tasks` (D8, WP4, root `⚛️reactor/⏲️tasks`, dependencies `actor-host` and `decl-emission`, rank between `reactor-app` and `decl-registry`), exists as a skeleton; `reactor-core` and `reactor-turn` gain a dependency on it. The `†` of a row means that the plan creates the crate root file, not that the directory is new: the roots of `composition-emission`, `extension` and `publication-mounted` are existing directories without a `🦀️.rs`, and `reactor-surfaces` (`⚛️reactor/📥️cold-pair`) has an occupied root (class B, 12.5.2). Appendix H is the authoritative table (rank, work package, class, dependencies).


### 6.2 Notes per tight or unusual crate

* **`abi`**: three wasip2-only files (`🧬️schema/🦀️.rs` bindings, `🚪️io/🛂️authority/📤️return`, `📤️snapshot`) plus the root's `owned_abi` and `component_persistent_local` (about 260 lines, `†` new file); no dependency on any plugin crate. The macro `__semio_export_component_guest` (5 lines, `$crate::component`) stays here with a root `pub mod component { pub use ... wasip2 }`.
* **`window-config`** (2,600 lines / 182 KB): `config` + `retained` + registry + preparation; the pair is mutually dependent (retained loader ↔ owner), so it is one crate; its 1,192 test lines go to a companion.
* **`decl-views`**: 3,028 file lines in 10 files; the small retirement owners for the empty lanes (`NoTransient`, `NoConfig`, `NoDraft`, `NoPresence`) sit here because the marker types are defined here (they were mapped to the window crates by directory, which created a false cycle).
* **`decl-app`**: the knot; 2,414 lines / 168 KB.
* **`reactor-lifecycle`**: the knot plus the close registry, fixed slots and the guard types handed down from the runtime file and `reactor/🦀️.rs` (2,446 lines after moves).
* **`runtime-ops`, `runtime-instances`, `runtime-exchange`, `extension`, `reactor-lifecycle`**: the five pieces of `⚛️reactor/🏃️runtime/🦀️.rs` (cut points: lines 1-599 to the knot, 600-1880 instances/close pumps/opening, 1881-3837 ops (wire limits, codecs, document ops, backbone, media, surfaces, render, refresh, context menu, command ingress, typed operations, task resume, intents), 3838-4649 `plugin_exchange` (812 lines, one function), 4650-5188 the four export macros to the facade, 5189-5587 extension bundle to `extension`, tests). The 24 test files mounted from the file go with the piece they exercise (11.2).
* **`time-travel`** (1,482) / **`time-travel-panel`** (1,139) / history helpers to `command`: cut at the file's own regions: `Limits`, `Outcome`, `Ledger`, `StoreState`, `Supersessions` stay; `PanelView`, `Pointer`, `Panel` go to the panel crate; `Driver` (`MutationPages`, `MemberHistory`, `HistoryViewStamps`) goes to `command`.
* **`window-kits` / `window-kits-rich`**: the base crate holds the kit contract, form panels, label axes, the small kits and the context-menu builder (`Menu`); `window-kits-rich` holds tree, table and world3d (a new root `†` mounts the three sibling directories).
* **`publication-mounted`** vs **`composition-publication`**: the `🤝️group/🪟️mounted` subtree (9 files, 1,291 lines, receipts and member groups) is one crate; its parent keeps the pending publication and the group owner.

### 6.3 Imports the dissolved root block must be replaced by

Every carved file keeps its own `use crate::...` list rewritten to the new crate name (2,676 occurrences in 114 files; the oracle's `crate::` → crate name table is generated from the catalogue). The 22 files whose `use super::*` parent lands in another crate are:

`⏪️time-travel/🦀️.rs` (parent `job`'s facet), `⏪️time-travel/🧵️instance/🦀️.rs`, `⏯️tool-run/🦀️.rs`, `⏯️tool-run/🧵️instance/🦀️.rs`, `⏳️operation-progress/🦀️.rs`, `⚛️reactor/💼️jobs/🦀️.rs`, `⚛️reactor/🔄️turn/🦀️.rs`, `⚛️reactor/🦀️.rs`, `🌱️initialization/🏪️catalog/🦀️.rs`, `👥️peer/🦀️.rs`, `📜️declaration/📇️registration/🦀️.rs`, `📬️completion/⚠️fault/🦀️.rs`, three `🧩️composition/*` files, `🧬️schema/🪶️sqlite/🦀️.rs`, `🪆️child/🦀️.rs`, the three ladders, `🚪️lifetime/🦀️.rs`, `🛠️tool-machine/🦀️.rs`.

For each: replace the glob by the explicit list of names the file uses (the token set intersected with the parent's glob exports), keep region markers. External crates a carved crate needs are the intersection of the 44 manifest dependencies with its token set (generated; for example `child` needs 9, `decl-app` 26, `abi` 13, `reactor-lifecycle` 4). The four root `extern crate ... as dsl|protocol|store|vcs` lines are copied into each crate root that uses the alias.

### 6.4 Features

| feature | crates that carry items gated by it (forwarded by the facade as `crate/feature`) |
|---|---|
| `artifact-app-testing` | `child` (14 gated items), `job` (10 + 2), `retained-command` (8), `decl-surface` (8), `instance-*` (tool-machine, dispatch, root, 8), `app-builder` (4), `tool-run` (3 + tool-machine), `window-kits` (2 + 1), `peer` (2), `document-media` (2), `reactor-core` (1), `time-travel-turns` (1), `instance-app`; `testing-laws`, `testing-acceptance` are the feature's crates |
| `media-turn-testing` | `decl-views` (4), `operation` (3), `document-media` (1) |
| `reserved-producer-testing` | `decl-app` (3) |
| `original-actor-context-testing` | `reactor-context` (4), `reactor-patches`, `reactor-lifecycle`, `runtime-ops`, `extension` (1 each) |
| `canvas-runtime-assets` | `builder` (optional dependency `semio-framework-canvas-assets`) |

### 6.5 Statics (single-owner rule)

See 7.8 for the reactor statics; the other production statics each have exactly one owner and no outside user in the final assignment: `abi` (`SNAPSHOT`, `NEXT_TICKET`), `actor-host` (`VTABLE`), `composition-publication` (`NEXT_COMMAND_OWNER`), `decl-registry` (`CONTRIBUTED_MUTATION_REGISTRY`, `OWNER_MUTATION_ROSTER_REGISTRY`), `job` (`PUBLICATION_UNITS`), `plugin-program` / `extension` (the three inference registries), `runtime-instances` (`NEXT_INSTANCE_ID`), `runtime-ops` (`PEAK_TYPED_OPERATION_SLOTS`, `TYPED_OPERATION_SLOT_DRAIN_PENDING`), `time-travel` (`TIME_TRAVEL_SCHEMA_SHOWS_INPUTS`), `window-kits-rich` (`ARENA_UNBUILT_ROWS`), `window-config` (`NEXT_LIFETIME`), `interaction` (`EMPTY` x3), `decl-views` (`EMPTY`), instance family (`WITNESS`, `STREAK` in typed-ops, `TURNS` in time-travel-turns).

## 7. The two large objects

### 7.1 `VcsArtifactApp<A, M>` (the artifact instance): what exists

`PLUG/🧵️instance/🦀️.rs` defines `pub struct VcsArtifactApp<A: ArtifactApp, M: SpaceMember + MemberFactory = NoMembers>` with **227 fields** (364 lines with docs; 80 `pub(crate)`, 27 under `ManuallyDrop`, one `#[cfg(test)]`). It has **26 inherent impl blocks, one `Drop` impl and the `PluginApp` impl in 21 files**: the six band files of the earlier split (`🚪️ingress` 2,338 method lines, `🧾️typed-operations` 1,828, `📨️dispatch` 1,655, `🔌️plugin-app-impl` 1,612, `🖼️render` 1,099, `🧷️command-routes` 997) and two older children (`🛠️tool-machine` 384, `🤝️group` 318), the three ladders (`🪜️maintenance-ladder` 1,107, `🪜️close-ladder` 570, `🪜️envelope-ladder` 215), the two feature turn files (`⏪️time-travel/🧵️instance` 1,344, `⏯️tool-run/🧵️instance` 1,103) and eight small ones, **674 methods, 14,971 lines**. 49 of the methods are `pub` (the API of the 284 external files that name the type), 169 `pub(crate)`, 92 `pub(super)`, 224 private. An inherent impl cannot span crates, so the struct has to be taken apart.

Measured coupling (field level, `self.field` inside the impls): 335 methods touch exactly one component of 7.3, 133 touch two, 64 three, 41 four or more, and 101 touch no field (they call other methods or are helpers). Call level: `self.m()` callees resolved by name where the name is unique among the 674 (that graph forces some methods up, 7.3 step 4).

### 7.2 Design rules (binding for WP6 and WP7)

1. **One component struct per field group** (7.3): it owns its fields, its constructor, its invariants and every method whose touched components are all at or below it in rank. Field names do not change (`self.stores.store`, not `self.stores.document`), which keeps the rewrite mechanical: inside a component `self.f`, outside `self.<component>.f`.
2. **A method that needs fields of lower components takes them as parameters**, after `&self` / `&mut self`, in rank order (`&mut Lower` if it mutates, `&Lower` otherwise).
3. **Methods that touch four or more components, or call a higher-ranked method, are use-case methods**: methods of a borrowed bundle defined in the use-case crate (`struct DispatchCtx<'a, A, M> { pub stores: &'a mut InstanceStores<A>, pub children: &'a mut ChildComposition<M>, ... }`), never a `&mut VcsArtifactApp` parameter. The bundle is built by one inherent method of the composer (`fn dispatch_ctx(&mut self) -> DispatchCtx<'_, A, M>` in `instance-app`; split borrows through `pub` component fields). Use-case crates never name `VcsArtifactApp`.
4. **The composer `VcsArtifactApp`** (crate `instance-app`, about 330 lines) is the 19 component structs as `pub` fields, `new` / `with_registry` / `with_registry_on_bus` (assembly that only calls component constructors), `impl Drop` (15 lines), the 49 `pub` methods that external code calls (`new`, `with_registry`, `with_registry_on_bus`, `snapshot`, `preview`, `interaction_state`, `open_child`, `child_store`, `register_child`, the envelope-ingress and store-replacement entry points, the time-travel and tool-run accessors, ...; constructors and readers are composer methods, the rest are one-line delegations to a component or use-case function) and `impl PluginApp`. It sits **at the top** of the instance family because `impl PluginApp for VcsArtifactApp<A, M>` must be written in the crate that owns the type or the trait; the trait lives in `reactor-app` (L6), so the composer cannot be below the use-case crates without a newtype, and a newtype would change the type that 284 external files name.
5. **Upward calls go through bundles, `X` or move the callee down.** A method may be lifted to a higher crate (always dependency-safe, with its callers); it may not be pushed down past a component whose fields it touches.
6. **No behaviour-only trait is introduced to split an impl.** `InstanceExtension` already exists and is the single inversion point (7.7).
7. **Test-only methods** (`test_*`, 31 `cfg(test)` lines) stay with the component they touch under the same cfg.

### 7.3 Components (field groups), rank order, homed methods

Rank order (low to high) is the crate dependency order of tier 1. `homed` = methods whose final home is the component (lines); the algorithm is at the end of this section and runs in `plan_tools.py home`.

| component | fields | homed lines | crate (tier) | field names |
|---|---:|---:|---|---|
| (composer) | 2 | 7 | `app`, `mounted_policy` stay in the composer | `app`, `mounted_policy` |
| `ConstructorState<A>` | 9 | 5 | instance-stores | `constructor_receipts`, `constructor_recipient_admitted`, `constructor_recipient_retired`, `constructor_terminal_receipt`, `constructor_document`, `constructor_config`, `constructor_draft`, `constructor_interaction`, `constructor_lane` |
| `InstanceStores<A>` | 17 | 532 | instance-stores | `store`, `config_store`, `draft_store`, `presence_store`, `transient_store`, `window_config_store`, `window_transient_store`, `retired_window_transient_stores`, `artifact_one_item_factory`, `config_one_item_factory`, `draft_one_item_factory`, `presence_one_item_factory`, `transient_one_item_factory`, `presence_local_root_retirement_factory`, `transient_local_root_retirement_factory`, `unsupported_publication_contracts`, `interaction_store` |
| `PeerRoster<A>` | 14 | 58 | instance-stores | `own_color`, `peer_presence`, `peer_presence_generation`, `peer_roster_admission_generation`, `peer_roster_processed_generation`, `peer_roster_outcome_generation`, `peer_roster_reservations`, `peer_roster_scope`, `peer_roster_publications`, `peer_roster_outcomes`, `peer_presence_retirements`, `presence_peer_retirements`, `pending_presence`, `presence_marked_keys` |
| `ActionIngress` | 5 | 40 | instance-stores | `action_ingress_sources`, `pending_action_ingress`, `pending_action_ingress_caller`, `pending_action_ingress_identity`, `pending_action_ingress_retirement` |
| `LocalInteraction<A>` | 14 | 94 | instance-stores | `local_interaction_query`, `local_interaction_query_receipt`, `local_interaction_authority`, `snapshot_read_return_stage`, `local_interaction_query_turn`, `local_interaction_emitted_bytes`, `local_interaction_retired_bytes`, `config_snapshot_read_returns`, `interaction_snapshot_read_returns`, `interaction_hover`, `interaction_leftover_selection`, `interaction_leftover_ids`, `interaction_ui_topology`, `interaction_topology_memo` |
| `CommandHistory<A>` | 14 | 302 | instance-history | `cache`, `command_log`, `command_prune_generation`, `command_prune_processed`, `pending_command_prune`, `next_command_seq`, `log_generation`, `history_dirty_sequences`, `history_view`, `history_backfill`, `shell_undone`, `shell_redo`, `history_filter`, `supersedes` |
| `HostEvents` | 5 | 86 | instance-history | `invocation_kind`, `host_event_utilities`, `host_event_roster`, `host_event_operations`, `delivering_host_event` |
| `Transactions<A>` | 2 | 0 | instance-history | `pending_transaction`, `pending_transaction_proposal` |
| `ChildComposition<M>` | 12 | 955 | instance-children | `children`, `private_child_groups`, `child_content_root`, `child_content_generation`, `child_content_retirements`, `child_member_retirements`, `child_member_retirement_generation`, `child_member_retirement_cursor`, `followed_parent_generation`, `child_follow_awaits_retirement`, `pending_child_pins`, `composition` |
| `PendingReserved<A>` | 22 | 203 | instance-jobs | `last_emit_wire`, `dispatch_report`, `pending_render_tree`, `pending_render_tree_identity`, `pending_render_tree_ready`, `pending_render_tree_close`, `pending_configuration_completion`, `pending_configuration_command`, `pending_configuration_command_close`, `pending_configuration_permit`, `pending_configuration_failure`, `pending_configuration_identity`, `pending_configuration_retirement`, `pending_render_failure`, `pending_render_identity`, `pending_render_close_failure`, `pending_render_compound`, `pending_render_retirement`, `pending_reserved`, `reserved_commits`, `reserved_commit_outcome`, `pending_document_window_admission` |
| `ToolOperations<A>` | 29 | 1,353 | instance-jobs | `pending_tool_lease_admission`, `typed_effect_outbox`, `typed_event_outbox`, `typed_ui_outbox`, `operation_progress_retired`, `typed_inline_interaction_verbs`, `typed_inline_interaction_leftover`, `typed_completion_outbox`, `typed_composed_outbox`, `registry`, `tool_job_controller_id`, `tool_jobs`, `agent_lane_preview`, `bounded_tool_proofs`, `bounded_tool_contracts`, `framework_tool_registrations`, `app_tool_registrations`, `instance_operation_owner`, `tool_cancellations`, `live_runtime_instance_id`, `tool_operations`, `test_tool_clock`, `typed_operation_reservations`, `typed_publication_cursor`, `typed_result_cursor`, `latest_wins_commands`, `latest_wins_order`, `latest_wins_keys`, `latest_wins_turn` |
| `ToolMachineRuntime` (existing type) | 1 | 459 | instance-jobs | `tool_machines` |
| `MediaTransfers<A>` | 12 | 643 | instance-docs | `pending_media_import_permit`, `media_exports`, `media_export_preparation`, `media_download_preparation`, `media_closures`, `snapshot_retirements`, `current_media_export`, `media_submission_refusal`, `media_submission_refusal_identity`, `media_submission_refusal_retirement`, `segmented_downloads`, `segmented_closures` |
| `EnvelopeJobs<A, M>` | 10 | 809 | instance-docs | `envelope_ingress`, `envelope_decode_jobs`, `store_replacement_jobs`, `import_staging`, `document_archive_loads`, `envelope_field_decoders`, `envelope_field_decoder_retirements`, `envelope_completed_records`, `envelope_completed_record_retirements`, `document_snapshot_read_returns` |
| `TimeTravelLedger<A>` (existing type) | 1 | 615 | time-travel (field of the composer) | `time_travel` |
| `ToolRunLedger<A>` (existing type) | 1 | 692 | tool-run (field of the composer) | `tool_runs` |
| `MaintenanceCursors` | 17 | 110 | instance-ladders | `maintenance_window_retirement_cursor`, `maintenance_stage`, `maintenance_cancellation_cursor`, `maintenance_tool_cursor`, `maintenance_media_cursor`, `maintenance_segment_cursor`, `maintenance_snapshot_cursor`, `maintenance_child_root_cursor`, `maintenance_peer_presence_cursor`, `maintenance_presence_peer_cursor`, `maintenance_peer_roster_cursor`, `maintenance_envelope_ingress_cursor`, `maintenance_envelope_decode_cursor`, `maintenance_store_replacement_cursor`, `maintenance_document_archive_cursor`, `maintenance_envelope_field_decoder_cursor`, `maintenance_envelope_completed_record_cursor` |
| `CloseCursors` | 40 | 501 | instance-ladders | `close_window_retirement_cursor`, `close_started`, `close_cancellation_cursor`, `close_typed_operation_cursor`, `close_media_cursor`, `close_media_cleanup_cursor`, `close_segment_cursor`, `close_segment_cleanup_cursor`, `close_snapshot_cursor`, `close_instance_operation_owner_drained`, `close_child_root_cursor`, `close_child_root_detached`, `close_child_member_cursor`, `close_child_member`, `close_peer_presence_cursor`, `close_peer_presence_detached`, `close_presence_peer_cursor`, `close_peer_roster_cursor`, `close_envelope_ingress_cursor`, `close_envelope_ingress_drained`, `close_envelope_decode_cursor`, `close_envelope_decode_stage`, `close_envelope_decode_jobs_drained`, `close_store_replacement_cursor`, `close_store_replacement_jobs_drained`, `close_document_archive_cursor`, `close_document_archive_loads_drained`, `close_envelope_field_decoder_cursor`, `close_envelope_field_decoders_drained`, `close_envelope_completed_record_cursor`, `close_envelope_completed_records_drained`, `close_snapshot_reads_drained`, `close_owned_stage`, `close_owned_advance_ready`, `close_document_disposer`, `close_config_disposer`, `close_draft_disposer`, `close_presence_disposer`, `close_transient_disposer`, `close_interaction_disposer` |

Algorithm (reproducible: `plan_tools.py home`, Appendix G). 1. `C(m)` = components whose fields `m` reads or writes (`self.f`, f in the table). 2. If `C(m) \ {core}` has four or more entries, or `m` belongs to the `PluginApp` impl, `m` is a **use-case** method (tier 2) or a **composer** method (tier 3). 3. Else `m` goes to the highest-ranked component of `C(m)`; the others become parameters. 4. Fixed point: while a method calls (by `self.name(..)`, unique name) a method of a higher rank, lift the caller. 5. A method with `C(m)` empty and no callee lifts nothing and goes to the first crate that calls it (`instance-history` by default, 875 lines: these are mostly associated functions with explicit parameters).

Result (methods by final home, lines): composer `PluginApp` 1,759; use-case 4,884; `tools` 1,353; `children` 946; free 875; `ingest` 809; `toolrun` 692; `media` 643; `timetravel` 615; `stores` 530; `close` 501; `toolmachine` 459; `history` 302; `pending` 203; `maintenance` 110; `interaction` 94; `hostevent` 86; `roster` 58; `ingress` 40; `core` 7; `constructor` 5.

### 7.4 Where every source file's methods go (lines of methods per final home)

| source file | i-stores | i-history | i-children | i-jobs | i-docs | i-ladders | i-commands | i-typed-ops | i-render | tt-turns | tr-turns | i-app |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `🧵️instance/🚪️ingress` | 196 | 540 | 508 | 413 | 514 | 37 | 130 |  |  |  |  |  |
| `🧵️instance/🧾️typed-operations` | 50 | 69 |  | 286 | 65 |  |  | 1346 |  | 12 |  |  |
| `🧵️instance/📨️dispatch` | 252 | 231 | 146 | 497 | 6 |  | 459 |  |  | 64 |  |  |
| `🧵️instance/🔌️plugin-app-impl` |  |  | 5 |  |  |  |  |  |  |  |  | 1607 |
| `⏪️time-travel/🧵️instance` |  |  |  |  |  |  |  |  |  | 1344 |  |  |
| `🪜️maintenance-ladder` | 10 | 49 |  | 5 | 323 | 693 |  |  |  |  |  | 27 |
| `⏯️tool-run/🧵️instance` |  |  |  |  |  |  |  |  |  |  | 1103 |  |
| `🧵️instance/🖼️render` | 72 | 71 | 39 | 195 | 298 |  |  |  | 424 |  |  |  |
| `🧵️instance/🧷️command-routes` | 28 |  |  | 195 | 70 |  | 637 |  |  | 67 |  |  |
| `🪜️close-ladder` |  | 12 |  | 68 | 27 | 338 |  |  |  |  |  | 125 |
| `🧵️instance/🛠️tool-machine` | 16 | 37 |  | 230 |  |  |  | 101 |  |  |  |  |
| `🧵️instance/🤝️group` |  | 88 | 60 | 7 | 52 |  | 111 |  |  |  |  |  |
| `🪜️envelope-ladder` |  |  |  |  | 9 | 206 |  |  |  |  |  |  |
| `🧵️instance` | 47 |  | 42 | 14 |  |  |  |  |  |  | 6 |  |
| `🎮️command/🎟️lease` |  |  |  |  | 75 |  |  |  |  |  |  |  |
| `🧵️instance/🔄️full-operation` |  |  |  |  |  | 56 |  |  |  |  |  |  |
| `🪟️window/🫧️transient/🔁️document-replacement` |  |  |  | 21 |  | 24 |  |  |  |  |  |  |
| `🧵️instance/⏳️operation-progress` |  |  |  | 44 |  |  |  |  |  |  |  |  |
| `🎮️command/📥️ingress` | 40 |  |  |  |  |  |  |  |  |  |  |  |
| `♻️lifecycle/🏪️stores` |  |  |  |  |  | 32 |  |  |  |  |  |  |
| `🪆️child/👁️read/🧪️test-view` | 2 |  | 9 |  |  |  |  |  |  |  |  |  |
| **total** | **713** | **1,097** | **809** | **1,975** | **1,439** | **1,386** | **1,337** | **1,447** | **424** | **1,487** | **1,109** | **1,759** |

(Columns are final homes. Components map to crates as in 7.3; the 875 "free" lines go to `instance-history`; the lines of a use-case column are the methods of that file that touch four or more components or call a higher-ranked method; `time-travel-turns` and `tool-run-turns` take their whole source file, including methods that would otherwise be tier 1. `plan_tools.py home --dump` prints every method with its home.)

### 7.5 The instance-family crates

| crate | tier | contents | est. lines | est. KB | deps inside the family | WP |
|---|---|---|---:|---:|---|---|
| `instance-stores` | 1 | components `ConstructorState`, `InstanceStores`, `PeerRoster`, `ActionIngress`, `LocalInteraction` (59 fields), the helper items of the instance parent (`OriginalRenderTreeIdentity`, `OriginalRenderInteractionStamp`, `artifact_handle_of`, ...), their methods | 1,300 | 100 | - | 6 |
| `instance-history` | 1 | `CommandHistory`, `HostEvents`, `Transactions` (21 fields) + the 875 free lines | 1,250 | 100 | stores | 6 |
| `instance-children` | 1 | `ChildComposition<M>` (12 fields; `children`, `private_child_groups`, `composition`, the child-content and retirement registries) | 950 | 75 | stores | 6 |
| `instance-jobs` | 1 | `PendingReserved<A>` (22), `ToolOperations<A>` (29), `ToolMachineRuntime` home (1) | 2,300 | 190 | history, children | 6 |
| `instance-docs` | 1 | `MediaTransfers<A>` (12), `EnvelopeJobs<A, M>` (10), `MediaExportPreparation` | 1,650 | 130 | jobs | 6 |
| `instance-extension` | 1x | `InstanceExtension` + the hook bundle types | 300 | 20 | all tier 1 | 6 |
| `instance-commands` | 2 | command routes, dispatch, group, ingress use-cases (`CommandCtx`) | 1,450 | 105 | tier 1, extension | 7 |
| `instance-typed-ops` | 2 | typed-operation publication, typed command, tool-machine turns (`TypedCtx`) | 1,550 | 115 | tier 1, extension | 7 |
| `instance-render` | 2 | render, interaction state, media jobs (`RenderCtx`) | 550 | 45 | tier 1, extension | 7 |
| `instance-ladders` | 2 | close, maintenance, envelope ladders; `CloseCursors` (40 fields), `MaintenanceCursors` (17) | 1,550 | 115 | tier 1, extension | 7 |
| `time-travel-turns` | 2 | `⏪️time-travel/🧵️instance` whole file + `TimeTravelExtension` | 1,600 | 115 | tier 1, extension, `time-travel` | 7 |
| `tool-run-turns` | 2 | `⏯️tool-run/🧵️instance` whole file + `ToolRunExtension` | 1,200 | 85 | tier 1, extension, `tool-run` | 7 |
| `instance-app` | 3 | `VcsArtifactApp`, constructors, `Drop`, the 49 public methods, `impl PluginApp` (128 methods), `InstanceExtensions` alias, `settle_framework_reserved_*`, MAINTENANCE stage constants, `editor_surface`/`viewer_surface` (I3) | 2,300 | 185 | everything above | 7 |
| `builder` | 4 | `PluginBuilder` typestate (`🏗️builder/🦀️.rs`) | 850 | 50 | `instance-app`, `app-builder`, `plugin-program`, `decl-registry`, `extension` | 7 |
| `testing-laws` | 4 (feature) | `🧪️testing` (`artifact_app_laws`, 2,707 lines / 195 KB, one module), `🎟️caller`, `⚖️authoring-law` | 2,850 | 214 | `builder`, `instance-app` | 7 |
| `testing-acceptance` | 4 (feature) | `🧪️tests/🧪️history-edit-acceptance` (1,946 lines), `node-drag-history`, `ui-action-collection` | 2,050 | 143 | `testing-laws` | 7 |

`testing-laws` is the one lib crate above 90 percent of the byte budget (214 of 230 KB); it carries no unit tests of its own (it is the law library the other crates' tests call) and has one natural cut (its `//#region` list) if it grows.

### 7.6 Use-case bundles (what each tier-2 crate touches)

| bundle | methods | lines | components touched (distinct field reads) |
|---|---:|---:|---|
| `CommandCtx` (`instance-commands`) | 27 | 1,337 | children 19, stores 16, tools 16, history 13, hostevent 9, pending 5, core 4, ingress 4, ingest 4, txn 2, toolmachine 2 |
| `TypedCtx` (`instance-typed-ops`) | 21 | 1,447 | tools 29, stores 29, children 7, core 5, toolmachine 5, txn 2, hostevent 2, interaction 2 |
| `RenderCtx` (`instance-render`) | 11 | 424 | interaction 15, tools 8, stores 4, roster 4, children 2 |
| `LadderCtx` (`instance-ladders`) | 51 | 1,386 | tools 70, stores 52, close 50, ingest 36, maintenance 33, children 32, roster 22, media 21, history 19, pending 8, interaction 7, txn 6 |
| `TimeTravelCtx` (`time-travel-turns`) | 71 | 1,487 | timetravel 48, stores 27, history 26, children 11, toolrun 3, ingest 3 |
| `ToolRunCtx` (`tool-run-turns`) | 44 | 1,109 | toolrun 38, stores 17, children 10, tools 8, core 2 |
| composer (`instance-app`) | 128 | 1,759 | tools 76, stores 59, roster 32, media 28, history 26, pending 26, close 23, interaction 22, children 12, ingest 11, timetravel 9 |

### 7.7 Inversion I1 in detail: the extension list becomes a parameter

Today `trait InstanceExtension` (176 lines) has 14 hooks, each `fn hook<A, M>(app: &VcsArtifactApp<A, M>, ...)`, and the instance calls `InstanceExtensions::hook(self, ..)` on the concrete alias at 22 sites in 6 files (`plugin-app-impl` 8, `command-routes` 4, `dispatch` 3, `typed-operations` 3, `tool-machine` 3, `ingress` 1). Target:

```rust
pub trait InstanceExtension {
    fn claims_action(action: &str) -> bool { false }
    async fn dispatch_action<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static>(ctx: &mut ExtensionCtx<'_, A, M>, action: &str, ...) -> Result<..>;
    fn mutation_refusal<A, M>(ctx: &ExtensionCtx<'_, A, M>, verb: &str) -> Option<Fault> { None }
    // the other twelve hooks, same shape
}
```

`ExtensionCtx` (in `instance-extension`) holds exactly the components the two implementations touch (stores, history, children, jobs, docs, the two ledgers). Use-case functions are `fn route<A, M, X: InstanceExtension>(ctx: &mut CommandCtx<'_, A, M>, ...)`. `instance-app` fixes `type InstanceExtensions = (ToolRunExtension, (TimeTravelExtension, ()))` and instantiates every use-case function with it, so monomorphisation is unchanged. Phase 2 (decision D2): add `type State<A>` to the trait, replace the fields `tool_runs`, `time_travel`, `tool_machines`, `supersedes` of the composer by `InstanceExtensions::State<A>`, and replace the four `PluginApp` contract methods `take_tool_run_receipt`, `take_history_command_receipt`, `tool_run_presence`, `tool_run_trace_delta` and the ladder rungs `CloseRung::{ToolRuns, TimeTravel}` / `MaintenanceUnit::{ToolRuns, TimeTravel}` (ordinal codes 105 and 106) by hooks; after that nothing below `instance-app` names the two domains and both families can be relocated to their products without touching the instance.

### 7.8 Reactor runtime state (not a god impl; a function bag and a static block)

**`PluginRuntime<PA>`** (`reactor-lifecycle`): 14 fields (`mounted_policy`, `guest_lifetimes: RefCell<NativeLifecycleRegistry<PA>>`, `plugin`, `plugin_assembly_error`, `instances`, `close_quarantine`, three cursors, `close_generation`, `local_interaction_query_generation`, `instance_actors`, `document_backbones`) and a 24-line impl (`new`, `mounted_owner_policy`). Because the struct lives in the lifecycle knot (its registry type names the lease that closes through it) the struct itself is in `reactor-lifecycle`; the 176 free functions over it stay free functions in the five function-family crates and reach the fields through the existing `pub(crate)` accessors, now `pub`. The struct is not decomposed.

**The 176 functions / 5 macros of `⚛️reactor/🏃️runtime/🦀️.rs`** by family (line ranges of the current file): lines 39-599 (guards, cleanup faults, status enums, `RuntimeInstanceRegistry<T>`, `RuntimeAppCell`, `PluginRuntime`) → `reactor-lifecycle` / `runtime-instances`; 600-1005 (descriptor, `plugin_manifest`, opening an actor instance, `plugin_create_app`) → `runtime-instances` and `plugin-program` (wire descriptor helpers); 1007-1880 (live and close cleanup pumps and jobs, `plugin_begin_instance_close`, `plugin_step_live_cleanup`) → `runtime-instances`; 1881-2910 (public JSON validation and wire limits, document ops, codecs, document backbone, media, surfaces, render, `plugin_refresh_ui`, context menu) → `runtime-ops`; 2911-3837 (open/default-app relays, `PluginCommandIngress`, typed-operation driving, task resume, dispatch intents, presence) → `runtime-ops`; 3838-4649 (`plugin_exchange`, 812 lines in one function) → `runtime-exchange`; 4650-5188 (four export macros) → facade; 5189-5587 (`ExtensionBundle`, `extension_*` entry points) → `extension`.

**Actor-persistent statics** (the `component_persistent_local!` block of `⚛️reactor/🦀️.rs` and its neighbours; one owner crate each; on wasip2 they are linear-memory globals, on native thread-locals; no crate may define a second copy):

| static | owner crate | users outside the owner |
|---|---|---|
| `PATCHES` (`PatchTracker`) | `reactor-patches` | `reactor-turn`, `reactor-core` via accessor |
| `PRESENCE`, `INSTANCE_METADATA`, `PACKAGE_ASSETS`, `ARMED_TIMERS`, `TASK_RECORDS`, `TASK_RESUMES`, `JOB_RENDER_BINDINGS` | `reactor-core` | `reactor-turn`, `reactor-surfaces` (checkpoint), `reactor-wit-bridge` |
| `REGISTRY` (`RequestRegistry`), `REACTOR_EXECUTOR`, `TASK_EXECUTOR` | `actor-host` | `reactor-core`, `reactor-turn` |
| `REACTOR_CLOSES`, `REACTOR_CLOSE_CURSOR`, `PENDING_PATCHES`, `RUNTIME_CLOSE_PENDING_AUTHORITY_TURNS` | `reactor-lifecycle` | `reactor-core`, `runtime-instances` |
| `COLD_PAIR_INGRESS`, `COLD_PAIR_DOCUMENT_LOAD`, `RESTORED_DOCUMENT_LOADS` | `reactor-surfaces` | `reactor-turn` |
| `RESPONSES`, `RESPONSE_CURSOR`, `INTENTS`, `TURN_EVENTS`, 10 more turn statics | `reactor-turn` | `reactor-wit-bridge` (3: `INTENTS`, `RESPONSES`, `TURN_EVENTS`) |
| `KIND_REGISTRY`, `KIND_REGISTRY_EPOCH`, `REGISTERED_KIND_COUNT`, `JOBS` | `reactor-jobs` | `reactor-turn` |

### 7.9 Residual items of the instance parent

* The three `include!`s (`document-replacement` 45 lines, `command/🎟️lease` 75, `lifecycle/🏪️stores` 32) become `mod`s of the crates that hold their homed methods (`instance-jobs`, `instance-docs`, `instance-ladders`); this retires W5 of the relocation result, which had deferred it because it widened privacy: here it is part of the component boundary, so the widening is exactly what the boundary needs.
* `DocumentWindowResetAdmission` (5 lines) joins `instance-stores` with its impl (I4).
* `OriginalRenderTreeIdentity`, `OriginalRenderInteractionStamp` (108-line impl), `MediaExportPreparation<A>` and the 13 free helper fns go to the crate that owns the fields they read (`instance-stores`, `instance-docs`).

## 8. Wrong-direction domains the seam map recorded, on the current tree

| domain | state after the relocation slice | plan |
|---|---|---|
| tool-run / time-travel inside the SDK | contracts (`ToolRunJobRequest`, ports, trace keys, run view) are declaration vocabulary in `decl-views` / `decl-app`; the runtimes (`tool-run` 1.9k, `time-travel` 1.5k + panel 1.1k + history views in `command`) and their instance turns (`time-travel-turns`, `tool-run-turns`) are domain modules behind `InstanceExtension`; the generic instance still names them at 18 + 12 call sites, 2 fields, 4 `PluginApp` methods and 2 ladder rungs | Phase 1: separate crates, lowered under the instance, turns above the generic use-cases (I1). Phase 2 (D2): state behind `InstanceExtension::State`, then relocate the five crates to their product modules. The peer note's "explicit plugin domain ports" for `time-travel` are I1 + the Phase 2 hooks: `time-travel` itself needs none, because after the move it depends only on lower crates (`decl-views`, `decl-app`, `child`, `command`, `tool-run`, `time-travel-panel`) |
| `ui_wgpu` | **no code mention remains in the crate** (checked with a comment-stripped scan; the earlier note listing builder, world3d, window bridge and three tests is out of date) although `PKG/Cargo.toml` still declares `ui_wgpu = { ..., features = ["wgpu"] }` | WP0 deletes the manifest line and its comment; the compiler confirms (any `E0432` names a remaining user). No carved crate gets the dependency |
| `os_vcs` in `GuestRuntime` | done in the host crate (`semio-framework-plugin-host` depends on `semio-framework-os-vcs` directly; the three re-exports are gone) | none here; that crate does not depend on this one |
| `semio-framework-os-kernel` aliasing | four `extern crate ... as dsl/protocol/store/vcs` at the root; 27 of 41 lower crates use at least one | each crate declares the aliases it uses; the kernel stays a peer-owned dependency of the whole family (no plugin crate can build before `os-kernel`, so the plugin chain of about 24 layers (19 with Option B of I6) starts after it) |
| composition by text (`include!`) | 3 in the instance parent, 1 in the runtime, 5 in tests | 7.9; the runtime one disappears with the lease move; tests keep `include!` inside one companion crate |

**[WP0 change] `ui_wgpu` (D3).** The dependency line and every code mention were already gone when WP0 started (a peer replaced them, see 1.3). Verified: no `ui_wgpu` key in `PKG/Cargo.toml`, a comment- and string-stripped scan of the lib and test closures finds no `ui_wgpu` in code (19 files mention it in comments or strings) and the manifest keeps three historical comment lines that name it. Nothing for WP0 to delete; the compiler confirms at the first real check.


## 9. cfg targets: guest wasip2 vs native

| target | what compiles | where |
|---|---|---|
| `wasm32-wasip2` (guest) | everything; plus the wasip2-only content below (1,311 lines) | all crates; the facade pulls `reactor-wit-bridge` through `[target.'cfg(all(target_arch = "wasm32", target_env = "p2"))'.dependencies]` |
| native (hosts, tests) | everything except the wasip2-only files; native fallbacks for `component_persistent_local` (thread-local) | all crates |
| `wasm32-unknown-unknown` | same file set as native (185 files) | all crates |

wasip2-only content: `abi` (`🧬️schema/🦀️.rs` 94 lines of `wit_bindgen::generate!` bindings, `🚪️io/🛂️authority/📤️return` 77, `📤️snapshot` 105, the root's `owned_abi` 140 and persistent-local module 30 → about 450 lines); `reactor-context` (`🎟️context/🌉️call/🎭️exports` 113); `reactor-wit-bridge` (594); facade glue (`component_export_anchor`, `wasip2` mount, 5 `cfg` sites). Counts of cfg sites inside single files: 67 `all(wasm32, p2)` (48 reactor, 10 io, 5 root, 3 host), 12 `wasm32` (8 reactor, 3 instance, 1 envelope ladder), 6 `not(all(wasm32, p2))` (host fallbacks, root, 1 reactor), 3 `any(test, wasm32)`, 1 `not(wasm32)`. The `wit_bindgen` dependency and the generated `wasip2::...` types live in `abi`; `reactor-context` and `reactor-wit-bridge` import them (`abi::wasip2::exports::...`), so exactly one `generate!` expansion exists. The `__semio_export_component_guest!` macro (`$crate::component`) is exported by `abi` with a `pub mod component` shim, and the facade's own `component` module re-exports it unchanged.

Per-target census rows to record per crate (section 13): lib native, lib `wasm32-wasip2`, lib `wasm32-unknown-unknown`, lib + testing features, test native, test `wasm32-wasip2`. Only `abi`, `reactor-context`, `reactor-wit-bridge` and the facade differ between native and wasip2 (rows differ by the lines above).

## 10. Public surface and consumer impact

### 10.1 What the facade keeps

* Cargo name `semio-framework-plugin`, features, `#[macro_export]` names, every root name and every `pub mod` path. 4,830 Rust files name the crate; the heaviest paths are root names (`UiAssemblyResult` 1,257, `ViewModel` 1,084, `artifact_app_laws::fixture_identity` 1,071, `io::Confidence` 918, `MountedOwnerPolicyV1` 723), `retained_command::*` (166 + 92 + 73 ...), `app::*`, `describe::describe_plugin` (13), `plugin_runtime::{install_plugin_bundle, install_plugin_bundle_result, PluginRuntime, plugin_manifest, extension_dispose_cold, register_document_codec_for_app}` (27 files), `reactor::{poll_kernel, original_actor_context::*}` (17 files), `document_backbone_binding`.
* The facade's `PLUG/🦀️.rs` keeps its shape: the 96 mounts become `pub use <crate>::*;` lines inside the existing `pub mod app` and explicit `pub use` for the `pub mod` paths: `pub use semio_framework_plugin_retained_command as retained_command;`, `pub mod plugin_runtime { pub use runtime_instances::{..}; pub use runtime_ops::{..}; pub use extension::{..}; pub use reactor_lifecycle::{PluginRuntime, ..}; }` (explicit lists generated by listing the `pub` items of each split piece), `pub mod reactor { pub use reactor_core::*; pub use reactor_turn::{poll_kernel, ..}; pub mod original_actor_context { pub use reactor_context::*; } }`, `pub mod declarations`, `describe`, `host`, `engagement`, `world3d_host`, `sqlite_wire`, `operation_authority`, `action_ingress`, `document_backbone_binding`, `wasip2` (cfg), `component_persistent_local`.
* `plugin_app_close_prelude` and `__semio_dispatch_PluginApp` (the `#[dyn_enum]` expansion on `PluginApp`, now exported by `reactor-app`) are re-exported by the facade; plugin crates keep calling `dyn_enum_close!` with `semio_framework_plugin::plugin_app_close_prelude::*` in scope (risk R2).
* `pub(crate)` modules that the facade's `app` hub globbed (`pub use super::artifact_declaration::*` and 40 more) become `pub use <crate>::*` of the owning crate; names that were `pub(crate)` and are now `pub` for cross-crate use carry `#[doc(hidden)]` and are not re-exported (the 247 widenings of 2.3 item 5).

### 10.2 The 18 exported macros

| macro | `$crate::` names inside | home |
|---|---|---|
| `__semio_actor_exports`, `__semio_owned_core_exports`, `__semio_plugin_actor_exports`, `plugin_exports`, `extension_exports` | `__async, component, io, operation_authority, plugin_runtime, reactor, sqlite_wire, encode_fault_bytes, owned_abi, describe, ExtensionBundle, PluginAssemblyError, app, component_export_anchor, component_persistent_local, __semio_*_descriptor_fresh_test` | facade |
| `register_plugin`, `derive_artifact_facets`, `subset`, `with_authoring_identity` | `plugin_runtime`, `ArtifactAnalysis ... io`, `Dialect ... derive_artifact_facets`, `__kernel`, `__value` | facade (already there) |
| `app_commands`, `app_action_enum`, `view_commands` | `ArtifactView, ConfigView, Emit, Fault, FromValue, ToValue, ToolCommandCatalog, ViewEmit, app_emission` | facade (the names are all re-exported at the facade root) |
| `transient_root`, `window_transient_owners` | `FromValue, ToValue, __diagnostic, __kernel, __pack_json, __value, Fault, FaultCode, FaultOrigin, TransientView, ViewModel, WindowTransient*` | facade |
| `window_transient_transfer` | `__kernel, __pack_json` | stays in `window-transient` (used once inside the crate); that crate roots `__kernel` / `__pack_json`; the facade re-exports the macro |
| `bounded_first_step_tool_proofs` | `ArtifactBoundedFirstStepProof` | facade |
| `component_persistent_local` | `component_persistent_local` module | `abi` (13 uses in the reactor crates), facade re-exports |
| `__semio_export_component_guest` | `component` | `abi` |

Rule: a macro moves to the facade unless a carved crate's own library code invokes it; in that case it stays with an own-crate `$crate` root that provides the names it spells. The five root re-exports `__diagnostic`, `__kernel`, `__pack_json`, `__value`, `__async` and the `ToValue`/`FromValue` derive re-exports stay in the facade root.

### 10.3 Consumer edits (all small)

1. `Plugin::builder(id)` is deleted (I2): the two callers are in the crate's own `🧪️testing/🦀️.rs` lines 2700 and 2729; they become `PluginBuilder::new(id)`; six doc comments and one ticket fixture mention it.
2. No Cargo edit for any dependent: all keep depending on `semio-framework-plugin`. The facade manifest gains the 57 path dependencies; the 5 features forward (`artifact-app-testing = ["dep:semio-framework-plugin-testing-laws", "semio-framework-plugin-child/artifact-app-testing", ...]`).
3. `semio-framework-os-renderer-wgpu`'s kernel pool, `os-run`, the hub compositions and the plugin test hosts keep calling `plugin_runtime::*` / `reactor::poll_kernel` through the facade; the renderer plan's WP1 edit (`package_assets_beside_module`) is unaffected.
4. The 284 external files that name `VcsArtifactApp` see the same type with the same 49 public methods.

## 11. Tests

### 11.1 What exists

274 test-only `.rs` files (60,546 lines, 4.98 MB), 269 embedded fixtures (1.73 MB), 10 Rust test files with **25 `include_str!("....rs")`** that embed other source files, and TS readers: 532 literals / 104 files / 283 targets. Largest single test files: `🔬️plugin-runtime-plugin-builder-contract` 10,660 lines / 869 KB, `🧪️time-travel` 3,394 / 286 KB, `🔬️tool-run` 2,381 / 205 KB, `🔬️app-typed-command-full-operation` 1,956 / 191 KB (+ an 859 KB fixture and 67 KB `child-group-wire`), `🧩️composition` 2,522 / 167 KB. Six fixtures of 40 KB to 859 KB total 1.26 MB.

### 11.2 Rules

1. A test file stays in its crate's `🧪️tests` if the crate's lib closure plus its tests fit (3,500 / 28 / 230,000). 34 of the 41 lower crates qualify by the host estimate (table in Appendix D, lib size x 1.08 plus the tests hosted by the crate); **COMPANION** hosts: `window-config`, `decl-views`, `decl-app`, `document-media`, `extension`, `reactor-turn`, `time-travel`, plus the instance family, the builder and the facade (host = the crate whose names the test mentions most, highest rank winning; tests that mention no plugin name go to the first companion bin).
2. Companion crates `x-tests`: dev-only, `publish = false`, lib root of `#[cfg(test)]` mounts, depend on the production crates through `pub` API only (decision D4). Packing: first-fit over hosts in rank order, 3,300 lines / 215 KB / 26 files per crate.
3. **Embedded fixtures over 40 KB** are read at run time from `CARGO_MANIFEST_DIR` (zero budget cost); smaller ones stay `include_str!`. 
4. **The 25 source embeds** (`include_str!("..🦀️.rs")`, mostly instance, runtime and time-travel files) become run-time `std::fs::read_to_string` of the file list that the crate map provides (a `readModuleTreeSource`-style helper in a shared `test-support` crate, `†` WP0), or move to TS; they cannot stay embedded because they pull another crate's source into the test closure. Concatenation tests (`instance_source()` of the earlier split) read the new crate roots.
5. The 532 TS/Rust path literals are re-pointed in the same change that moves their target; WP0 owns the generated list (`rwp0-tools/ts_compare.py` style).
6. The heap-witness law (`#[global_allocator] HeapWitness`, `cfg(test)`) belongs to exactly one binary at a time: it moves into the companion crate that hosts the turn-retention law.

### 11.3 Packing (computed from the host estimate; the oversize files must be cut by test function)

| bin | files | lines | KB | hosts |
|---|---:|---:|---:|---|
| T01 | 26 | 931 | 45 | none |
| T02 | 6 | 747 | 60 | none, window-config |
| T03 | 16 | 3,002 | 214 | decl-app, decl-views |
| T04 | 6 | 2,507 | 182 | decl-app, document-media |
| T05 | 1 | 141 | 35 | document-media |
| T06 | 15 | 2,953 | 213 | extension, reactor-turn |
| T07 | 20 | 2,790 | 213 | reactor-turn, time-travel |
| T08 | 26 | 1,095 | 186 | time-travel |
| T09 | 5 | 64 | 22 | time-travel |
| T10 | 3 | 3,199 | 214 | TOP |
| T11 | 3 | 2,441 | 214 | TOP |
| T12 | 2 | 2,167 | 201 | TOP |
| T13 | 4 | 2,454 | 204 | TOP |
| T14 | 4 | 1,070 | 158 | TOP |
| T15 | 1/5 of `🔬️plugin-runtime-plugin-builder-contract` | 2,132 | 173 | TOP (cut by test-function groups) |
| T16 | 1/5 of `🔬️plugin-runtime-plugin-builder-contract` | 2,132 | 173 | TOP (cut by test-function groups) |
| T17 | 1/5 of `🔬️plugin-runtime-plugin-builder-contract` | 2,132 | 173 | TOP (cut by test-function groups) |
| T18 | 1/5 of `🔬️plugin-runtime-plugin-builder-contract` | 2,132 | 173 | TOP (cut by test-function groups) |
| T19 | 1/5 of `🔬️plugin-runtime-plugin-builder-contract` | 2,132 | 173 | TOP (cut by test-function groups) |
| T20 | 1/2 of `🧪️time-travel` | 1,697 | 143 | TOP (cut by test-function groups) |
| T21 | 1/2 of `🧪️time-travel` | 1,697 | 143 | TOP (cut by test-function groups) |

21 bins = 14 packed bins + 5 for the 10,660-line builder-contract file + 2 for the time-travel file. The instance family and the builder own T10..T21 (12 bins) because their tests mention `VcsArtifactApp` / `PluginBuilder`. If every test moved out the packing would need 25 bins.

## 12. Work packages and hand-off protocol

### 12.1 Packages (disjoint source file sets; one executor each)

| WP | scope | crates | owns | est. lib lines |
|---|---|---|---|---:|
| 0 | infrastructure (serial, first and last) | all 58 lib + 21 test skeletons; registries; mount hub; facade; test-support crate; TS/Rust readers; locks; census | `PKG/**`, `PLUG/🦀️.rs`, `.vscode/*`, `📋️build-owners.json`, workspace + lock files, `🔣️taxonomy.json`, TS readers | - |
| 1 | foundations and windows | abi, actor-host, identity, interaction, operation, window-config, window-transient, window-kits, window-kits-rich | `🚪️io`, `🧬️schema`, `🌐host`, `📡️backbone`, `⚛️reactor/🧵️executor`, `🪪️identity`, `🕹️interaction`, `♻️cancellation`, `⏳️operation-progress`, `🌱️initialization`, `📬️completion`, `🪟️window/**`, `🪟️window-kits/**`, `🖱️context-menu` | 12,922 |
| 2 | declaration contracts | decl-views, decl-emission, decl-app, decl-surface, decl-registry, app-builder, plugin-program | `📜️declaration/**`, `🏗️builder/🧩️app`, `🏗️builder/🔌️program`, `⚛️reactor/🪟️surfaces`, `🫧️transient`, `🎚️config`, `📝️draft`, `👥️presence`, `🧵️task` | 13,539 |
| 3 | owners and extension | child, composition-emission, composition-publication, publication-mounted, document-media, job, command, peer, retained-command, extension | `🪆️child`, `🧩️composition/**`, `🗃️document`, `🎞️media`, `👷️job`, `🎮️command` (minus instance impls), `👥️peer`, `🧵️retained-command`, `🧩️extension`, `💡️inference`, `🛂️describe` | 14,340 |
| 4 | reactor | reactor-app, reactor-lifecycle, runtime-instances, runtime-ops, runtime-exchange, reactor-patches, reactor-surfaces, reactor-core, reactor-jobs, reactor-turn, reactor-wit-bridge, reactor-context | `⚛️reactor/**` (minus executor, surfaces), `🚪️lifetime` | 13,578 |
| 5 | features | tool-run, time-travel, time-travel-panel | `⏯️tool-run` (minus instance), `🛠️tool-machine`, `⏪️time-travel` (minus instance) | 4,544 |
| 6 | instance tier 1 | instance-stores, instance-history, instance-children, instance-jobs, instance-docs, instance-extension | the component struct definitions and the methods homed there | 7,750 |
| 7 | instance tier 2/3, builder, testing | instance-commands, instance-typed-ops, instance-render, instance-ladders, time-travel-turns, tool-run-turns, instance-app, builder, testing-laws, testing-acceptance | `🧵️instance/**`, `🪜️*`, `⏪️time-travel/🧵️instance`, `⏯️tool-run/🧵️instance`, `🏗️builder/🦀️.rs`, `🧪️testing`, `🧪️tests`, `⚖️authoring-law` | 15,950 |

Companion test crates are owned by the WP of their host (the T10..T21 bins by WP7).

### 12.2 Protocol (nobody edits another package's files; registries are edited once)

**Phase 0 (WP0, serial):**
**[WP0 change] executed 2026-10-15; what it created and the rules that follow are in 12.5.** Deviations from the steps below: step 2 builds 80 skeletons (42 lower, 16 family, 21 bins, `test-support`) and writes no boundary-corpus row (the corpus is the UI area's); step 3 does not cut the 11 exported macros, `owned_abi` and `component_persistent_local` out of their files (that would have changed textual macro scope and the equivalence evidence): the three facade macro mount modules exist and the owning package fills them (12.5.6), WP1 cuts the two root regions (12.5.3 rule 2b), the dead `ui_wgpu` line was already gone (D3), the boundary triple is replaced, the hubs are `PLUG/🪝️mounts/<hub>` (the `📥️` of this step collides with the existing `📥️poll` and `📥️receipt` folders); step 4 (S0) is not done by WP0: the owning package makes its own file cut as its first step, because those cuts are disjoint by owner (runtime file, reactor root and `wit_bridge`: WP4; time-travel file: WP5; instance family: WP6, 12.5.9) and the cross-package item moves are prepared as lift files (Appendix J).

1. Re-run `plan_tools.py census` and `closure.ts`; diff against sections 1 and 6; if a file set changed, rename the row in 12.1 (do not edit foreign files).
2. Generate all skeletons in one pass: `Cargo.toml` (name, `workspace` depth, `[lib] path = "../../🦀️.rs"`, the exact three-key boundary triple at the ceiling values, `[lints] workspace = true`, `[features]` per 6.4, dependencies from the `deps` column plus the externals of 6.3), `📋️project.json`, stub `🦀️.rs` with a docstring and the `extern crate` aliases it uses, `🧪️tests/🦀️.rs` stub, workspace member (byte-sorted), `📋️build-owners.json` row, six launch routes in `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`, and the boundary corpus row where the area has one; refresh locks only through the sanctioned launcher. Register the `†` directories in `🔣️taxonomy.json` first (RGI check). No crate declares a dependency that is not in its `deps` cell, so `cargo metadata` cycle-checks the DAG before anyone moves code.
3. Cut the facade root into mount hubs: `PLUG/📥️mounts/wp1..wp7/🦀️.rs`, each owned by one package and `include!`d from `PLUG/🦀️.rs`; the 96 mounts move there so five executors can remove their own mounts without editing the same file. Move `owned_abi`, `component_persistent_local` (module + macro) to the `abi` skeleton; move the 11 exported macros that are not yet in the facade root (5 reactor, 3 emission, 1 job, 2 window transient) out of their current files into `PLUG/🧪️macros/*` (facade-owned); delete the dead `ui_wgpu` manifest line; replace the manifest boundary triple.
4. **Step S0 (mechanical, about one hour of scripting)**: apply the three file cuts and the I3/I4/I6 moves in place: write the runtime file as five staging files (`⚛️reactor/🏃️runtime/📥️staging/<crate>/🦀️.rs`), the time-travel file as three, `⚛️reactor/🦀️.rs` as four (`wit_bridge` to its own file), and every move of 5.2 as `📥️lifted/<source-slug>/🦀️.rs` in the destination crate's directory. The old tree must keep compiling (the staging files are mounted by the old parents), so the differential reports an identical multiset before and after S0; losslessness = the line multiset of the staging files equals that of the originals.
5. Freeze the interface in this ticket (this plan is the contract): crate names, the 19 component struct names and field names of 7.3, the bundle names of 7.6, the `InstanceExtension` hook list of 7.7, the facade's public list, the `deps` column. `📌️important.md` stays empty.

**Phase 1 (WP1..WP7 in parallel):** each package, in this order, inside its own file set:
1. Move whole files to their crate directory (mount by `#[path]` from the crate root only for the sibling directories listed in 6.1); replace each moved item's former mount with nothing (the mount hub entry is removed by the owner).
2. Wire the `📥️lifted/` items it receives with one `mod` line each; extract the items it gives away into the destination's `📥️lifted/`.
3. Widen visibility only as the definition scan demands (`pub(crate)` → `pub`, `#[doc(hidden)]` where the facade must not re-export), keep region markers balanced, replace every `crate::x` and `use super::*` by explicit imports (6.3).
4. Run the gates of section 13 in the foreground and write `RESULT-W4-PLUGIN-WPn.md` with the census rows.

Order inside a package: WP1 `abi`, `actor-host`, `identity`, `interaction`, `operation`, `window-config`, then `window-transient`, `window-kits`, `window-kits-rich`. WP2 `decl-views`, `decl-emission`, `decl-app` (the knot first, together with the member moves), `app-builder`, `decl-surface`, `decl-registry`, `plugin-program`. WP3 `child`, `composition-emission`, `composition-publication`, `publication-mounted`, `document-media`, `job`, `command` (with the history helpers), `peer`, `retained-command`, `extension`. WP4 `reactor-app` (with the I6 helpers), `reactor-lifecycle` (the knot, first), `runtime-instances`, `reactor-patches`, `reactor-surfaces`, `reactor-core`, `reactor-jobs`, `runtime-ops`, `runtime-exchange`, `reactor-turn`, `reactor-wit-bridge`, `reactor-context`. WP5 `tool-run`, `time-travel`, `time-travel-panel`. WP6 per tier-1 crate: component struct, constructor and invariants first, then methods (`plan_tools.py home --dump` prints every method with its home and regenerates the staging files), composer-independent. WP7 bundles and `X` first (frozen by WP0), then use-case crates, then `instance-app`, `builder`, the two testing crates.

**Cross-package order constraints (compile level, not edit level):** WP1 and WP2 are imported by everyone and merge into the skeleton first; WP3 after WP2's `decl-app`; WP4 after WP2's `plugin-program` and `decl-registry`; WP5 after WP3's `command`; WP6 after WP2/WP3/WP5; WP7 after WP6 and WP5. The facade (WP0) compiles last.

**Phase 2 (WP0, serial):** facade `Cargo.toml` and `pub use` lists, `app` hub rewrite, macros check, `pub mod` shims, delete the dissolved modules and the mount hubs, re-point readers (`readModuleTreeSource`, the 532 literals), regenerate the distribution manifest, run census for all crates, export parity (names listed by `cargo doc` JSON before and after: the set of public paths of the facade must be equal), `cargo metadata --offline`, lock refresh, TS suites that read these sources.

### 12.3 Executor rules that matter here

No `git commit/stash/checkout`, no worktrees, no sub-agents, re-read before every edit, foreground builds only, no compiling around the gate, `[DEBUG] ` prefix for temporary logs, one RESULT file per package, no `ticket_close` from executors, no `🗑️generated` sweep, never edit the peer-owned crates beyond consumer-side lines.

### 12.4 Decisions as resolved by the coordinator at WP0 (replaces 15.2)

| id | decision | consequence for the work packages |
|---|---|---|
| D1 | yes: use-case bundles and a top composer | sections 7.2 to 7.7 are binding for WP6 and WP7 |
| D2 | yes: Phase 2 of the extension inversion, scheduled as the final work package (**WP8**) after the whole carve has landed | WP7 implements Phase 1 of I1 only (type parameter `X`); the `State` associated type and the removal of the four contract methods and two ladder rungs are WP8's, not WP7's |
| D3 | yes: delete the dead `ui_wgpu` dependency | **already done by a peer before WP0** (the relocation session replaced it by `semio-framework-ui-layout` and `semio-framework-ui-declaration`); WP0 verified: no `ui_wgpu` line in `PKG/Cargo.toml`, and a comment-stripped scan finds no code mention in the lib closure. The manifest keeps three historical comment lines that name it; no carved crate gets the dependency |
| D4 | yes: companion test crates depend on production crates through `pub` API only | 11.2 rule 2; WP widening for tests uses `#[doc(hidden)] pub`, never a test-only feature on a production crate |
| D5 | keep `runtime-instances` and `reactor-lifecycle` separate | as in 6.1 |
| D6 | remove `Plugin::builder`, update its two callers, no compatibility | I2; **WP7** does it together with the builder carve (`🏗️builder/🦀️.rs`, callers at `🧪️testing/🦀️.rs`); WP0 did not touch it because it removes an item and would break the equivalence evidence |
| D7 | the facade owns the 15 macros | `[WP0 change]` WP0 pre-created the three facade macro mount modules (12.5.6); the owning package moves its `#[macro_export]` definitions into them |
| D8 | Option B of I6: a `reactor-tasks` crate, if WP4's definition scan confirms the statics move cleanly | `[WP0 change]` the skeleton `reactor-tasks` exists (WP4, root `⚛️reactor/⏲️tasks`, rank between `reactor-app` and `decl-registry`); `reactor-core` and `reactor-turn` depend on it, and the instance crates depend on `reactor-tasks` instead of `reactor-core`. If WP4's scan refuses (a static or record that cannot move without a back edge), WP4 records it in `RESULT-W4-PLUGIN-WP4.md` and asks WP0 to switch to Option A (the instance edges go to `reactor-core`; the skeleton row is deleted by WP0, never by WP4) |

### 12.5 WP0 hand-off contract (phase 0 executed 2026-10-15; evidence in `RESULT-PLUGIN-WP0.md`)

This section freezes what WP1 to WP7 work under. Appendices H (crate registry), I (hub units), J (lift files) and K (cross-package mount edges) are generated from the live tree and are part of the contract.

#### 12.5.1 State of the tree after phase 0

* **One compilation unit.** Nothing moved between crates; only mounts moved between files. Evidence: the patched full-resolution differential (12.5.11), the mount-equivalence check (`verify_hub.py`: 92 original mount units, identical module name, absolute target file and cfg attributes before and after), the TS module-tree comparison (`tree_compare2.ts`: the live tree is the original tree plus the seven hub files, no file lost, no line lost except the `../../` rebasing of 89 `#[path]` literals) and the static registry gate (`verify_static.ts`).
* **80 new crates** exist as skeletons: 42 lower (41 of 6.1 plus `reactor-tasks`), 16 instance-family / builder / testing, 21 companion test bins (`tests-01` to `tests-21`) and `test-support`. Each has `Cargo.toml`, `📋️project.json` and a one-file stub (12.5.2). The facade is the existing crate.
* **Mount hubs**: 84 of the root's 92 module mounts move into `PLUG/🪝️mounts/<hub>/🦀️.rs`, one hub per work package (`🌱️wp1-foundation`, `📜️wp2-declaration`, `👥️wp3-owners`, `⚛️wp4-reactor`, `⏯️wp5-features`, `🧱️wp6-instance-tier1`, `🧵️wp7-instance-tier2`), spliced into the root by seven `include!` lines at the position of the old facet block. Path literals are rebased by `../../`; facet regions are repeated, balanced, in every hub that holds a piece. Eight mounts stay in the root. Two stay because of textual macro scope: `derived_artifact_children_tests` and `subset_macro_tests` (tests of the root macros `derive_artifact_facets!` and `subset!`, which rely on definition order). Six stay because their file defines a `#[macro_export]` macro: `generated_test_contracts`, `wasip2`, `window_transient` (`transient_root!`, `window_transient_owners!`, `window_transient_transfer!`), `app_emission` (`app_commands!`, `app_action_enum!`, `view_commands!`), `tool_contract` (`bounded_first_step_tool_proofs!`) and `plugin_runtime` (five export macros). The first differential run put `window_transient` and `tool_contract` into hubs and reported six new `macro_expanded_macro_exports_accessed_by_absolute_paths` errors (`crate::transient_root!`, `crate::window_transient_owners!`, `$crate::window_transient_transfer!`, `crate::bounded_first_step_tool_proofs!` x4): a module declared inside an `include!`d file makes its `#[macro_export]` macros macro-expanded, and in-crate absolute paths to them are refused. All such units were returned to the root. Appendix I lists every unit with its hub. Hub 5 is empty: the time-travel and tool-run modules are mounted inside `👷️job/🦀️.rs` (Appendix K), not in the root.
* **Facade manifest** (`PKG/Cargo.toml`): one transitional block of 58 path dependencies (the 42 lower and 16 instance-family crates; `testing-laws` and `testing-acceptance` optional, `reactor-wit-bridge` in the `cfg(all(target_arch = "wasm32", target_env = "p2"))` table), the four feature lists now forward to the crates of 6.4 (`artifact-app-testing` also enables the two optional testing crates; `canvas-runtime-assets` also forwards to `builder`), and the `rust-source-boundary` triple is at the recipe ceiling (262144 / 32 / 4000). **No package edits the facade manifest during phase 1.** `Cargo.lock` is not refreshed.
* **Registries** are written and frozen (12.5.8): 80 workspace members, 80 owner rows, 480 launch routes in each launch file.
* **Not changed by WP0**: any source file of a package (except the root), `Plugin::builder`, the macro definitions, the 25 source embeds and 532 path literals of 11.2, `Cargo.lock`, the distribution manifest, any peer-owned crate.

#### 12.5.2 Crate folders and skeleton layout

Every skeleton has the package folder `<root>/📦️packages/🦀️rust/` with `Cargo.toml` and `📋️project.json`. Package name `semio-framework-plugin-<key>`, id `plugin-<key>`, Nx project `@semio-tech/plugin-<key>-rs`, `role = "framework"`, `workspace` = the repository root (a relative path of 8 to 10 `..`), the exact three boundary keys at the ceiling, `[lints] workspace = true`; companions and `test-support` also `publish = false`. The root directory is the one of 6.1 / 7.5 (Appendix H). Two classes exist, decided by what the root directory contained at WP0:

* **Class A (free root, 37 crates)**: the root directory had no `🦀️.rs` (new directories, `🧩️composition`, `🧩️extension`, the mounted-group directory, the companions). `[lib] path = "../../🦀️.rs"` as in the recipe; WP0 wrote the stub `<root>/🦀️.rs` (docstring, the `extern crate semio_framework_os_kernel as <alias>;` lines the crate's code names, and a `cfg(test)` mount of `🧪️tests/🦀️.rs` where the directory had no tests folder). This is the final form: the owner replaces the stub body.
* **Class B (occupied root, 43 crates)**: `<root>/🦀️.rs` is the module file the monolith still mounts. The manifest has `[lib] path = "🦀️.rs"` and the stub lives at `📦️packages/🦀️rust/🦀️.rs` (so the skeleton compiles and the monolith is untouched). **The carve flips it**: the owner reworks `<root>/🦀️.rs` into the crate root, sets `[lib] path = "../../🦀️.rs"` and deletes the package-local stub, in the same change that removes the unit from the hub (12.5.4). Until then the closure of a class-B crate is its stub (1 file).

Companions and `test-support` live under `PLUG/🧷️companions/<fruit>️tests-NN` and `🧯️test-support`, class A, with empty stubs; the owner mounts existing test files with `#[cfg(test)] #[path = "…"] mod …;` (no copies, no moves). Longest manifest path: 185 bytes.

**Folder and emoji rules** (checked by `verify_static.ts`): new directories are `<emoji>️<kebab>`, the emoji is RGI (bare or with U+FE0F), never repeated among siblings, and `🛬️lifted` is a name used nowhere else in the repository. The taxonomy has no member kind for the children of `🔌️plugin`: every existing plugin directory is `directory-kind-unresolved`, and so are the new ones (the verifier reports no other code), so no `🔣️taxonomy.json` row is written.

#### 12.5.3 Ownership (disjoint; nobody edits another package's files)

| WP | hub | crates | source trees (12.1, plus the reconciliations phase 0 found) |
|---|---|---|---|
| 0 | none | `test-support`, the facade, registries, root, `📣️macros` mounts | `PKG/**`, `PLUG/🦀️.rs`, `.vscode/*`, `📋️build-owners.json`, `Cargo.toml` files and locks, TS readers |
| 1 | `🌱️wp1-foundation` | abi, actor-host, identity, interaction, operation, window-config, window-transient, window-kits, window-kits-rich, `tests-01`, `tests-02` | `🚪️io`, `🧬️schema`, `🌐host`, `📡️backbone`, `⚛️reactor/🧵️executor`, `🪪️identity`, `🕹️interaction`, `♻️cancellation`, `⏳️operation-progress`, `🌱️initialization`, `📬️completion`, `🪟️window/**`, `🪟️window-kits/**`, `🖱️context-menu`; the root regions `🔖️ComponentPersistentLocal` and `owned_abi` (12.5.6) |
| 2 | `📜️wp2-declaration` | decl-views, decl-emission, decl-app, decl-surface, decl-registry, app-builder, plugin-program, `tests-03`, `tests-04` | `📜️declaration/**`, `🏗️builder/🧩️app`, `🏗️builder/🔌️program`, `⚛️reactor/🪟️surfaces`, `🫧️transient`, `🎚️config`, `📝️draft`, `👥️presence`, **`🧵️task`** (12.1 gives it to WP2; 6.1 lists the cold-task part under `operation`: the 19 lines that stay in `operation` are a lift, Appendix J) |
| 3 | `👥️wp3-owners` | child, composition-emission, composition-publication, publication-mounted, document-media, job, command, peer, retained-command, extension, `tests-05`, `tests-06` | `🪆️child`, `🧩️composition/**`, `🗃️document`, `🎞️media`, `👷️job`, `🎮️command` (minus the two instance impl files), `👥️peer`, `🧵️retained-command`, `🧩️extension`, `💡️inference`, `🛂️describe` |
| 4 | `⚛️wp4-reactor` | reactor-app, reactor-tasks, reactor-lifecycle, runtime-instances, runtime-ops, runtime-exchange, reactor-patches, reactor-surfaces, reactor-core, reactor-jobs, reactor-turn, reactor-wit-bridge, reactor-context, `tests-07` | `⚛️reactor/**` (minus executor and surfaces), `🚪️lifetime` |
| 5 | `⏯️wp5-features` (empty) | tool-run, time-travel, time-travel-panel, `tests-08`, `tests-09` | `⏯️tool-run` (minus `🧵️instance`), `🛠️tool-machine`, `⏪️time-travel` (minus `🧵️instance`) |
| 6 | `🧱️wp6-instance-tier1` | instance-stores, instance-history, instance-children, instance-jobs, instance-docs, instance-extension | the five new component directories, `🧵️instance/🧩️extensions`, and **step S0 of the instance family (12.5.9)** |
| 7 | `🧵️wp7-instance-tier2` | instance-commands, instance-typed-ops, instance-render, instance-ladders, time-travel-turns, tool-run-turns, instance-app, builder, testing-laws, testing-acceptance, `tests-10` to `tests-21` | `🧵️instance/**` after S0, `🪜️*`, `⏪️time-travel/🧵️instance`, `⏯️tool-run/🧵️instance`, `🏗️builder/🦀️.rs`, `🧪️testing`, `🧪️tests`, `⚖️authoring-law`, `♻️lifecycle`; the root fixture mounts of hub 7 |
| 8 | none | none | Phase 2 of I1 (D2), after the carve |

Rules that follow:

1. A package edits only its own hub, its own crate folders (manifest, root, tests, moved sources), its source trees and its lift files. A needed edit in a file another package owns is made by that owner, or recorded in the RESULT file as a request.
2. **Tolerated edits of foreign files, exactly these** (re-read first, touch only the named lines): (0) the owner of a root-resident unit replaces or deletes that unit in the root when the crate is carved (`window_transient` and `wasip2`: WP1; `app_emission` and `tool_contract`: WP2; `plugin_runtime`: WP4; Appendix I marks them `root`); (a) the owner of a **child** deletes or replaces the mount line of that child in a foreign parent file (Appendix K lists the 15 edges: for example WP5 removes the `pub mod tool_run;`, `pub mod tool_machine;` and `pub mod time_travel;` mounts in `👷️job/🦀️.rs` and WP1 the `pub mod operation_progress;` mount); (b) WP1 cuts the root regions `🔖️ComponentPersistentLocal` and `owned_abi`; (c) WP7 removes `Plugin::builder` and its two callers.
3. **Moves between packages are lifts.** The package that owns the source file cuts the items into the lift file named in Appendix J (a docstring-only stub already exists there), the destination package wires it with one `mod` line in its crate root. Until the destination crate exists, the source module re-exports the items with `pub use` so the monolith keeps resolving.
4. No package creates a crate, an emoji folder outside its own crate directories, a workspace member, an owner row or a launch route, and none edits `PKG/Cargo.toml`. A package that needs one asks WP0 (edit the row in Appendix H, not the registries).

#### 12.5.4 The carve protocol with the hubs

1. Follow the order of 12.2 inside the package. For one crate: (1) make the root: class A, replace the stub body; class B, move the module file's content into the crate root layout, flip `[lib] path` and delete the stub (12.5.2). Keep every `//#region` marker balanced.
2. In your hub replace the unit of the module by `extern crate semio_framework_plugin_<key> as <old module name>;` (keep the unit's `cfg` attributes and its doc comment). The monolith keeps compiling against `crate::<old module name>::…` because an `extern crate … as name;` at the crate root defines `name` in the root namespace, and the `pub use super::<name>::*;` lines of `pub mod app` keep working. The renderer plan proved the pattern on a toy (10.4.4 there). For a module that maps to several crates (`plugin_runtime`, `reactor`, `artifact_declaration`, `artifact_inference`, `describe`), keep the unit until the last piece has left; the parent file you own becomes the thin facade that `pub use`s the new crates explicitly.
3. Replace every `use crate::x` and `use super::*` of the moved files by explicit imports of the new crate names (6.3), keep `extern crate semio_framework_os_kernel as dsl|protocol|store|vcs;` for exactly the aliases the crate names (the stub already carries them; the compiler tells the rest), and add the crate-level attributes the facade root carries where the moved items need them: `#![feature(associated_type_defaults)]` (decl-app, decl-surface and whoever holds `ArtifactEditor`/`ArtifactViewer`), `#![allow(async_fn_in_trait)]` (reactor-app, instance-app).
4. Widen visibility only as the definition scan demands (`pub(crate)` to `pub`, `#[doc(hidden)]` where the facade must not re-export); never fix with a qualified path.
5. Add downward edges to your own manifests only; an upward edge, a cycle or a dependency of a production crate on a companion is a defect (`verify_static.ts` fails on it). Rank order = Appendix H.
6. A `#[macro_export]` macro defined in a module declared inside an `include!`d hub cannot be named by absolute paths (`macro_expanded_macro_exports_accessed_by_absolute_paths`); this is why the units of Appendix I marked `root` and the macro mounts of 12.5.6 are root mounts, and why no macro-exporting module may be moved into a hub by hand. A package whose carve leaves a macro-exporting file behind keeps its unit in the root.
7. Run the gates of 12.5.11 in the foreground and write `RESULT-W4-PLUGIN-WPn.md` with the census rows. No package calls `ticket_close`.

#### 12.5.5 Lifts and cross-package mount edges

Appendix J lists the **26 cross-package lift files** (3,342 lines) with source and destination package; 28 further item-move groups (1,661 lines) stay inside one package and need no lift. Appendix K lists the **15 mount lines** in a foreign parent file; the child's owner edits that one line when the child leaves (rule 2a). Both tables are computed from the 12.1 ownership prefixes (`pwp0-tools/ownership.py`), so a file whose crate belongs to another package (for example `🪟️surfaces`, `🧵️task`) shows up there.

#### 12.5.6 Macros and the root

* The five root macros `with_authoring_identity!`, `register_plugin!`, `derive_artifact_facets!`, `subset!` and the `__semio_*_fresh_test` macros of `generated_test_contracts` stay in the root; their textual order is unchanged and nothing in a hub may precede them.
* **Facade-owned macro mounts (D7)**: `PLUG/📣️macros/{🌱️wp1-window, 📜️wp2-declaration, ⚛️wp4-reactor}/🦀️.rs` are mounted from the root by three `#[macro_use] #[path] mod` lines placed before every hub. They are empty (docstring only). When a package carves the file that defines an exported macro, it moves the `#[macro_export]` definition verbatim into the file of its package, in the same change: WP1 `transient_root!`, `window_transient_owners!` (`window_transient_transfer!` stays with `window-transient`, which roots `__kernel` and `__pack_json`); WP2 `app_commands!`, `app_action_enum!`, `view_commands!`, `bounded_first_step_tool_proofs!`; WP4 `__semio_actor_exports!`, `__semio_owned_core_exports!`, `plugin_exports!`, `__semio_plugin_actor_exports!`, `extension_exports!`. `component_persistent_local!` and `__semio_export_component_guest!` go to `abi` (I5), cut by WP1 from the root region and from `🧬️schema/🦀️.rs`; the facade re-exports them.
* A macro that a carved crate's own library code invokes stays with that crate and spells names it can resolve; the owner records it in the RESULT file.

#### 12.5.7 Dependencies, features and cfg

* Each skeleton manifest declares exactly the first-party edges of Appendix H and the external crates its units name (token evidence of `pwp0-tools/unitdeps.py`, with the specs and target tables of the old manifest, paths re-relativised; derive macros bring `semio-framework-value` / `-derive` / `-dsl-record(-derive)` / `-dispatch-macros`). The external lists over-approximate (a name in a doc or string counts); each package prunes to what its crate names and adds what the compiler asks for.
* Instance-family lower edges come from the method homing of 7.3; edges the token graph found towards `reactor-lifecycle`, `runtime-ops`, `reactor-jobs`, `reactor-core` and `extension` are the call sites that I6 removes, so they are replaced by `reactor-app`, `reactor-tasks` and the lower crates named in 5.3 (`builder` takes the dependencies of 7.5).
* Features: the `[features]` of 6.4 are declared (empty) in the carrying crates, forwarded by the facade; `builder` carries `canvas-runtime-assets = ["dep:semio-framework-canvas-assets"]` and the optional dependency. Dev-dependencies are the ones the hosted test files name (companions: the facade's six).
* cfg: no crate is native-only; `reactor-wit-bridge` is gated as a whole by the facade's wasip2 target table; `abi` keeps `wit-bindgen` as a normal dependency (its `generate!` stays in `🧬️schema/🦀️.rs`, path `../../🧬️schema` becomes `../../../🧬️schema` from a class-B package, R11).
* **Profiles**: the repository's `wasm-prototype` hot rows (`iterateHotPackages` of `🧑‍💻dev/…/🦀️wasm-profile-policy/🧬️v1/🔣️.json`, 146 manifests) exist only for the neural dag, replication and pack crates; neither the plugin crate nor any of its neighbours has a `[profile.*.package.semio-framework-plugin]` row, so splitting it loses no optimisation level and **no profile row is added** (a row for a crate without a hot-path witness would only add 80 lines to 146 manifests and to the fixture). If WP4 measures the 8 ms turn contract below `opt-level = 2`, it asks WP0 for rows for `reactor-turn`, `runtime-ops` and `reactor-core`.

#### 12.5.8 Registries (frozen; edited once by WP0)

* Root `Cargo.toml` `members`: 340 to 420 rows (80 new), byte-sorted in the one block.
* `📋️build-owners.json`: 80 rows `@semio-tech/plugin-<key>-rs`, `root` = package folder, `manifest` = its `📋️project.json`, `packages: []`, inserted by name; validated with `owners-tools/validate.ts` (`registry valid, owners: 2384`) and `scan.ts` (`missing: []`).
* `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: six routes per crate (`🦀️ <package> 🏗️build|🔎️check|🧪️test 🛠️prototype|🚀️release`, groups `4_build`, `4_gate`, `4_test`, command `bun repo run @semio-tech/plugin-<key>-rs:<target> --configuration=<mode> --parallel=1 -- --lib --offline`, with the trailing `env` object the peers' normalizer adds), inserted after the `semio-framework-os-renderer-wgpu-test-support` routes. Order keys are unused ones, identical in both files: build from 900.3, check from 922.062, test from 903.6011, key = base + 2 x (rank in Appendix H) + (0 prototype, 1 release), in steps of 0.00001.
* A package never adds, renames or removes a row. `verify_static.ts` checks all of it in about 30 seconds.

#### 12.5.9 The instance family: order of work

`🧵️instance/**` and the ladders are one tangle of 21 files that WP6 and WP7 both cut. **WP6 runs step S0 first** (`plan_tools.py home --dump` of Appendix G is its input): for each (source file, target crate) pair it writes one staging file `<crate root>/📥️staging/<source slug>/🦀️.rs` holding the assigned methods in their original `impl VcsArtifactApp { … }` wrapper plus the free items, and removes the same text from the original, so the old tree keeps compiling (the staging files are mounted by the old parents) and the differential reports an identical multiset before and after S0. Until S0 is announced in the RESULT file of WP6 (`S0 DONE`), WP6 owns every file of `🧵️instance/**`, the ladders and the two feature `🧵️instance` files; afterwards the staging directories of tier 1 and `instance-extension` stay WP6's and the staging directories of the use-case crates, the two feature turns and `instance-app` pass to WP7. WP7 meanwhile starts with what S0 does not touch: the bundle and `X` definitions of 7.6 and 7.7, `builder`, the testing crates, `Plugin::builder` (D6).

#### 12.5.10 Tests

* Production crates keep their tests in-crate where 11.2 says so (put them in `🧪️tests/🦀️.rs` and the language-neutral vectors in `🧪️tests/🧫️fixtures/`); the other tests go to the companions. The skeleton companions are `tests-01` to `tests-21` in the packing of 11.3 (owner = the package of the first host; `tests-01` hosts the tests that name no plugin item, owner WP1). `pwp0-tools/testhosts.py` prints the directory-mapped and mention-mapped host of every test-only file as a starting point; Appendix D stays the budget authority.
* `test-support` is a skeleton. WP3 (it owns the 25 source embeds in the runtime and instance tests' neighbours) implements `pub fn read_module_tree_source(root: &std::path::Path) -> Result<String, String>` and `pub fn read_module_tree_files(root: &std::path::Path) -> Result<Vec<std::path::PathBuf>, String>` (follows `#[path]` mounts and `include!("literal")` of a `.rs` root in declaration order, `🧪️tests` excluded, each file once, parent first); the TS helper `readModuleTreeSource` is the independent oracle. Until then the `include_str!` embeds stay.
* The heap-witness `#[global_allocator]` stays in the root and moves with `tests-10` (WP7).

#### 12.5.11 Toolbox, gates and what is blocked

Tools (`pwp0-tools/`, ticket folder): `closure.ts` (gate counter), `plan_tools.py` (Appendix G, default closure path fixed), `appendix_a_drift.py`, `unitdeps.py` (token evidence of external and first-party edges), `crates.py` (the frozen table; `python3 crates.py` rewrites `🗑️generated/pwp0-crates.json`), `gen_files.py`, `gen_lift.py`, `gen_registry.py` (all idempotent, never overwrite), `liftgroups.py`, `crossmounts.py`, `ownership.py`, `rootunits.py`, `hubsplit.py`, `verify_static.ts`, `verify_hub.py`, `oracle.py` (differential; `--baseline inverse|snapshot`, `--seed`, `--noise`), `reparse.py`, `tree_compare2.ts`, `taxonomy-scope.ts`, `plan_appendix.py`, `testhosts.py`.

* **Census**: `cd 🗑️generated && ONLY=semio-framework-plugin-<key> bun census.ts`. Every skeleton is 1 file and at most 528 bytes; the facade row is 192 files / 6,016,589 bytes / 83,265 lines (185 + 7 hubs). Only `identity`, `test-support` and `tests-01` have a closure inside the gate today; every other crate sits behind the oversized frontier (`semio-framework`, `os-kernel`, `plugin`, …).
* **Differential of phase 0** (`oracle.py hub2`, `rustc +nightly-2026-07-20 --emit=metadata`, sandbox patches of `📓️RESOLUTION-ORACLE.md`, baseline = the same tree with the hubs inlined back): native lib 9,522 vs 9,523 errors, native `--test` 22,057 vs 22,058, `wasm32-wasip2` 9,813 vs 9,814, lib + the four testing features 10,370 vs 10,371; in every configuration the multiset differs by exactly the same three diagnostics, all `E0432` for names that exist only behind the neutralised external globs or the empty `store` stub: `crate::ArtifactOwnedDisposer` x2 is reported at baseline only (its hub is 2), `super::window_config::WindowConfigApply{Cursor,Edit,Mutation}` in one grouped message by the hub tree only (its hub is 1; inlining hub 1 removes it, inlining hub 2 brings the first pair back). Swapping two adjacent mounts of the baseline changes nothing (control), so the shift is specific to `include!`: rustc attributes a failed import chain to a different import when the module is declared in an included file. Every other class (E0412, E0425, E0422, E0423, E0603, E0616, E0624, macro and attribute errors) is identical. The seeded sensitivity runs are caught (a broken `#[path]`: new `couldn't find file`; a removed glob re-export and a renamed module: 51 new, 8,137 old diagnostics). The first run (before the root-resident units were returned) found six real regressions, 12.5.1. Expect the two import chains to settle at the first real compile with the externals present.
* **After the first crate leaves the monolith** `oracle.py` can no longer answer (`extern crate … as …;` becomes an empty module in the sandbox, which turns a real name into an unresolved one); use the per-crate full-resolution differential of 13.2 with the union oracles of the earlier carves.
* **Blocked, not fixed by phase 0**: (1) `bun repo cargo --store=shared --configuration=prototype -- check -p <crate> --lib` did not run a compiler at either end of WP0: at the start (`-p semio-framework-plugin-mounted-owner`, before any edit) it stopped in `cargo fetch --locked` with `cannot update the lock file … because --locked was passed` (the root `Cargo.lock` is behind the manifests of several fleets), at the end (`-p semio-framework-plugin-identity`) with `Finite original maintenance authority required`. Nothing was compiled by cargo and `Cargo.lock` was not refreshed: the 80 new packages and the facade's 58 new edges are missing from it until the first `update --workspace --offline` of the compile window (`📓️COMPILE-WINDOW-CHECKLIST.md`, item 1), which also makes `--locked` builds fail until then. (2) The differential reports resolver-order noise in import chains that already fail (external names such as `crate::store::…` and `ArtifactOwnedDisposer`); see `RESULT-PLUGIN-WP0.md` for the exact residue and the control experiment. (3) The test closure now stubs one non-Rust file that a peer removed (`stubbed 1` in the gate counter). (4) The facade and every class-B crate remain far over the gate (facade: 195 files / 6,018,889 bytes / 83,307 lines); only `identity`, `test-support` and `tests-01` have a closure inside it today.


## 13. Verification and gates

Per crate, before its mount hub entry is removed:
1. `ONLY=<crate> bun census.ts` (or `closure.ts` with `ENTRY=` and `PKG=` set): lib and test rows ≤ 3,500 / 28 / 230,000 on native, `wasm32-wasip2` and `wasm32-unknown-unknown`; a miss of more than 8 percent means re-cut, not squeeze.
2. Full-resolution differential from the crate root (`RESULT-HARDEN-WGPU.md` method: `rustc +nightly-2026-07-20 --emit=metadata`, native, `--cfg test`, `wasm32-wasip2`, with the four feature set; baseline = a sandbox of the old tree with the same external-glob neutralisation): error multisets must be identical after crate-name substitution. A seeded sensitivity check (remove one `pub`, one import) must be caught.
3. Definition scan for fields, methods and items that cross the new boundary, with a seeded regression (`RESULT-RELOCATE-PLUGIN-SDK.md` `visscan.py` method): it finds the hidden widenings (fields and inherent methods the token graph cannot see).
4. `rustfmt --edition 2021 --emit stdout` on every new file; path-literal resolver for all `#[path]`, `include*`; TOML parse; both launch JSON files parse; owner-registry sort.
5. Real compile attempted at the start and end of each package with `bun repo run @semio-tech/plugin-<x>-rs:check --configuration=prototype --parallel=1 -- --lib --offline`; today the gate refuses the old crate (and the manifest triple), so a green real compile is expected only for the leaf crates (`abi`, `identity`, `operation`, `interaction`, `window-config`, `actor-host`) and, after WP0 lands the three-key triple, for everything that has no oversized dependency. Report actual output; do not claim a pass that did not run.
6. Regressions to look for after a carve: a duplicated static, a lost `#[cfg]` on an import, a `pub use` glob that widens a name, trait-method scoping of moved traits, `Deref` scoping, a feature that did not forward, an `extern crate` alias missing in one crate root, a macro `$crate::` path that no longer resolves.

Top level (WP0, phase 2): export parity (facade public paths before = after), `cargo metadata --offline` on both workspaces, the wasm32-wasip2 build of one plugin guest and of one extension guest with a size comparison of the `.wasm` (acceptance: within +2 percent, otherwise add `#[inline]` on the hot cross-crate paths of `reactor-turn`/`runtime-ops`), the turn-retention law (4,437 B per poll turn must not grow), the TS suites that read these sources, and the `🔬️rust-policy-source-evidence` registry guard.

## 14. Reproducing the analysis

Appendix G holds the programs (about 400 lines). In order: `LIST=closure.json bun closure.ts`; `python3 plan_tools.py census` (items, units, SCCs, crate sizes, simulated moves); `python3 plan_tools.py moves` (the table of 5.2); `python3 plan_tools.py home` (the homing totals of 7.3). The crate map and the rank order are the two data structures at the top of the script (`MAP`, `ORDER`); changing a crate boundary means editing `MAP` and re-running. The census uses the repo's own counter (Appendix F). Parameters worth re-running at WP0: the census (section 13.1), and the item-level back-edge listing after each package merges its moves (`plan_tools.py moves` must print nothing for the merged crates).

## 15. Risks and open decisions

### 15.1 Risks

1. **R1 Typeck-unproven base.** The instance and host splits, the relocation slice and the instance-extension change were verified only by the name-resolution oracle; the crate has not compiled since before them. The first real `cargo check` after WP0 will report errors that are not caused by this plan. Mitigation: do the check on the unchanged tree first (`RESULT-PLUGIN-INSTANCE-HOST-SPLIT.md` section 9) and carry its error list as the baseline of the differential.
2. **R2 Orphan rule and `dyn_enum`.** `impl PluginApp for VcsArtifactApp` must be in the composer crate; `#[dyn_enum]` on `trait PluginApp` generates a `macro_rules!` (`__semio_dispatch_PluginApp`) whose expansion in each plugin crate spells the vocabulary that `plugin_app_close_prelude` provides. After the move to `reactor-app` its `$crate` is `reactor-app`. Verify first (WP4, day one) with the zero-variant `dyn_enum_close!` at the end of `⚛️reactor/🔌️app/🦀️.rs` and with one real plugin crate (one small artifact crate is enough); fallback: keep `plugin_app_close_prelude` as a module of `reactor-app` that re-exports the lower crates' names.
3. **R3 2,676 `crate::` rewrites in 114 files and 22 cross-crate `use super::*`.** Mechanical but large; one missed alias is a compile error, never a silent change. The differential catches all of them.
4. **R4 Visibility.** 247 widened units are known; the unknown part (fields, inherent methods) is the definition scan's job; the 49 public methods of the composer are the API contract.
5. **R5 Source-text laws.** 25 embeds and 532 literals; a missed one fails at run time, not at compile time. WP0 generates the list from the tree and each WP re-points its own.
6. **R6 Peer churn.** The tree moved under two earlier plans; the mount hub (12.2 step 3) and the staging directories exist to make moves conflict-free; WP0 re-measures first.
7. **R7 Binary size and speed in guests.** Cross-crate inlining is lost for non-generic non-`#[inline]` functions; the wasm size gate of section 13 and the turn-retention law decide. The `release` profile of guests must keep `lto` and `codegen-units = 1` (check the workspace profile in WP0).
8. **R8 Build critical path.** About 24 layers (19 with Option B of I6) behind the peer-owned `os-kernel` (3.9 MB); a plugin guest does not get faster until `os-kernel` is itself partitioned, only incremental builds do.
9. **R9 `testing-laws` at 214 of 230 KB.** Any growth needs the region cut.
10. **R10 Window-kits file attribution.** The base crate holds `DocumentWindowKit` after the tool's moves (135 lines up to `window-kits-rich`); if WP1 prefers the document kit in the base crate, 135 lines move back and `window-kits-rich` shrinks; no edge changes.
11. **R11 `CARGO_MANIFEST_DIR` is `PKG` today.** 37 uses (1 in the lib: the lease `include!`; 36 in tests) and the `wit_bindgen::generate!({ path: "../../🧬️schema" })` of `abi` resolve relative to the package directory; for every crate whose package sits elsewhere the relative part changes (`abi`: `../../../🧬️schema`). The three `[[test]]` targets (`media-turn`, `owned-output`, `reserved-producer`, each with `required-features`) move with their companion crates.

### 15.2 Decisions requested

* **D1** Use-case bundles and a top composer (this plan) versus keeping the 674 methods as free functions over `&mut VcsArtifactApp`: the brief's god-object rule requires the first; the second needs a composer below the use cases and a newtype for `impl PluginApp` and would change the public type.
* **D2** Phase 2 of the extension inversion (`InstanceExtension::State`, removal of the four `PluginApp` contract methods and the two ladder rungs): recommended as the next ticket; it makes the relocation of `tool-run` and `time-travel` to their products a pure move.
* **D3** Delete the dead `ui_wgpu` dependency in WP0 (the compiler confirms).
* **D4** Companion test crates may depend on production crates through `pub` API only (assumed yes; the alternative blows `window-config`, `decl-app`, `reactor-turn`, `time-travel`).
* **D5** Merge `runtime-instances` into `reactor-lifecycle` (3,225 lines) to save 563 lines of moves, or keep both (assumed keep both).
* **D6** `Plugin::builder` removal (2 in-crate callers).
* **D7** Facade owns 15 of the 18 macros.
* **D8** Option B of I6: a `reactor-tasks` crate so the instance family stays below the reactor (recommended once WP4's definition scan confirms the two statics move cleanly; default is Option A).

**Resolved at WP0 (coordinator):** D1 yes; D2 yes (final work package WP8); D3 yes, already done by a peer; D4 yes; D5 keep separate; D6 yes (WP7); D7 yes; D8 Option B, conditional on WP4's definition scan. See 12.4.


## Appendix A. Production files and their crate (188 files of the union of the lib closures; a row with `;` lists the crates that receive items of the file; lines/KB are the gate's per-file counts; `†` marks new directories)

| file (relative to `PLUG/`) | lines | KB | crate |
|---|---:|---:|---|
| `🚪️io/🛂️authority/📤️result/🦀️.rs` | 17 | 0 | abi |
| `🚪️io/🛂️authority/📤️return/🦀️.rs` | 77 | 10 | abi |
| `🚪️io/🛂️authority/🦀️.rs` | 34 | 8 | abi |
| `🚪️io/🛂️authority/🪶️snapshot/🦀️.rs` | 105 | 17 | abi |
| `🚪️io/🦀️.rs` | 520 | 28 | abi |
| `🧬️schema/🦀️.rs` | 94 | 5 | abi |
| `🧬️schema/🪶️sqlite/🦀️.rs` | 178 | 11 | abi |
| `⚛️reactor/🧵️executor/♻️wake/🦀️.rs` | 42 | 3 | actor-host |
| `⚛️reactor/🧵️executor/🦀️.rs` | 700 | 31 | actor-host |
| `🌐host/📖️body/🦀️.rs` | 59 | 2 | actor-host |
| `🌐host/📮️requests/🦀️.rs` | 552 | 28 | actor-host |
| `🌐host/🦀️.rs` | 522 | 25 | actor-host |
| `📡️backbone/🔗️binding/🦀️.rs` | 208 | 10 | actor-host |
| `🪪️identity/🦀️.rs` | 708 | 30 | identity |
| `🕹️interaction/♻️retirement/🦀️.rs` | 285 | 13 | interaction |
| `🕹️interaction/📃️query/🦀️.rs` | 245 | 10 | interaction |
| `🕹️interaction/📖️capture/🦀️.rs` | 230 | 9 | interaction |
| `🕹️interaction/📡️live/🦀️.rs` | 265 | 12 | interaction |
| `🕹️interaction/🔐️authority/📖️inputs/🦀️.rs` | 81 | 3 | interaction |
| `🕹️interaction/🔐️authority/🦀️.rs` | 45 | 1 | interaction |
| `🕹️interaction/🔺️diff/🦀️.rs` | 97 | 3 | interaction |
| `🕹️interaction/🦀️.rs` | 20 | 0 | interaction |
| `🕹️interaction/🧬️mutations/🔁️set-state/🦀️.rs` | 39 | 2 | interaction |
| `🕹️interaction/🧬️mutations/🦀️.rs` | 65 | 3 | interaction |
| `🕹️interaction/🧭️topology/🦀️.rs` | 408 | 25 | interaction |
| `⏳️operation-progress/🦀️.rs` | 105 | 5 | operation; decl-app 7 |
| `♻️cancellation/🎟️issuer/🦀️.rs` | 214 | 32 | operation |
| `♻️cancellation/📦️lease/🦀️.rs` | 101 | 14 | operation |
| `♻️cancellation/🦀️.rs` | 234 | 10 | operation |
| `🌱️initialization/🏪️catalog/🦀️.rs` | 86 | 10 | operation |
| `📬️completion/♻️effects/🦀️.rs` | 47 | 4 | operation |
| `📬️completion/♻️vacant/🦀️.rs` | 23 | 2 | operation |
| `🪟️window/🎚️config/📥️retained/🦀️.rs` | 1572 | 105 | window-config |
| `🪟️window/🎚️config/🗂️registry/🦀️.rs` | 83 | 5 | window-config |
| `🪟️window/🎚️config/🦀️.rs` | 960 | 74 | window-config |
| `🪟️window/🎚️config/🧬️preparation/🔁️replace/🦀️.rs` | 96 | 6 | window-config |
| `🪟️window/📬️mutation/🔄️refresh/🦀️.rs` | 14 | 2 | window-config |
| `🪟️window/🎛️engagement/🦀️.rs` | 42 | 2 | window-transient |
| `🪟️window/📬️mutation/🦀️.rs` | 38 | 3 | window-transient; window-config 7 |
| `🪟️window/🦀️.rs` | 229 | 13 | window-transient; interaction 5 |
| `🪟️window/🫧️transient/🦀️.rs` | 803 | 50 | window-transient |
| `🖱️context-menu/🦀️.rs` | 274 | 13 | window-kits |
| `🪟️window-kits/🎬️media/🦀️.rs` | 226 | 10 | window-kits |
| `🪟️window-kits/📝️text/🦀️.rs` | 238 | 11 | window-kits |
| `🪟️window-kits/🖼️image/🦀️.rs` | 154 | 9 | window-kits |
| `🪟️window-kits/🦀️.rs` | 760 | 42 | window-kits |
| `🪟️window-kits/🧊️mesh/🦀️.rs` | 54 | 2 | window-kits |
| `🪟️window-kits/🌍️world3d/🦀️.rs` | 653 | 33 | window-kits-rich; window-kits 19 `[WP0 change]` (was 652) |
| `🪟️window-kits/🌳️tree/🦀️.rs` | 922 | 54 | window-kits-rich; window-kits 6 |
| `🪟️window-kits/📃️document/🦀️.rs` | 206 | 11 | window-kits-rich; window-kits 34 |
| `🪟️window-kits/📊️table/🦀️.rs` | 513 | 28 | window-kits-rich |
| `🎚️config/🚫️none/♻️retirement/🦀️.rs` | 21 | 1 | decl-views |
| `👥️presence/♻️retirement/🦀️.rs` | 249 | 13 | decl-views |
| `📜️declaration/⏯️tool-run/🦀️.rs` | 258 | 16 | decl-views; decl-app 36 |
| `📜️declaration/🎞️media/🦀️.rs` | 515 | 39 | decl-views; decl-app 35 |
| `📜️declaration/🎮️command/🦀️.rs` | 191 | 10 | decl-views |
| `📜️declaration/👁️view/🦀️.rs` | 869 | 40 | decl-views; operation 9 |
| `📜️declaration/🛠️tool-machine/🦀️.rs` | 114 | 5 | decl-views |
| `📜️declaration/🪆️child/🦀️.rs` | 675 | 40 | decl-views |
| `📝️draft/🚫️none/♻️retirement/🦀️.rs` | 100 | 5 | decl-views |
| `🫧️transient/♻️retirement/🦀️.rs` | 36 | 2 | decl-views |
| `📜️declaration/📨️emission/🦀️.rs` | 1207 | 78 | decl-emission; decl-app 45 |
| `🧩️composition/📨️emission/📦️preparation/🦀️.rs` | 499 | 37 | decl-emission; composition-emission 174 |
| `🧵️task/🛬️source/🦀️.rs` | 63 | 6 | decl-emission; operation 19 |
| `📜️declaration/👷️job/♻️alias/🦀️.rs` | 28 | 2 | decl-app |
| `📜️declaration/👷️job/🎟️birth/🦀️.rs` | 48 | 5 | decl-app |
| `📜️declaration/👷️job/🦀️.rs` | 1459 | 91 | decl-app; decl-views 235 |
| `📜️declaration/🧩️app/🦀️.rs` | 1162 | 81 | decl-app; document-media 45 |
| `📬️completion/♻️retirement/🚫️refusal/🦀️.rs` | 16 | 1 | decl-app |
| `📬️completion/♻️retirement/🦀️.rs` | 92 | 9 | decl-app; window-transient 2 |
| `📬️completion/⚠️fault/🦀️.rs` | 34 | 6 | decl-app; operation 10 |
| `🏗️builder/🧩️app/🖐️gumball-audience/🦀️.rs` | 21 | 1 | app-builder |
| `🏗️builder/🧩️app/🦀️.rs` | 1572 | 89 | app-builder `[WP0 change]` (was 88 KB) |
| `📜️declaration/📥️natural-file/🦀️.rs` | 336 | 22 | decl-surface |
| `📜️declaration/🪟️surface/🦀️.rs` | 2115 | 126 | decl-surface; decl-emission 24 |
| `🪆️child/🦀️.rs` | 1122 | 69 | child |
| `🧩️composition/♻️root/👁️read/🦀️.rs` | 55 | 6 | composition-emission |
| `🧩️composition/♻️root/🦀️.rs` | 46 | 4 | composition-emission |
| `🧩️composition/📨️emission/🌱️genesis/📄️input/🦀️.rs` | 144 | 11 | composition-emission |
| `🧩️composition/📨️emission/📦️preparation/♻️retirement/🦀️.rs` | 61 | 5 | composition-emission |
| `🧩️composition/🪪️metadata/🦀️.rs` | 274 | 26 | composition-emission |
| `🧩️composition/📬️publication/🤝️group/📦️owner/🦀️.rs` | 703 | 66 | composition-publication |
| `🧩️composition/📬️publication/🤝️group/🦀️.rs` | 114 | 11 | composition-publication |
| `🧩️composition/📬️publication/🤝️group/🪆️child/🦀️.rs` | 168 | 14 | composition-publication |
| `🧩️composition/📬️publication/🦀️.rs` | 405 | 29 | composition-publication |
| `🎞️media/⚠️turn/🦀️.rs` | 58 | 6 | document-media; decl-app 2 |
| `🎞️media/💥️fault/🦀️.rs` | 219 | 12 | document-media |
| `🎞️media/🛫️intrinsic/🦀️.rs` | 138 | 12 | document-media; decl-app 1 |
| `🎞️media/🦀️.rs` | 182 | 17 | document-media |
| `🗃️document/🦀️.rs` | 1861 | 133 | document-media |
| `👷️job/🏛️reserved/⚠️configuration/🦀️.rs` | 45 | 10 | job |
| `👷️job/🏛️reserved/🦀️.rs` | 918 | 57 | job; retained-command 20 |
| `👷️job/📤️outcomes/🦀️.rs` | 178 | 8 | job |
| `👷️job/🦀️.rs` | 932 | 68 | job |
| `👷️job/🧾️bytes/🦀️.rs` | 21 | 1 | job |
| `🎮️command/📥️admission/🦀️.rs` | 312 | 26 | command |
| `🎮️command/🗝️latest/🦀️.rs` | 40 | 4 | command |
| `🎮️command/🦀️.rs` | 607 | 41 | command; time-travel-panel 218 |
| `👥️peer/🦀️.rs` | 550 | 52 | peer |
| `🧩️composition/📬️publication/🤝️group/🪟️mounted/📚️members/🦀️.rs` | 143 | 15 | publication-mounted |
| `🧩️composition/📬️publication/🤝️group/🪟️mounted/📦️owner/🦀️.rs` | 84 | 8 | publication-mounted |
| `🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/👤️member/🦀️.rs` | 181 | 18 | publication-mounted |
| `🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/📄️entry/🦀️.rs` | 119 | 8 | publication-mounted |
| `🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/🦀️.rs` | 134 | 21 | publication-mounted |
| `🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/📚️command/🪟️mounted/🦀️.rs` | 23 | 0 | publication-mounted |
| `🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/📦️group/🦀️.rs` | 207 | 21 | publication-mounted |
| `🧩️composition/📬️publication/🤝️group/🪟️mounted/🧾️receipt/🦀️.rs` | 437 | 40 | publication-mounted |
| `🧩️composition/📬️publication/🤝️group/🪟️mounted/🪪️identity/🦀️.rs` | 79 | 6 | publication-mounted |
| `🧵️retained-command/🎟️admission/🦀️.rs` | 19 | 1 | retained-command |
| `🧵️retained-command/📸️checkpoint/👣️cursor/🦀️.rs` | 35 | 4 | retained-command |
| `🧵️retained-command/🔄️full-operation/🦀️.rs` | 1058 | 105 | retained-command; job 16 |
| `🧵️retained-command/🔗️read/🦀️.rs` | 20 | 1 | retained-command |
| `🧵️retained-command/🦀️.rs` | 928 | 62 | retained-command |
| `🧵️retained-command/🧬️context/📸️fields/🦀️.rs` | 75 | 7 | retained-command; decl-app 9 |
| `🧵️retained-command/🧬️context/🦀️.rs` | 48 | 5 | retained-command; decl-app 13 |
| `🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs` | 89 | 22 | retained-command |
| `⚛️reactor/🔌️app/🦀️.rs` | 538 | 47 | reactor-app |
| `📜️declaration/🌳️tree/🦀️.rs` | 510 | 33 | decl-registry; extension 106 |
| `📜️declaration/📇️registration/🦀️.rs` | 826 | 50 | decl-registry |
| `📜️declaration/🦀️.rs` | 1197 | 62 | decl-registry |
| `⚛️reactor/🪟️surfaces/🦀️.rs` | 191 | 9 | plugin-program; reactor-surfaces 5 |
| `🏗️builder/🔌️program/🦀️.rs` | 299 | 16 | plugin-program; decl-registry 9 |
| `⚛️reactor/📨️pending/🦀️.rs` | 652 | 35 | reactor-lifecycle |
| `⚛️reactor/🚪️lifetime/🦀️.rs` | 503 | 23 | reactor-lifecycle |
| `🚪️lifetime/🦀️.rs` | 90 | 5 | reactor-lifecycle; runtime-instances 6 |
| `⚛️reactor/🏃️runtime/🎞️command/📥️custody/🛂️manifest/🦀️.rs` | 178 | 39 | runtime-ops |
| `⚛️reactor/🏃️runtime/🦀️.rs` | 5465 | 382 | runtime-ops; reactor-lifecycle 918, runtime-exchange 812, runtime-instances 761, extension 392, plugin-program 89 |
| `💡️inference/🚪️gateway/🦀️.rs` | 323 | 38 | extension; plugin-program 90 |
| `💡️inference/🦀️.rs` | 791 | 46 | extension; plugin-program 246, decl-registry 197 |
| `🛂️describe/🦀️.rs` | 318 | 17 | extension |
| `🧩️extension/📨️invoke/⚠️failure/🦀️.rs` | 108 | 19 | extension |
| `🧩️extension/🚪️retirement/🦀️.rs` | 642 | 32 | extension |
| `⚛️reactor/🩹️patches/🦀️.rs` | 1279 | 75 | reactor-patches; reactor-lifecycle 137 |
| `⚛️reactor/📥️cold-pair/🦀️.rs` | 421 | 19 | reactor-surfaces |
| `⚛️reactor/📸️checkpoint/🦀️.rs` | 145 | 9 | reactor-surfaces |
| `⚛️reactor/🦀️.rs` | 1260 | 115 | reactor-core; reactor-wit-bridge 594, reactor-lifecycle 199, reactor-turn 36, runtime-instances 12 |
| `⚛️reactor/💼️jobs/🏛️reserved/🦀️.rs` | 145 | 10 | reactor-jobs |
| `⚛️reactor/💼️jobs/💡️infer/🦀️.rs` | 323 | 24 | reactor-jobs |
| `⚛️reactor/💼️jobs/🔀️migrate/🦀️.rs` | 78 | 5 | reactor-jobs |
| `⚛️reactor/💼️jobs/🦀️.rs` | 692 | 54 | reactor-jobs; actor-host 9 |
| `⚛️reactor/💼️jobs/🧬️mutation-plan/🦀️.rs` | 55 | 4 | reactor-jobs |
| `⚛️reactor/🔄️turn/📥️receiving/📦️completed/⚠️failure/🦀️.rs` | 50 | 7 | reactor-turn |
| `⚛️reactor/🔄️turn/📥️receiving/📦️completed/⚠️fault/🦀️.rs` | 75 | 10 | reactor-turn |
| `⚛️reactor/🔄️turn/📥️receiving/📦️completed/🦀️.rs` | 85 | 15 | reactor-turn; command 2 |
| `⚛️reactor/🔄️turn/📥️receiving/🦀️.rs` | 105 | 18 | reactor-turn; reactor-core 2 |
| `⚛️reactor/🔄️turn/🦀️.rs` | 2251 | 150 | reactor-turn; reactor-core 1 |
| `⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs` | 113 | 33 | reactor-context |
| `⚛️reactor/🎟️context/🌉️call/💼️job/📤️receipt/🦀️.rs` | 95 | 9 | reactor-context |
| `⚛️reactor/🎟️context/🌉️call/📤️failure/🦀️.rs` | 48 | 6 | reactor-context |
| `⚛️reactor/🎟️context/🌉️call/📤️receipt/🦀️.rs` | 30 | 4 | reactor-context |
| `⚛️reactor/🎟️context/🌉️call/📥️input/🦀️.rs` | 51 | 8 | reactor-context |
| `⚛️reactor/🎟️context/🌉️call/🦀️.rs` | 190 | 43 | reactor-context |
| `⚛️reactor/🎟️context/🦀️.rs` | 21 | 1 | reactor-context |
| `⏯️tool-run/🎞️tick/📦️payload/🦀️.rs` | 27 | 1 | tool-run |
| `⏯️tool-run/🎟️publication/🦀️.rs` | 36 | 4 | tool-run |
| `⏯️tool-run/👷️job/🎟️receiving/🦀️.rs` | 8 | 0 | tool-run |
| `⏯️tool-run/🦀️.rs` | 1680 | 101 | tool-run; job 4 |
| `🛠️tool-machine/🖐️gesture/🎟️issuer/🦀️.rs` | 22 | 1 | tool-run |
| `🛠️tool-machine/🦀️.rs` | 331 | 17 | tool-run |
| `⏪️time-travel/♻️retirement/🦀️.rs` | 94 | 11 | time-travel |
| `⏪️time-travel/🎮️decision/🗑️discard/🦀️.rs` | 96 | 17 | time-travel |
| `⏪️time-travel/🦀️.rs` | 3444 | 205 | time-travel; command 1032, time-travel-panel 921, tool-run 4 |
| `🏗️builder/🦀️.rs` | 822 | 47 | builder; plugin-program 6 |
| `♻️lifecycle/🏪️stores/🦀️.rs` | 53 | 5 | instance family (7.4) |
| `🎮️command/🎟️lease/🦀️.rs` | 95 | 10 | instance family (7.4) |
| `🎮️command/📥️ingress/🦀️.rs` | 164 | 23 | instance family (7.4) |
| `🧵️instance/⏳️operation-progress/🦀️.rs` | 50 | 3 | instance family (7.4) |
| `🧵️instance/📨️dispatch/🦀️.rs` | 1621 | 112 | instance family (7.4) |
| `🧵️instance/🔄️full-operation/🦀️.rs` | 60 | 5 | instance family (7.4) |
| `🧵️instance/🔌️plugin-app-impl/🦀️.rs` | 1707 | 125 | instance family (7.4) |
| `🧵️instance/🖼️render/🦀️.rs` | 1113 | 99 | instance family (7.4) |
| `🧵️instance/🚪️ingress/🦀️.rs` | 2263 | 180 | instance family (7.4) |
| `🧵️instance/🛠️tool-machine/🦀️.rs` | 388 | 31 | instance family (7.4) |
| `🧵️instance/🤝️group/🦀️.rs` | 325 | 38 | instance family (7.4) |
| `🧵️instance/🦀️.rs` | 1231 | 85 | instance family (7.4) |
| `🧵️instance/🧩️extensions/🦀️.rs` | 176 | 9 | instance family (7.4) |
| `🧵️instance/🧷️command-routes/🦀️.rs` | 1006 | 78 | instance family (7.4) |
| `🧵️instance/🧾️typed-operations/🦀️.rs` | 1832 | 143 | instance family (7.4) |
| `🪆️child/👁️read/🧪️test-view/🦀️.rs` | 16 | 0 | instance family (7.4) |
| `🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs` | 86 | 9 | instance family (7.4) |
| `🪜️close-ladder/🦀️.rs` | 754 | 53 | instance-ladders |
| `🪜️envelope-ladder/🦀️.rs` | 256 | 17 | instance-ladders |
| `🪜️maintenance-ladder/🦀️.rs` | 1204 | 86 | instance-ladders |
| `⏪️time-travel/🧵️instance/🦀️.rs` | 1449 | 107 | time-travel-turns |
| `⏯️tool-run/🧵️instance/🦀️.rs` | 1194 | 80 | tool-run-turns |
| `📦️packages/🦀️rust/🦀️.rs` | 36 | 1 | facade |
| `🦀️.rs` | 960 | 58 | facade `[WP0 change]` (was 940 / 55; now the root with the hub `include!`s, 12.5) |
| `⚖️authoring-law/🦀️.rs` | 91 | 7 | facade (testing law) |
| `🧪️tests/🧬️generated-test-contracts/🦀️.rs` | 100 | 5 | facade `[WP0 change]` (missing here; unconditional root mount, exports the `__semio_*_fresh_test` macros) |
| `🧵️retained-command/♻️metadata/🦀️.rs` | 8 | 0 | retained-command `[WP0 change]` (missing here) |
| `🪟️window/🎚️config/🧬️preparation/✏️apply/🦀️.rs` | 3 | 0 | window-config `[WP0 change]` (missing here) |

## Appendix D. Lib size versus hosted tests per lower crate (columns are raw item lines/KB; the verdict applies x 1.08 to the lib side for `use` and region lines)

| crate | lib lines | lib KB | hosted test lines | hosted test KB | test files | verdict |
|---|---:|---:|---:|---:|---:|---|
| `abi` | 932 | 74 | 2,215 | 119 | 34 | in-crate |
| `actor-host` | 1,968 | 98 | 571 | 32 | 3 | in-crate |
| `identity` | 671 | 30 | 103 | 7 | 1 | in-crate |
| `interaction` | 1,537 | 79 | 872 | 62 | 4 | in-crate |
| `operation` | 767 | 80 | 94 | 10 | 2 | in-crate |
| `window-config` | 2,600 | 182 | 743 | 60 | 5 | COMPANION |
| `window-transient` | 942 | 61 | 562 | 37 | 6 | in-crate |
| `window-kits` | 1,487 | 78 | 322 | 19 | 6 | in-crate |
| `window-kits-rich` | 2,018 | 113 | 867 | 60 | 2 | in-crate |
| `decl-views` | 2,832 | 182 | 1,847 | 139 | 14 | COMPANION |
| `decl-emission` | 1,457 | 101 | 66 | 8 | 2 | in-crate |
| `decl-app` | 2,414 | 168 | 3,492 | 249 | 7 | COMPANION |
| `app-builder` | 1,464 | 82 | 47 | 2 | 1 | in-crate |
| `decl-surface` | 2,148 | 133 | 79 | 4 | 1 | in-crate |
| `child` | 981 | 62 | 0 | 0 | 0 | in-crate |
| `composition-emission` | 730 | 67 | 0 | 0 | 0 | in-crate |
| `composition-publication` | 1,323 | 117 | 0 | 0 | 0 | in-crate |
| `document-media` | 2,344 | 179 | 311 | 45 | 2 | COMPANION |
| `job` | 1,694 | 120 | 1,354 | 83 | 4 | in-crate |
| `command` | 1,638 | 111 | 112 | 6 | 2 | in-crate |
| `peer` | 510 | 32 | 0 | 0 | 0 | in-crate |
| `publication-mounted` | 1,291 | 139 | 647 | 78 | 4 | in-crate |
| `retained-command` | 1,912 | 143 | 260 | 13 | 2 | in-crate |
| `reactor-app` | 463 | 43 | 0 | 0 | 0 | in-crate |
| `decl-registry` | 2,364 | 139 | 332 | 20 | 4 | in-crate |
| `plugin-program` | 860 | 49 | 419 | 28 | 3 | in-crate |
| `reactor-lifecycle` | 2,446 | 125 | 716 | 36 | 6 | in-crate |
| `runtime-instances` | 779 | 60 | 1,104 | 87 | 3 | in-crate |
| `runtime-ops` | 2,082 | 172 | 356 | 29 | 5 | in-crate |
| `extension` | 1,917 | 149 | 1,118 | 105 | 10 | COMPANION |
| `reactor-patches` | 1,159 | 68 | 1,812 | 111 | 1 | in-crate |
| `reactor-surfaces` | 541 | 26 | 282 | 26 | 3 | in-crate |
| `reactor-core` | 748 | 39 | 288 | 20 | 3 | in-crate |
| `reactor-jobs` | 1,033 | 86 | 904 | 97 | 10 | in-crate |
| `runtime-exchange` | 812 | 52 | 0 | 0 | 0 | in-crate |
| `reactor-turn` | 2,459 | 198 | 2,418 | 154 | 12 | COMPANION |
| `reactor-wit-bridge` | 594 | 49 | 0 | 0 | 0 | in-crate |
| `reactor-context` | 462 | 102 | 924 | 63 | 5 | in-crate |
| `time-travel-panel` | 1,139 | 75 | 0 | 0 | 0 | in-crate |
| `tool-run` | 1,923 | 120 | 238 | 19 | 3 | in-crate |
| `time-travel` | 1,482 | 109 | 3,366 | 375 | 44 | COMPANION |

## Appendix F. `closure.ts` (canonical counter; run with `LIST=closure.json bun closure.ts`; set `PKG` and `ENTRY` to census any other crate)

```ts
import {readFileSync, writeFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {readRustSourceClosureV1} from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";
const ROOT = "/Users/ueli/Documents/semio";
const PKG = process.env.PKG ?? ROOT + "/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust";
const ENTRY = process.env.ENTRY ?? PKG + "/🦀️.rs";
const rustc = execFileSync("rustup", ["which", "rustc"], {encoding: "utf8"}).trim();
const cfgOf = (target?: string) => execFileSync(rustc, ["--print", "cfg", ...(target ? ["--target", target] : [])], {encoding: "utf8"}).trim().split("\n").map(v => v.replace(/=(".*")$/u, (_, q) => "=" + JSON.parse(q)));
let stubbed = 0;
const read = async (p: string): Promise<Uint8Array | null> => {
  try { return readFileSync(p); } catch (e) {
    if ((e as NodeJS.ErrnoException).code === "ENOENT") { if (!p.endsWith(".rs")) { stubbed++; return new Uint8Array(0); } return null; }
    throw e;
  }
};
const feats = ["artifact-app-testing","media-turn-testing","reserved-producer-testing","original-actor-context-testing"].map(f=>"feature="+f);
const rows: [string, string | undefined, string[]][] = [
  ["lib native", undefined, []],
  ["lib wasm32-wasip2", "wasm32-wasip2", []],
  ["lib wasm32-unknown-unknown", "wasm32-unknown-unknown", []],
  ["lib native canvas", undefined, ["feature=canvas-runtime-assets"]],
  ["lib native testing-features", undefined, feats],
  ["test native", undefined, ["test"]],
  ["test wasm32-wasip2", "wasm32-wasip2", ["test"]],
  ["test native all-features", undefined, ["test","feature=canvas-runtime-assets",...feats]],
];
const only = process.env.ROW;
const listOut = process.env.LIST;
const listing: Record<string, unknown[]> = {};
for (const [label, target, extra] of rows) {
  if (only && label !== only) continue;
  stubbed = 0;
  const cfg = [...cfgOf(target), "debug_assertions", ...extra];
  try {
  const inputs = await readRustSourceClosureV1([ENTRY], ROOT, cfg, read, {manifestDirectory: PKG, generated: () => undefined});
  const bytes = inputs.reduce((s, i) => s + i.bytes, 0), lines = inputs.reduce((s, i) => s + i.activeLines, 0);
  const emb = inputs.filter(i => i.kind === "embedded");
  console.log([label, "files", inputs.length, "bytes", bytes, "lines", lines, "embedded", emb.length, "stubbed", stubbed].join("\t"));
  listing[label] = inputs.map(i => ({kind: i.kind, path: (i as any).path ?? (i as any).file ?? "", bytes: i.bytes, lines: i.activeLines}));
  } catch (e) { console.log(label, "ERROR", String((e as Error).message).slice(0,300)); }
}
if (listOut) writeFileSync(listOut, JSON.stringify(listing, null, 1));
```

## Appendix G. `plan_tools.py` (read-only; Python 3.9+, stdlib; `python3 plan_tools.py census|moves|home [--dump]`)

`MAP` is the crate map (file prefix → crate), `item_crate` holds the three file cuts, `ORDER` the rank order of 4.3, `RULES` the field → component patterns of 7.3.

```python
#!/usr/bin/env python3
# plan_tools.py -- reproduces the PLAN-PLUGIN-CRATES measurements. Python 3.9+, stdlib only, read-only on the repo.
#   LIST=closure.json bun closure.ts   (Appendix F: canonical gate counter; writes the per-file listing)
#   python3 plan_tools.py census              # item graph, SCCs, crate sizes
#   python3 plan_tools.py moves               # fixed-order min-cost item moves (section 5.2)
#   python3 plan_tools.py home                # VcsArtifactApp method homing (section 7)
import json, re, sys, collections, os
ROOT = os.environ.get('SEMIO_ROOT', '/Users/ueli/Documents/semio/')
P = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/'
CLOSURE = json.load(open(os.environ.get('CLOSURE', 'closure.json')))   # {"lib native":[{path,bytes,lines}], "lib wasm32-wasip2":[...]}

def strip(t):
    o = []; i = 0; n = len(t)
    while i < n:
        c = t[i]
        if t.startswith('//', i):
            j = t.find('\n', i); i = n if j < 0 else j; continue
        if t.startswith('/*', i):
            d = 1; i += 2
            while i < n and d:
                if t.startswith('/*', i): d += 1; i += 2
                elif t.startswith('*/', i): d -= 1; i += 2
                else:
                    if t[i] == '\n': o.append('\n')
                    i += 1
            continue
        m = re.match(r'(?:br|rb|r|cr)(#*)"', t[i:i + 12]) if c in 'rbc' and (i == 0 or not (t[i - 1].isalnum() or t[i - 1] == '_')) else None
        if m:
            e = '"' + m.group(1); j = t.find(e, i + m.end()); j = n if j < 0 else j + len(e)
            o.append('""' + '\n' * t[i:j].count('\n')); i = j; continue
        if c == '"':
            j = i + 1
            while j < n and t[j] != '"': j += 2 if t[j] == '\\' else 1
            o.append('""' + '\n' * t[i:j].count('\n')); i = j + 1; continue
        if c == "'":
            m = re.match(r"'(?:\\.[^']*|[^\\'])'", t[i:i + 12])
            if m: o.append("''"); i += m.end(); continue
            m = re.match(r"'[A-Za-z_]\w*", t[i:])
            if m: o.append(' '); i += m.end(); continue
        o.append(c); i += 1
    return ''.join(o)

HEAD = re.compile(r'^(?:pub(?:\([^)]*\))?\s+)?(?:(?:async|const|unsafe|extern\s+"[^"]*")\s+)*(fn|struct|enum|union|trait|type|static|const|mod|impl|macro_rules!|thread_local!|use)(?:\b|(?<=!))\s*([A-Za-z_<][\w:<>, ]*)?')
IDENT = re.compile(r'[A-Za-z_]\w*')
GENERIC = set('Cell Inner Node Frame Cursor Source Slot Lane Phase Reserve Admission Vacant Step Ready Pending Closed Open Failed Disabled Enabled Idle Running Done Active new default from into fmt len get set push pop run step next poll drop clone eq hash cmp as_ref self Self super crate std Some None Ok Err Result Option String Vec Box Arc Rc RefCell Mutex HashMap HashSet BTreeMap BTreeSet VecDeque u8 u16 u32 u64 u128 usize i32 i64 bool str f32 f64 mod pub fn let mut const static ref use impl struct enum trait type where for if else match return async await move dyn in as loop while break continue unsafe extern'.split())

def skip_angle(s, i):
    d = 0
    while i < len(s):
        if s[i] == '<': d += 1
        elif s[i] == '>' and not (i > 0 and s[i - 1] == '-'):
            d -= 1
            if d == 0: return i + 1
        i += 1
    return i

def impl_target(head):
    m = re.search(r'\bimpl\s*', head); i = m.end(); s = head
    if i < len(s) and s[i] == '<': i = skip_angle(s, i)
    rest = re.split(r'\bwhere\b|\{', s[i:].strip())[0]
    mm = re.search(r'\bfor\b', rest)
    tr, ty = (rest[:mm.start()].strip(), rest[mm.end():].strip()) if mm else (None, rest)
    tid = IDENT.findall(re.sub(r'<.*', '', re.sub(r'^[&\s]*', '', ty)))
    trid = IDENT.findall(re.sub(r'<.*', '', tr)) if tr else []
    return (tid[-1] if tid else None), (trid[-1] if trid else None)

def outline(path):
    raw = open(path, encoding='utf-8').read(); L = strip(raw).split('\n'); R = raw.split('\n'); n = len(L)
    depth = 0; starts = []
    for l in L: starts.append(depth); depth += l.count('{') - l.count('}')
    ln = 0; items = []
    while ln < n:
        l = L[ln]; m = HEAD.match(l) if starts[ln] == 0 and l and not l[0].isspace() and l[0] not in '}#' else None
        if m:
            d = 0; seen = False; j = ln
            while j < n:
                d += L[j].count('{') - L[j].count('}'); seen = seen or '{' in L[j]
                if (seen and d <= 0) or (not seen and L[j].rstrip().endswith(';')): break
                j += 1
            s = ln
            while s > 0 and R[s - 1].lstrip().startswith(('///', '#[', '//!')): s -= 1
            if m.group(1) != 'use': items.append(dict(s=s + 1, e=j + 1, kind=m.group(1), head=l.strip(), name=(m.group(2) or '').strip()))
            ln = j + 1
        else: ln += 1
    return items, L, R

def tokens(txt):
    out = set()
    for m in IDENT.finditer(txt):
        st = m.start(); w = m.group(0)
        if re.search(r'(?<![A-Za-z0-9_])[A-Z]\w*::$', txt[max(0, st - 48):st]): continue
        pre = txt[max(0, st - 3):st].rstrip()
        if pre.endswith('.') and not pre.endswith('..'): continue
        post = txt[m.end():m.end() + 3].lstrip()
        if post.startswith(':') and not post.startswith('::'): continue
        if w[0].islower() or w[0] == '_':
            if not (post.startswith(('(', '::', '!', '<')) or txt[max(0, st - 2):st].endswith('::')): continue
        out.add(w)
    return out

# ---- crate map (prefix -> crate, longest prefix wins); TOP-* are the instance family, builder, facade root
MAP = {
 '🪟️window/🫧️transient/🔁️document-replacement/': 'TOP-instance', '🎮️command/🎟️lease/': 'TOP-instance', '🎮️command/📥️ingress/': 'TOP-instance', '🪆️child/👁️read/🧪️test-view/': 'TOP-instance',
 '♻️cancellation/': 'operation', '⏳️operation-progress/': 'operation', '🧵️task/': 'operation', '🌱️initialization/': 'operation', '📬️completion/': 'operation',
 '🚪️io/': 'abi', '🧬️schema/': 'abi', '🚪️lifetime/': 'runtime-instances', '🕹️interaction/': 'interaction', '🖱️context-menu/': 'window-kits',
 '🪟️window/🎚️config/': 'window-config', '🪟️window/': 'window-transient',
 '🫧️transient/': 'decl-views', '🎚️config/': 'decl-views', '📝️draft/': 'decl-views', '👥️presence/': 'decl-views',
 '📜️declaration/👁️view/': 'decl-views', '📜️declaration/🪆️child/': 'decl-views', '📜️declaration/🎮️command/': 'decl-views', '📜️declaration/🛠️tool-machine/': 'decl-views', '📜️declaration/⏯️tool-run/': 'decl-views', '📜️declaration/🎞️media/': 'decl-views',
 '📜️declaration/📨️emission/': 'decl-emission', '📜️declaration/👷️job/': 'decl-app', '📜️declaration/🧩️app/': 'decl-app',
 '📜️declaration/🪟️surface/': 'decl-surface', '📜️declaration/📥️natural-file/': 'decl-surface',
 '📜️declaration/📇️registration/': 'decl-registry', '📜️declaration/🌳️tree/': 'decl-registry', '📜️declaration/🦀️.rs': 'decl-registry',
 '🪪️identity/': 'identity', '💡️inference/': 'extension', '🧩️extension/': 'extension', '🛂️describe/': 'extension',
 '🧩️composition/📬️publication/🤝️group/🪟️mounted/': 'publication-mounted', '🧩️composition/📬️publication/': 'composition-publication', '🧩️composition/': 'composition-emission',
 '🪆️child/': 'child', '🗃️document/': 'document-media', '🎞️media/': 'document-media', '🎮️command/': 'command', '👷️job/': 'job', '🧵️retained-command/': 'retained-command', '👥️peer/': 'peer',
 '🌐host/': 'actor-host', '📡️backbone/': 'actor-host', '⚛️reactor/🧵️executor/': 'actor-host',
 '🪟️window-kits/🌳️tree/': 'window-kits-rich', '🪟️window-kits/📊️table/': 'window-kits-rich', '🪟️window-kits/🌍️world3d/': 'window-kits-rich', '🪟️window-kits/': 'window-kits',
 '⏪️time-travel/🧵️instance/': 'TOP-tt', '⏪️time-travel/': 'time-travel', '⏯️tool-run/🧵️instance/': 'TOP-tr', '⏯️tool-run/': 'tool-run', '🛠️tool-machine/': 'tool-run',
 '⚛️reactor/🔌️app/': 'reactor-app', '⚛️reactor/🚪️lifetime/': 'reactor-lifecycle', '⚛️reactor/📨️pending/': 'reactor-lifecycle',
 '⚛️reactor/🪟️surfaces/': 'reactor-surfaces', '⚛️reactor/📥️cold-pair/': 'reactor-surfaces', '⚛️reactor/📸️checkpoint/': 'reactor-surfaces',
 '⚛️reactor/🩹️patches/': 'reactor-patches', '⚛️reactor/💼️jobs/': 'reactor-jobs', '⚛️reactor/🔄️turn/': 'reactor-turn', '⚛️reactor/🎟️context/': 'reactor-context',
 '⚛️reactor/🏃️runtime/': 'runtime-ops', '⚛️reactor/🦀️.rs': 'reactor-core',
 '♻️lifecycle/': 'TOP-instance', '🧵️instance/': 'TOP-instance', '🪜️': 'TOP-ladders',
 '🏗️builder/🧩️app/': 'app-builder', '🏗️builder/🔌️program/': 'plugin-program', '🏗️builder/': 'TOP-builder', '🦀️.rs': 'TOP-root', '⚖️authoring-law/': 'TOP-root', '📦️packages/': 'TOP-root',
}
RT = '⚛️reactor/🏃️runtime/🦀️.rs'; TT = '⏪️time-travel/🦀️.rs'; RC = '⚛️reactor/🦀️.rs'
def file_crate(f):
    best = None
    for k, v in MAP.items():
        if f.startswith(k) and (best is None or len(k) > len(best[0])): best = (k, v)
    return best[1] if best else 'UNMAPPED:' + f
def item_crate(f, s):
    if f == RT:   # function families of the 5,465-line runtime file (start line of the item)
        return ('reactor-lifecycle' if s < 600 else 'runtime-instances' if s < 1881 else 'runtime-ops' if s < 3838 else 'runtime-exchange' if s < 4650
                else 'TOP-root' if s < 5189 else 'extension' if s < 5588 else 'runtime-ops')
    if f == TT:   # regions of the 3,444-line time-travel file
        return 'command' if 1773 <= s <= 2277 else 'time-travel-panel' if (1140 <= s <= 1771 or s >= 2524) else 'time-travel'
    if f == RC and 1236 <= s <= 1829: return 'reactor-wit-bridge'
    return file_crate(f)

# fixed rank order of the lower crates (low -> high), found by the search of section 5.2
ORDER = ["abi", "actor-host", "identity", "interaction", "operation", "window-config", "window-transient", "window-kits", "window-kits-rich", "decl-views", "decl-emission", "decl-app", "app-builder", "decl-surface", "child", "composition-emission", "composition-publication", "document-media", "job", "command", "peer", "publication-mounted", "retained-command", "reactor-app", "decl-registry", "plugin-program", "reactor-lifecycle", "runtime-instances", "runtime-ops", "extension", "reactor-patches", "reactor-surfaces", "reactor-core", "reactor-jobs", "runtime-exchange", "reactor-turn", "reactor-wit-bridge", "reactor-context", "time-travel-panel", "tool-run", "time-travel"]
SUBSYSTEM = {c: s for s, cs in {'F': 'abi actor-host identity interaction operation', 'W': 'window-config window-transient window-kits window-kits-rich', 'D': 'decl-views decl-emission decl-app decl-surface app-builder',
             'O': 'child composition-emission composition-publication publication-mounted document-media job command peer retained-command',
             'R': 'reactor-app decl-registry plugin-program reactor-lifecycle runtime-instances runtime-ops extension reactor-patches reactor-surfaces reactor-core reactor-jobs runtime-exchange reactor-turn reactor-wit-bridge reactor-context',
             'T': 'time-travel-panel tool-run time-travel'}.items() for c in cs.split()}

def tarjan(nodes, adj):
    sys.setrecursionlimit(100000)
    idx = {}; low = {}; st = []; on = set(); out = []; c = [0]
    def sc(v):
        idx[v] = low[v] = c[0]; c[0] += 1; st.append(v); on.add(v)
        for w in adj.get(v, ()):
            if w not in idx: sc(w); low[v] = min(low[v], low[w])
            elif w in on: low[v] = min(low[v], idx[w])
        if low[v] == idx[v]:
            comp = []
            while True:
                w = st.pop(); on.discard(w); comp.append(w)
                if w == v: break
            out.append(comp)
    for v in nodes:
        if v not in idx: sc(v)
    return out

def build():
    files = sorted({x['path'] for k in ('lib native', 'lib wasm32-wasip2') for x in CLOSURE[k] if x['path'].endswith('.rs')})
    items = []; text = {}
    for p in files:
        its, L, R = outline(ROOT + p); f = p.replace(P, ''); text[f] = (L, R)
        for it in its:
            it['file'] = f; k = it['s'] - 1; at = []
            while k < len(R) and R[k].lstrip().startswith(('#[', '///', '//!')): at.append(R[k]); k += 1
            atx = ' '.join(at)
            it['cfgtest'] = any(re.search(r'\btest\b', x) and not re.search(r'not\(\s*test', x) for x in re.findall(r'cfg\((.*?)\)\]', atx)) or '#[test]' in atx
            body = '\n'.join(L[it['s'] - 1:it['e']]); it['toks'] = tokens(body)
            if it['kind'] == 'impl': it['target'], it['trait'] = impl_target(it['head']); it['defname'] = None
            else:
                nm = re.match(r'[A-Za-z_]\w*', it['name']) if it['name'] else None; it['defname'] = nm.group(0) if nm else None
            items.append(it)
    for i, it in enumerate(items): it['id'] = i
    defs = collections.defaultdict(list)
    for it in items:
        d = it['defname']
        if d and d not in ('_', 'tests') and not re.fullmatch(r'[a-z][a-z0-9]{0,12}', d) and it['kind'] not in ('impl', 'use', 'mod') and not it['cfgtest']: defs[d].append(it['id'])
    owner = {}
    for it in items:
        if it['kind'] == 'impl' and it.get('target') and not it.get('trait'):
            d = [j for j in defs.get(it['target'], []) if items[j]['kind'] in ('struct', 'enum', 'union', 'type', 'trait')]
            if len(d) == 1: owner[it['id']] = d[0]
    return files, items, defs, owner

def graph(items, defs, owner):
    unit = lambda i: owner.get(i, i)
    size = collections.Counter(); adj = collections.defaultdict(set); crate_of = {}
    for it in items:
        f = it['file']
        if '/🧪️tests/' in f or f.startswith('🧪️') or it['cfgtest']: continue
        u = unit(it['id']); size[u] += it['e'] - it['s'] + 1
    for u in size: crate_of[u] = item_crate(items[u]['file'], items[u]['s'])
    for it in items:
        u = unit(it['id'])
        if u not in size: continue
        for t in it['toks']:
            if t in GENERIC or t == (it['defname'] or it.get('target')): continue
            ds = defs.get(t)
            if not ds or len(ds) > 3: continue
            for j in ds:
                v = unit(j)
                if v != u and v in size: adj[u].add(v)
    return size, adj, crate_of

def simulate(size, adj, crate_of, items, order=ORDER):
    low = [u for u in size if not crate_of[u].startswith('TOP')]; lows = set(low)
    ladj = {u: {v for v in adj.get(u, ()) if v in lows} for u in low}
    sccs = tarjan(low, ladj); node = {u: k for k, c in enumerate(sccs) for u in c}
    members = collections.defaultdict(list)
    for u in low: members[node[u]].append(u)
    nl = {k: sum(size[u] for u in m) for k, m in members.items()}
    ncr = {}
    for k, m in members.items():
        c = collections.Counter()
        for u in m: c[crate_of[u]] += size[u]
        ncr[k] = c.most_common(1)[0][0]
    rank = {c: i for i, c in enumerate(order)}
    nadj = collections.defaultdict(set); rn = collections.defaultdict(set)
    for u in low:
        for v in ladj[u]:
            a, b = node[u], node[v]
            if a != b: nadj[a].add(b); rn[b].add(a)
    mult = lambda c1, c2: 1.0 if SUBSYSTEM.get(c1) == SUBSYSTEM.get(c2) else 2.0
    moves = []
    while True:
        be = [(a, b) for a, bs in nadj.items() for b in bs if rank[ncr[b]] > rank[ncr[a]]]
        if not be: break
        best = None
        for a, b in be:
            ra, rb = rank[ncr[a]], rank[ncr[b]]
            D = set(); st = [b]
            while st:
                x = st.pop()
                if x in D or rank[ncr[x]] <= ra: continue
                D.add(x); st.extend(nadj.get(x, ()))
            Up = set(); st = [a]
            while st:
                x = st.pop()
                if x in Up or rank[ncr[x]] >= rb: continue
                Up.add(x); st.extend(rn.get(x, ()))
            cd = sum(nl[x] * mult(ncr[x], ncr[a]) for x in D); cu = sum(nl[x] * mult(ncr[x], ncr[b]) for x in Up)
            opt = (cd, 'down', D, ncr[a]) if cd <= cu else (cu, 'up', Up, ncr[b])
            if best is None or opt[0] < best[0]: best = opt
        _, kind, S, dest = best
        for x in S:
            moves.append((nl[x], items[members[x][0]]['defname'] or items[members[x][0]]['head'][:40], ncr[x], dest, kind)); ncr[x] = dest
    final = {u: ncr[node[u]] for u in low}
    return final, moves, sccs

def census():
    files, items, defs, owner = build(); size, adj, crate_of = graph(items, defs, owner)
    print(len(files), 'lib files;', len(items), 'items;', len(size), 'production units;', sum(size.values()), 'unit lines')
    sc = sorted((c for c in tarjan(sorted(size), adj) if len(c) > 1), key=lambda c: -sum(size[x] for x in c))
    print('unit SCCs > 1:', [(len(c), sum(size[x] for x in c)) for c in sc[:8]])
    low = [u for u in size if not crate_of[u].startswith('TOP')]
    final, moves, _ = simulate(size, adj, crate_of, items)
    lines = collections.Counter()
    for u, c in final.items(): lines[c] += size[u]
    print('moves', len(moves), 'lines', sum(m[0] for m in moves))
    for c in ORDER: print(f'{c:24s} {lines[c]:6d}')

def moves_cmd():
    files, items, defs, owner = build(); size, adj, crate_of = graph(items, defs, owner)
    final, moves, _ = simulate(size, adj, crate_of, items)
    agg = collections.defaultdict(list)
    for l, n, f, t, k in moves: agg[(f, t)].append((l, n))
    for (f, t), v in sorted(agg.items(), key=lambda kv: -sum(x[0] for x in kv[1])):
        v.sort(reverse=True); print(f'{sum(x[0] for x in v):5d} {f} -> {t}: ' + ', '.join(f'{n[:36]}({l})' for l, n in v[:6]))

RULES = [(r'^app$|^mounted_policy$', 'core'), (r'^constructor_', 'constructor'),
 (r'^(store|config_store|draft_store|presence_store|transient_store|interaction_store)$|_one_item_factory$|local_root_retirement_factory$|^unsupported_publication_contracts$|^window_config_store$|^window_transient_store$|^retired_window_transient_stores$', 'stores'),
 (r'^close_', 'close'), (r'^maintenance_', 'maintenance'), (r'^peer_|^own_color$|^presence_peer_retirements$|^pending_presence$|^presence_marked_keys$', 'roster'), (r'action_ingress', 'ingress'),
 (r'^(children|private_child_groups|child_|followed_parent_generation|pending_child_pins|composition)', 'children'),
 (r'local_interaction|^interaction_|snapshot_read_return_stage|^config_snapshot_read_returns$', 'interaction'),
 (r'^command_|^log_generation$|^history_|^shell_|^next_command_seq$|^supersedes$|^cache$|^pending_command_prune$', 'history'),
 (r'^pending_reserved$|^reserved_commit|^pending_configuration|^pending_render|^pending_document_window_admission$|^dispatch_report$|^last_emit_wire$', 'pending'),
 (r'^media_|^segmented_|^snapshot_retirements$|^current_media|^pending_media_import', 'media'),
 (r'^envelope_|^store_replacement|^import_staging$|^document_archive|^document_snapshot_read_returns$', 'ingest'),
 (r'^host_event|^delivering_host_event$|^invocation_kind$', 'hostevent'), (r'^pending_transaction', 'txn'),
 (r'^tool_runs$', 'toolrun'), (r'^time_travel$', 'timetravel'), (r'^tool_machines$', 'toolmachine'),
 (r'^typed_|^tool_|^latest_wins|^bounded_tool|^framework_tool|^app_tool|^instance_operation_owner$|^live_runtime_instance_id$|^agent_lane_preview$|^pending_tool_lease|^operation_progress_retired$|^registry$|^test_tool_clock$', 'tools')]
COMP_ORDER = ['core', 'constructor', 'stores', 'roster', 'ingress', 'interaction', 'history', 'children', 'pending', 'tools', 'media', 'ingest', 'hostevent', 'txn', 'toolmachine', 'timetravel', 'toolrun', 'maintenance', 'close']

def home():
    files, items, defs, owner = build()
    va = [it for it in items if it['defname'] == 'VcsArtifactApp'][0]
    body = '\n'.join(text_of(items, va)); inner = body[body.index('{') + 1:body.rindex('}')]
    cur = ''; d = 0; parts = []; prev = ''
    for ch in inner:
        if ch in '<([{': d += 1
        elif ch in '>)]}' and not (ch == '>' and prev in '-='): d -= 1
        prev = ch
        if ch == ',' and d == 0: parts.append(cur); cur = ''
        else: cur += ch
    fields = []
    for p in parts + [cur]:
        m = re.match(r'(?:pub(?:\([^)]*\))?\s+)?(\w+)\s*:', re.sub(r'#\[[^\]]*\]', '', p).strip())
        if m: fields.append(m.group(1))
    comp = {f: next((c for pat, c in RULES if re.search(pat, f)), 'UNASSIGNED') for f in fields}
    methods = []
    for it in items:
        if it['kind'] == 'impl' and it.get('target') == 'VcsArtifactApp':
            seg = text_of(items, it); depth = 0; cur = None
            for k, l in enumerate(seg):
                before = depth
                m = re.match(r'\s*(?:pub(?:\([^)]*\))?\s+)?(?:const\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+(\w+)', l)
                if before == 1 and m:
                    if cur: cur['end'] = k
                    cur = dict(name=m.group(1), start=k, file=it['file'], trait=it.get('trait')); methods.append(cur)
                depth = before + l.count('{') - l.count('}')
                if k == 0 and '{' in l: depth = max(depth, 1)
            if cur: cur['end'] = len(seg)
            for m in methods:
                if 'text' not in m and m['file'] == it['file']: m['text'] = '\n'.join(seg[m['start']:m.get('end', len(seg))])
    fn = set(fields); by = collections.defaultdict(list)
    for m in methods: by[m['name']].append(m)
    uniq = {n: v[0] for n, v in by.items() if len(v) == 1}; rk = {c: i for i, c in enumerate(COMP_ORDER)}; USE = len(COMP_ORDER)
    for m in methods:
        m['lines'] = m['text'].count('\n') + 1; m['fields'] = sorted(set(re.findall(r'\bself\.(\w+)', m['text'])) & fn)
        m['calls'] = sorted(set(re.findall(r'\b(?:self|Self)(?:\.|::)(\w+)\s*(?:\(|::<)', m['text'])))
        cs = {comp[f] for f in m['fields']}; base = max([rk[c] for c in cs], default=-1)
        if len(cs - {'core'}) >= 4: base = USE
        if m.get('trait') == 'PluginApp': base = USE + 1
        m['rank'] = base
    ch = True
    while ch:
        ch = False
        for m in methods:
            for cn in m['calls']:
                t = uniq.get(cn)
                if t and t is not m and t['rank'] > m['rank']: m['rank'] = t['rank']; ch = True
    tot = collections.Counter()
    for m in methods: tot[COMP_ORDER[m['rank']] if 0 <= m['rank'] < USE else ('free' if m['rank'] < 0 else 'usecase' if m['rank'] == USE else 'pluginapp')] += m['lines']
    print(len(fields), 'fields', len(methods), 'methods', sum(tot.values()), 'lines'); print(dict(tot.most_common()))
    dist = collections.Counter(min(len({comp[f] for f in m['fields']}), 4) for m in methods)
    print('components touched per method (0,1,2,3,4+):', [dist[k] for k in range(5)])
    if '--dump' in sys.argv:
        for m in methods:
            r = m['rank']; h = COMP_ORDER[r] if 0 <= r < USE else ('free' if r < 0 else 'usecase' if r == USE else 'pluginapp')
            print('\t'.join([m['file'], m['name'], str(m['lines']), h, ','.join(sorted({comp[f] for f in m['fields']}))]))
def text_of(items, it):
    return globals()['_TEXT'][it['file']][0][it['s'] - 1:it['e']]

if __name__ == '__main__':
    cmd = sys.argv[1] if len(sys.argv) > 1 else 'census'
    if cmd == 'home':
        _files, _items, _d, _o = build()
        _TEXT = {}
        for p in _files: L, R = outline(ROOT + p)[1:]; _TEXT[p.replace(P, '')] = (L, R)
    {'census': census, 'moves': moves_cmd, 'home': home}[cmd]()
```


## Appendix H. Crate registry (frozen by WP0, generated by `pwp0-tools/crates.py`; rank = the topological order, a crate may depend only on lower ranks)

80 crates. `stub` A: the stub is the real root (`[lib] path = "../../🦀️.rs"`); B: the stub is package-local until the carve flips `[lib] path` (12.5.2). `root` is relative to `PLUG/`; the package is `<root>/📦️packages/🦀️rust`. Companion bins list their hosts (11.3).

| rank | crate | kind | WP | root | stub | first-party dependencies | features |
|---:|---|---|---:|---|---|---|---|
| 0 | `test-support` | support | 0 | `🧷️companions/🧯️test-support` | A: `🧯️test-support/🦀️.rs` | - | - |
| 1 | `abi` | lower | 1 | `🚪️io` | B: `📦️packages/🦀️rust/🦀️.rs` | - | - |
| 2 | `actor-host` | lower | 1 | `🌐host` | B: `📦️packages/🦀️rust/🦀️.rs` | - | - |
| 3 | `identity` | lower | 1 | `🪪️identity` | B: `📦️packages/🦀️rust/🦀️.rs` | - | - |
| 4 | `interaction` | lower | 1 | `🕹️interaction` | B: `📦️packages/🦀️rust/🦀️.rs` | - | - |
| 5 | `operation` | lower | 1 | `♻️cancellation` | B: `📦️packages/🦀️rust/🦀️.rs` | `interaction` | `media-turn-testing` |
| 6 | `window-config` | lower | 1 | `🪟️window/🎚️config` | B: `📦️packages/🦀️rust/🦀️.rs` | - | - |
| 7 | `window-transient` | lower | 1 | `🪟️window` | B: `📦️packages/🦀️rust/🦀️.rs` | `interaction`, `window-config` | - |
| 8 | `window-kits` | lower | 1 | `🪟️window-kits` | B: `📦️packages/🦀️rust/🦀️.rs` | `interaction`, `operation`, `window-transient` | `artifact-app-testing` |
| 9 | `window-kits-rich` | lower | 1 | `🪟️window-kits/🧱️rich` | A: `🧱️rich/🦀️.rs` | `interaction`, `window-transient`, `window-kits` | - |
| 10 | `decl-views` | lower | 2 | `📜️declaration/👁️view` | B: `📦️packages/🦀️rust/🦀️.rs` | `interaction`, `operation`, `window-config` | `media-turn-testing` |
| 11 | `decl-emission` | lower | 2 | `📜️declaration/📨️emission` | B: `📦️packages/🦀️rust/🦀️.rs` | `actor-host`, `operation`, `window-config`, `decl-views` | - |
| 12 | `decl-app` | lower | 2 | `📜️declaration/🧩️app` | B: `📦️packages/🦀️rust/🦀️.rs` | `interaction`, `operation`, `window-config`, `window-transient`, `window-kits`, `decl-views`, `decl-emission` | `reserved-producer-testing` |
| 13 | `app-builder` | lower | 2 | `🏗️builder/🧩️app` | B: `📦️packages/🦀️rust/🦀️.rs` | `window-transient`, `decl-views`, `decl-app` | `artifact-app-testing` |
| 14 | `decl-surface` | lower | 2 | `📜️declaration/🪟️surface` | B: `📦️packages/🦀️rust/🦀️.rs` | `abi`, `interaction`, `window-config`, `window-transient`, `window-kits`, `decl-views`, `decl-emission`, `decl-app`, `app-builder` | `artifact-app-testing` |
| 15 | `child` | lower | 3 | `🪆️child` | B: `📦️packages/🦀️rust/🦀️.rs` | `decl-views` | `artifact-app-testing` |
| 16 | `composition-emission` | lower | 3 | `🧩️composition` | A: `🧩️composition/🦀️.rs` | `decl-views`, `decl-emission`, `child` | - |
| 17 | `composition-publication` | lower | 3 | `🧩️composition/📬️publication` | B: `📦️packages/🦀️rust/🦀️.rs` | `window-config`, `window-transient`, `decl-views`, `decl-emission`, `decl-app`, `child`, `composition-emission` | - |
| 18 | `document-media` | lower | 3 | `🗃️document` | B: `📦️packages/🦀️rust/🦀️.rs` | `operation`, `decl-views`, `decl-app`, `child`, `composition-emission` | `artifact-app-testing`, `media-turn-testing` |
| 19 | `job` | lower | 3 | `👷️job` | B: `📦️packages/🦀️rust/🦀️.rs` | `interaction`, `operation`, `window-config`, `window-transient`, `window-kits`, `decl-views`, `decl-emission`, `decl-app` | `artifact-app-testing` |
| 20 | `command` | lower | 3 | `🎮️command` | B: `📦️packages/🦀️rust/🦀️.rs` | `actor-host`, `interaction`, `operation`, `window-transient`, `window-kits`, `window-kits-rich`, `decl-views`, `decl-emission`, `decl-app`, `job` | - |
| 21 | `peer` | lower | 3 | `👥️peer` | B: `📦️packages/🦀️rust/🦀️.rs` | `decl-views`, `decl-app` | `artifact-app-testing` |
| 22 | `publication-mounted` | lower | 3 | `🧩️composition/📬️publication/🤝️group/🪟️mounted` | A: `🪟️mounted/🦀️.rs` | `decl-views`, `child`, `composition-publication` | - |
| 23 | `retained-command` | lower | 3 | `🧵️retained-command` | B: `📦️packages/🦀️rust/🦀️.rs` | `operation`, `window-config`, `window-transient`, `decl-views`, `decl-emission`, `decl-app`, `composition-publication`, `job` | `artifact-app-testing` |
| 24 | `reactor-app` | lower | 4 | `⚛️reactor/🔌️app` | B: `📦️packages/🦀️rust/🦀️.rs` | `operation`, `window-config`, `decl-views`, `decl-emission`, `decl-app`, `composition-publication`, `document-media`, `job`, `peer` | - |
| 25 | `reactor-tasks` | lower | 4 | `⚛️reactor/⏲️tasks` | A: `⏲️tasks/🦀️.rs` | `actor-host`, `decl-emission` | - |
| 26 | `decl-registry` | lower | 2 | `📜️declaration` | B: `📦️packages/🦀️rust/🦀️.rs` | `identity`, `window-kits`, `decl-app`, `app-builder`, `decl-surface`, `reactor-app` | - |
| 27 | `plugin-program` | lower | 2 | `🏗️builder/🔌️program` | B: `📦️packages/🦀️rust/🦀️.rs` | `identity`, `decl-views`, `decl-emission`, `app-builder`, `decl-surface`, `job`, `reactor-app`, `decl-registry` | - |
| 28 | `reactor-lifecycle` | lower | 4 | `⚛️reactor/📨️pending` | B: `📦️packages/🦀️rust/🦀️.rs` | `actor-host`, `interaction`, `job`, `reactor-app`, `plugin-program` | `original-actor-context-testing` |
| 29 | `runtime-instances` | lower | 4 | `⚛️reactor/🏃️runtime/📦️instances` | A: `📦️instances/🦀️.rs` | `job`, `reactor-app`, `decl-registry`, `plugin-program`, `reactor-lifecycle` | - |
| 30 | `runtime-ops` | lower | 4 | `⚛️reactor/🏃️runtime/🧰️ops` | A: `🧰️ops/🦀️.rs` | `abi`, `actor-host`, `operation`, `window-transient`, `decl-views`, `decl-emission`, `decl-app`, `decl-surface`, `document-media`, `job`, `peer`, `reactor-app`, `plugin-program`, `reactor-lifecycle`, `runtime-instances` | `original-actor-context-testing` |
| 31 | `extension` | lower | 3 | `🧩️extension` | A: `🧩️extension/🦀️.rs` | `abi`, `identity`, `decl-surface`, `reactor-app`, `decl-registry`, `plugin-program`, `reactor-lifecycle`, `runtime-instances` | `original-actor-context-testing` |
| 32 | `reactor-patches` | lower | 4 | `⚛️reactor/🩹️patches` | B: `📦️packages/🦀️rust/🦀️.rs` | `reactor-lifecycle` | `original-actor-context-testing` |
| 33 | `reactor-surfaces` | lower | 4 | `⚛️reactor/📥️cold-pair` | B: `📦️packages/🦀️rust/🦀️.rs` | `reactor-app`, `reactor-lifecycle`, `runtime-instances`, `runtime-ops` | - |
| 34 | `reactor-core` | lower | 4 | `⚛️reactor` | B: `📦️packages/🦀️rust/🦀️.rs` | `actor-host`, `decl-emission`, `reactor-app`, `reactor-lifecycle`, `runtime-instances`, `reactor-surfaces`, `reactor-tasks` | `artifact-app-testing` |
| 35 | `reactor-jobs` | lower | 4 | `⚛️reactor/💼️jobs` | B: `📦️packages/🦀️rust/🦀️.rs` | `abi`, `actor-host`, `job`, `decl-registry`, `plugin-program`, `extension` | - |
| 36 | `runtime-exchange` | lower | 4 | `⚛️reactor/🏃️runtime/🔄️exchange` | A: `🔄️exchange/🦀️.rs` | `window-config`, `decl-views`, `decl-emission`, `decl-app`, `job`, `reactor-app`, `plugin-program`, `reactor-lifecycle`, `runtime-instances`, `runtime-ops` | - |
| 37 | `reactor-turn` | lower | 4 | `⚛️reactor/🔄️turn` | B: `📦️packages/🦀️rust/🦀️.rs` | `abi`, `actor-host`, `operation`, `window-kits-rich`, `job`, `command`, `peer`, `reactor-app`, `reactor-lifecycle`, `runtime-instances`, `runtime-ops`, `extension`, `reactor-patches`, `reactor-surfaces`, `reactor-core`, `runtime-exchange`, `reactor-tasks` | - |
| 38 | `reactor-wit-bridge` | lower | 4 | `⚛️reactor/🧬️wit-bridge` | A: `🧬️wit-bridge/🦀️.rs` | `reactor-app`, `reactor-lifecycle`, `reactor-turn` | - |
| 39 | `reactor-context` | lower | 4 | `⚛️reactor/🎟️context` | B: `📦️packages/🦀️rust/🦀️.rs` | `abi`, `reactor-app`, `reactor-lifecycle`, `runtime-ops`, `reactor-jobs`, `reactor-turn` | `original-actor-context-testing` |
| 40 | `time-travel-panel` | lower | 5 | `⏪️time-travel/🪟️panel` | A: `🪟️panel/🦀️.rs` | `interaction`, `window-transient`, `window-kits`, `window-kits-rich`, `decl-views`, `command` | - |
| 41 | `tool-run` | lower | 5 | `⏯️tool-run` | B: `📦️packages/🦀️rust/🦀️.rs` | `interaction`, `window-config`, `window-transient`, `window-kits`, `decl-views`, `decl-emission`, `decl-app` | `artifact-app-testing` |
| 42 | `time-travel` | lower | 5 | `⏪️time-travel` | B: `📦️packages/🦀️rust/🦀️.rs` | `decl-views`, `decl-app`, `child`, `command`, `time-travel-panel`, `tool-run` | - |
| 43 | `instance-stores` | instance | 6 | `🧵️instance/🏪️stores` | A: `🏪️stores/🦀️.rs` | `interaction`, `operation`, `window-config`, `window-transient`, `decl-views`, `decl-emission`, `decl-app`, `child`, `composition-publication`, `job`, `command`, `peer`, `retained-command`, `reactor-app`, `tool-run` | - |
| 44 | `instance-history` | instance | 6 | `🧵️instance/📜️history` | A: `📜️history/🦀️.rs` | `interaction`, `operation`, `window-config`, `window-transient`, `window-kits`, `decl-views`, `decl-emission`, `decl-app`, `child`, `document-media`, `job`, `command`, `publication-mounted`, `retained-command`, `reactor-app`, `reactor-tasks`, `decl-registry`, `tool-run`, `time-travel`, `instance-stores` | - |
| 45 | `instance-children` | instance | 6 | `🧵️instance/🪆️children` | A: `🪆️children/🦀️.rs` | `window-kits-rich`, `decl-views`, `decl-app`, `child`, `command`, `publication-mounted`, `tool-run`, `instance-stores` | - |
| 46 | `instance-jobs` | instance | 6 | `🧵️instance/👷️jobs` | A: `👷️jobs/🦀️.rs` | `interaction`, `operation`, `window-config`, `window-transient`, `window-kits`, `decl-views`, `decl-emission`, `decl-app`, `composition-publication`, `document-media`, `job`, `command`, `retained-command`, `reactor-app`, `tool-run`, `time-travel`, `instance-history`, `instance-children` | - |
| 47 | `instance-docs` | instance | 6 | `🧵️instance/📄️docs` | A: `📄️docs/🦀️.rs` | `interaction`, `operation`, `decl-views`, `decl-emission`, `decl-app`, `child`, `composition-publication`, `document-media`, `job`, `command`, `retained-command`, `instance-jobs` | - |
| 48 | `instance-extension` | instance | 6 | `🧵️instance/🧩️extensions` | B: `📦️packages/🦀️rust/🦀️.rs` | `tool-run`, `time-travel`, `instance-stores`, `instance-history`, `instance-children`, `instance-jobs`, `instance-docs` | - |
| 49 | `instance-commands` | instance | 7 | `🧵️instance/🧷️command-routes` | B: `📦️packages/🦀️rust/🦀️.rs` | `interaction`, `operation`, `window-kits`, `decl-views`, `decl-emission`, `decl-app`, `child`, `composition-publication`, `document-media`, `job`, `command`, `publication-mounted`, `retained-command`, `reactor-app`, `reactor-tasks`, `plugin-program`, `tool-run`, `instance-stores`, `instance-history`, `instance-children`, `instance-jobs`, `instance-docs`, `instance-extension` | `artifact-app-testing` |
| 50 | `instance-typed-ops` | instance | 7 | `🧵️instance/🧾️typed-operations` | B: `📦️packages/🦀️rust/🦀️.rs` | `abi`, `actor-host`, `interaction`, `operation`, `window-config`, `window-transient`, `decl-views`, `decl-emission`, `decl-app`, `composition-publication`, `job`, `command`, `retained-command`, `reactor-app`, `reactor-tasks`, `tool-run`, `instance-stores`, `instance-history`, `instance-children`, `instance-jobs`, `instance-extension` | `artifact-app-testing` |
| 51 | `instance-render` | instance | 7 | `🧵️instance/🖼️render` | B: `📦️packages/🦀️rust/🦀️.rs` | `interaction`, `decl-views`, `decl-emission`, `decl-app`, `job`, `instance-stores`, `instance-children`, `instance-jobs`, `instance-extension` | - |
| 52 | `instance-ladders` | instance | 7 | `🪜️maintenance-ladder` | B: `📦️packages/🦀️rust/🦀️.rs` | `interaction`, `operation`, `window-config`, `window-transient`, `decl-views`, `decl-app`, `child`, `composition-emission`, `composition-publication`, `document-media`, `peer`, `publication-mounted`, `time-travel`, `instance-stores`, `instance-history`, `instance-children`, `instance-jobs`, `instance-docs`, `instance-extension` | - |
| 53 | `time-travel-turns` | instance | 7 | `⏪️time-travel/🧵️instance` | B: `📦️packages/🦀️rust/🦀️.rs` | `interaction`, `decl-views`, `decl-emission`, `decl-app`, `child`, `document-media`, `job`, `command`, `reactor-app`, `time-travel-panel`, `tool-run`, `time-travel`, `instance-stores`, `instance-history`, `instance-children`, `instance-jobs`, `instance-docs`, `instance-extension` | `artifact-app-testing` |
| 54 | `tool-run-turns` | instance | 7 | `⏯️tool-run/🧵️instance` | B: `📦️packages/🦀️rust/🦀️.rs` | `interaction`, `decl-views`, `decl-emission`, `decl-app`, `job`, `command`, `time-travel-panel`, `tool-run`, `time-travel`, `instance-stores`, `instance-children`, `instance-jobs`, `instance-extension` | - |
| 55 | `instance-app` | instance | 7 | `🧵️instance` | B: `📦️packages/🦀️rust/🦀️.rs` | `interaction`, `operation`, `window-config`, `window-transient`, `window-kits`, `decl-views`, `decl-emission`, `decl-app`, `child`, `composition-publication`, `document-media`, `job`, `command`, `peer`, `publication-mounted`, `retained-command`, `reactor-app`, `time-travel-panel`, `tool-run`, `time-travel`, `instance-stores`, `instance-history`, `instance-children`, `instance-jobs`, `instance-docs`, `instance-extension`, `instance-commands`, `instance-typed-ops`, `instance-render`, `instance-ladders`, `time-travel-turns`, `tool-run-turns` | `artifact-app-testing` |
| 56 | `builder` | instance | 7 | `🏗️builder` | B: `📦️packages/🦀️rust/🦀️.rs` | `identity`, `window-kits`, `decl-views`, `decl-emission`, `decl-app`, `app-builder`, `decl-surface`, `job`, `reactor-app`, `decl-registry`, `plugin-program`, `extension`, `instance-app` | `canvas-runtime-assets` |
| 57 | `testing-laws` | instance | 7 | `🧪️testing` | B: `📦️packages/🦀️rust/🦀️.rs` | `decl-registry`, `plugin-program`, `extension`, `builder`, `instance-app` | - |
| 58 | `testing-acceptance` | instance | 7 | `🧪️tests/🧪️history-edit-acceptance` | B: `📦️packages/🦀️rust/🦀️.rs` | `testing-laws` | - |
| 59 | `tests-01` | companion | 1 | `🧷️companions/🍎️tests-01` | A: `🍎️tests-01/🦀️.rs` | 1 crates (hosts none and their closure, `test-support`) | - |
| 60 | `tests-02` | companion | 1 | `🧷️companions/🍐️tests-02` | A: `🍐️tests-02/🦀️.rs` | 2 crates (hosts window-config and their closure, `test-support`) | - |
| 61 | `tests-03` | companion | 2 | `🧷️companions/🍊️tests-03` | A: `🍊️tests-03/🦀️.rs` | 10 crates (hosts decl-app, decl-views and their closure, `test-support`) | - |
| 62 | `tests-04` | companion | 2 | `🧷️companions/🍋️tests-04` | A: `🍋️tests-04/🦀️.rs` | 13 crates (hosts decl-app, document-media and their closure, `test-support`) | - |
| 63 | `tests-05` | companion | 3 | `🧷️companions/🍌️tests-05` | A: `🍌️tests-05/🦀️.rs` | 13 crates (hosts document-media and their closure, `test-support`) | - |
| 64 | `tests-06` | companion | 3 | `🧷️companions/🍉️tests-06` | A: `🍉️tests-06/🦀️.rs` | 35 crates (hosts extension, reactor-turn and their closure, `test-support`) | - |
| 65 | `tests-07` | companion | 4 | `🧷️companions/🍇️tests-07` | A: `🍇️tests-07/🦀️.rs` | 38 crates (hosts reactor-turn, time-travel and their closure, `test-support`) | - |
| 66 | `tests-08` | companion | 5 | `🧷️companions/🍓️tests-08` | A: `🍓️tests-08/🦀️.rs` | 17 crates (hosts time-travel and their closure, `test-support`) | - |
| 67 | `tests-09` | companion | 5 | `🧷️companions/🫐️tests-09` | A: `🫐️tests-09/🦀️.rs` | 17 crates (hosts time-travel and their closure, `test-support`) | - |
| 68 | `tests-10` | companion | 7 | `🧷️companions/🍈️tests-10` | A: `🍈️tests-10/🦀️.rs` | 49 crates (hosts TOP and their closure, `test-support`) | - |
| 69 | `tests-11` | companion | 7 | `🧷️companions/🍒️tests-11` | A: `🍒️tests-11/🦀️.rs` | 49 crates (hosts TOP and their closure, `test-support`) | - |
| 70 | `tests-12` | companion | 7 | `🧷️companions/🍑️tests-12` | A: `🍑️tests-12/🦀️.rs` | 49 crates (hosts TOP and their closure, `test-support`) | - |
| 71 | `tests-13` | companion | 7 | `🧷️companions/🥭️tests-13` | A: `🥭️tests-13/🦀️.rs` | 49 crates (hosts TOP and their closure, `test-support`) | - |
| 72 | `tests-14` | companion | 7 | `🧷️companions/🍍️tests-14` | A: `🍍️tests-14/🦀️.rs` | 49 crates (hosts TOP and their closure, `test-support`) | - |
| 73 | `tests-15` | companion | 7 | `🧷️companions/🥥️tests-15` | A: `🥥️tests-15/🦀️.rs` | 49 crates (hosts TOP and their closure, `test-support`) | - |
| 74 | `tests-16` | companion | 7 | `🧷️companions/🥝️tests-16` | A: `🥝️tests-16/🦀️.rs` | 49 crates (hosts TOP and their closure, `test-support`) | - |
| 75 | `tests-17` | companion | 7 | `🧷️companions/🍅️tests-17` | A: `🍅️tests-17/🦀️.rs` | 49 crates (hosts TOP and their closure, `test-support`) | - |
| 76 | `tests-18` | companion | 7 | `🧷️companions/🥑️tests-18` | A: `🥑️tests-18/🦀️.rs` | 49 crates (hosts TOP and their closure, `test-support`) | - |
| 77 | `tests-19` | companion | 7 | `🧷️companions/🍆️tests-19` | A: `🍆️tests-19/🦀️.rs` | 49 crates (hosts TOP and their closure, `test-support`) | - |
| 78 | `tests-20` | companion | 7 | `🧷️companions/🥔️tests-20` | A: `🥔️tests-20/🦀️.rs` | 49 crates (hosts TOP and their closure, `test-support`) | - |
| 79 | `tests-21` | companion | 7 | `🧷️companions/🥕️tests-21` | A: `🥕️tests-21/🦀️.rs` | 49 crates (hosts TOP and their closure, `test-support`) | - |

## Appendix I. Mount units of the root and their hubs (generated by `pwp0-tools/rootunits.py`; 95 units)

The `module` is the name the monolith knows; carving a crate replaces the unit by `extern crate semio_framework_plugin_<key> as <module>;` (12.5.4). Units marked `root` stay in the root.

| hub | module | target (relative to `PLUG/`) | cfg | region |
|---|---|---|---|---|
| WP1 `🌱️wp1-foundation` | `bounded_initializer_laws` | `🌱️initialization/♻️retirement/🧪️tests/🦀️.rs` | test | 🏛️OwnershipFacet |
| WP1 `🌱️wp1-foundation` | `sqlite_wire` | `🧬️schema/🪶️sqlite/🦀️.rs` | - | 🏛️OwnershipFacet |
| WP1 `🌱️wp1-foundation` | `document_kit` | `🪟️window-kits/📃️document/🦀️.rs` | - | 🪟️WindowFacet |
| WP1 `🌱️wp1-foundation` | `engagement` | `🪟️window/🎛️engagement/🦀️.rs` | - | 🪟️WindowFacet |
| WP1 `🌱️wp1-foundation` | `image_kit` | `🪟️window-kits/🖼️image/🦀️.rs` | - | 🪟️WindowFacet |
| WP1 `🌱️wp1-foundation` | `interaction_topology` | `🕹️interaction/🧭️topology/🦀️.rs` | - | 🪟️WindowFacet |
| WP1 `🌱️wp1-foundation` | `local_interaction` | `🕹️interaction/🦀️.rs` | - | 🪟️WindowFacet |
| WP1 `🌱️wp1-foundation` | `media_kit` | `🪟️window-kits/🎬️media/🦀️.rs` | - | 🪟️WindowFacet |
| WP1 `🌱️wp1-foundation` | `mesh_kit` | `🪟️window-kits/🧊️mesh/🦀️.rs` | - | 🪟️WindowFacet |
| WP1 `🌱️wp1-foundation` | `table_kit` | `🪟️window-kits/📊️table/🦀️.rs` | - | 🪟️WindowFacet |
| WP1 `🌱️wp1-foundation` | `text_kit` | `🪟️window-kits/📝️text/🦀️.rs` | - | 🪟️WindowFacet |
| WP1 `🌱️wp1-foundation` | `tree_kit` | `🪟️window-kits/🌳️tree/🦀️.rs` | - | 🪟️WindowFacet |
| WP1 `🌱️wp1-foundation` | `window_config` | `🪟️window/🎚️config/🦀️.rs` | - | 🪟️WindowFacet |
| WP1 `🌱️wp1-foundation` | `window_kits` | `🪟️window-kits/🦀️.rs` | - | 🪟️WindowFacet |
| WP1 `🌱️wp1-foundation` | `window_mutation` | `🪟️window/📬️mutation/🦀️.rs` | - | 🪟️WindowFacet |
| WP1 `🌱️wp1-foundation` | `window_surface` | `🪟️window/🦀️.rs` | - | 🪟️WindowFacet |
| WP1 `🌱️wp1-foundation` | `world3d_host` | `🪟️window-kits/🌍️world3d/🦀️.rs` | - | 🪟️WindowFacet |
| WP1 `🌱️wp1-foundation` | `artifact_identity` | `🪪️identity/🦀️.rs` | - | 📜️DeclarationFacet |
| WP1 `🌱️wp1-foundation` | `artifact_io` | `🚪️io/🦀️.rs` | - | 📜️DeclarationFacet |
| WP1 `🌱️wp1-foundation` | `completion_fault` | `📬️completion/⚠️fault/🦀️.rs` | - | 📜️DeclarationFacet |
| WP1 `🌱️wp1-foundation` | `completion_refusal_retirement` | `📬️completion/♻️retirement/🚫️refusal/🦀️.rs` | - | 📜️DeclarationFacet |
| WP1 `🌱️wp1-foundation` | `completion_retirement` | `📬️completion/♻️retirement/🦀️.rs` | - | 📜️DeclarationFacet |
| WP1 `🌱️wp1-foundation` | `host` | `🌐host/🦀️.rs` | - | 📜️DeclarationFacet |
| WP1 `🌱️wp1-foundation` | `tool_cancellation` | `♻️cancellation/🦀️.rs` | - | 📜️DeclarationFacet |
| WP1 `🌱️wp1-foundation` | `document_backbone_binding` | `📡️backbone/🔗️binding/🦀️.rs` | - | 📬️PublicationFacet |
| WP1 `🌱️wp1-foundation` | `completion_effects` | `📬️completion/♻️effects/🦀️.rs` | - | 👷️JobsFacet |
| WP1 `🌱️wp1-foundation` | `completion_vacant` | `📬️completion/♻️vacant/🦀️.rs` | - | 👷️JobsFacet |
| WP1 `🌱️wp1-foundation` | `operation_authority` | `🚪️io/🛂️authority/🦀️.rs` | - | 👷️JobsFacet |
| WP2 `📜️wp2-declaration` | `fixture_projection_retirement_tests` | `🧪️tests/🌲️fixture-projection/🦀️.rs` | test | 📄️DeclarationFixtureMutationMount |
| WP2 `📜️wp2-declaration` | `segmented_download_contract` | `🧪️tests/📤️segmented-download/🦀️.rs` | test | 🏛️OwnershipFacet |
| WP2 `📜️wp2-declaration` | `app_builder` | `🏗️builder/🧩️app/🦀️.rs` | - | 📜️DeclarationFacet |
| WP2 `📜️wp2-declaration` | `app_contract` | `📜️declaration/🧩️app/🦀️.rs` | - | 📜️DeclarationFacet |
| WP2 `📜️wp2-declaration` | `app_views` | `📜️declaration/👁️view/🦀️.rs` | - | 📜️DeclarationFacet |
| WP2 `📜️wp2-declaration` | `artifact_declaration` | `📜️declaration/🦀️.rs` | - | 📜️DeclarationFacet |
| WP2 `📜️wp2-declaration` | `artifact_registration` | `📜️declaration/📇️registration/🦀️.rs` | - | 📜️DeclarationFacet |
| WP2 `📜️wp2-declaration` | `child_view` | `📜️declaration/🪆️child/🦀️.rs` | - | 📜️DeclarationFacet |
| WP2 `📜️wp2-declaration` | `gesture_contract` | `📜️declaration/🛠️tool-machine/🦀️.rs` | - | 📜️DeclarationFacet |
| WP2 `📜️wp2-declaration` | `history_view` | `📜️declaration/🎮️command/🦀️.rs` | - | 📜️DeclarationFacet |
| WP2 `📜️wp2-declaration` | `media_contract` | `📜️declaration/🎞️media/🦀️.rs` | - | 📜️DeclarationFacet |
| WP2 `📜️wp2-declaration` | `natural_file_import` | `📜️declaration/📥️natural-file/🦀️.rs` | - | 📜️DeclarationFacet |
| WP2 `📜️wp2-declaration` | `presence_retirement` | `👥️presence/♻️retirement/🦀️.rs` | - | 📜️DeclarationFacet |
| WP2 `📜️wp2-declaration` | `surface_contract` | `📜️declaration/🪟️surface/🦀️.rs` | - | 📜️DeclarationFacet |
| WP2 `📜️wp2-declaration` | `tool_run_contract` | `📜️declaration/⏯️tool-run/🦀️.rs` | - | 📜️DeclarationFacet |
| WP2 `📜️wp2-declaration` | `no_config_retirement` | `🎚️config/🚫️none/♻️retirement/🦀️.rs` | - | 📬️PublicationFacet |
| WP2 `📜️wp2-declaration` | `no_draft_retirement` | `📝️draft/🚫️none/♻️retirement/🦀️.rs` | - | 📬️PublicationFacet |
| WP2 `📜️wp2-declaration` | `transient_retirement` | `🫧️transient/♻️retirement/🦀️.rs` | - | 📬️PublicationFacet |
| WP2 `📜️wp2-declaration` | `plugin_program` | `🏗️builder/🔌️program/🦀️.rs` | - | ⚛️ReactorFacet |
| WP2 `📜️wp2-declaration` | `declarations` | `📜️declaration/🌳️tree/🦀️.rs` | - | 🧵️InstanceFacet |
| WP2 `📜️wp2-declaration` | `natural_file_lifecycle_tests` | `🧪️tests/📄️natural-file-lifecycle/🦀️.rs` | test | 🏛️OwnershipFacet |
| WP3 `👥️wp3-owners` | `artifact_inference` | `💡️inference/🦀️.rs` | - | 📜️DeclarationFacet |
| WP3 `👥️wp3-owners` | `child_emit_preparation` | `🧩️composition/📨️emission/📦️preparation/🦀️.rs` | - | 📜️DeclarationFacet |
| WP3 `👥️wp3-owners` | `owned_context` | `🧵️retained-command/🧬️context/🦀️.rs` | - | 📜️DeclarationFacet |
| WP3 `👥️wp3-owners` | `child_content` | `🪆️child/🦀️.rs` | - | 📬️PublicationFacet |
| WP3 `👥️wp3-owners` | `document_ingress` | `🗃️document/🦀️.rs` | - | 📬️PublicationFacet |
| WP3 `👥️wp3-owners` | `peer_roster` | `👥️peer/🦀️.rs` | - | 📬️PublicationFacet |
| WP3 `👥️wp3-owners` | `private_child_input` | `🧩️composition/📨️emission/🌱️genesis/📄️input/🦀️.rs` | - | 📬️PublicationFacet |
| WP3 `👥️wp3-owners` | `store_publication` | `🧩️composition/📬️publication/🦀️.rs` | - | 📬️PublicationFacet |
| WP3 `👥️wp3-owners` | `command_admission` | `🎮️command/📥️admission/🦀️.rs` | - | 👷️JobsFacet |
| WP3 `👥️wp3-owners` | `command_history` | `🎮️command/🦀️.rs` | - | 👷️JobsFacet |
| WP3 `👥️wp3-owners` | `extension_invocation_failure` | `🧩️extension/📨️invoke/⚠️failure/🦀️.rs` | - | 👷️JobsFacet |
| WP3 `👥️wp3-owners` | `media_export` | `🎞️media/🦀️.rs` | - | 👷️JobsFacet |
| WP3 `👥️wp3-owners` | `media_operation_refusal` | `🎞️media/⚠️turn/🦀️.rs` | - | 👷️JobsFacet |
| WP3 `👥️wp3-owners` | `reserved_job_bytes` | `👷️job/🧾️bytes/🦀️.rs` | - | 👷️JobsFacet |
| WP3 `👥️wp3-owners` | `reserved_job_outcomes` | `👷️job/📤️outcomes/🦀️.rs` | - | 👷️JobsFacet |
| WP3 `👥️wp3-owners` | `reserved_jobs` | `👷️job/🏛️reserved/🦀️.rs` | - | 👷️JobsFacet |
| WP3 `👥️wp3-owners` | `retained_command` | `🧵️retained-command/🦀️.rs` | - | 👷️JobsFacet |
| WP3 `👥️wp3-owners` | `tool_jobs` | `👷️job/🦀️.rs` | - | 👷️JobsFacet |
| WP3 `👥️wp3-owners` | `typed_operation` | `🧵️retained-command/🔄️full-operation/🦀️.rs` | - | 👷️JobsFacet |
| WP3 `👥️wp3-owners` | `describe` | `🛂️describe/🦀️.rs` | - | ⚛️ReactorFacet |
| WP3 `👥️wp3-owners` | `inference_gateway` | `💡️inference/🚪️gateway/🦀️.rs` | - | ⚛️ReactorFacet |
| WP4 `⚛️wp4-reactor` | `cooperative_pump_tests` | `🧪️tests/🤝️cooperative-pump/🦀️.rs` | test | 📄️DeclarationFixtureMutationMount |
| WP4 `⚛️wp4-reactor` | `plugin_runtime_runtime_laws` | `🧪️tests/🏃️plugin-runtime-runtime-laws/🦀️.rs` | test | 📄️DeclarationFixtureMutationMount |
| WP4 `⚛️wp4-reactor` | `plugin_app` | `⚛️reactor/🔌️app/🦀️.rs` | - | ⚛️ReactorFacet |
| WP4 `⚛️wp4-reactor` | `reactor` | `⚛️reactor/🦀️.rs` | - | ⚛️ReactorFacet |
| WP6 `🧱️wp6-instance-tier1` | `action_ingress` | `🎮️command/📥️ingress/🦀️.rs` | - | 🏛️OwnershipFacet |
| WP7 `🧵️wp7-instance-tier2` | `ui_action_laws` | `🧪️tests/🧰️ui-action-collection/🦀️.rs` | any(test, feature = "artifact-app-testing") | 🏛️OwnershipFacet |
| WP7 `🧵️wp7-instance-tier2` | `declaration_fixture_mutations` | `🧪️tests/🛰️declaration-channels/🦀️.rs` | test | 📄️DeclarationFixtureMutationMount |
| WP7 `🧵️wp7-instance-tier2` | `publication_fixture` | `🧪️tests/📢️publication-fixtures/🦀️.rs` | test | 🏛️OwnershipFacet |
| WP7 `🧵️wp7-instance-tier2` | `window_transient_owners` | `🧪️tests/📢️publication-owners/🦀️.rs` | test | 🏛️OwnershipFacet |
| WP7 `🧵️wp7-instance-tier2` | `test_app_mutation_fixture` | `🧪️tests/🖥️test-app-mutations/🦀️.rs` | test | 🏛️OwnershipFacet |
| WP7 `🧵️wp7-instance-tier2` | `contributed_mutation_wire` | `🧪️tests/📡️contributed-mutation-wire/🦀️.rs` | test | 🏛️OwnershipFacet |
| WP7 `🧵️wp7-instance-tier2` | `artifact_instance` | `🧵️instance/🦀️.rs` | - | 🧵️InstanceFacet |
| WP7 `🧵️wp7-instance-tier2` | `builder` | `🏗️builder/🦀️.rs` | - | 🧵️InstanceFacet |
| WP7 `🧵️wp7-instance-tier2` | `app_testing` | `🧪️testing/🦀️.rs` | any(test, feature = "artifact-app-testing") | ⚖️AppLaws |
| root | `macros_wp1_window` | `📣️macros/🌱️wp1-window/🦀️.rs` | - | 📣️FacadeMacroMounts |
| root | `macros_wp2_declaration` | `📣️macros/📜️wp2-declaration/🦀️.rs` | - | 📣️FacadeMacroMounts |
| root | `macros_wp4_reactor` | `📣️macros/⚛️wp4-reactor/🦀️.rs` | - | 📣️FacadeMacroMounts |
| root | `generated_test_contracts` | `🧪️tests/🧬️generated-test-contracts/🦀️.rs` | - | - |
| root | `wasip2` | `🧬️schema/🦀️.rs` | all(target_arch = "wasm32", target_env = "p2") | 🏛️OwnershipFacet |
| root | `window_transient` | `🪟️window/🫧️transient/🦀️.rs` | - | 🪟️WindowFacet |
| root | `app_emission` | `📜️declaration/📨️emission/🦀️.rs` | - | 📜️DeclarationFacet |
| root | `tool_contract` | `📜️declaration/👷️job/🦀️.rs` | - | 📜️DeclarationFacet |
| root | `plugin_runtime` | `⚛️reactor/🏃️runtime/🦀️.rs` | - | ⚛️ReactorFacet |
| root | `derived_artifact_children_tests` | `🧪️tests/🔬️derived-artifact-children/🦀️.rs` | test | - |
| root | `subset_macro_tests` | `🧪️tests/🔬️subset-macro/🦀️.rs` | test | - |

## Appendix J. Lift files of the item moves of 5.2 (generated by `pwp0-tools/liftgroups.py`)

26 cross-package lift files (3,342 lines) have a docstring-only stub in the destination crate's directory (folder `🛬️lifted/<source module>`); the source package fills it (12.5.3 rule 3). the rows for I3 (`editor_surface` and three more items) and I6 (`drive_framework_reserved_registry_job`) are manual additions; I4 is internal to the instance family (WP6).

| lift file (create-if-missing; stub exists) | from WP | to WP | lines | items (largest first) |
|---|---:|---:|---:|---|
| `🎮️command/🛬️lifted/⏪️time-travel/🦀️.rs` (from `⏪️time-travel/🦀️.rs`) | 5 | 3 | 1032 | HistoryPanelText, time_travel_band_section, history_mutation_view_of, impl store::MemberStoreVisitor, time_travel_reprojection_section, +51 more |
| `🧩️extension/🛬️lifted/🏃️runtime/🦀️.rs` (from `⚛️reactor/🏃️runtime/🦀️.rs`) | 4 | 3 | 616 | ExtensionBundle, extension_exports, ExtensionManifest, extension_invoke, foreign_kernel_value_bridge, +18 more |
| `📜️declaration/📨️emission/🛬️lifted/📦️preparation/🦀️.rs` (from `🧩️composition/📨️emission/📦️preparation/🦀️.rs`) | 3 | 2 | 304 | TypedChildEmitPreparation, ChildEmitPreparation, OwnedChildEmit, ChildEmitPreparationOwner, ChildEmitPreparationStep, +5 more |
| `🏗️builder/🔌️program/🛬️lifted/💡️inference/🦀️.rs` (from `💡️inference/🦀️.rs`) | 3 | 2 | 246 | validate_wire_request_resources, WireArtifactInferenceResult, WireArtifactInferenceRequest, validate_inference_execution, validate_wire_request_metadata, +19 more |
| `⏪️time-travel/🪟️panel/🛬️lifted/🎮️command/🦀️.rs` (from `🎮️command/🦀️.rs`) | 3 | 5 | 218 | ui_history_panel |
| `📜️declaration/🛬️lifted/💡️inference/🦀️.rs` (from `💡️inference/🦀️.rs`) | 3 | 2 | 197 | ArtifactInferenceServiceRegistry, ArtifactInferenceService, ArtifactInferenceExecutionError, ArtifactInferenceServiceMetadata, ArtifactInferenceExecutionRequest, +15 more |
| `👷️job/🛬️lifted/🏛️reserved/🦀️.rs` (from `⚛️reactor/💼️jobs/🏛️reserved/🦀️.rs`) | 4 | 3 | 145 | drive_framework_reserved_registry_job, drive_framework_reserved_registry_job_counted |
| `🧩️extension/🛬️lifted/🌳️tree/🦀️.rs` (from `📜️declaration/🌳️tree/🦀️.rs`) | 2 | 3 | 106 | preflight_artifact_declarations, commit_artifact_declarations |
| `🏗️builder/🔌️program/🛬️lifted/🚪️gateway/🦀️.rs` (from `💡️inference/🚪️gateway/🦀️.rs`) | 3 | 2 | 90 | wire_artifact_infer_with_service_cold, wire_artifact_infer_from_context_cold, wire_artifact_infer_cold, wire_artifact_infer_from_cold |
| `🏗️builder/🔌️program/🛬️lifted/🏃️runtime/🦀️.rs` (from `⚛️reactor/🏃️runtime/🦀️.rs`) | 4 | 2 | 89 | wire_artifact_mutation_plan, key_ordered_wire_value, PluginDescriptorExtras, with_extension_inference_context, wire_list_artifact_mutations, +4 more |
| `📜️declaration/🧩️app/🛬️lifted/♻️retirement/🦀️.rs` (from `📬️completion/♻️retirement/🦀️.rs`) | 1 | 2 | 82 | CompletionCell, impl<A:ArtifactApp> Retirement, derived_admit_completion_retirement, CompletionRetirement, CompletionPayload, +9 more |
| `🧵️instance/🛬️lifted/🌳️tree/🦀️.rs` (from `📜️declaration/🌳️tree/🦀️.rs`) | 2 | 7 | 63 | editor_surface, viewer_surface, EditorSurfaceApp, ViewerSurfaceApp |
| `🗃️document/🛬️lifted/🧩️app/🦀️.rs` (from `📜️declaration/🧩️app/🦀️.rs`) | 2 | 3 | 45 | MediaArtifact, whole_document_media, impl semio_framework_job::Orig |
| `📜️declaration/🧩️app/🛬️lifted/⚠️fault/🦀️.rs` (from `📬️completion/⚠️fault/🦀️.rs`) | 1 | 2 | 19 | ArtifactCompletionFault, borrowed_report, borrowed_report_copy_bytes |
| `♻️cancellation/🛬️lifted/🧵️task/🦀️.rs` (from `🧵️task/🛬️source/🦀️.rs`) | 2 | 1 | 19 | impl<M: ,C: ,D: > semio_framew, impl<M: ,C: ,D: > semio_framew, TypedAsyncTaskRun, impl<M: ,C: ,D: > Drop for Asy, COLD_TASK_RETIREMENT_STEPS, +1 more |
| `📜️declaration/🧩️app/🛬️lifted/🧬️context/🦀️.rs` (from `🧵️retained-command/🧬️context/🦀️.rs`) | 3 | 2 | 13 | ArtifactOwnedContextHandle |
| `📜️declaration/🧩️app/🛬️lifted/🚫️refusal/🦀️.rs` (from `📬️completion/♻️retirement/🚫️refusal/🦀️.rs`) | 1 | 2 | 10 | impl<A:ArtifactApp> RetireOwne, impl<A:ArtifactApp> RetireOwne |
| `🌐host/🛬️lifted/💼️jobs/🦀️.rs` (from `⚛️reactor/💼️jobs/🦀️.rs`) | 4 | 1 | 9 | cancel_job |
| `♻️cancellation/🛬️lifted/👁️view/🦀️.rs` (from `📜️declaration/👁️view/🦀️.rs`) | 2 | 1 | 9 | ArtifactOperationProgress, CANCEL_TYPED_OPERATION_ACTION_ID |
| `📜️declaration/🧩️app/🛬️lifted/📸️fields/🦀️.rs` (from `🧵️retained-command/🧬️context/📸️fields/🦀️.rs`) | 3 | 2 | 9 | ContextOwnedFields, ContextToolRunMetadata, ContextToolRunSources, ContextOriginalResidual, ContextRetirementParts |
| `📜️declaration/🧩️app/🛬️lifted/⏳️operation-progress/🦀️.rs` (from `⏳️operation-progress/🦀️.rs`) | 1 | 2 | 7 | cancellation_result_lane |
| `⚛️reactor/📥️cold-pair/🛬️lifted/🪟️surfaces/🦀️.rs` (from `⚛️reactor/🪟️surfaces/🦀️.rs`) | 2 | 4 | 5 | impl Default for SurfaceContex |
| `👷️job/🛬️lifted/⏯️tool-run/🦀️.rs` (from `⏯️tool-run/🦀️.rs`) | 5 | 3 | 4 | is_tool_run_action_id |
| `🎮️command/🛬️lifted/📦️completed/🦀️.rs` (from `⚛️reactor/🔄️turn/📥️receiving/📦️completed/🦀️.rs`) | 4 | 3 | 2 | Inputs |
| `📜️declaration/🧩️app/🛬️lifted/🎞️media-turn/🦀️.rs` (from `🎞️media/⚠️turn/🦀️.rs`) | 3 | 2 | 2 | key_media_error |
| `📜️declaration/🧩️app/🛬️lifted/🛫️intrinsic/🦀️.rs` (from `🎞️media/🛫️intrinsic/🦀️.rs`) | 3 | 2 | 1 | media_projection_error |

The 28 groups below are inside one package (their sum is 1,661 lines); the owner moves them while carving and needs no lift file.

| source file | crate (as cut) -> crate | WP | lines | items |
|---|---|---:|---:|---|
| `⚛️reactor/🏃️runtime/🦀️.rs` | `runtime-instances+runtime-ops` -> `reactor-lifecycle` | 4 | 343 | RuntimeCloseCleanupJob, RuntimeLiveCleanupJob, plugin_begin_instance_close, RuntimeLiveCleanupPump, +8 more |
| `📜️declaration/👷️job/🦀️.rs` | `decl-app` -> `decl-views` | 2 | 235 | ArtifactInstanceOperationOwnerHandle, InstanceInferenceSessions, ArtifactInferenceSessionAccess, ArtifactInstanceOperationOwner, +5 more |
| `⚛️reactor/🦀️.rs` | `reactor-core` -> `reactor-lifecycle` | 4 | 199 | ReactorFixedSlots, reserve_reactor_close, ReactorCloseRegistry, ReactorCloseState, +13 more |
| `⚛️reactor/🪟️surfaces/🦀️.rs` | `reactor-surfaces` -> `plugin-program` | 2 | 180 | SurfaceContexts, SurfaceRole, SurfaceBinding, synthetic_window_surface, +2 more |
| `⚛️reactor/🩹️patches/🦀️.rs` | `reactor-patches` -> `reactor-lifecycle` | 4 | 137 | MountedTreeTerminal, PatchTrackerState, ReadySlot, SurfaceSlot, +7 more |
| `🪟️window-kits/📃️document/🦀️.rs` | `window-kits` -> `window-kits-rich` | 1 | 135 | DocumentWindowKit, impl WindowKit for DocumentWin |
| `🚪️lifetime/🦀️.rs` | `runtime-instances` -> `reactor-lifecycle` | 4 | 78 | PluginInstanceCloseLease |
| `📜️declaration/📨️emission/🦀️.rs` | `decl-emission` -> `decl-app` | 2 | 45 | EphemeralEmit, impl<A: ArtifactApp> Default f |
| `🧵️task/🛬️source/🦀️.rs` | `operation` -> `decl-emission` | 2 | 42 | dispose_cold_task_run, AsyncTaskRun, impl<T:AsyncTaskSource<M,C,D>,, ColdTaskRun, +4 more |
| `⚛️reactor/🦀️.rs` | `reactor-core` -> `reactor-turn` | 4 | 36 | step_reactor_close |
| `📜️declaration/⏯️tool-run/🦀️.rs` | `decl-views` -> `decl-app` | 2 | 36 | ToolRunJobRequest |
| `📜️declaration/🎞️media/🦀️.rs` | `decl-views` -> `decl-app` | 2 | 35 | ArtifactMediaExportRequestRetirement, ArtifactMediaExportJobRequest, impl<A:ArtifactApp> Drop for A |
| `📜️declaration/🪟️surface/🦀️.rs` | `decl-surface` -> `decl-emission` | 2 | 24 | ViewEmit |
| `👷️job/🏛️reserved/🦀️.rs` | `job` -> `retained-command` | 3 | 20 | FRAMEWORK_RESERVED_ROUTE_CAPACITY |
| `🪟️window-kits/🌍️world3d/🦀️.rs` | `window-kits-rich` -> `window-kits` | 1 | 19 | WorldSunConfig, world3d_scene, world3d_environment_json |
| `⏪️time-travel/🦀️.rs` | `time-travel-panel` -> `time-travel` | 5 | 17 | time_travel_selection_value |
| `🧵️retained-command/🔄️full-operation/🦀️.rs` | `retained-command` -> `job` | 3 | 16 | TypedCommandFullOperationJobFactory |
| `⚛️reactor/🦀️.rs` | `reactor-core` -> `runtime-instances` | 4 | 12 | take_original_instance_metadata_app_id |
| `🪟️window/📬️mutation/🔄️refresh/🦀️.rs` | `window-transient` -> `window-config` | 1 | 12 | refresh_transfer, WindowAuthorityRefreshStep, refresh_demands, WindowRefreshSnapshot |
| `🏗️builder/🔌️program/🦀️.rs` | `plugin-program` -> `decl-registry` | 2 | 9 | AppFactory |
| `🪟️window/📬️mutation/🦀️.rs` | `window-transient` -> `window-config` | 1 | 7 | ErasedWindowMutationValue |
| `🪟️window-kits/🌳️tree/🦀️.rs` | `window-kits-rich` -> `window-kits` | 1 | 6 | tree_item |
| `🪟️window/🦀️.rs` | `window-transient` -> `interaction` | 1 | 5 | ui_assembly_error, UiAssemblyResult |
| `⏪️time-travel/🦀️.rs` | `time-travel` -> `time-travel-panel` | 5 | 4 | TIME_TRAVEL_ENTITY_NAME_VISITS, TIME_TRAVEL_SHORT_ID_CHARS |
| `⏪️time-travel/🦀️.rs` | `command` -> `tool-run` | 5 | 4 | time_travel_arg_generation |
| `⚛️reactor/🔄️turn/📥️receiving/🦀️.rs` | `reactor-turn` -> `reactor-core` | 4 | 2 | response_key_empty, intent_key_empty |
| `📬️completion/♻️retirement/🦀️.rs` | `operation` -> `window-transient` | 1 | 2 | EphemeralParts |
| `⚛️reactor/🔄️turn/🦀️.rs` | `reactor-turn` -> `reactor-core` | 4 | 1 | original_receiving_sources_empty |

## Appendix K. Mount lines in a foreign parent file (generated by `pwp0-tools/crossmounts.py`; 15 edges)

The root's own mounts are in Appendix I. For the edges below the child's owner edits the one named line of the parent's file when the child leaves (12.5.3 rule 2a); an `include!` edge disappears when the last item of the included file has been lifted.

| parent file (owner WP) | child file (owner WP) | mount | who edits the parent line |
|---|---|---|---|
| `⏪️time-travel/🦀️.rs` (WP5) | `⏪️time-travel/🧵️instance/🦀️.rs` (WP7) | `instance_turns` | WP7 (child owner), one line, after re-reading |
| `⏯️tool-run/🦀️.rs` (WP5) | `⏯️tool-run/🧵️instance/🦀️.rs` (WP7) | `instance_turns` | WP7 (child owner), one line, after re-reading |
| `⚛️reactor/🏃️runtime/🦀️.rs` (WP4) | `🧩️extension/🚪️retirement/🦀️.rs` (WP3) | `extension_retirement` | WP3 (child owner), one line, after re-reading |
| `⚛️reactor/🦀️.rs` (WP4) | `⚛️reactor/🧵️executor/🦀️.rs` (WP1) | `executor` | WP1 (child owner), one line, after re-reading |
| `⚛️reactor/🦀️.rs` (WP4) | `⚛️reactor/🪟️surfaces/🦀️.rs` (WP2) | `surface_context` | WP2 (child owner), one line, after re-reading |
| `👷️job/🦀️.rs` (WP3) | `⏳️operation-progress/🦀️.rs` (WP1) | `operation_progress` | WP1 (child owner), one line, after re-reading |
| `👷️job/🦀️.rs` (WP3) | `⏯️tool-run/🦀️.rs` (WP5) | `tool_run` | WP5 (child owner), one line, after re-reading |
| `👷️job/🦀️.rs` (WP3) | `🛠️tool-machine/🦀️.rs` (WP5) | `tool_machine` | WP5 (child owner), one line, after re-reading |
| `👷️job/🦀️.rs` (WP3) | `⏪️time-travel/🦀️.rs` (WP5) | `time_travel` | WP5 (child owner), one line, after re-reading |
| `📜️declaration/🧩️app/🦀️.rs` (WP2) | `🎞️media/🛫️intrinsic/🦀️.rs` (WP3) | `include!` | WP3 (child owner), one line, after re-reading |
| `📜️declaration/🧩️app/🦀️.rs` (WP2) | `🎞️media/💥️fault/🦀️.rs` (WP3) | `include!` | WP3 (child owner), one line, after re-reading |
| `📜️declaration/🪆️child/🦀️.rs` (WP2) | `🧩️composition/♻️root/👁️read/🦀️.rs` (WP3) | `include!` | WP3 (child owner), one line, after re-reading |
| `🗃️document/🦀️.rs` (WP3) | `🌱️initialization/🏪️catalog/🦀️.rs` (WP1) | `original_catalog_constructor` | WP1 (child owner), one line, after re-reading |
| `🧵️instance/🦀️.rs` (WP7) | `🪆️child/👁️read/🧪️test-view/🦀️.rs` (WP6) | `child_frame_scene_view` | WP6 (child owner), one line, after re-reading |
| `🧵️instance/🦀️.rs` (WP7) | `🧵️instance/🧩️extensions/🦀️.rs` (WP6) | `extensions` | WP6 (child owner), one line, after re-reading |
