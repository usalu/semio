# Standalone Original Staged Input Independent Audit

Source-only stable checkpoint2026-10-21. Reviewed plugin reactor/turn/receiving/staged.rs, shared ReceivingSource witness, Pack take_value_into handoff and staging/request updated u64 cursor. Native uncompiled/unexecuted; actual SurfaceVisible/event/context lease integration remains absent at this checkpoint.

## Confirmed P2: expired deadline can prevent paid terminal close

staged.rs close_step uses shared metadata(cx) to remove empty receiver/retirement wrappers and mark closed. receiving root metadata checks cx.should_yield(), whereas closing ownership transitions must use retirement_should_yield(). Worker independently confirmed an expired semantic deadline blocks these paid close transitions even within with_original_retirement. Cancellation alone does not cause this particular should_yield result. High semantics is correcting only closing helpers and checking OriginalNativeDecodeSource close/control callbacks for the same problem, with expired-deadline authored regression. Fix is pending source re-review here; no runtime closure claim.

## Custody source observations

Initial admit validates finite limits/private wire length/maximum-owned policy, authentic ReceivingSource and original grant/fuel. It posts inline custody receipt before donor.take(), so refused initial receipt preserves donor. Retained Record admission stores the receiver before posting its receipt; failure latches the owner and leaves it available for controlled close. Parsed DslValue moves into an explicitly funded Box before request cursor begins, preserving root address as enclosing owner moves. Args/token transfer occurs only after census/closed admission. No whole args clone occurs.

Record take_value_into checks ready/exact one declared field/empty destination/funding, moves actual dynamic payload while retaining empty Record backing for close. Box birth stores parsed owner before posting. Request cursor stores typed output before posting; if posting also refuses, typed owner remains in Inputs, and secondary parse errors are retained in issue rather than dropped. Failed/closing input refuses resume; original operation/generation,parent,recipient,clock,epoch/deadline witness validates before advancement/close.

Close ordering drains Record receiver, ControlledRetirement<Inputs> (wire,parsed leftovers,typed/error/cursor), OriginalNativeDecodeSource and ReceivingSource loan before terminal. ManuallyDrop suppresses accidental cold owner destruction and asserts paid-close completion. This is source intent and custody trace, not evidence of actual native receipts, grant exactness or successful destruction under scheduler/deadline conditions.

Updated request generation scan now uses checked u64 multiply/add per byte; canonical spelling plus overflow refusal closes the earlier cross-contract domain gap at source level. Shared neutral MAXu64/MAX+1/21digit vectors are worker-reported. Wire strict key mode rejects duplicate/unsorted nested maps before typed movement; ordinary nonwire census defaults remain unchanged.

No blanket ingress approval: true event source slot ownership, real-window context lease, mounting and native builder remain open. Genuine native run remains required for donor denial/source substitution/all-prefix close/receipt-refusal tests.

## Source correction checkpoint
Re-reviewed corrected staged.rs close_metadata: funded closing=true uses retirement_should_yield for all three terminal metadata steps. OriginalNativeDecodeSource now selects paused(cx,closing) for close-only progress/allocation callbacks and metadata; producer close/close_step use retirement_should_yield, while normal receiving remains should_yield plus cancellation. The confirmed P2 is closed at source-review level. Authored expired-deadline test advances authentic clock beyond unchanged deadline, asserts ordinary close blocked/no debit, then original-retirement close terminal with heap/receipt equality. Test remains uncompiled/unexecuted; runtime proof is open.
