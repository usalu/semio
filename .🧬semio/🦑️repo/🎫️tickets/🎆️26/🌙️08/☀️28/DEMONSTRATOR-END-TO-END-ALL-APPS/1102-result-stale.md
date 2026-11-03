STATUS: Stale FIXED and proven where reachable (patch applied; law test green; verfolgen load now passes hydration and the whole archive load). Verfolgen smoke still RED, now on harness custody (per-turn fixture caller cannot retire), not on Stale. Energy mounted tests blocked earlier by a separate dispatch fault.

## Root cause
`history-invalid ... Stale` is `MemberOpenDiagnostic::Stale` from `RetainedPersistedDocumentHydration::check_authority`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🧬️hydration-types-constructors-and-drive/🦀️.rs:304`),
which requires `cx.operation()/cx.generation()` of the stepping job to equal the values captured at construction.
The only construction site of the archive-load parent hydration
(`🧵️instance/📄️docs/📥️staging/🪜️maintenance-ladder/🦀️.rs:243-255`, HydrateParent) captured
`OperationId(active.operation)` (the host-chosen archive-load slot id: `pending.operation`, `RESTORE_DOCUMENT_LOAD_OPERATION = 1<<62`, seq)
and `Generation(stores.store.generation_now())` (document store generation). The hydration is stepped from
`drive_document_archive_load_retirements` (ladder:534) inside `RuntimeLiveCleanupJob::advance_source`, whose `cx` carries the
cleanup worker job's own operation id and generation. Two unrelated id spaces: the first `step` after creation is always Stale.
Not a snapshot, cursor, revision or work-accounting problem; the bundled examples are fine. Latent since 1bae6115bc5 (2026-10-16);
the 10-28 split of check_authority (82dd1da2006) only separated Cancelled/Expired from Stale. It hits every pane that loads a document
through the archive-load path (all panes with a bundled example, plus checkpoint restore), so the demonstrator page cannot boot any fixture.
The other construction site (store composition open, `from_decoded_pack`) passes the job's own `self.operation/self.generation` and is correct.

## Fix
Patch `C/1102-tools/stale/patches/hydration-job-authority.py` (fail-closed, anchor-count asserted; supersedes energy's `hydration-binds-job-authority.py`):
1. create the hydration with `cx.operation(), cx.generation()`;
2. before each `step`, re-bind `owner.operation/generation` (pub fields) to the current `cx`, because the hydration is owned by the archive load and spans several turns (and a session may be replaced), not by one job turn.
check_authority still guards cancellation and expiry.

## Law test (written, not run)
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs`:
`persisted_hydration_authority_is_the_stepping_job_identity_not_the_archive_load_slot` (slot-style ids -> Stale; job-bound ids -> never Stale).
It proves the mechanism natively and does not need the patch to compile. The pane-level proof is the existing mounted tests
(energy `editor-mounted-tests` "case 600 loads", and every pane's mounted load smoke) after the patch is applied: they must stop reporting Stale.
Residual risk: after the Stale fix the mounted load may expose a further fault in later phases; unknown until run.

## To run in the next window
1. `python3 C/1102-tools/stale/patches/hydration-job-authority.py` from the repo root.
2. via slot.sh, label W-stale: `cargo test` the store-document hydration crate unit test above, then `cargo check --target wasm32-wasip2 --lib` for the plugin crate, then energy `editor-mounted-tests`.

## Update 21:50 (coordinator follow-ups)
### (1) Scan for the same id-space mismatch
Authorities compared against `cx.operation()/cx.generation()` (grep over `🧰️framework`, ~20 sites). Only the archive-load parent hydration is created with a foreign id space and stepped inline under another job's cx. Others checked:
- history decoder (`document_archive_history_decoder`): captures no ids, steps through `StepTurn`; safe.
- store replacement initializer (`drive_initializer`): runs its own worker session built from the slot operation + artifact generation (job identity = those ids); consistent by design.
- member open (`drive_member_open` -> `M::begin_open`): `InitialMemberStoreOpen::begin(admission, cx, ..)` and genesis/replay authorities capture `cx.operation()/cx.generation()` at birth; consistent. Residual: the replacement's own `closure` (`OwnedDocumentClosure::new(self.operation, self.generation, ..)`, checked at closure.rs:137 against `cx`; here `drive_closure(grant)` takes no cx, so no comparison).
- Not extended: no second site needs a patch from reading; the mounted energy load (which crosses hydration, replacement initializer, members, closure, commit) is the empirical check of the rest.
### (2) Why mounted tests passed before (false-green analysis)
`from_pack(.. OperationId(active.operation), Generation(store.generation_now()) ..)` and `check_authority` coexisted since 2026-10-10 (c44e964518f). Then the ladder stepped the hydration with a PRIVATE context it built itself with exactly those ids (`StepContext::new(OperationId(active.operation), Generation(self.store.generation_now()), ..)`, c44e964518f ladder:985-993), so the check passed by construction. The work-accounting migration (signature `advance_document_archive_load(.., cx: &mut StepContext)`, caller cx threaded through) removed the private context but left the creation ids. So it is a regression of the migration (10-28..10-31), not a long-latent failure; my earlier "latent since 10-16" was wrong. Energy's mounted test never ran before (1030/1101 results: "not run"), so it was never green; tests that passed before the migration and have not been re-run since are not evidence. Any other place where the migration replaced a privately built `StepContext::new(OperationId(x), ..)` by a threaded caller cx while an owner kept the old ids has the same shape; grep `StepContext::new(` in production code at c44e964518f vs now to list them (not done).
### (3) Verification
Chain `C/1102-tools/stale/run.sh` (patch applied at 21:47, W- slot labels): law test, wasm --lib check, energy editor-mounted-tests, verfolgen smoke. Logs `C/🗑️generated/1102/stale-{law,wasm,energy,verfolgen}.{txt,rc}`; `stale-all.rc` at the end. Waiting in capacity queue (memory floor) at the time of writing: UNVERIFIED.

## W3 results (patch applied 21:47)
- wasm check `-p semio-framework-plugin --lib --target wasm32-wasip2`: GREEN, rc 0, 0 errors (`C/🗑️generated/1102/stale-wasm.txt`).
- law test (bin-02): did not compile because of an unrelated stale test line (`bin-02/🔬️unit/🦀️.rs:707` resolved `ArtifactStore` to the testing double, which lacks `derived_checkpoint_preview_admission_demands`); fixed test-side by qualifying `semio_framework_os_store_engine::lifted_store::ArtifactStore`. Rerun still to do (`stale-law.txt` is the failed compile).
- energy `editor-mounted-tests` (`stale-energy.txt`, rc 1, 0 passed / 2 failed in 4.4 s): both now fail BEFORE the archive load, at `mounted-bestest/🦀️.rs:30` `dispatch_fixture_action("setActiveExample")` with `WorkLimit "original action fixture dispatch exceeded its authored turn bound"` (every `handle_action` turn returns `Ok(None)` for `OriginalFixtureCaller::maximum_turns()` turns). The test file changed 11-01 (ccac6866cc1) from `dispatch_original_fixture_command` to `dispatch_fixture_action`, so this is a second, independent blocker upstream of hydration (action never completes in the fixture harness); the Stale site is not reached, so energy cannot judge the patch yet. Owner: energy/testkit/runtime (fixture action endpoint never settles).
- verfolgen smoke: queued (`stale-verfolgen.txt`), result below when it lands.

## W3b (coordinator decision: per-poll fresh cx in the load helpers)
- Law test `persisted_hydration_authority_is_the_stepping_job_identity_not_the_archive_load_slot`: GREEN native (`stale-law2.txt`: 1 passed). It asserts both halves: slot-style ids are Stale, job-bound ids are not.
- Testkit API `drive_fresh_turns` + `LAW_DOCUMENT_LOAD_MAXIMUM_TURNS` in `🧪️testing/🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs` (fresh `OriginalFixtureCaller` + context + full fuel per turn, retirement paid from fresh fuel, `WorkLimit` at the turn bound) with laws `fresh_turn_laws::{a_multi_turn_load_gets_fresh_fuel_every_turn_and_completes_past_one_contexts_fuel, a_load_that_never_completes_stops_at_the_turn_bound}`: GREEN (`stale-fresh4.txt`, 2 passed).
- Wiring the load helpers (`load_document`, `plugin_load_document`) to it was tried and REVERTED. Evidence (`stale-verfolgen2/3/4.txt`): with fresh fuel per turn the 243 KB verfolgen demo loads completely (hydration, replacement, members all pass; no Stale, no fuel exhaustion), but every per-turn caller then fails its paid retirement ("original fixture caller retirement exceeded its supplied turn bound", testing-laws .../fixture-dispatch...:314): the app keeps owners bound to the first caller's source parent/ledger after the turn, so a fresh caller per turn cannot retire, and each turn's caller would need that custody to be returned. Leaving the wiring in would also turn every currently passing small-document pane test red, so the helpers are back to the single shared context (their original behavior).
- Conclusion for the owner (testkit/job): the faithful production model is ONE long-lived source parent with a fresh context per turn (`OriginalCallerSourceParent::borrow_context`), not a fresh caller. `OriginalFixtureCaller::context()` is single-issue (`budget.take()`), so the harness needs a second, per-turn `borrow_context` with a fresh per-turn fuel account; the open question is the source's lifetime return-budget stock (`fuel_remaining()` is `min(per-turn, return_budget remaining)`), which would still be 32768 unless the budget is renewed per turn like production. That is a job-crate change (new `testkit::renew_turn(cx, fuel)`), larger than the harness, and I have not made it.
- verfolgen smoke status: RED at `pane smoke: the bundled example loads` (fuel exhaustion in the single shared context, `stale-verfolgen.txt`) with the shared helpers; hydration itself is past Stale. The browser path (fresh cx per reactor turn) is not affected by this harness limit.
- Also changed (test-only): `🧰️framework/.../🏪️store/🧪️tests/🌆️bin-02/🔬️unit/🦀️.rs:707` qualified the engine `ArtifactStore` (was resolving to the testing double; bin-02 did not compile).
- Scripts: `C/1102-tools/stale/run.sh`, `run2.sh`, `run3.sh`, `run4.sh`; logs `C/🗑️generated/1102/stale-*`.
