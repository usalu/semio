# RESULT W4: `semio-framework-os-kernel-db` carve (OS-KERNEL-DB)

Executor: Sonnet 5.5, foreground, no sub-agents. Tools: `🪓️monolith-split/osdb-tools/` (python). Logs: `🪓️monolith-split/🗑️generated/osdb-*.log`. Staging: session scratchpad `osdb/` (the repo tree is untouched until the switch step).

Status: DONE AND SWITCHED INTO THE REPO (not compiled: the cargo launcher is down, see section 8). Sections 1 to 4 are the plan, 5 onwards the execution results.

## 1. Measured baseline (before any edit)

Canonical counter (`readRustSourceClosureV1`, `🗑️generated/census.ts`, `ONLY=semio-framework-os-kernel-db`):

| crate | files | bytes | active lines | verdict |
|---|---|---|---|---|
| `semio-framework-os-kernel-db` (lib `db`) | 29 | 2,614,222 | 51,618 | over (lines ratio 12.9) |

Shape of the source (tool `outline.py`, `scc.py`, `modgraph2.py`): one facade crate (`📦️packages/🦀️rust/🦀️.rs`, 121 lines) that `#[path]`-mounts 28 module files of the product tree `🛢️db/*`. Every module starts with `use crate::*;`, so the real dependency edges are invisible in the manifest; the crate root re-exports the family through `pub use db_durability::*`, `db_policy::*` and a handful of explicit names. Lib closure lines per module: engine 9,708, storage 8,799 (+ sqlite 1,135, postgres 1,334, neo4j 1,479, writer 671 + 566, driver-runtime 168), artifact 6,257, wal 3,047, sync 2,662, index 2,659, compact 1,987, cli 1,545, state 1,474, query 1,438, snapshot 1,247, actor 989, observe 754, projection 655, security 620, cluster 512, preview 504, conflict 450, policy 246, durability 214, ids 200. Test files add 25.5k lines (engine 5,414, artifact 2,862, storage 3,350, wal 1,970, sync 1,036, compact 957, ...). Only consumer of the crate: `🌎️hub/📦️packages/🦀️rust` (as `db`); the root `Cargo.toml` also holds three unused alias rows (`-db-state`, `-db-storage`, `-db-wal`).

Real compile: `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-kernel-db` at start: `Finite original maintenance authority required` (log `🗑️generated/osdb-cargo-check-start.log`). Same refusal as every wave-4 peer, so verification is the patched resolution oracle plus scans (section 3).

Oracle baseline (rustc nightly-2026-07-20, `--emit=metadata -o /dev/null`, `extern crate` lines removed): native 1,057 errors, `--test` 2,785 errors; every one is E0433/E0432/attribute about an external crate (`protocol`, `store`, `pack`, `semio_framework_*`, `sqlx`, `neo4rs`, `serde_json`, ...). Typeck is never reached (probe with a deliberate private-field access reports nothing), so the oracle covers name resolution and path privacy only; field/method privacy, trait-in-scope and coherence are covered by the scans in section 3.

## 2. Findings that drive the design

1. Module graph (name-use analysis) is a clean layered DAG: ids < durability < policy < observe < actor < storage(io, contract, backends) < state < wal/snapshot/index/query/conflict/preview/projection < compact/sync/cluster < artifact < engine < cli. The unit that is tightly cyclic is the DbIo kernel inside `🗄️storage/🦀️.rs` (ledger, page arena, task model, lost-owner parking, backend registry, task arena, drive loop, WAL writer table and controller): one 1,660-line SCC at item level plus layers above and below it, 6.5k lines in all. It can only be cut along the topological order of the item graph, not along semantic components (component graph is one SCC, `osdb-tools/compgraph.py`).
2. Engine, artifact and wal are layered by region: engine = capability-open, catalog-read, catalog-bootstrap, create-catalog, shared types, document-mount, `Database`, `ArtifactHandle` with submit and history; artifact = command/diff/conflict, durable-group receipt, state, engine, snapshot pages, ledger tail, history replay, actor runner; wal = records/sink and recovery/segment/ArtifactWal. The largest inherent impls are `DatabaseCreateCatalogState` (1,020 lines), `ArtifactEngine` (928), `DbIoPageWriter` (197), `Database<E>` (597); none exceeds one crate, so no god-object decomposition of a state struct is needed; the god-object rule is satisfied by keeping each struct and its whole impl in one crate and by moving the misplaced Drop impls and live counters (`DATABASE_*_LIVE`, `Drop for Database*State`) next to their types.
3. `DbBackend` has a `#[cfg(test)] Fault(Box<db_fault_testing::FaultStorage>)` variant and `FaultStorage` wraps a `DbBackend`: a test-only cycle between the dispatch enum and the fault harness. The fault harness is split: the storage wrapper (`FaultStorage`, `FaultScript`) moves with the dispatch crate behind a `testkit` feature, the laws (which drive `ArtifactEngine`, `Database`, `ProjectionEngine`) become a test-support crate above engine.
4. `cfg(test)` helpers are used across modules in tests (`db_io_test_pool`, `process_isolated_law`, `history_capacity_test_lock`, the fault harness). Across crates they need a `testkit` feature (`cfg(any(test, feature = "testkit"))`) and dev-dependency features.
5. Tests use `use super::*` against private items (the DbIo retained fixtures touch 70 private items). Tests move with their subject crate; items used across a crate boundary are widened to `pub`, computed by identifier use (sound over-approximation), never by guess.

## 3. Method

Staging and switch: everything is generated into the scratchpad by `osdb-tools/build.py` from a pristine snapshot of the tree, verified there, and only then copied into the repo in one scripted step (with anchored edits for the shared registry files). If any verification fails the repo stays at HEAD content.

Per target file the generator emits: docstring (unique emoji), explicit `use` lines synthesised from the original scope rules (local items, explicit `use`, then the `crate::*` root glob), the original item text unchanged, `//#region` pairs repeated and balanced where a region spans files, visibility widened only where another crate or module uses the name. No comments inside definitions, no aliases, no compatibility layers: all consumers (hub, root `Cargo.toml`, owners, launch routes, scripts that name the db manifest) are updated at once.

Verification: (a) census rows for every crate, lib and test cfg; (b) union-sandbox resolution oracle: every crate simulated as a top-level module of one crate (`crate::` and `pub(crate)` rewritten per crate so private paths are really private), run native, `--test`, wasm32-unknown-unknown, wasm32-wasip2, accepting only errors that name an external crate; (c) scans: definition/visibility scan for fields and methods used across crates, trait-in-scope scan (traits imported `as _` by method-name use), orphan and inherent-impl scan (every inherent impl and every trait impl must be local to the crate of its type or trait), cfg(test)-across-crate scan, lossless line multiset against HEAD; (d) rustfmt parse, path resolution, TOML and JSON parse; (e) `bun repo cargo` at start and end (expected to be refused).

## 4. Crate plan

Module crates keep their old module identifier as lib name (`db_wal`, `db_state`, ...), so qualified paths such as `db_wal::WalRecord` stay valid without rewriting. Whole-module crates (lib root = the module file, package under `<module>/📦️packages/🦀️rust`): ids, durability, policy, observe, actor, security, state, conflict, preview, projection, cluster, snapshot, index, query, compact, sync, cli. Split modules get child nodes: storage (DbIo kernel in three topological layers, trait contract, memory, fs, writer-in-kernel, sqlite, postgres, neo4j, driver-runtime, dispatch with the fault wrapper), artifact (command, durable-group, history, engine, actor), engine (types, capability-open, catalog-read, catalog-bootstrap, create-catalog, mount, core `Database`, handle), wal (record, recovery). The facade `semio-framework-os-kernel-db` (lib `db`) shrinks to explicit `pub use` of the surface the hub uses. Final sizes, the dependency graph and the file list are recorded in section 5 after execution.

## 5. Result: census after (canonical counter, lib and test cfg, 41 producer crates plus the facade)

Gate (manifest `rust-source-boundary`): 262,144 bytes, 32 files, 4,000 active lines. Brief target: 230,000 bytes, 28 files, 3,500 lines. Rows are `files / bytes / active lines`; the last column is the verdict of the test row (`ok` = under target, `above-target` = under the gate, over the brief target).

Before: 1 crate, 29 files, 2,614,222 bytes, 51,618 active lines. After: 42 crates (41 new producers plus the facade `db`, 2,077 bytes / 60 lines, the only crate the hub depends on), largest lib closure 3,316 lines (`db_io_core`), largest test closure 3,854 lines (`db_sync`). Over-gate rows: 0.

| level | lib | package suffix | lib closure | test closure | test verdict |
|---|---|---|---|---|---|
| 0 | `db_ids` | `db-ids` | 1 / 8994 / 201 | 3 / 16667 / 275 | ok |
| 1 | `db_durability` | `db-durability` | 1 / 9938 / 216 | 2 / 15240 / 343 | ok |
| 1 | `db_observe` | `db-observe` | 1 / 32739 / 757 | 2 / 45543 / 1069 | ok |
| 2 | `db_policy` | `db-policy` | 1 / 10549 / 246 | 3 / 15269 / 336 | ok |
| 2 | `db_security` | `db-security` | 1 / 32851 / 623 | 2 / 51661 / 983 | ok |
| 3 | `db_actor` | `db-actor` | 1 / 41885 / 991 | 2 / 52790 / 1254 | ok |
| 3 | `db_io_core` | `db-io-core` | 1 / 145242 / 3316 | 5 / 168790 / 3670 | above-target |
| 4 | `db_io_registry` | `db-io-registry` | 1 / 92078 / 1968 | 3 / 104685 / 2097 | ok |
| 4 | `db_storage_driver_runtime` | `db-storage-driver-runtime` | 1 / 8140 / 169 | 2 / 10270 / 207 | ok |
| 5 | `db_index` | `db-index` | 1 / 127139 / 2660 | 3 / 159570 / 3368 | ok |
| 5 | `db_io_driver` | `db-io-driver` | 1 / 91730 / 1763 | 7 / 199506 / 3693 | above-target |
| 5 | `db_state` | `db-state` | 1 / 55993 / 1475 | 2 / 72608 / 1909 | ok |
| 5 | `db_wal_log` | `db-wal-log` | 1 / 111591 / 2249 | 4 / 128995 / 2796 | ok |
| 6 | `db_conflict` | `db-conflict` | 1 / 23735 / 450 | 2 / 34510 / 685 | ok |
| 6 | `db_projection` | `db-projection` | 1 / 36595 / 656 | 2 / 59086 / 1091 | ok |
| 6 | `db_query` | `db-query` | 1 / 63976 / 1439 | 2 / 96483 / 2074 | ok |
| 6 | `db_snapshot` | `db-snapshot` | 1 / 69119 / 1250 | 3 / 88743 / 1802 | ok |
| 6 | `db_storage_fs` | `db-storage-fs` | 1 / 69420 / 1251 | 5 / 103870 / 1730 | ok |
| 6 | `db_storage_memory` | `db-storage-memory` | 1 / 66534 / 1146 | 6 / 145334 / 2344 | ok |
| 6 | `db_storage_neo4j` | `db-storage-neo4j` | 1 / 84967 / 1478 | 3 / 98366 / 1707 | ok |
| 6 | `db_storage_postgres` | `db-storage-postgres` | 1 / 77919 / 1333 | 4 / 99214 / 1685 | ok |
| 6 | `db_storage_sqlite` | `db-storage-sqlite` | 1 / 70895 / 1135 | 4 / 91715 / 1412 | ok |
| 7 | `db_preview` | `db-preview` | 1 / 24019 / 503 | 3 / 41689 / 845 | ok |
| 7 | `db_storage` | `db-storage` | 1 / 48851 / 720 | 8 / 92692 / 1752 | ok |
| 7 | `db_wal` | `db-wal` | 1 / 43395 / 828 | 8 / 179741 / 2597 | ok |
| 8 | `db_artifact_history` | `db-artifact-history` | 1 / 76642 / 1543 | 3 / 126253 / 2021 | ok |
| 8 | `db_compact` | `db-compact` | 1 / 114574 / 1991 | 3 / 175125 / 3328 | ok |
| 8 | `db_engine_capability_open` | `db-engine-capability-open` | 1 / 63387 / 1232 | 3 / 112180 / 2112 | ok |
| 8 | `db_engine_catalog_bootstrap` | `db-engine-catalog-bootstrap` | 1 / 69349 / 1365 | 2 / 106143 / 2086 | ok |
| 8 | `db_engine_catalog_read` | `db-engine-catalog-read` | 1 / 63039 / 1252 | 3 / 106859 / 2021 | ok |
| 8 | `db_sync` | `db-sync` | 1 / 141733 / 2666 | 3 / 231835 / 3854 | above-target |
| 9 | `db_artifact_contract` | `db-artifact-contract` | 1 / 65122 / 1068 | 2 / 72457 / 1223 | ok |
| 9 | `db_cluster` | `db-cluster` | 1 / 27136 / 516 | 3 / 57448 / 918 | ok |
| 10 | `db_artifact_engine` | `db-artifact-engine` | 1 / 116836 / 2045 | 4 / 209533 / 3090 | ok |
| 11 | `db_artifact` | `db-artifact` | 1 / 92866 / 1681 | 6 / 228778 / 3279 | ok |
| 12 | `db_engine_base` | `db-engine-base` | 1 / 41390 / 941 | 2 / 49104 / 1136 | ok |
| 13 | `db_engine_catalog_create` | `db-engine-catalog-create` | 1 / 99293 / 2044 | 3 / 216956 / 2984 | ok |
| 13 | `db_engine_handle` | `db-engine-handle` | 1 / 93956 / 1977 | 2 / 110927 / 2334 | ok |
| 14 | `db_engine` | `db-engine` | 1 / 61606 / 1043 | 6 / 199493 / 3206 | ok |
| 15 | `db_cli` | `db-cli` | 1 / 71473 / 1547 | 3 / 116965 / 1840 | ok |
| 15 | `db_fault_testing` | `db-fault-testing` | 1 / 46817 / 766 | 2 / 61837 / 1056 | ok |
| 15 | `db` (facade) | `semio-framework-os-kernel-db` | 1 / 2077 / 60 | 2 / 10455 / 237 | ok |

Above target but under the gate (documented residuals, all line counts only; bytes are under 230,000): `db_sync` test 3,854 lines (lib 2,666 + unit tests 1,038 plus the embedded wal fixture), `db_io_core` test 3,670, `db_io_driver` test 3,693. Splitting them needs a second test-only crate per subject or a lib split of the sync/io kernel; deferred (see 9).

## 6. Crate graph

A strict DAG with 16 levels (0 to 15, table above), 0 cycles, 0 dev-dependency cycles (`finalize.py` prints both; a dev-dependency back-edge to a crate that depends on you is a duplicate-type hazard, so tests referencing a higher crate live in that higher crate). Shape:

- L0 to L3 leaves: `ids`, `durability`, `observe`, `policy`, `actor`, `security` (whole-module crates, lib name = old module identifier, so `db_wal::X` paths stay valid).
- L4 to L8 state: `state`, `conflict`, `preview`, `projection`, `cluster`, `snapshot`, `index`, `query`, `compact`, `sync` (whole-module crates).
- DbIo kernel (one 1,660-line item-level SCC, the writer + release + storage core is a tightly cyclic mesh): cut by topological order into `db_io_core` (traits, credit ledger, page arena, `claimed_backend`), `db_io_registry`, `db_io_driver`; backends `db_storage_memory`, `db_storage_fs`, `db_storage_sqlite`, `db_storage_postgres`, `db_storage_neo4j`, `db_storage_driver_runtime`; dispatch `db_storage` (`DbBackend`, facet enums, `FaultStorage`/`FaultScript` gated `any(test, feature = "testkit")`); fault laws `db_fault_testing`.
- `db_wal_log` (frame/record log) then `db_wal`.
- artifact: `db_artifact_contract` then `db_artifact_history` then `db_artifact_engine` then `db_artifact`.
- engine: `db_engine_base`, `db_engine_capability_open`, `db_engine_catalog_read`, `db_engine_catalog_bootstrap`, `db_engine_catalog_create`, `db_engine_handle`, then `db_engine` (`Database`, supervisor); `db_cli` on top; facade `db`.

The graph is recomputed from the manifests by `bun test ./🧪️tests/📦️crate-boundaries/🟦️.ts` from the manifests (acyclic check with the independent `graphlib` SCC oracle).

## 7. Files changed

- Db tree (`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db`): 175 files written, 7 deleted (the 28 old module files were re-homed or thinned; facade `📦️packages/🦀️rust/🦀️.rs` moved to `🦀️.rs`). 41 new `📦️packages/🦀️rust/Cargo.toml` plus `📋️project.json`; test files stay next to their subject; `include_str!`/`include!` paths re-relativised (external store fixture is still read from `🏪️store`).
- Authored language-agnostic boundary test (new): `🛢️db/🧬️schema/📦️crate-boundaries/🔣️.json` (draft-07 schema), `🛢️db/🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json` (corpus: package, manifest, entry, dependencies, forbidden dependencies = every transitive dependent), `🛢️db/🧪️tests/📦️crate-boundaries/🟦️.ts` (Bun TOML vs `@iarna/toml`, Ajv schema oracle, `graphlib` SCC oracle, lib path, dependency equality, forbidden edges, six launch routes in `launch.json` and the seed, owner row, workspace membership, lib and test closure under the gate, one owner per production file).
- Consumers re-pointed: root `Cargo.toml` (41 members added, 3 dead db alias rows removed), `📋️build-owners.json` (41 rows), `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` (246 routes each = 41 x 3 targets x 2 configurations, same names/group/order scheme), hub (27 Rust files `db::db_storage::` to `db::storage::`, `db::db_engine::` to `db::engine::`; hub `📜️script.ts` ledger source now `🗿️artifact/⚙️engine`), root `📜️script.ts` (four db audit readers read the whole crate family through the new `policyReadRustPolicyFamily`: all roots' production first, then the test evidence; 20 audit snippets re-quoted to the carved paths, e.g. `db_storage::DB_IO_PAGE_BYTES` to `db_io_core::...`, `pub(crate)` to `pub`; the CLI census regex `db::actor::block_on` to `db_actor::block_on`), `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts` (live-law cargo commands: crate manifests, law names `fence_conformance::...`, `round_trips::`).
- Source-text laws (scan tests that `include_str!` Rust sources) were the biggest hidden coupling: 47 scan tests analysed against the original source by `srcfix.py`. Pure scans were re-homed into the crate that owns the scanned text (the artifact submit/history laws into `db_engine_handle`, the replay-cap laws into `db_artifact_history`, the runner laws into `db_artifact`), mixed ones keep their place with the embed re-pointed, 7 multi-file tests were split per file (`srcfix_splits.py`, e.g. the crc/tokenizer law into a history half and a wal-log half), the global `retained_retry_owners...` law was replicated into the 11 engine and artifact crates (each polices its own source), and quoted `pub(crate)`/`db_x::` snippets were rewritten the way the code was. The preview law now reads the preview crate's own manifest and forbids dashed package names too.
- Balance moves so no test closure exceeds the gate: the concurrent-write module and the two durable-group tests moved up into `db_artifact` (`assign.py` `BALANCED_UP*`); the contract-source assertion tail of the durable-group journal test moved into `db_artifact_contract`.
- Not touched on purpose: `🔒️dependencies.json` (stale generated), Cargo locks (launcher down), taxonomy registration (`directory-kind-unresolved` sweep is step 6 of the compile-window checklist).

## 8. Verification (real output)

Launcher: `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-kernel-db-ids --lib --offline` at start and end of this ticket fails with `Finite original maintenance authority required` (cache maintenance authority, not a db problem); `bun repo run @semio-tech/os-db-ids-rs:check ...` fails with `Canonical build task identity required`. No crate was compiled by cargo.

Resolution oracle (union sandbox, rustc nightly-2026-07-20, every crate as a top-level module, accepted errors only for external crates), final stage:

```
[native] external-accepted=1378 flagged=0
[test]   external-accepted=3337 flagged=0
[wasm]   external-accepted=1112 flagged=23   (identical set on wasm32-wasip2)
```
The 23 wasm flags are all references from ungated code to `cfg(not(target_arch = "wasm32"))` items (`SqliteStorage`, `db_storage_driver_runtime`, `ArtifactAuthority`, `ArtifactRunnerTerminalJob`); the original has the same ungated references (`db_artifact::ArtifactAuthority` at engine line 7029, `Sqlite(...)` in `DbBackend`), so the baseline aborts earlier on the missing externals; the two carve-introduced wasm gaps (`FsStorage` import, `claimed_backend` import) were fixed with explicit gates (`finalize.py native_gates`).

Static scans on the stage (and live, which equals the stage byte for byte except the authored boundary files):

```
scans.py:   no orphan or inherent-impl violations
defscan.py: not-pub / unresolved cross-crate references: 0; sensitivity check: seeded private ArtifactId flagged 2 new problem(s); restricted-visibility inherent methods used by other crates: 0
checks.py:  files with path > 240 bytes: 0; cargo manifests parsed: 42; rustfmt parsed 100 files, failures 0
toml-check.ts (live db tree): manifests 42 problems 0   (Bun.TOML equals @iarna/toml, lib paths, boundary keys, workspace + path deps resolve)
lossless.py: 3,600 original items, 3,664 carved (helpers copied per test file); the 11 'missing' are tests whose text changed on purpose (include paths, quoted paths, facade rewrite); all present under the same names
```
Census: `over-limit rows: 0` (section 5). Parse checks after the switch: `launch.json` 12,272 configurations, seed 9,304, `📋️build-owners.json` 2,483 owners, root `Cargo.toml` 472 members all parse; the three edited `📜️script.ts` files transpile.

Tests run: `bun test ./🧪️tests/📦️crate-boundaries/🟦️.ts` in the db tree: 45 pass, 0 fail (1,728 expects, includes lib and test closure gate checks for all 42 crates, owner rows, 246 routes in both launch files, workspace membership, SCC acyclicity). The os `📦️crate-boundaries` test: 60 pass, 1 fail, a 5 s timeout of `all new kernel owners enforce active source caps ... test` under machine load 70+ (not a content failure; rerun when load is low). `workspace-contract` `resolveCargoPackageName`: the db case passes in content but timed out at 5 s under load.

## 9. Residual risks

1. Nothing was compiled by cargo. Oracle-clean means names resolve; it does not cover trait-in-scope for method calls, field privacy beyond the identifier-use widening, `cfg` interplay, or type inference. Expect a first-compile round per crate (checklist step 3, bottom-up by level above).
2. Over-widening: visibility was widened by identifier use (`pub(crate)` to `pub`, cross-crate members to `pub`, cfg(test) support to `cfg(any(test, feature = "testkit"))`); a few laws that quoted `pub(crate)` now quote `pub`. `testkit.json` (scratch) lists cfg(test) member candidates left gated.
3. Test-only features: `testkit` and `claimed-backend-laws` are declared per crate; `db_fault_testing` is gated non-wasm; the dispatch `db_storage` crate pulls `FaultStorage` only under `any(test, testkit)`.
4. Root `📜️script.ts` interactivity audits (`verify interactivity`) cannot run outside the original parent Script process, so they were checked statically: every quoted snippet of the 14 db audit functions that exists in the original source and not in the carve was re-quoted (20) or listed (1: the CLI regex, fixed). Audit logic that counts across files (`interactivityProductionSource`) now sees all roots of a family; counts may differ by crate-boundary effects. Run `verify interactivity` in the compile window.
5. `include_str!` source-scan laws now live in the owning crate; the three above-target test closures (section 5) stay at 3,670 to 3,854 lines.
6. Cargo locks and `🔒️dependencies.json` are stale (`update --workspace --offline`, `verify dependencies write-baseline` in the compile window, checklist steps 1 and 2); the hub lock follows the facade, whose path and package name are unchanged.
7. Taxonomy memberNames are not validated for the new folders (same as earlier carves); folder names follow existing emoji vocabulary.
8. Peers changed `🧪️tests/🔬️interactivity-db-io-direct-writer/🟦️.ts` (a replication rename) during the work; the switch kept their edit (drift check now only guards files the carve touches).
9. The facade `semio-framework-os-kernel-db` has no `📋️project.json` and no launch routes (as before); the other 41 crates have six routes each.

## 10. Tools

`osdb-tools/` (python and bun, all scratch under the ticket): `build.py` (compose into stage), `widen.py`, `testkit.py`, `srcfix.py` + `srcfix_splits.py`, `finalize.py`, `manifest.py`, `oracle2.py`, `scans.py`, `defscan.py`, `lossless.py`, `checks.py`, `toml-check.ts`, `census-stage.ts`, `closure-stage.ts`, `fixture-gen.ts`, `register.py`, `switch.py`, `run_oracle.sh`, `boundary-files/`. Re-run order: `run_oracle.sh native test wasm`, `census-stage.ts <sp>`, `fixture-gen.ts`, `switch.py dry|apply` (single-shot: the drift check refuses a second apply).
