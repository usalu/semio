STATUS: PARTIAL. Verified (logs valuetests-5/8/9/12): native-default lib 9/9 + integration 2/2; ordered lib 36/36 (1 bun-oracle set test skipped on purpose) + owned-entry integration 1/1; persistent lib 23/24 + integration 2/2; edit lib 13/15. RED: persistent observation frontier quote (128 vs fixture 4096), edit original_named_receiving_* x2 (refuse InvalidValue before output). Neural model tests UNVERIFIED: valuetests-11 build blocked by job-context compile errors from a peer edit (qualify_preissued_budget, fuel_turn_untouched, loan_admission_demand).

Logs: C/🗑️generated/1105b/valuetests-<n>.txt. Runner: C/1105b-tools/valuetests/t.sh.

## Audit of testdebt-g "stale expectations changed" (verdicts)
- bounded-ord maximumTurnWork 1 to 3: UNVERIFIED reason (two binding re-checks + byte compare) is plausible and matches runtime ruling binding_work_demand=3; kept.
- staged birth copy (constructor_copy_bytes only alias handoff; 248 placement): kept; derived from constants, consistent with testdebt-h.
- sealed receipt/birth-work rows (borrowedReturn copy, birth work 2/4, alias 3, admission_grant): kept; they derive from production quotes (SealedShared::birth_work_demand, duplicate_alias_work_demand).
- per-axis next_*_byte_demand grants -> selected_grant: kept (ruling: complete selected quote).
- native-default law 6 (interrupted receipt): removal LEGITIMATE. The leaf cursor no longer posts receipts to a construction recipient (final native-decoding shape, brief 05:13; receive_retained_work is gone); the interrupted/recover-receipt behaviour is pinned where the frame lives (decode/construction/effect producer_reply.rs lines 26-32 and the effect_bank unwind test).
- native-default law 9 (refused receipt): removal NOT legitimate as an invariant. The language-agnostic fixture outcome/failure ready=false still holds, and production violated it (a refused commit_work left the placed output ready). Fixed production (leaf cursor sets failed on a refused commit; output_ready false; demands zero) and restored the law as refused_default_work_settlement_keeps_output_unready_for_paid_close (over-reporting newtype triggers WorkLimit). UNVERIFIED until valuetests-2 reports.
- missing-recipient law now proves retry: kept. Array null-allocation law split (failed, not retryable): kept.

## Production faults found and fixed (all claimed in C/1105b-claims.md)
- persistent splice/set `demand()`: phase 2 (alias handoff) quotes SealedShared::duplicate_copy_bytes exactly when its slot duplicates (`alias_copy_bytes`); birth phases quote the full SealedShared::admission_demand (copy+capacity+depth), not just capacity (every birth was refused "original shared value birth is not fully funded"). `projection_demand` adds duplicate_copy_bytes; clone `root_projection_work_demand` = root.duplicate_work_demand (3) for a real root. try_duplicate's gate untouched. (Persistent lib 16 -> 23 of 24 passing.)
- ordered `UpdateCursor::next_work_demand`: pending Entry/Node allocation quoted 1 unit but admit_shared_allocation performs 2 (=> "exceeded its original work preflight"); now 2. `OriginalOwnedEntry::advance_work_demand` phase 3 returned constant 1 so any 2-unit turn never ran and the entry stalled all 10000 turns; now UpdateCursor::next_work_demand. Owned entry phase 3 now publishes the update's real allocation attempt into effect.actual() (32 B physical with a None marker).
- ordered `UpdateCursor::next_copy_byte_demand` (testdebt-g edit) = shared_allocation_value_demand::<Entry|Node>().copy_bytes, which is exactly what admit_shared_allocation checks: correct at source. VERIFIED by the controlled-insert test passing (valuetests-4) and 35/36 ordered lib.
- neural model `RetainedDictionaryInput::next_copy_byte_demand`: port-j's `max(capacity+2 words)` compensation REMOVED (edit made, model tests rerun pending, see below).
- ordered typed retirement `step_with`: depth gate used the per-currency `next_depth_demand` which disagreed with `next_demand().depth`; now the selected quote (valuetests-7 showed DepthLimit on a quote-funded grant).
- native-default leaf: refused commit_work now fails the cursor (output not ready); array close_step rewind step records effect.performed(1) (reported 1 / receipt 0).

## Test rows derived (not weakened)
- persistent: projection copy == duplicate_copy_bytes; builder page-birth copy == quote; update debit work <= max(page_birth_work_demand, alias work); staged admission work == items x duplicate_alias_work_demand; read step copy == step quote copy; update copy <= quote.
- ordered tests: grants for allocation phases carry the quote copy.
- edit transfer/receiving: authored body 1<<16 (cursors are KeyPreservingPersistentSet 3888 B, source 592 B; quotes 6528 and 38848 B are legitimate moves of those, not >4096 bodies); constructor_demand asserted as 2 x size_of.
- native-default tests: retire bound = stock maximumTurns.

## Open
- persistent observation frontier: turn-1 ControlledRetirement quote capacity 128 vs fixture frontierPayloads 4096 (8 vs 256 slots of Box<dyn RetirementCursor>). Not decided: fixture or the retirement lane's first-page policy.
- edit commit (`indexed_collection_commit` 2 tests): close stalls (close_turns >= limit) with quote <= 4096; message now prints quote/issued/step.

- edit commit: FIXED and verified (valuetests-12, both commit tests green). Production fault: the unstarted-field close quote came from child close_demands (default depth 0) so the funded begin_close_paid never ran; commit demand phases 5 and close_demand now quote field.begin_close_demand until the close starts.
- edit original_named_receiving_* (2): after the BODY change the loop now ends in a refusal with InvalidValue before output (output_ready false, admitted expected). Not diagnosed. Suspects: the native-default leaf `failed` path (a refused commit_work now yields InvalidValue on the next advance) or a named-root receiver quote. Next owner: run one case with the refusal error printed.
- Cross-language funding vectors keep the 4096 authored close body (LAW_BODY); real runs use 1<<16 because the nested cursors are 3.9 KB and legitimately quote 6.5-38 KB moves. The 38848 B close quote of NativeReceivingOriginal<DslValue,NamedPersistentRoot> exceeds the cross-language 4096 body: that is a design question (box the 19 KB frame) for runtime2, not settled here.
- Neural model: rerun with `C/1105b-tools/valuetests/model.sh <n>` (builds --no-run through the slot, then each test alone under a 40 s kill) once job-context compiles. RetainedDictionaryInput compensation removal is EDITED, UNVERIFIED.
