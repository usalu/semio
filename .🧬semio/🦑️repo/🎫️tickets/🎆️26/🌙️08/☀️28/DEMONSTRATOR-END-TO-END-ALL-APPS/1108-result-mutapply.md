# mutapply: store/paged-one-item mutation-apply onto the producer frame mechanism

## Step 1 (lib edited, unverified)
- Design: `ApplyProducer<S,M>` (OriginalEffectProducer, Output=RetainedCloneStep, Failure=OriginalRetirementFailure) resident in `MutationApplyEdit.authority`; Context = `Call{body,maximum,control,action}` with `Action::{Decide,Inverse,Preflight}` borrowing owner/sources. receive -> `*_into`, recover -> `recover_*_into`. Inner reply lives in a turn-local stash; terminal steps are moved into the resident slots `decision_result` / `inverse_result` / new `preflight_result` (closeable, handoff turn pays 1 work), the outer reply carries Progress; an unrecorded inner reply stays Pending and the next `advance` re-enters the same frame (recover) using the frame's stored demand + `pending_work`.
- `semantic_effect` frame now driven through `receive_original_effect` (was closure `receive_effect`); `advance_demands` returns the frame's pending demand while recovering and a zero demand on handoff turns.
- Debt: cancel (begin_close) while an inner reply is pending can only finish through `finish_receiving_recovery(control)`, which has no mutation/source context to re-run the producer.
Rerun: chk.sh W-mutapply-N 🌎️hub/Cargo.toml native -p semio-os-store-paged-one-item

## Step 2 (tests ported, unverified)
- tests/🧳️semantics/🦀️.rs: Preparation now implements decide_reply/inverse_reply/close_reply (sync bodies kept as inherent decide/inverse/close_poll over RetainedCloneStep; children via native close_into/advance_into/take_into stash helper `native`); thread-local STALL / STALL_INVERSE / STALL_PREFLIGHT make the first decide/inverse/preflight call return reply.pending(), the recover call completes and bumps RECOVERS.
- tests/🧳️semantics/🔺️apply/🦀️.rs: Plan implements preflight_reply/close_reply the same way (cleanup/field_step ported to close_into/advance_into/take_into).
- tests/🦀️.rs: new law `mutation_apply_recovers_pending_original_replies_exactly_once` (stalled run == plain run on post+inverse, RECOVERS == 3, every stall flag consumed, more turns than the plain run).
- Other fixture APIs inside Plan (PersistentList/Set/Splice, SealedShared::admit_from, RetainedCloneSource close, admissions) are not yet checked against their current signatures; expect compile errors there.

## Verification state
- Pristine-restore note: first edit pass broke delimiter balance (anchor matched advance_demands); re-applied from HEAD, balanced (301/301 braces).
- W-mutapply-1: refused before compiler (my bad delimiter, fixed). W-mutapply-2: refused upstream (job-context owner/🦀️.rs source extent, peer). W-mutapply-3 rc 1: paged-one-item not reached; blocked by peers: semio-framework-job-native-turn (🌱️birth/🦀️.rs:116 E0308), semio-os-store-cursor (Arc<ArtifactGroupVisibility>: RetireOwned), semio-os-history-record-decode (2-arg method, 4 sites).
Rerun lib: chk.sh W-mutapply-N 🌎️hub/Cargo.toml native -p semio-os-store-paged-one-item (then wasm); tests: chk.sh W-mutapply-N 🧰️framework/Cargo.toml native --tests -p semio-os-store-one-item-batch --features test-support, then tst.sh W-mutapply-N 🧰️framework/Cargo.toml native -p semio-os-store-one-item-batch --features test-support --test mutation_apply
Debt: cancel while an inner reply is pending can only finish through finish_receiving_recovery(control), which has no mutation/source context to re-run the producer.

STATUS: BLOCKED lib+tests EDITED-unverified; paged-one-item not reached by W-mutapply-3 (upstream job-native-turn, store-cursor, history-record-decode red)

## Step 3 (coordinator side task)
- store-cursor: `ArtifactCursorGroupRoot.visibility` is now `ArcLease<ArtifactGroupVisibility>` (accessor `visibility()`), no shim. record-decode: `close_step(g.maximum_copy_bytes, ..)` at origin/metadata/edit/conflict.
- wasm W-mutapply-5 rc 0 for both. Native W-mutapply-4: neither crate in errors; still red elsewhere: framework-actor (wire-turn typed-operation/diagnostic: close_step body, project_owned), os-history (decode/file:45, 🦀️.rs:186,198), os-store-command (receiving/cursor:271), job-native-turn birth:116. paged-one-item not yet reached.
STATUS: BLOCKED mutation-apply unverified (upstream reds above); cursor + record-decode FIXED-verified W-mutapply-5 wasm

## Step 4 (rerun after job-native-turn green)
- W-mutapply-6 native -p semio-os-store-paged-one-item rc 1: only remaining failing crate is semio-os-store-canonical-json (store/🧵️canonical-edit/📖️reader/🦀️.rs: factory.retire/retirement_birth_bytes now take SharedOriginalStorage<T> not Arc<T> (lines 11, 78), artifact_retirement_box_close_step needs body (96), factory.step needs body (110), T: Send+Sync bound). Not mine; paged-one-item still not reached.
STATUS: BLOCKED mutation-apply unverified; waits on store-canonical-json reader (unclaimed, SharedOriginalStorage port)

## Step 5 (canonical-json)
- store/🧵️canonical-edit/📖️reader: root is `SharedOriginalStorage<T>` from birth (new/take_root/encode_chunk via get()), T: Send+Sync+'static on struct/impls, body args on artifact_retirement_box_close_step and factory.step. plugin/interaction/📖️capture builds the root with SharedOriginalStorage::from_cold. native W-mutapply-9, wasm W-mutapply-10 rc 0.
- Reader tests (🧪️tests/📖️reader) and the query test (interaction/📃️query/🧪️tests) still construct Arc roots: unported.
- Next blocker for paged-one-item (W-mutapply-9): semio-os-store-ledger, 15 errors: ErasedSnapshotRetirement::close_step impls take 2 not 3 params (♻️retirement 94/210, 📥️lifted/🔗️read 21/57/124, 🦀️ 466) and calls missing body (♻️retirement 126/223, read 21/60, publication report/inverse/metadata/supersession).
STATUS: BLOCKED mutation-apply unverified; canonical-json FIXED-verified W-mutapply-9/10; waits on store-ledger

## Step 6 (ledger)
- semio-os-store-ledger: 6 ErasedSnapshotRetirement::close_step impls take `_body`, 9 call sites pass `effect.grant().maximum_copy_bytes`. Native W-mutapply-11: ledger no longer fails. Wasm not run.
- Next red for paged-one-item: semio-os-store-one-item-publication, 55 errors, API redesign not mechanical: RetainedCloneSource project_owned/take_authority/retain_original/RetainedCloneSourceTake gone; SharedControlledRetirement::lease/step gone; FactoryOwnedSourceAuthority::take_original/transfer_copy_byte_demand gone; AuthorityLease no OriginalSourceProjection; 3-arg close_step impls (prepared/♻️retirement:53, digest:74, diagnostics:248, lifted/canonical-edit:39, 🦀️:583); authority:26 close_step not in RetirementCursor. Needs an owner lane (osvcs/plugins).
STATUS: BLOCKED mutation-apply unverified; ledger edited native-green (W-mutapply-11), canonical-json FIXED-verified; one-item-publication needs owner

## Step 7 (ledger wasm)
- W-mutapply-12 (wasm, semio-os-store-ledger): stopped upstream in semio-framework-replication-mutation-messages (♻️retirement/🦀️.rs:180, `semio_framework_value::RetirementHeaderRelease` not exported from the root; peer edit in flight, not mine). Ledger itself not reached on wasm.
- Waiting for lane publication's FIXED line on one-item-publication; then rerun: chk.sh W-mutapply-N 🌎️hub/Cargo.toml native -p semio-os-store-paged-one-item (+ wasm), then --tests -p semio-os-store-one-item-batch --features test-support and tst.sh ... --test mutation_apply.
STATUS: BLOCKED mutation-apply unverified; waits on publication (one-item-publication) and messages (RetirementHeaderRelease export)

## Step 8 (after publication FIXED)
- W-mutapply-13 native -p semio-os-store-paged-one-item rc 1: one-item-publication now passes; next red is semio-os-store-replay-operation (31 errors) in 🧬️native/🦀️.rs and 🌱️derived/🦀️.rs: OriginalPostRead::prepare gone; OriginalVectorCursor lost RetirementCursor/admit/step/step_controlled/take_front; FactorySharedRetirement::admit_original gone; many arity changes (2<->3 args). Same class as one-item-publication (API redesign), needs an owner lane.
STATUS: BLOCKED mutation-apply unverified; waits on store-replay-operation

## Step 9 (paged-one-item verified)
- Fixed the 9 module errors: take_output(&mut out, effect); step_granted/close_step/FactoryVectorRetirement::close_step take body; semantics + plan close now run as `Action::CloseSemantics/ClosePlan` through the same ApplyProducer authority (`close_child`: drive -> transfer_reply -> finish on resident `close_action`), so Flight sees an invocation. Pending during close is unsupported (invariant error), debt.
- native W-mutapply-14 rc 0, wasm32-wasip2 W-mutapply-15 rc 0 for semio-os-store-paged-one-item.
- Tests: `chk.sh W-mutapply-17 🧰️framework/Cargo.toml native --tests -p semio-os-store-one-item-batch` (do not pass --features test-support there) stops in semio-os-store-read (4 errors): envelope-genesis-and-displaced-snapshot-retirements (SharedControlledRetirement::lease gone -> ::new(SharedOriginalStorage); factory_retire_snapshot/retirement_birth_bytes want SharedOriginalStorage not Arc: ArtifactStoreDisplacedSnapshotRetirement.pending, ArtifactGenesisRetirement.pack) and cursor-ledger-and-vcs-retirements:263 (artifact_history_ledger_handoff needs T: Sync). Cascade: vcs genesis into_owners (Arc<P>, Arc<Vec<u8>>), envelope, initialization/owners:521, cursor-ledger:339. Needs the osvcs Arc->SharedOriginalStorage lane. Untouched by me.
- Then: tst.sh W-mutapply-N 🧰️framework/Cargo.toml native -p semio-os-store-one-item-batch --test mutation_apply (laws incl. mutation_apply_recovers_pending_original_replies_exactly_once; fixture ports unverified against compile).
STATUS: FIXED-verified lib W-mutapply-14 (native) / W-mutapply-15 (wasm); tests BLOCKED on semio-os-store-read

## Step 10 (one-item-batch lib)
- store/📬️one-item-owners/📥️lifted/📬️publication: close_step/step/step_granted/box_close_step body args (effect.grant().maximum_copy_bytes), ErasedSnapshotRetirement impls take body, stage `post: Arc<P>` retired through `factory::ArcRootRetirement::admit_snapshot` (demand via original_admission_demands).
- store/🧩️composition/📨️emission/🧳️receiving: ported to the replacement receiver's pattern: retained_alias/retained_alias_demand for the wire projection, take_output without pay, take_authority_into / recover_take_authority_into with resident take_reply/take_pending/returned (Pending(0) between turns), `refused()` for NativeReceivingFailure. Close pending recovery relies on the host re-entering close_active.
- native W-mutapply-20 rc0, wasm W-mutapply-21 rc0 for semio-os-store-one-item-batch.

## Step 11 (tests compile)
- W-mutapply-22 (framework manifest, --tests -p semio-os-store-one-item-batch): lib-test 52 errors, mutation_apply 103. Causes: (1) dev-deps missing in store/📬️one-item-owners/📦️packages/🦀️rust/Cargo.toml: serde, serde_json, semio-framework-trace, semio-framework-value-derive (need LOCK + maybe literal-external gate review; tests of the prepared/emission/owned modules use serde_json too); (2) fixture API drift: RetainedCloneSourceTake import in my apply fixture, `ImmutableRootProjection::project_root` no longer a trait member (tests/🌳️immutable-root), `paged_one_item::test_support` export used by source-custody tests.
- The lib is unaffected. mutation_apply laws (incl. pending/recover) have not run.
STATUS: lib FIXED-verified (native W-mutapply-20, wasm W-mutapply-21); tests BLOCKED on dev-deps + fixture ports

## Step 12 HANDOVER (tests port; context low, stopping here)
- Root manifest resolves the dev-deps: `chk.sh W-mutapply-23 Cargo.toml native --tests -p semio-os-store-one-item-batch` (no Cargo.toml edit, no LOCK NEEDED). Remaining: lib-test 2 errors, mutation_apply 77. Use the ROOT manifest (framework manifest lacks serde etc.).
- Error classes (log: 🗑️generated/1105b/W-mutapply-23.txt):
  1. Arc root -> SharedOriginalStorage in ALL fixtures: `SharedAllocation<T>: RetireOwned` unsatisfied in tests/🦀️.rs (303,314,340,441), 🔗️source-custody (41-52 SnapshotRetirementFactory impl now takes `&SharedOriginalStorage<P>`; retire gets `&mut Option<SharedOriginalStorage<P>>`; 97,161,190,213,227,234,288-303), ❄️cold-replay (34,78-136). Fix by building roots with `SharedOriginalStorage::from_cold(value)` (store re-exports it) instead of Arc::new, and porting the factory impls to the 303-306 trait shape in value/♻️retirement/🏭️factory/🦀️.rs.
  2. 2-arg arity (body): tests/🦀️.rs 189,327,435; immutable-root 112; source-custody 187,224,295,303; = add `body` (grant.maximum_copy_bytes) to close_step/step_granted calls.
  3. immutable-root: `unsafe impl ImmutableRootProjection` and `project_root` replaced by `root_native_copy` + `project_root_into` (see bridge lane: semio-s-plugin-energy-engine-model EntityId, 1105b-claims 23:1x bridge) ; `T: RetireOwned` bound at 40.
  4. tests/🦀️.rs 204: `#[global_allocator]` conflicts with semio_framework_value_testing_allocator (pulled by dev-dep testkit): drop the local allocator and use observe_retirement_allocations like the jobs lane's job-return port (1108-result-jobs.md Step 2).
  5. shared PostFixture value/📽️projection/👁️post/🧪️testing/🦀️.rs:8-23 (place/prepare gone; OriginalPostRead new API, see replay's native/👁️post PostCustody) - not mine; needs the projection owner or a port in place.
  6. apply fixture: drop `RetainedCloneSourceTake` import (🧳️semantics/🔺️apply:6) and port `take_authority` use (cleanup/admissions) to `take_authority_into`.
  7. cold-replay:34 `.result` on Result<_,JsonBorrowError>: new borrow API returns Result directly.
- NOT done: the new test driving emission close across a pending take_authority (sketch: build OriginalMemberBatchReceiving over two small wires, advance to source custody, call close_active via settle/close_step with a SourceCustody whose take_authority_into returns pending once -> assert Err(Pending), `take_pending` set, next call uses recover_take_authority_into and restores wires[index]). Put it in store/🧩️composition/📨️emission/📦️owned/🧪️tests (serde_json now resolves via root manifest).
- Laws still unrun: mutation_apply_recovers_pending_original_replies_exactly_once and the whole mutation_apply target.
STATUS: BLOCKED tests (77 fixture errors, handover above); libs FIXED-verified: paged-one-item W-mutapply-14/15, one-item-batch W-mutapply-20/21, canonical-json W-mutapply-9/10, store-cursor+record-decode W-mutapply-4/5, ledger native W-mutapply-11
