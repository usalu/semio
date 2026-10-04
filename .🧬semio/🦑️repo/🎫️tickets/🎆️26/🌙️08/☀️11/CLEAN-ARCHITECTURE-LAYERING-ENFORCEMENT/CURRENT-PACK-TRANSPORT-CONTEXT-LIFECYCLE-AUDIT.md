# Pack Transport Context Lifecycle Audit

Independent read-only audit of the unmounted capture/context draft on 2026-10-03. Provenance: 📓️2026-10-03-pack-transport-capture-admission.md. Exact observed inputs are bound in CURRENT-PACK-TRANSPORT-CONTEXT-LIFECYCLE-HASH-RECEIPT.md. No native compilation or runtime safety verdict is supplied.

## Portable Lifecycle Defect Found And Repaired

The initial observed TypeScript counterpart leaked modeled storage on three throwing callback paths, independently reproduced with Bun before the concurrent owner repair:

| Throwing phase | Context reserves | Context frees | Error reserves | Error frees |
| --- | ---: | ---: | ---: | ---: |
| contextAfter checkpoint | 1 | 0 | 0 | 0 |
| initial reserved progress | 1 | 0 | 0 | 0 |
| published progress, followed by caller context disposal | 1 | 0 | 1 | 0 |

The initial SharedReservation.publish set its active flag false and invoked progress before constructing the CapturedTransport owner. A throw leaves the error cell and cloned context unowned; disposal of that reservation is then inert. The initial createSharedContext likewise lacked cleanup around its post-allocation ContextAfter callback and initial progress callback. The existing three claim/panic corpus cases exercise pending claim exclusion and post-source-reserve callbacks, so their passing results do not cover these lifecycle holes.

The owner repaired the portable model during this audit with an owner installed before each callback and cleanup in finally/catch that preserves the original panic. For published failure, free physical error storage and release the context clone while keeping already published cumulative ledger charge; Rust establishes that owner and marks its claim committed before reporting SourcePublished. Add exact throw/free-count laws for these three phases, plus final progress callback failure. This is a demonstrated portable defect and a mismatch with the Rust cleanup inference, not a demonstrated Rust leak.

An independent rerun against the repaired module confirmed all three paths free each reserved context once, published failure additionally frees its error cell once, and each original thrown message survives. Observed repaired results were contextAfter 1/1 context reserve/free, initial reserved 1/1, published 1/1 context and 1/1 error. The exact-source receipt captures the successor inputs observed at report creation; it does not retrospectively hash the initial leaking portable revision. The owner independently reports strict whole-suite RED for these three cases before repair; this audit executed its own direct reproductions and rerun, not that whole suite.

## Rust Ownership And Unsafe Lifetime Inference

Capture storage is a typed Vec<TransportCell<E>>. Its nonzero cell layout includes AtomicUsize and Option<PackTransportContext>, so even zero-sized E does not produce a zero-sized allocation. try_reserve_exact reserves at least one typed slot; actual capacity times typed size is checked, then capacity and original typed pointer are retained. Publication initializes exactly one element and forgets the Vec. The matching type-specialized release vtable reconstructs Vec with length one and the original capacity exactly once at final release. Pointer casts preserve the address/provenance of that original allocation; source references borrow from a live handle and cannot safely outlive it. Capacity greater than one remains supported.

All public constructors constrain E to Error + Send + Sync + static. Context policy has Send + Sync and static construction. The unsafe wrapper Send/Sync implementations therefore erase only values meeting those bounds; Mutex protects the mutable admission ledger and atomics protect cancellation/refcounts. Context immutable vtable/pointer/capacity fields do not mutate after publication. Private vtable construction ties each pointer to the matching concrete E/P destructor.

Both refcounts use Relaxed retain, Release decrement and an Acquire fence before destruction. A live borrowed handle guards retain/source access. Concurrent final decrements serialize through the atomic count, and only the decrement seeing one reconstructs storage. Retain aborts at the conventional isize threshold before wrap can continue. These are source-level ownership arguments; native pointer/layout/drop behavior and concurrent memory ordering remain unexecuted in this audit.

Vec ownership remains live on allocation failure, actual-capacity refusal, callback panic and before publication. The shared path creates AdmissionClaim before the allocator, adds requested bytes under the mutex, and reconciles excess capacity under the mutex. Storage is declared after the claim, so ordinary unwind drops transient storage before refunding its requested claim. After reconciliation, SharedTransportReservation owns both storage and full refund claim before AfterReserve and ReservationCommitted callbacks. Its Drop frees unused storage, refunds only its own charge, and suppresses progress during unwinding. The claim fallback refunds any remaining charge. No whole-ledger rollback can erase unrelated reservations.

Shared publication creates the OwnedTransportError local, then clears the claim before SourcePublished reporting. A reporting panic drops that owning local and its embedded context; the cumulative published charge stays consumed intentionally. The original source is destroyed before its retained context field. Context final release reconstructs its owning Vec before ContextClosed reporting, so a reporting panic still owns storage for unwinding. ContextReady likewise has an owning context local before callback. A policy/source destructor that panics during an existing unwind may abort according to ordinary Rust double-panic behavior; arbitrary user destructors are not made panic-proof by this design.

## Concurrent, Reentrant And Cancellation Limits

No ledger mutex is held over allocator hooks, IO, checkpoint or progress policy calls. The second requested-credit check after BeforeReserve covers a policy that admits another reservation reentrantly. AdmissionClaim prevents a concurrent allocator from spending the same requested credit while the first allocator is still pending. Actual excess is reconciled against the live ledger, rather than an obsolete snapshot. A reentrant AfterReserve callback can observe and reserve against the complete existing charge; rollback remains subtraction of the failing reservation only.

Cancellation is checked before and after policy checkpoints. A cancellation racing after a completed checkpoint is cooperative and can be observed by the next operation checkpoint; publication intentionally contains no fallible cancellation step. progress callbacks can reenter public methods, but arbitrary callback recursion or user-held lock cycles are the policy implementation's responsibility. Policies that retain a context clone may create a reference cycle; the existing reentrancy test explicitly clears its alias. This design does not detect such user-created cycles.

The physical allocator can temporarily grant excess capacity before the excess admission check; refusing that grant releases it, and its physical witness is distinct from retained charge. The ledger ceiling proves admitted retention, not an absolute cap on allocator transient overgrant or preexisting source-owned external storage.

## Executed Independent Portable Evidence

Strict installed Ajv2020 validation accepted all five closed schema/corpus pairs: capture 9, provider 3, context 3, policy 32, claim 3. Schema closure validates shape and enum values; it does not itself prove each enum case occurs uniquely or that native execution matches fixtures.

A separate Bun integer-state model enumerated 3,467 states and 1,750 terminal interleavings for two actors, requested charge 2, actual allocation 2 or 3, baseline 1 and maxima 4/5/6. Transitions independently model pending claims, allocation failure, actual-excess reconciliation, unused refund and cumulative publication. Every visited state preserved ledger = baseline + pending claims + cumulative published charge and ledger <= maximum. This is a bounded logical proof of those arithmetic transitions, not Rust execution, a mutex scheduler proof, or an allocator ownership proof.

The callback-failure reproduction above exercised the actual TypeScript counterpart with custom counters and throwing policy hooks. It confirmed modeled leaked owners without creating dependencies or changing source. No generated file or runtime log was retained outside this Markdown evidence.

## Native Test Quality And Remaining Evidence Gaps

Authored native tests inspect original source identity, one final sentinel drop, no-allocation clone/projection, actual capacity overgrant/undergrant, pre-operation refusal, original policy/error/message identity, scoped concurrent reservation accounting, pending allocator exclusion, two callback panic/refund phases and reentrant ledger observation. The claim exclusion and callback panic tests materially distinguish the previously broken implementations.

Native execution remains unverified here. Add or execute exact laws for concurrent clone/drop of a published cause and context, over-aligned and zero-sized concrete E/P, SourcePublished progress panic, ContextReady and ContextClosed panic, ReservationReleased panic, allocator panic, and source/policy destructor unwinding. Current concurrency tests publish one cause per thread but do not race its final clones. Current reentrant policy reads committed bytes; it does not reserve another source from a policy callback. Barrier tests may hang rather than report a bounded failure if the first allocator is rejected unexpectedly, so the native runner must retain its independent timeout/cancellation control.

No new concrete Rust source blocker was found in this audited revision. Publication remains held: this observation cannot replace the owning native epoch, sanitizer/interpreter evidence, or native whole-roster runtime receipts.
