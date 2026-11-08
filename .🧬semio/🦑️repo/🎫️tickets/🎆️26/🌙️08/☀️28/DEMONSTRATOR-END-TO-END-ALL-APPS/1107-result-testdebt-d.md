STATUS: PARTIAL. GREEN (testdebt-d-38): native-shared-slots 1/1, native-shared-field (0 tests). value-ordered 36/37 lib, owned-entry-effects integration RED. Not green/not started: native-default lib, persistent, retained-map, original-slot, value-edit, snapshot-read, retirement-owner, presence-retirement. Logs: C/🗑️generated/1105b/testdebt-d-<n>.txt; error splitter C/1105b-tools/testdebt-d/errs.py.

Note: in zsh pass cargo package args as an array ("${P[@]}"); a single string made the repo admit the whole workspace (false unicode fixture refusal, testdebt-d-1/2).

## value-ordered (testdebt-d-19)
- Ported to the effect API: owned-entry effect.rs, owned-entry 🧪️.rs + ➖remove, update/lookup/immutable-alias/cursor-work (u128 work, scratch bank), big tests/🗂️ordered (RetirementCursor seed, ControlledRetirement drive). Helper `bank` mod inlined in 🦀️.rs (a new file broke maxSourceFiles 32).
- Production fixes: ordered/♻️retirement TypedOwner::step_with and owned-entry advance now record reported work into the effect (was "reported work differs from original performed receipt"); controlled next_depth_demand now equals the step's next_demand().depth before the cursor frontier exists (DepthLimit for any caller using per-axis quotes).
- Still RED (7): (1) immutable-alias admitWork law is 2 but SharedOwner::admit/admit_shared_allocation reports 1 (law changed in commit 729, production did not; owner decision: runtime). (2) 4 tests in tests/🗂️ordered stall (`turns < N`): controlled step makes no progress with per-axis grants; (3) update-work controlled insert needs fixture::receive with a work loan (dev-dep testing-admission); (4) owned-entry/effect.rs:26 marker 32 vs heap 0 at a step with quote-derived work.

## native-shared-slots (testdebt-d-8..11)
Inline test ported to EffectBank + fixture::receive + close_owned; dev-deps added (no lock run). Compiles. RED: first receive_shared/arc turn returns InvariantViolated "native receiving frame requires its original recipient and callback phase" and leaves work pending. Possibly fixture::receive (testing-admission) vs decoder frame; unresolved, report to runtime.

## native-default
tests/original-default-placement.rs ported (advance Poll, take through a resident OriginalEffectAuthority reply, close via EffectBank); compiled, NOT run. lib test 🎟️work/🧪️tests (9 laws) still on removed NativeDecode*Recipient API: not started. advance_work_demand quote no longer exists, so "turn.work <= quote" became "<= maximum".

## Not started (compile errors from testdebt-d-5)
persistent (144), retained-map (101), original-slot (71), value-edit (20, valuefix is editing; needs test-only root re-exports, added), native-shared-field (no errors seen), snapshot-read / retirement-owner / presence-retirement test support (not touched; port-a claimed presence-retirement 08:49).
Cross-cutting for runtime: the two items above (admitWork law, fixture::receive vs decoder frame).

## Update 10:3x (testdebt-d-20..23)
- Stalls fixed: the four "turns < N" tests stalled because grants carried body-only copy bytes (1/3/64) while the step now also needs the header copy of next_demand; tests now grant max(body, next_demand().copy_bytes). Production: controlled next_capacity/next_release quote 0 in the pending-cursor push turn (was a cursor-birth number the step never needs: denied grants still progressed).
- admitWork: allocation-return/🔗️shared admit_shared_allocation success now reports 2 (runtime2 ruling); callers (job session-return, builder, shared-utf8, arc-custody original) not re-verified.
- Test cursor MixedReceiptPayload now records its work via effect.performed.
- Remaining ordered RED: (a) update-work actual_controlled_insert...: with fixture::receive + WorkLoan the first advance_insert_controlled turn is quote 1 / work 1 / Ok, but receive returns WorkLimit "original effect exceeded its original work preflight" (settlement after apply). (b) tests/effect.rs:8 `take_admitted()==Some(false)` in a bank run (maybe stock exhausted at fuel 32768, unverified).
- native-shared-slots: runtime2 says keep ONE ConstructionLoan/WorkLoan across turns of a frame; fixture::receive builds fresh loans per call, so multi-turn frames cannot work through it. Not yet changed.

## Update 10:5x (testdebt-d-24..28)
- ReceiveFrame (testing-admission) landed: loans persist across turns.
- ROOT CAUSE found for the "native receiving frame requires its original recipient and callback phase" invariant: decode/🏭️construction/🦀️.rs receive_effect_typed called the control-level prepare_work, which never binds frame.work_action, so effect_admitted (📦️effect:105) always failed. Fixed to frame.prepare_work (claimed; None -> retry later, Some(false) -> cancel). The temporary backtrace print is removed.
- native-shared-slots now gets past that and fails next: turn 1 returns Err with only `native_allocation: Some(NativeDecodeAllocationOutcome{admitted:true, actual:Some(48), no refusal})`, progress copied 40 / work 0. An allocation outcome with no refusal is treated as failure: likely the allocation settlement/ledger (settle_allocation_progress) is not completed for a Local binding. Open with runtime2.
- receive_prepared_construction (same file, ~line 110) still uses control-level prepare_work; not changed, not run.
- admit_shared_allocation work 2: job callers pass work through, SealedShared::birth_work_demand already 2 (runtime). shared-utf8 and arc-custody/📦️original:31 not re-verified.
- Still not started: native-default lib test, persistent, retained-map, original-slot, value-edit, snapshot-read/retirement-owner/presence-retirement.

## Update 11:0x (testdebt-d-29..31)
- On top of runtime2's effect-bridge work recording, native-shared-slots turn 1 now fails with work_overrun "original receiving work exceeded its original quote": actual 4 vs quote 2 (SealedShared::birth_work_demand = issuance + allocation). Cause 1 (mine, fixed): prepare_birth (arc-custody/📦️original:36) built on admit_shared_allocation, which now charges 2 incl. a placement write the empty storage never does; success work reduced by 1 there. Still 4 after that: suspect double counting between the effect bridge's owed-work record (runtime2, 📦️effect) and the child's own recorded work; needs child.work vs recorded_work printed. Log testdebt-d-31 (debug print removed).
- receive_prepared_construction still not ported (control-level prepare_work).

## Update 11:2x (testdebt-d-32..38) native-shared-slots GREEN
Root causes fixed (all claimed in 1105b-claims.md): (1) decode/🏭️construction receive_effect_typed used control-level prepare_work (frame.work_action never bound); (2) allocation-return/🔗️shared/📦️original prepare_birth reported turn.work+2 / const 3 while the laws (original-slot actual.json, backing 📐️.json birthWork) and SealedShared::birth_work_demand=2 say 1 (now turn.work, const 1; retained-clone source admission adds the issuance +1 itself); (3) receive_shared quoted 1 for the publication turn while publication.commit reports SharedOriginalPublication::work_demand=4 (now SealedSharedBirth::publication_work_demand()); plus runtime2's effect-bridge owed-work record and my ReceiveFrame. My arc-custody/📦️original edit was reverted (not the live storage).
Ordered still RED: update-work controlled insert (ReceiveFrame + WorkLoan, "exceeded its original work preflight") and effect.rs:8 take_admitted==Some(false). Not rerun: other consumers of birth work 3->1 (backing, original-slot, source-admission laws, job-context).

## Update 11:5x (testdebt-d-39..44) consumers of the birth-work change
- DECISION CHANGE: storage prepare_birth work/const back to 3 (law value-allocation-return 📦️original/📐️.json work.prepare.total=3, passes). SealedShared::birth_work_demand()=2 is the refused-attempt cost, not the success quote; receive_shared and NativeSharedAdmission::shared_work_demand now quote 1 + SealedShared::prepare_birth_work_demand() (=4) for birth and SealedSharedBirth::publication_work_demand() (=4) for publication. native-shared-slots GREEN with this.
- allocation-return: lib tests ported (u128 work) and GREEN 13/13 (testdebt-d-44) incl. admit work 2 (successWork) and storage laws; real fix: admit_shared_allocation now also refuses on the copy axis (was unchecked).
- Not runnable (tests still on the old API, other lanes): shared-issuance lib test (80 errors), original-slot lib test (71, mine, factory/FactoryPayloadRetirement based, not ported), retained-clone not reached; native-decoding tests exceed the source budget (known). job-context/shared-utf8/arc-custody consumers not re-run (job-context unreachable; shared-utf8 lives inside semio-framework-value).
- receive_prepared_construction prepare_work and the remaining crates (native-default lib, persistent, retained-map, original-slot, value-edit, snapshot-read, retirement-owner, presence-retirement) NOT done.

## Update 12:1x original-slot GREEN (testdebt-d-57: 11/11)
Tests ported (run helper over EffectBank, delegating test factories, u128 work). Production: advance_read/retain_read/return_read now record owed work in the effect (record_owed_work). Law change (documented): a source-admission read step reports up to SharedOriginalPublication::work_demand()+1 (5), not 1: the old "turn.work <= 1" bound predates the storage laws (prepare 3, publication 4); the quote the source admission itself uses is publication+1. receive_prepared_construction ported (frame.prepare_work + owed record), compiles, not run. Next: retained-map, then shared-issuance, native-default lib, persistent, value-edit, test support.

## Update 12:4x retained-map (testdebt-d-58/59) PARTIAL, stopped
- Test fixture mount fixed (cfg(test) retained_clone module exposes the shared effect fixture; paged-list dev-dep; claim-free, test-only). clone cursor advance/close_step/step calls converted to effect::advance/effect::settle (26 sites) in 🧪️tests/🔬️unit, ➖️remove, 🔢️bounded-ord.
- Still RED to compile (about 45 errors, testdebt-d-59): the insert/remove/lookup cursors take (grant, effect) / (source,target,grant,phys) shapes of their own; u64/u128 work compares in bounded-ord; `.kind` on OriginalRetirementFailure; OriginalEffectReply based `take`. Needs per-cursor porting like original-slot did.
- NOT STARTED: shared-issuance lib test (80 errors), native-default lib test (9 laws, old recipient API), persistent (144 errors), value-edit (50 KB tests, valuefix editing), snapshot-read / retirement-owner / presence-retirement test support.
- GREEN overall in this lane: native-shared-slots, original-slot 11/11, allocation-return 13/13; value-ordered 36/37 + effect test RED.
