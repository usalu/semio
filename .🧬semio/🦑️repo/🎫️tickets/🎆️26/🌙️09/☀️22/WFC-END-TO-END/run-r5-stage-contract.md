# Snapshot Stage Custody Decision

The current nested Result contract can destroy a produced owned value or authored IoError when a later allocation settlement refuses. An already occupied NativeSnapshotFailure settlement slot also cannot absorb another owned refusal without losing one. Retaining both is required; stringification, first-error-only replacement, uncharged boxing and compatibility wrappers are rejected.

## Selected Contract

Use a stack-owned generic stage failure family with explicit NotStarted(ValueError), Operation(E), and Settlement { operation: Result<T,E>, refusal: ValueError } states, paired with the actual work receipt. The settlement state keeps even a successful T and can nest an earlier typed failure as E. Pre-operation refusal contains no fabricated operation result. No wrapper allocation is needed to construct any branch. Public retirement support must preserve every owned field and original control receipt.

Apply one coherent contract to both known lossy allocation_stage and allocation_stage_native paths and handcraft every actual caller. Preserve independent observer refusal where the operation owns one; do not replace an occupied refusal. The relevant SQLite/space-history traits and validation/port callers must expose and propagate typed custody rather than forcing a lossy conversion back to NativeSnapshotFailure. Generic ownership prevents a base-to-IO dependency cycle. At a final native boundary a value may leave the typed owner only by actual recipient handoff or paid original retirement; otherwise custody remains explicit in the returned or pending owner. No hidden dropping branch or fake work quote is allowed.

Native verifier owns the shared stage contract, native snapshot/receiving/trait implementation and schema coordination. VCS boundary owner assists validation/lifted-IO/shared workspace receiving callers after that contract is settled. Required cases include authored IoError plus later settlement refusal, successful owned output plus refusal, an earlier NativeSnapshotFailure with occupied settlement plus another refusal, and before-body denial. Assert original ownership/diagnostic identity, actual work and all refusal fields with portable fixtures and independent serializer/oracle checks. Then run focused owning suites before retrying full core.

The shared retirement contract is independently changing to OriginalRetirementOutcome. The IO owner must implement current real original-control/quote forwarding and preserve both computation and settlement failures, adapting required ticket helpers if still incoherent. No waiting for an unidentified agent or stale contract assumption should stop prerequisite work; re-read and coordinate current source rather than overwrite others.

## Explicit Output Slot

Approved SnapshotReceivingOutput<T> = Result<T,SqliteStageFailure<T,NativeSnapshotFailure>>. Receiving methods accept a caller-owned mutable optional output slot and return RetainedWorkOutcome<(),SqliteStageFailure<(),NativeSnapshotFailure>>. The paid published result remains in that slot even when a later native/observer receipt fails; callers must not create a disposable local slot and propagate an error while dropping a published owner. Slots thread from actual original owning frames.

Native encoding uses a paid parent frame containing projection, JSON cursor and published output, with an inner projected frame that owns every partial key/value/vector before the next fallible action. The JSON cursor must report committed work, so pre-effect capacity/checkpoint refusal yields zero effect work. Text copying needs a caller-owned partial destination under the actual constructor quote and receipt. Existing return-a-fresh-string behavior cannot drop partial data on refusal. Native helper migration is still incomplete and unverified.

Zero remaining SQL body allocation is not a blanket reason to reject a valid zero-allocation validation. Framework frame custody is funded by the original native/physical grant; the SQL body is separately bounded by its own actual data allowance. The bare unmetered fallback is not restored. Actual zero-allocation and denied-publication cases are being added.

## Original SQL Work And Octet Receivers

Root authored portable model work tests before adding a fourth original work-recipient stage callback. VCS now owns that model integration, including caller-retained work failure on unwind and raw declared/before/after/delta custody. Model red attempt80270 terminated before compiler on an intermediate IO delimiter; no model behavioral red or green is claimed.

Root authored portable octet laws for completion, first-copy receipt refusal and work denial, then implemented exact-layout wallet copying into a caller-owned Vec through original effect quotes. Each allocation/copy effect adopts output before posting work and physical receipts. The source now includes encoder/decoder original-owner wrappers whose wallet scope preserves accepted progress on unwind, plus an artifact SnapshotOwner adapter borrowing the actual direction. Focused octets test77590 is pending. A brief mid-edit missing module was captured by67145-era prerequisite attempts; all such attempts are precompiler failures and require recapture.

These lower helpers do not establish parent safety by themselves. Cell/Projection must insert the empty field into their caller-retained row before receiving, Reconstruction must target caller-retained fields, and monolithic authored SQLite fallback methods must be migrated or excluded from original receiving execution by a real staged owner. The old fresh-copy blob family has not yet been removed or replaced at every actual caller. Paged-text aligns to SnapshotOwner and the same caller output/effect/unwind contract.

## IO Test Ownership Extraction

Octets77590 ended before compiler: the IO-control Source capture had only96 bytes remaining when reading a746-byte cursor fixture. Root did not raise limits or trim assertions. The81KB public root law suite now mounts from a new semio-framework-io-control-tests companion depending on the actual production owner; its receipt/drive child files and fixture paths remain. Private snapshot/failure/octet laws remain with their implementation owner, and the single public-suite test of private effect_outcome moved into a small internal effects file. New companion has test-only dependencies and an observed allocator; existing source files were not copied. The current owner octet gate62785 and companion encoded law gate are pending actual admission/compiler/test results.
