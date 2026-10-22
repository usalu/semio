# Result wire-fix (executor wire-fix, wave 1021a)

Lane `wirefix`, `bun repo cargo --store=shared`, foreground, `nice -n 10`. Logs in `C/🗑️generated/wire-fix/` (scratch, delete at ticket close).

## State

| crate | check lib | check tests | cargo test --lib |
|---|---|---|---|
| `semio-framework-os-store-operation-wire` | green | green | 8/8 |
| `semio-framework-os-store-replay` | green | green | 6/6 |
| `semio-framework-os-store-one-item-publication` | green | n/a | not run |
| `semio-framework-os-vcs` (read ledger) | green | green | 26/26 |

Direct dependents (`check --lib`, 21 crates + `os-inference`): compile now: `store-read`, `store-text-io`, `store-ephemeral`, `os-inference`, `codec-registry`, `presence`. Still red, root causes below, NOT wire/replay defects: `store-one-item-batch`, `store-engine-owners`, `store-initialization-runtime`. The other dependents (engine, bundle, live, space-member, document, test-support, testing, tests-01/04/10, os-kernel) were not reached because those three are upstream of them.

## Root causes fixed

1. `operation-wire`: the inherent `impl ArtifactOperationWire<M>` (696) lived in the wrong crate (E0116) and moved to `os-store-one-item-publication` (`lifted/🧵️operation-wire`); `OperationWirePreparationFactory` lacked `begin_copy_byte_demand` (delegates to its inner factory); `ArtifactOperationImage` was exported but never defined (now an owned-bytes `ArtifactOperationText` leaf); an idle (non-Pack) `ArtifactPreparedOperationCursor` priced pack capacity (392 B) and needed 2 close turns: new `pack_started` flag makes `next_capacity_byte_demand`, `next_close_byte_demand`, `retirement_demands` and `close` zero/immediate for Text, JSON and Hex sources. The stale `ui-scene-pack` dev-dependency was removed (lock edited); the test allocator (`trace-memory-testing`) is now installed, otherwise every heap-law assertion saw zeros.
2. `replay`: `retire_projection` lives on `MutationDiff`, not `ColdMutationDiff`; `step_with_operation_cap` was private across modules; `foreign_step_source` is ambiguous between `Mutation` and `ColdMutation` (explicit `super::Mutation::`); cancelled replays hold no decoder, so `operation_retirement` now closes through a local cold `NativeDecodeControl` bounded by the paid capacity grant; the missing `RetainedCloneRef` to owned-alias constructor is `ArtifactHistoryLedger::retain_read` (vcs) plus `RetainedOwnedProjection::project` (no change to the value crate); `ArtifactMutationSource` calls `retain_original` on the source directly; `close_demand` quoted the controlled-frame size (104) for `post`/`pending`, but the close path allocates `FactoryOwnedRetirement` (88): quote fixed to the real frame.
3. Laws changed by 696 and not carried into the fixtures: turn receipts are independent currencies (the `copy + capacity <= 4096` body clause was removed from `admit_replay_preparation_step`), so fixture row 7 is now `accepted: true` and the TS oracle `🧪️supersede-replay` query drops `?2 + ?4 <= ?8`.
4. Test ports: `control` arguments, local controls, paged-ledger fixture and oracle paths (vcs), `retire_terminal_slot` loop before page release, strong-count order (Arc retirement needs the last owner, so plain drops first), pumping returned registry leases, denied-turn refusals.
5. Dependents: `ColdMutation<P>` + `P: Clone` bounds propagated to `store-read`, `store-text-io` (`replay_ops`, `parse_*`, `settle_parsed_envelope`; stale name `validate_parsed_envelope`), `store-ephemeral` stores (incl. `Published(receipt, progress)` arity, `protocol::ColdMutation`); `presence` rejection duplicated `next_demand`; `os-inference` and `codec-registry` imported the derive macro `RetireOwned` as a trait; `job` `validate_borrowed_context` was `pub(crate)` but used by inference.

## Open (hand-off to cargo-fix / owner of the groups)

- `store-one-item-batch` (29): `📬️paged-one-item/🧬️mutation-apply` child lost its parent imports (`SnapshotProjection`, `fault`, `scalar` helper, `retained_*` functions now in `protocol::retained`), plus `paged-one-item/🦀️.rs:246` `S::Cursor`, plus `ephemeral/📢️publication/🧩️preparation` `advance` arity (3 vs trait 2).
- `store-engine-owners` (12) and `store-initialization-runtime` (21): `P: Clone`, `P: FromValue`, `Mutation: Clone` unsatisfied at `owners/🦀️.rs:613-616,764` (the replay bounds must be propagated or the call sites must stop requiring them).
- Test targets of `store-read`/others were not checked; `cargo test` was only run for the three crates above.

## Notes for others

- `Cargo.lock` (root) edited: operation-wire loses `ui-scene-pack`, operation-wire and replay gain `trace-memory-testing`; a peer manifest change also required `update --workspace` (one `semio-framework-trace` edge added), which the wrapper's `--locked` refused until done.
- Capacity queue waits were 20 to 90 minutes per cargo call under the fleet (swap full); nothing of mine runs now.

## Round 2 (coordinator hand-over: one-item-batch, engine-owners, initialization-runtime)

Private target `/private/tmp/wire-fix/target`, lane `wirefix`; scripts `wire-fix-tools/chk.sh` (check lib+tests / test) and `wire-fix-tools/lib.sh` (check --lib). No `cargo update` was needed this round: Cargo.lock untouched (only `store-ephemeral` moved `value-native-owned` from dev- to normal dependency, same lock edge).

`check --lib` is green for `store-one-item-batch`, `store-engine-owners`, `store-initialization-runtime`, `store-ephemeral` (and `paged-one-item`, `one-item-owners`). Root causes, all 696 half-migrations:

- `paged-one-item/mutation-apply`: only the impl of the retained `MutationApplyEdit` existed; the old struct, `Stage`, `Default` and helpers were stale. Rewritten from the call sites and the `RetainedMutationOwner` template (`Stage` Birth..Done, owner fields as `ManuallyDrop`, `parent_copy`, `context`, `progress`, local `fault`/`scalar`), imports explicit (no globs), `protocol::MutationMessageConcat`.
- `paged-one-item/🦀️.rs`: struct lacked `mutation_source`, `mutation_authority`, `returned_edit`, `returned_edit_retirement`, `returned_post`; `S::Cursor` is `S::Projection`; the sealing path needs `S: SnapshotProjection<Output = S>` (declared on the preparation, the factories and `mutation_apply_preparation_factory`).
- `ephemeral`: `ArtifactEphemeralOneItemPreparation::advance`, `advance_preparation` and both `advance_publish_one` take the original `NativeDecodeControl` (the impl and the bin-03 laws already did); `Published(receipt, progress)` arity; stores need `ColdMutation<P>` (bounds on the impls, `protocol::ColdMutation`); `value-native-owned` became a normal dependency.
- `one-item-owners/lifted/publication`: `ArtifactStoreBatchStage.diagnostics` (`ArtifactStoreDiagnosticStage`) and the batch publication's `transaction` (`ManuallyDrop`), `transaction_close`, `lane_insert`, `lane_close`, `original_registry`, `registry_retirement` were never declared (types taken from the engine constructor and the close ladder); `terminal_is_empty` now covers them. The engine constructor passed `caller_generation` and `lane_ready`, which no code declares or reads: both initializers removed in `store-components/.../store-cursor-operations`.
- `engine-owners`: `MemberStoreProducer` / `DocumentStoreOwnerProducer` already declare `P: Clone + ToValue + FromValue`, `M: Clone + ...`; `funded_member_store_owners` and `funded_bounded_artifact_store_owners_with_semantic_preparation` restated them (no owner gained `Clone`).
- `initialization-runtime`: `read_registry` and `new_with_registry` were never written (struct field `ManuallyDrop<SnapshotReadLeaseRegistryOwner>` plus constructor, `new_with_owner_catalog` delegates); `permits` is `retained_clone_grant_admits`; `effective_forward` and `fold_forward` carry `P: Clone` + `ColdMutation<P>`; `foreign_step_source` disambiguated to the native trait.

Test targets still red (not touched, blind ports): `store-one-item-batch` test `mutation_apply` (derive `SnapshotProjection` emits `semio_framework_value::snapshot_projection::*`, which the value crate no longer has; `OriginalMutationIssuer` undefined; `ConfigApplyMutation::exchange` stale; `protocol` ambiguous) and `store-ephemeral` lib test (presence test fixtures: `os_store` path, `RetireOwned` for `ValueMutation`, private `CLOSE_GRANT`/`observed_step`, duplicate `ColdDiffAlgebra` impl, private registry `state`, `semio_framework_async_macros` dev-dependency).

## First downstream error that is not mine

`semio-framework-os-store-durable-group-wire` (lib, 19 errors), blocking the rest of the walk (`check --lib` of engine, bundle, live, space-member, document, test-support, testing, text-io, codec-registry): `🧩️composition/🗄️durable-group/📜️wire/📥️lifted/🗄️durable-group/🦀️.rs:59:33` E0364 (`valid_edit_identities` is `pub(crate)` but re-exported) and `…/🧾️group-schema-constants-anchors-and-decision-errors/🦀️.rs:146-157` (splitter dropped imports: `DslField`, `Shape`, `FieldValue`, `planning`).

## Round 3 (store group walk)

Green `check --lib` (last verified l29/l30): durable-group-wire, engine (core, live, stage, bundle, root), durable-group, space-member, codec-factory, initialization-replay, sync; the rest of l29 (text-io, codec-registry, test-support, testing-*, member-open*, config, cursor, space, space-history, child) had no errors.
Edited since l29, NOT yet re-checked (loader broke: `workspaceTestingCollectionRoot` export missing in `🗂️workspaces/🟨️.cjs`, waiting for result-loader-fix.md): document (hydration bounds `P: Clone+FromValue`, `M: Clone+ColdMutation<P>` via pm.py) and composition (`ReferenceSpaceMember` facet mounted in space-member `lifted_store`, removed the 19 duplicated wire methods from `SpaceMember`, its store impl, `NoMembers` impl and `space_members!`; re-exported at store root). fem-finish queue: document/composition/durable-group-wire/durable-group are expected green; durable-group was verified green, document and composition are pending one `check --lib`.
Design fixes: binding ladder `advance_binding`/`take_bound`/`from_owners` (journal file), `TakingPrepared` assembly arm, `prepare_staged_diagnostics` (stage), bound planning moved to stage crate, `deflate` feature removed (always on), stage owner fields public, `original_addresses` public, `EditReplay::admit_original_preparation`, orphan replay files mounted in engine (removed from root).
Cargo.lock: `cargo update -p semio-framework-os-store-durable-group-wire` and `-p semio-framework-os-store-engine-stage` (new dep `semio-framework-replication-edit`). A pending `--locked` drift comes from a peer's plugin manifest (`test.4` keys), not from my crates.
Still open: red test targets (`store-one-item-batch` mutation_apply, `store-ephemeral` lib test) and `--tests`/`test --lib` for the group.
