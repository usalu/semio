# edit-b result (value-edit named-index root + root/receive)
Files: 📇️index/🌳️root/🦀️.rs, 📇️index/🌳️root/🛬️receive/🦀️.rs.
- Preparation::advance passes body to RetainedCloneBinding::close_one; close_step(body,effect) and close_original_step(original,body,effect) return OriginalRetirementPoll (exit_work!, Pending propagates).
- RootRetirement: reply-shaped close_into / recover_close_into / close_original_into / recover_close_original_into via one relay with route (controlled,start); poll children are self-resuming, recovery re-enters the same poll; requires_original_* honour the route.
- project_rows (removed retain_original) -> project_rows_{demand,work_demand,into,recover_into} on RetainedOwnedProjectionAdmission.
- receive: close_poll moved to inherent impl; list child type via <PersistentList<T> as NativeReceive<I>>::Receiver; admission.take_output(effect); children called with body; recover_close_step re-runs close_step for poll children.
Labels: W-edit-b-3 native: 0 errors in my files except edit-a callee arity (index/admission close_step(body,..), close_original_step(original,body,..)).
Rerun: chk.sh W-edit-b-<n> 🌎️hub/Cargo.toml native -p semio-framework-value-edit
Debts: wasm not run; receive tests (🧪️tests) still call project_rows (nested's file); borrowck/type errors past arity unknown until edit-a lands.
Update: aligned to edit-a INTERFACE (close_via over CloseReply, AdmittedNamedKeyIndex::new); Preparation::close_poll(original,body,effect,recover); receive close_poll threads original+recover.
W-edit-b-5 native rc 0 (0 errors whole crate), W-edit-b-6 wasm32-wasip2 rc 0.
Debts: lib only; receive tests still call project_rows.
STATUS: FIXED-verified W-edit-b-6

## Tests step
- root/receive/🧪️tests/🦀️.rs ported: Authorities with source-turn witness, owner advance(control,witness,body,work,grant), close of owner AND root through the original close authority (close_retire: CloseProducer + OriginalEffectAuthority, drive redriven until ready = recover path, RECOVERED counter), project_rows -> RetainedOwnedProjectionAdmission demand/admit/take with denial grants (exact zero progress, zero heap), reader close via close_step(body,effect). Physical ledger law (total.0==total.1) kept.
- Check with `chk.sh W-edit-b-N 🧰️framework/Cargo.toml native --tests -p semio-framework-value-edit` (hub lock lacks the dev-deps). W-edit-b-11: zero errors in my file; remaining 10 errors are in edit-a/edit-c test files, so the test binary cannot build yet.
- Not yet run: tests / pass counts.

## Tests step (final state, 21:5x)
- Target `named_root_receive` ([[test]] in value-edit Cargo.toml; integration test over the public API + edit-a helpers). Run: `tst.sh W-edit-b-N 🧰️framework/Cargo.toml native -p semio-framework-value-edit --test named_root_receive`.
- Fixed at source: receive phase 2 work quote (child+2); source-admission `projectionStart` driven via project_owned (+ quote owned_projection_demand); NamedRecordKey receiver native_terminal_release; drive.rs `S::KeyMetadata` demand now covers the Post/release actions (max with OriginalConstructionCompletion::demand()), which removes the "unsettled reply" drop panic of denied grants; reader closed through close_loop (close_into/recover_close_into).
- Pass counts: 0/2. Remaining: (64,0) heap born without receipt on a funded list-child turn (progress copied_bytes 1 = receiver metadata only) = ownedfix "list denied-turn heap"; not in root/receive code. Pending-close recovery (RECOVERED counter) not yet asserted: blocked behind that failure.
- Driver test (record_matcher_driver) not rerun: persistent crate not compiling (peer edit, native_clone_engine import). drive.rs change is demand-only.
- All [DEBUG] removed (receive, tests, drive.rs).
STATUS: BLOCKED ownedfix list-child heap (64,0); receive 0/2

## Rerun after W-ownedfix-37 (W-edit-b-39/41)
- named_root_receive: 0 passed, 2 failed (ledger failure gone). Both now fail in `owner.demands(BODY,WORK)` at turn 177 of the 1-key case: ValueError InvariantViolated "semantic failure is not available" from decode `🏭️construction/📦️effect/🧳️frame/🦀️.rs:102 semantic_transfer_demand` (row receiver frame), reached through NamedPersistentRootReceiver::demands phase 2 -> list child -> shared row -> record matcher. Not in root/receive code; owner = nested/ownedfix (frame semantic-transfer demand asked outside a semantic-failure state).
- Pending-close recovery assertion not added: unreachable until the receive loop gets past turn 177 (close phase never reached). RECOVERED counter plumbing is in the harness.
- [DEBUG] removed.
STATUS: BLOCKED frame semantic_transfer_demand at turn 177; receive 0/2
