# Finite Headless Owner Review

Read-only current engine/job/🏃️headless and 🎟️caller source before native gate. No edits/queues.

Concrete compile concern sent native owner: HeadlessCaller::turn<R> forwards generic R directly through with_original_work_loan whose R:Copy constraint is explicit. Current turn has no Copy bound. Preserve generic output through an external Option<R>, with nested loan callback returning (), as owned TestJobDriver already does. A Copy restriction would unnecessarily exclude owned callback values.

Concrete recovery concern: HeadlessState.close_step returns Blocked whenever both original failure slots are occupied, before invoking close_failure_step. A drive error filling first slot plus close refusal filling second creates a permanent no-progress owner even if budget remains. Retire a queued cause through paid same-source cursor turns before soliciting another potentially failing business operation. Never discard/overwrite either cause to obtain room.

Additional cause-preservation limitation: drive maps checked_out_outcome ValueError into a generic String with map_err(|_|...), dropping exact cause; source/refusal paths that can carry owned original diagnostics should move that cause into retained failures before formatting summary. Worker outcome/continuation custody itself remains in session, so this observation is distinct from losing the checked-out producer.

Positive custody: finite stock and fuel are issued once, monotonically spent; half-reserves narrow business grants rather than issuing fresh authority. Absolute lifetime deadline uses initial now.checked_add once. Teardown borrowing uses with_original_retirement and remaining same-source stock. Missing clock/overflow before birth retains unadmitted job through failure; into_unadmitted_job only succeeds when no source/params/session/admission/failures/alias owners remain. Successful generic R is retained in HeadlessFailure.output when teardown cannot complete; into_result refuses while state remains.

Completion ordering is correct: payload copied while outcome borrowed, descriptor ACK completed, then checked_out_job_mut/finish. Canonical worker authority.job_mut236 explicitly rejects pending receipt/outcome/continuation, so ACK must precede mutable completion extraction. Failure keeps session and returned result ownership during bounded close retries; no self-funded close fallback appears.

Remaining capacity limits: arbitrary InteractiveJob can exhaust reserve; API honestly returns owned failure rather than promising universal teardown. Terminal methods require callers to retain/resolve that owner, not map_err into String and drop it. Payload_bytes allocates/copies an owned Vec outside original source accounting, so finite retained stock is not a bound on every convenience-driver allocation. No native compile/runtime pass claimed.

Resolution update: native owner added private Caller.turn<R:Copy> and State.turn<T:Copy>. Actual internal result roster is bool,(),WorkerJobPoll; generic public finish R remains outside the loan and retained in failure output. This resolves the compile concern without a public API restriction. Failure-roster retirement ordering is being fixed by that owner; not yet rereviewed.


## Frozen Neutral Terminal Materialization

Read exact current framework job/🏃️headless/🦀️.rs29–30 and drive158–195. payload_bytes flattens all retained pages into an owned Vec with collect. Complete176 and Fault178 invoke it after the self.turn worker step and before the next self.turn ACK, with no original capacity/copy/item/work admission. No staging cursor or retained copy owner exists for this terminal materialization.

The copied Vec is local Seen, not a HeadlessState field. It survives the immediate ACK loop in normal execution, then Complete189 lends its slice to finish; the Vec drops on drive return. Fault192 converts to another owned String. If an ACK turn fails or exceeds the bounded ACK loop, the local Vec also drops on early return. None of those allocations/copies/releases are reflected in original source receipts. Thus finite retained stock bounds the admitted job/session actions but not all generic convenience-driver terminal copying. The semantic finish(&mut J,&[u8]) API currently motivates copying before ACK because mutable job access requires semantic ACK; that does not provide budget authority.

Actual required scope: staged terminal copy/diagnostic conversion with original admission and retained owner across possible failures, or a deliberately different supervised API that consumes semantic terminal kind without copying bytes. Do not classify this as a solver progress/cancellation test failure or claim a particular model exceeds limits; the concrete issue is unmetered terminal materialization and release. Parent/native owner notified. Frozen source was not edited or restarted.

## Retained Terminal Capture Repair

The canceled r2 check never compiled or executed tests and is not acceptance evidence. Root strengthened the existing PayloadCapture module with immutable OriginalSourceBinding plus exact payload header/extent/page identity. Capacity is quoted before binding/allocation; reservation is fallible; page copies cannot reallocate; cancellation and changed-source probes preserve the current frontier; completion stays source-checked. Same-source teardown pays buffer release and binding removal on distinct turns. Eight portable native laws include real canonical allocator observations, allocation refusal, copy quantum, denied capacity/release, cached completion and foreign caller/payload. These are authored and syntax-checked, not yet native-executed. The neutral package has one new DEV-only allocator dependency; external headless consumers omit it.


## Private Capture and Eight Frontier Laws Before R3

Reviewed current framework headless/📥️capture source and native test module. No concrete source-level blocker found; no edits or queues. This supersedes the earlier local Seen Vec gap: capture is now retained in HeadlessState, filled inside original caller turns before semantic ACK, and physically closed before other business custody. ACK failure leaves capture in retained state. Finish borrows its bytes after ACK; subsequent paid close releases backing. Supervised callbacks receive semantic terminal kind without converting fault payload to an uncharged String in the driver.

Capture identity validates genuine OriginalSourceBinding plus original wire(header identity,length,operation) and page count before completed no-op. Denied work/items/depth/capacity paths precede state mutation. Source-binding metadata, fallible reserve, bounded page copy, buffer release and source-binding removal are separate1work events. Copy count is bounded by copy grant, exact extent and existing capacity; no extend reallocation can occur. Actual reserve capacity is reported, and any posting refusal leaves ownership retained. Empty absent payload completes after binding+emptybuffer turns. Foreign caller/payload rejection precedes effects. Close checks stable original binding and releases buffer separately from binding metadata.

Eight frontier cases cover capacity denial, real allocation refusal, absent payload, cancelled partialcopy, exact multipage copy/completed noop, foreign payload, foreign caller and release denial. measured helper compares actual allocator requested/released bytes with same caller cumulative receipt delta, uses nested1work loan and asserts actualwork<=1; denial expects0work/0effects. Cleanup checks total allocated capture bytes equal paid release. Tests borrow builder payloads only while immutable, then end borrow before closing builders; no escaping borrow found.

The new DEV semio-framework-value-testing-allocator dependency is sufficient linkage: its library declares #[global_allocator] unconditionally and forwards allocations to canonical RequestedAllocator, while supporting actual null refusal. This is different from relying on dependency cfg(test). Runtime dependency list remains six internal framework packages. These are source assertions, not a claim the native frontier suite has executed or passed.


## External Testkit Terminal Diagnostic Integration

Concrete mismatch: bitmap host inference unit133–135 requires a closed ordinary semantic failure whose message is bitmap-inference-unreachable-pin. Frozen neutral default run_headless352 intentionally maps all semantic Fault to label-fault, yielding bitmap-inference-headless-fault instead. Test expectation and current convenience wrapper behavior disagree.

Recommended local integration: five cfg(test/testkit) solve_with_clock adapters use run_headless_supervised with no-op supervisor |_|None. Match caller-owned terminal decoder: Complete(bytes) uses existing artifact commit decode; Fault(bytes) returns artifact diagnostic string; Cancelled returns explicit artifact cancellation label. This preserves typed HeadlessFailure and terminal kind, leaves generic runtime unchanged, and makes diagnostic conversion allocation part of the dev-only caller adapter rather than pretending framework admitted it. Do not recover error String by dropping a live failure owner.

Grid3d authored-only viewer review: render63–73 projects only authored_assignments; bounds,known tile IDs and masks are filtered57–60. It renders a generic localized Preview title and a scene, with no solved/satisfiable/contradiction claim or status text. Therefore no concrete false solved-status claim exists in current source. An explicit authored-only/no-current-result caption would improve clarity but is a separate UX completeness point, not evidence the viewer reports a solve result. No queues, edits or frozen-source changes.


## Five Supervised Wrappers Revalidated

Read all five current solve_with_clock function bodies directly, including graph3d factory-and-solve. Each now calls run_headless_supervised with six current arguments: job,operation,clock,finite site,no-op supervisor,terminal decoder. Complete borrows captured bytes and decodes the matching artifact commit; Fault returns String::from_utf8_lossy(bytes).into_owned preserving existing UTF8 diagnostic bytes such as bitmap-inference-unreachable-pin; Cancelled supplies explicit artifact-specific cancellation. Typed HeadlessFailure<ActualParentJob,ActualCommit> remains the wrapper return. Job constructor refusal is lifted through rejected; no live owner is mapped/dropped into a String.

Current WFC engine job reexports run_headless_supervised and HeadlessOutcome from the neutral component; root engine reexports that job module. No stale signature/type name was found. This is source-level coherence, not a native test result or exact diagnostic assertion pass. Global WFC scan also found no remaining RetainedJobPayloadWriter,admit_payload_page,test_step_context or close_with_self_funded_context references.

Independent useful review while neutral r3 runs: audit retained publication frontier state plus capture/ACK ownership across all ten WFC parents and their native test fixtures, without touching the neutral source. In particular verify needs_retirement persists through the delivered-flag clear until separate Box release; ensure parent lifecycle regression traverses a genuinely nonempty payload and final2stage cleanup. This reviews external integration independently of neutral compile outcome and avoids starting duplicate queues.

## Root R3 Capture Law Re-Read

At 2026-11-05T02:14:02.869Z, re-read the frozen actual allocator frontier law. Cases build real two-page source payloads under original callers, narrow each measured capture/close to one work unit, reconcile capacity/release with actual allocator requests, and return both source builders before caller retirement. Allocation-refusal and cancellation retain state; foreign payload/caller probes assert zero receipt/work; cached and post-close probes assert no effects. Found no additional concrete source blocker. This read is not native execution; r3 remains pending.
