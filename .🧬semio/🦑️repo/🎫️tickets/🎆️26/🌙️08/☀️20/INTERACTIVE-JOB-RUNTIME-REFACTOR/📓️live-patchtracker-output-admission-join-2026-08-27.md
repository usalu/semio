# Live PatchTracker Output Admission Join

Production owner: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️component.rs`. The runtime transaction module is cfg(test)-only and is not this live join. Dag reserved grant/producer/ready regions for this lane; NativeCloseKey, issued receipt, and final return-source authority remain his regions.

Current exact gaps read in production:

- Ready capacity is checked during drive, after producer admission. Reserve the shared output entry and queue before returning a mounted producer grant.
- `SurfaceReconcileJob::take_ready` reserves another handback after candidate sealing. The exact output handback must be acquired before producer work and retained through cancellation/refusal.
- Producer completion takes several optional roots in a tuple before validating all operands. Preflight their presence and exact generation before any move.
- Drive detaches the SurfaceSlot and producer before invoking the child. The slot, producer, tree, and reservation must remain structurally retained through child unwind; restoring only Result failures is insufficient.
- A rejected Ready transfer must remain in its structural source. Dropping it into newly allocated Box handback is not successful bounded transfer.

Proposed live representation: ReadySlot stores only generation/instance plus a small `SurfaceReconcileOutputs` queue handle, not a second Ready payload array. One entry is reserved before producer invocation, remains held by the mounted grant/producer/job, and receives the paired Ready after the job completes. Existing exact generation ordering and closing-instance exclusion remain unchanged. FIFO extraction targets an already-empty structural caller slot; poison is a typed error, contention is no progress. A rejected downstream transfer retains the original Ready until re-admission succeeds.

Required native REDs before production edits: saturated output pool prevents actual presenter/producer invocation; saturated handback cannot admit a producer that will later strand at seal; every partial completion presence combination keeps original roots; actual child panic leaves the structural producer/job roots recoverable; rejected final publication preserves the same Ready payload pointer and credit.

The pure runtime pool six laws and static same-ledger join are executed prerequisites, not this live gate. The preceding inventory describes the original source checkpoint, before the changes recorded below.

## Current Native Inventory Boundary

Runtime paired handback admission and in-place Job transfer now have actual R69–R75 evidence in `📓️runtime-output-handback-preadmission-2026-08-27.md`. The authored Plugin Job consumer now keeps its structural SurfaceSlot during child advancement and uses granted `take_ready_into`; this Plugin join has not yet compiled successfully. Producer completion, shared output admission, original NativeCloseKey propagation, and per-tracker backing remain open.

Three Plugin semantic RED tests are mounted against existing APIs, with production producer behavior unchanged: full shared output pool refuses before tree creation; an actual child-step panic retains the original Slot/Box/reservation; and a missing reconciler at producer completion preserves the remaining reservation and tree. The third test holds the omitted reconciler outside the driver, observes both remaining owners before cleanup, restores only that deliberately held reconciler, and drains before asserting. No missing API is injected into the baseline Plugin inventory. These three tests are not executed yet.

The permanent output schema and Node Buffer oracle now include incomplete-source preservation. This is a source contract and independent ownership model, not native recovery evidence. Native Plugin R6 stopped before execution with 19 compile diagnostics; R7 subsequently stopped with 11 E0599 trait-import diagnostics in the Mutation transaction fixture, before any test ran. The full R7 output is retained in `📓️plugin-native-inventory-r7-2026-08-27.md`. The shared source hold was released, with an immediate R8 inventory queued after the owning lane's narrow source release.

Full runtime R76's 113 passing/six failing transaction cases remain historical evidence. The cfg(test)-only canonical transaction cutover then passed full119 at R78. An additional independent zero-node-credit law was actual RED at R79, repaired and GREEN at R80, then exhaustive R81 passed all120 with no exclusions. See `📓️runtime-full-exhaustive-r81-native-2026-08-27.md`; none of these runtime-library gates executes the still-pending Plugin producer admission tests or proves live callback timing.

## Captured Lifetime Admission Boundary

The intended production signature is `reserve_mounted(surface, captured NativeCloseKey)`, with the real WIT caller supplying a key captured from its runtime-owned lease. The key is not reconstructed from surface text, the current numeric instance, or a current-generation lookup. It remains with the structural surface/grant/producer/job metadata and Ready handoff. Unit fixtures may use private fixture keys only; they are not guest capture evidence.

Close reservation must validate this original authority too. Merely attaching a key to Ready while `reserve_close_instance` accepts a stale key and scans all matching numeric descendants would still allow a reused instance to be closed. The exact ClosingInstance/receipt owner remains Dag's region; this read-only prerequisite was sent to him before mounting key fields.

The per-tracker boxed banks still require real backing/initialization admission. The process-static runtime ledger cannot be used to pretend these per-tracker allocations are already charged. Shared output FIFO currently orders completed `put` calls, while PatchTracker publishes the minimum admitted generation; live adoption must preserve existing ordering and exact cancellation, not silently switch to completion order.

Follow-up: the narrow cfg(test) transaction oracle cutover completed separately at R77 16/0 and R78 full119/0; see `📓️transaction-canonical-oracle-cutover-2026-08-27.md`. This does not close any of the live prerequisites above.

## Structural Shared Output Transfer Design

Removing the per-tracker Ready payload bank requires more than reserving the pool and then creating an intermediate local Ready. The shared entry must be the structural receiver of the job transfer. A narrow runtime operation can validate the exact reservation/queue/generation, preflight all queue metadata, and pass the reserved entry's empty `Option<Ready>` to `take_ready_into`, with the original Job and current-root destination retained in the caller's structural slot. It must debit both fixed transfers, retain an entry on any partial fault, and never invoke a producer or allocate under the registry guard. This is an unimplemented API proposal, not a source-ready claim.

One small output-queue handle per mounted output admission avoids changing existing minimum-generation selection: Ready metadata can select the original generation, then transfer from its exact one-entry queue into the preadmitted Pending receiver. Multiple completed outputs for the same surface still require distinct preadmitted Ready metadata. The shared 64-entry/64-queue limits remain unchanged, and the exact captured NativeCloseKey stays in admission metadata. This design avoids relying on the current shared FIFO's completion order as an implicit replacement for generation order.

Cancellation must retain both a possibly pending reservation return and its exact queue handle until terminal. Close and fault paths need explicit entry states for reserved, payload-owned, and detached phases; an atomic reservation return cannot silently release an entry that has already received a Ready. Actual saturation, partial transfer, contention, unwind, generation-order, and receiver-refusal tests are required before this proposal can become live proof.

## R2–R3 Current Boundary

R2 executed the original live tests plus an exact cancellation/Drop-generation law: three passed, one failed. Child-step unwind and incomplete-source ownership are green; cancellation preserves the real nonzero admitted generation and closes. Saturation now fails at its intended admission assertion after successful cleanup, rather than the former generation-zero cleanup defect. Full output is in `📓️plugin-mounted-output-admission-r2-native-2026-08-27.md`.

The direct runtime job-to-reserved-entry API then compiled RED at R82 (missing methods) and passed its native exact-pointer/refusal/post-transfer callback-unwind law at R83 (one passed,120 skipped). It declares8,936 transfer bytes within the unchanged32KiB grant; no intermediate local Ready exists. `📓️runtime-direct-output-receiver-r82-r83-2026-08-27.md` contains both outputs.

Current live candidate reserves queue+entry before returning the mounted grant, stores only queue/reservation/lifetime/generation metadata in ReadySlot, preserves original NativeCloseKey across SurfaceSlot/rejected/terminal paths, and transfers directly into the reserved entry. Peek/take validates both captured key and admission generation; old production ownership-returning Ready APIs are removed, with an explicitly cfg(test)-owned helper retained. Dag owns actual WIT/Pending caller adoption. Empty results and canceled reservations retain their queue until incremental close. Fault status is retained with exact lifetime rather than treated as successful retirement.

R3 stopped before tests with two Mutation fixture boxed-command E0308 diagnostics; it did not verify this live candidate. See `📓️plugin-mounted-output-admission-r3-native-2026-08-27.md`. Producer constructor allocation/root handoff, eager per-tracker banks, actual guest integration, and whole callback timing remain separate open obligations.
