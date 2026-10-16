# 🧱 RESULT OS-KERNEL-WP2-3 (Sonnet executor, 2026-10-16)

Scope given: execute WP2 and WP3 of `PLAN-OS-KERNEL-CRATES.md` and every other WP that does not touch the member-hydration batch files or the `ArtifactStore` engine decomposition (so WP4 of section 10.1 minus the blocked crates, WP7, WP8; WP5 and WP6 belong to the engine agent). No compiler was used for the carve (ingress gate); every claim below says which scan or differential produced it. Nothing was committed, stashed or checked out; the executor did not call `ticket_close`.

## 1. Verdict

* **14 library crates carved, applied to the live tree and verified statically**: `os-store-child`, `-ledger`, `-replay`, `-read`, `-backbone`, `-canonical-json`, `-one-item-publication`, `-operation-wire`, `-snapshot-clone`, `-one-item-batch`, `-space-schema`, `-retirement` (WP2/WP3) and `-owned-schema-fields`, `-owned-schema-decode` (WP4 of section 10.1).
* **Not carved, with the measured reason** (all by `carve.py` dry runs on the live tree, outputs in `🗑️generated/` are not kept; the violation classes are reproducible with `python3 okwp23-tools/carve.py <crate>`):
  * `os-store-text-io`, `os-store-codec-registry`, `os-io`, `os-store-space-history` form one strongly connected component through the trait `ArtifactSqliteSnapshotHost` (`S/🚪️io/🦀️.rs`). Its default methods call `super::sqlite_snapshot_workspace::{project,reconstruct,validate_*}` and `sqlite_snapshot_retirement::{export,import}` (`S/📦️codec/🪶️snapshot-capability`, plan crate `codec-registry`) and `crate::io::io_mechanism::{export_snapshot_file_read,import_snapshot_file_into}` (`M/🚪️io/🦀️.rs`, plan crate `os-io`), while those callees are generic over the trait. The plan puts the trait in `text-io` (rank 2) and the callees above it: a cycle. A cut needs a design decision (callback port inside `ArtifactSqliteSnapshotCodec`, or one merged crate of about 5,300 lines, which exceeds the 3,500 line budget). 6 violations in `text-io`, 9 in `codec-registry`, 13+ in `os-io`, 7 in `space-history` (it also needs `register_native_snapshot_codec` of `os-io` and `bare_artifact_codec`).
  * `os-store-member-open` and `os-store-engine-owners`: `member-open` imports `SpaceMember`, `DocumentStoreOwners` and `ArtifactStoreInitializationRuntime` from the store root, and `engine-owners` contains the hydration-batch file `S/🏗️initialization/🏠️owners/🏭️member` and the store-close port cut out of the `ArtifactStore` root (C1). Both belong to the engine/hydration work.
  * `os-store-sync`, `-sync-native`, `-sync-wasm` (WP7): `S/🔄️sync/🦀️.rs` uses `ArtifactStore`, `BlobStore`, `serialization::ArtifactPackFiles`, `codec_registry::*`, `text_format::*` (15 violations), all of which are blocked above or are engine.
  * WP8 companions: every companion exercises a blocked crate or the engine (`os-store-tests-NN` exercise the `ArtifactStore` root, the member-open and durable-group companions their blocked crates). `os-store-testing-fixtures` (consolidation of 50 fixture files) was not started: decision D6 keeps the taxonomy component files one per component and the consolidated helper mounts live in `S/🦀️.rs` which the engine agent now edits.
* Real compile: refused by the launcher (see section 7).

## 2. Census (canonical counter `closure.ts`, worst of lib/test, native / wasm32-unknown-unknown / wasm32-wasip2; budget 3,500 lines / 28 files / 230,000 bytes)

| crate | files | bytes | lines | verdict |
|---|---:|---:|---:|---|
| os-store-child | 3 | 43,502 | 801 | ok |
| os-store-ledger (`hot-path-census`) | 12 | 104,364 | 1,181 | ok |
| os-store-replay (`hot-path-census`) | 9 | 156,876 | 2,333 | ok |
| os-store-read | 4 | 97,393 | 1,200 | ok |
| os-store-backbone | 5 | 53,478 | 916 | ok |
| os-store-canonical-json | 11 | 90,288 | 1,645 | ok |
| os-store-one-item-publication | 11 | 126,159 | 1,646 | ok |
| os-store-operation-wire | 7 | 90,964 | 935 | ok |
| os-store-snapshot-clone | 14 | 136,587 | 1,440 | ok |
| os-store-one-item-batch | 17 | 189,840 | 2,778 | ok |
| os-store-space-schema | 22 | 63,298 | 1,036 | ok |
| os-store-retirement | 5 | 22,043 | 219 | ok |
| os-store-owned-schema-fields | 3 | 102,959 | 1,497 | ok |
| os-store-owned-schema-decode | 2 | 157,257 | 2,667 | ok |

Command: `python3 okwp0-tools/census_all.py <crate>…` (log `🗑️generated/okwp23-census-all.log`). The corpus ceilings written to `🧪️tests/🧫️fixtures/📦️crate-boundaries/🔣️.json` are `ceil(1.2 x lines)` rounded to the 500 steps (14 producer rows, 13 cut rows for the crates that have a kernel shim line).

## 3. How the carve was done (tools in `okwp23-tools/`, all temporary except the scans)

* `ns.py` (name resolver over the live tree, per crate world), `plan.py`/`assign.py`/`edges.py` (item-to-crate assignment from the planner's table plus the overrides in section 5, item level edge analysis), `rewrite.py` (token level path rewrite: `crate::os_spr::X` -> `semio_framework_os_spr::X`, sibling imports, `pub(crate)` widening only where a consumer outside the crate names it), `carve.py` (pure output builder), `apply.py` (guarded writer: every overwritten file must still equal the text the batch was built from), `manifests.py` (dependencies from the rewritten sources), `corpus.py` (producer and cut rows), `mover.py` (orphan-rule repairs on the live tree).
* Files stay physically in place; the crate root is the node `🦀️.rs`; multi-owner files were split into staging modules `📥️lifted/<src-dir>/🦀️.rs` (`lifted_<module>`), the kernel root `S/🦀️.rs` carries shims (`pub use semio_framework_os_store_X as m;` or merged inline modules with glob `pub use`s). Host-mounted tests that need kernel test doubles were relocated to the kernel root as `#[cfg(test)]` modules: `raw_replay_retirement_tests`, `history_read_admission_tests`, `history_plan_retirement_tests`, `reader_tests`, `canonical_edit_tests`, `native_source_tests`, `canonical_edit_borrowed_tests`, `command_custody_tests`, `resident_backing_tests`, `set_active_space_alternative_tests`.
* Census hook: `S/🧮️hot-path-census` is a `#[cfg(any(test, feature = "hot-path-census"))]` module of `os-store-ledger`; `os-store-replay` forwards the feature; the kernel manifest dev-depends on both with the feature.
* Kernel `Cargo.toml`: +14 path dependencies, +2 dev-dependencies; kernel `📜️script.ts`: selectors renamed for the census.

## 4. Verification (what was actually run, on the final tree unless stated)

| gate | tool | result |
|---|---|---|
| union differential, all 14 crates plus the WP1 crates as modules of one facade (A) against the pristine snapshot (B), patched oracle, lib / test / wasm32-unknown-unknown / wasm32-wasip2 / wasm-test | `okwp23-tools/union2.py all` (log `🗑️generated/okwp23-union-all.log`) | see section 4.1; tree state: before the last 13 text edits listed in 4.3 (they are extern-path or TS-only edits that the sandbox cannot see; per the SPEED rule no second run) |
| definition scan (lost items) | `okwp23-tools/lostitems.py` (multiset of top-level items of the pristine store closure against the live store tree) | 3,684 items before, 4,302 after, **0 lost** (the 1 listed is `artifact_retire_struct!(HistoryColumn…)`, relocated to the root with a longer path) |
| orphan scan | `okwp23-tools/orphan.py` | kernel: 2 benign (`impl DslField for OwnerRef` pre-existing, `TryFrom<OpsSupersededInput> for SupersededInput` local type via alias); crates: 1 false positive (const-generic braces in `impl From<PagedList<Mu,{usize::MAX}>>`, the type is defined in the same crate). Seeded: `seeded23.py` flags `Display for String` and passes it for a local type |
| extern-path scan | `okwp23-tools/extcheck23.py` | 14 crates; remaining 8 MISSING are pre-existing text (`semio_framework_value::retirement::{Owned,Shared}ValueRetirementFactory` in 3 test files exist unchanged in the pristine tree) or a scanner glitch on `use X::{A as N, B as T}`. Seeded: bogus item and typo item are flagged |
| manifest audit | `okwp23-tools/manifest_audit.py` (first-party and third-party idents used by the sources against the manifest, lib-name aliases such as `pack`) | 0 missing after fixes (space-schema lacked `diagnostic`, `io-sqlite-snapshot`, `os-space-sqlite-cells`, `pack-json`, `pack`) |
| dangling attribute scan | `okwp23-tools/dangling.py` | 0 files with new dangling attributes (2 found and fixed, section 5) |
| path literals, rustfmt parse, regions | `okwp23-tools/verify_paths23.py` | 260 files, 460 literals, 3 region findings, all identical to the pristine tree (`S/🦀️.rs` 62/53, `dsl/🦀️.rs`, `protocol-laws-unit`) |
| registry scan | `okwp0-tools/verify_static.py` | 0 failures (67 manifests, 402 kernel partition routes in both launch files) after moving `os-store-space-schema` behind `os-dsl` in `okcrates.RANK` |
| surface parity | `okwp0-tools/pubnames.py os_store` | 281 = 281 names, 0 lost, 0 gained; the WP1 modules are unchanged from RESULT-OS-KERNEL-WP0 |
| test-attribute count | `okwp23-tools/testcount23.py` (unique files over the facade, the WP1 crates, the companions and the 14 crates) | **1077 = 1077** |
| census | section 2 | all within budget |
| corpus gate | `bun test ./🧪️tests/📦️crate-boundaries/🟦️.ts` in `💻️os` | 84 pass, 0 fail when the machine is quiet (under load a 5 s test timeout fires on 1 to 5 tests; two quiet runs were 84/0) |
| store TS suites | per file against a pristine copy of the store module at `HEAD` (`git archive`, symlinked repository around it): 94 files | before fixes 6 files regressed; after fixes **0 regressions**, 1 improvement (`operation-wire` 10/2 -> 11/1). Suites outside the store that read store text: 7 compared the same way, 2 regressed, both fixed (section 5). 171 -> 172 passing tests in the store tree |

### 4.1 Union differential rows

| config | A internal / external | B internal / external | onlyNew | onlyOld | classified |
|---|---|---|---:|---:|---|
| lib (native) | 932 / 3,224 | 1,012 / 2,861 | 10 | 195 | 9 WP0-baseline keyed entries + 1 moved derive helper (4.2) |
| test (native) | 2,515 / 8,779 | 2,569 / 8,239 | 39 | 221 | 10 WP0-baseline keys, extern-re-export artifacts, moved `serde`/`value` helpers (4.2); 22 of the 39 are the WP0 baseline count |
| wasm32-unknown-unknown | 934 / 3,099 | 950 / 2,794 | 10 | 131 | same set as lib |
| wasm32-wasip2 | 833 / 2,991 | 872 / 2,686 | 10 | 125 | same set as lib |
| wasm32 test | 2,446 / 8,357 | 2,429 / 7,882 | 39 | 148 | same set as test |

The key sets of wasm / wasip2 equal lib and the key set of wasm-test equals test (checked by set difference). No new E0412, E0425, E0422, E0423, E0603, E0616, E0624 remains; the `SpaceMemberPin` E0425/E0432 cluster, the `record_tree` macro and the dangling `cfg` entries seen in the first run (26 lib entries) were real defects and are fixed (section 5, items 2 and 5). Wall clock: the five configs took about 50 minutes under load 20 to 40. Sandbox note: the facade-path propagation artifacts excluded from `onlyNew` are 105 to 106 per config.

### 4.2 Classified union residue (not code defects)

* lib / wasm: 9 keyed entries identical to the WP0 baseline (`ArtifactReplayPrepared` 1, `CompositionPin` 4, `FromValue` 3, `crate::Mutation` 1: split pieces of a lumped `unresolved imports` message whose names come through extern re-exports of `os-spr`) plus 1 `value` attribute that has its twin in `onlyOld` (the same derive helper moved from `canonical-edit/🦀️.rs` to the lifted staging file).
* test: the same plus the `serde` twin, and `crate::__os_spr::{Edit,MutationLeaf,MutationId,DiffAlgebra,MutationDiff,command}` and `__os_store_read::SNAPSHOT_READ_LEASE_CAPACITY`: names that `os-spr` / `os-store-read` re-export from extern crates (`os-command`, `value-snapshot-read`) which the sandbox cannot resolve; the pristine tree sees the same names through the kernel hub, where the failed glob hides them.
* `onlyOld` is always larger (192 to 221): errors of the pristine tree whose location moved or that disappeared with the extern glob chain.

### 4.3 Edits after the union run started (extern paths, attributes, manifests, TS pins; the sandbox cannot see them)

removed `use …::value;` (unused) in `one-item/📬️publication/📥️lifted/🧵️canonical-edit`; 11 imports `semio_framework_os_store_ephemeral::{size_of,size_of_val,LaneItemReceipt}` fixed (section 5, item 4); 4 manifests lost the unused `os-store-ephemeral`; `space-schema` gained 5 dependencies; the three lifted staging pieces of space-schema (section 5, item 2); batch b3 (owned-schema) applied after the run began; 7 TS pins updated.

## 5. Plan defects, overrides and repairs found on the way (all handcrafted, no shims)

1. **Item overrides of the planner's table** (`okwp23-tools/assign.py`): `Identities` and `Returned` of `canonical-edit/🧳️source` -> `one-item-publication` (back edge otherwise); `ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS` -> `replay`; the SQLite snapshot row helpers (`schema`, `rows`, `word`, `pair`) -> `space-schema`; the preflight / native / admission / validation / receiving files of the space-history SQLite snapshot -> `space-history` (not carved).
2. **Orphan repairs for `os-store-space-schema`** (the plan counted the crate at 798 lines with no dependencies; it needs `os-spr`, `os-vcs`, `os-pack`, `pack`, `pack-json`, `diagnostic`, `io-sqlite-snapshot`, `os-space-sqlite-cells`): impls of foreign traits for the space types (`ArtifactPack`, `ArtifactDsl`, `OpText`, `ArtifactCanonicalJsonTree` incl. the `record_tree!` macro and its 8 invocations, `ArtifactSqliteSnapshot`, `RetireOwned`/`artifact_retire_struct!`) moved into `🧬️schema/📥️lifted/{💾️binary-snapshot,📝️text-snapshot,📸️snapshot,🧬️mutations,♻️retirement}`. The first version of this repair lost three impls (staging path collision in `mover.py`, found by `lostitems.py`); restored verbatim from the pristine snapshot.
3. **Rank order**: `os-store-space-schema` moved behind `os-dsl` in `okwp0-tools/okcrates.RANK` (the rank was an ordering convention; the real constraint is acyclicity and `os-spr` does not depend on the store).
4. **Wrong extern imports produced by the resolver** (found by `extcheck23.py`): `size_of`/`size_of_val` (std prelude names) and `LaneItemReceipt` had been resolved through a private `use` of the `ephemeral` crate seen through `pub use ephemeral::*`; the resolver now treats glob lookups across a crate boundary as `pub`-only. Fixed 11 imports, and removed the then-unused `ephemeral` dependency from 4 manifests.
5. **Dangling attributes** (found by the union, `SpaceMemberPin` and `pack_rt` were `cfg(wasm32)`-gated): dropping an unused `use` left its `#[cfg]` behind; `rewrite.py` now drops attributes with the statement, two files repaired by hand (`📒️ledger/♻️retirement`, `🧬️schema/📜️space-history`).
6. **Tool defects fixed on the way** (no repo effect): exponential glob lookups, mutated cached results, top-level `use {a, crate::b}` groups, module aliases of hub modules (`use crate::os_spr as protocol`), staging path collisions in the mover, wrong keeper in `Plan._classify`.
7. **TS pins updated** (text pins that read moved sources): `♻️retirement/📦️backing/📦️paged` and `🧩️composition/🚪️open/🧪️tests/🔬️unit/📏️retirement` (`pub(super)` -> `pub`), `🕰️history/📐️planning/🎟️admission` (mount now in `🔗️read/🦀️.rs`), `🧩️composition/📨️emission/📦️owned` (two reads), `🚪️io/🧪️tests/🧫️fixtures/📦️codec-ownership/🔣️.json` (owner paths), plugin `🧩️composition/📨️emission/🌱️genesis` and `🪟️mounted-owned-child` (publication text now in `📬️one-item-owners/📥️lifted/📬️publication`).
8. **Plan defects not fixed here**: the `ArtifactSqliteSnapshotHost` cycle (section 1); `space-schema` dependency list; `member-open` depends on `SpaceMember`/`DocumentStoreOwners`/`ArtifactStoreInitializationRuntime`, which the plan lists below it only through cuts C1..C4 that need engine edits.

## 6. Files

All repository files this task wrote or edited are listed one per line in `okwp23-tools/touched.txt` (143 paths; the WP0 list is `okwp0-tools/touched.txt`): store modules of the 14 crates, 3 new staging folders in `🧬️schema/📥️lifted`, the owned-schema staging under `📦️codec/🧬️owned-schema/🧱️fields/📥️lifted`, 14 `Cargo.toml`, the kernel `Cargo.toml` and `📜️script.ts`, `S/🦀️.rs`, the OS corpus fixtures (`🔣️.json`, `🧬️cuts.json`) and the 9 TS / JSON pins of section 5 item 7. New tools and logs: `okwp23-tools/`, `🗑️generated/okwp23-*`; `okwp0-tools/okcrates.py` (rank).

## 7. Real compile attempt

Tried once, after all scans: `SEMIO_SCRIPT_POLICY='{"version":1,"owner":"repository:cargo:caller","maximumElapsedMilliseconds":3600000}' bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-os-store-child --message-format=short --lib` (cwd = repository root). The launcher started (`[script] {"owner":"repository:cargo:caller","command":"repo","stage":"running"}`) and then failed before invoking cargo with `error: Live Original resource control issuance required` at `requireIssuedResourceControlV1` (`🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:76`, called from `🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts:48`); exit code 1. The raw policy variable does not issue the live resource control the compiler-ingress wrapper demands, so the refusal is the launcher's, not a Rust diagnostic. Log: `🗑️generated/okwp23-real-compile.log`. No Nx route attempt was made (same launcher).

## 8. Residual risks

* No rustc evidence for any crate: privacy, trait scope and method resolution are proven only by the oracle (extern crates unresolved), `orphan.py`, `extcheck23.py` and `manifest_audit.py`. The first real `cargo check` of the 14 crates may show widened-visibility, unused-import warnings (workspace lints are warnings only) or missing `pub` on items reached through the new staging modules.
* Locks (`Cargo.lock`) and `.nx` graphs were not refreshed; the root workspace `members` already list the crates (WP0).
* The root `📜️script.ts` readers (`policyReadOsStoreSource`, `OS_STORE_ROOT_SOURCES`, the 40 scanners of plan 8.5) were not re-pointed (WP0 phase 2); the store TS suites show no regression against `HEAD`.
* The union ran on the tree before the last extern-only edits (4.3); a final full run belongs to the coordinator.
* The engine agent now edits `S/🦀️.rs`; the relocated-test mounts and the shim lines of the 14 crates sit in that file.
