STATUS: operation-wire lib tests COMPILE (reviewfix8-105 green build, was 75 errors); F1 re-fixed on committed-work evidence; its law and the job-level single-charge test UNVERIFIED (run blocked, see Round 3).

## F1 REAL (re-fixed after settling the doubt)
Original F1: `advance` re-debited the cumulative Pending work on every repoll (3 then 2 debited 3+5=8). My first fix (debit each delta once) was wrong in kind: the doubt is settled by reading where fuel is paid.
Evidence: (1) `with_native_receiving` -> `original_receiving_outcome` (job/⏱️context/🛬️owned) only admits the return-budget scope (stock claim, `consume_fuel(work)` for admission overhead); `settle_receiving_scope` only calls `consume_receiving_retained` (physical progress, never fuel). (2) The existing job test `native-progress` charges the work itself after the receiving call (`cx.consume_fuel(completed_work)`); `with_native_encoder_work` does the same with `native.actual_work()` delta. (3) `NativeDecodeControl::actual_work()` counts committed work; Pending work stays a reservation until it commits, so a committed-work delta is single-charge by construction and charges nothing for a pending poll.
Fix in `operation-wire/🧳️replacement/🦀️.rs`: `advance` measures `control.actual_work()` before/after the receiving callback and debits that delta once; removed the pending-total debit, the `settled()` `consume_fuel(copied_items)` (F2), and `performed.max(1)` in `close` (F2, now debits `performed`). `pending_receive_work()` stays an observation only.
Job-level test (UNVERIFIED, not yet run): `job/📬️outcome/⏸️continuation/🧪️tests/🎟️source/🦀️.rs` `actual_native_receiving_leaves_fuel_to_its_caller_which_debits_the_committed_work_once` asserts the scope leaves `fuel_remaining` untouched, `committed == turn.work`, and one `consume_fuel(committed)` lowers fuel by exactly `turn.work`. If its first assert fails, the scope pays fuel and the caller debit must go.

## F2 REAL (the two extra debits fixed under F1); the fabricated `pay(effect,1)` in advance_native kept: it is a real committed effect unit, not a charge outside the turn
`settled()` adds `consume_fuel(copied_items)` and `close()` adds `performed.max(1)`; both look like extra/fabricated fuel against W1/W2 and the "do not also debit elsewhere" ruling, but whether `with_native_receiving`'s scope pre-pay (`admit_scope` consume_fuel) already covers it needs a run. Removing blind could starve turns. Owner store5.

## F3 PARTLY REAL, not changed
`polled()`/`settled_step()` map Pending to zero-progress Ok and drop `work` (lines confirmed: custody:25, snapshot-clone:521, config-apply:99). Real as a ruling mismatch (Pending should be a typed refusal); depends on unverified `effect.nested` accounting. store3/store4 own snapshot-clone.

## F4 REAL (W4), not changed
Close paths return `Progress(default)` forever when the grant never fits (replacement `close_native`, original `close_step`, one-item close_ladder). Fix needs a typed WorkLimit refusal modelled on the job's `close_original_admission`; cross-crate, left to owners.

## F5 REAL, not changed
snapshot-clone:550 `Err(error)=>{self.cancelled=true;..}` cancels on a too-small step grant (confirmed). store3/store4 file.

## F6 NOT A BUG as stated (design debt)
`.min(64)` confirmed: text 4, output 4, common 1, root 3, plus tests 3. Chunk-not-cap refactor of store5's crate; larger than a review fix.

## F7 PARTLY REAL
`ArtifactStoreOneItemLiveAuthority::mint` is `pub` without witness (confirmed at one-item/📬️publication/🦀️.rs:87, store4's file); unwrap/expect and Drop-assert items are style debt, not re-verified individually.

## F8 FALSE POSITIVE as a stub risk
`finish_receiving_recovery` is an explicit InvariantViolated refusal, not `Ok(Default)`; the Seal-phase single-charge question stays open.

## F9 REAL, minor
`let _=cursor.begin_close()` at snapshot-clone:586 (confirmed) discards the bool; duplicated `terminal_is_empty` terms harmless.

## F10 FALSE POSITIVE
Audit itself found manifests clean.

## F11 not verified
Blocked-forever when cancelled and the 64-byte slice are unconfirmed.

## Runs
- reviewfix8-101 (native --lib test): rc 1, 75 errors, ALL in store5's unported in-crate test files (common/history/edit/original/replacement canonical tests); none in replacement/🦀️.rs. The new law could not run.
- reviewfix8-102 (native --lib check of the fix): still queued for a slot at 20:17, no verdict. Rerun: `C/1105b-tools/chk.sh reviewfix8-103 <operation-wire Cargo.toml> native --lib`. Compile of the F1 edit is UNVERIFIED.

F6 detail: every `.min(64)` is a per-turn chunk with a continuation (state resident, next turn continues), not a refusal or a cap on payload; it costs N/64 turns, a throughput choice. Left; changing it shifts every demand quote and test fixture.
F4 detail: replacement `close` cannot reuse `close_original_effect_step` (it needs the native retirement control alongside the effect), so the typed WorkLimit refusal needs a job API that hands both; left, with evidence.

## F7 mint ruling request (for the coordinator)
`ArtifactStoreOneItemLiveAuthority::mint` is pub and unwitnessed; `mint_for_store` takes `&dyn StoreMintWitness`, but nothing implements that trait and `StoreLog::mint_witness`, named in its doc, does not exist. Every caller of `mint` in the store tree is a test or fixture (snapshot-clone, paged-one-item, prepared, durable-group helpers, canonical-edit tests); `mint_for_store` has zero callers. So no real event can mint a witness today, and a witness minted without an event would defeat the type. I did not invent one. Ruling needed: (a) gate `mint` behind `cfg(any(test,feature="testkit"))` and add `testkit` to the dev-dependency of each test caller, leaving production only `mint_for_store`; the real `StoreMintWitness` implementor then lands with the store engine's log; or (b) keep as is until the engine mints.

## Round 2 (coordinator follow-up)
- Took the 75 operation-wire in-crate test errors (nobody had claimed them; storecallers has replay). Added `operation-wire/🧪️effect/🦀️.rs` (cfg(test) `with_effect`, `fx`, `settled`, `construct_ready`, copied from clone-source-testing; no manifest change) and ported call sites with codemods `1105b-tools/reviewfix8-fx.py` and `reviewfix8-construct.py` (grant -> effect, Poll -> settled, single-turn construction -> multi-turn `OriginalConstructionAdmission` driver).
- reviewfix8-103 (--lib test build): 75 -> 15 errors, all test-call-site shapes, none in production files; those 15 were then patched by hand. reviewfix8-104 (--lib test, whole crate) queued at 21:18; verdict pending at the time of writing.
- A later step of 103 died on `Rust crate Source extent ... /Users/ueli/Documents/semio/Cargo.toml observedBytes 133179` (root manifest above the boundary): not mine, coordinator's lock/sync owner should look. I did not touch any limit.
- Fixture caveat: the per-turn heap assertion in the ported construction sites now sums physical receipts over all admission turns plus metadata close turns.

## Round 3
- reviewfix8-105 (--lib test): whole crate compiled (log 🗑️generated/1105b/reviewfix8-105.txt). Runtime: 13 tests, 4 FAILED (original_bytes x3, original_edit x1) and the process then died with SIGTRAP; no panic text was captured, so the ported construction/effect sites are unproven at runtime. My law `a_receive_that_pends_first_keeps_its_state_and_then_completes` did not get to run in that batch.
- reviewfix8-106 (filtered rerun `a_receive_that_pends original_typed_operation --test-threads=1`): refused before the compiler: `semio-framework-job-context` Source `📦️owner/🦀️.rs` is 3702 bytes against a 3478 limit. Not my file (job-context owner is another lane's, claims line 1425/1435 area); I did not raise the limit. Rerun when it is under the limit: `C/1105b-tools/runtime/tst.sh reviewfix8-107 <operation-wire Cargo.toml> native --lib -- a_receive_that_pends original_typed_operation --test-threads=1`.
- The job-level test (`job/📬️outcome/⏸️continuation/🧪️tests/🎟️source/🦀️.rs`) needs the job tests to compile, which depends on the same job-context fix; not run.

## F7 ruling (a): NOT landed, evidence needs a second ruling; correction to my earlier F7 text
Correction: my earlier claim "nothing implements StoreMintWitness / mint_for_store has no callers" was wrong (my first grep pattern missed it). The engine does implement it: `engine/store-components/.../store-component-structs` has `impl StoreMintWitness for StoreMintToken` and `StoreLog::mint_witness`, and `store-cursor-operations` calls `mint_for_store(store_log.mint_witness(), ..)` at :270 and :450. So `mint_for_store` stays (not deleted: it has callers).
Why gating `mint` behind cfg(any(test,feature="testkit")) breaks production: three non-test callers mint from decoded data, not from a live log event:
1. `engine/stage/.../durable-group/🦀️.rs:487` `verify_bound_member_edit(member, decision_sha256)` (takes no store handle),
2. same file :617 `DurableStoreBoundOutcomeV1::recover(store: &StorePartsRef, member, group_id)` (has a store ref; may be able to present the log witness),
3. `composition/durable-group/wire/.../outcome-codecs-json-budget-and-map-handles/🦀️.rs:131` `verify_inverse(&self)` (no store at all).
Gating would stop the product from compiling; the other 14 callers are tests/fixtures (durable-group helpers+fixtures, snapshot-clone x3, paged-one-item x2, prepared x2, canonical-edit x2, stage binding test) and only need `features=["testkit"]` on their dev-dependency (LOCK NEEDED for ~8 Cargo.tomls). Ruling needed for the three: (i) thread `&dyn StoreMintWitness` (from `store_log.mint_witness()`) into 1 and 3 and use `mint_for_store` (sites 2 has the store already), then gate `mint`; or (ii) accept that recovery verification legitimately re-derives an authority and give it its own explicitly named constructor (e.g. `from_recovered_outcome`) instead of the generic `mint`. I made no edit to one-item/publication (store4's file) or any Cargo.toml.

## Pending
- reviewfix8-106 rerun waits for joblaws to post job-context under budget in 1105b-claims.md (not posted at 23:05); the 4 failing original_* tests and the SIGTRAP stay uninvestigated (no panic text in log 105); investigate after the rerun with --test-threads=1 and RUST_BACKTRACE=1.
