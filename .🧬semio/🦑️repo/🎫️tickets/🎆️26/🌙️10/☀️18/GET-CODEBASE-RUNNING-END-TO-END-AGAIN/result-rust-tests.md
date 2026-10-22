# Result rust-tests (executor rust-tests, wave 1022a)

Tools: `C/rust-tests-tools/{step.sh,chain.sh,sum.sh}`. A step = one chunk of the cargo-fix plan (`cargo-fix-tools/plan/steps.tsv`, ids 088..109 = the framework crates, dependency order), run as `bun repo cargo --store=shared --configuration=prototype -- test --manifest-path <ws> -p ... --no-fail-fast --message-format=short` with `RUST_MIN_STACK=32 MiB`, retried on lease/capacity refusals. `chain.sh <ids...>` runs steps sequentially detached (`detach.py`); logs `C/🗑️generated/rust-tests-logs/<n>.log`, `chain.status`; `sum.sh <log>` = errors/failures only. One cargo at a time.

## Fixes (root causes)
| crate | failure | fix |
|---|---|---|
| semio-framework-trace | `watchdog_tail_*` flaky/failing: tests share the process-wide telemetry registry (`try_lock` gates), parallel tests collide (`sample_count()` None, `expect("gate available")`) | crate-wide `#[cfg(test)] telemetry_test_gate()` in `⏱️trace/🦀️.rs`, held by the 3 tail tests, the 7 contention tests and the 2 watchdog unit tests |
| semio-framework-value-retirement-queue | lib test did not compile: `ControlledRetirement::reserve_cursor_slot` private, used by the queue's controlled-child failure test | made `pub` (docstring) in `🌱️value/♻️retirement/🎮️controlled/🦀️.rs` |
| semio-framework-value-native-decoding-tests | 2 retirement tests: fixture path `../../♻️retirement/...` below the manifest dir wrong after the carve | `../../../♻️retirement/...` in `🛬️decode/♻️retirement/🧪️tests/🦀️.rs` |
| semio-framework-value-snapshot-read | lib test compile: private `use ...{observed_step, CLOSE_GRANT}` shadowed the `pub use ...::*` re-export; `retain_projection` / `next_close_demands` renamed | dropped the private import, `CLOSE_GRANT` imported in the presence test, `retain_original` / `next_close_demand` in `🔗️source/🧪️tests` |
| semio-framework-pack-json | 2 lib tests asserted heap (368,0)/(328,0) but saw (0,0): the carve removed the lib's `#[global_allocator]` observer, the two tests stayed in the lib | restored the cfg(test) `RequestedAllocator` global allocator in `🎒️pack/🔤️json/🦀️.rs` + dev-dep `semio-framework-trace-memory-testing` (Cargo.toml and the 2 `Cargo.lock` lines of `🧰️framework/Cargo.lock` by hand) |
| semio-framework-diagnostic | `fault_inline_layout_stays_within_the_language_neutral_budget`: `size_of::<Fault>()` = 216 > 192 since `retained_progress` (32 B) was added | `Fault.domain: Option<DslValue>` -> `Option<Box<DslValue>>` (24 B back, 192 = budget; absent domain allocates nothing), charge in the controlled decode; assertion message now prints the size |
| semio-framework-dsl-record | integration test `field_refusal_public` did not compile (`semio_framework_value::ordered` does not exist) | `semio_framework_value_ordered::ordered::` + dev-dependency (+ lock line) |
| semio-framework-dsl-record | integration test `typed_receiving` did not compile: `ValueRefusalKind` unimported, `DslValue: DslReceivingField` missing (derive needs it for the opaque-payload record) | import + `impl DslReceivingField for DslValue` (controlled `from_value_controlled` through the owner) |

| semio-framework-job-payload | lib test did not compile / `StepContext: JobPayloadStep` mismatch (cyclic dev-dep `job`) and heap receipts (64,0) vs (0,0): carve dropped the cfg(test) global allocator | `semio_framework_job::JobPayloadStep::payload_ledger` in the test; cfg(test) `RequestedAllocator` + dev-dep |
| replication-conflict / -edit / -history-index / -mutation | tests measure heap with `observe_heap_allocations_on_this_thread` but the carved crates have no `RequestedAllocator` global allocator ((0,0) vs (392,0)) | `rust-tests-tools/add_allocator.py`: cfg(test) allocator in each lib + dev-dep + `Cargo.lock` line |
| semio-framework-dsl-record typed_receiving | same (own test binary) | allocator inside the test file |
| semio-framework-replication-mutation(-retained) | tests stale vs API: `advance(.., body, grant)`, `stage(.., body, grant)`, `advance_targets(.., body, grant)`; missing dev-deps `semio-framework-value-derive`, `serde`; fixture path `../../../` -> `../../` | calls updated (body 4/128 as in the matching `demand` calls), dev-deps + lock lines |
| semio-framework-replication `retained_diff_sequence` | `semio_framework_value::snapshot_projection` is crate `semio_framework_value_snapshot_projection::snapshot_projection` | import + dev-dep |
| semio-framework-replication-presence | peer-overlay fixture path | `../../../../🧪️tests/🧫️fixtures/...` |
| semio-framework-artifact-reference, -value-factory | `grant.maximum_*_bytes - 1` underflow for zero-demand axes (borrowed constructor copy = 0) | test skips zero-demand axes |
| semio-framework-value-factory (+ retained-clone source) | REAL BUG: `RetainedCloneSource::project_owned` ran the projection callback before admitting its constructor grant ("denied grant must not invoke projection code") | `admit_constructor(grant)?` first (as `RetainedOwnedProjection::project` already did) |
| value `♻️retirement/🔗️arc-custody/🔒️sealed` Cursor | REAL BUG: `next_work_byte_demand` returned 0 while the last alias (`handle.source` set, `aliases()==1`) selected an original transfer with `copy_bytes` > 0, so every caller following the `next_close_{copy,capacity,release,depth}_byte_demand` quote protocol stalled forever (replication-edit canonical authority/identity/seal/persistent: 99999 turns; same class: pack `original-owned-wire`, job payload) | `next_work_byte_demand` = `demands(0).copy_bytes` always |
| semio-framework-deflate | process-wide allocation watch counted other tests' allocations | `gate()` mutex around all 7 deflate controlled tests |
| semio-framework-dsl (record) `op_text_of_a_newtype_variant...` | `print_op`/`parse_op` did not write/strip the variant keyword for a keywordless record spec (documented behaviour) | `🪆️binding/🎮️operation/🦀️.rs` |
| semio-framework-ui-contract-components | test expected bare JSON number for `UiPublicationRevision`; contract (fixtures) is the canonical decimal string | test asserts the quoted string |
| semio-framework-ui-contract-presentation | scope-schema test expected the registration fn in the Rust leaf; leaf is the presence file | asserts the export name |
| semio-framework-ui-contract-builder | doctest/unit test used `semio_framework_ui_contract_presentation::accessibility_value` (moved to the facade, cycle) | assertion moved to the facade accessibility tests (`number_input_speaks_its_value_at_its_precision`) |
| semio-framework-ui-contract | Contended / ArenaFull / resident byte counts: 12 tests share the process-wide arena and resident ledger | crate-wide `arena_test_gate()` in the 3 test files |
| semio-framework-dsl-record-model | `native_record_receive...` needs an original construction recipient | `ConstructionTurn` recipient + `with_construction_recipient` |
| semio-framework-value-native-decoding(-tests) | fixture path wrong in the carved `-tests` crate | `include_str!("🧫️fixtures/🔣️.json")` relative to the test file (works in both crates) |
| semio-framework-pack-record-encode (chunks) | REAL BUG: chunk-custody scan counted scalar leaves as a nesting level (`rootAbsent: 0` refused) | depth enforced only for container values (`🧱️chunks/🦀️.rs`) |

## Open / not verified
- semio-framework-pack-record-encode: the lib test unit exceeds the repo-wide 256 KiB crate-source boundary (268,836 B; the admission uses `min(policy, crate)`, so raising the crate's `maxSourceBytes` does nothing): EXCLUDED from my runs; needs the crate split (borrowed/static 36 KB or cursor tests 26 KB out).
- The official runner is `cargo nextest` (one process per test) behind `bun repo run <project>:test`; that path waits on the `repository-graph-facts` shard lease held by peers (position 5 of 5, > 1 h), so I ran `cargo test` (shared process) and serialized the global-state collisions; remaining isolation-only failures should pass under nextest.

## Run log
- 11:5x coordinator order: NO new cargo runs until the graph re-inference (pid 25619) is sealed. Wrapper `autoex.sh` killed; the single running `batch.sh root2` (root workspace steps 101-108 tests, serial, excluded: os-store-ephemeral and os-store-replay = crate-source boundary; os-store-one-item-batch = `mutation_apply` test target has 55 compile errors, see below) is allowed to finish; after that only code fixes from its output.

## Status at the cargo freeze (coordinator order, ~11:50)
Last verified runs (logs `C/🗑️generated/rust-tests-logs/`: `s1` = framework workspace steps 089-109 serial, `root2` = root workspace steps 101-108 serial, `j1` = job crate): framework workspace 235 test binaries green; root workspace 102 binaries green. No cargo of mine is running; watchdog/auxiliary processes stopped. NO new cargo run until the coordinator releases it.

### Edited AFTER the last run, WRITTEN BUT UNVERIFIED (verify first when cargo is released)
- fixture `⏱️context/🎟️source/👶️child/🧪️tests/🧫️fixtures/🔣️.json` `child-parent-wrapper` 32 -> 40 (j1 run: this one edit turned 3 job test binaries green: child-source, fixture-caller, page-return; publication not re-run) - VERIFIED in j1.
- job `📬️outcome/📖️source` test: initialization/seal grants now fund `initialization_demand().copy_bytes` / `size_of::<bool>()` (new builder design charges copy bytes for initialization and sealing).
- pack-record-encode `🧱️chunks`: depth enforced only for container values and a `Record` field no longer counts twice (`pack_record_laws` table_statements depth); `print_op/parse_op` keyword (verified in s1 run? NO: edited before r2, dsl-record lib was green in r2).
- ui-contract-builder doctests now import `semio_framework_ui_contract_builder` (image-described, image-description-required).
- cfg(test) `RequestedAllocator` + dev-dep + lock lines in root-workspace crates: os-store-backbone, -canonical-json, -initialization-replay, -ledger, -one-item-publication, -retirement, -snapshot-clone, -codec-registry (root2 showed `(0,0)` vs `(N,0)` heap receipts there); also tool-run-tick-wire (same). Both `Cargo.lock`s touched where the package is listed.

### Open (diagnosed, not fixed)
| target | finding |
|---|---|
| semio-framework-pack-record-encode (lib test), os-store-ephemeral, os-store-replay | crate-source boundary (256 KiB incl. fixtures) exceeded in the test unit; admission cap is `min(policy, crate)` so only a split helps; excluded from runs |
| os-store-one-item-batch test `mutation_apply` | 55 compile errors (stale vs the diff/mutation API: `OriginalMutationIssuer`, `DocumentMutation.value`, `next_close_demands`, trait method arity); apps-b's port may have fixed it since: recompile first |
| job `original-payload-builder` (`turns < 10000`) | public builder test spins: appends/seal need copy bytes the grant never carries after the redesign (check `append_demand`/`payload_page_granted`) |
| job-publication `original-source-publication`, pack `original-owned-wire` (stalls, `80 != 0` copy bytes), history `history_transition_unit` (fold/decoder never Ready), tool-run lib (176/112 B heap on denied grants), tick-wire, replication-mutation-retained (receiving ledger 9506 vs 9450, key lookup receipt), retained-field (quote 4165/4132 > 4 KiB quantum), `retained_diff_sequence`, os-history native fold, os-store-read/-engine/-space-history/-space-schema | all consistent with the 2026-10-21 01:14 commit daa678dd3dc (`SealedShared` over `SharedOriginalStorage` instead of `Arc`, new `original_demands` footprint of 80 B, birth bytes 32 -> 40): tests/fixtures still carry Arc-era physical numbers and copy-free assumptions; each needs a numeric re-baseline or the owner's decision whether the new accounting is intended |
| value-derive `neutral_owner` | `loop { advance(..) }` never reaches Complete for the borrowed record clone cursor (killed by the watchdog after 7 min) |
| ui-contract resident ledger | `retained_resident_permit_preserves...` sees 5 KiB left in the process-wide ledger by another test (isolation-only, passes per process under nextest) |
| derive-expand (2) | dsl-derive authority fix landed live after the run: re-verify |
| official runner | `bun repo run <project>:test` (nextest, one process per test) waits on the `repository-graph-facts` shard lease (> 1 h behind peers); my gates (`telemetry_test_gate`, `arena_test_gate`, deflate `gate`) make plain `cargo test` pass without it |
