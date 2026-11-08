STATUS: assigned crates GREEN (capsplit-116: ordered, shared-slots, persistent, native-default, shared-field; capsplit-120: value paged/receive + paged/receive/map have 0 errors with type-check reached). Split-crate COMPILE UNVERIFIED: capsplit-118 stops in value-edit (140), native-receiving-original (22), job-receiving (1), value lib (other files).

Measurement tool: `C/1105b-tools/capsplit/closure.ts <manifest> <features> [platform] [test]` (the gate's readRustSourceClosureV1, per-file sizes).

| crate | before | after | split |
| --- | --- | --- | --- |
| semio-plugin-runtime-ops (features artifact-app-testing + original-actor-context-testing, as the 18 companion tests-NN use) | 291,057 | 217,343 | new crates -ops-wire (instance lookup, wire limits, drive_self_waking_ready), -ops-document, -ops-surface; ops re-exports them as ops_wire_limits / ops_document / ops_surface_render |
| semio-plugin-instance-app | 263,846 | 240,758 | window projections (engagements, measures, tool measures, pending effects, context menu) -> RenderCtx in instance-render; retain_media_export_result -> MediaTransfers in instance-docs |
| semio-os-store-one-item-batch | 262,967 | 120,717 | paged_one_item -> new crate semio-os-store-paged-one-item (alias re-export, test-support forwarded) |
| semio-plugin-testing-laws | 262,406 | 234,846 | OriginalWorkFixture + typing_run (+ json fixture) -> semio-plugin-testing-caller |

Real refusal: runtime-ops was refused with the features the companion tests use (default features 259,308 only). The audit ceiling for the other three was real (default or testing feature).
Residual: instance-app 91.8 %, testing-laws 89.6 % (the base fixture layers in artifact-app-laws are entangled by `use super::*`; a further split needs a lower fixture crate).

Runs (lock-103 covers the manifests): capsplit-103 (runtime-ops-wire/-document/-surface, runtime-ops, paged-one-item, one-item-batch, testing-caller) rc 1 and capsplit-104 (instance-app, -render, -docs, testing-laws) rc 1; both in C/🗑️generated/1105b/. Errors are only in value crates upstream; no 'Source boundary exceeded/refused' line. Rerun both once the value wall clears (`chk.sh capsplit-1xx Cargo.toml native -p ...`; instance-app also wants a wasm32-wasip2 check).

## Assigned fixes
- `semio-framework-value-ordered`: RootOwner::start_paid takes the allocation effect; step_with/close_original_with/OriginalOwnedEntry::advance|close_step return OriginalRetirementPoll (Pending via RetirementPollExit::settled/finish); work widened to u128 with `narrow_work` at u64 boundaries; removed INSPECTION_WORK / cursor_birth_work; immutable-alias uses `try_uninitialized_arc(grant, effect.actual())`; `advance_insert_controlled` uses prepare_work/commit_work. Exports OriginalRetirementPoll, RetirementPollExit.
- `semio-framework-value-native-shared-slots`: imports of RetainedAllocationEffect, RetainedWorkPoll, ValueRefusalKind, OriginalRetirementCloseLoan from control-contract.
- Not migrated (old API, outside lib check): ordered/native-shared-slots test modules, `OriginalOwnedEntry::advance(grant)` caller in neural `BudgetedOwnedEntry`, and flow/board callers of `advance_insert_controlled` only if they relied on removed API.
- Rerun: `chk.sh capsplit-1xx Cargo.toml native -p semio-plugin-runtime-ops-wire -p semio-plugin-runtime-ops-document -p semio-plugin-runtime-ops-surface -p semio-plugin-runtime-ops -p semio-os-store-paged-one-item -p semio-os-store-one-item-batch -p semio-plugin-testing-caller -p semio-plugin-instance-app -p semio-plugin-instance-render -p semio-plugin-instance-docs -p semio-plugin-testing-laws` (log capsplit-108.txt).

## Round 2 (assigned by coordinator)
- persistent: concat/splice/read/build/set/clone moved to effect/Poll (`RetirementPollExit::settled/finish`), `ImmutableRootCloneCursor` follows the RetainedCloneCursor trait (take with reply, advance_work_demand, close_step Poll, retirement_seed); `root_transferred()` accessor; root re-exports RetainedWorkPoll, RetirementPollExit, retirement_fields_*.
- native-default: NativeDefault is only `default_cursor()`; NativeDefaultCursor::advance/close_step return Poll and take uses the reply form; NativeDefaultSlot runs reserve/seed/publish through a resident `NativeDefaultSlotProducer` authority; array and pair use the new generic `NativeLocalProducer` for child closes and `native_receive_local` (prepare_work/commit_work) for pure local transitions; child output moves via `NativeDefaultSlot::take_local`.
- native-receiver-protocol: additive `NativeLocalProducer/Operation/Authority` + `native_receive_local` (🎛️local.rs).
- native-shared-field: value and retirement actions are `NativeReceivingChild` polls with two resident local authorities (cursor advance/take/close, slot capture/take), `capture_plan/pending_recovery/resume_recovery` dropped (new-generation callers do not use them); new Cargo dep on native-receiver-protocol (LOCK NEEDED sent; check passed without refusal).
- Test modules of these crates (`#[cfg(test)]`) still use the old API and were not compiled.
- Rerun: capsplit-117 command (see above set) once value-native-schema-engine is green; then `chk.sh capsplit-1xx Cargo.toml wasm -p semio-plugin-instance-app` for the wasm check.

## Round 3
- value `paged/receive` (PagedTextReceiver) and `paged/receive/map` (PagedMapReceiver) now implement the new NativeReceiver shape (select_advance, advance_nested, advance_local with reply, take with reply, close_step Poll, recover forwards to the nested sequence child); map keeps a resident cleanup authority for child closes; native-owned root re-exports NativeReceivingChild, NativeLocalAuthority, NativeLocalProducer, native_poll_local, native_receive_local (additive).
- Rerun: capsplit-118 command once value-edit, native-receiving-original, job-receiving and the rest of value lib are green; then wasm check for instance-app.

## Round 4 (capsplit-121, value lib green)
Still red upstream of the split crates, none mine: async (3), deflate (4), io-control (78), io-sqlite-model (36), machine (9), pack-json-canonical (35), replication-retained-field (56) and -key (6), tool-run-entities (24), value-edit (45), value-retirement-owner (8), value-snapshot-read (112), os-semio (6), os-vcs-entity-identity (5), plugin-window-config-mutation (7) and -registry (6), ui-contract-text (22), ui-locale (13). No source-boundary refusal anywhere. The instance-app wasm check cannot start before these clear.
