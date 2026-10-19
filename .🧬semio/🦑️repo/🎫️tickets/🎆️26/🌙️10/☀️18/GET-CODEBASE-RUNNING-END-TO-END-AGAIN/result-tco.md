# Result tco (executor prefix `tco`): test-companion access, root batches 68 and 76

`C` = this ticket, `F` = `🧰️framework`, `S` = `F/🛍️products/💻️os/🔨️modules/🏪️store`. Sonnet 5.5, foreground, no sub-agents, no modifying git command, no package.json/bun.lock edit. Logs: `C/🗑️generated/tco-*` and `cc-b068.*`/`cc-b076.*` (runner copy with retries: `C/🗑️generated/tco-run.mjs`, adhoc command in `tco-cmd.txt`).

## 0. Finding that changes the task

Of the ~365 errors in `cc-b068`/`cc-b076`, only a minority were crate-private access. The rest were (a) companion wiring (missing dependency, missing `use`, wrong crate path), (b) tests that had not followed an API move (`ArtifactStore` fields moved into `log`/`owners`/`wire`/`cursor` components, `OwnedValueRetirementFactory` moved to `semio_framework_value::factory`, `ArtifactStoreOwnedDisposer` takes `&dyn ArtifactStoreCloseHost`, `ComputePool::prepare_job_params` takes a `ComputeJobParamsPreparation`), (c) cfg(test)-only serde derives that cannot cross a crate boundary. Also, the old per-crate attribution of `cc-tools/crate-errs.mjs` was wrong (with `--keep-going` the buffered errors are printed before the next `could not compile` line, so neural/vcs/directory errors were listed under neighbours).

Pattern used for access (as in `job`, HDR-2/3): `#[cfg(any(test, feature = "testkit"))]` accessor `impl`s or `#[doc(hidden)] pub` on types the crate already hands out through a public signature; companions enable the feature in `[dependencies]`/`[dev-dependencies]`. No `cfg(test)` across crates, no test moved back, no compat re-export.

## 1. Changes by crate under test

| crate | change |
|---|---|
| `semio-framework-os-kernel-neural-engine-model` | serde derives and the hand-written `Serialize`/`Deserialize` impls of `Dictionary`, `Cardinality`, `Value`, `Schema`, `Tree`, `Neuron`, `ChannelSpec`, `OperatorInfo`, ... moved from `cfg(test)` to `cfg(any(test, feature = "testkit"))` (5 files); `testkit = ["dep:serde"]` + optional `serde` dependency. Fixes the in-crate tests of `neural-engine`, `-registry`, `-budgeted` (their dev-dependency already enabled `testkit`). |
| `semio-framework-os-vcs` | `#[cfg(test)]` `test_allocation` alias + global allocator (`semio-framework-trace-memory-testing`, new dev-dependency) in the crate root, as `pack` does; the vcs unit tests and `entity-identity/control` laws called `crate::test_allocation`. |
| `semio-framework-os-kernel-dsl-derive-expand` | `MutationAggregateSourceAuthority`, `MutationAuthorityCommon`, `MutationLeafJson`, `MutationsAttrs`, `CompositeAttrs` became `#[doc(hidden)] pub` (they are returned by functions the `testkit` module already exports; fields used by the companion were already `#[doc(hidden)] pub`). |
| `semio-framework-os-services` | `FileStamp` `#[doc(hidden)] pub` (type of the public `OwnedFileChangeWatcher::previous`). |
| `semio-framework-value-snapshot-read` | `testkit`/test accessors: `SnapshotReadLeaseRegistry::state_lock`, `SnapshotReadLeaseRegistryState::set_cleanup_cursor`, `SnapshotReadLeaseRegistryOwner::alias_custody`, `SnapshotReadLease::slot_index/slot_generation`, `ErasedSnapshotRead::swap_owner`. |
| `semio-framework-pack-json-retained-schema` | `OwnedSchemaRecordCursor::decode_pages` (testkit) replaces `record.tokens.pages`. |
| `semio-framework-os-store-engine-owners` | `DocumentCatalogOwner::as_ref` (inspector). |
| `semio-framework-os-store-engine` | `lifted_store::StoreLog` is now re-exported (it carries `OPEN_CONFLICT_CAP`/`RESOLVED_CONFLICT_CAP`, formerly inherent consts of the composed store). |
| `semio-framework-os-store-ephemeral` | `PresenceStore::local_root` (testkit). |
| `semio-framework-os-store-composition` | new `testkit = []`; `OwnsRoot`/`LinkRoot` derive `Clone, PartialEq, Eq` under `any(test, testkit)`. |
| `semio-framework-os-store-canonical-json` (`🧵️canonical-edit/🧳️source`) | `ArtifactStoreCanonicalSource::source_constructor_demand` `pub` (sibling of the public `constructor_demand`). |
| `semio-framework-os-store-one-item-publication` | `ArtifactStoreOneItemSealCheckpoint` serde derives `any(test, testkit)`. |
| `semio-framework-os-mcp-bridge` | session types/fields `pub(super)`/private -> `pub(crate)` in `🧵️bridge/🪢️session/*` (the `bridge_quick` law is a crate-root sibling of `session`). |
| `semio-framework-os-mcp-bridge-codec` | `ValidatedShellFrame` `pub` (already in the public `ShellDecodeStep::Complete`). |

## 2. Companion and test wiring

* `os-directory-tests-1/-2`: `semio-framework-value` dependency, `FromValue` derive import, `client` alias (+ `semio-framework-os-directory-client` dependency).
* `os-services-tests`: `InteractiveJob` and `OriginalWorkerFixtureCaller` trait imports; compute-pool laws build `BatchJobParams` directly (cancel alias = `ctx.cancel.clone()`, the same construction the refusal law already used) instead of the removed 6-argument `ComputePool::prepare_job_params`.
* `server-tests`: removed `extern crate semio_framework_replication as protocol;` (the replication lib is named `protocol`).
* `os-store-tests-01/02/03/05/06/09/10`: dependencies added (`one-item-batch`, `one-item`, `ephemeral[testkit]`, `composition[testkit]`, `engine-owners`, `schema-state`), imports fixed (`LaneItemReceipt`, `lifted_member_group::MemberGroupPreparation`, `FromValue`, `test_support` alias, `os_spr` path, dead `use super::{fixture_mutations, snapshot_read}` and `physical_test_close_grant`), stale API ported (`store.0.envelope` -> `store.0.log.envelope`, `created.*_factory` -> `owners`, `to.dag` -> `log.dag`, `OwnedValueRetirementFactory` -> `factory`, `StoreLog::OPEN_CONFLICT_CAP`, disposer trait signatures), borrow fixes (`drop(cx)` before the next `StepContext` on the same `sequence`: `StepContext` has a `Drop`), snapshot-read/registry/cursor accessors in the tests.

## 3. Verification (compile executor lanes, `C/🗑️generated/tco-run.mjs` = copy of `cc-tools/run.mjs` with fast foreign-cargo wait and retries for `database or disk is full`/`Resource lease timed out`)

| run | scope | result |
|---|---|---|
| 15:08 | batch 68 (25 crates, `--tests`) | only `os-services-tests` (private `FileStamp`) and `server-tests` (`extern crate`) failed; every other crate compiled: dsl-derive-tests, directory-tests-1/2, neural-engine/-registry/-budgeted, os-vcs, os-kernel-dsl-derive, ... |
| 17:55 | batch 76 + the two above | `os-services-tests`, `os-mcp-bridge`, `os-store-tests-04/06/07/08/10` compiled; failed `store-tests-01/02/03/05/09`, `server-tests` (after the fixes of section 1/2: second layer) |
| 20:10 | store-tests-01/02/03/05/09, server-tests | store-tests-01/02/03/05/09 compiled; `server-tests` only 5 `E0659 ambiguous Room/Member/Outflow` (testkit aliases vs the now public presence types) |
| final | server-tests, store-tests-03 | not obtained: three attempts failed in the launcher with transient image-roster errors after 24-30 min each (concurrent executors); the runner was stopped |

The remaining failures of batch 76 (`plugin-operation`, `plugin-interpreter-runtime`, `ui-render-text`, `ui-wgpu-raster`, `surface-paint`, `infinite-*`, `os-flow-host-source`, `os-mcp-schema`, `editor`, `os-renderer-wgpu-media-slots`) are library carve defects of the ui/plugin carve (missing `os_command`/`schema_state` dependencies, `RetireOwned` bounds, `interpreter` path, `CoreError -> TurnFault`), not test-companion access; they belong to those executors.

## 4. Open

* **Not compiled after the last edit**: `server-tests` (removed the `Member`/`Room`/`Outflow` aliases from `F/🛍️products/🖥️server/🔨️modules/📡️gateway/🪝️testkit/🦀️.rs`, the types are now `#[doc(hidden)] pub` and come through `gateway::*`; fields of `Room`/`Share`/`Outflow` that were `pub(super)` are `pub`) and `store-tests-03` (`StoreOwners::RESOLVED_CONFLICT_CAP` + `os-store-cursor[testkit]`; it compiled in the 20:10 run except for the ambiguity, which is server-only). Re-run: `CC_CMD="pl check --manifest-path Cargo.toml -p semio-framework-server-tests -p semio-framework-os-store-tests-03 --tests --keep-going --message-format=short" bun C/🗑️generated/tco-run.mjs C 399` (waits for foreign cargo, retries transients). Everything else of batches 68 and 76 that belongs to test companions compiled in a real run.
* Production drift found, not touched (not companion access): `F/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🌉️router-effect-handler-and-jobs/🦀️.rs` still calls the removed 6-argument `ComputePool::prepare_job_params(ctx, retained, now, params, control, wallet)`; the lib has `prepare_job_params(&mut ComputeJobParamsPreparation, params, cx)`. `F/🔨️modules/📡️replication/🎮️mutation/♻️retirement/🦀️.rs` imports `semio_framework_value::retirement::OwnedValueRetirementFactory` (moved to `semio_framework_value::factory`). `F/🛍️products/🖥️server/🔨️modules/📡️gateway/🧪️tests/...` untouched.
* New Cargo features/dependencies (no `package.json`/`bun.lock` change, nothing under the frozen `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/**` or `🏗️build/📥️inputs/🔏️revision/**`): `testkit` on `os-store-composition` and `neural-engine-model` (`dep:serde`), dev-dependency `semio-framework-trace-memory-testing` on `os-vcs`; companion dependencies listed in section 2. Cargo.lock refreshes are done by the runner (`update --workspace`).
* Infrastructure observed while verifying: concurrent `bun repo cargo` runs from other executors starve each other (each attempt costs 15-30 min of planning; transient `System image plist required`, `Bounded unique parent kernel roster required`, `SQLITE_FULL` in the fixed-capacity source-lease database and `Resource lease timed out: cache-prune`); `cc-tools/run.mjs` `waitForeign` uses `pgrep -f "bun repo cargo"`, which also matches any shell whose own command line contains that text (use `bun repo ca[r]go`), which deadlocked waiters for ~10 minutes.
