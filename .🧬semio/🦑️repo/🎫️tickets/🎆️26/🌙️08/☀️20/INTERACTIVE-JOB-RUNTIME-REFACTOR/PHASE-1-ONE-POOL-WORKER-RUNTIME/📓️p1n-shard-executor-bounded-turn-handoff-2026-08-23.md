# P1n ShardExecutor Bounded-Turn Handoff

Date: 2026-08-23

## Outcome

The production `ShardExecutor` pool closure no longer calls `semio_framework_async::block_on`, waits on an async receive, drains the outcome channel, or loops epochs. It validates the admitted epoch before locking shard state, polls one retained `ShardLoop::drive_one` future, takes at most one nonblocking outcome, and attempts one successor only when retained work remains.

The first Terra audit rejected the packet because a pending drive had a no-op waker and immediate `Blocked` resubmission, a rejected successor depended on later ingress, deferred owners were dynamic, and one closure drained every registration. The live remediation replaces all four source shapes. This is a source-only re-audit packet, not Phase 1 acceptance.

`WorkerPool::try_submit` rejection returns the exact closure into the executor's retained handoff slot. Contended/saturated handoffs arm one generation-keyed process-pool timer callback with a finite eight-attempt backoff; no new thread, runtime, periodic poll, or ingress dependency was introduced. Wake/retry storms coalesce behind one atomic admission. Shutdown, poison, and exhausted retry transfer the original lane and exact closure into `terminal_handoff`; the host can take that one authority or explicitly resume it after restoring admission.

A pending shard future registers a real `Weak<ShardExecutor>` waker tagged with the active drive generation. Pending releases the single-flight gate and parks. The first matching readiness/cancellation/deadline wake transitions `waiting -> scheduled`; duplicate and stale wakes do not submit. `ShardDrive::Blocked` is no longer manufactured from `Poll::Pending` or included in immediate successor decisions.

The second Terra re-audit rejected malformed-frame accounting, terminal late-ingress ownership, incomplete lifecycle-byte admission, and inline Register/Unregister lifecycle execution. The remediation below closes those source paths for another independent source review; it does not claim Phase 1 acceptance.

Malformed, trailing-byte, nested-payload-malformed, and permanently over-capacity frames move their exact raw `Vec<u8>` into a fixed terminal-frame ring. A retained capacity-rejected frame is `(original ingress epoch, exact raw frame)`, so transient saturation advances no epoch; later successful admission or permanent terminalization returns that original epoch exactly once. The executor uses a FIFO compare-exchange from `epoch - 1` to that exact epoch. Terminal frames are excluded from `has_pending_work`, so a fault cannot hot-resubmit solely because its terminal owner is waiting. `take_terminal_frame`/`close_terminal_frame` pop one exact owner and clear that readiness.

Ingress is serialized by `ingress_gate`. Raw bytes beyond `SHARD_FRAME_MAX_BYTES`, or any frame arriving after Closing/Shutdown/Poisoned, return `FrameIngress::Rejected(TerminalFrameOwner)` before transport enqueue, admitted-epoch mutation, or lane mutation. The owner exposes its reason and can be recovered with `into_frame` or explicitly closed. A malformed admitted frame closes later ingress while still permitting already-admitted FIFO work to finish.

The third Terra source review isolated one remaining boundary: a full 256-slot raw terminal ring previously converted the 257th terminal frame back into ordinary `rejected_frame` readiness. That exact seam now has a separate fixed one-slot, generation-keyed `terminal_frame_overflow` handoff. The 257th malformed or permanently-over-capacity frame parks there with its original epoch and raw `Vec<u8>`; it does not enter `has_pending_work`, acknowledge its epoch, or permit the executor's epoch deficit to schedule a successor.

`take_terminal_frame`/`close_terminal_frame` removes exactly one older FIFO owner. While holding shard state, that capacity change checks both item and byte room and appends at most one overflow owner to the freed terminal-ring tail. The executor then clears the occupied marker, acknowledges exactly that overflow epoch once, and claims at most one successor only when a later already-admitted epoch remains. If one retrieval did not free enough byte capacity, the overflow remains parked and another retrieval returns one older owner without scheduling. Shutdown/Poisoned keeps scheduling terminal while the same one-owner-per-call retrieval/close path remains available.

An executor-side `terminal_overflow_occupied` marker is set and ingress is closed before the completed drive releases shard state. Any 258th or later newly submitted frame therefore returns its exact `TerminalFrameOwner` synchronously with `TerminalCapacity` before transport, epoch, or lane mutation. Already-admitted FIFO frames remain on transport and resume only after the parked overflow is re-armed; no second overflow layer can be created.

## Shard Drive Boundary

`ShardLoop::drive_one` consumes at most one transport frame and executes at most one actor turn, job-step, or lifecycle authority. `ShardDrive::{Idle,MoreWork,Fault}` returns the exact consumed ingress epoch, so the executor advances only the frame actually admitted or terminalized. A deferred actor turn therefore cannot falsely acknowledge and strand a newer transport frame.

Registration, Register/Unregister, completion/event, explicit job-step, cancellation-cursor, Suspend, Resume, terminal lifecycle, raw terminal-frame, and failure owners use the fixed-capacity generation-keyed ring primitive. Item and byte admission are preflighted before frame mutation. Grant and Envelope authorities split the already-admitted raw frame length exactly; Register/Unregister each reserve one item and their complete raw identifier-bearing frame length. Generated completion ownership uses exact inline `Event` size plus the owned result buffer capacity, not a whole-envelope re-encode. A rejected owner is returned intact; a temporarily unadmittable frame retains its original epoch and exact bytes, while a frame that cannot fit an empty ring retains its bytes for observable terminal close. Generation keys prevent a stale key from addressing a reused physical slot. Registrations stay FIFO and `ShardExecutor::run` pops at most one before its one drive poll.

Actor cancellation retains a fixed FIFO cursor and cancels at most one job authority per drive before returning that exact cursor to one freed slot. Cancellation failure terminally retires the actor and emits one fault rather than discarding a dynamic remainder. Completion/failure saturation retains one exact terminal owner with an explicit retrieval path.

The native thread transport now exposes owned `send_now` and `try_recv_now`; `SharedThreadTransport::recv` delegates directly to the latter, so the retained shard drive never parks on a thread-channel receive. The potentially suspending shard drive registers its real one-shot waker and remains an owned future cursor across closures.

## Source Ratchet and Fixtures

The existing root `📜️script.ts` interactivity audit now scans the exact production shard executor even though that file is outside the current general UI-root prefixes. Production `block_on` in this executor is therefore denied.

The bounded-handoff source verifier checks the live executor/shard pair and runs rejection fixtures for:

- full-queue local draining instead of yielding;
- more than one drive opportunity per admission;
- stale-epoch mutation before validation;
- cap-plus-one closure ownership loss;
- pending-drive no-op wake registration;
- duplicate submission under wake storms;
- saturated quiet-ingress handoff without a timer trigger;
- terminal successor retention without a take/resume path;
- malformed fault without exact epoch consumption;
- terminal-frame retrieval that leaves readiness set;
- terminal ring 256/+1 falling back to ordinary rejected-frame readiness;
- overflow without one-slot generation ownership or FIFO tail re-arm;
- no-retrieval overflow hot resubmission;
- occupied-overflow 258th ingress enqueue/epoch mutation;
- terminal late ingress that reaches transport ownership;
- raw Grant bytes without cap/+1 ownership;
- Register/Unregister identifier bytes without fixed admission;
- Suspend/Resume item or byte cap bypass;
- mixed lifecycle execution inside one grant;
- fallback byte estimates or whole-envelope re-encoding;
- dynamic item overflow and missing byte preflight;
- ABA generation loss;
- all-registration draining in one closure;
- interrupted close draining multiple authorities per grant.

The Rust source also contains direct fixtures for malformed one-shot terminalization/no-other-work, 256/+1 malformed terminal overflow, permanently-over-capacity overflow, no-retrieval non-readiness, one-pop FIFO tail re-arm, exact overflow epoch, overflow-slot ABA rejection, occupied-overflow +1/+2 synchronous ingress handback and close, exact terminal retrieval clearing, retained original-epoch admission, terminal late-ingress handback before mutation, raw frame bytes/+1, exact Grant credit splitting, Suspend/Resume item and byte cap/+1 handback, mixed lifecycle FIFO one-pop behavior, fixed-owner item/byte boundaries, wake-storm coalescing, and one interrupted-close authority per grant. Under the command restriction these Rust tests were not executed; the TypeScript adversarial source fixtures were executed by the interactivity verifier.

## Verification

Passing source-only checks:

```text
rustfmt --edition 2021 <async pool, actor transport, shard component, shard executor>
rustfmt --edition 2021 --check <same files>
bun ./📜️script.ts verify interactivity
bun ./📜️script.ts verify interactivity --self-test
git diff --check -- <packet files and root script>
git diff --check
git diff --cached --check
git diff HEAD --check
```

The interactivity audit completed in deny mode. Its sole reported blocking-bridge finding is the existing allowlisted native renderer process entry. Production `ShardExecutor` has zero `block_on` findings; its remaining literal `block_on` matches are inside `#[cfg(test)]` sender fixtures and are excluded by the audit.

No Cargo, Nx, Wasm, browser, root lint, or external-network command was run. Consequently compilation, runtime wake timing, saturation recovery timing, and worker census remain unverified; this report records source and verifier status only.

## Files

- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs`
- `🧰️framework/🔨️modules/⏳️async/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs`
- `📜️script.ts`

## Remaining Phase 1 Blockers

This packet does not establish Phase 1 acceptance. The current readiness audit still assigns the separately owned MCP transport additional-runtime path and store-sync nested actor blocking path as Phase 1 blockers. Current serialized native runtime evidence for worker/thread census, wake/saturation behavior, permit behavior, and the supported plugin-host synthetic runtime path also remains outstanding. The source-only packet itself still requires independent compilation/runtime review because the permitted verification boundary explicitly excluded Cargo and runtime tests. MCP, store-sync, SceneStore/schema, renderer/board, P8 stores/components, dependencies, Compose, Dagre, ticket metadata/checklists, coordinator reports, and Git state were not edited by this packet.

The repository-wide worktree contains concurrent unrelated changes, including dependency files and other phase sources. Scoped and whole-tree whitespace checks pass, but this packet makes no claim about those unrelated diffs.
