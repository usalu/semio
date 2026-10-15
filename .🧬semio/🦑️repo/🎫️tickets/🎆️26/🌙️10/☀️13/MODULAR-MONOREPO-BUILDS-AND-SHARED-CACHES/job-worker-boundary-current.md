# Remaining Worker Source Boundary

The current worker production closure is 171449 bytes across 8 files; its unit closure is 379234 bytes across 33 files. The unchanged admission limits are 262144 bytes, 32 files and 4000 active lines. Multiple private test modules still aggregate into one Rust unit, so moving their filenames alone cannot resolve this boundary.

| Actual test owner | Rust body bytes | Custody classification |
| --- | ---: | --- |
| Worker retained ownership | 38989 | Session identity, contention, private exhaustion and slot laws remain with their real owner; public semantic portions may be separated. |
| Terminal authority retirement | 28706 | Contains real authority/job/params retirement plus session/node assertions. These are not all movable to a public integration. |
| Worker temporal budgets | 26946 | Direct authority, quarantine and injected overrun state. The actual authority lifecycle owns these private invariants. |
| Typed wake retirement | 16232 | Native issuer, TLS lease and parked node/Arc custody. Keep private node/slot assertions beside the nominal owner. |
| Original worker turn | 14635 | Some public receipt/admission behavior; private mutation must be separated rather than exported. |
| Native erasure | 13876 | Mixed public successful erasure and deliberate private receipt refusal/scaffold assertions. |
| Native preparation | 12875 | Public behavior candidate with private authority/job/ledger observations requiring a separate invariant. |
| Close demand | 10304 | Exact private authority/close ladder and deliberate receipt refusal. |
| Staged metadata | 7521 | Public behavior. Its independent successor now uses one actual Original caller for every turn. |
| Session frame return | 5121 | Private sealed frame/handle semantics. |
| Worker clock stride | 4259 | Private direct authority behavior. |

The two existing public worker targets (payload and normal failure) are already independent and therefore do not reduce the present unit aggregate again. The new staged target is declared after genuine missing-target RED; its native successor is still pending. It cannot be counted as a completed Source reduction before the old mount is removed after runtime qualification.

Native preparation, successful erasure and public worker turn behavior are further useful independent targets, but the measured private aggregate still needs a real production owner boundary. Exporting a mutable `WorkerJobAuthority`, `SessionHandle`, raw retirement slot, ledger Arc or a giant test runner would hide the same SCC and is unsuitable.

A concrete candidate is the lower native job authority lifecycle: its actual fixed heap owner, initialized source, batch parameters, original callback/receipt/quarantine state, fault payload and paid retirement. It currently occupies worker root declarations `WorkerJobSource`, `BatchDriveConfig`, `BatchJobParams`, `WorkerJobAuthority` and `WorkerJobAuthorityOwner`, plus the callback driver and authority demand/retirement functions. It depends on core/Async/Value/Trace. Session submission, slot/node/Arc custody and wake delivery can remain above it. This is a candidate dependency cut, not an implemented or proven acyclic crate.

Qualification must first classify every cross-boundary operation into an actual opaque ownership transition: prepaid native storage admission, bounded source/destination initialization, issuing and receiving one callback receipt, immutable semantic/job borrow, guarded mutable job borrow, original cancellation witness, and paid closure. The lower owner must not depend on sessions or return a raw mutable authority. Direct private authority/quarantine tests can follow the real lower bodies; mixed session tests retain their private session invariants above. The original 103 named law bodies must remain inventoried throughout. New package members/dependencies/registry and all actual downstream imports need the same TOML/SQLite graph and bounded native proof as the nine completed production owners.

No production extraction, new package, reverse edge, relaxed cap or native success is claimed by this audit.

## Implemented Boundary Superseding the Initial Allocation Proposal

The native allocation belongs to the lower authority owner. Keeping it above would leave the real private allocation/ledger laws unable to follow their actual owner without exposing mutable custody. The lower crate therefore owns its fixed native Vec and private state through `OriginalWorkerJobAuthority`; the higher worker retains only session scheduling, checkout, handles, native Arc, retirement slots and wake coordination. The lower exports opaque live-allocation identity equality and fault-capacity observations, without Deref, mutable fields, ledger Arc or a core-to-worker edge.

Actual production84861 passed after Framework65813. Lower initial9 native laws passed both modes39400/31390; later source-reader and exact paid terminal source removal additions require current requalification. Genuine full unit50110 compiled23 laws and ran13pass/10fail. The current corrected unit has an added actual Original-funded phase fence law and preserves every original named test. Native and portable proofs remain unfinished until the current source is requalified.

The lower callback and complete child-close ladder stay nominal. The upper checks out the same opaque allocation through its existing atomic session phase gate, restores the same owner after every pending/error outcome, and consumes the lower receipt through a shared original before/after composer. The actual normal-failure public runtime exposed a preexisting terminal cancellation-witness stall. Only authority-empty state now reports false/None; every live unsupported phase still refuses. Separate native frame return also required its genuine body-empty predicate rather than an already-fully-released-owner predicate. Neither repair changes Source limits or admits a fabricated original.

The existing Script API remains Bun/Nx and project-script only. The new owner is registered at its actual package root and both editor catalogs include its native route. Public turn/preparation targets preserve original selectors and testkit requirements. The remaining duplicated unmounted old test bodies will be removed only after equivalent current public targets pass both modes. No giant owner is hidden behind a facade.
