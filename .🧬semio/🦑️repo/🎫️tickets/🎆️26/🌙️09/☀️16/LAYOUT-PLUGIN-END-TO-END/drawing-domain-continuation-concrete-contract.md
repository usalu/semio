# Concrete Original Continuation Return Boundary

Proposal for defining review before production changes. No test execution or SourceReady claim. This supersedes the earlier generic return-capsule sketch and rejects an enclosing-grant-three workaround.

## Actual Current Configuration

BatchDriveConfig in Job/worker/authority has caller-authored retained, fuel_per_step and step_budget_us. NativeWorkerAuthorityState::issued_grant intersects caller requested grant with config.retained; there is no declared minimum of three. drive_worker_job_authority derives StepBudget directly from that unchanged policy. The genuine fixture admits a fixed total source then with_retained_loan narrows each requested callback to one. Therefore subtracting return credits from every current one-item view, or requiring a current three-item view, can block forever. Those are not proposed.

The current worker payload context supplies a genuine payload operation ledger but no OriginalCallerSourceParent; the actor bridge also uses payload-only context. Both routes must receive their real source loan from their actual admitting caller. They cannot fabricate a parent from BatchDriveConfig, operation numbers or an arbitrary budget.

## Minimum Resource Rule

A claimed receiving place must hold real previously issued original lifetime currencies. This requires small source-owned hold accounting; physical slot capacity alone is insufficient.

A return claim holds exactly two item/CPU units: one actual typed capture/erasure and one actual persistent receiver write. Each is a separate actual transition, work one; ordinary child work can be zero or one independently. Copy/capacity/release hold is zero; depth is the original admitted depth, not additive authority. The quoted static frame and receiver header already own their actual storage.

Before body, a separate current quantum-one claim-control turn (one actual metadata item/CPU) checks original lifetime remaining >= claim-control plus the two holds, binds the real source and marks the real receiving place Claimed. The holds are NOT physical receipts or spent CPU. They reduce all subsequent spendable source views. They are created once per return claim, not subtracted from every body quantum. A separately paid existing OriginalCallerSourceLoan retains the live roots/clock through eventual return and close.

The core invariant is spent + held <= the original source total on items and fuel. No reservation increases total, deadline, current body quantum, byte capacity or policy. Source hold accounting is private and shared by all contexts of the same live source, including aliases, so another view cannot spend a held credit. It belongs to the Job source owner, not a producer-local integer bank. Actual receiving consumes one held credit into its own explicit reserved receipt and actual CPU, even after the semantic current view is exhausted. Unused held credit is released unspent, never added to total. Claim cancellation/disarm itself is an actual funded control turn; a held credit can fund that real transition, with the remainder merely unheld.

Current and reserved-return receipts must remain separate in the caller report. The existing assertion that the whole callback delta fits its one-item semantic loan must not be relabelled: body delta fits that current view; reserved return delta fits the exact one-shot prior claim; their sum is checked against the original cumulative stock. This is the required receiver accounting change, not a quota increase.

## Proposed Canonical APIs

- OriginalJobReturnSlot: caller-owned inline Empty/Claimed/Occupied state, actual source loan, claim identity/serial, two held credits and the original owned capsule. Its physical extent is included in the actual native authority/bridge owner quote.
- claim_step(cx) -> RetainedWorkOutcome<ClaimStatus>: one real claim-control turn; denied/cached no effects. It cannot enter body before the genuine source loan and held claim exist.
- OriginalJobReturnClaim: private noncloneable borrow of that real Claimed slot and its real source claim. A producer cannot manufacture it. It authorizes exactly one typed capture and one actual receipt into that slot.
- OriginalJobReturn<'job>: Semantic(Option<JobOutcomeBorrow<'job>>) or Continuation(OriginalJobContinuation). Ordinary payload success is unchanged.
- InteractiveJob::step gains a mutable OriginalJobReturnClaim argument and returns that enum under its existing native Result. There is no old Result alias. Native errors before body remain ordinary native errors; pending continuation is not a terminal error or serialized Fault.
- drive_step carries the owned enum unchanged through its watchdog/qualification handling. It never applies result? in a way that discards an owned capsule. Post-body qualification failure is appended as a separate complete owner in the returned capsule.
- OriginalJobContinuation: owns the exact prepaid typed Box through object-safe continuation custody plus complete callback native and final qualification causes inline. No new Box is born after body. Typed Copy progress can be moved into a prequoted typed slot during the actual claimed capture; actual owned success remains in producer.
- claim.capture: one actual move into the prepaid typed return storage, charged from the first held original credit. This is where the current unpaid callback assignment disappears. The complete native return and final qualification are separate, not flattened into available Native fields.
- claim.receive: moves the owned capsule into the already Claimed real caller place using the second held credit. It records its explicit reserved receipt/work. It does not infer authority from descriptor capacity, child receipt, callback entry or remaining semantic fuel.
- insert_step(cx): under a later genuine current view, move exactly one native cause from occupied return custody into the prepaid domain payload; one actual item/CPU, zero allocation/copy. Denial retains exact owner/pointers. Source mismatch returns the same owned capsule plus refusal; it cannot persist an unpaid foreign record.
- canonical paid RetireOwned/close_step for slot and capsule; close exact child causes/frame before returning held credit/source loan and finally releasing receiving owner storage.

No production API has been changed to this proposal yet. Exact type spelling can be shortened during implementation, but the ownership and currencies above must remain visible.

## Real Receiver Ownership Map

1. Worker authority: NativeWorkerAuthorityState gets the inline return slot at the same actual quoted native birth. Source loan/return claim are separately admitted through the actual WorkerJobAdmissionControl issuing caller. drive_worker_job_authority first advances claim, then invokes drive_step with its claim; its Continuation arm calls the actual claim.receive. This does NOT populate step_fault, set fault_pending or publish a terminal Fault. Later callbacks advance insertion/close before reentering the producer. Watchdog quarantine retains the same occupied owner and never drops it.

2. Context-owned authority: step_on_original_caller uses the same slot and actual initiating cx. It must return separate current/reserved receipts, rather than treating a previously claimed return as part of the fresh one-item view. It likewise retains Continuation through claim.receive, not native-error propagation or a free field assignment. Closing drains the slot before job/source removal.

3. Actor caller bridge: its actual caller must supply the real source loan/claim; replacing payload-only context is required. The bridge gets an inline slot in its quoted owner. Native Continuation bypasses JobOutcomeProjection/pending semantic descriptors and goes through claim.receive; ordinary semantic projection remains unchanged. The source/claim closes after occupied return custody and pending descriptor, before payload authority/source close.

Action-bus erasure forwards the new owned arm/claim without changing ownership. The UI direct caller is testkit only and must migrate, but is not evidence of a production receiver.

This source-required loan/admission input is a concrete current API change. WorkerJobAdmissionControl currently does not expose a lasting genuine Original source; it must require a real source return-authority operation, not default to a manufactured source. Cursor owns the existing SourceParent/lifecycle slice; the new hold and receiver code should live in a separate Job continuation module, with only agreed source-owner hooks/fields edited by Cursor.

## Test-First Laws

The closed language-neutral contract is drawing-domain-continuation-contract.json and its schema, currently ticket inputs for review. It fixes original total, body quantum one and all actual costs; no production cap/timeout changes.

Actual Rust laws must use OriginalJobFixtureCaller, real declared receiver source loans and real preborn typed domain frames. Both actual authority routes and the actor receiver need composition laws, not only a flat receiver model. Cases cover repeated one-item claim phases followed by one-item body progress; the body spending every current item/fuel; readonly native failure with child work zero but positive actual capture/receive; complete native plus independent final qualification; genuine denied claim (no body); denied receive/insertion preserving exact owner; foreign/invalid contexts; cancellation; and paid same-source close. Validate body/return receipts separately and original spent+held conservation after every transition. Measure actual heap/refusal, exact original pointers and unchanged Box allocation; independent Serde projection compares every complete cause tree. No owning gate is claimed until genuine normal compilation/tests run.
