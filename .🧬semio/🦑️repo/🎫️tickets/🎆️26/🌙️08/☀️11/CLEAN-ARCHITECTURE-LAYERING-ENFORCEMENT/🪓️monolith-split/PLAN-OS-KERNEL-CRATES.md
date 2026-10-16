# 🪓️ Plan: Crate Partition of `semio-framework-os-kernel`

Planner: Sonnet 5.5, read-only, foreground, no sub-agents. Consumers: wave-4 executors. This is a plan: nothing was edited, compiled or committed. Every number below was measured on the live working tree on 2026-10-15 (about 17:30 to 19:30 CEST) with the canonical counter `readRustSourceClosureV1` (a copy of `rwp0-tools/closure.ts` that takes `FEATURES=a,b` for the `feature=` cfgs) and a token-level reference graph; section 12 lists the exact method so any executor can re-measure in minutes.

Notation. `K/` = `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/` (the package directory; lib root `K/🦀️.rs`, 331 lines, manifest `K/Cargo.toml`). `M/` = `🧰️framework/🛍️products/💻️os/🔨️modules/`. `S/` = `M/🏪️store/`. `F/` = `🧰️framework/🔨️modules/`. A crate named `x` below is `semio-framework-x` (for example `os-store-ledger`). "Lines" are the gate's active lines (the cfg-selected text split on newlines; blank, comment and doc lines count). "WP" is a work package (section 10). "Facade" is the surviving `semio-framework-os-kernel` package.

## 0. Verdict in fourteen lines

1. Today the kernel library closure is **180 files / 3,676,487 bytes / 54,677 lines** with default features (`deflate`), **182 files / 3,994,566 B / 59,414 lines** with `sync,ureq,worker`, 179 files / 3,920,933 B / 55,898 lines on `wasm32-unknown-unknown`, and **175 files / 3,583,426 B / 52,474 lines on `wasm32-wasip2`** (the plugin guests link the kernel; `sync`, `worker` and the extension package format drop out there). The test closures are **579 files / 9,131,580 B / 104,813 lines** (native, 236 embedded fixture files) and 568 files / 8,965,353 B / 97,690 lines (wasm32). The task statement's 150 to 180 files / 54.8k lines is the default-feature lib closure.
2. The binding limits are **lines and bytes**: 59,414 / 3,500 means at least 17 lib crates, 3,994,566 B / 230,000 B at least 18, and the test closure needs at least 30. This is a tree dominated by the Store (45,787 lines and 3.03 MB of the 59,414 lines hang under `S/🦀️.rs`; the root file alone is 11,227 lines / 756 KB, and one inherent `impl ArtifactStore` is 6,093 lines).
3. The plan proposes **48 new library crates plus the thinned facade, and 18 companion test crates: 67 packages**. Every library crate is at or below 3,500 lines / 28 files / 230,000 bytes for its lib closure (largest: `os-store-sync-native` 2,735 lines, `os-store-engine` about 2,600 plus about 600 delegate lines, `os-store-owned-schema-decode` 2,531; section 6.1) and, with its unit tests, for its test closure except where section 9 moves the tests to a companion.
4. At **module (file) level** the reference graph has one strongly connected component of 28 files / 27,155 lines, a second of 10 files / 5,515 lines and four small ones (7, 2, 2 and 2 files; 236 to 1,083 lines). At **item level** (2,511 column-0 items, 2,866 nodes once `ArtifactStore` is expanded to its methods) there is exactly one large knot: `ArtifactStore` with its inherent and trait impl blocks, `DocumentStoreOwners`, `ArtifactStoreCloseView`/`ArtifactStoreOwnedDisposer`, the member-group impl and three durable-group staging functions: 24 items, 7,628 lines, 6,093 of them the one impl. Every other item cycle is below 1,100 lines. File boundaries are simply the wrong cut surface; the tree is acyclic once about 6,000 lines of small items move (section 5.2) and `ArtifactStore` is decomposed.
5. After those cuts the 48 library crates form a **DAG with 0 residual back edges**, depth 13 (longest chain `engine-owners` -> `engine-core` -> `engine-live` -> `engine-stage` -> `engine` -> its dependents). Three ports are needed (section 5.3): the store-close port, one capability for authority minting, and the shared `StoreParts` borrow bundle; none is an extension trait.
6. **`ArtifactStore` is the god object** (section 7): 46 fields, **353 methods / 5,780 lines in four inherent impl blocks of the same file**, 152 `pub`. It is decomposed into six component structs (`StoreLog`, `StoreOwners`, `StoreWire`, `StoreCursor` in `os-store-engine-core`; `StoreLive`, `StoreReplay` in `os-store-engine-live`) and a composer `ArtifactStore` in `os-store-engine`. Methods are homed by field ownership and call direction with the algorithm of 7.4: 297 methods end in the two component crates (163 without any foreign parameter, 93 with one, 41 with two or more), 56 use-case methods (1,857 lines) stay in the composer; the composer also carries 125 thin delegates for the `pub` methods that moved down.
7. **Statics and macros** (6.5, 6.6): 22 `static`s and one `thread_local!` in the lib closure, 8 of them the codec/IO registries (all land in `os-store-codec-registry`, one instance each); 14 `#[macro_export]` macros of which the 6 that name facade paths (`space_members!`, `field_set_mutations!`, `config_record!`, `config_diff!`, `field_set_mutations_binary!`, `_text!`) stay in the facade and the 8 self-contained ones travel with their crate.
8. **Targets** (8.3): `wasm32-wasip2` guests link 42 of the 48 crates; `os-store-sync`, `-sync-native`, `-sync-wasm` and `os-extension` are absent there, `os-pack-tools` and `os-spr-cli` are native only. Feature unification can still switch `sync` on in a wasip2 graph (`🌉️mcp` and the wgpu renderer request it), so the facade must keep the target-gated optional dependency form it has today.
9. **Public paths stay** (section 8): 2,857 Rust files and 1,933 manifest dependency rows (793 of them dev) name the kernel; consumers use the owning module path `os_store::<module>::X` (E1b), `io`, `os_vcs`, `sqlite_snapshot`, `os_dsl`, `os_directory` and four package aliases (`protocol` 1,047 files, `store` 844, `dsl` 718, `vcs` 124). The facade keeps every one through explicit `pub use` (no globs) generated from the old surface; no Rust consumer edit is planned. About 40 files that name `S/🦀️.rs` by path (23 TS/JSON scanners, schemas and fixtures, 15 Rust path references, 1 manifest) must be re-pointed.
10. **Tests are the larger cost** (section 9): 194 test-only source files / 44,720 lines / 3.29 MB plus 203 embedded fixtures / 1.85 MB. The store-root-mounted tests are 22,720 lines / 2.08 MB in 67 files, of which one file (`S/🧪️tests/🔬️unit/🦀️.rs`) is 12,625 lines / 1,006 KB: they need **10 companion crates** (bytes bind, not lines) plus one shared fixtures crate; one 1,126 KB JSON fixture in the DSL grammar tests must be read at run time.
11. The kernel is **peer-owned and moving**: a fleet already cut `os-store-{envelope,command,cursor,projection,message-ledger,one-item,ephemeral,blob-identity,link-schema}`, `os-history-*`, `os-vcs`, `os-pack`, `os-command`, `os-channel`, `os-directory(-client)`, `os-identity`, `os-inference`; the member-hydration batch (generic `C` on `RetainedPersistedDocumentHydration`) and the store-producer capability work are still in flight in files this plan assigns to `os-store-document`, `os-store-member-open-store` and `os-store-engine-owners` (section 1.4).
12. Parallelism: WP0 (serial, first: skeletons, registries, the lossless root splitter S0) then WP1..WP8 on **disjoint source files**, then WP0 again (facade, TS readers, locks, gates). WP5 (the seven engine crates) is the critical path and a single executor.
13. Nothing here could be compiled: the compiler-ingress gate refuses the kernel closure (it is over every ceiling). Verification is the full-resolution differential per crate root plus census rows, definition/privacy scans and rustfmt (section 11); the first real `cargo check` of the leaf crates is expected to be possible, the engine crates are not until the closure below them passes the gate.
14. Decisions requested (section 13): the `ArtifactStoreOneItemLiveAuthority::mint` capability (open since E1b), `StoreParts` as the only bundle, the `engine` size spill valve, and whether companion test crates may use `pub` API only.

## 1. Baseline, budgets, what binds

### 1.1 Closure measurements (gate counter, host cfg `aarch64-apple-darwin` + `debug_assertions`)

| Closure | cfg / features | files | bytes | active lines | embedded files |
|---|---|---:|---:|---:|---:|
| lib | native, `deflate` (default) | 180 | 3,676,487 | 54,677 | 33 |
| lib | native, `deflate,sync,ureq,worker` | 182 | 3,994,566 | 59,414 | 33 |
| lib | native, all features (`+protocol-laws,record-codec-laws,durable-group-testing,mutation-testing`) | 183 | 4,062,252 | 60,737 | 33 |
| lib | `wasm32-unknown-unknown`, `deflate,sync,ureq,worker` | 179 | 3,920,933 | 55,898 | 33 |
| lib | `wasm32-wasip2` (same features; `sync`/`worker`/`extension` compiled out) | 175 | 3,583,426 | 52,474 | 33 |
| lib + tests | native, `deflate,sync,ureq,worker` | 579 | 9,131,580 | 104,813 | 236 |
| lib + tests | `wasm32-unknown-unknown`, same features | 568 | 8,965,353 | 97,690 | 234 |

The tree parser of section 3 reproduces these file sets to within 1 to 3 files per row. All other numbers use the `deflate,sync,ureq,worker` native lib closure (the union of what the 150 manifests enable), 182 files, 149 of them `.rs` and 33 embedded data files (50,290 bytes: 15 KB TS deployment schema of the plugin registry, SQL/JSON fixtures, the 15 KB of store-root JSON/SQL).

### 1.2 Per-crate budget

At most **3,500 active lines, 28 files, 230,000 bytes** for the lib closure **and** for the test closure (`cargo test` units are gated too). Hard ceiling is 4,000 / 32 / 262,144; the declared boundary of every new crate is the ceiling (recipe: exactly the three keys), the 3,500 / 28 / 230,000 target is enforced by the census. The kernel manifest today declares 12,000 / 128 / 1,048,576, which the facade manifest drops to the ceiling once it is thin. Embedded `include_str!` data counts as files and bytes; source-text readers (`include_str!("….rs")`) must read at run time (section 9). Keep every repo-relative path at most 256 characters and 240 UTF-8 bytes; the new nodes below add at most 3 segments.

### 1.3 What else binds a new crate (read from the repo, not assumed)

* **OS crate-boundary corpus** (`K/../../🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json` and `🧬️cuts.json`, test `🧪️tests/📦️crate-boundaries/🟦️.ts`). Each producer row names `{package, manifest, entry, maximumActiveLines, forbiddenDependencies}`; the test checks that both TOML parsers agree, that all six launch commands (`bun repo run @semio-tech/<ID>-rs:{build,check,test} --configuration={prototype,release} --parallel=1 -- --lib --offline`) exist in `.vscode/launch.json`, that `[lib] path` resolves to `entry`, that no forbidden dependency is declared, the source caps for four cfg sets, and that **no source token of any selected file is** `semio_framework_os_kernel`, `semio_framework_plugin`, `os_store`, `os_spr` or `os_directory`. The `cuts` rows pin, per cut, a `canonical` re-export line in the consumer file and a `definition` line that must exist in the producer and be absent from the consumer. Consequence: every new crate gets a corpus row; every `crate::os_spr::…` (808 occurrences in the lib closure), `crate::os_vcs` (171), `crate::os_pack` (146), `crate::sqlite_snapshot` (99), `crate::io_schema` (79), `crate::os_directory` (65), `crate::list_delta` (62) and `crate::os_store::…` path is rewritten to the owning external crate (section 6.3). The cuts file is also the place that records "the facade re-exports X from crate Y".
* **Taxonomy** (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`): directory emoji must be registered, `packagesDirName` `📦️packages`, `libWiringLineBudget` **150** (lib-root wiring lines), `⚙️engine` is a globally valid leaf parent, `cargoDependencyDirections` forbids framework-role crates from depending on `s-module/plugin/extension/hub` (all 48 new crates are role `framework`, which is how the kernel is declared). Nodes marked "new" in 6.1 are proposals for WP0 to validate with `rwp0-tools/taxonomy-scope.ts`.
* **Recipe** (`PRODUCER-CRATE-RECIPE.md`): `<node>/🦀️.rs` lib root beside the package dir, `[lib] path = "../../🦀️.rs"`, `workspace` depth (OS crates point at the repo root with as many `..` as the package path has segments: 8 for `S/🫧️ephemeral/📦️packages/🦀️rust`, 11 for the renderer's `🏛️host-base`), `📋️project.json`, `📋️build-owners.json` row, six launch routes in `.vscode/launch.json` **and** `.vscode/🧩️launch.seed.jsonc`, a member row (byte-sorted) in the **root** `Cargo.toml` (that is where `os-store-ephemeral` and the other OS producers are listed), the root `Cargo.lock`, a `[profile.wasm-prototype.package.<crate>]` row where the old crate had one (the kernel has none; the neural-engine rows are separate crates).
* **Compiler ingress** refuses raw cargo ("Finite original maintenance authority required"), so no executor may compile around it.

### 1.4 What is in flight (do not collide)

* The Codex fleet (ticket `26/10/13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES`) and the CAD/renderer waves edit these files continuously: `S/🦀️.rs`, `S/🔗️read`, `S/🔁️replay`, `S/📒️ledger`, `S/🧬️schema`, `S/📦️codec`, `S/🚪️io`, `S/🧩️composition`, `S/🧾️document`, `S/🎮️one-item`, `S/👥️presence`. Their cuts that are **done and excluded from every number here**: `os-store-{envelope,command,cursor,projection,message-ledger,one-item,ephemeral,blob-identity,link-schema}`, `os-presence-native-snapshot`, `value-snapshot-read`, `pack-json-retained-schema`, `os-history-*`, `os-vcs`, `os-pack`, `os-command`, `os-channel`, `os-directory(-client)`, `os-identity`, `os-inference`. The new crates sit above them. Do not recreate them; the existing `os-store-one-item` (footprint/ceilings) is why the publication protocol crate is named `os-store-one-item-publication`.
* **Member hydration batch** (`member-hydration-next-owned-source-plan.md`, 2026-10-15): `RetainedPersistedDocumentHydration<P,M>` gains the selected producer `C`, `MemberStoreProducer` grows the decode/publication ports, the plugin archive gets an app-associated producer. Files: `S/🧾️document/📜️history/💧️hydration`, `S/🧩️composition/🚪️open/🏭️operation`, `S/🏗️initialization/🏠️owners/🏭️member`. WP6 (crates `os-store-document`, `os-store-member-open-store`) and WP4 (`os-store-engine-owners`) must start from the tree after that batch lands or coordinate the rename in the same change; this plan treats those three files as "peer-hot: re-read before every edit".
* **Store producer capabilities** (`store-producer-current-readonly.md`): `ArtifactStore::new<C: ArtifactStorePublication<P>>`/`construct<C>` already take the explicit producer; `create_member_store`/`open_member_store` still select `ArtifactSnapshotPublication`. Keep that seam exactly as it is when moving `new`/`construct` into the composer.
* `K/🦀️.rs`, `K/Cargo.toml` and `K/📜️script.ts` (226 KB) are edited by several waves; WP0 owns them.
* The source guards in `S/🏗️initialization/🏠️owners/🏭️member/📥️source` and the root `📜️script.ts` readers `policyReadOsStoreSource` / `OS_STORE_ROOT_SOURCES` pin the store sources by text; section 8.5 lists them.

## 2. Inventory

### 2.1 Crate layout and mounts

* `K/Cargo.toml` (`semio-framework-os-kernel`, lib name `semio_framework_os_kernel`, path `🦀️.rs`, 68 nominal dependency keys, features `default=[deflate]`, `deflate`, `sync`, `ureq`, `worker`, `typegen`, `native-bin`, `protocol-laws`, `record-codec-laws`, `mutation-testing`, `durable-group-testing`; `[[bin]]` `pack`, `spr`, `semio` behind `native-bin`; `[[test]]` `sqlite_snapshot_native_admission` and `artifact_command_retirement`).
* `K/🦀️.rs` (331 lines, 13 KB): a clippy `allow` list, a `#[global_allocator]` test allocation observer (`cfg(test)`), seven `extern crate self as …` aliases (`dsl`, `dsl_grammar`, `semio_format`, `semio_framework_os_kernel`, `spr`, `store`, `vcs`) plus `wasm_bindgen_futures` for browser wasm32, and `#[path]` mounts grouped in inline modules: `os_dsl` (component, 7 family files, grammar, lsp, registry native-only, fixture sweep tests), `os_pack` (control and cli native-only, re-exports of `pack`), `list_delta`, `os_spr` (component, cli native-only, io, materialize, protocol laws under `cfg(any(test, feature="protocol-laws"))`, re-exports of `protocol` and the extracted crates), `os_plugin_module_schema`, `native_catalog`, `io`, `os_store` (component = `S/🦀️.rs`, `sync` under `feature="sync"` and not wasip2, `worker` under `feature="worker"` and wasm32-not-p2), `os_semio`, `os_extension` (not wasip2), and the glob re-exports `pub use crate::os_dsl::*; os_inference::*; os_pack::*; os_spr::*; os_store::test_support; os_store::*; os_vcs::*`.
* `S/🦀️.rs` (11,227 lines, 311 column-0 items of which 77 are module items): the Store root; 59 file-backed child modules (18 `pub`) plus the inline `pub mod` `mounted_pack_rt` and `test_support` and the `pub` mounts `store_schema`, `sync` and `worker` (module list in 6.4).
* cfg gates on mounts: `not(target_arch="wasm32")` (spr cli, pack cli and control, dsl registry, sync native actor and folder storage: 3,172 item lines), `all(target_arch="wasm32", not(target_env="p2"))` (sync wasm actor, backbone web-sys: 944 lines), `all(feature="sync", not(all(wasm32,p2)))` (the whole sync module and `F/🏃️process/🧭️routing/🏠️workspace`), `not(all(wasm32,p2))` (extension), `feature="durable-group-testing"` (167 lines), `all(not(wasm32), feature="native-bin")` (72 lines of the semio bootstrap), `test` (115 items / 664 lines in lib files).

### 2.2 Families (native lib closure with `sync`; tests = test-only source files and embedded fixtures that no lib file mounts)

| family | lib files | lib lines | lib KB | test files | test lines | test KB (incl. embedded) |
|---|---:|---:|---:|---:|---:|---:|
| store root | 1 | 11227 | 756 | 0 | 0 | 0 |
| store composition (open, durable-group, coordinator) | 21 | 9166 | 570 | 14 | 4110 | 452 |
| store sync | 1 | 4726 | 310 | 8 | 4243 | 276 |
| store codec (owned-schema, registry, snapshot capability) | 8 | 4791 | 305 | 7 | 1317 | 103 |
| kernel io | 5 | 2644 | 193 | 4 | 602 | 88 |
| store io (text format, binary, serialization) | 11 | 1971 | 126 | 0 | 0 | 2 |
| store replay | 5 | 1929 | 120 | 2 | 416 | 35 |
| store document (hydration, mounted pack) | 3 | 1914 | 117 | 1 | 506 | 42 |
| store read | 3 | 1406 | 112 | 2 | 209 | 47 |
| store initialization | 8 | 1774 | 108 | 2 | 406 | 26 |
| store canonical-edit | 6 | 1849 | 108 | 7 | 1680 | 143 |
| spr | 4 | 1917 | 100 | 21 | 3738 | 204 |
| store snapshot-clone | 7 | 1159 | 99 | 3 | 265 | 33 |
| store one-item batch (paged, owners, prepared, ephemeral) | 6 | 1410 | 89 | 3 | 444 | 35 |
| dsl | 12 | 1522 | 85 | 32 | 3431 | 1365 |
| store one-item publication | 2 | 1208 | 82 | 1 | 28 | 9 |
| store space-history sqlite | 10 | 943 | 81 | 2 | 746 | 170 |
| store config | 1 | 1049 | 66 | 0 | 0 | 0 |
| store ledger | 4 | 861 | 63 | 2 | 157 | 30 |
| store operation-wire | 4 | 726 | 57 | 1 | 290 | 37 |
| list-delta | 1 | 903 | 49 | 1 | 355 | 14 |
| store child | 2 | 784 | 41 | 1 | 230 | 14 |
| store backbone | 2 | 705 | 38 | 3 | 280 | 20 |
| pack tools | 2 | 674 | 37 | 2 | 181 | 16 |
| semio container | 1 | 422 | 31 | 2 | 155 | 24 |
| store space schema | 9 | 528 | 28 | 7 | 137 | 8 |
| store retirement | 2 | 235 | 17 | 2 | 466 | 47 |
| extension | 1 | 294 | 13 | 1 | 126 | 6 |
| kernel lib root | 1 | 309 | 13 | 0 | 0 | 0 |
| plugin registry | 2 | 154 | 11 | 2 | 26 | 16 |
| space cells | 1 | 152 | 9 | 2 | 150 | 12 |
| framework-mounted (io text, process routing, value/pack/replication tests) | 3 | 62 | 3 | 4 | 157 | 68 |
| store other 📬️command | 0 | 0 | 0 | 1 | 48 | 9 |
| store other 📏️supersession-index | 0 | 0 | 0 | 1 | 16 | 4 |
| store presence tests | 0 | 0 | 0 | 1 | 130 | 8 |
| store tests (root-mounted) | 0 | 0 | 0 | 30 | 18873 | 1593 |
| vcs tests | 0 | 0 | 0 | 1 | 33 | 2 |
| store testing fixtures | 0 | 0 | 0 | 20 | 750 | 37 |
| other | 0 | 0 | 0 | 1 | 19 | 3 |
| **total** | **149** | **59,414** | **3,852** | **194** | **44,720** | **5,017** |

The per-file table (149 files with lines, bytes and target crate) is Appendix A.

### 2.3 Cross-cutting facts that shape the plan

* **Imports.** Most children import explicitly (`use super::X`, `use crate::os_store::…`); only 16 of 149 files use `super::*` globs. That is why the item graph is clean enough to cut: the cycles are real name references, not glob soup.
* **`crate::os_*` aliases** are the kernel's own facade paths: `crate::os_spr::*` is the replication crate `protocol` (lib name of `semio-framework-replication`) plus `os-command`, `os-history`, `os-channel` and the 206-line spr component; `crate::os_vcs` is `semio-framework-os-vcs`; `crate::os_pack` is `pack`/`os-pack`; `crate::io_schema`, `crate::sqlite_snapshot` are `semio-framework-io-schema` and `-io-sqlite-snapshot`. Table 6.3 is the rewrite.
* **Derive-emitted paths.** The `dsl_derive` proc macro (`M/🗣️dsl/✨️derive`, 1,449 lines) emits 32 `::semio_framework_os_kernel::…` paths (`os_store::BorrowedRecordSpecProducer`, `os_pack::pack_rt`, `ProtocolError`, `CompositeMutationKind`, …). Kernel-internal users: `os-store-backbone` (4 `#[dsl(…)]` attributes, 2 derives), `os-store-durable-group-wire` (6 / 2), `os-store-durable-group` (1), the config and list-delta macros. Below the facade `::semio_framework_os_kernel` does not exist, so WP1 gives the derive a path-root attribute (default unchanged for the 2,857 external files).
* **Public-by-accident widening.** E1b measured 830 `pub(crate)` widenings (395 of them fields) across the decomposed Store files; after the crate split every one that crosses a crate becomes `pub`. 6.2 and 13 name the ones that must stay sealed.
* **Statics:** 22 `static` items and one `thread_local!` (6.5). **Macros:** 14 `#[macro_export]` (6.6). **Embedded assets:** 33 in the lib closure, all small (largest 15 KB); the test closure embeds 203 more (1.85 MB, one of them 1,126 KB).
* **Files mounted from outside `M/`:** `F/🚪️io/📝️text/{🦀️.rs,💡️inferences}` (the `io::text` and `inferences` modules, 52 lines) and `F/🏃️process/🧭️routing/🏠️workspace/🦀️.rs` (10 lines, mounted by sync) are framework files; they stay where they are and are mounted by exactly one new crate (`os-io` and `os-store-sync`). Keep the `#[path]` form; do not copy.
* **Consumers** (8.4): 1,933 dependency rows in 150 manifests plus the `✏️s` plugin tree.

## 3. Method (what the numbers mean)

1. **Closure**: `readRustSourceClosureV1([K/🦀️.rs], ROOT, cfg, read, {manifestDirectory: K, generated: () => undefined})` with `cfg = rustc --print cfg [--target T]`, `debug_assertions`, the `feature=` cfgs and optionally `test`; results in 1.1.
2. **Module tree**: regex mount parser over `#[path]`, plain `mod`, `include!`/`include_str!`, with the attached `#[cfg]` chain and inline-module directory rules (`#[path="."]` resets), evaluated for native, wasm32-unknown-unknown and wasm32-wasip2 (574 nodes with tests, 184 lib files including embedded data).
3. **Item graph**: the 149 lib `.rs` files give 2,511 column-0 items (56,650 of the 59,414 lines; the rest are `use` lines, attributes and inter-item comments). A token counts as a reference to an item only if that name is defined by exactly one item (a name with several definitions resolves only to a same-file definition), is not preceded by `.`, is not a lowercase single word (`rows`, `schema`, `pair`, …) and is not in a short ignore list (`Cursor`, `MAGIC`, `HEADER_SIZE`, `REC_*`, `Text`, `Field`, `Phase`, `Frame`, …). Inherent impls are bound to their type; trait impls to the higher-ranked of trait and type crate (orphan rule). Two false-edge families were found and removed by hand: variant names equal to type names (`ArtifactStoreCursorDisposerPhase::Backbone`) and external-crate constants that share a name with the spr cli (`HEADER_SIZE`, `REC_DOC`, `REC_STR_DICT`, `REC_COMMIT`, `MAGIC`).
4. **`ArtifactStore` expanded**: the struct (119 lines, 46 fields) becomes six component nodes plus a composer node, and its four inherent impl blocks 353 method nodes with their text, their `self.field` accesses (to components) and their `self.m()` / `Self::m()` calls (to method nodes): 2,866 nodes, 6,059 edges. The external pokers into private fields (about 95 accesses in durable-group, 12 in hydration, config and text) were found by a receiver scan (7.6).
5. **Partition**: crates are file lists plus item moves. An automatic resolver treats every back reference to a higher-ranked item whose down-closure is at most 130 lines as a move down into the crate of its lowest user (iterated to a fixed point in three passes over the crate order of 4.3), normalises impl placement (an inherent impl follows its type; a trait impl follows the higher-ranked of trait and type; 22 impls moved), and reports what is left. Starting from 83 back-edge targets (heuristic order of the file-level crates) it ends at 0.
6. **Tests**: each test source file is attached to its mounting lib file (host) and, separately, to the crate whose items it names most (at least 60 lines or 15 percent of its references); roots-mounted tests use the second rule (section 9).
7. **Limits of this evidence** (carry into verification): references through method syntax (outside `ArtifactStore`), trait dispatch, blanket impls, macros and `Deref` are invisible; the sizes are plus or minus 8 percent (items exclude `use` lines; headers are redistributed proportionally); re-measure every crate with the census before and after each carve.

## 4. Layers and the crate DAG

### 4.1 Rules the partition obeys

1. **Lines and bytes decide the cut, cohesion decides the shape.** A crate is a domain (one Store subsystem, one DSL, one registry), never a size bucket. A domain larger than the budget is split along its item-level DAG: `owned-schema` (3,730 lines), `durable-group` (3,540), `sync` (4,726) and the `ArtifactStore` impl (6,093) are the four cases.
2. **Dependencies point down only.** The rank order of 4.3 is the topological order of the crate DAG; a reference to a higher-ranked crate is a defect and would appear in 5.2.
3. **A type and every inherent impl of it share a crate** (E0116); a trait impl lives in the crate of the trait or the type (E0117), in practice the higher-ranked of the two. The resolver enforces both (22 impls moved to their type's crate).
4. **Statics live in the lowest crate that names them** (6.5), so a crate boundary never creates a second instance.
5. **Exported macros that name facade paths stay in the facade** (6.6); self-contained macros travel with their crate and get `$crate` paths rewritten.
6. **No extension trait splits an impl.** The three ports of 5.3 are inversion points with one implementor each.
7. **Whole-crate target gating:** native-only crates (`os-pack-tools`, `os-spr-cli`, `os-store-sync-native`) and wasm-only crates (`os-store-sync-wasm`) are gated by target-specific dependency tables in their consumer; nothing below the sync crates may depend on `tokio`, `ureq` or `web-sys`.

### 4.2 Layer sketch (low to high; `o-` = `os-`, `s-` = `os-store-`; longest dependency chain to the crate's lowest dependency)

```
L0  o-list-delta  o-semio  o-space-sqlite-cells  o-plugin-registry-schema  s-space-schema  s-child  s-ledger  s-canonical-json
L1  o-extension  s-retirement  s-replay  s-backbone  s-one-item-publication  s-owned-schema-fields
L2  o-pack-tools  s-read  s-operation-wire  s-snapshot-clone
L3  o-spr  o-dsl  s-text-io  s-one-item-batch  s-owned-schema-decode
L4  s-member-open  s-space-history  s-initialization-runtime  s-durable-group-wire  o-spr-cli
L5  s-engine-owners
L6  s-codec-registry  s-engine-core
L7  o-io  s-engine-live
L8  s-engine-stage
L9  s-engine
L10 s-initialization-replay  s-space-member  s-document  s-durable-group  s-sync
L11 s-codec-factory  s-member-open-store  s-config  s-composition  s-sync-native  s-sync-wasm
L12 s-space
L13 s-test-support
```

The authoritative edge list is the `direct deps` column of 6.1 (a direct-dependency edge exists wherever an item of the crate names an item of another; the facade's edges are not counted). The critical path (build parallelism, not edit parallelism) is `s-ledger` -> `s-read` -> `s-engine-owners` -> `s-engine-core` -> `s-engine-live` -> `s-engine-stage` -> `s-engine` -> `s-space-member` / `s-document` -> `s-space` -> `s-test-support`.

### 4.3 Rank order (the order used for back-edge accounting)

`list-delta, semio-container, space-cells, plugin-registry-schema, space-schema, extension, pack-tools, spr, dsl, child, ledger, retirement-backing, replay, read, backbone, canonical-json, one-item-publication, operation-wire, text-io, snapshot-clone, one-item-batch, owned-schema-fields, owned-schema-decode, member-open, engine-owners, codec-registry, io, space-history, initialization-runtime, durable-group-wire, engine-core, engine-live, engine-stage, engine, initialization-replay, codec-factory, space-member, document, member-open-store, config, durable-group, composition, space, test-support, sync, sync-native, sync-wasm, spr-cli, facade` (short ids as in 6.1, prefix `os-`/`os-store-` omitted).

## 5. Cycle cuts

### 5.1 What the cycles are

* **File level.** One SCC of 28 files / 27,155 lines: the store root, `owned-schema` (3,730 lines), `durable-group` (3,540), `composition/open` (12 files), `hydration`, `canonical-edit` (+ source, borrowed), `operation-wire`, `one-item publication`, `snapshot-clone` (+ metadata, config-apply), `initialization/owners`, `prepared/retirement`, `io/binary/genesis` and `io/binary/operation-identity`. A second SCC of 10 files / 5,515 lines: `ledger` (+ retirement), `read`, `replay` (+ operation), `backbone`, `io` (root + text), `history/planning`, `snapshot-capability/workspace-receiving`. Small ones: the seven `schema/mutations` files (236 lines), `spr/cli` with the spr component (850), `composition` with `graph/group` (1,083), `canonical-edit/value` with its admission (375). Everything else (`dsl`, `semio`, `extension`, `list-delta`, `pack`, `space`, `plugin`, `child`, `config`, `document` root) is acyclic. In the first, looser graph (without the generic-word filter of 3) the big SCC was 91 files / 42,885 lines: most of that was name collision.
* **Item level.** 295 item SCCs larger than one item; **one real knot**: `ArtifactStore` (struct 119 lines, impls 6,093 + 12 + 10 + 72 + 52 + the member-group impl 149 + `SpaceMember for ArtifactStore` 478), `DocumentStoreOwners` (+ 5 items, 328-line impl), `ArtifactStoreCloseView`, `ArtifactStoreOwnedDisposer`, `config_store_from_initialized_runtime_with_owners`, and the durable-group staging functions `stage_prebuilt_batch_root`, `abort_staged_store_member`, `adopt_staged_store_member`: 24 items, 7,628 lines. The next knots are all single-file state machines that fit one crate: `RetainedPersistedDocumentHydration` (6 items, 1,100 lines), `RetainedConfigStoreHydration` (988), the one-item sealer cluster (32 items, 857), the durable-group map/commit family (18 items, 855), `ArtifactStoreReplayInitializer` (755). **Everything else is acyclic once items move.**

### 5.2 Item moves and designed splits (apply before or while carving)

Executors extract an item with its doc comment, attributes and `//#region` markers intact (regions that span a cut are repeated and balanced on both sides). A moved item keeps its name; the old path disappears (no aliases). Items are named, not line-numbered, because peers edit these files; use the outline listing of section 12 to find them.

**Designed splits (one source file, several crates).** The machine assignment of every item is Appendix C; summary:

| source | split | why |
|---|---|---|
| `S/🦀️.rs` (11,227 lines) | into 12 crates + the facade: `space-member` 1,079, `test-support` 829, `initialization-runtime` 570, `kernel-root` (macros) 310, `codec-factory` 291, `space` 290, `engine` 286, `engine-core` 272, `config` 181, `engine-owners` 152, `engine-live` 76, `document` 15, plus the `ArtifactStore` struct and impl blocks (7.4) | the root mixes the engine with vocabulary, glue, members and tests |
| `S/📦️codec/🧬️owned-schema/🦀️.rs` (3,730) | lines 1 to 1,499 -> `owned-schema-fields` (1,369), the rest -> `owned-schema-decode` (2,361) | field decoders, SPR edit authority and registry vs the paged decode authority, completed records and fresh VCS authority |
| `S/🧩️composition/🗄️durable-group/🦀️.rs` (3,540) | `durable-group-wire` 1,039 (schema constants, anchors, decision/outcome types that do not touch a store), `engine-stage` 636 (staging functions and the outcome types they use), `durable-group` 1,862 (journal records, the three-store commit/recovery machinery, the decision codec) | split by transitive dependence on `ArtifactStore`; staging takes `StoreParts` (5.3) |
| `S/🔄️sync/🦀️.rs` (4,726 active lines) | shared header + `SyncSession` + `ArtifactHost` + socket/link vocabulary 2,062 -> `sync`; `mod native_actor` (2,315, `ArtifactActor` impl 1,416) + folder event-log/text storage 420 -> `sync-native`; `mod wasm_actor` (913) -> `sync-wasm` | native/wasm cfg split already present; three crates are the cfg boundaries |
| `S/🎮️one-item/📬️publication/🦀️.rs` (1,106) | protocol, live authority, prepared, preparation factory 581 -> `one-item-publication`; batch stage/publication and the member wire 525 -> `one-item-batch` | the batch half names `ArtifactStoreBatchPublication`; nothing below needs it |
| `S/🧵️canonical-edit/🦀️.rs` (748) | canonical JSON traversal and cursor 341 -> `canonical-json`; the one-item sealer 407 -> `one-item-publication` | the sealer holds the live authority; the traversal is vocabulary |
| `M/🚪️io/🦀️.rs` (2,170 item lines) | composer registry, `IoKey`, subset validators, format catalog, native-snapshot registration plan 531 -> `codec-registry`; `io_mechanism` (998) + fallback + wire -> `io` | `commit_artifact_assembly_registry_plan` needs the codec guards; both registries are statics (6.5) |
| `S/🧩️composition/🚪️open/**` (12 files, 3,607) | input pipeline (open root, admission, receiving, genesis, dictionary, history) 2,121 -> `member-open`; `InitialMemberStoreOpen` operation + `MemberStoreOpenRetained` 1,050 -> `member-open-store`; the factory selection (328) and `Unsupported*` stubs (88) -> `space-member` | `MemberFactory` is the trait the factory selection needs, and `SpaceMember`/`MemberFactory` sit above the engine |

**Computed item moves** (every group not caused by a designed split; destination = the lowest-ranked crate that uses the item; `lines` are item lines):

| destination crate | from file (stays in crate) | lines | what |
|---|---|---:|---|
| `os-store-ledger` | `S/🔗️read/🦀️.rs` (`os-store-read`) | 163 | ArtifactStoreStringRetirement, impl ArtifactStoreStringRetirement, ArtifactStoreEditRetirement, ArtifactStoreEditMetadata, ArtifactStoreEditRetirementState, impl ArtifactStoreEditRetirement, … (+6) |
| `os-store-ledger` | `S/🔁️replay/🦀️.rs` (`os-store-replay`) | 32 | EffectiveSupersessions, FoldProjection, supersession_digest, edit_supersession_digest |
| `os-store-replay` | `S/🔗️read/🦀️.rs` (`os-store-read`) | 85 | ArtifactStoreSupersessionRetirement, impl ArtifactStoreSupersessionRetirement, ArtifactStoreStringVectorRetirement, impl ArtifactStoreStringVectorRetirement |
| `os-store-canonical-json` | `S/🧵️canonical-edit/🦀️.rs` (`os-store-one-item-publication`) | 341 | ARTIFACT_CANONICAL_JSON_DEPTH, ARTIFACT_CANONICAL_JSON_CHUNK_BYTES, CANONICAL_EDIT_MAXIMUM_BYTES, CANONICAL_EDIT_MAXIMUM_OVERHEAD_BYTES, ArtifactCanonicalJsonEncodeError, ArtifactCanonicalJson, … (+8) |
| `os-store-one-item-publication` | `S/🧵️operation-wire/🦀️.rs` (`os-store-operation-wire`) | 95 | ArtifactPreparedOperationSource, ArtifactPreparedOperationError, impl ArtifactPreparedOperationError, ArtifactCanonicalTreeSource, impl ArtifactCanonicalTreeSource, ArtifactOperationWire, … (+6) |
| `os-store-one-item-publication` | `S/🧩️composition/📨️emission/📦️owned/🦀️.rs` (`os-store-one-item-batch`) | 17 | BatchRetirement, impl ControlledRetirement, BatchAdmission, SourceRetirementIssuer |
| `os-store-one-item-publication` | `S/🧵️operation-wire/📝️text/🦀️.rs` (`os-store-operation-wire`) | 12 | ArtifactOperationTextNode, ArtifactOperationText |
| `os-store-text-io` | `S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` (`os-store-space-schema`) | 19 | impl SpaceHistorySnapshot |
| `os-store-text-io` | `S/📦️codec/🪶️snapshot-capability/🪆️workspace-receiving/🦀️.rs` (`os-store-codec-registry`) | 10 | validate_decoding, validate_encoding |
| `os-store-text-io` | `S/🚪️io/💾️binary/📸️snapshot/🦀️.rs` (`os-store-space-schema`) | 9 | impl protocol, impl SpaceHistorySnapshot, impl DslValue |
| `os-store-text-io` | `S/🚪️io/📝️text/📸️snapshot/🦀️.rs` (`os-store-space-schema`) | 9 | record_tree |
| `os-store-text-io` | `S/🚪️io/📝️text/🧬️mutations/🦀️.rs` (`os-store-space-schema`) | 9 | record_tree |
| `os-store-one-item-batch` | `S/🎮️one-item/📬️publication/🦀️.rs` (`os-store-one-item-publication`) | 525 | MemberStoreOneItemWire, MemberStoreOneItemWireRequest, MemberStoreOneItemWirePreparationFactory, ErasedMemberStoreOneItemPublication, MemberStoreOneItemPublication, impl MemberStoreOneItemPublication, … (+6) |
| `os-store-one-item-batch` | `S/🧩️composition/📬️publication/🤝️group/🦀️.rs` (`os-store-engine`) | 36 | MemberGroupPreparation, impl MemberGroupPreparation |
| `os-store-owned-schema-fields` | `S/📦️codec/🧬️owned-schema/🦀️.rs` (`os-store-owned-schema-decode`) | 1369 | ARTIFACT_ENVELOPE_SCHEMA_FIELD, ARTIFACT_ENVELOPE_ID_FIELD, ARTIFACT_ENVELOPE_VCS_FIELD, ARTIFACT_ENVELOPE_BACKBONE_FIELD, ARTIFACT_ENVELOPE_ACTIVE_ALTERNATIVE_FIELD, ARTIFACT_ENVELOPE_CURSOR_FIELD, … (+61) |
| `os-store-engine-owners` | `S/🧩️composition/🚪️open/🏭️operation/🦀️.rs` (`os-store-member-open-store`) | 20 | MemberSnapshotOpenProgress, MemberSnapshotOpenStep, MemberSnapshotOpenOperation |
| `os-store-codec-registry` | `M/🚪️io/🦀️.rs` (`os-io`) | 531 | Confidence, ComposeError, ErasedComposeSource, ComposedArtifact, ComposeFuture, AsyncComposeFn, … (+46) |
| `os-store-space-history` | `S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` (`os-store-space-schema`) | 39 | SQLITE_SNAPSHOT_DIALECT, schema, rows, word, pair, semantic |
| `os-store-space-history` | `S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/📏️preflight/🫳️borrowed/🦀️.rs` (`os-store-space-schema`) | 27 | Source, absent, impl Source, spec, measure |
| `os-store-initialization-runtime` | `S/🏗️initialization/🔁️replay/🦀️.rs` (`os-store-initialization-replay`) | 82 | ArtifactStoreInitializationNext, impl ArtifactStoreInitializationRuntime |
| `os-store-initialization-runtime` | `S/🧾️document/🦀️.rs` (`os-store-document`) | 18 | fresh_replica_clock, now_ms |
| `os-store-durable-group-wire` | `S/🧩️composition/🗄️durable-group/🦀️.rs` (`os-store-durable-group`) | 1039 | DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1, DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1, DURABLE_OWNED_GROUP_SHAPE_V1, DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES, DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES, … (+68) |
| `os-store-engine-core` | `S/🧾️document/🦀️.rs` (`os-store-document`) | 39 | now_iso, uncommitted_edit_ids, EMPTY_CHECKPOINT_MESSAGE |
| `os-store-engine-live` | `S/🧾️document/🦀️.rs` (`os-store-document`) | 31 | stamp_primary_operation_identity |
| `os-store-engine-stage` | `S/🧩️composition/🗄️durable-group/🦀️.rs` (`os-store-durable-group`) | 636 | DurableStorePreparedOwnerV1, DurableStoreBoundOutcomeV1, stage_store_member, stage_prebuilt_batch_root, abort_staged_store_member, adopt_staged_store_member, … (+12) |
| `os-store-space-member` | `S/🧩️composition/🚪️open/🏭️operation/🦀️.rs` (`os-store-member-open-store`) | 88 | UnsupportedMemberSnapshotOpen, impl UnsupportedMemberSnapshotOpen, UnsupportedMemberFactoryOpen, impl UnsupportedMemberFactoryOpen |
| `os-store-space-member` | `S/🧾️document/🦀️.rs` (`os-store-document`) | 35 | edit_ids_for_changes, materialize_document_snapshot |
| `os-store-member-open-store` | `S/🧩️composition/🚪️open/🦀️.rs` (`os-store-member-open`) | 200 | MemberStoreOpenRetained, impl MemberStoreOpenRetained |
| `os-store-sync-native` | `S/🔄️sync/🦀️.rs` (`os-store-sync`) | 420 | FolderEventLogStorage, FOLDER_EVENT_MAGIC, MAX_FOLDER_EVENT_BYTES, DOCUMENT_PUT_EVENT, BLOB_PUT_EVENT, BLOB_DELETE_EVENT, … (+7) |
| | **total (excluding the store root, Appendix C)** | **5936** | 29 groups |

Name collisions that look like dependencies and are **not** (verified by reading the definitions; do not cut them): `Cursor`, `MAGIC`, `HEADER_SIZE`, `REC_*` (spr cli constants vs replication constants), `Backbone` as an enum variant, `decode_with`, `Phase`, `label`, `paint`-style generic words.

### 5.3 Ports, capabilities and bundles (the only non-lowering cuts)

| # | seam | evidence | prescribed cut |
|---|---|---|---|
| C1 | `os-store-engine-owners` -> `ArtifactStore` | `ArtifactStoreOwnedDisposer` (trait, 19 lines) and `ArtifactStoreCursorDisposer` (81-line impl) take `&mut ArtifactStoreCloseView{store: &mut ArtifactStore}` and call `close_cursor_admit`, `close_cursor_demands`, `maintenance_retirements_step`, `maintenance_retirements_terminal_is_empty`, `owned_roots_terminal_is_empty`, `close_take_backbone_retirement` and read `snapshot_read_leases`, `initial_snapshot_retirement_factory`, `prefix_ring`, `member_inbox`, `backbone`; this is also the field `owned_disposer: Box<dyn …>` of the owners component, so the trait must sit below the component | `trait ArtifactStoreCloseHost<P, Mutation>` (about 14 query/step methods, defined in `engine-owners`); `ArtifactStoreCloseView` holds `&mut dyn ArtifactStoreCloseHost`; the composer implements the trait in `engine` by delegating to the component methods. This is E1b's S3 split finished: the disposal contract is base, the engine type is not named below. |
| C2 | authority minting (E1b open decision) | `ArtifactStoreOneItemLiveAuthority::mint`/`rebind_group` are `pub(crate)`; the engine and the durable-group recovery mint, `one_item_publication_source_denies_generic_or_unbounded_shortcuts` forbids raw field fabrication | keep every authority field private in `os-store-one-item-publication`; `mint` takes a `StoreMintWitness` that only `StoreLog::new` constructs (a unit struct with a private constructor in `engine-core`, re-exported by the publication crate as an opaque borrow `&StoreMintWitness`). Fabrication then needs a live store. The alternative (move authority, seal and prepared types into `engine-core`) puts 1,100 lines of publication vocabulary above the sealer and the batch crates and is rejected. |
| C3 | engine components vs use cases | 56 use-case methods (9 touch five or six components directly, 47 are lifted because they call a higher-ranked method) and the durable-group staging functions need several components at once | one borrowed bundle `StoreParts<'a, P, Mutation>` (six `&'a mut` component references, defined in `engine-live`, built by `ArtifactStore::parts(&mut self)`); `engine-stage` and the use-case methods of `engine` take it. Never `&mut ArtifactStore` below the composer. |
| L1 | `register_sqlite_snapshot` -> `bare_artifact_codec` / `register_native_snapshot_codec` | the 5-line registration glue of the space-history snapshot calls two functions two layers above it (closure 597 and 422 lines) | lift the 5-line caller to `os-store-space` (rank above both); no port |
| L2 | genesis capture -> fresh VCS authority | `S/🚪️io/💾️binary/🌱️genesis/🦀️.rs` (35 lines) uses `ArtifactEnvelopeFreshVcsAuthority` (119-line closure) | lift the 35-line file into `os-store-owned-schema-decode` |
| L3 | member-group staging impl | the 149-line `impl ArtifactStore` in `composition/publication/group` calls the staging functions | it is an inherent impl of the composer, so it moves into `os-store-engine` as use-case methods over `StoreParts` |

Everything else in the first version of the back-edge list (83 targets) was a lowering move of a few lines (5.2).

### 5.4 Residual risk list for section 5

* Method syntax is outside the item graph except inside `ArtifactStore`: a scan of the 316 `ArtifactStore` method names that occur nowhere else found calls from below the composer only in `engine-owners` (the C1 methods listed above). After every move run the differential oracle (section 11) before trusting a cut.
* `os-store-one-item-publication` receives the largest hand-moved groups (operation-wire vocabulary 107 lines, the sealer 407, the batch-digest family 44): its executor keeps `LiveAuthority`, `Prepared`, `Sealer` and the preparation traits together (the knot of 32 items / 857 lines in 5.1).
* `engine-owners` is below `codec-registry` only because `RetainedArtifactCodecCatalog` names `DocumentStoreOwners`; if a peer re-homes the catalog the two can swap rank.

## 6. The crates

### 6.1 Catalogue

Sizes are the item lines of the final assignment plus a proportional share of each file's header lines (`use`, attributes, comments between items); `lib files` counts source files plus embedded data files the crate includes, **after** S0 (a source file that is split yields one file per destination crate). "host-mounted tests" is the test closure that the crate's own lib files mount (test lines / KB including embedded fixtures / files, from 9.3); it stays in the crate unless 9.3 and 9.4 move it to a companion (`os-dsl`, `os-store-member-open`, `os-store-durable-group`, `os-store-sync-native` and the facade rows), and the root-mounted store tests are not in this column at all (section 9). Direct dependencies list every first-party crate the code names (Cargo needs each declared); the crates already extracted by the fleet (`os-vcs`, `os-pack`, `os-command`, `os-history`, `os-channel`, `os-directory(-client)`, `os-identity`, `os-inference`, `os-store-{envelope,command,cursor,projection,message-ledger,one-item,ephemeral,blob-identity,link-schema}`, `value-snapshot-read`) are below every row and not repeated. `engine` carries about 600 more lines than shown for the delegates of 7.5.

| # | crate (`semio-framework-…`) | WP | lib lines | lib KB | lib files | host-mounted tests: lines / KB / files | direct first-party deps (kernel-internal) | cfg |
|---:|---|---:|---:|---:|---:|---|---|---|
| 1 | `os-list-delta` | 1 | 903 | 49 | 1 | 355 / 14 / 1 | - | all targets |
| 2 | `os-semio` | 1 | 432 | 31 | 1 | 155 / 23 / 5 | - | all targets |
| 3 | `os-space-sqlite-cells` | 1 | 176 | 9 | 1 | 0 / 0 / 0 | - | all targets |
| 4 | `os-plugin-registry-schema` | 1 | 154 | 26 | 3 | 26 / 11 / 4 | - | all targets |
| 5 | `os-store-space-schema` | 2 | 798 | 47 | 14 | 137 / 8 / 7 | - | all targets |
| 6 | `os-extension` | 1 | 294 | 13 | 1 | 126 / 6 / 2 | `os-semio` | native + wasm32-unknown (not wasip2) |
| 7 | `os-pack-tools` | 1 | 674 | 37 | 2 | 181 / 15 / 2 | `os-extension` | native only |
| 8 | `os-spr` | 1 | 1111 | 65 | 3 | 730 / 37 / 4 | `os-pack-tools` | native + wasm32 (io module native-only) |
| 9 | `os-dsl` | 1 | 1522 | 89 | 19 | 2471 / 1323 / 41 | `os-pack-tools` | native + wasm32 (registry native-only) |
| 10 | `os-store-child` | 2 | 784 | 41 | 2 | 0 / 0 / 0 | - | all targets |
| 11 | `os-store-ledger` | 2 | 1084 | 79 | 7 | 8 / 6 / 1 | - | all targets |
| 12 | `os-store-retirement` | 2 | 235 | 17 | 2 | 514 / 51 / 5 | `os-store-space-schema`, `os-store-child` | all targets |
| 13 | `os-store-replay` | 2 | 1991 | 124 | 6 | 416 / 34 / 4 | `os-store-ledger` | all targets |
| 14 | `os-store-read` | 2 | 1125 | 93 | 3 | 209 / 46 / 4 | `os-store-ledger`, `os-store-replay` | all targets |
| 15 | `os-store-backbone` | 2 | 705 | 38 | 2 | 184 / 13 / 3 | `os-store-ledger` | all targets |
| 16 | `os-store-canonical-json` | 2 | 1290 | 62 | 6 | 637 / 37 / 5 | - | all targets |
| 17 | `os-store-one-item-publication` | 2 | 1447 | 113 | 9 | 1054 / 115 / 13 | `os-store-ledger`, `os-store-canonical-json` | all targets |
| 18 | `os-store-operation-wire` | 2 | 605 | 50 | 4 | 290 / 36 / 3 | `os-store-canonical-json`, `os-store-one-item-publication` | all targets |
| 19 | `os-store-text-io` | 2 | 1873 | 121 | 11 | 0 / 0 / 0 | `os-store-space-schema`, `os-store-ledger`, `os-store-replay`, `os-store-canonical-json`, `os-store-one-item-publication`, `os-store-operation-wire` | all targets |
| 20 | `os-store-snapshot-clone` | 2 | 1159 | 99 | 7 | 265 / 30 / 6 | `os-store-one-item-publication` | all targets |
| 21 | `os-store-one-item-batch` | 2 | 2107 | 131 | 10 | 611 / 43 / 5 | `os-store-ledger`, `os-store-replay`, `os-store-read`, `os-store-one-item-publication`, `os-store-snapshot-clone` | all targets |
| 22 | `os-store-owned-schema-fields` | 3 | 1444 | 98 | 1 | 0 / 0 / 0 | `os-store-ledger`, `os-store-canonical-json` | all targets |
| 23 | `os-store-owned-schema-decode` | 3 | 2531 | 151 | 2 | 0 / 0 / 0 | `os-store-ledger`, `os-store-read`, `os-store-owned-schema-fields` | all targets |
| 24 | `os-store-member-open` | 3 | 2271 | 119 | 10 | 1499 / 134 / 13 | `os-spr` | all targets |
| 25 | `os-store-engine-owners` | 3 | 891 | 63 | 5 | 0 / 0 / 0 | `os-store-ledger`, `os-store-replay`, `os-store-read`, `os-store-backbone`, `os-store-one-item-publication`, `os-store-one-item-batch`, `os-store-member-open` | all targets |
| 26 | `os-store-codec-registry` | 3 | 1569 | 104 | 10 | 143 / 18 / 6 | `os-semio`, `os-store-text-io`, `os-store-engine-owners` | all targets |
| 27 | `os-io` | 3 | 1915 | 146 | 4 | 505 / 58 / 6 | `os-store-text-io`, `os-store-codec-registry` | all targets |
| 28 | `os-store-space-history` | 3 | 709 | 64 | 10 | 0 / 0 / 0 | `os-store-space-schema`, `os-store-one-item-batch` | all targets |
| 29 | `os-store-initialization-runtime` | 4 | 825 | 53 | 4 | 92 / 7 / 2 | `os-store-ledger`, `os-store-replay`, `os-store-read`, `os-store-text-io` | all targets |
| 30 | `os-store-durable-group-wire` | 4 | 1042 | 57 | 1 | 0 / 0 / 0 | `os-semio`, `os-store-ledger`, `os-store-replay`, `os-store-read`, `os-store-one-item-publication`, `os-store-text-io`, `os-store-one-item-batch` | all targets |
| 31 | `os-store-engine-core` | 4 | 2389 | 169 | 2 | 0 / 0 / 0 | `os-store-child`, `os-store-ledger`, `os-store-replay`, `os-store-read`, `os-store-backbone`, `os-store-one-item-publication`, `os-store-text-io`, `os-store-one-item-batch`, `os-store-owned-schema-fields`, `os-store-engine-owners`, `os-store-initialization-runtime`, `os-store-durable-group-wire` | all targets |
| 32 | `os-store-engine-live` | 4 | 2457 | 160 | 2 | 0 / 0 / 0 | `os-store-ledger`, `os-store-replay`, `os-store-read`, `os-store-backbone`, `os-store-one-item-publication`, `os-store-operation-wire`, `os-store-text-io`, `os-store-one-item-batch`, `os-store-initialization-runtime`, `os-store-engine-core` | all targets |
| 33 | `os-store-engine-stage` | 4 | 636 | 43 | 1 | 0 / 0 / 0 | `os-store-ledger`, `os-store-replay`, `os-store-read`, `os-store-one-item-publication`, `os-store-text-io`, `os-store-one-item-batch`, `os-store-durable-group-wire`, `os-store-engine-core`, `os-store-engine-live` | all targets |
| 34 | `os-store-engine` | 4 | 2607 | 197 | 23 | 83 / 8 / 2 | `os-store-ledger`, `os-store-retirement`, `os-store-replay`, `os-store-read`, `os-store-backbone`, `os-store-one-item-publication`, `os-store-text-io`, `os-store-one-item-batch`, `os-store-engine-owners`, `os-store-codec-registry`, `os-store-initialization-runtime`, `os-store-durable-group-wire`, `os-store-engine-core`, `os-store-engine-live`, `os-store-engine-stage` | all targets |
| 35 | `os-store-initialization-replay` | 5 | 889 | 44 | 5 | 314 / 18 / 1 | `os-store-text-io`, `os-store-owned-schema-fields`, `os-store-engine-owners`, `os-store-codec-registry`, `os-store-initialization-runtime`, `os-store-engine` | all targets |
| 36 | `os-store-codec-factory` | 5 | 313 | 20 | 1 | 0 / 0 / 0 | `os-store-ledger`, `os-store-text-io`, `os-store-owned-schema-fields`, `os-store-engine-owners`, `os-store-codec-registry`, `os-store-engine`, `os-store-initialization-replay` | all targets |
| 37 | `os-store-space-member` | 5 | 1650 | 103 | 4 | 585 / 46 / 2 | `os-store-child`, `os-store-ledger`, `os-store-replay`, `os-store-one-item-publication`, `os-store-text-io`, `os-store-one-item-batch`, `os-store-member-open`, `os-store-engine-owners`, `os-store-engine-core`, `os-store-engine` | all targets |
| 38 | `os-store-document` | 5 | 1784 | 109 | 4 | 506 / 42 / 4 | `os-store-ledger`, `os-store-replay`, `os-store-read`, `os-store-text-io`, `os-store-owned-schema-decode`, `os-store-member-open`, `os-store-engine-owners`, `os-store-codec-registry`, `os-store-initialization-runtime`, `os-store-engine` | all targets (mounted-pack native-split) |
| 39 | `os-store-member-open-store` | 5 | 1062 | 76 | 2 | 0 / 0 / 0 | `os-spr`, `os-store-child`, `os-store-ledger`, `os-store-read`, `os-store-text-io`, `os-store-member-open`, `os-store-engine-owners`, `os-store-initialization-runtime`, `os-store-engine`, `os-store-space-member`, `os-store-document` | all targets |
| 40 | `os-store-config` | 5 | 1243 | 76 | 2 | 0 / 0 / 0 | `os-store-ledger`, `os-store-retirement`, `os-store-read`, `os-store-text-io`, `os-store-snapshot-clone`, `os-store-engine-owners`, `os-store-initialization-runtime`, `os-store-engine`, `os-store-document` | all targets |
| 41 | `os-store-durable-group` | 5 | 1862 | 117 | 1 | 1285 / 110 / 3 | `os-semio`, `os-store-ledger`, `os-store-one-item-publication`, `os-store-text-io`, `os-store-one-item-batch`, `os-store-durable-group-wire`, `os-store-engine-stage`, `os-store-engine` | all targets |
| 42 | `os-store-composition` | 5 | 1482 | 90 | 5 | 199 / 71 / 2 | `os-store-space-member` | all targets |
| 43 | `os-store-space` | 5 | 323 | 20 | 2 | 0 / 0 / 0 | `os-store-space-schema`, `os-store-backbone`, `os-store-text-io`, `os-store-engine-owners`, `os-store-codec-registry`, `os-store-space-history`, `os-store-engine`, `os-store-initialization-replay`, `os-store-codec-factory`, `os-store-space-member`, `os-store-document` | all targets |
| 44 | `os-store-test-support` | 5 | 929 | 57 | 1 | 0 / 0 / 0 | `os-spr`, `os-store-replay`, `os-store-read`, `os-store-one-item-publication`, `os-store-text-io`, `os-store-engine-owners`, `os-io`, `os-store-engine-core`, `os-store-engine`, `os-store-initialization-replay`, `os-store-space-member`, `os-store-document`, `os-store-config`, `os-store-space` | all targets |
| 45 | `os-store-sync` | 6 | 2092 | 114 | 2 | 0 / 0 / 0 | `os-store-backbone`, `os-store-text-io`, `os-store-codec-registry`, `os-store-engine` | feature `sync`; native + wasm32-unknown (not wasip2) |
| 46 | `os-store-sync-native` | 6 | 2735 | 145 | 1 | 4243 / 316 / 15 | `os-semio`, `os-store-ledger`, `os-store-backbone`, `os-store-text-io`, `os-store-codec-registry`, `os-store-engine`, `os-store-space-member`, `os-store-sync` | feature `sync`; native only |
| 47 | `os-store-sync-wasm` | 6 | 913 | 50 | 1 | 0 / 0 / 0 | `os-store-ledger`, `os-store-backbone`, `os-store-codec-registry`, `os-store-sync` | feature `sync`; wasm32-unknown only (not wasip2) |
| 48 | `os-spr-cli` | 1 | 806 | 34 | 1 | 397 / 19 / 1 | `os-pack-tools`, `os-spr` | native only (bin) |
| 49 | `os-kernel (facade)` | 0 | 694 | 47 | 4 | 3780 / 222 / 46 | all 48 crates above (hub of explicit `pub use`, macros, `worker`) | all targets |

Budget status: every row is at or below 3,500 lines / 230 KB / 28 files in the lib closure. The tight rows are `os-store-engine` (about 3,200 with delegates, spill valve in the spec), `os-store-sync-native` (2,735), `os-store-owned-schema-decode` (2,531, 151 KB), `os-store-engine-live` (2,457), `os-store-engine-core` (2,389), `os-io` (1,915 lines but 146 KB) and `os-store-text-io` (121 KB, 11 files).

### 6.2 Per-crate specifications

Each entry: members (source file relative to `M/` unless marked, and the lines it contributes), direct crate dependencies, the external crates the code names (the Cargo dependency list to start from: keep the old manifest's feature sets, for example `semio-framework-value` with `testkit`, `semio-framework-os-history` with `default-features = false`, `semio-framework-os-services` and `ureq` only for `sync-native`), and what is specific. Cfg: see the cfg column of 6.1 and the matrix in 8.3.

#### 1. `os-list-delta` (WP 1; 903 lines, 49 KB, 1 files)

* node: `M/🪡️list-delta`
* members (source, lines contributed): `🪡️list-delta/🦀️.rs` 843
* direct crate deps: none
* external crates named in the code: none
* Leaf. The three exported macros `list_delta!`, `plain_list_delta!`, `row_patch!` (412 lines, 102 `$crate` references) travel with the crate: rewrite `$crate::list_delta::X` to `$crate::X` and add `#[doc(hidden)] pub use` of `semio_framework_value_derive as __value_derive` and `semio_framework_dsl_record_derive as __dsl_record_derive` (the facade has the same re-exports today). The facade re-exports `pub use semio_framework_os_list_delta::{list_delta, plain_list_delta, row_patch, …}` and keeps `pub mod list_delta` as the path.

#### 2. `os-semio` (WP 1; 432 lines, 31 KB, 1 files)

* node: `M/🧬️semio`
* members (source, lines contributed): `🧬️semio/🦀️.rs` 432
* direct crate deps: none
* external crates named in the code: `semio_framework_value`
* Leaf (`.semio` envelope, `wrap_*`/`unwrap_*`, `split_text_preamble*`). Static `STATE`. The `[[bin]] semio` source (`🧬️semio/🏗️bootstrap/🦀️.rs`) stays mounted by the facade under `native-bin`.

#### 3. `os-space-sqlite-cells` (WP 1; 176 lines, 9 KB, 1 files)

* node: `M/🪐️space/🪶️sqlite`
* members (source, lines contributed): `🪐️space/🪶️sqlite/🦀️.rs` 176
* direct crate deps: none
* external crates named in the code: `semio_framework_value`, `semio_framework_pack_json`, `semio_framework_dsl_record`
* Leaf (bounded ordinal/literal cell builders for space entities). Its oracle test lives in the file's own `🧪️tests` (150 lines).

#### 4. `os-plugin-registry-schema` (WP 1; 154 lines, 26 KB, 3 files)

* node: `M/🔌️plugin/📇️registry/📦️deployment/🧬️schema`
* members (source, lines contributed): `🔌️plugin/📇️registry/📦️deployment/🧬️schema/🦀️.rs` 74, `🔌️plugin/📇️registry/📥️runtime/🦀️.rs` 53
* direct crate deps: none
* external crates named in the code: `serde`, `serde_json`
* Leaf, serde only. The plugin peer owns the tree: only the two mounts move. Embeds the 15 KB TS deployment schema (`include_str!`), so the crate is byte-light but file-heavy (3 files). Static `INSTALLED_NATIVE_CATALOG` for `native_catalog`.

#### 5. `os-store-space-schema` (WP 2; 798 lines, 47 KB, 14 files)

* node: `M/🏪️store/🧬️schema`
* members (source, lines contributed): `🏪️store/🧬️schema/📜️space-history/🦀️.rs` 194, `🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` 122, `🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/📏️preflight/🫳️borrowed/🦀️.rs` 48, `🏪️store/🧬️schema/🧬️mutations/🎯️set-active-space-alternative/🦀️.rs` 46, `🏪️store/🧬️schema/🧬️mutations/🧹️remove-space-alternative/🦀️.rs` 40, `🏪️store/🧬️schema/🧬️mutations/🌿️create-space-alternative/🦀️.rs` 34, … (+8 files)
* direct crate deps: none
* external crates named in the code: `semio_framework_value`, `semio_framework_pack_json`, `pack`, `serde`
* Leaf. `store_schema` (the 7-line hub of `pub use semio_framework_os_store_{cursor,projection,envelope,…}`), the space-history document and snapshot entities, the seven `schema/mutations` files, and (orphan rule) the `ArtifactDsl`/`ArtifactPack`-style impls for `SpaceHistorySnapshot`, `SpaceCheckpoint`, `SpaceHistoryMutation` that today sit in `S/🚪️io/{📝️text,💾️binary}/…` and `S/📜️space-history/…/📸️snapshot`.

#### 6. `os-extension` (WP 1; 294 lines, 13 KB, 1 files)

* node: `M/🧩️extension`
* members (source, lines contributed): `🧩️extension/🦀️.rs` 247
* direct crate deps: `os-semio`
* external crates named in the code: `semio_framework_pack_json`, `semio_framework_hash`
* `.sxt` package format; imports `os-semio` and, behind `not(wasm32-wasip2)`, the first-party `semio-framework-deflate` zip. Absent from wasip2 guests.

#### 7. `os-pack-tools` (WP 1; 674 lines, 37 KB, 2 files)

* node: `M/🎒️pack/⌨️cli`
* members (source, lines contributed): `🎒️pack/⌨️cli/🦀️.rs` 532, `🎒️pack/🚦️control/🦀️.rs` 78
* direct crate deps: `os-extension`
* external crates named in the code: `semio_framework_dsl_record`, `semio_framework_pack_source`
* Native only: the `pack` CLI (`cmd_inspect/verify/hash/to-dsl/from-dsl/diff`) and `🚦️control` (`CommandBytes`, `CommandContext`, `read_command_file`). Depends on `os-extension::content_hash`. The `[[bin]] pack` source stays a facade bin that calls it.

#### 8. `os-spr` (WP 1; 1111 lines, 65 KB, 3 files)

* node: `M/📡️spr`
* members (source, lines contributed): `📡️spr/💎️materialize/🦀️.rs` 466, `📡️spr/🔌️io/🦀️.rs` 355, `📡️spr/🦀️.rs` 131
* direct crate deps: `os-pack-tools`
* external crates named in the code: `semio_framework_pack_source`, `protocol`
* The spr component (`extract_range`, `verify_slice`, `content_frontier`, `compile_ops`, …), native `.spr` file lifecycle (`🔌️io`) and materialization. Native-only items inside `io`. Feature `protocol-laws` mounts `🧪️tests/⚖️protocol-laws` (1,114 lines) into the lib (not `cfg(test)`): with it the lib closure is about 2,200 lines; feature `mutation-testing` exports `mutation_fixture_ops`. The 52 re-export statements of the old component become the facade's `os_spr` hub (6.4).

#### 9. `os-dsl` (WP 1; 1522 lines, 89 KB, 19 files)

* node: `M/🗣️dsl`
* members (source, lines contributed): `🗣️dsl/🦀️.rs` 465, `🗣️dsl/👪️family/🕸️graph/🦀️.rs` 131, `🗣️dsl/🫴️receiving/🦀️.rs` 112, `🗣️dsl/👪️family/📊️sheet/🦀️.rs` 104, `🗣️dsl/🧠️lsp/🦀️.rs` 102, `🗣️dsl/👪️family/🧑‍🍳recipe/🦀️.rs` 76, … (+6 files)
* direct crate deps: `os-pack-tools`
* external crates named in the code: `semio_framework_value`, `semio_framework_dsl_record`, `semio_framework_dsl`, `protocol`, `semio_framework_diagnostic`, `semio_framework_pack_json`
* The OS product grammars (`dsl` component 465 lines, 7 `👪️family` kits, grammar, lsp, receiving) and the native-only `registry` (needs `os-pack-tools::SchemaResolver`, so that one edge is `cfg(not(wasm32))`). Macros `original_ordinal_operation_receiving!` and `original_tagged_operation_receiving!` travel (`$crate::os_dsl::original_receiving` becomes `$crate::original_receiving`). Static `SCHEMA_REGISTRY`; 7 `.semio` grammar files embedded (3.4 KB). The tests embed a 1,126 KB originals fixture: read at run time (section 9).

#### 10. `os-store-child` (WP 2; 784 lines, 41 KB, 2 files)

* node: `M/🏪️store/🪆️child`
* members (source, lines contributed): `🏪️store/🪆️child/🦀️.rs` 407, `🏪️store/🪆️child/🧬️retained-clone/🦀️.rs` 281
* direct crate deps: none
* external crates named in the code: `semio_framework_dsl_record`, `semio_framework_value`, `semio_framework_artifact_reference`, `semio_framework_schema_composition`, `protocol`
* Leaf: `ArtifactChild`, child references, retained clone of a composed child, `OwnerRef` DSL records. `derive`/record paths come from `dsl_record`, no kernel dependency.

#### 11. `os-store-ledger` (WP 2; 1084 lines, 79 KB, 7 files)

* node: `M/🏪️store/📒️ledger`
* members (source, lines contributed): `🏪️store/📒️ledger/🦀️.rs` 517, `🏪️store/📒️ledger/♻️retirement/🦀️.rs` 199, `🏪️store/🔗️read/🦀️.rs` 163, `🏪️store/📨️messages/✂️clamp/🔁️settlement/🦀️.rs` 37, `🏪️store/🔁️replay/🦀️.rs` 32, `🏪️store/📨️messages/✂️clamp/🦀️.rs` 3, … (+1 files)
* direct crate deps: none
* external crates named in the code: `semio_framework_value`, `protocol`, `semio_framework_pack_json`, `semio_framework_artifact_reference`, `semio_framework_hash`, `semio_framework_diagnostic`
* The edit-message ledger, cursor revision accumulator, history commit reservation, pending command report, edit retirement (decoded edits, changes, paged operation rows) and the ledger-level retirement helpers that `read` and `replay` used to host (163 + 32 lines moved in, 5.2). Bottom of the engine foundation.

#### 12. `os-store-retirement` (WP 2; 235 lines, 17 KB, 2 files)

* node: `M/🏪️store/♻️retirement`
* members (source, lines contributed): `🏪️store/♻️retirement/📦️backing/🦀️.rs` 164, `🏪️store/♻️retirement/🦀️.rs` 35
* direct crate deps: `os-store-space-schema`, `os-store-child`
* external crates named in the code: `semio_framework_value`
* Whole physical retirement of empty resident Store metadata backings and the small `retirement_integration` mount. Tests are byte-light but numerous (514 lines).

#### 13. `os-store-replay` (WP 2; 1991 lines, 124 KB, 6 files)

* node: `M/🏪️store/🔁️replay`
* members (source, lines contributed): `🏪️store/🔁️replay/🦀️.rs` 1081, `🏪️store/🔁️replay/🎮️operation/🦀️.rs` 368, `🏪️store/🔁️replay/🛑️cancel/🦀️.rs` 232, `🏪️store/🔗️read/🦀️.rs` 85, `🏪️store/🔁️replay/⏳pending/🦀️.rs` 40, `🏪️store/🔁️replay/⏱️budget/🦀️.rs` 34
* direct crate deps: `os-store-ledger`
* external crates named in the code: `semio_framework_value`, `protocol`, `semio_framework_job`, `semio_framework_diagnostic`
* Effective forwards, edit replay with the prefix snapshot ring, pair merges; `raw_replay_retirement` (cancel), `replay_preparation` (domain-owned semantic preparation, `ReplayOwnedState` 174-line impl stays with its type), replay budget and pending-step types. Receives `ArtifactStoreSupersessionRetirement` and the string-vector retirement from `read` (85 lines).

#### 14. `os-store-read` (WP 2; 1125 lines, 93 KB, 3 files)

* node: `M/🏪️store/🔗️read`
* members (source, lines contributed): `🏪️store/🔗️read/🦀️.rs` 910, `🏪️store/🕰️history/📐️planning/🎟️admission/🦀️.rs` 78, `🏪️store/🕰️history/📐️planning/♻️retirement/🦀️.rs` 3
* direct crate deps: `os-store-ledger`, `os-store-replay`
* external crates named in the code: `semio_framework_value`, `protocol`, `semio_framework_artifact_reference`
* Opaque snapshot reads and the owner catalogs: derived snapshots, replays, history previews, the snapshot-read lease registry, per-owner retirement factories and displaced-owner reservations; `history_read_admission`. After 248 lines of ledger/replay vocabulary moved down it is 1,125 lines.

#### 15. `os-store-backbone` (WP 2; 705 lines, 38 KB, 2 files)

* node: `M/🏪️store/🔗️backbone`
* members (source, lines contributed): `🏪️store/🔗️backbone/🦀️.rs` 502, `🏪️store/🔗️backbone/♻️retirement/🦀️.rs` 138
* direct crate deps: `os-store-ledger`
* external crates named in the code: `semio_framework_dsl_record`, `web_sys`, `semio_framework_dsl_record_derive`, `semio_framework_diagnostic`, `semio_framework_value`
* Hot-swappable backbones (memory, channel, port, registry) and their retirement. Static `HOST_BACKBONE_PORT`; `web_sys` window access on browser wasm32 (two uses). Uses the `#[dsl(…)]` derive: needs the derive path-root attribute (2.3).

#### 16. `os-store-canonical-json` (WP 2; 1290 lines, 62 KB, 6 files)

* node: `M/🏪️store/🧵️canonical-edit`
* members (source, lines contributed): `🏪️store/🧵️canonical-edit/🦀️.rs` 355, `🏪️store/🧵️canonical-edit/🌱️value/🪪️admission/🦀️.rs` 326, `🏪️store/🧵️canonical-edit/🧵️borrowed/🦀️.rs` 306, `🏪️store/🧵️canonical-edit/📖️reader/🦀️.rs` 163, `🏪️store/🧵️canonical-edit/🌱️value/🦀️.rs` 49, `🏪️store/🧵️canonical-edit/🧳️source/🦀️.rs` 2
* direct crate deps: none
* external crates named in the code: `semio_framework_value`, `protocol`
* Store-owned canonical JSON traversal (`ArtifactCanonicalJson`, cursor, frames), canonical value admission, borrowed typed nodes, byte-bounded reader. The sealer that used to share its root file moves up (5.2).

#### 17. `os-store-one-item-publication` (WP 2; 1447 lines, 113 KB, 9 files)

* node: `M/🏪️store/🎮️one-item/📬️publication`
* members (source, lines contributed): `🏪️store/🎮️one-item/📬️publication/🦀️.rs` 583, `🏪️store/🧵️canonical-edit/🦀️.rs` 410, `🏪️store/🧵️canonical-edit/🧳️source/🦀️.rs` 117, `🏪️store/🧵️operation-wire/🦀️.rs` 95, `🏪️store/📦️prepared/♻️retirement/🦀️.rs` 50, `🏪️store/🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs` 36, … (+3 files)
* direct crate deps: `os-store-ledger`, `os-store-canonical-json`
* external crates named in the code: `semio_framework_value`, `semio_framework_job`, `semio_framework_hash`
* The retained one-item publication protocol: live authority (fields private, C2), prepared owner and seal, preparation trait/factory, batch item authority, the member one-item wire request vocabulary, the canonical-edit **sealer** (407 lines: `ArtifactStoreOneItemSealer` + 3 impls) and its source lease, operation-wire vocabulary (`ArtifactOperationWire`, prepared-operation source/error/text node, 107 lines) and the batch-digest family (44 lines). Keeps the knot of 32 items / 857 lines together.

#### 18. `os-store-operation-wire` (WP 2; 605 lines, 50 KB, 4 files)

* node: `M/🏪️store/🧵️operation-wire`
* members (source, lines contributed): `🏪️store/🧵️operation-wire/🦀️.rs` 188, `🏪️store/🧵️operation-wire/📦️output/🦀️.rs` 153, `🏪️store/🧵️operation-wire/📦️output/📋️operations/🦀️.rs` 126, `🏪️store/🧵️operation-wire/📝️text/🦀️.rs` 82
* direct crate deps: `os-store-canonical-json`, `os-store-one-item-publication`
* external crates named in the code: `semio_framework_value`
* Prestage operation bytes from borrowed typed sources and the paged output (`📦️output`, `📋️operations`): `ArtifactPreparedOperationCursor` and the `OperationWirePreparationFactory`. Small, byte-heavy (48 KB for 605 lines).

#### 19. `os-store-text-io` (WP 2; 1873 lines, 121 KB, 11 files)

* node: `M/🏪️store/🚪️io`
* members (source, lines contributed): `🏪️store/🚪️io/📝️text/🦀️.rs` 1329, `🏪️store/🚪️io/🦀️.rs` 206, `🏪️store/🚪️io/💾️binary/🪪️operation-identity/🦀️.rs` 41, `🏪️store/🚪️io/📝️text/🧬️mutations/🎚️fields/🦀️.rs` 28, `🏪️store/🚪️io/💾️binary/🪪️operation-identity/📑️rows/🦀️.rs` 25, `🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` 19, … (+5 files)
* direct crate deps: `os-store-space-schema`, `os-store-ledger`, `os-store-replay`, `os-store-canonical-json`, `os-store-one-item-publication`, `os-store-operation-wire`
* external crates named in the code: `semio_framework_value`, `semio_framework_artifact_reference`, `semio_framework_pack_json`, `protocol`, `semio_framework_diagnostic`, `pack`, `semio_framework_hash`
* The serialization seam (`ArtifactDsl`, `ArtifactPack`, `ArtifactStoreSnapshot`, `ArtifactCodec` table, erased codecs), the document text format (`.dsl`/`.ops`/`.spr` parse/print, 1,329 lines), operation identity cursor and rows. The genesis pack admission file that used to sit in `S/🚪️io/💾️binary/🌱️genesis` moves up to `owned-schema-decode` (L2). 11 files; its subject-based test lines are 1,884, so its tests are companion crates (section 9).

#### 20. `os-store-snapshot-clone` (WP 2; 1159 lines, 99 KB, 7 files)

* node: `M/🏪️store/🧬️snapshot-clone`
* members (source, lines contributed): `🏪️store/🧬️snapshot-clone/🦀️.rs` 650, `🏪️store/🧬️snapshot-clone/✏️config-apply/🦀️.rs` 167, `🏪️store/🧬️snapshot-clone/⚖️law/🦀️.rs` 81, `🏪️store/🧬️snapshot-clone/🪪️metadata/🦀️.rs` 73, `🏪️store/🧬️snapshot-clone/🚚️handoff/🦀️.rs` 61, `🏪️store/🧬️snapshot-clone/♻️pending/🦀️.rs` 24, … (+1 files)
* direct crate deps: `os-store-one-item-publication`
* external crates named in the code: `semio_framework_value`, `semio_framework_job`
* Bounded retained-clone publication preparation (`RetainedClonePreparation`, handoff, custody, law, metadata, config-apply). `ConfigApplyMutation` is named by the facade macro `field_set_mutations!`.

#### 21. `os-store-one-item-batch` (WP 2; 2107 lines, 131 KB, 10 files)

* node: `M/🏪️store/📬️one-item-owners`
* members (source, lines contributed): `🏪️store/📬️paged-one-item/🦀️.rs` 563, `🏪️store/🎮️one-item/📬️publication/🦀️.rs` 525, `🏪️store/📬️one-item-owners/🦀️.rs` 246, `🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs` 219, `🏪️store/📬️paged-one-item/🧬️mutation-apply/🦀️.rs` 158, `🏪️store/🧩️composition/📨️emission/📦️owned/🦀️.rs` 146, … (+4 files)
* direct crate deps: `os-store-ledger`, `os-store-replay`, `os-store-read`, `os-store-one-item-publication`, `os-store-snapshot-clone`
* external crates named in the code: `semio_framework_value`, `semio_framework_job`, `protocol`
* Batch stage and publication (`ArtifactStoreBatchStage`, `ArtifactStoreBatchPublication` 243-line impl), the member one-item publication, owners of one one-item preparation, paged one-item preparation (594 + `mutation-apply` 177), prepared retirement, ephemeral publication transfer/preparation, owned batch emission, member-group preparation (36 lines moved out of the `ArtifactStore` impl file).

#### 22. `os-store-owned-schema-fields` (WP 3; 1444 lines, 98 KB, 1 files)

* node: `M/🏪️store/📦️codec/🧬️owned-schema/🧱️fields (new)`
* members (source, lines contributed): `🏪️store/📦️codec/🧬️owned-schema/🦀️.rs` 1369
* direct crate deps: `os-store-ledger`, `os-store-canonical-json`
* external crates named in the code: `semio_framework_value`, `semio_framework_job`, `semio_framework_pack_json`
* Envelope field decoders: field catalog, SPR edit/array authorities, repository history entry authority/decoder, bounded array authorities, field decoder registry (lines 1 to 1,499 of the old file plus 4 constants and `OwnedSchemaEmptyArrayAuthority` that the upper half defined).

#### 23. `os-store-owned-schema-decode` (WP 3; 2531 lines, 151 KB, 2 files)

* node: `M/🏪️store/📦️codec/🧬️owned-schema`
* members (source, lines contributed): `🏪️store/📦️codec/🧬️owned-schema/🦀️.rs` 2361, `🏪️store/🚪️io/💾️binary/🌱️genesis/🦀️.rs` 35
* direct crate deps: `os-store-ledger`, `os-store-read`, `os-store-owned-schema-fields`
* external crates named in the code: `semio_framework_value`, `semio_framework_job`, `semio_framework_hash`, `protocol`
* The paged, credit-bounded decode authority (353-line impl, `InteractiveJob`), rejected/unadmitted decode retirement, completed-record registry, fresh VCS authority (307-line impl), fresh field decoder, `ArtifactEnvelopeDecodeOwnerBundle`, plus the genesis-pack capture file (L2). Implements `semio_framework_job::InteractiveJob` for the authority (trait foreign, type local).

#### 24. `os-store-member-open` (WP 3; 2271 lines, 119 KB, 10 files)

* node: `M/🏪️store/🧩️composition/🚪️open`
* members (source, lines contributed): `🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs` 548, `🏪️store/🧩️composition/🚪️open/🦀️.rs` 413, `🏪️store/🧩️composition/🚪️open/🌱️genesis/🦀️.rs` 263, `🏪️store/🧩️composition/🚪️open/📜️history/🦀️.rs` 250, `🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/🧾️record/🦀️.rs` 228, `🏪️store/🧩️composition/🚪️open/📜️history/🗂️dictionary/📇️index/🦀️.rs` 170, … (+4 files)
* direct crate deps: `os-spr`
* external crates named in the code: `semio_framework_value`, `protocol`, `semio_framework_hash`, `semio_framework_artifact_reference`, `semio_framework_job`
* The member-open input pipeline: caller-retained request (272-line impl), admission, receiving, canonical genesis framing, the dictionary (index, identity, record), history verification and the committed-input handoff. 11 files; 2,272 lines.

#### 25. `os-store-engine-owners` (WP 3; 891 lines, 63 KB, 5 files)

* node: `M/🏪️store/🏗️initialization/🏠️owners`
* members (source, lines contributed): `🏪️store/🏗️initialization/🏠️owners/🦀️.rs` 586, `S/🦀️.rs [close port region]` 152, `🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs` 20, `🏪️store/🏗️initialization/🏠️owners/🏭️producer/🧬️schema/🦀️.rs` 16, `🏪️store/🏗️initialization/🏠️owners/🏭️member/🧬️schema/🦀️.rs` 8
* direct crate deps: `os-store-ledger`, `os-store-replay`, `os-store-read`, `os-store-backbone`, `os-store-one-item-publication`, `os-store-one-item-batch`, `os-store-member-open`
* external crates named in the code: `semio_framework_value`, `semio_framework_pack_json`
* `DocumentStoreOwners` (328-line impl) and its erased retirements, the unscheduled catalog completion, `MemberStoreProducer` and the member/producer schema bits, the **store-close port** `ArtifactStoreCloseHost` with `ArtifactStoreOwnedDisposer`, `ArtifactStoreCursorDisposer` (81-line impl) and `ArtifactStoreCloseView` (C1), and `MemberSnapshotOpenOperation` (vocabulary 20 lines moved down).

#### 26. `os-store-codec-registry` (WP 3; 1569 lines, 104 KB, 10 files)

* node: `M/🏪️store/📦️codec`
* members (source, lines contributed): `🚪️io/🦀️.rs` 531, `🏪️store/📦️codec/🦀️.rs` 358, `🏪️store/📦️codec/🎮️operation/🦀️.rs` 113, `🚪️io/🪪️bindings/🦀️.rs` 88, `🏪️store/📦️codec/🪶️snapshot-capability/🛫️native-encoding/🦀️.rs` 58, `🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-retirement/🦀️.rs` 50, … (+4 files)
* direct crate deps: `os-semio`, `os-store-text-io`, `os-store-engine-owners`
* external crates named in the code: `semio_framework_artifact_reference`, `semio_framework_value`, `semio_framework_schema_registry`, `semio_framework_dsl_record`, `pack`, `semio_framework_diagnostic`
* All codec/IO registries: document codec registry, dialect migrations, the old composer registry (`IoKey`, `ComposerEntry`, subset validators, format catalog), the `io::bindings` catalog, native/sqlite snapshot capability, `RetainedArtifactCodecCatalog` (names `DocumentStoreOwners`), assembly registry plan and guards (`commit_artifact_assembly_registry_plan`). **Eight statics** (6.5).

#### 27. `os-io` (WP 3; 1915 lines, 146 KB, 4 files)

* node: `M/🚪️io`
* members (source, lines contributed): `🚪️io/🦀️.rs` 1653, `🚪️io/📤️serialization/📦️owned/🦀️.rs` 83, `🚪️io/🧰️testing/🦀️.rs` 24
* direct crate deps: `os-store-text-io`, `os-store-codec-registry`
* external crates named in the code: `semio_framework_value`, `pack`, `semio_framework_schema_registry`, `semio_framework_artifact_reference`, `semio_framework_async`, `semio_framework_diagnostic`, `semio_framework_pack_json`, `semio_framework_job`, `semio_framework_io_sqlite_snapshot`
* OS artifact IO mechanism: `io_mechanism` (typed registry keyed `(from,into)`, routing, `io_run`, native snapshot registration), the IO fallback, wire framing, `serialization/owned`, `io::testing`. Statics `IO_FALLBACK`, `INTERNED`, `IO_MECHANISM_REGISTRY`, `NATIVE_SNAPSHOT_REGISTRY`. Macros `compose_thunk!`, `io_run_thunk!`, `io_sniff_thunk!` travel (`$crate::io::X` to `$crate::X`, `$crate::io_schema` hidden re-export). Mounts `F/🚪️io/📝️text` as `io::text`.

#### 28. `os-store-space-history` (WP 3; 709 lines, 64 KB, 10 files)

* node: `M/🏪️store/📜️space-history`
* members (source, lines contributed): `🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs` 436, `🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🛂️validation/🦀️.rs` 52, `🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🛂️admission/🦀️.rs` 49, `🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` 45, `🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs` 41, `🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/📏️preflight/🫳️borrowed/🦀️.rs` 27, … (+3 files)
* direct crate deps: `os-store-space-schema`, `os-store-one-item-batch`
* external crates named in the code: `semio_framework_value`, `serde_json`, `semio_framework_pack_json`, `semio_framework_artifact_reference`, `pack`, `semio_framework_dsl_record`
* SQLite snapshot IO of space history (native/preflight/admission/validation/receiving) and the registration glue's callee side; the 5-line caller moves up to `os-store-space` (L1). 9 files.

#### 29. `os-store-initialization-runtime` (WP 4; 825 lines, 53 KB, 4 files)

* node: `M/🏪️store/🏗️initialization/🏃️runtime (new)`
* members (source, lines contributed): `S/🦀️.rs [initialization region]` 589, `🏪️store/🏗️initialization/🔁️replay/🦀️.rs` 82, `🏪️store/🏗️initialization/📚️catalog/🦀️.rs` 79, `🏪️store/🧾️document/🦀️.rs` 18
* direct crate deps: `os-store-ledger`, `os-store-replay`, `os-store-read`, `os-store-text-io`
* external crates named in the code: `semio_framework_value`, `protocol`, `semio_framework_hash`, `js_sys`
* `ArtifactStoreInitializationRuntime` (361-line impl) with digest, edit index and owner catalog, `bound_edit_messages`, the catalog close macro files and the clock helpers `fresh_replica_clock`, `now_ms` (js-sys on browser wasm32).

#### 30. `os-store-durable-group-wire` (WP 4; 1042 lines, 57 KB, 1 files)

* node: `M/🏪️store/🧩️composition/🗄️durable-group/📜️wire (new)`
* members (source, lines contributed): `🏪️store/🧩️composition/🗄️durable-group/🦀️.rs` 1042
* direct crate deps: `os-semio`, `os-store-ledger`, `os-store-replay`, `os-store-read`, `os-store-one-item-publication`, `os-store-text-io`, `os-store-one-item-batch`
* external crates named in the code: `semio_framework_value`, `semio_framework_dsl_record`, `semio_framework_artifact_reference`, `semio_framework_pack_json`, `semio_framework_job`, `semio_framework_hash`, `semio_framework_dsl_record_derive`, `semio_framework_async`, `semio_framework_diagnostic`
* Schema constants, anchors/members, the decision error, unbound/bound one-item outcomes and their DSL/pack codecs, journal receipt/record types that do not name a store, `ArtifactStoreDurableGroupRootV1` (the type stored in `StoreWire`) and the neutral JSON admission helpers. Uses `os-semio` wrap/unwrap.

#### 31. `os-store-engine-core` (WP 4; 2389 lines, 169 KB, 2 files)

* node: `M/🏪️store/⚙️engine/🧱️core (new)`
* members (source, lines contributed): `S/🦀️.rs [impl ArtifactStore (methods)]` 1791, `S/🦀️.rs [history columns]` 156, `S/🦀️.rs [ArtifactStore struct]` 91, `S/🦀️.rs [envelope view region]` 86, `S/🦀️.rs [post-impl helpers]` 57, `🏪️store/🧾️document/🦀️.rs` 39
* direct crate deps: `os-store-child`, `os-store-ledger`, `os-store-replay`, `os-store-read`, `os-store-backbone`, `os-store-one-item-publication`, `os-store-text-io`, `os-store-one-item-batch`, `os-store-owned-schema-fields`, `os-store-engine-owners`, `os-store-initialization-runtime`, `os-store-durable-group-wire`
* external crates named in the code: `semio_framework_value`, `protocol`, `semio_framework_dsl_record`, `semio_framework_job`, `semio_framework_artifact_reference`, `semio_framework_diagnostic`, `semio_framework_io_base64`, `semio_framework_pack_json`, `semio_framework_hash`
* Components `StoreLog`, `StoreOwners`, `StoreWire`, `StoreCursor` (struct definitions, constructors, invariants) and their 171 methods / 1,791 lines (7.4), `CommandReceipt`, `ArtifactEnvelopeView`, `OwnerRef` DSL impl, `HistoryColumn` + `build_history_columns`, `now_iso`, `uncommitted_edit_ids`, `EMPTY_CHECKPOINT_MESSAGE`. Defines `StoreMintWitness` (C2).

#### 32. `os-store-engine-live` (WP 4; 2457 lines, 160 KB, 2 files)

* node: `M/🏪️store/⚙️engine/🎞️live (new)`
* members (source, lines contributed): `S/🦀️.rs [impl ArtifactStore (methods)]` 2132, `S/🦀️.rs [post-impl helpers]` 66, `S/🦀️.rs [ArtifactStore struct]` 46, `🏪️store/🧾️document/🦀️.rs` 31, `S/🦀️.rs [ArtifactStore struct region]` 10
* direct crate deps: `os-store-ledger`, `os-store-replay`, `os-store-read`, `os-store-backbone`, `os-store-one-item-publication`, `os-store-operation-wire`, `os-store-text-io`, `os-store-one-item-batch`, `os-store-initialization-runtime`, `os-store-engine-core`
* external crates named in the code: `protocol`, `semio_framework_value`, `semio_framework_pack_json`
* Components `StoreLive` and `StoreReplay`, the `StoreParts` bundle (C3), their 126 methods / 2,132 lines, and the free helpers the live fold needs (`ArtifactStoreDocumentRootCommitAuthority`, `edit_actor_from_meta`, `AppliedMutation`, `transition_retraction_closure`, `document_backbone_batches`, `stamp_primary_operation_identity`). Static `TURNS` is test-only.

#### 33. `os-store-engine-stage` (WP 4; 636 lines, 43 KB, 1 files)

* node: `M/🏪️store/⚙️engine/🗄️stage (new)`
* members (source, lines contributed): `🏪️store/🧩️composition/🗄️durable-group/🦀️.rs` 636
* direct crate deps: `os-store-ledger`, `os-store-replay`, `os-store-read`, `os-store-one-item-publication`, `os-store-text-io`, `os-store-one-item-batch`, `os-store-durable-group-wire`, `os-store-engine-core`, `os-store-engine-live`
* external crates named in the code: `semio_framework_hash`, `semio_framework_value`, `semio_framework_job`, `semio_framework_artifact_reference`, `semio_framework_pack_json`
* The durable-group staging functions (`stage_store_member`, `stage_prebuilt_batch_root`, `abort_staged_store_member`, `adopt_staged_store_member`, validate/clear/retire) and the store-touching outcome impls (`DurableStoreBoundOutcomeV1`, `DurableStorePreparedOwnerV1`), rewritten from `&mut ArtifactStore` to `&mut StoreParts`.

#### 34. `os-store-engine` (WP 4; 2607 lines, 197 KB, 23 files)

* node: `M/🏪️store/⚙️engine (new)`
* members (source, lines contributed): `S/🦀️.rs [impl ArtifactStore (methods)]` 1857, `🏪️store/🧩️composition/📬️publication/🤝️group/🦀️.rs` 152, `S/🦀️.rs [post-impl helpers]` 122, `S/🦀️.rs [config store constructor]` 67, `S/🦀️.rs [merge helpers]` 63, `S/🦀️.rs [module mounts]` 54, … (+7 files)
* direct crate deps: `os-store-ledger`, `os-store-retirement`, `os-store-replay`, `os-store-read`, `os-store-backbone`, `os-store-one-item-publication`, `os-store-text-io`, `os-store-one-item-batch`, `os-store-engine-owners`, `os-store-codec-registry`, `os-store-initialization-runtime`, `os-store-durable-group-wire`, `os-store-engine-core`, `os-store-engine-live`, `os-store-engine-stage`
* external crates named in the code: `semio_framework_value`, `protocol`, `semio_framework_hash`, `semio_framework_artifact_reference`, `semio_framework_diagnostic`
* The composer `ArtifactStore<P, Mutation>` (six component fields), `new`/`construct`/seed/adopt, the 56 use-case methods (1,857 lines: `dispatch`, `advance_apply_batch` 301, `ingest_remote_merge` 361, `begin_apply_batch_owned` 130, `adopt_state`, `close_cursor_admit` 121, …), **125 delegates** for the `pub` methods homed in the components (about 600 lines), `impl Drop` (72), `config_store_from_initialized_runtime_with_owners`, the member-group impl (152), the post-impl helpers (`fold_history`, `merge_by_id`, `ScratchOwner`, …), and the `ArtifactStoreCloseHost` implementation (C1). The 19 small fixtures embedded by the old root (15 KB) go to the crate of the item that includes them (the S0 tool reads the `include_str!` argument). Spill valve if it exceeds 3,300 lines: move `ingest_remote_merge`, `advance_apply_batch` and `begin_apply_batch_owned` (792 lines) into a second crate of bundle methods over `StoreParts`.

#### 35. `os-store-initialization-replay` (WP 5; 889 lines, 44 KB, 5 files)

* node: `M/🏪️store/🏗️initialization/🔁️replay`
* members (source, lines contributed): `🏪️store/🏗️initialization/🔁️replay/🦀️.rs` 799, `🏪️store/🏗️initialization/🤝️authority/🦀️.rs` 32, `🏪️store/🏗️initialization/📣️publication/🧬️schema/🦀️.rs` 8, `🏪️store/🏗️initialization/📚️catalog/🦀️.rs` 6, `🏪️store/🏗️initialization/🌱️genesis/🏭️factory/🧬️schema/🦀️.rs` 5
* direct crate deps: `os-store-text-io`, `os-store-owned-schema-fields`, `os-store-engine-owners`, `os-store-codec-registry`, `os-store-initialization-runtime`, `os-store-engine`
* external crates named in the code: `semio_framework_pack_json`, `semio_framework_job`
* `ArtifactStoreReplayInitializer` (626-line impl), `ArtifactStoreInitializationAuthority`, `ArtifactStorePublication`, owner producer/genesis factory schema files, catalog close.

#### 36. `os-store-codec-factory` (WP 5; 313 lines, 20 KB, 1 files)

* node: `M/🏪️store/📦️codec/🏭️factory (new)`
* members (source, lines contributed): `S/🦀️.rs [codec constructors region]` 291
* direct crate deps: `os-store-ledger`, `os-store-text-io`, `os-store-owned-schema-fields`, `os-store-engine-owners`, `os-store-codec-registry`, `os-store-engine`, `os-store-initialization-replay`
* external crates named in the code: `semio_framework_value`
* Engine-side codec constructors `artifact_codec::<P, Mutation>` / `bare_artifact_codec` (163 lines), `close_codec_reduction_store`, `replay_envelopes_onto_pair`, `ARTIFACT_CODEC_APPLY_CLOSE_MAXIMUM_STEPS` (E1b's S1: the 296 call sites in 200 files already use these free functions).

#### 37. `os-store-space-member` (WP 5; 1650 lines, 103 KB, 4 files)

* node: `M/🏪️store/🧩️composition/👥️space-member (new)`
* members (source, lines contributed): `S/🦀️.rs [space_members! macro and SpaceHost region]` 960, `🏪️store/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs` 328, `S/🦀️.rs [link vocabulary region]` 125, `🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs` 88, `🏪️store/🧾️document/🦀️.rs` 35
* direct crate deps: `os-store-child`, `os-store-ledger`, `os-store-replay`, `os-store-one-item-publication`, `os-store-text-io`, `os-store-one-item-batch`, `os-store-member-open`, `os-store-engine-owners`, `os-store-engine-core`, `os-store-engine`
* external crates named in the code: `semio_framework_value`, `semio_framework_artifact_reference`, `semio_framework_job`, `semio_framework_schema_composition`, `protocol`, `semio_framework_hash`
* `SpaceMember` (194), `impl SpaceMember for ArtifactStore` (478, allowed: the type is local to `engine`), `MemberFactory`/`MemberVisit`/`MemberStoreVisitor(Mut)`, `NoMembers` (238), `BlobStore`, the link vocabulary (`ArtifactRefs`, `LinkState`, `LinkResolver`, `MemberDirectory`, `NoBlobStore`, `MemberLinkResolver`), the factory selection (328) and `Unsupported*` stubs, `edit_ids_for_changes`, `materialize_document_snapshot`. The macro `space_members!` stays in the facade.

#### 38. `os-store-document` (WP 5; 1784 lines, 109 KB, 4 files)

* node: `M/🏪️store/🧾️document`
* members (source, lines contributed): `🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs` 1225, `🏪️store/🧾️document/📥️mounted-pack/🦀️.rs` 448, `🏪️store/🧾️document/🦀️.rs` 44, `S/🦀️.rs [envelope view region]` 15
* direct crate deps: `os-store-ledger`, `os-store-replay`, `os-store-read`, `os-store-text-io`, `os-store-owned-schema-decode`, `os-store-member-open`, `os-store-engine-owners`, `os-store-codec-registry`, `os-store-initialization-runtime`, `os-store-engine`
* external crates named in the code: `semio_framework_value`, `pack`, `semio_framework_hash`, `protocol`, `semio_framework_dsl_record`, `semio_framework_artifact_reference`
* Retained typed hydration of a persisted Pack+SPR history (834-line impl, **peer-hot**), the mounted-pack session, document envelope materialization and the inline `mounted_pack_rt`.

#### 39. `os-store-member-open-store` (WP 5; 1062 lines, 76 KB, 2 files)

* node: `M/🏪️store/🧩️composition/🚪️open/🏭️operation`
* members (source, lines contributed): `🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs` 850, `🏪️store/🧩️composition/🚪️open/🦀️.rs` 200
* direct crate deps: `os-spr`, `os-store-child`, `os-store-ledger`, `os-store-read`, `os-store-text-io`, `os-store-member-open`, `os-store-engine-owners`, `os-store-initialization-runtime`, `os-store-engine`, `os-store-space-member`, `os-store-document`
* external crates named in the code: `semio_framework_value`, `semio_framework_schema_composition`, `semio_framework_hash`, `serde_json`, `semio_framework_job`, `semio_framework_trace`
* `InitialMemberStoreOpen` (460-line impl), `MemberStoreOpenRetained` (200), member snapshot open operation bodies. **Peer-hot** (hydration `C` batch).

#### 40. `os-store-config` (WP 5; 1243 lines, 76 KB, 2 files)

* node: `M/🏪️store/🎚️config`
* members (source, lines contributed): `🏪️store/🎚️config/📥️retained/🦀️.rs` 1032, `S/🦀️.rs [config region]` 181
* direct crate deps: `os-store-ledger`, `os-store-retirement`, `os-store-read`, `os-store-text-io`, `os-store-snapshot-clone`, `os-store-engine-owners`, `os-store-initialization-runtime`, `os-store-engine`, `os-store-document`
* external crates named in the code: `semio_framework_value`, `protocol`, `pack`
* Retained config-store hydration (613 + 222 + 222-line impls) and `ConfigEnvelope`/`ConfigStore`/`ConfigRecord`/`create_config_envelope`. The three config macros (168 lines, still counted in this crate's 1,243 by the tool) stay in the facade, so the crate is about 1,075 lines.

#### 41. `os-store-durable-group` (WP 5; 1862 lines, 117 KB, 1 files)

* node: `M/🏪️store/🧩️composition/🗄️durable-group`
* members (source, lines contributed): `🏪️store/🧩️composition/🗄️durable-group/🦀️.rs` 1862
* direct crate deps: `os-semio`, `os-store-ledger`, `os-store-one-item-publication`, `os-store-text-io`, `os-store-one-item-batch`, `os-store-durable-group-wire`, `os-store-engine-stage`, `os-store-engine`
* external crates named in the code: `semio_framework_pack_json`, `semio_framework_hash`, `semio_framework_artifact_reference`, `serde_json`, `semio_framework_value`, `semio_framework_dsl_record`, `semio_framework_dsl_record_derive`, `semio_framework_async`, `semio_framework_diagnostic`
* Journal records, the three-store map assembly/commit/recovery machinery (`DurableOwnedThreeStoreMapAssemblyV1` 320-line impl, `…CommitV1` 329), the decision codec. Test helpers behind `feature = "durable-group-testing"` (167 lines) stay here.

#### 42. `os-store-composition` (WP 5; 1482 lines, 90 KB, 5 files)

* node: `M/🏪️store/🧩️composition`
* members (source, lines contributed): `🏪️store/🧩️composition/🦀️.rs` 802, `🏪️store/🧩️composition/🌳️graph/🤝️group/🦀️.rs` 281, `🏪️store/🧩️composition/🌳️closure/🦀️.rs` 252, `🏪️store/🧩️composition/🌳️graph/🔗️links/🦀️.rs` 38, `🏪️store/🧩️composition/🌱️initial/🪪️identity/🦀️.rs` 3
* direct crate deps: `os-store-space-member`
* external crates named in the code: `semio_framework_artifact_reference`, `protocol`, `semio_framework_async`, `semio_framework_value`
* The composition coordinator (`TransactionCoordinator` 355-line impl, `CompositionGraph` 217), graph group/links, closure admission, initial child identity.

#### 43. `os-store-space` (WP 5; 323 lines, 20 KB, 2 files)

* node: `M/🏪️store/📜️space-history/🏠️space (new)`
* members (source, lines contributed): `S/🦀️.rs [space_members! macro and SpaceHost region]` 199, `S/🦀️.rs [member store functions]` 82, `S/🦀️.rs [link vocabulary region]` 15, `🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` 5
* direct crate deps: `os-store-space-schema`, `os-store-backbone`, `os-store-text-io`, `os-store-engine-owners`, `os-store-codec-registry`, `os-store-space-history`, `os-store-engine`, `os-store-initialization-replay`, `os-store-codec-factory`, `os-store-space-member`, `os-store-document`
* external crates named in the code: `semio_framework_artifact_reference`, `semio_framework_pack_json`
* `SpaceHost` (struct, `Drop`, 162-line impl, `MemberDirectory` impl), `space_history_store_owners`, `create_member_store`, `open_member_store`, `install_unscheduled_catalog`, `install_funded_member_store_owners`, `validate_member_history_identity`, `register_sqlite_snapshot` (L1).

#### 44. `os-store-test-support` (WP 5; 929 lines, 57 KB, 1 files)

* node: `M/🏪️store/🧪️test-support (new)`
* members (source, lines contributed): `S/🦀️.rs [test_support region]` 865
* direct crate deps: `os-spr`, `os-store-replay`, `os-store-read`, `os-store-one-item-publication`, `os-store-text-io`, `os-store-engine-owners`, `os-io`, `os-store-engine-core`, `os-store-engine`, `os-store-initialization-replay`, `os-store-space-member`, `os-store-document`, `os-store-config`, `os-store-space`
* external crates named in the code: `semio_framework_value`, `semio_framework_pack_json`, `semio_framework_schema_registry`, `serde`, `serde_json`, `semio_framework_job`, `semio_framework_artifact_reference`
* `pub mod test_support` (round-trip assertions shared by every technology crate's tests; 809 lines; used by 252 consumer paths) and `STORE_SCHEMA_EXPORTS` registration. A production module, not `cfg(test)`.

#### 45. `os-store-sync` (WP 6; 2092 lines, 114 KB, 2 files)

* node: `M/🏪️store/🔄️sync`
* members (source, lines contributed): `🏪️store/🔄️sync/🦀️.rs` 2082, `FW/🔨️modules/🏃️process/🧭️routing/🏠️workspace/🦀️.rs` 5
* direct crate deps: `os-store-backbone`, `os-store-text-io`, `os-store-codec-registry`, `os-store-engine`
* external crates named in the code: `semio_framework_value`, `semio_framework_pack_json`, `semio_framework_async`, `semio_framework_diagnostic`, `pack`, `semio_framework_value_derive`, `semio_framework_hash`, `vcs`
* Local-first sync actor layer, shared half: `SyncSession`, `ArtifactHost`, mailbox, document socket/link vocabulary and policies, persistence data class, backbone worker wire, command rejection codes. Optional dependency under feature `sync` and `not(all(wasm32, p2))`. Mounts `F/🏃️process/🧭️routing/🏠️workspace`.

#### 46. `os-store-sync-native` (WP 6; 2735 lines, 145 KB, 1 files)

* node: `M/🏪️store/🔄️sync/🖥️native (new)`
* members (source, lines contributed): `🏪️store/🔄️sync/🦀️.rs` 2735
* direct crate deps: `os-semio`, `os-store-ledger`, `os-store-backbone`, `os-store-text-io`, `os-store-codec-registry`, `os-store-engine`, `os-store-space-member`, `os-store-sync`
* external crates named in the code: `vcs`, `semio_framework_async`, `semio_framework_diagnostic`, `tokio`, `tokio_tungstenite`, `semio_framework_value`, `futures`, `semio_framework_os_services`, `semio_framework_hash`
* `mod native_actor` (`ArtifactActor` 1,416-line impl, runner, wake/reactor) and the folder event-log/text storage; `tokio`, `tokio-tungstenite`, `futures`, `ureq`, `semio-framework-os-services` (feature `ureq`). Statics `REACTOR`, `WRITERS`.

#### 47. `os-store-sync-wasm` (WP 6; 913 lines, 50 KB, 1 files)

* node: `M/🏪️store/🔄️sync/🌐️wasm (new)`
* members (source, lines contributed): `🏪️store/🔄️sync/🦀️.rs` 913
* direct crate deps: `os-store-ledger`, `os-store-backbone`, `os-store-codec-registry`, `os-store-sync`
* external crates named in the code: `semio_framework_async`, `semio_framework_diagnostic`, `js_sys`, `semio_framework_value`, `wasm_bindgen`, `vcs`, `tokio`
* `mod wasm_actor` (`WasmActor` 700-line impl); `js-sys`, `wasm-bindgen`, `web-sys`. The `worker` module (109 lines, `BackboneWorkerHost` with `#[wasm_bindgen]`) stays in the facade so the export is linked from the top crate.

#### 48. `os-spr-cli` (WP 1; 806 lines, 34 KB, 1 files)

* node: `M/📡️spr/⌨️cli`
* members (source, lines contributed): `📡️spr/⌨️cli/🦀️.rs` 719
* direct crate deps: `os-pack-tools`, `os-spr`
* external crates named in the code: none
* Native only: the `protocol` binary (inspect/verify/hash/log/compile/decompile/diff/compact/repair/upgrade). Its constants (`HEADER_SIZE`, `REC_*`, `MAGIC`) are its own; they collide by name with replication constants and are **not** a dependency edge (5.2).

#### 49. `os-kernel (facade)` (WP 0; 694 lines, 47 KB, 4 files)

* node: `K/📦️packages/🦀️rust`
* members (source, lines contributed): `S/🦀️.rs [space_members! macro and SpaceHost region]` 310, `OS/📦️packages/🦀️rust/🦀️.rs` 183, `FW/🔨️modules/🚪️io/📝️text/💡️inferences/🦀️.rs` 37, `FW/🔨️modules/🚪️io/📝️text/🦀️.rs` 2
* direct crate deps: `os-list-delta`, `os-store-replay`, `os-store-one-item-batch`, `os-store-member-open`, `os-store-engine-core`, `os-store-engine`, `os-store-space-member`, `os-store-member-open-store`, `os-store-space`
* external crates named in the code: `semio_framework_value`, `semio_framework_job`, `protocol`, `pack`, `semio_framework_pack_json`, `semio_framework_artifact_reference`, `semio_framework_composition_admission`, `semio_framework_os_pack`, `semio_framework_os_command`
* The thinned `semio-framework-os-kernel`: `extern crate self` aliases, the hub of explicit `pub use` (6.4; generated, in mounted export files), exported macros that name facade paths (`space_members!` 310 counted here; `field_set_mutations!` 132, `config_record!` 23, `config_diff!` 13 counted under `os-store-config` and `field_set_mutations_binary!`/`_text!` 38 under `os-store-text-io` by the tool, but they move here: ≈ 520 lines of macro text in total), the `worker` mount, bins, features and `[[test]]` targets, the test allocation observer. About 900 lines in total with those macros; budget under 1,000.


### 6.3 Crate-level imports the dissolved kernel aliases must be replaced by (token usage in the lib closure)

The corpus gate forbids the tokens `os_store`, `os_spr`, `os_directory`, `semio_framework_os_kernel` in producer sources, so every kernel-internal path becomes the owning external or new crate. Counts are occurrences of `crate::<head>` (and the `::semio_framework_os_kernel::<head>::` full-path form) in the lib closure; the last column names the crates that use the head most.

| old path head | occurrences | new path | most used by |
|---|---:|---|---|
| `crate::os_spr::X` | 808 | the defining crate listed in the `pub use` lines of `M/📡️spr/🦀️.rs`: `protocol::{format, history, causal, conflict, wire, crypto, ids, dictionary, scalar, io}` (`semio-framework-replication`), `semio_framework_os_command::{…, fold}`, `semio_framework_os_history`, `semio_framework_os_channel`; only `extract_range`, `verify_slice`, `content_frontier`, `RecordSlice`, `io`, `materialize` come from `os-spr` | text-io 111, engine-core 84, engine-live 82, engine 64, replay 54 |
| `crate::os_store::<module>::X` and the flat `os_store::X` | 418 | the crate of the module in 6.4 | member-open-store 46, codec-registry 39, member-open 25, the facade macros 172 (they stay) |
| `crate::os_vcs::X` | 171 | `semio_framework_os_vcs::X` (lib `vcs`) | initialization-runtime 26, replay 17, engine-stage 13, engine 12 |
| `crate::os_pack::X` | 146 | `semio_framework_os_pack::X` (`pack_rt`, codec re-exports); `cli`/`control` from `os-pack-tools` | pack-tools 35, spr 33, spr-cli 28, dsl 22 |
| `crate::sqlite_snapshot::X` | 99 | `semio_framework_io_sqlite_snapshot::X` | io 55, text-io 32, codec-registry 10 |
| `::semio_framework_os_kernel::os_store::…` (full path) | 92 | as the module row | durable-group 40, codec-factory 10, test-support 10, engine-stage 10 |
| `crate::io_schema::X` | 79 | `semio_framework_io_schema::X` | text-io 25, io 23, codec-registry 22 |
| `crate::os_directory::X` | 65 | `semio_framework_os_directory[::client]` | sync 36, sync-native 17, sync-wasm 8 |
| `crate::list_delta::X` | 62 | `crate::X` inside `os-list-delta`, `semio_framework_os_list_delta::X` elsewhere | list-delta 62 |
| `crate::__value`, `__pack_json`, `__value_derive`, `__dsl_record_derive`, `__diagnostic` | 71 | the external crate itself (`semio_framework_value`, `…pack_json`, …); the hidden re-exports remain in the facade for macro expansions in consumers | config 25, text-io 17, list-delta 26 |
| `crate::os_dsl::X` | 21 | `semio_framework_os_dsl::X` | dsl 16, backbone 2, durable-group-wire 2 |
| `crate::io::X` | 18 | `semio_framework_os_io::X` | initialization-replay 4, text-io 4, codec-registry 4 |
| `crate::os_identity`, `crate::os_semio`, `crate::os_extension` | 3 and a few | the extracted or new crate | initialization-runtime, extension |

Every kernel-internal `use super::*` (16 files) becomes explicit imports; the dissolved `extern crate self as store/dsl/spr/vcs/dsl_grammar/semio_format` aliases are not recreated in any new crate: the facade keeps them for the derives' and macros' emitted paths and for consumers.

### 6.4 Module path -> crate (what the facade's `pub use` hub preserves)

Consumers reach the Store through the **owning module path** `os_store::<module>::X` (E1b removed the flat re-exports), through flat names that `K/🦀️.rs` glob-re-exports from `os_dsl`, `os_spr`, `os_pack`, `os_vcs`, `os_inference` and `os_store`, and through four package aliases. The facade replaces `pub use component::*` by explicit, tool-generated `pub use` lists (the pack carve's `pubnames` method): a module backed by one crate becomes `pub use <crate> as <module>;`, a module backed by several crates becomes `pub mod <module> { pub use a::{…}; pub use b::{…}; }`.

| public path | crate(s) |
|---|---|
| `os_store::artifact_child` | `os-store-child` |
| `os_store::backbone` | `os-store-backbone` |
| `os_store::codec_registry` | `os-store-codec-registry` |
| `os_store::composition_coordinator` | `os-store-composition` |
| `os_store::document_materialize` | `os-store-document` (+ `now_iso`, `uncommitted_edit_ids`, `EMPTY_CHECKPOINT_MESSAGE` from `engine-core`, `stamp_primary_operation_identity` from `engine-live`, `fresh_replica_clock` from `initialization-runtime`, `edit_ids_for_changes`, `materialize_document_snapshot` from `space-member`) |
| `os_store::document_store_owners` | `os-store-engine-owners` |
| `os_store::durable_group` | `os-store-durable-group-wire` + `os-store-engine-stage` + `os-store-durable-group` |
| `os_store::ledger` | `os-store-ledger` |
| `os_store::member_open` | `os-store-member-open` + `os-store-member-open-store` (+ factory selection in `os-store-space-member`) |
| `os_store::one_item_publication` | `os-store-one-item-publication` + `os-store-one-item-batch` |
| `os_store::owned_schema_decode` | `os-store-owned-schema-fields` + `os-store-owned-schema-decode` |
| `os_store::replay` | `os-store-replay` (+ `EffectiveSupersessions`, `FoldProjection` now in `os-store-ledger`) |
| `os_store::serialization`, `os_store::text_format` | `os-store-text-io` |
| `os_store::snapshot_clone_preparation` | `os-store-snapshot-clone` |
| `os_store::snapshot_read` | `os-store-read` (+ retirement helpers now in `os-store-ledger` and `os-store-replay`) |
| `os_store::space_history`, `space_history_document`, `store_schema` | `os-store-space-history`, `os-store-space-schema`, `os-store-space-schema` |
| `os_store::mounted_pack_rt`, `os_store::test_support` | `os-store-document`, `os-store-test-support` |
| `os_store::sync` (feature `sync`) | `os-store-sync` + `-sync-native` + `-sync-wasm` |
| `os_store::worker` (features `worker`, wasm32-unknown) | stays in the facade (the `#[wasm_bindgen]` class `BackboneWorkerHost`) |
| `os_store::<name>` (flat names of the root: `ArtifactStore`, `SpaceHost`, `SpaceMember`, `CommandReceipt`, `ConfigStore`, `artifact_codec`, …) | explicit per-name lists to `os-store-engine`, `-space`, `-space-member`, `-engine-core`, `-config`, `-codec-factory` |
| `io`, `io::bindings`, `io::testing`, `io::text` | `os-io`; `io::bindings` and the composer registry from `os-store-codec-registry`; `io::text` is the framework mount `F/🚪️io/📝️text` |
| `list_delta`, `os_semio`, `os_extension`, `native_catalog`, `os_plugin_module_schema` | `os-list-delta`, `os-semio`, `os-extension`, `os-plugin-registry-schema` (both mounts) |
| `os_dsl::{component, grammar, lsp, registry, family::*}` | `os-dsl` |
| `os_pack::{cli, control}` (native) | `os-pack-tools` |
| `os_spr::{component, io, materialize}`, `os_spr::cli` (native), `os_spr::protocol_laws` (feature `protocol-laws`) | `os-spr`, `os-spr-cli`, `os-spr` |

The 41 private child modules need no public path; they follow their files. Registered consumer paths that would break are exactly the module rows whose backing crate list has more than one entry: for those the explicit lists must be diffed against the old surface (section 11, `pubnames`).

### 6.5 Statics and thread-locals

22 `static` items and one `thread_local!` in the lib closure. The item graph shows **no cross-crate reference to a static itself** (they are reached through accessor functions in the same crate), so no static needs a second owner.

| crate | static / thread-local |
|---|---|
| `os-store-codec-registry` (8) | `DOCUMENT_CODEC_REGISTRY`, `COMPONENT_DOCUMENT_CODEC_REGISTRY`, `DIALECT_MIGRATION_REGISTRY`, `IO_REGISTRY`, `SUBSET_VALIDATOR_REGISTRY`, `FORMAT_CATALOG`, `CATALOG`, `BINDINGS` |
| `os-io` (4) | `IO_FALLBACK`, `INTERNED`, `IO_MECHANISM_REGISTRY`, `NATIVE_SNAPSHOT_REGISTRY` |
| `os-store-sync-native` (2) | `REACTOR`, `WRITERS` |
| `os-store-engine-core` (1 + 1) | `CENSUS` and one `thread_local!` (both in `hot_path_census`, `cfg(test)`) |
| `os-semio`, `os-plugin-registry-schema`, `os-dsl`, `os-store-backbone`, `os-store-engine-live`, `os-store-test-support`, facade | `STATE`, `INSTALLED_NATIVE_CATALOG`, `SCHEMA_REGISTRY`, `HOST_BACKBONE_PORT`, `TURNS` (test), `TEMP_DIR_SEQUENCE`, `TEST_ALLOCATION_OBSERVER` (the `#[global_allocator]` of the facade's own test binary and of each companion that wants it) |

Singleton hazard to test after carving: the six registries of `codec-registry` and the four of `io` must each exist once (the oracle's duplicate-name scan finds a re-mounted file); `#[global_allocator]` may appear once per test binary, so it stays in the facade and in each companion test crate that wants the allocation witness.

### 6.6 Macros

14 `#[macro_export]` macros plus 19 private `macro_rules!`. The private ones are file-local and move with their file. The exported ones:

| macro | defined in | `$crate` refs | destination | rule |
|---|---|---:|---|---|
| `field_set_mutations!` | `S/🦀️.rs` (124 lines) | 39 | facade | names `$crate::__value::*`, `$crate::snapshot_clone_preparation::ConfigApplyMutation`, `$crate::Mutation…` |
| `config_record!`, `config_diff!` | `S/🦀️.rs` | 3, 4 | facade | name `$crate::ConfigRecord`, `$crate::sparse_record_diff` |
| `space_members!` | `S/🦀️.rs` (290 lines) | 196 | facade | names `$crate::os_store::{SpaceMember, VcsError, one_item_publication, member_open, snapshot_read, …}`, `$crate::os_vcs::io` |
| `field_set_mutations_binary!` | `S/🚪️io/💾️binary/🧬️mutations/🎚️fields/🦀️.rs` (13 lines) | 11 | facade (mounted by `#[path]`) | `$crate::os_pack::pack_rt`, `$crate::ProtocolError` |
| `field_set_mutations_text!` | `S/🚪️io/📝️text/🧬️mutations/🎚️fields/🦀️.rs` (25 lines) | 19 | facade (mounted by `#[path]`) | `$crate::__pack_json`, `$crate::__diagnostic` |
| `original_ordinal_operation_receiving!`, `original_tagged_operation_receiving!` | `M/🗣️dsl/🫴️receiving` | 6, 10 | `os-dsl` | `$crate::os_dsl::original_receiving` -> `$crate::original_receiving` |
| `list_delta!`, `plain_list_delta!`, `row_patch!` | `M/🪡️list-delta` | 41, 54, 7 | `os-list-delta` | `$crate::list_delta::X` -> `$crate::X`; hidden re-exports of the two derive crates |
| `compose_thunk!`, `io_run_thunk!`, `io_sniff_thunk!` | `M/🚪️io/🦀️.rs` | 2, 3, 2 | `os-io` | `$crate::io::X` -> `$crate::X`; `$crate::io_schema` becomes a hidden re-export |

Rule: a macro whose expansion names facade paths must keep resolving through the facade, because `$crate` is the defining crate and the consumers' expansions (`store::space_members!`, `field_set_mutations!`, …) must not need new dependencies. The facade therefore owns ≈ 520 lines of macro text. Every macro gets one expansion test in the facade that invokes it with the paths a consumer uses (the existing hub and plugin call sites are the oracle).

## 7. `ArtifactStore` and the other large impls

### 7.1 What exists

`S/🦀️.rs` defines `pub struct ArtifactStore<P, Mutation>` with **46 fields** (119 lines with docs; 24 of them `ManuallyDrop`, the rest plain) and **four inherent `impl` blocks in the same file**: lines 2409 to 8501 (6,093 lines, 348 methods) and three small ones of 12, 10 and 52 lines; together **353 methods / 5,780 lines, 152 `pub`** (`impl Drop` 72 lines, `impl SpaceMember for ArtifactStore` 478 and the member-group impl in `S/🧩️composition/📬️publication/🤝️group` 149 are separate). Inside the big impl live seven `//#region` blocks (`TimeTravelReads` 256 lines, `OneItemPublication` 777, `EventSourcedHistory` 978, `ToolTransactions` 170, `DeferredReprojection` 254, `Supersede` 324, `HistoryLane` 209); they are repeated, balanced, in every file a region is cut across.

An inherent impl cannot span crates, and 5,780 lines (421,524 bytes with docs) cannot fit one crate, so the brief's god-object rule applies: components own their fields and their impl in their own crate, the old struct becomes a thin owner, no extension traits.

Measured coupling (direct `self.field` accesses): 82 of the 353 methods touch no field (they call other methods or are helpers), 151 touch exactly one of the six field groups below, 72 two, 30 three, 9 four, 5 five and 4 six. Call level: 316 method names are unique among all `fn` names of the closure, so `self.m()` / `Self::m()` calls resolve by name. Outside the impl, **about 95 private-field accesses sit in the durable-group staging functions** (`durable_group_root` 18, `revision_accumulator` 14, `content_revision` 12, `applied_edit_ids` 11, `displaced_retirements` 10, `tail_undo_cache` 5, `snapshot_retirement_factory` 5, …) and about 12 in hydration, config and text format; unit tests read about 40 more.

### 7.2 Design rules (binding for WP5)

1. **One component struct per field group** (7.3). A component owns its fields, an invariant-preserving API and every method whose touched components are all at or below it in rank. Field names do not change (`self.log.applied_edit_ids` in the composer, `self.applied_edit_ids` inside `StoreLog`), which keeps the rewrite mechanical.
2. **A method that needs fields of lower components takes them as parameters**, in rank order after `&self`/`&mut self`: `&mut Lower` if it mutates, `&Lower` otherwise. Of the 297 methods that end in components 163 need no foreign parameter, 93 need one, 27 two, 13 three and one four.
3. **Methods that touch five or more components, or that call a higher-ranked method, are use-case methods** and stay in the composer (56 methods, 1,857 lines), which reaches the components through split borrows of its own `pub(crate)` component fields or through `StoreParts` (C3). Never `&mut ArtifactStore` as a parameter of anything below the composer.
4. **The composer `ArtifactStore`** is exactly: the six component fields, `new`/`construct`/seed/adopt (they build components), `impl Drop` (asserts terminal-emptiness through the components' `terminal_is_empty`), the 56 use-case methods, **125 delegates** for the `pub` methods homed in components, the `ArtifactStoreCloseHost` implementation (C1) and the member-group impl. It has no other behaviour.
5. **Upward calls move the caller up** (always dependency-safe), except the one port C1. A method may be lifted; it may not be pushed down past a component whose fields it touches.
6. **Generic bounds.** The old impl blocks carry four different `where` clauses (`P: Clone + ToValue + FromValue`; `+ Send + Sync + 'static`; `+ PartialEq`; `+ ArtifactPack + ArtifactStoreSnapshot`). Each component struct carries the minimal clause of its fields; each method keeps the clause it has today; WP5 lists the clause per method in its RESULT file.
7. **`ManuallyDrop` fields keep their types and their order inside their component**; the field-by-field drop order of the old struct is reproduced by the composer's `Drop` calling the components in reverse rank order.

### 7.3 Components (field groups) and where they live

Rank order (low to high) is the order of the table and the crate dependency order of the three engine crates.

| struct | crate | rank | fields | field names | methods homed / lines | of which `pub` (delegated by the composer) |
|---|---|---:|---:|---|---:|---:|
| `StoreLog` | `os-store-engine-core` | 1 | 13 | `envelope`, `envelope_detached`, `dag`, `applied_edit_ids`, `redo_edit_ids`, `edit_sequence`, `current_checkpoint_id`, `local_actor_id`, `merge_policy`, `clock`, `authoring_verb`, `initial_digest`, `operation_wire` | 59 / 407 | 29 |
| `StoreOwners` | `os-store-engine-core` | 2 | 11 | `snapshot_retirement_factory`, `initial_snapshot_retirement_factory`, `mutation_retirement_factory`, `snapshot_read_leases`, `displaced_retirements`, `owned_disposer`, `owned_disposer_terminal`, `factory_retirement_tickets`, `one_item_preparation_factory`, `replay_preparation_factory`, `one_item_wire_preparation_factory` | 56 / 721 | 26 |
| `StoreWire` | `os-store-engine-core` | 3 | 3 | `backbone`, `member_inbox`, `durable_group_root` | 17 / 190 | 6 |
| `StoreCursor` | `os-store-engine-core` | 4 | 6 | `content_revision`, `revision_accumulator`, `revision_dirty_from`, `mirror_dirt`, `generation`, `last_projection_cause` | 39 / 473 | 22 |
| `StoreLive` | `os-store-engine-live` | 5 | 9 | `current`, `current_detached`, `tail_undo_cache`, `supersessions`, `prefix_ring`, `prefix_ring_dirty_from`, `fold_frontier`, `tail_step_generation`, `snapshot_equality` | 78 / 1196 | 25 |
| `StoreReplay` | `os-store-engine-live` | 6 | 4 | `replay_budget`, `local_replay_budget`, `pending_reprojection`, `pending_report` | 48 / 936 | 17 |
| `ArtifactStore` (composer) | `os-store-engine` | 7 | 6 components | (the six structs above) | 56 / 1857 use-case methods | 27 |

The groups follow the data: the *retirement* fields (`displaced_retirements` alone is touched by 54 methods) form the owner discipline that every value swap pays into; the *log* fields are the event-sourced history; the *live* fields are the incremental fold and its prefix ring; *replay* is the deferred reprojection state; *wire* is the backbone transport plus the durable-group root; *cursor* is the revision mirror (`bump` has 22 callers and touches four components).

### 7.4 Method homing (algorithm, so WP5 reproduces it)

1. For each of the 353 methods compute `C(m)` = the components whose fields `m` reads or writes (`self.f`, `f` in the table of 7.3).
2. If `|C(m)| >= 5` the method is a **use-case method** (composer). Otherwise it goes to the **highest-ranked component in `C(m)`**; the other components of `C(m)` become parameters.
3. Fixed point: while a method calls (by `self.name(…)`/`Self::name(…)`, unique name) a method homed in a higher-ranked crate, lift the caller to that home. Helpers with no field access and no callee go to the lowest-ranked home among their callers (history if none).
4. `new`, `construct`, `seed_runtime_state`, `adopt_runtime_seed`, `preflight_runtime_seed` and `from_initialized_runtime_with_owners` are pinned to the composer (they build components).
5. The rank order and the threshold were chosen by exhaustive search over all 720 rank permutations of the six components with `K` (use-case threshold) in {3, 4}, minimising the largest crate and then the sum of squares: `history < retire < transport < cursor < projection < replay` and `K = 4` (that is `|C| >= 5` goes to the composer). With `K = 3` the composer would be 2,742 lines of methods, with `K = 5` the live crate 3,645 lines.

Result (methods / lines): `StoreLog` 59 / 407, `StoreOwners` 56 / 721, `StoreWire` 17 / 190, `StoreCursor` 39 / 473 (crate `engine-core`: 171 methods, 1,791 lines), `StoreLive` 78 / 1,196, `StoreReplay` 48 / 936 (crate `engine-live`: 126, 2,132), composer 56 / 1,857. **The full machine assignment of every method is Appendix B.** If a crate overflows its budget while carving, its highest-fan-in lifted methods spill to the next crate up (always safe); the composer's spill valve is in its spec (6.2).

### 7.5 Ports and the bundle

* **C1 `ArtifactStoreCloseHost`** (5.3): about 14 methods; one implementor (the composer); consumers of the trait object are the owner disposers installed through `DocumentStoreOwners`.
* **C3 `StoreParts<'a, P, Mutation>`**: six `&'a mut` component references, defined in `engine-live`, built by `ArtifactStore::parts(&mut self)`, taken by `engine-stage` and by the large use-case methods that must split borrows.
* **Delegates.** A delegate has the old signature and body `self.<component>.<method>(&mut self.<lower>, …)`; each carries `#[inline]`. 125 of them at about 4.8 lines is ≈ 600 lines, the reason `os-store-engine` is not 2,607 but ≈ 3,200 lines. Generic methods are monomorphised in the calling crate, so cross-crate inlining of `P`/`Mutation`-generic code is unaffected; non-generic helpers called on the hot path (`bump`, `fold_is_live`, the prefix-ring helpers) get `#[inline]` too (the `⚡️hot-path` tests in `S/🧪️tests` are the regression oracle).

### 7.6 Free items and external callers that name `ArtifactStore`

| item | lines | decision |
|---|---:|---|
| `ArtifactStoreCloseView`, `ArtifactStoreOwnedDisposer`, `ArtifactStoreCursorDisposer` | 7 + 19 + 8 + 81 + 17 | `engine-owners` over `ArtifactStoreCloseHost` (C1) |
| `config_store_from_initialized_runtime_with_owners` | 60 | composer (`engine`); builds components |
| durable-group staging (`stage_store_member`, `stage_prebuilt_batch_root`, `abort_staged_store_member`, `adopt_staged_store_member`, `validate_adopted_store_member`, `clear_adopted_store_member`, `retire_unstaged_store_member`, `captured_store_snapshot`) and the store-touching outcome impls | 636 | `engine-stage` over `StoreParts`; `ArtifactStoreDurableGroupRootV1` moves to `durable-group-wire` so `StoreWire` can hold it |
| member-group impl on `ArtifactStore` | 149 | composer use-case methods calling `engine-stage` |
| `impl SpaceMember for ArtifactStore` | 478 | `space-member` (trait there, type in `engine`: legal because `engine` is a dependency) |
| `retire_document_envelope`, hydration/config reads of `applied_edit_ids`, `redo_edit_ids` | ≈ 12 accesses | public read accessors on `StoreLog` (`applied_edit_ids()`, …) reached through `ArtifactStore::envelope_view`-style composer methods |
| unit tests with private field reads (`applied_edit_ids`, `tail_undo_cache`, `snapshot_retirement_factory`, `owned_disposer`, …) | ≈ 40 accesses | `#[cfg(test)]`-only accessors on the components (E1b's rule: no `pub` shim) |

### 7.7 Other large impls (explicit)

Every other multi-hundred-line inherent impl sits whole in one crate after the splits of 5.2; the resolver's impl-placement check (E0116) reports no type whose impl spans crates after the 22 normalisation moves (initialization runtime/catalog impls, prepared retirement, `ReplayOwnedState`, `ArtifactStoreHistoryMetadataRetirement`, `UnsupportedMemberFactoryOpen`).

| type | impl lines | crate |
|---|---:|---|
| `ArtifactActor` | 1,416 | `os-store-sync-native` |
| `RetainedPersistedDocumentHydration` | 834 (+ 46 + 44 + 114 trait) | `os-store-document` (peer-hot) |
| `WasmActor` | 700 | `os-store-sync-wasm` |
| `RetainedConfigStoreHydration` | 613 + 222 | `os-store-config` |
| `InitialMemberStoreOpen` | 460 | `os-store-member-open-store` |
| `ArtifactStoreReplayInitializer` | 626 | `os-store-initialization-replay` |
| `ArtifactEnvelopeDecodeAuthority` | 353 + 73 (`InteractiveJob`) | `os-store-owned-schema-decode` |
| `TransactionCoordinator` | 355 | `os-store-composition` |
| `ArtifactStoreInitializationRuntime` | 361 (+ 72 moved in) | `os-store-initialization-runtime` |
| `DurableOwnedThreeStoreMapAssemblyV1` / `…CommitV1` | 320 / 329 | `os-store-durable-group` |
| `ArtifactEnvelopeFreshVcsAuthority` | 307 + 177 | `os-store-owned-schema-decode` |
| `DocumentStoreOwners` | 328 | `os-store-engine-owners` |
| `MemberOpenRequest` | 272 | `os-store-member-open` |
| `ArtifactStoreBatchPublication` | 243 | `os-store-one-item-batch` |
| `ArtifactStoreOneItemLiveAuthority` / `…Sealer` | 211 / 192 + 53 + 86 | `os-store-one-item-publication` |
| `SpaceHost` | 162 | `os-store-space` |

## 8. Public surface, features, targets, consumers

### 8.1 What the facade is

`semio-framework-os-kernel` keeps its name, lib name `semio_framework_os_kernel`, manifest path and package location; it becomes a thin crate (about 900 lines: macros ≈ 520, the root's own items 183, aliases, `worker` mount 109, test observer, framework mounts; the generated export lists live in mounted files):

* the seven `extern crate self as …` aliases and the browser `wasm_bindgen_futures` alias (the derives and the six facade macros emit paths through them);
* the **hub**: `pub mod os_dsl { … }`, `pub mod os_pack { … }`, `pub mod os_spr { … }`, `pub mod os_store { … }` and the flat re-exports, written as explicit `pub use` lists generated from the old public surface (6.4); long lists live in mounted `📚️exports/<module>/🦀️.rs` files so the lib root stays inside the taxonomy's 150-line `libWiringLineBudget`;
* the six exported macros that name facade paths (6.6) and the `worker` module (`BackboneWorkerHost` with `#[wasm_bindgen]`, 109 lines, feature `worker`, wasm32-unknown only);
* `[[bin]]` `pack`, `spr`, `semio` behind `native-bin` (their sources call into `os-pack-tools`, `os-spr-cli`, `os-semio`), `[[test]]` `sqlite_snapshot_native_admission` and `artifact_command_retirement` (or moved to companions), the features below and the 68 dependency keys reduced to the 48 new crates plus the externals only the facade names.

### 8.2 Features

| facade feature | forwards to |
|---|---|
| `default = ["deflate"]`, `deflate` | `os-history/deflate`, `replication/deflate`, `pack/deflate` (unchanged) |
| `sync` | optional deps `os-store-sync`, and in target tables `os-store-sync-native` (`not(wasm32)`), `os-store-sync-wasm` (`all(wasm32, not(p2))`); `os-directory-client/sync`, `tokio/rt`, `tokio/time`, `dep:semio-framework-os-services`, `dep:ureq` |
| `ureq` | `os-directory-client/ureq` (unchanged) |
| `worker` | `["sync"]` plus the facade's own `worker` mount |
| `protocol-laws`, `record-codec-laws` | `os-spr/protocol-laws` (mounts `🧪️tests/⚖️protocol-laws` into that lib), `pack/record-codec-laws` |
| `mutation-testing` | `os-command/mutation-testing`, `os-spr/mutation-testing` (`mutation_fixture_ops`) |
| `durable-group-testing` | `os-store-durable-group/testing` (167 lines of test helpers) |
| `native-bin`, `typegen` | unchanged (bins, no dependency) |

Consumers enable `mutation-testing` 791 times, `protocol-laws` 715, `sync` 119, `ureq` 68, `durable-group-testing` 2 (1,933 dependency rows, measured), so the forwarding is exercised by almost every dev build.

### 8.3 Targets (native, `wasm32-unknown-unknown`, `wasm32-wasip2`)

`target_arch = "wasm32"` is true for wasip2, so every gate below is the full `all(target_arch="wasm32", not(target_env="p2"))` form, as in the kernel manifest today.

| group | crates | native | wasm32-unknown | wasm32-wasip2 |
|---|---|:-:|:-:|:-:|
| everywhere | 41 crates: every crate except the next four rows | yes | yes | yes |
| partly native | `os-spr` (`io` module), `os-dsl` (`registry`), `os-store-member-open` and `os-store-document` (one cfg-split file each) | full | minus native-only files | minus native-only files |
| native only | `os-pack-tools`, `os-spr-cli`, `os-store-sync-native` | yes | no | no |
| not wasip2 | `os-extension`, `os-store-sync` | yes | yes | no |
| wasm-only | `os-store-sync-wasm` (`all(wasm32, not(p2))`) | no | yes | no |

The wasip2 lib closure is 175 files / 52,474 lines today and 42 of 48 new crates after the split. Guests link `tokio` nowhere: `tokio`, `futures`, `ureq`, `tokio-tungstenite`, `semio-framework-os-services` appear only in `sync`, `sync-native`; `js-sys`, `wasm-bindgen`, `web-sys` only in `sync-wasm`, `backbone` (web-sys, two `window()` uses), `initialization-runtime` (one `js_sys::Date::now`) and the facade `worker`. Feature unification can still activate `sync` inside a wasip2 graph (`🌉️mcp`, the wgpu renderer), which is exactly why the facade must keep the sync crates in the `not(all(wasm32, p2))` target table as optional dependencies; gate: `cargo metadata --filter-platform wasm32-wasip2` of a guest with `sync` on shows none of the three sync crates.

wasm exports: the lib has exactly one `#[wasm_bindgen]` class (`BackboneWorkerHost`, `handleRequestBytes`, `postReady`); it stays in the facade so the export survives linking from the top crate. Export parity (WP0): the `.d.ts` of the built worker before and after must list the same names.

### 8.4 Consumer impact

2,857 Rust files name `semio_framework_os_kernel`; the first path segments they use are `io` (1,379 occurrences), `os_vcs` (1,088), `os_store` (903), `sqlite_snapshot` (802), `os_dsl` (328), `io_schema` (305), `os_directory` (282), `ActorId` (203), `LOCAL_ACTOR_ID` (191), `NativeSnapshotEncodeOwner` (190), `NativeSnapshotDecodeOwner` (158), `os_pack` (124), `DslArtifact` (118), `Viewport2d` (112); inside `os_store`: `serialization` 434+, `test_support` 252, `document_store_owners` 41, `codec_registry`, `ArtifactSnapshotPublication` 105. The package aliases are `protocol` (1,047 files), `store` (844), `dsl` (718), `vcs` (124). **All of these keep resolving through the facade**: no Rust consumer file, no manifest and no derive/macro call site changes. What changes for consumers: nothing in source; build graph (the facade now depends on 48 crates), lockfiles (176 lockfiles and 378 manifests name the kernel; the new members enter the framework workspace, WP0), and 40 text-pinned scanners (8.5).

### 8.5 Source-text readers that name the store by path

About 40 files name `S/🦀️.rs` (23 TS/JSON, 15 Rust path references, 1 manifest). The ones that read it as text and must follow the code: the root `📜️script.ts` tool-job policies (`policyReadOsStoreSource`, `policyReadOsStoreEvidence`, `OS_STORE_ROOT_SOURCES`, `policyReadMemberStoreProducerSources` near line 16,893), `K/📜️script.ts` and `M/🖥️host/…/📜️script.ts`, `F/📚️library/🔍️discovery/🟦️.ts` (`POLICY_SPLIT_SOURCE_ROOTS`), `F/📚️library/⚡️caching/…/⚡️cache-contracts/🟦️.ts`, the member source guard (`S/🏗️initialization/🏠️owners/🏭️member/📥️source/{🔣️.json,🧬️schema/🔣️.json,🟦️.ts}`), plugin tests (`⏪️time-travel/♻️retirement`, `🧪️reactor-contract-oracles`, `🔬️tool-job-latest-wins`, `♻️lifecycle/🏪️stores`), replication tests (`🌐️foreign`, `🔀️transition/📝️drafts`), `F/🧬️schema/🧩️composition/📥️admission` (`📋️callers.json`), `F/🛠️tool-machine/🧪️history-closure-policy`, `S/🧩️composition/📨️emission/📦️owned/🧪️tests`, `S/🧵️canonical-edit/🧪️tests/🔬️canonical-error-progress`, and the OS corpus `🧬️cuts.json`. Each re-points to the owning crate's lib root through `readModuleTreeSource` (as the pack and renderer carves did) or to a child-specific read; the TS suites to re-run are listed in section 11. About 15 plugin editor/support `.rs` files mention the path only in comments. Separately, `K/📋️project.json` declares **93 Nx targets**, all `bun ./📜️script.ts <name>` (for example `test-returned-read-custody`, `test-member-group-publication-source`, `test-snapshot-sqlite-admission`, `retained-clone-check`, `database-history-completion-check`), and `K/📜️script.ts` (226 KB, 17 cargo invocations with test selectors) runs cargo test filters and source-law checks for tests that move to other crates: WP0 builds the selector table (target -> owning crate and test path) and re-points each target to the owning package's `📜️script.ts`, as the pack carve did for its facade; project.json files of the new crates declare no targets.

## 9. Tests

### 9.1 What there is

The test-only closure is **194 source files / 44,720 lines / 3.29 MB plus 203 embedded fixtures / 1.85 MB**; the `--lib` suite is 1,465 tests (E1/E1b one-process-per-test sweeps), plus 72 integration tests (`sqlite_snapshot_native_admission` 70, `artifact_command_retirement` 2). Largest: `S/🧪️tests/🔬️unit/🦀️.rs` 12,625 lines / 1,006 KB (381 `fn` items, 69 `#[test]`, 101 impl blocks of demo owners and preparation doubles), `S/🔄️sync/🧪️tests/🔬️unit` 2,811 lines / 188 KB, `S/🧪️tests/🧪️supersede-replay` 1,459 / 108 KB, `S/🧩️composition/🗄️durable-group/🧪️tests/🔬️unit` 1,285 / 82 KB, `M/🗣️dsl/🧪️tests/🔬️unit` 1,130 / 69 KB, the spr protocol laws 1,114 + 911 lines. Embedded: the DSL grammar originals fixture is **1,126 KB in one file**; only nine `include_str!("….rs")` source-text embeds exist (3 in the store root, 2 in the sync echo-suppression test, 2 in the unit file, 1 plugin-registry, 1 standalone) and one `concat!(include_str!…)`.

### 9.2 Rules

1. **Placement**: a test lives in the crate that mounts it when the crate has budget for it and its subject is that crate or lower; otherwise in a companion `x-tests` crate placed above everything it exercises (dev-only, `publish = false`, lib root only `#[cfg(test)]` mounts, depends on production crates through `pub` API only; decision D4). Root-mounted store tests are placed by subject: the crate whose items (at least 60 lines or 15 percent of the names) the test uses most, then packed (9.4).
2. **Visibility**: tests that used `use super::*` private access get the narrowest `pub` on exactly the items they name (the definition scan finds them); never a `#[cfg(test)] pub` shim for a production type; `#[cfg(test)]` accessors on components are allowed (7.6).
3. **Source-text laws read at run time**: the nine embeds read `std::fs` through `env!("CARGO_MANIFEST_DIR")` with an explicit file list per law; module-tree reads belong to the TS side (`readModuleTreeSource`). The 1,126 KB DSL originals fixture is read at run time (`std::fs`), never embedded. This removes 1.13 MB from the test closure of `os-dsl` (from 1,398 KB to 272 KB with its lib).
4. **Fixtures** stay beside their tests and are counted (file and bytes). The byte-tight groups are the DSL originals (above), the space-history snapshot tests (163 KB with fixtures), `supersede-replay` (120 KB), `viewer-head` (96 KB with 44 KB fixtures).
5. **Tiny files**: about 50 mutation-fixture files (`S/🧪️testing/🧬️mutations/**` and `S/🧪️tests/🧪️fixture-*`, 6 to 66 lines each, 880 lines total) exceed the 28-file budget if carried by one crate: WP8 consolidates them into at most 12 files in the shared dev crate `os-store-testing-fixtures`, which every store test bin depends on (the consolidation is a mechanical merge of `#[path]` mounts).
6. **Global allocator**: `#[global_allocator] TEST_ALLOCATION_OBSERVER` stays in the facade and in each companion that wants the witness (one per test binary).
7. **Count gate**: the sum of `cargo test -- --list` over the 67 packages equals the old 1,465 lib tests plus the 72 integration tests; a missing test is a failure.

### 9.3 Placement of the unit tests that stay in their crate (host-mounted)

The tests column of 6.1 is this table: the tests mounted by a lib file of the crate, in lines / KB (embedded fixtures included) / files. Totals with the lib closure stay below 3,500 lines / 230 KB / 28 files for every crate except the ones named in 9.4, whose host-mounted tests move to companions: `os-dsl` (2,471 lines + 1,150 KB of fixtures), `os-store-member-open` (1,499 lines / 134 KB, 249 KB total), `os-store-durable-group` (1,285 / 110 KB, 227 KB total, 1 percent headroom), `os-store-sync` family (4,243 / 316 KB), and the facade (3,780 lines / 222 KB mounted at `K/🦀️.rs`: spr protocol laws, DSL fixture sweep, mutation laws). Tight but kept in place: `os-store-one-item-publication` (2,401 lines / 225 KB: move `🧵️borrowed/🧪️tests` (421 lines) to `os-store-canonical-json`, which is the subject-based home) and `os-io` (2,265 / 202 KB).

### 9.4 Companion test crates (18)

| companion (`semio-framework-…`) | content | lines / KB / files |
|---|---|---|
| `os-store-testing-fixtures` | the consolidated demo/lossy/severity/timestamped/validated mutation fixtures and `fixture_mutations` | ≈ 880 / 38 / ≤ 12 |
| `os-store-tests-01` | `unit file chunk 1/6` 2104, `📸️typed-snapshot` 97, `🔬️test-support-scratch-directory` 15 | 2216 / 213 / 4 |
| `os-store-tests-02` | `unit file chunk 2/6` 2104, `🧪️supersede-law` 394, `🪶️snapshot-capability` 35 | 2533 / 213 / 5 |
| `os-store-tests-03` | `unit file chunk 3/6` 2104, `🪶️native-decoding` 348, `📦️catalog` 86 | 2538 / 214 / 6 |
| `os-store-tests-04` | `unit file chunk 4/6` 2104, `🤝️group` 195, `📏️preflight` 94 | 2393 / 213 / 4 |
| `os-store-tests-05` | `unit file chunk 5/6` 2104, `✂️clamp` 149, `⚡️hot-path` 111, `🧩️schema-composition` 29 | 2393 / 214 / 6 |
| `os-store-tests-06` | `unit file chunk 6/6` 2104, `🧪️replay-retirement` 302, `🧩️composed-pack-schema` 162 | 2568 / 214 / 5 |
| `os-store-tests-07` | `📸️snapshot` 707, `🚫️rejected-page-close` 567, `✂️detach` 96 | 1370 / 214 / 13 |
| `os-store-tests-08` | `🧪️supersede-replay` 1459, `🪶️native-encoding` 745, `🛫️encoding` 230, `🛂️validation` 39 + 1 more | 2489 / 211 / 13 |
| `os-store-tests-09` | `🧪️viewer-head` 630, `🧪️deferred-reprojection` 621, `🧪️tool-transaction` 516, `🔬️oracle` 56 + 1 more | 1869 / 212 / 10 |
| `os-store-tests-10` | `♻️bounded-value-retirement` 435, `🧬️retained-clone` 714, `📤️outbound-announcement` 148, `🪪️artifact-addressing` 61 + 1 more | 1386 / 103 / 12 |
| `os-store-sync-tests-1`, `-2` | `S/🔄️sync/🧪️tests`: `🔬️unit` 2,811 lines (alone in bin 1 with `🧪️command-rejection`), the rest (native-actor retained-turn fixtures 480, backbone parity 351, document-socket-connect 256, link shortage 117, echo suppression 88, persistence data class 70) in bin 2 | 4,243 / 316 / 8 over two bins |
| `os-dsl-tests` | `M/🗣️dsl/🧪️tests`, family kits' unit tests, grammar unit tests; the 1,126 KB originals fixture read at run time | 2,471 / 197 / 25 |
| `os-dsl-fixture-sweep-tests` | `M/🗣️dsl/🧹️fixture-sweep/🧪️tests` (9 files; `m5-auto-discovery` 467, …) mounted at the facade root today | 1,029 / 59 / 9 |
| `os-spr-laws-tests` | `M/📡️spr/🧪️tests/{⚖️protocol-laws-unit,🧬️mutation-laws*}`, `🧪️testing/🧬️mutation-laws/**` (17 files) | 2,738 / 148 / 17 + 11 embedded |
| `os-store-member-open-tests` | the host tests of `os-store-member-open` (factory 585, dictionary 433 + 198, open 390, history 231, …) | 1,499 / 134 / 7 |
| `os-store-durable-group-tests` | `S/🧩️composition/🗄️durable-group/🧪️tests/🔬️unit` (1,285 lines, 28 KB fixtures) | 1,285 / 110 / 2 |

The ten `os-store-tests-NN` bins are the first-fit-decreasing packing (limits 3,300 lines / 215 KB / 26 files so the tests keep headroom) of the 37 units that remain after the fixtures are taken out: the 12,625-line unit file is cut into six chunks of about 2,100 lines at the subject boundaries visible in its item outline (fixtures, envelope field decoders and snapshot-read counting at lines 1 to 2,400; member factories, owned batches and member open to 3,300; one-item, retained-clone and paged preparation to 5,400; batch publication and retained groups to 6,300; history lanes, columns and codec round trips to 7,700; replay, projection, ops log and backbone replication to 9,700; pack wire, member link resolver and group dispatch to 11,450; the physical-grant laws to the end), the other 36 units pack around them. Bins 7 to 10 are the byte-heavy small files (`supersede-replay` 120 KB, the space-history snapshot tests 163 KB with fixtures, `viewer-head`, `deferred-reprojection`, `tool-transaction`, `rejected-page-close`, `bounded-value-retirement`, `retained-clone`, …). The packing is an estimate; the hard rule is the census of each bin.

## 10. Work packages and hand-off protocol

### 10.1 Packages (disjoint source file sets; one executor each)

| WP | scope | crates | owns (source files) | est. lib lines | tests |
|---|---|---|---|---:|---|
| 0 | infrastructure (serial, first and last) | all 66 skeletons, registries, mount hub, the S0 splitter; later the facade, TS readers, locks, corpus rows, export parity | `K/**`, `.vscode/*`, `📋️build-owners.json`, the root `Cargo.toml` + `Cargo.lock`, `🧪️tests/🧫️fixtures/📦️crate-boundaries/*`, the 40 scanners of 8.5, `S/🦀️.rs` until S0 hands it over | 694 | - |
| 1 | non-store leaves and hubs | `os-list-delta`, `os-semio`, `os-space-sqlite-cells`, `os-plugin-registry-schema`, `os-extension`, `os-pack-tools`, `os-spr`, `os-spr-cli`, `os-dsl` (+ the `dsl_derive` path-root attribute) | `M/🪡️list-delta`, `🧬️semio`, `🪐️space/🪶️sqlite`, `🔌️plugin/📇️registry` (2 files), `🧩️extension`, `🎒️pack/{⌨️cli,🚦️control}`, `📡️spr/**`, `🗣️dsl/**`, `F/🚪️io/📝️text` mount | 6,072 | in crate |
| 2 | store base A: ledger, replay, read, backbone | `os-store-space-schema`, `-child`, `-ledger`, `-retirement`, `-replay`, `-read`, `-backbone` | `S/{🧬️schema,🪆️child,📒️ledger,📨️messages,♻️retirement,🔁️replay,🔗️read,🕰️history,🔗️backbone}/**` | 6,722 | in crate |
| 3 | store base B: one-item cluster and text IO | `os-store-canonical-json`, `-one-item-publication`, `-operation-wire`, `-text-io`, `-snapshot-clone`, `-one-item-batch` | `S/{🧵️canonical-edit,🎮️one-item,🧵️operation-wire,🚪️io,🧬️snapshot-clone,📬️one-item-owners,📬️paged-one-item,📦️prepared,🫧️ephemeral}/**`, `S/🧩️composition/📨️emission`, `…/📬️publication/🤝️group/🔏️digest` | 8,481 | in crate |
| 4 | codec, decode, member open, io | `os-store-owned-schema-fields`, `-owned-schema-decode`, `-member-open`, `-engine-owners`, `-codec-registry`, `os-io`, `os-store-space-history` | `S/📦️codec/**`, `S/🧩️composition/🚪️open/**` (minus operation and factory), `S/🏗️initialization/🏠️owners/**`, `S/📜️space-history/**`, `M/🚪️io/**` | 11,330 | in crate |
| 5 | **engine (the `ArtifactStore` decomposition)** | `os-store-initialization-runtime`, `-durable-group-wire`, `-engine-core`, `-engine-live`, `-engine-stage`, `-engine`, `-codec-factory` | the root staging files S0 produces for these crates (Appendix B, C), `S/🏗️initialization` runtime parts, `S/🧩️composition/🗄️durable-group` stage/wire parts, `S/🧩️composition/📬️publication/🤝️group` | 10,269 | `#[cfg(test)]` accessors |
| 6 | above the engine | `os-store-initialization-replay`, `-space-member`, `-document`, `-member-open-store`, `-config`, `-durable-group`, `-composition`, `-space`, `-test-support` | `S/🏗️initialization/🔁️replay`, `S/🧾️document/**`, `S/🎚️config/**`, `S/🧩️composition/{🗄️durable-group,🚪️open/🏭️operation,🦀️.rs,🌳️*,🌱️*}`, the root staging files for `space-member`, `space`, `test-support` | 11,224 | in crate / companions |
| 7 | sync | `os-store-sync`, `-sync-native`, `-sync-wasm` | `S/🔄️sync/**`, `F/🏃️process/🧭️routing/🏠️workspace` | 5,740 | companions |
| 8 | test companions | the 18 crates of 9.4 | every test-only file of 9.1, the fixtures consolidation | - | 44,720 lines |

### 10.2 Protocol (nobody edits another package's files; registries are edited once)

**Phase 0 (WP0, serial, before anyone else starts):**
1. Re-run the census of section 3 and diff it against sections 1 and 6: peers moved the tree twice while this plan was produced. Re-run the resolver; if an Appendix C run changed, the owner of the moved file transfers it by renaming its row in 10.1, not by editing foreign files.
2. Generate the 66 crate skeletons in one pass (48 libs, 18 companions): `Cargo.toml` (name, `workspace` depth, `[lib] path = "../../🦀️.rs"`, the three `rust-source-boundary` keys at the ceiling values, dependencies exactly the direct deps of 6.1 plus the externals of 6.2, `[features]` as needed, `[lints] workspace = true`), `📋️project.json`, stub `🦀️.rs` with its docstring, `🧪️tests/🦀️.rs` stub, member row (byte-sorted) in the root `Cargo.toml`, `📋️build-owners.json` row (lexical), six launch routes in `.vscode/launch.json` **and** `.vscode/🧩️launch.seed.jsonc`, corpus rows in `📦️crate-boundaries/🔣️.json` (and `🧬️cuts.json` rows for the main cuts), `Cargo.lock` refresh through the sanctioned launcher (if refused: hand-derived entries, exactly as the pack carve did, and say so). All crates declare their dependencies from day one, so `cargo metadata` already cycle-checks the DAG.
3. **S0, the lossless splitter** (WP0, about one hour of scripting, mechanical): apply Appendix C to the six multi-owner files (`S/🦀️.rs`, `owned-schema`, `durable-group`, `sync`, `M/🚪️io/🦀️.rs`, `one-item/publication` and `canonical-edit`) and the small move groups of 5.2, and Appendix B to the four `ArtifactStore` impl blocks. For every (source file, destination crate) write one staging file `M/<destination node>/📥️lifted/<source-slug>/🦀️.rs` containing the assigned items verbatim (impl blocks of `ArtifactStore` stay in their original `impl<P, Mutation> ArtifactStore<P, Mutation> { … }` wrapper, one wrapper per destination), and cut the same text out of the original. The old kernel crate keeps compiling as one crate (the original files mount the staging files), so the oracle must report an identical error multiset before and after S0, and the line multiset of the staging files plus the remaining originals must equal that of the originals (losslessness check, as in the pack carve). From here the staging directories are disjoint. `include_str!` arguments of moved items are re-resolved by the tool.
4. Rewrite `K/🦀️.rs` into a **mount hub**: the module mounts move into five files `K/📥️mounts/wp<N>/🦀️.rs`, each owned by one package and `include!`d from the root (this is the only way five executors can remove their own mounts without editing one file); the root keeps the aliases and macros until the facade phase.
5. Create the 18 empty companion crates and the shared `os-store-testing-fixtures`.
6. Freeze the interface in the ticket: crate names, component struct names and field names (7.3), `StoreParts`, `ArtifactStoreCloseHost` method list, `StoreMintWitness`, bundle names. This plan is the contract.

**Phase 1 (WP1..WP8 in parallel):** each package carves its crates bottom-up inside the package, in this order:
1. Take the staging files of its crates (S0), place them as the crate's modules (`mod` lines instead of `#[path]` into foreign trees), keep `#[path]` only for the three files that live outside `M/` (`F/🚪️io/📝️text`, `F/🚪️io/📝️text/💡️inferences`, `F/🏃️process/🧭️routing/🏠️workspace`).
2. Widen visibility only as the definition scan demands (`pub(crate)` -> `pub` for cross-crate; fields stay private where C2 or an authority law says so), keep region markers balanced, and replace `crate::os_*` paths by the owning crate (6.3).
3. For macros: apply 6.6; for statics: nothing to do beyond the duplicate scan.
4. Run the per-crate gates of section 11 in the foreground and write `RESULT-W4-OS-KERNEL-WP<N>.md` with the census rows.

**Cross-package order constraints** (compile level, not edit level): WP1, WP2 and WP3 first (they have no dependency on each other except `os-store-space-schema` and `os-store-child` below `os-store-retirement`); WP4 after WP2 and WP3; WP5 after WP4, its internal order being `initialization-runtime`, `durable-group-wire`, `engine-core`, `engine-live`, `engine-stage`, `codec-factory`, `engine`; WP6 after WP5 and after the hydration batch of 1.4; WP7 after WP6; WP8 after the crate it exercises. Edit-level the packages are independent once S0 has run.

**Phase 2 (WP0, serial):** the facade (`K/Cargo.toml` dependency list and features, `K/🦀️.rs` hub with the generated export lists, macros, the `worker` mount), delete the dissolved mounts and the hub files, re-point the 40 scanners (8.5), regenerate `K/📜️script.ts` readers and re-point the 93 Nx targets of `K/📋️project.json` (8.5), run `pubnames` before/after, the macro expansion tests, `cargo metadata --offline` for native, wasm32-unknown and wasm32-wasip2, the count gate (9.2), the corpus gate, export parity of `BackboneWorkerHost`, census of all 66 crates, lock refresh.

### 10.3 Executor rules that matter here

No `git commit/stash/checkout`, no worktrees, no sub-agents, re-read before every edit (the fleet edits these trees), foreground builds only, no compiling around the ingress gate, `[DEBUG] ` prefix for temporary logs, one RESULT file per package, no `ticket_close` from executors, no `🗑️generated` sweep; the three peer-hot files of 1.4 are re-read before every edit and changed only through the peer's batch. WP5 is a single executor: the decomposition is one design and splitting it across executors reintroduces the interface drift the S0 freeze prevents.

## 11. Verification and gates

Per crate (WP-level, before its mount is removed from the hub):
1. `cd <monolith-split>/🗑️generated && ONLY=<crate> bun census.ts`: lib and test rows within 3,500 / 28 / 230,000; a miss of more than 8 percent means re-cut, not squeeze. Also the four cfg sets of the OS corpus test and the wasm32 targets.
2. **Full-resolution differential** from the crate root (`RESULT-HARDEN-WGPU.md` / `RESULT-W4-PACK.md` method: `rustc +nightly-2026-07-20 --emit=metadata`, native lib, `--cfg test`, `wasm32-unknown-unknown` lib and `--test`, `wasm32-wasip2` lib; crates simulated as modules of one crate with `pub(crate)` rewritten to `pub(in <module>)` so private paths are really private; error multisets compared with the baseline sandbox of the old tree). Seeded sensitivity check: remove one `pub`, one import, one `cfg`; the oracle must report it.
3. **Scans the oracle cannot do** (rustc stops before privacy/trait passes once there are errors and extern crates are missing): field/method privacy across crates (`defscan.py`), trait-in-scope for methods imported `as _`, orphan and inherent-impl placement (the resolver's check: every inherent impl and every trait impl must be local to its type's or trait's crate), unused imports, duplicate statics/thread-locals, `cfg(test)` items used across crates, and the receiver scan for `ArtifactStore` private fields (7.6).
4. `rustfmt --edition 2021 --emit stdout` on every new file; path-literal resolver (all `#[path]`, `include*` resolve, longest path under 256 characters); TOML parse of the manifests with both parsers; both launch JSON files parse; every `//#region` pair balanced per file.
5. Real compile attempt at start and end of each package: `bun repo run @semio-tech/<project>-rs:check --configuration=prototype --parallel=1 -- --lib --offline`. The gate refuses the old kernel and every crate whose first-party closure is over a ceiling; a green real compile is expected only for crates whose whole first-party closure is inside the ceilings. Most of this tree's closure runs through `semio-framework-value`, `-replication`, `-job` and `-schema-*`, which the last census still lists as oversized peer crates, so treat a green compile as unlikely before those land (the leaves `os-list-delta`, `os-space-sqlite-cells`, `os-plugin-registry-schema` are the candidates). Report the actual output; never claim a pass that was not run.
6. Regressions to look for after a carve: duplicate singleton statics, a lost `#[cfg]` on an import, a glob accidentally widening a name, trait scoping of types that moved, `Drop` order of the composer, `ManuallyDrop` leaks when a component is dropped alone, and delegates that forgot `#[inline]` on the hot path.

Top level (WP0, phase 2): `pubnames` parity (the set of `(public path, name, kind)` of the old crate equals the facade's, including macros); consumer compile of a sample (the hub, `os-mcp`, one `✏️s` plugin, the wgpu renderer) when the launcher allows; the TS suites that read these sources, each through the module-tree helper: root `📜️script.ts` policies, `🏗️initialization/🏠️owners/🏭️member/📥️source` guard tests, plugin `⏪️time-travel/♻️retirement`, `🧪️reactor-contract-oracles`, `🔬️tool-job-latest-wins`, `♻️lifecycle/🏪️stores`, replication `🌐️foreign` and `🔀️transition/📝️drafts`, `F/🧬️schema/🧩️composition/📥️admission`, `F/🛠️tool-machine` history closure policy, store `📨️emission` and `🔬️canonical-error-progress`, and `🧪️tests/📦️crate-boundaries`; baselines are taken against the pre-move text (E1's method: swap the old file in for a minute, guard by mtime, restore with `cmp`).

## 12. Reproducing the analysis

Everything above was produced from eight small programs (nothing in the repo was written):

* **closure**: `rwp0-tools/closure.ts` with `PKG=K ENTRY=K/🦀️.rs` and one added line (`FEATURES=deflate,sync,ureq,worker` becomes `feature=` cfgs), five rows (lib native, wasm32-unknown, wasip2, test native, test wasm32), optional `LIST=` file listing.
* **tree**: regex mount parser over `#[path]`, `mod x;`, `include!`, `include_str!`/`include_bytes!` with the cfg chain, inline-module directory rules and a cfg evaluator for the three targets (574 nodes).
* **outline**: the column-0 item scanner of Appendix E of `PLAN-RENDERER-WGPU-CRATES.md` (length-preserving comment/string stripper, brace matcher), plus a depth-1 method scanner for impl bodies (353 methods).
* **graph**: item `defs`/`refs` with the rules of 3; `ArtifactStore` expanded to method and component nodes; SCCs by Tarjan; domain and file condensations.
* **resolver**: `auto()` (move the down-closure of a back-referenced item to its lowest user's crate when it is at most 130 lines) and `normalize()` (impl follows type; trait impl to the higher-ranked of trait and type), iterated to a fixed point; `show()` prints each remaining back target with its closure cost.
* **engine**: field table to component, `self.field` and `self.m()` extraction, the homing algorithm of 7.4 and an exhaustive rank search.
* **tests**: host (mount parent) and subject (token weights) placement, first-fit-decreasing packing.
* **stats**: per-crate lines/bytes/files (item lines plus header share), embedded data, statics, macros, `crate::` heads, external crate tokens.

The programs fit in about 1,500 lines of Python and live in the planner's session scratchpad (ephemeral); recreate them from this description. The parameters worth re-running at WP0: the closure rows, the resolver (`limit = 130`) after each package merges its moves, and the receiver scan of 7.6.

## 13. Risks and open decisions

1. **The `ArtifactStore` decomposition is the whole risk.** 353 methods change receiver; borrow-splitting conflicts (a use-case method calling `self.x()` while holding `&mut self.log`) are invisible to every scan here. Expect the composer methods to need local rewrites for about a third of their bodies; budget WP5 accordingly and keep S0's one-crate-compiles invariant until each component has landed.
2. **C2 (authority mint)** is the one design decision with a privacy consequence: a witness type constructed only by `StoreLog::new` (this plan) versus moving 1,100 lines of publication vocabulary above the sealer. If rejected, `os-store-one-item-publication` merges into `os-store-engine-core` and the DAG loses one crate but `engine-core` grows to about 3,800 lines (over budget; then split `engine-core` at `StoreWire`/`StoreCursor`).
3. **`os-store-engine` size.** ≈ 3,200 lines with delegates; the spill valve (792 lines of `ingest_remote_merge`, `advance_apply_batch`, `begin_apply_batch_owned` as bundle methods in a second crate) must be pre-agreed. Generic monomorphisation across the extra crate boundaries enlarges downstream codegen units (every consumer instantiates `ArtifactStore<P, Mutation>` with its own types).
4. **Peer churn.** The tree moved while the plan was produced, and four peer designs touch the same files (1.4). The hydration `C` batch changes three files by name; the store-producer batch changes `new`/`construct`. S0 must run on a quiet tree and WP5/WP6 must not start before the batches land, or must absorb them in the same change.
5. **Name-graph blind spots.** Trait dispatch and blanket impls (`ArtifactDsl`, `ArtifactPack`, `ArtifactStoreSnapshot` live in `text-io` and have blanket impls for `P`), macros and `Deref` are not in the graph; only the oracle plus the first real `cargo check` close that gap. The derive path-root attribute (2.3) is a change in a proc macro used by 2,857 files.
6. **Budget tightness**: `os-store-sync-native` 2,735 lines but 6,978 with its 4,243 test lines, `os-store-owned-schema-decode` 151 KB lib, `os-io` 146 KB, `os-store-engine-core`/`-live` 160 KB each before tests; `os-store-one-item-publication` is the tightest with tests (225 KB).
7. **66 packages is a lot of registry**: 66 x 6 launch routes x 2 JSON files, owner rows, corpus rows, locks, `[profile.wasm-prototype.package.*]` rows if the kernel's code is on the hot wasm path (it has none today; copy the pack carve's decision if perf wants it). Entirely WP0.
8. **TS scanners** (40 files) and the corpus tokens rule: every `crate::os_spr::` (808) must be rewritten before a new crate can pass `📦️crate-boundaries`; this is the largest mechanical edit of the plan and is done per crate by its owner with the table of 6.3.
9. **Cfg**: `target_arch = "wasm32"` is true for wasip2; the sync crates and `os-extension` are absent there, but `feature = "sync"` can be on; the optional target-gated dependency form is load-bearing.
10. **Decisions requested**: (D1) accept C2 as written; (D2) accept the six-component field grouping and `K = 4` (7.4) as the interface freeze; (D3) accept the spill valve for `os-store-engine`; (D4) companion test crates may use `pub` API only (assumed yes; the alternative, `pub(crate)` plus in-crate tests, is what blows 6 crates past 3,500 lines); (D5) keep `worker` and the six facade macros in the facade; (D6) whether `os-store-testing-fixtures` may consolidate the 50 tiny mutation files.

### 13.1 Decisions recorded by OS-KERNEL-WP0 (coordinator rulings, 2026-10-15)

* **D1** accepted: C2 as written (`StoreMintWitness` constructed only by `StoreLog::new`; authority fields stay private in `os-store-one-item-publication`).
* **D2** accepted: the six-component grouping and `K = 4` of 7.4 are the interface freeze for WP5.
* **D3** accepted: the `os-store-engine` spill valve (`ingest_remote_merge`, `advance_apply_batch`, `begin_apply_batch_owned` as bundle methods in a second crate) may be used.
* **D4** accepted: companion test crates use `pub` API only (the `os-spr-laws-tests`, `os-dsl-tests` and `os-dsl-fixture-sweep-tests` companions already follow it; the two private helpers they named became `pub`).
* **D5** accepted: `worker` and the six exported macros that name facade paths stay in the facade.
* **D6** accepted with a limit: consolidate the tiny mutation files only where they are test-only fixture helpers (`S/🧪️tests/🧪️fixture-*`); the taxonomy component files under `S/🧪️testing/🧬️mutations/…` stay one file per component. Consequence for WP8: `os-store-testing-fixtures` keeps those component files and may exceed 28 files; if the census says so it is split into two fixture crates (demo + severity, lossy + timestamped + validated) instead of merging component files.

### 13.2 WP0 and WP1 reconciliation (RESULT-OS-KERNEL-WP0.md has the evidence)

* Census re-run: identical to section 1.1 (lib native 182 / 3,994,566 / 59,414; test native 579 / 9,131,580 / 104,813).
* **Hubs.** `K/📥️mounts/` holds two hubs, not five: `🌱️wp1-leaves` (the facade lines of the carved WP1 crates) and `🧪️wp8-tests` (root-mounted tests). The `io` and `os_store` mounts stay in the kernel root: a module declared inside an `include!`d file is macro-expanded, so `crate::<macro>!` of its `#[macro_export]` macros is refused (test-configuration differential: `list_delta!`, `space_members!`, `field_set_mutations*!`, `original_*_operation_receiving!`).
* **Skeletons.** 67 crates (48 libraries, 19 companions). 34 library nodes already own their `🦀️.rs` (the existing module file becomes the crate root in place; 9 of them are carved by WP1), 14 got a stub; of the 18 companions 14 got a stub. Node corrections: `os-store-test-support` -> `S/🧰️test-support` (the `🧪` sibling emoji was taken), the ten `os-store-tests-NN` bins, the two sync bins and the four other companions got distinct folder emojis (`🏙 🌆 🌈 🌌 🌅 🌴 🌲 🍃 🍎 🍯`, `🍰 🥧`, `🥪`, `🥣`, `🍰`, `🥧`) because the plan's `📦️bin-NN` repeated one emoji among siblings; `os-spr-laws-tests` moved from the existing `⚖️protocol-laws-unit` test file to `🧪️tests/🥣️laws-companion`.
* **Corpus rows** (`📦️crate-boundaries/🔣️.json`, `🧬️cuts.json`) are added per carved crate, not for skeletons: a row on a crate that still mounts store files fails the token law (`os_store`, `os_spr`, ...) until its carve lands.
* **Corrections to 6.1/6.2 found by reading the sources:** `os-list-delta` needs `semio-framework-mutation-diff`, `-value`, `-value-derive`, `-dsl-record-derive` (the plan listed none); `os-dsl` also depends on `os-semio` (`semio_format::sniff`/`Component`); `os-pack-tools` does **not** depend on `os-extension` (its `content_hash` is `semio_framework_os_pack::content_hash`); `os-spr` keeps the replication re-export surface (modules `format`, `history`, `wire`, `channel`, `command`, ...) so the facade module `os_spr` is `pub use semio_framework_os_spr::*` plus the cli; `os-space-sqlite-cells` is mounted by a store file (`S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot`), which now imports the crate (one-line consumer edit).
* **Tests of WP1** (plan 9.2 rule 1: a test lives in its crate when the budget allows): `os-list-delta`, `-semio`, `-space-sqlite-cells`, `-plugin-registry-schema`, `-extension`, `-pack-tools`, `-spr-cli` and the io/materialize/unit tests of `os-spr` stay; the dsl root unit tests, the schema/viewport consumers, the family/grammar/registry/receiving unit tests went to the new companion `os-dsl-module-tests` (the plan's single `os-dsl-tests` came to 48 files with its 22 embedded fixtures, over the 32-file ceiling, so the companions are now 19 and the packages 67), the fixture sweep to `os-dsl-fixture-sweep-tests` (the 36-file test closure of `os-dsl` was over the ceiling); `⚖️protocol-laws-unit` and the mutation-law fixtures went to `os-spr-laws-tests`. The 1,126 KB originals fixture is read at run time.
* **S0.** The only non-store multi-owner file, `M/🚪️io/🦀️.rs`, is split: 60 codec-registry items moved verbatim to `S/📦️codec/📥️lifted/🚪️io/🦀️.rs`, which the original `include!`s. Appendix C line numbers of that file are stale by +1; the splitter derives runs by item names.

## Appendix A. Library files (149 `.rs`; `S/` = `M/🏪️store/`, `M/` = `🧰️framework/🛍️products/💻️os/🔨️modules/`, `F/` = `🧰️framework/🔨️modules/`, `K/` = kernel package dir; paths without a prefix are relative to the shown root)

Lines and KB are the gate counter's numbers for the file; the last column gives the crate (or crates with item lines when the file is split; Appendix C has the item runs). Mount-only files (no column-0 item) follow their parent's crate.

| lines | KB | file (`S/`=`M/🏪️store/`, `M/`=`🧰️framework/🛍️products/💻️os/🔨️modules/`, `F/`=`🧰️framework/🔨️modules/`, `K/`=kernel package dir) | crate (lines) |
|---:|---:|---|---|
| 10 | 0 | `F/🏃️process/🧭️routing/🏠️workspace/🦀️.rs` | `os-store-sync` |
| 47 | 2 | `F/🚪️io/📝️text/💡️inferences/🦀️.rs` | `os-kernel (facade)` |
| 5 | 0 | `F/🚪️io/📝️text/🦀️.rs` | `os-kernel (facade)` |
| 309 | 13 | `K/🦀️.rs` | `os-kernel (facade)` |
| 591 | 26 | `M/🎒️pack/⌨️cli/🦀️.rs` | `os-pack-tools` |
| 83 | 11 | `M/🎒️pack/🚦️control/🦀️.rs` | `os-pack-tools` |
| 806 | 34 | `M/📡️spr/⌨️cli/🦀️.rs` | `os-spr-cli` |
| 536 | 26 | `M/📡️spr/💎️materialize/🦀️.rs` | `os-spr` |
| 369 | 22 | `M/📡️spr/🔌️io/🦀️.rs` | `os-spr` |
| 206 | 16 | `M/📡️spr/🦀️.rs` | `os-spr` |
| 61 | 7 | `M/🔌️plugin/📇️registry/📥️runtime/🦀️.rs` | `os-plugin-registry-schema` |
| 93 | 4 | `M/🔌️plugin/📇️registry/📦️deployment/🧬️schema/🦀️.rs` | `os-plugin-registry-schema` |
| 24 | 1 | `M/🗣️dsl/👪️family/🌍️geo/🦀️.rs` | `os-dsl` |
| 36 | 2 | `M/🗣️dsl/👪️family/🎬️scene/🦀️.rs` | `os-dsl` |
| 150 | 8 | `M/🗣️dsl/👪️family/📊️sheet/🦀️.rs` | `os-dsl` |
| 16 | 0 | `M/🗣️dsl/👪️family/📎️embed/🦀️.rs` | `os-dsl` |
| 162 | 7 | `M/🗣️dsl/👪️family/🕸️graph/🦀️.rs` | `os-dsl` |
| 75 | 3 | `M/🗣️dsl/👪️family/🗂️catalog/🦀️.rs` | `os-dsl` |
| 101 | 4 | `M/🗣️dsl/👪️family/🧑‍🍳recipe/🦀️.rs` | `os-dsl` |
| 90 | 5 | `M/🗣️dsl/📇️registry/🦀️.rs` | `os-dsl` |
| 105 | 5 | `M/🗣️dsl/📖️grammar/🦀️.rs` | `os-dsl` |
| 524 | 29 | `M/🗣️dsl/🦀️.rs` | `os-dsl` |
| 120 | 4 | `M/🗣️dsl/🧠️lsp/🦀️.rs` | `os-dsl` |
| 119 | 13 | `M/🗣️dsl/🫴️receiving/🦀️.rs` | `os-dsl` |
| 103 | 6 | `M/🚪️io/📤️serialization/📦️owned/🦀️.rs` | `os-io` |
| 2352 | 168 | `M/🚪️io/🦀️.rs` | `os-io` (1653); `os-store-codec-registry` (531) |
| 32 | 4 | `M/🚪️io/🧰️testing/🦀️.rs` | `os-io` |
| 54 | 7 | `M/🚪️io/🪪️bindings/🏛️catalog/🦀️.rs` | `os-store-codec-registry` |
| 103 | 7 | `M/🚪️io/🪪️bindings/🦀️.rs` | `os-store-codec-registry` |
| 294 | 13 | `M/🧩️extension/🦀️.rs` | `os-extension` |
| 422 | 31 | `M/🧬️semio/🦀️.rs` | `os-semio` |
| 152 | 9 | `M/🪐️space/🪶️sqlite/🦀️.rs` | `os-space-sqlite-cells` |
| 903 | 49 | `M/🪡️list-delta/🦀️.rs` | `os-list-delta` |
| 180 | 14 | `S/♻️retirement/📦️backing/🦀️.rs` | `os-store-retirement` |
| 55 | 3 | `S/♻️retirement/🦀️.rs` | `os-store-retirement` |
| 1049 | 66 | `S/🎚️config/📥️retained/🦀️.rs` | `os-store-config` |
| 1175 | 80 | `S/🎮️one-item/📬️publication/🦀️.rs` | `os-store-one-item-publication` (583); `os-store-one-item-batch` (525) |
| 33 | 2 | `S/🎮️one-item/📬️publication/🪪️authority/🦀️.rs` | `os-store-one-item-publication` |
| 8 | 0 | `S/🏗️initialization/🌱️genesis/🏭️factory/🧬️schema/🦀️.rs` | `os-store-initialization-replay` |
| 16 | 0 | `S/🏗️initialization/🏠️owners/🏭️member/🧬️schema/🦀️.rs` | `os-store-engine-owners` |
| 24 | 1 | `S/🏗️initialization/🏠️owners/🏭️producer/🧬️schema/🦀️.rs` | `os-store-engine-owners` |
| 668 | 49 | `S/🏗️initialization/🏠️owners/🦀️.rs` | `os-store-engine-owners` |
| 93 | 6 | `S/🏗️initialization/📚️catalog/🦀️.rs` | `os-store-initialization-runtime` (79); `os-store-initialization-replay` (6) |
| 17 | 0 | `S/🏗️initialization/📣️publication/🧬️schema/🦀️.rs` | `os-store-initialization-replay` (8); `os-store-engine` (4) |
| 909 | 46 | `S/🏗️initialization/🔁️replay/🦀️.rs` | `os-store-initialization-replay` (799); `os-store-initialization-runtime` (82) |
| 39 | 2 | `S/🏗️initialization/🤝️authority/🦀️.rs` | `os-store-initialization-replay` |
| 232 | 21 | `S/📒️ledger/♻️retirement/🦀️.rs` | `os-store-ledger` |
| 582 | 33 | `S/📒️ledger/🦀️.rs` | `os-store-ledger` |
| 4 | 0 | `S/📜️space-history/🚪️io/🦀️.rs` | `os-store-space-history` |
| 17 | 1 | `S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/📏️preflight/🦀️.rs` | (no top-level item: mount-only) |
| 85 | 6 | `S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/📏️preflight/🫳️borrowed/🦀️.rs` | `os-store-space-schema` (48); `os-store-space-history` (27) |
| 457 | 30 | `S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs` | `os-store-space-history` (436); `os-store-one-item-batch` (4) |
| 54 | 4 | `S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🛂️admission/🦀️.rs` | `os-store-space-history` |
| 61 | 11 | `S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🛂️validation/🦀️.rs` | `os-store-space-history` |
| 207 | 14 | `S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` | `os-store-space-schema` (122); `os-store-space-history` (45); `os-store-text-io` (19); `os-store-space` (5) |
| 50 | 11 | `S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🫴️receiving/🦀️.rs` | `os-store-space-history` |
| 4 | 0 | `S/📜️space-history/🚪️io/🪶️sqlite/🦀️.rs` | `os-store-space-history` |
| 4 | 0 | `S/📜️space-history/🦀️.rs` | `os-store-space-history` |
| 121 | 10 | `S/📦️codec/🎮️operation/🦀️.rs` | `os-store-codec-registry` |
| 40 | 3 | `S/📦️codec/📸️native-snapshot/🦀️.rs` | `os-store-codec-registry` |
| 442 | 24 | `S/📦️codec/🦀️.rs` | `os-store-codec-registry` |
| 3935 | 247 | `S/📦️codec/🧬️owned-schema/🦀️.rs` | `os-store-owned-schema-decode` (2361); `os-store-owned-schema-fields` (1369) |
| 66 | 4 | `S/📦️codec/🪶️snapshot-capability/🛫️native-encoding/🦀️.rs` | `os-store-codec-registry` |
| 60 | 4 | `S/📦️codec/🪶️snapshot-capability/🛬️native-decoding/🦀️.rs` | `os-store-codec-registry` |
| 65 | 4 | `S/📦️codec/🪶️snapshot-capability/🪆️workspace-receiving/🦀️.rs` | `os-store-codec-registry` (32); `os-store-text-io` (10) |
| 62 | 6 | `S/📦️codec/🪶️snapshot-capability/🪶️native-retirement/🦀️.rs` | `os-store-codec-registry` |
| 57 | 10 | `S/📦️prepared/♻️retirement/🦀️.rs` | `os-store-one-item-publication` (50); `os-store-one-item-batch` (3) |
| 43 | 8 | `S/📨️messages/✂️clamp/🔁️settlement/🦀️.rs` | `os-store-ledger` |
| 4 | 0 | `S/📨️messages/✂️clamp/🦀️.rs` | `os-store-ledger` |
| 259 | 14 | `S/📬️one-item-owners/🦀️.rs` | `os-store-one-item-batch` |
| 594 | 31 | `S/📬️paged-one-item/🦀️.rs` | `os-store-one-item-batch` |
| 177 | 8 | `S/📬️paged-one-item/🧬️mutation-apply/🦀️.rs` | `os-store-one-item-batch` |
| 37 | 1 | `S/🔁️replay/⏱️budget/🦀️.rs` | `os-store-replay` |
| 50 | 1 | `S/🔁️replay/⏳pending/🦀️.rs` | `os-store-replay` |
| 395 | 28 | `S/🔁️replay/🎮️operation/🦀️.rs` | `os-store-replay` |
| 250 | 24 | `S/🔁️replay/🛑️cancel/🦀️.rs` | `os-store-replay` |
| 1197 | 64 | `S/🔁️replay/🦀️.rs` | `os-store-replay` (1081); `os-store-ledger` (32) |
| 4726 | 310 | `S/🔄️sync/🦀️.rs` | `os-store-sync-native` (2735); `os-store-sync` (2082); `os-store-sync-wasm` (913) |
| 147 | 11 | `S/🔗️backbone/♻️retirement/🦀️.rs` | `os-store-backbone` |
| 558 | 26 | `S/🔗️backbone/🦀️.rs` | `os-store-backbone` |
| 1314 | 101 | `S/🔗️read/🦀️.rs` | `os-store-read` (910); `os-store-ledger` (163); `os-store-replay` (85) |
| 6 | 0 | `S/🕰️history/📐️planning/♻️retirement/🦀️.rs` | `os-store-read` |
| 86 | 10 | `S/🕰️history/📐️planning/🎟️admission/🦀️.rs` | `os-store-read` |
| 40 | 3 | `S/🚪️io/💾️binary/🌱️genesis/🦀️.rs` | `os-store-owned-schema-decode` |
| 31 | 1 | `S/🚪️io/💾️binary/📸️snapshot/🦀️.rs` | `os-store-space-schema` (11); `os-store-text-io` (9) |
| 17 | 0 | `S/🚪️io/💾️binary/🧬️mutations/🎚️fields/🦀️.rs` | `os-store-text-io` |
| 17 | 0 | `S/🚪️io/💾️binary/🧬️mutations/🦀️.rs` | `os-store-space-schema` |
| 29 | 2 | `S/🚪️io/💾️binary/🪪️operation-identity/📑️rows/🦀️.rs` | `os-store-text-io` |
| 50 | 4 | `S/🚪️io/💾️binary/🪪️operation-identity/🦀️.rs` | `os-store-text-io` |
| 37 | 2 | `S/🚪️io/📝️text/📸️snapshot/🦀️.rs` | `os-store-space-schema` (15); `os-store-text-io` (9) |
| 1435 | 86 | `S/🚪️io/📝️text/🦀️.rs` | `os-store-text-io` (1329); `os-store-ledger` (3) |
| 29 | 1 | `S/🚪️io/📝️text/🧬️mutations/🎚️fields/🦀️.rs` | `os-store-text-io` |
| 47 | 3 | `S/🚪️io/📝️text/🧬️mutations/🦀️.rs` | `os-store-space-schema` (22); `os-store-text-io` (9) |
| 239 | 19 | `S/🚪️io/🦀️.rs` | `os-store-text-io` |
| 11227 | 756 | `S/🦀️.rs` | split: see Appendix C (store root) and B (methods) |
| 46 | 2 | `S/🧩️composition/🌱️initial/🪪️identity/🦀️.rs` | `os-store-engine` (37); `os-store-composition` (3) |
| 267 | 10 | `S/🧩️composition/🌳️closure/🦀️.rs` | `os-store-composition` |
| 40 | 3 | `S/🧩️composition/🌳️graph/🔗️links/🦀️.rs` | `os-store-composition` |
| 298 | 22 | `S/🧩️composition/🌳️graph/🤝️group/🦀️.rs` | `os-store-composition` |
| 170 | 12 | `S/🧩️composition/📨️emission/📦️owned/🦀️.rs` | `os-store-one-item-batch` (146); `os-store-one-item-publication` (17) |
| 44 | 4 | `S/🧩️composition/📬️publication/🤝️group/🔏️digest/🦀️.rs` | `os-store-one-item-publication` |
| 196 | 20 | `S/🧩️composition/📬️publication/🤝️group/🦀️.rs` | `os-store-engine` (152); `os-store-one-item-batch` (36) |
| 3481 | 218 | `S/🧩️composition/🗄️durable-group/🦀️.rs` | `os-store-durable-group` (1862); `os-store-durable-group-wire` (1042); `os-store-engine-stage` (636) |
| 278 | 16 | `S/🧩️composition/🚪️open/🌱️genesis/🦀️.rs` | `os-store-member-open` |
| 25 | 5 | `S/🧩️composition/🚪️open/🎟️admission/🦀️.rs` | `os-store-member-open` |
| 129 | 13 | `S/🧩️composition/🚪️open/🏭️operation/🛂️receiving/🦀️.rs` | `os-store-member-open` |
| 912 | 71 | `S/🧩️composition/🚪️open/🏭️operation/🦀️.rs` | `os-store-member-open-store` (850); `os-store-space-member` (88); `os-store-engine-owners` (20) |
| 16 | 0 | `S/🧩️composition/🚪️open/🏭️operation/🧬️schema/📦️decoder/🦀️.rs` | `os-store-member-open` |
| 355 | 17 | `S/🧩️composition/🚪️open/📜️history/🏭️factory/🦀️.rs` | `os-store-space-member` |
| 184 | 7 | `S/🧩️composition/🚪️open/📜️history/🗂️dictionary/📇️index/🦀️.rs` | `os-store-member-open` |
| 116 | 3 | `S/🧩️composition/🚪️open/📜️history/🗂️dictionary/🛂️identity/🦀️.rs` | `os-store-member-open` |
| 578 | 27 | `S/🧩️composition/🚪️open/📜️history/🗂️dictionary/🦀️.rs` | `os-store-member-open` |
| 236 | 8 | `S/🧩️composition/🚪️open/📜️history/🗂️dictionary/🧾️record/🦀️.rs` | `os-store-member-open` |
| 272 | 13 | `S/🧩️composition/🚪️open/📜️history/🦀️.rs` | `os-store-member-open` |
| 649 | 34 | `S/🧩️composition/🚪️open/🦀️.rs` | `os-store-member-open` (413); `os-store-member-open-store` (200) |
| 874 | 53 | `S/🧩️composition/🦀️.rs` | `os-store-composition` |
| 214 | 11 | `S/🧬️schema/📜️space-history/🦀️.rs` | `os-store-space-schema` |
| 7 | 0 | `S/🧬️schema/🦀️.rs` | (no top-level item: mount-only) |
| 43 | 2 | `S/🧬️schema/🧬️mutations/🌿️create-space-alternative/🦀️.rs` | `os-store-space-schema` |
| 51 | 3 | `S/🧬️schema/🧬️mutations/🎯️set-active-space-alternative/🦀️.rs` | `os-store-space-schema` |
| 40 | 1 | `S/🧬️schema/🧬️mutations/📌️commit-space-checkpoint/🦀️.rs` | `os-store-space-schema` |
| 40 | 2 | `S/🧬️schema/🧬️mutations/🔀️switch-space-alternative/🦀️.rs` | `os-store-space-schema` |
| 40 | 2 | `S/🧬️schema/🧬️mutations/🗑️remove-space-checkpoint/🦀️.rs` | `os-store-space-schema` |
| 44 | 1 | `S/🧬️schema/🧬️mutations/🦀️.rs` | `os-store-space-schema` |
| 49 | 2 | `S/🧬️schema/🧬️mutations/🧹️remove-space-alternative/🦀️.rs` | `os-store-space-schema` |
| 25 | 3 | `S/🧬️snapshot-clone/♻️custody/🦀️.rs` | `os-store-snapshot-clone` |
| 29 | 2 | `S/🧬️snapshot-clone/♻️pending/🦀️.rs` | `os-store-snapshot-clone` |
| 91 | 5 | `S/🧬️snapshot-clone/⚖️law/🦀️.rs` | `os-store-snapshot-clone` |
| 185 | 13 | `S/🧬️snapshot-clone/✏️config-apply/🦀️.rs` | `os-store-snapshot-clone` |
| 72 | 3 | `S/🧬️snapshot-clone/🚚️handoff/🦀️.rs` | `os-store-snapshot-clone` |
| 679 | 59 | `S/🧬️snapshot-clone/🦀️.rs` | `os-store-snapshot-clone` |
| 78 | 11 | `S/🧬️snapshot-clone/🪪️metadata/🦀️.rs` | `os-store-snapshot-clone` |
| 58 | 2 | `S/🧵️canonical-edit/🌱️value/🦀️.rs` | `os-store-canonical-json` |
| 347 | 13 | `S/🧵️canonical-edit/🌱️value/🪪️admission/🦀️.rs` | `os-store-canonical-json` |
| 181 | 9 | `S/🧵️canonical-edit/📖️reader/🦀️.rs` | `os-store-canonical-json` |
| 807 | 49 | `S/🧵️canonical-edit/🦀️.rs` | `os-store-one-item-publication` (410); `os-store-canonical-json` (355) |
| 129 | 16 | `S/🧵️canonical-edit/🧳️source/🦀️.rs` | `os-store-one-item-publication` (117); `os-store-canonical-json` (2) |
| 327 | 16 | `S/🧵️canonical-edit/🧵️borrowed/🦀️.rs` | `os-store-canonical-json` |
| 104 | 6 | `S/🧵️operation-wire/📝️text/🦀️.rs` | `os-store-operation-wire` (82); `os-store-one-item-publication` (12) |
| 137 | 13 | `S/🧵️operation-wire/📦️output/📋️operations/🦀️.rs` | `os-store-operation-wire` |
| 165 | 13 | `S/🧵️operation-wire/📦️output/🦀️.rs` | `os-store-operation-wire` |
| 320 | 23 | `S/🧵️operation-wire/🦀️.rs` | `os-store-operation-wire` (188); `os-store-one-item-publication` (95) |
| 1255 | 81 | `S/🧾️document/📜️history/💧️hydration/🦀️.rs` | `os-store-document` |
| 461 | 23 | `S/🧾️document/📥️mounted-pack/🦀️.rs` | `os-store-document` |
| 198 | 11 | `S/🧾️document/🦀️.rs` | `os-store-document` (44); `os-store-engine-core` (39); `os-store-space-member` (35); `os-store-engine-live` (31); `os-store-initialization-runtime` (18) |
| 484 | 28 | `S/🪆️child/🦀️.rs` | `os-store-child` |
| 300 | 13 | `S/🪆️child/🧬️retained-clone/🦀️.rs` | `os-store-child` |
| 86 | 5 | `S/🫧️ephemeral/📢️publication/🔁️transfer/🦀️.rs` | `os-store-one-item-batch` |
| 237 | 19 | `S/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs` | `os-store-one-item-batch` |

## Appendix B. `ArtifactStore` method assignment (353 methods, algorithm 7.4)

**history** (crate `os-store-engine-core`; 59 methods, 407 lines). Name(lines); `*` = `pub`; `+n` = foreign components passed as parameters.

clock_now(6)*, envelope(3)*, envelope_view(4)*, current_checkpoint_id(5)*, local_actor_id(5)*, authoring_verb(4)*, set_authoring_verb(5)*, open_transaction(4)*, edit_mutations(6)*, history_columns(4)*, history_read_retirement_birth_bytes(2)*, install_operation_wire(5)*, operation_wire(4)*, take_string_at_retained(3), snapshot_alias_retirement_birth_bytes(6)*, fold_history(7), OPEN_CONFLICT_CAP(23), ensure_open_conflict_capacity(17), conflicts(5)*, open_conflicts(4)*, merge_policy(4)*, set_merge_policy(6)*, messages_for_edit(5)*, trunk_alternative_id(6)*, active_line_id(4)*, branched_alternative(4), replay_edits(4)*, cancel_apply_batch(8)*, edit_mutation_ids(6), checkpoint_origin(14), alternative_origin(10), operation_envelopes(4), operation_envelopes_since(4), observed_foreign_operation(7), seed_known_events(16), admit_history_shape(11), transition_actor(4), transition_envelope(10), fold_live_log(7), event_frontier(5), open_transaction_named(4), pending_fold(8), prospective_fold(4), prospective_fold_at(6), edit_lane(9), viewer_head_now(4), viewer_head(5), snapshot_pack(7)*, envelope_json(12)*, event_log(11)*, event_log_payload(6)*, announce_tail_edit_payload(17)*, backbone_ref(3)*, assert_equivalent_remote_envelope(9), assert_equivalent_remote_mutation(19), same_operation_identity_and_payload(5), take_candidate_envelope(5), verify_genesis(8), edit_is_local(4)

**retire** (crate `os-store-engine-core`; 56 methods, 721 lines). Name(lines); `*` = `pub`; `+n` = foreign components passed as parameters.

retained_derived_read(8)*, prepare_history_read_retirement(8)*, admit_history_owner_retirement(7), retire_derived_history_preview(7)*, retire_derived_report_replay(7)*, retire_finished_history_replay(5)*, owned_snapshot_retirement_factory(4)*, owned_mutation_retirement_factory(4)*, owned_mutation_retirement_birth_bytes(5)*, retire_owned_mutation(6)*, install_document_store_owners_exact(19)*+1, owned_disposer_installed(3)*, displaced_envelope_retirement(5), replace_envelope_retained(8)+1, replace_causal_dag_retained(6)+1, reserve_edit_history_slot(13)+1, reserve_change_history_slot(10)+1, insert_reserved_change_history(15)+1, reserve_checkpoint_history_slot(10)+1, insert_reserved_checkpoint_history(15)+1, reserve_alternative_history_slot(10)+1, insert_reserved_alternative_history(15)+1, maintenance_retirements_step(3)*, handoff_batch_publication_close(16)*, maintenance_retirements_terminal_is_empty(3)*, maintenance_retirements_demands(2)*, maintenance_retirements_under_pressure(4)*, close_owned_phase_witness(4)*, close_one_factory_alias(8), retire_snapshot_alias(6)*, snapshot_read_retirement_demand(4)*, returned_snapshot_read_retirement_demand(8)*, take_returned_snapshot_read_retirement(18)*, snapshot_read_leases_terminal_is_empty(3)*, returned_snapshot_read_count(5)*, outstanding_snapshot_read_count(9)*, RESOLVED_CONFLICT_CAP(1), prune_resolved_conflicts(12)+1, fold_batch_item(61)+1, empty_batch_stage(26), empty_batch_edit(14), preflight_reprojection(27)+1, retire_replay_result(16), retire_raw_replay(14), retire_pending_replay(4), adopt_history_facts(82)+1, replace_edit_messages(23)+1, authored_operation_bytes(29)*+1, authored_edit_operation_bytes(7)*+1, resolve_operation_wire(4), authored_operation_rows(20), authored_operation_identity(10)+1, replay_mutations(58)+1, refuse_with_candidate_dag(7), relieve_displaced_pressure(22), factory_aliases_present(1)

**transport** (crate `os-store-engine-core`; 17 methods, 190 lines). Name(lines); `*` = `pub`; `+n` = foreign components passed as parameters.

durable_group_read_root(3), ensure_durable_group_idle(6), artifact_revision(11)*+1, applied_edit_ids(3)*+1, redo_edit_ids(4)*+1, replace_backbone_retained(7)+1, close_displaced_witness(6)*+1, close_take_backbone_retirement(3), batch_amend_target_ref(5)+1, batch_amend_target(3)+1, author_head_alternative(27)+1, redo_position(5)+1, pending_checkpoint_transition(26)+1, send_member_mutations(12)*, take_member_inbound(11)*, announce_history(34)+1, sync_cursor(24)+2

**cursor** (crate `os-store-engine-core`; 39 methods, 473 lines). Name(lines); `*` = `pub`; `+n` = foreign components passed as parameters.

generation(3)*, generation_now(4)*, content_revision(3)*, content_revision_now(4)*, projection_stamp(4)*, last_projection_invalidation(6)*, accept_projection_result(11)*, seal_derived_snapshot(3)+1, admit_derived_source(6)+1, snapshot_read_derived(13)*+1, derive_provisional_snapshot(10)*, derive_history_snapshot(15)*, begin_derived_history_preview(8)*+2, finish_derived_history_preview(6)*, begin_derived_report_replay(5)*+2, finish_derived_report_replay(10)*, revision_owner_slots(3), push_revision_replacement_reserved(6)+1, replace_revision_accumulator_retained(5)+1, mutation_position(2)*, applied_edit_position(7)*+1, applied_revision(3), applied_unit_flag(3), applied_edit(5)+1, locate_mutation(11)+1, unit_operations(11)*, applied_edit_units(11), admit_apply_batch(38)*+3, begin_apply_batch(17)*, begin_outbound_apply_batch(21)*, begin_typed_apply_batch(27)+2, admit_member_apply_batch(9)*+1, begin_member_apply_batch(15)*+1, begin_apply_batch_owned(130)+2, take_report_replay(17)+1, restamp_applied_tail(5)+1, push_applied_edit_id(5)+1, push_redo_edit_id(5)+1, pop_redo_edit_id(6)+1

**projection** (crate `os-store-engine-live`; 78 methods, 1196 lines). Name(lines); `*` = `pub`; `+n` = foreign components passed as parameters.

projection_event(4)*, snapshot(6)*, snapshot_ref(4)*, derived_snapshot_head(4)*, derived_snapshot_before(5)*, step_derived_history_preview(51)*+1, step_derived_report_replay(45)*+3, step_history_read_plan(55)+1, snapshot_read(14)*+2, snapshot_owner(3)*, snapshot_root(4)*, replace_current_retained(20)+2, replace_document_roots_retained(16)+2, prepare_document_root_commit(10)+1, commit_document_roots_retained(31)+2, string_vector_owner_slots(3), string_owner_slots(3), take_string_vector_replacement(4), take_string_replacement(4), replace_applied_edit_ids_retained(17)+3, replace_redo_edit_ids_retained(15)+3, replace_current_checkpoint_retained(10)+2, insert_reserved_edit_history(17)+2, replace_tail_undo_cache_retained(21)+2, close_cursor_demands(34)+3, fold_current(7)+1, supersessions(4)*, mutation_ops(9)*+1, applied_mutation(14)*+1, applied_edit_mutations(20)*+1, mutation_outcomes(5)*, durable_outcomes(8)+1, applied_edit_outcomes(29)*, unit_id(6)*, state_before(31)*+1, begin_report_replay(13)*+1, refuse_blocking_history(12)*+1, replay_report(10)*, batch_item_base(15)+2, original_proposal_forward(12)+1, advance_prepared_foreign_steps(11)*, advance_prepared_operation_wire(9)*, adopt_remote_transitions(13)+1, insert_transition(7)+1, reproject(8), reproject_now(7), adopt_folded_tail(10)+1, live_prefix_state(43)+2, live_base(20)+1, assert_prefix_entry_is_live(9)+2, reprojection_replay(14)+1, reproject_replayed(19), adopt_reprojection(49)+1, adopt_replayed_projection(16)+2, adopt_history_positions(11)+1, replay_window(11)+1, report_replay(19)+2, convergence_checkpoints(24)+1, prefix_state(4), prefix_state_recorded(27)+2, adopt_report_replay(18), replace_supersessions_retained(13)+1, prefix_stride(4), prefix_ring_capacity(6), retain_prefix_snapshots(20)+3, admit_prefix_snapshots(45)+2, evict_prefix_snapshot_reserved(8)+1, prune_prefix_ring(16)+1, clear_prefix_ring(12)+1, lift_open_transaction(13)+1, dry_run(7)+1, supersede_inputs(43)+1, supersede_input(19)+1, draft_inputs(23)+1, snapshot_json(4)*, take_candidate_current(5), pop_applied_edit_id(8)+2, enable_convergence_early_exit(6)*

**replay** (crate `os-store-engine-live`; 48 methods, 936 lines). Name(lines); `*` = `pub`; `+n` = foreign components passed as parameters.

invalidate_after_replay(4)*, invalidate_after_policy_change(4)*, invalidate_after_external_resource_change(4)*, invalidate_projections(6)+1, pending_report_owner_slots(5), push_pending_report_replacement_reserved(6)+1, replace_pending_report_retained(5)+1, replay_suffix_partitioned(26), flush_published_apply_batch(11)*, announce_operations(20)+1, admit_remote_transitions(34)+1, retract_transitions(24)+2, folded_tail(9)+2, tail_step(21)+3, append_transaction(35)+3, continue_pending_replay_across_append(11)+1, open_transaction_edit(40)+3, commit_open_transaction(14)*+1, abort_open_transaction(15)*+1, flush_announcements(5)*, restore_open_transaction(24)+1, defer_remote_replays(9)*, defer_local_replays(12)*, local_step_pending(4)*, discard_local_step(14)*, reprojection_progress(5)*+1, reprojection_operation_preparation_progress(4)*+1, cancel_reprojection(10)*, step_reprojection(14)*+1, defer_local_step(32)+3, advance_reprojection(60)+3, refuse_local_step(8), adopt_pending(61)+1, record_replay_report(12), absorb_merge_report(10), switch_local_alternative(15)+1, checkout_local_checkpoint(14)+1, move_head(14)+1, apply_command(61)+3, record_edit_messages(29)+2, detach_backbone(13)*+2, resolve_conflict(41)*+2, accept_quarantined_events(85)+2, flush_outbound(19)+1, flush_apply_outbound(19)+2, fold_is_live(5)+1, bump(17)+4, assert_mirrors_are_live(26)+3

**ops (composer use-case methods)** (crate `os-store-engine`; 56 methods, 1857 lines). Name(lines); `*` = `pub`; `+n` = foreign components passed as parameters.

preflight_runtime_seed(13), adopt_runtime_seed(12), seed_runtime_state(20), new(25)*, construct(83), from_initialized_runtime_with_owners(6)*, reset(8)*, set_state(12)*, adopt_state(55), set_checkpoint_composition_pins(30)*, close_owned_demands(20)*, close_owned_store_step(33)*, close_owned_store_terminal_is_empty(4)*, next_resident_backing_byte_demand(10), close_resident_backing_step(21), close_owned_step(5)*, close_owned_terminal_is_empty(4)*, close_owned_unscheduled(10)*, close_cursor_admit(121), close_structural_owners_terminal_is_empty(19), owned_roots_terminal_is_empty(9)*, edits_for_ids(14), dispatch(20)*, dispatch_apply_exact(14), advance_apply_batch(301)*, spend_item_close_grant(20), apply_one(19)*, commit_transition(9), install_transitions(43), adopt_tail_step(54), dispatch_inner(90), supersede_command(16), create_alternative_with_supersede(6), commit_finished_replay(16)*, commit_slotted_replay(36), author_supersession(14), author_slotted_supersession(81), undo_with_policy(20), undo_lane_position(10), redo_lane_position(7), commit_pending_checkpoint(5), dispatch_text(11)*, dispatch_binary(10)*, attach_backbone(13)*, ingest_remote_payload(12)*, attach_hot_backbone(8)*, attach_backbone_uri(8)*, tick(9)*, tick_backbone_reports(4)*, ingest_remote(11)*, ingest_remote_merge(361), pump(4), pump_with_reports(43), resident_backings_terminal_is_empty(3), merge_persisted_pair(5)*, merge_persisted_history(40)*


## Appendix C. Item runs of the split files (the S0 splitter specification)

For every file whose items belong to more than one crate: consecutive item runs in source order. The root table omits the `ArtifactStore` struct and its impl blocks (Appendix B and 7.3); `kernel-root` means the facade (macros). Region markers inside a run travel with it; a region that crosses a run boundary is repeated, balanced, in both staging files.

**STORE ROOT `S/🦀️.rs`** (4351 item lines → `os-store-space-member` 1079, `os-store-test-support` 829, `os-store-initialization-runtime` 570, `os-kernel (facade)` 310, `os-store-codec-factory` 291, `os-store-space` 290, `os-store-engine` 286, `os-store-engine-core` 272, `os-store-config` 181, `os-store-engine-owners` 152, `os-store-engine-live` 76, `os-store-document` 15)

| lines (from–to) | crate | item lines | first items |
|---|---|---:|---|
| 163–321 | `os-store-engine-owners` | 152 | ArtifactStoreCloseView, impl ArtifactStoreCloseView, ArtifactStoreOwnedDisposer, ArtifactStoreCursorDisposer, impl ArtifactStoreCursorDisposer |
| 348–409 | `os-store-space-member` | 46 | content_id, ArtifactRefs, LinkState, LinkResolver, MemberDirectory |
| 411–425 | `os-store-space` | 15 | impl SpaceHost |
| 427–504 | `os-store-space-member` | 73 | NoBlobStore, impl NoBlobStore, MemberLinkResolver, impl MemberLinkResolver |
| 506–591 | `os-store-space` | 82 | create_member_store, open_member_store, install_unscheduled_catalog, install_funded_member_store_owners, validate_member_history_identity |
| 599–676 | `os-store-engine-core` | 65 | impl OwnerRef, CommandReceipt, ArtifactEnvelopeView, impl ArtifactEnvelopeView |
| 678–679 | `os-store-engine` | 2 | DraftStore |
| 719–733 | `os-store-document` | 15 | mounted_pack_rt |
| 791–1085 | `os-store-codec-factory` | 291 | artifact_codec, bare_artifact_codec, ARTIFACT_CODEC_APPLY_CLOSE_MAXIMUM_STEPS, close_codec_reduction_store, replay_envelopes_onto_pair |
| 1091–1155 | `os-store-engine` | 63 | checkpoint_ancestors, merge_base, reconcile_alternative |
| 1159–1159 | `os-store-config` | 1 | ConfigEnvelope |
| 1160–1160 | `os-store-engine` | 1 | ConfigStore |
| 1162–1347 | `os-store-config` | 180 | create_config_envelope, ConfigRecord, field_set_mutations, config_record, config_diff |
| 1365–1551 | `os-store-engine-core` | 150 | impl crate, HistoryColumn, branched_alternatives, checkpoint_alternatives, is_checkpoint_main_only, has_main_only_descendant, assign_history_checkpoint_lanes, … (+5) |
| 1553–2146 | `os-store-initialization-runtime` | 570 | artifact_initial_digest, artifact_initial_digest_of_pack, ArtifactStoreInitializationDigest, impl ArtifactStoreInitializationDigest, ArtifactStoreInitializationEditIndex, ArtifactStoreInitializationEditAdmission, impl ArtifactStoreInitializationEditIndex, … (+6) |
| 2180–2194 | `os-store-engine` | 15 | impl Arc |
| 2324–2334 | `os-store-engine-live` | 10 | ArtifactStoreDocumentRootCommitAuthority, edit_actor_from_meta |
| 2336–8532 | `os-store-engine` | 79 | config_store_from_initialized_runtime_with_owners, checkpoint_identity, HistoryFinalization, SupersedeScope |
| 8534–8550 | `os-store-engine-live` | 17 | AppliedMutation |
| 8563–8658 | `os-store-engine` | 94 | impl ArtifactStore, preflight_merge_by_id, merge_by_id |
| 8660–8700 | `os-store-engine-core` | 37 | ScratchOwner, impl ScratchOwner |
| 8702–8721 | `os-store-engine-live` | 20 | transition_retraction_closure |
| 8723–8732 | `os-store-engine` | 10 | retract_history_transitions_from_spr |
| 8734–8762 | `os-store-engine-live` | 29 | document_backbone_batches |
| 8764–8783 | `os-store-engine-core` | 20 | fold_history |
| 8847–8872 | `os-store-engine` | 22 | impl Arc, impl HashMap, impl ArtifactHistoryLedger |
| 8899–9886 | `os-store-space-member` | 960 | BlobStore, SpaceMember, impl ArtifactStore, MemberFactory, MemberStoreVisitor, MemberStoreVisitorMut, MemberVisit, … (+2) |
| 9888–10197 | `os-kernel (facade)` | 310 | space_members |
| 10215–10410 | `os-store-space` | 193 | space_history_store_owners, SpaceHost, impl SpaceHost |
| 10433–11338 | `os-store-test-support` | 829 | test_support, STORE_SCHEMA_EXPORTS, register_store_schema_exports |

**`S/🔄️sync/🦀️.rs`** (5710 item lines → `os-store-sync-native` 2735, `os-store-sync` 2062, `os-store-sync-wasm` 913)

| lines (from–to) | crate | item lines | first items |
|---|---|---:|---|
| 19–2083 | `os-store-sync` | 1897 | is_repository_root, SyncError, impl SyncError, actor_identity_admission_admits, actor_identity_batch_admits, add_actor_identity_progress, wire_envelopes_with_original_actor, … (+100) |
| 2087–4401 | `os-store-sync-native` | 2315 | native_actor |
| 4410–5322 | `os-store-sync-wasm` | 913 | wasm_actor |
| 5328–5492 | `os-store-sync` | 165 | fixtures |
| 5498–5933 | `os-store-sync-native` | 420 | FolderEventLogStorage, FOLDER_EVENT_MAGIC, MAX_FOLDER_EVENT_BYTES, DOCUMENT_PUT_EVENT, BLOB_PUT_EVENT, BLOB_DELETE_EVENT, DOCUMENT_ARCHIVE_PUT_EVENT, … (+6) |

**`S/📦️codec/🧬️owned-schema/🦀️.rs`** (3730 item lines → `os-store-owned-schema-decode` 2361, `os-store-owned-schema-fields` 1369)

| lines (from–to) | crate | item lines | first items |
|---|---|---:|---|
| 36–1143 | `os-store-owned-schema-fields` | 1035 | ARTIFACT_ENVELOPE_SCHEMA_FIELD, ARTIFACT_ENVELOPE_ID_FIELD, ARTIFACT_ENVELOPE_VCS_FIELD, ARTIFACT_ENVELOPE_BACKBONE_FIELD, ARTIFACT_ENVELOPE_ACTIVE_ALTERNATIVE_FIELD, ARTIFACT_ENVELOPE_CURSOR_FIELD, ARTIFACT_ENVELOPE_DIALECT_FIELD, … (+37) |
| 1145–1180 | `os-store-owned-schema-decode` | 35 | ArtifactEnvelopeDecodeOwnerBundle, impl ArtifactEnvelopeDecodeOwnerBundle |
| 1182–1511 | `os-store-owned-schema-fields` | 306 | ArtifactEnvelopeFieldDecoder, artifact_envelope_field_demands, artifact_retirement_grant_covers, artifact_retirement_self_grant, artifact_envelope_diagnostic_error, ARTIFACT_ENVELOPE_FIELD_DECODER_CAPACITY, ArtifactEnvelopeFieldDecoderTicket, … (+14) |
| 1513–1750 | `os-store-owned-schema-decode` | 235 | OwnedSchemaBoundedArrayState, OwnedSchemaBoundedArrayAuthority, impl OwnedSchemaBoundedArrayAuthority |
| 1752–1780 | `os-store-owned-schema-fields` | 28 | impl OwnedSchemaEmptyArrayAuthority, ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES |
| 1782–3937 | `os-store-owned-schema-decode` | 2091 | ARTIFACT_ENVELOPE_DECODE_MAXIMUM_BYTES, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_PAGES, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_CLOSE_ALLOCATION_BYTES, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_VCS_BYTES, ARTIFACT_ENVELOPE_DECODE_MAXIMUM_RETAINED_FIELD_BYTES, ArtifactEnvelopeDecodePage, artifact_envelope_decode_record, … (+38) |

**`S/🧩️composition/🗄️durable-group/🦀️.rs`** (3537 item lines → `os-store-durable-group` 1862, `os-store-durable-group-wire` 1039, `os-store-engine-stage` 636)

| lines (from–to) | crate | item lines | first items |
|---|---|---:|---|
| 21–119 | `os-store-durable-group-wire` | 88 | DURABLE_OWNED_GROUP_DECISION_SCHEMA_V1, DURABLE_OWNED_GROUP_ANCHOR_SCHEMA_V1, DURABLE_OWNED_GROUP_SHAPE_V1, DURABLE_OWNED_GROUP_EVENT_MAX_BYTES, DURABLE_OWNED_GROUP_RECOVERY_PACK_MAX_BYTES, DURABLE_OWNED_GROUP_STRUCTURAL_MAX_BYTES, DURABLE_OWNED_GROUP_ID_MAX_BYTES, … (+18) |
| 121–132 | `os-store-durable-group` | 12 | DurableOwnedThreeMemberDecisionV1 |
| 134–224 | `os-store-durable-group-wire` | 85 | DurableOwnedGroupDecisionError, impl DurableOwnedGroupDecisionError, DurableUnboundOneItemOutcomeV1, DurableBoundOneItemOutcomeV1, DurableStorePreparedOutcomeV1, DurableStoreVerifiedOutcomeV1 |
| 226–241 | `os-store-engine-stage` | 15 | DurableStorePreparedOwnerV1, DurableStoreBoundOutcomeV1 |
| 243–260 | `os-store-durable-group` | 16 | DurableOwnedThreeStorePreparedV1, DurableOwnedThreeStoreBoundV1, DurableOwnedThreeStoreRecoveryV1 |
| 262–269 | `os-store-durable-group-wire` | 8 | DurableOwnedGroupJournalReceiptV1 |
| 271–281 | `os-store-durable-group` | 11 | DurableOwnedGroupJournalRecordV1 |
| 283–317 | `os-store-durable-group-wire` | 34 | DurableOwnedGroupVerifiedThreeEditsV1, impl DurableOwnedGroupVerifiedThreeEditsV1 |
| 319–441 | `os-store-durable-group` | 123 | impl DurableOwnedGroupJournalRecordV1 |
| 443–649 | `os-store-durable-group-wire` | 196 | DurableOwnedGroupJournalAdvanceV1, DurableOwnedGroupJournalCommitV1, close_journal_owner, DurableOwnedGroupJournalSinkV1, ArtifactStoreDurableGroupRootV1, StagedRootRetirement, impl StagedRootRetirement, … (+2) |
| 651–822 | `os-store-engine-stage` | 171 | stage_store_member, stage_prebuilt_batch_root |
| 824–828 | `os-store-durable-group-wire` | 5 | retain_displaced_owner |
| 830–971 | `os-store-engine-stage` | 138 | abort_staged_store_member, adopt_staged_store_member, validate_adopted_store_member, clear_adopted_store_member, retire_unstaged_store_member |
| 973–1014 | `os-store-durable-group-wire` | 40 | DurableOwnedThreeStoreCommitPhaseV1, DurableOwnedThreeStoreCommitAdvanceV1, DurableOwnedThreeStoreSnapshotV1 |
| 1016–1224 | `os-store-durable-group` | 198 | captured_store_snapshot, capture_store_owned_three_snapshot, DurableOwnedThreeStoreCommitV1, DurableOwnedMapCommitOperationV1, DurableOwnedMapCommitHostV1, DurableOwnedMapCommitOwnersV1, DurableOwnedMapRecoveryAdmissionV1, … (+5) |
| 1226–1234 | `os-store-durable-group-wire` | 9 | DurableOwnedMapRecoveryAdvanceV1 |
| 1236–1283 | `os-store-durable-group` | 47 | impl DurableOwnedMapRecoveryAdmissionV1 |
| 1285–1334 | `os-store-durable-group-wire` | 46 | DurableOwnedMapMemberAdmissionV1, impl DurableOwnedMapMemberAdmissionV1, DurableOwnedThreeStoreMapAssemblyFailureV1, DurableOwnedThreeStoreMapAssemblyPhaseV1, DurableOwnedThreeStoreMapAssemblyAdvanceV1 |
| 1336–1383 | `os-store-durable-group` | 47 | DurableOwnedThreeStoreMapAssemblyOwnersV1, DurableOwnedThreeStoreMapAssemblyV1 |
| 1385–1406 | `os-store-durable-group-wire` | 21 | close_assembly_publication, take_assembly_prepared |
| 1408–2418 | `os-store-durable-group` | 999 | impl DurableOwnedThreeStoreMapAssemblyV1, impl DurableOwnedThreeStoreBoundV1, impl DurableOwnedThreeStoreCommitV1, impl DurableOwnedMapCommitOperationV1, impl DurableOwnedMapCommitHostV1, impl DurableOwnedMapRecoveryOperationV1, impl DurableOwnedMapRecoveryHostV1 |
| 2420–2561 | `os-store-durable-group-wire` | 141 | impl DurableUnboundOneItemOutcomeV1, impl DurableStorePreparedOutcomeV1 |
| 2563–2573 | `os-store-engine-stage` | 11 | impl DurableStorePreparedOwnerV1 |
| 2575–2645 | `os-store-durable-group-wire` | 69 | parse_canonical_json_value, validate_json_budget, valid_edit_identities |
| 2647–2783 | `os-store-engine-stage` | 135 | verify_bound_member_edit, store_post_revision, impl DurableStorePreparedOwnerV1 |
| 2785–2859 | `os-store-durable-group-wire` | 75 | impl DurableBoundOneItemOutcomeV1 |
| 2861–2948 | `os-store-engine-stage` | 88 | impl DurableStoreBoundOutcomeV1 |
| 2950–2966 | `os-store-durable-group` | 17 | impl DurableOwnedThreeStorePreparedV1 |
| 2968–2972 | `os-store-durable-group-wire` | 5 | DurableMemberRecoveryFrontier |
| 2974–3000 | `os-store-engine-stage` | 27 | recovery_frontier |
| 3002–3038 | `os-store-durable-group` | 37 | impl DurableOwnedThreeMemberDecisionV1 |
| 3040–3093 | `os-store-engine-stage` | 51 | store_member_identity, prepared_member, bound_member, prepared_base_matches_store |
| 3095–3143 | `os-store-durable-group` | 49 | impl DurableOwnedThreeStorePreparedV1 |
| 3145–3289 | `os-store-durable-group-wire` | 134 | impl DurableUnboundOneItemOutcomeV1, impl DurableBoundOneItemOutcomeV1, DurableOwnedGroupMapHandlesV1, DurableOwnedGroupMapFrontiersV1, valid_hash, valid_identity, valid_reference, … (+3) |
| 3291–3557 | `os-store-durable-group` | 264 | impl DurableOwnedThreeMemberDecisionV1, durable_owned_group_journal_test_record |
| 3559–3644 | `os-store-durable-group-wire` | 83 | durable_group_test_revision, durable_group_test_outcome, durable_group_test_member, durable_group_test_bind_member |
| 3646–3687 | `os-store-durable-group` | 42 | durable_owned_group_journal_test_record_from_edits |

**`M/🚪️io/🦀️.rs`** (2170 item lines → `os-io` 1639, `os-store-codec-registry` 531)

| lines (from–to) | crate | item lines | first items |
|---|---|---:|---|
| 19–24 | `os-io` | 6 | AnalyzeSource |
| 26–32 | `os-store-codec-registry` | 7 | Confidence |
| 34–55 | `os-io` | 20 | Analysis, ComposeSource, Composition |
| 57–94 | `os-store-codec-registry` | 29 | ComposeError, ErasedComposeSource, ComposedArtifact, ComposeFuture, AsyncComposeFn |
| 96–144 | `os-io` | 47 | compose_thunk, io_run_thunk, io_sniff_thunk |
| 148–260 | `os-store-codec-registry` | 101 | ComposerEntry, IoDirection, IoKey, impl IoKey, IO_REGISTRY, io_registry, IoRegistryConflict, … (+6) |
| 262–442 | `os-io` | 162 | preflight_composer_entries, preflight_composer_entry_refs, preflight_composer_entry_refs_in_assembly, register_composer_entries, register_composer_entry_refs, register_composer_entry_refs_in_assembly, IoResolveError, … (+12) |
| 446–458 | `os-store-codec-registry` | 13 | SubsetValidator |
| 460–467 | `os-io` | 8 | SnapshotSubsetValidator |
| 469–478 | `os-store-codec-registry` | 9 | SnapshotSubsetValidation, SubsetValidatorEntry |
| 483–516 | `os-io` | 31 | validate_payload_thunk, subset_validator_entry_of, snapshot_subset_validator_entry_of, validate_registered_snapshot_subset |
| 518–545 | `os-store-codec-registry` | 24 | SUBSET_VALIDATOR_REGISTRY, subset_validator_registry, SubsetValidatorRegistryConflict, SubsetValidatorRegistryError, impl SubsetValidatorRegistryError |
| 547–552 | `os-io` | 6 | SubsetValidationError |
| 554–560 | `os-store-codec-registry` | 7 | same_subset_validator_entry |
| 562–565 | `os-io` | 4 | register_subset_validator |
| 567–587 | `os-store-codec-registry` | 21 | validate_subset_validators |
| 589–932 | `os-io` | 310 | preflight_subset_validators, preflight_subset_validators_in_assembly, register_subset_validators, register_subset_validators_in_assembly, list_registered_subset_validator_dialects, run_subset_validation, IoFidelityClass, … (+22) |
| 936–1013 | `os-store-codec-registry` | 70 | FormatDescriptor, FormatRegistryConflict, impl FormatRegistryConflict, FormatRegistryError, impl FormatRegistryError, FORMAT_CATALOG, sqlite_snapshot_format_descriptor, … (+1) |
| 1015–1044 | `os-io` | 27 | register_format_descriptors, register_format_descriptors_in_assembly, preflight_format_descriptors, preflight_format_descriptors_in_assembly |
| 1046–1154 | `os-store-codec-registry` | 104 | FormatDescriptorIndexes, index_format_descriptors, canonicalize_format_descriptor, format_mimes, validate_format_descriptors, format_descriptor_keys |
| 1156–1177 | `os-io` | 20 | format_descriptor, normalize_format_kind, format_accept_filter |
| 1185–1339 | `os-store-codec-registry` | 146 | ArtifactAssemblyRegistryPlan, impl ArtifactAssemblyRegistryPlan, ArtifactAssemblyRegistryError, impl ArtifactAssemblyRegistryError, register_native_snapshot_codec, publish_native_snapshot, register_native_document_codec, … (+2) |
| 1350–2347 | `os-io` | 998 | io_mechanism |

**`S/🔗️read/🦀️.rs`** (1158 item lines → `os-store-read` 910, `os-store-ledger` 163, `os-store-replay` 85)

| lines (from–to) | crate | item lines | first items |
|---|---|---:|---|
| 46–324 | `os-store-read` | 266 | ArtifactDerivedSnapshot, impl ArtifactDerivedSnapshot, ArtifactDerivedReplay, impl ArtifactDerivedReplay, ArtifactDerivedHistoryPreview, impl ArtifactDerivedHistoryPreview, HistoryReadPlan, … (+4) |
| 328–347 | `os-store-replay` | 18 | ArtifactStoreSupersessionRetirement, impl ArtifactStoreSupersessionRetirement |
| 349–366 | `os-store-ledger` | 16 | ArtifactStoreStringRetirement, impl ArtifactStoreStringRetirement |
| 368–437 | `os-store-replay` | 67 | ArtifactStoreStringVectorRetirement, impl ArtifactStoreStringVectorRetirement |
| 439–475 | `os-store-read` | 32 | ArtifactStoreRevisionAccumulatorRetirement, impl ArtifactStoreRevisionAccumulatorRetirement, ArtifactStoreCursorRetirement, impl ArtifactStoreCursorRetirement |
| 477–621 | `os-store-ledger` | 134 | ArtifactStoreEditRetirement, ArtifactStoreEditMetadata, ArtifactStoreEditRetirementState, impl ArtifactStoreEditRetirement, ArtifactStoreHistoryMetadataRetirement, ArtifactStoreHistoryMetadataRetirementState, impl ArtifactStoreHistoryMetadataRetirement |
| 623–809 | `os-store-read` | 161 | ArtifactStoreMutationDagRetirement, impl ArtifactStoreMutationDagRetirement, ArtifactStoreVcsRetirementPhase, admit_original_snapshot_alias, ArtifactStoreHistoryLedgerRetirement, impl ArtifactStoreHistoryLedgerRetirement, ArtifactStoreVcsRetirement |
| 811–819 | `os-store-ledger` | 8 | ArtifactStoreOperationRowOwners, impl ArtifactStoreOperationRowOwners |
| 821–1166 | `os-store-read` | 314 | ArtifactStoreCheckpointRetirementFactory, impl ArtifactStoreCheckpointRetirementFactory, ArtifactStoreAlternativeRetirementFactory, impl ArtifactStoreAlternativeRetirementFactory, artifact_history_ledger_birth, artifact_history_ledger_handoff, impl ArtifactStoreVcsRetirement, … (+14) |
| 1168–1172 | `os-store-ledger` | 5 | ArtifactStoreDisplacedOwnerReservation |
| 1174–1313 | `os-store-read` | 137 | impl ArtifactStoreDisplacedRetirements, ArtifactStoreCursorDisposerPhase, ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS |

**`S/🔁️replay/🦀️.rs`** (1109 item lines → `os-store-replay` 1077, `os-store-ledger` 32)

| lines (from–to) | crate | item lines | first items |
|---|---|---:|---|
| 30–36 | `os-store-replay` | 7 | PairMerge |
| 38–40 | `os-store-ledger` | 3 | EffectiveSupersessions |
| 42–167 | `os-store-replay` | 120 | EffectiveOperation, impl EffectiveOperation, effective_operation, effective_operation_with_supersession, admit_replacement, effective_forwards |
| 169–173 | `os-store-ledger` | 5 | FoldProjection |
| 175–276 | `os-store-replay` | 97 | fold_effective_edit, apply_outcome, fold_operation, fold_recorded |
| 280–304 | `os-store-ledger` | 24 | supersession_digest, edit_supersession_digest |
| 306–1190 | `os-store-replay` | 853 | order_ledger_as, shared_prefix_len, mirror_id_suffix, forward_prefix_digests, forward_prefix_genesis, forward_prefix_step, superseded_positions, … (+22) |

**`S/🎮️one-item/📬️publication/🦀️.rs`** (1106 item lines → `os-store-one-item-publication` 581, `os-store-one-item-batch` 525)

| lines (from–to) | crate | item lines | first items |
|---|---|---:|---|
| 43–643 | `os-store-one-item-publication` | 581 | ArtifactStoreOneItemLiveAuthority, impl ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemSeal, ArtifactStoreOneItemPrepared, impl ArtifactStoreOneItemPrepared, ArtifactStoreOneItemPreparedParts, ArtifactStoreOneItemPreparationRequest, … (+11) |
| 645–1189 | `os-store-one-item-batch` | 525 | MemberStoreOneItemWire, MemberStoreOneItemWireRequest, MemberStoreOneItemWirePreparationFactory, ErasedMemberStoreOneItemPublication, MemberStoreOneItemPublication, impl MemberStoreOneItemPublication, ArtifactStoreBatchAdmissionRejected, … (+5) |

**`S/🧩️composition/🚪️open/🏭️operation/🦀️.rs`** (954 item lines → `os-store-member-open-store` 846, `os-store-space-member` 88, `os-store-engine-owners` 20)

| lines (from–to) | crate | item lines | first items |
|---|---|---:|---|
| 23–34 | `os-store-member-open-store` | 12 | admit_member_input_buffer |
| 36–57 | `os-store-engine-owners` | 20 | MemberSnapshotOpenProgress, MemberSnapshotOpenStep, MemberSnapshotOpenOperation |
| 59–103 | `os-store-space-member` | 42 | UnsupportedMemberSnapshotOpen, impl UnsupportedMemberSnapshotOpen |
| 105–257 | `os-store-member-open-store` | 148 | ProducedMemberSnapshotOpen, PACK_MEMBER_SNAPSHOT_CHUNK_BYTES, impl ProducedMemberSnapshotOpen |
| 259–306 | `os-store-space-member` | 46 | UnsupportedMemberFactoryOpen, impl UnsupportedMemberFactoryOpen |
| 308–1000 | `os-store-member-open-store` | 686 | Phase, InitialMemberStoreOpen, impl InitialMemberStoreOpen, snapshot_input_tests |

**`S/🏗️initialization/🔁️replay/🦀️.rs`** (878 item lines → `os-store-initialization-replay` 796, `os-store-initialization-runtime` 82)

| lines (from–to) | crate | item lines | first items |
|---|---|---:|---|
| 18–22 | `os-store-initialization-replay` | 4 | ARTIFACT_STORE_REPLAY_FIELD_BYTES, INDEX_RETIRE_PAGE, ITEM |
| 24–106 | `os-store-initialization-runtime` | 82 | ArtifactStoreInitializationNext, impl ArtifactStoreInitializationRuntime |
| 108–906 | `os-store-initialization-replay` | 792 | Phase, Turn, permits, nested, ArtifactStoreReplayInitializer, impl ArtifactStoreReplayInitializer |

**`S/🧵️canonical-edit/🦀️.rs`** (748 item lines → `os-store-one-item-publication` 407, `os-store-canonical-json` 341)

| lines (from–to) | crate | item lines | first items |
|---|---|---:|---|
| 33–388 | `os-store-canonical-json` | 341 | ARTIFACT_CANONICAL_JSON_DEPTH, ARTIFACT_CANONICAL_JSON_CHUNK_BYTES, CANONICAL_EDIT_MAXIMUM_BYTES, CANONICAL_EDIT_MAXIMUM_OVERHEAD_BYTES, ArtifactCanonicalJsonEncodeError, ArtifactCanonicalJson, invalid_path, … (+7) |
| 393–808 | `os-store-one-item-publication` | 407 | ArtifactStoreOneItemAuthorityRetirement, impl ArtifactStoreOneItemAuthorityRetirement, ArtifactStoreOneItemSealCheckpoint, ArtifactStoreOneItemSealer, impl ArtifactStoreOneItemSealer |

**`S/🧩️composition/🚪️open/🦀️.rs`** (602 item lines → `os-store-member-open` 402, `os-store-member-open-store` 200)

| lines (from–to) | crate | item lines | first items |
|---|---|---:|---|
| 23–441 | `os-store-member-open` | 402 | MEMBER_OPEN_IDENTITY_BYTES, member_step_grant, record_member_step, receive_member_close, MemberOpenDiagnostic, MemberOpenPhase, MemberOpenProgress, … (+10) |
| 443–646 | `os-store-member-open-store` | 200 | MemberStoreOpenRetained, impl MemberStoreOpenRetained |

**`S/🧵️operation-wire/🦀️.rs`** (276 item lines → `os-store-operation-wire` 181, `os-store-one-item-publication` 95)

| lines (from–to) | crate | item lines | first items |
|---|---|---:|---|
| 16–23 | `os-store-one-item-publication` | 8 | ArtifactPreparedOperationSource |
| 25–34 | `os-store-operation-wire` | 10 | ArtifactPreparedOperationProgress |
| 36–49 | `os-store-one-item-publication` | 12 | ArtifactPreparedOperationError, impl ArtifactPreparedOperationError |
| 51–228 | `os-store-operation-wire` | 171 | text_failure, ArtifactPreparedOperationCursor, impl ArtifactPreparedOperationCursor, operation_wire_preparation_factory_birth_bytes, operation_wire_preparation_factory_source_birth_demand, operation_wire_preparation_factory, OperationWirePreparationFactory, … (+1) |
| 230–317 | `os-store-one-item-publication` | 75 | ArtifactCanonicalTreeSource, impl ArtifactCanonicalTreeSource, ArtifactOperationWire, BatchDigestAdmission, impl ArtifactOperationWire, canonical_tree_source, semantic_schema, … (+2) |

**`S/🧩️composition/📬️publication/🤝️group/🦀️.rs`** (188 item lines → `os-store-engine` 152, `os-store-one-item-batch` 36)

| lines (from–to) | crate | item lines | first items |
|---|---|---:|---|
| 5–41 | `os-store-one-item-batch` | 36 | MemberGroupPreparation, impl MemberGroupPreparation |
| 43–195 | `os-store-engine` | 152 | retain_group_owner, impl ArtifactStore |

**`S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`** (185 item lines → `os-store-space-schema` 122, `os-store-space-history` 39, `os-store-text-io` 19, `os-store-space` 5)

| lines (from–to) | crate | item lines | first items |
|---|---|---:|---|
| 22–23 | `os-store-space-history` | 2 | SQLITE_SNAPSHOT_DIALECT |
| 24–28 | `os-store-space` | 5 | register_sqlite_snapshot |
| 29–62 | `os-store-space-history` | 34 | schema, rows, word, pair |
| 63–93 | `os-store-space-schema` | 31 | write_rows |
| 94–96 | `os-store-space-history` | 3 | semantic |
| 97–115 | `os-store-text-io` | 19 | impl SpaceHistorySnapshot |
| 116–206 | `os-store-space-schema` | 91 | impl SpaceHistorySnapshot |

**`S/🧾️document/🦀️.rs`** (167 item lines → `os-store-document` 44, `os-store-engine-core` 39, `os-store-space-member` 35, `os-store-engine-live` 31, `os-store-initialization-runtime` 18)

| lines (from–to) | crate | item lines | first items |
|---|---|---:|---|
| 26–72 | `os-store-document` | 44 | create_document_envelope, create_document_envelope_from_genesis, create_document_envelope_from_genesis_owners, empty_document_envelope_from_genesis_owners |
| 74–109 | `os-store-space-member` | 35 | edit_ids_for_changes, materialize_document_snapshot |
| 111–116 | `os-store-engine-core` | 6 | now_iso |
| 118–136 | `os-store-initialization-runtime` | 18 | fresh_replica_clock, now_ms |
| 138–160 | `os-store-engine-core` | 23 | uncommitted_edit_ids |
| 162–192 | `os-store-engine-live` | 31 | stamp_primary_operation_identity |
| 194–203 | `os-store-engine-core` | 10 | EMPTY_CHECKPOINT_MESSAGE |

**`S/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/📏️preflight/🫳️borrowed/🦀️.rs`** (75 item lines → `os-store-space-schema` 48, `os-store-space-history` 27)

| lines (from–to) | crate | item lines | first items |
|---|---|---:|---|
| 10–10 | `os-store-space-history` | 1 | Source |
| 11–17 | `os-store-space-schema` | 7 | Node |
| 18–18 | `os-store-space-history` | 1 | absent |
| 19–59 | `os-store-space-schema` | 41 | impl Node |
| 60–84 | `os-store-space-history` | 25 | impl Source, spec, measure |


## Appendix D. Skeleton table for WP0 (node, package directory, project, workspace depth)

`<PKG>` = `M/<node>/📦️packages/🦀️rust` (facade: `K/`), lib root `M/<node>/🦀️.rs`, project name `@semio-tech/<id>-rs` with `<id>` the crate name without `semio-framework-`. `workspace` = as many `..` as the package path has segments below the repo root (`<n>` below). A node marked "new" is a proposal to validate against `🔣️taxonomy.json` (directory emoji must be registered; `⚙️engine` is globally valid); where a validation fails, pick another registered emoji of the same store family and update this table once, before S0.

| # | crate (`semio-framework-…`) | node (new = proposal) | project | `workspace` ups |
|---:|---|---|---|---:|
| 1 | `os-list-delta` | `M/🪡️list-delta` | `@semio-tech/os-list-delta-rs` | 7 |
| 2 | `os-semio` | `M/🧬️semio` | `@semio-tech/os-semio-rs` | 7 |
| 3 | `os-space-sqlite-cells` | `M/🪐️space/🪶️sqlite` | `@semio-tech/os-space-sqlite-cells-rs` | 8 |
| 4 | `os-plugin-registry-schema` | `M/🔌️plugin/📇️registry/📦️deployment/🧬️schema` | `@semio-tech/os-plugin-registry-schema-rs` | 10 |
| 5 | `os-store-space-schema` | `M/🏪️store/🧬️schema` | `@semio-tech/os-store-space-schema-rs` | 8 |
| 6 | `os-extension` | `M/🧩️extension` | `@semio-tech/os-extension-rs` | 7 |
| 7 | `os-pack-tools` | `M/🎒️pack/⌨️cli` | `@semio-tech/os-pack-tools-rs` | 8 |
| 8 | `os-spr` | `M/📡️spr` | `@semio-tech/os-spr-rs` | 7 |
| 9 | `os-dsl` | `M/🗣️dsl` | `@semio-tech/os-dsl-rs` | 7 |
| 10 | `os-store-child` | `M/🏪️store/🪆️child` | `@semio-tech/os-store-child-rs` | 8 |
| 11 | `os-store-ledger` | `M/🏪️store/📒️ledger` | `@semio-tech/os-store-ledger-rs` | 8 |
| 12 | `os-store-retirement` | `M/🏪️store/♻️retirement` | `@semio-tech/os-store-retirement-rs` | 8 |
| 13 | `os-store-replay` | `M/🏪️store/🔁️replay` | `@semio-tech/os-store-replay-rs` | 8 |
| 14 | `os-store-read` | `M/🏪️store/🔗️read` | `@semio-tech/os-store-read-rs` | 8 |
| 15 | `os-store-backbone` | `M/🏪️store/🔗️backbone` | `@semio-tech/os-store-backbone-rs` | 8 |
| 16 | `os-store-canonical-json` | `M/🏪️store/🧵️canonical-edit` | `@semio-tech/os-store-canonical-json-rs` | 8 |
| 17 | `os-store-one-item-publication` | `M/🏪️store/🎮️one-item/📬️publication` | `@semio-tech/os-store-one-item-publication-rs` | 9 |
| 18 | `os-store-operation-wire` | `M/🏪️store/🧵️operation-wire` | `@semio-tech/os-store-operation-wire-rs` | 8 |
| 19 | `os-store-text-io` | `M/🏪️store/🚪️io` | `@semio-tech/os-store-text-io-rs` | 8 |
| 20 | `os-store-snapshot-clone` | `M/🏪️store/🧬️snapshot-clone` | `@semio-tech/os-store-snapshot-clone-rs` | 8 |
| 21 | `os-store-one-item-batch` | `M/🏪️store/📬️one-item-owners` | `@semio-tech/os-store-one-item-batch-rs` | 8 |
| 22 | `os-store-owned-schema-fields` | `M/🏪️store/📦️codec/🧬️owned-schema/🧱️fields` (new) | `@semio-tech/os-store-owned-schema-fields-rs` | 10 |
| 23 | `os-store-owned-schema-decode` | `M/🏪️store/📦️codec/🧬️owned-schema` | `@semio-tech/os-store-owned-schema-decode-rs` | 9 |
| 24 | `os-store-member-open` | `M/🏪️store/🧩️composition/🚪️open` | `@semio-tech/os-store-member-open-rs` | 9 |
| 25 | `os-store-engine-owners` | `M/🏪️store/🏗️initialization/🏠️owners` | `@semio-tech/os-store-engine-owners-rs` | 9 |
| 26 | `os-store-codec-registry` | `M/🏪️store/📦️codec` | `@semio-tech/os-store-codec-registry-rs` | 8 |
| 27 | `os-io` | `M/🚪️io` | `@semio-tech/os-io-rs` | 7 |
| 28 | `os-store-space-history` | `M/🏪️store/📜️space-history` | `@semio-tech/os-store-space-history-rs` | 8 |
| 29 | `os-store-initialization-runtime` | `M/🏪️store/🏗️initialization/🏃️runtime` (new) | `@semio-tech/os-store-initialization-runtime-rs` | 9 |
| 30 | `os-store-durable-group-wire` | `M/🏪️store/🧩️composition/🗄️durable-group/📜️wire` (new) | `@semio-tech/os-store-durable-group-wire-rs` | 10 |
| 31 | `os-store-engine-core` | `M/🏪️store/⚙️engine/🧱️core` (new) | `@semio-tech/os-store-engine-core-rs` | 9 |
| 32 | `os-store-engine-live` | `M/🏪️store/⚙️engine/🎞️live` (new) | `@semio-tech/os-store-engine-live-rs` | 9 |
| 33 | `os-store-engine-stage` | `M/🏪️store/⚙️engine/🗄️stage` (new) | `@semio-tech/os-store-engine-stage-rs` | 9 |
| 34 | `os-store-engine` | `M/🏪️store/⚙️engine` (new) | `@semio-tech/os-store-engine-rs` | 8 |
| 35 | `os-store-initialization-replay` | `M/🏪️store/🏗️initialization/🔁️replay` | `@semio-tech/os-store-initialization-replay-rs` | 9 |
| 36 | `os-store-codec-factory` | `M/🏪️store/📦️codec/🏭️factory` (new) | `@semio-tech/os-store-codec-factory-rs` | 9 |
| 37 | `os-store-space-member` | `M/🏪️store/🧩️composition/👥️space-member` (new) | `@semio-tech/os-store-space-member-rs` | 9 |
| 38 | `os-store-document` | `M/🏪️store/🧾️document` | `@semio-tech/os-store-document-rs` | 8 |
| 39 | `os-store-member-open-store` | `M/🏪️store/🧩️composition/🚪️open/🏭️operation` | `@semio-tech/os-store-member-open-store-rs` | 10 |
| 40 | `os-store-config` | `M/🏪️store/🎚️config` | `@semio-tech/os-store-config-rs` | 8 |
| 41 | `os-store-durable-group` | `M/🏪️store/🧩️composition/🗄️durable-group` | `@semio-tech/os-store-durable-group-rs` | 9 |
| 42 | `os-store-composition` | `M/🏪️store/🧩️composition` | `@semio-tech/os-store-composition-rs` | 8 |
| 43 | `os-store-space` | `M/🏪️store/📜️space-history/🏠️space` (new) | `@semio-tech/os-store-space-rs` | 9 |
| 44 | `os-store-test-support` | `M/🏪️store/🧪️test-support` (new) | `@semio-tech/os-store-test-support-rs` | 8 |
| 45 | `os-store-sync` | `M/🏪️store/🔄️sync` | `@semio-tech/os-store-sync-rs` | 8 |
| 46 | `os-store-sync-native` | `M/🏪️store/🔄️sync/🖥️native` (new) | `@semio-tech/os-store-sync-native-rs` | 9 |
| 47 | `os-store-sync-wasm` | `M/🏪️store/🔄️sync/🌐️wasm` (new) | `@semio-tech/os-store-sync-wasm-rs` | 9 |
| 48 | `os-spr-cli` | `M/📡️spr/⌨️cli` | `@semio-tech/os-spr-cli-rs` | 8 |
| 49 | `os-kernel (facade)` | `K/` (existing package) | `@semio-tech/os-kernel-rs (existing)` | 5 |
| 50 | `os-store-testing-fixtures` (companion, `publish = false`) | `M/🏪️store/🧪️testing` | `@semio-tech/os-store-testing-fixtures-rs` | 8 |
| 51 | `os-store-tests-01` (companion, `publish = false`) | `M/🏪️store/🧪️tests/📦️bin-01` | `@semio-tech/os-store-tests-01-rs` | 9 |
| 52 | `os-store-tests-02` (companion, `publish = false`) | `M/🏪️store/🧪️tests/📦️bin-02` | `@semio-tech/os-store-tests-02-rs` | 9 |
| 53 | `os-store-tests-03` (companion, `publish = false`) | `M/🏪️store/🧪️tests/📦️bin-03` | `@semio-tech/os-store-tests-03-rs` | 9 |
| 54 | `os-store-tests-04` (companion, `publish = false`) | `M/🏪️store/🧪️tests/📦️bin-04` | `@semio-tech/os-store-tests-04-rs` | 9 |
| 55 | `os-store-tests-05` (companion, `publish = false`) | `M/🏪️store/🧪️tests/📦️bin-05` | `@semio-tech/os-store-tests-05-rs` | 9 |
| 56 | `os-store-tests-06` (companion, `publish = false`) | `M/🏪️store/🧪️tests/📦️bin-06` | `@semio-tech/os-store-tests-06-rs` | 9 |
| 57 | `os-store-tests-07` (companion, `publish = false`) | `M/🏪️store/🧪️tests/📦️bin-07` | `@semio-tech/os-store-tests-07-rs` | 9 |
| 58 | `os-store-tests-08` (companion, `publish = false`) | `M/🏪️store/🧪️tests/📦️bin-08` | `@semio-tech/os-store-tests-08-rs` | 9 |
| 59 | `os-store-tests-09` (companion, `publish = false`) | `M/🏪️store/🧪️tests/📦️bin-09` | `@semio-tech/os-store-tests-09-rs` | 9 |
| 60 | `os-store-tests-10` (companion, `publish = false`) | `M/🏪️store/🧪️tests/📦️bin-10` | `@semio-tech/os-store-tests-10-rs` | 9 |
| 61 | `os-store-sync-tests-1` (companion, `publish = false`) | `M/🏪️store/🔄️sync/🧪️tests/📦️bin-1` | `@semio-tech/os-store-sync-tests-1-rs` | 10 |
| 62 | `os-store-sync-tests-2` (companion, `publish = false`) | `M/🏪️store/🔄️sync/🧪️tests/📦️bin-2` | `@semio-tech/os-store-sync-tests-2-rs` | 10 |
| 63 | `os-dsl-tests` (companion, `publish = false`) | `M/🗣️dsl/🧪️tests/📦️companion` | `@semio-tech/os-dsl-tests-rs` | 9 |
| 64 | `os-dsl-fixture-sweep-tests` (companion, `publish = false`) | `M/🗣️dsl/🧹️fixture-sweep` | `@semio-tech/os-dsl-fixture-sweep-tests-rs` | 8 |
| 65 | `os-spr-laws-tests` (companion, `publish = false`) | `M/📡️spr/🧪️tests/⚖️protocol-laws-unit` | `@semio-tech/os-spr-laws-tests-rs` | 9 |
| 66 | `os-store-member-open-tests` (companion, `publish = false`) | `M/🏪️store/🧩️composition/🚪️open/🧪️tests/📦️companion` | `@semio-tech/os-store-member-open-tests-rs` | 11 |
| 67 | `os-store-durable-group-tests` (companion, `publish = false`) | `M/🏪️store/🧩️composition/🗄️durable-group/🧪️tests/📦️companion` | `@semio-tech/os-store-durable-group-tests-rs` | 11 |

## Appendix E. Tools

The item outline scanner is Appendix E of `PLAN-RENDERER-WGPU-CRATES.md` verbatim (`outline.py`); the depth-1 method scanner is the same brace walker started inside an impl body (items at the impl's brace depth that start with `fn`, `const` or `type`). The closure counter is `rwp0-tools/closure.ts` with one extra line mapping `FEATURES` to `feature=` cfgs.
