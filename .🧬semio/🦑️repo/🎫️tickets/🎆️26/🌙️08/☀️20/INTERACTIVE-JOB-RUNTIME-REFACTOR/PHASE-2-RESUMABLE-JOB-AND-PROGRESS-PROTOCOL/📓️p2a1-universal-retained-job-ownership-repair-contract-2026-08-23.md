# P2a1 Universal Retained-job Ownership Repair Contract — 2026-08-23

## Status

**RED contract prepared for implementation.** This is a read-only coordinator census of the
universal job layer after P2d's independent acceptance. It does not accept P2a or Phase 2. No Cargo,
Nx, Wasm, browser, runtime, or network gate was run.

## Live Boundary

The owned source boundary is `🧰️framework/🔨️modules/🧵️job/🦀️component.rs`, its crate exports and
the minimum callers needed to remove the two live terminal-drain adapters. `WorkerJobSession` has
multiple live plugin, reactor, and host consumers; changes must be schema-first and may update all
callers because the repository has no compatibility requirement.

The existing `InteractiveJob::step` and `drive_step` timing/cancellation vocabulary is useful. The
universal ownership around inputs, outputs, children, submission, result delivery, and close is not
yet safe.

## Source Defects to Remove

1. `Checkpoint.state`, `CommitCandidate::{state,output}`, `JobFault.detail`, preview payloads, and
   progress payloads are ordinary cloneable `Vec<u8>` owners. A job can allocate arbitrary output
   before returning it; the runtime has no output item/byte/process credit, exact rejected owner, or
   page-by-page terminal disposer.
2. `Operation::next_preview_sequence` and `StepContext::next_preview_sequence` use unchecked `+= 1`.
   Release overflow aliases a prior sequence; debug overflow panics after work has begun.
3. `JobScope` is only an unbounded `AtomicU32` count. Child admission can overflow, child identity
   and generation do not exist, and `assert_completable` is a release-mode no-op. A parent may
   publish terminal success with live children.
4. `run_to_completion` is an unbounded public loop with a live renderer caller. `run_on_worker` and
   `run_on_worker_async` use blocking mutexes, unbounded/self-requeue terminal drains, dynamically
   allocated channels, and recursive scheduling. A discarded receiver loses the only terminal
   outcome and can deep-drop it on a worker.
5. `WorkerJobSession` permits overlapping submissions against one blocking mutex, panics on finite
   submission rejection, drops the rejected closure, turns post-terminal submissions into `Yield`,
   ignores receiver-send failure, and has no public rejected/terminal take-resume/one-owner close
   authority. Panic/poison, pool shutdown, receiver discard, cancellation, and session `Drop` do not
   converge on one observable terminal owner.
6. The session state is an `Arc<Mutex<_>>`; `try_into_job` is merely an Arc-count probe and can never
   recover a terminal output or close retained job/output owners incrementally. There is no exact
   operation/item/byte/process registry, fixed in-flight slot, generation exhaustion rule, quiet
   wake authority, or terminal-empty witness.

## Required Owned Protocol

### Payload and output credits

- Replace every universal dynamic byte payload with a non-Clone owned paged payload. Each page must
  be the actual storage, at most 16 KiB, admitted from fixed operation item/byte and process byte
  authorities before the job can write into it. Do not wrap an already-built `Vec` and do not keep
  decorative accounting pages beside unrelated allocations.
- Expose a retained writer/reservation through `StepContext`. A job that needs multiple pages keeps
  the writer in its own cursor and fills at most one admitted page per turn. Checkpoint, preview,
  state, output, and fault variants transfer exact page authorities rather than cloning bytes.
- Saturation and max + 1 return the identical rejected reservation/source owner without registry
  mutation. Separate state and output streams need separate exact credits. Every terminal or stale
  path retires one payload page/root per close grant and returns aggregate process credit only after
  terminal-empty.
- Keep codecs behind owned interfaces; no public type may expose a third-party type.

### Fixed structured-child authority

- Replace the debug count with a fixed child registry keyed by parent operation/generation and exact
  child slot/generation. Child creation is a fallible pre-admission transition with retained
  rejection; slot generations never wrap and an exhausted slot is never aliased.
- Parent `Complete` must be rejected or retained-pending in every build while an exact child is live.
  Parent cancel/fault/drop records durable child-close intent; one mounted pump advances at most one
  child terminal/close unit and wakes only on a meaningful state transition.
- Child guard loss, panic, stale token, duplicate completion, max/max + 1, and completion racing the
  last child close must preserve the exact authority and cannot underflow/overflow a scalar.

### Single-step worker session

- Make `WorkerJobSession` a generation-tagged fixed authority with exactly one of idle job,
  submitted ticket, one retained outcome, terminal outcome, rejected owner, or close cursor. A
  second submission for the same generation must return a typed retained contention result; it may
  not block or enqueue another step.
- Transfer the exact job authority into one `WorkerPool::try_submit` closure. On lane rejection,
  recover the closure/job through the pool's error and restore the identical idle authority. Never
  panic and never erase the recoverable job.
- Worker panic/fault, pool shutdown, cancellation, receiver discard, callback/send failure, and
  session `Drop` must publish a durable terminal intent independently of a contended registry lock.
  A mounted wake/pump retries the intent. Terminal retrieval is `take`; contention or consumer Drop
  performs exact atomic handback; `resume` preserves generation and freshness.
- A session step returns after one `drive_step` opportunity. It never self-requeues and never waits
  on a blocking mutex/channel. Native and Wasm use the same state machine; Wasm pumping is one
  explicit opportunity, not a loop until delivery.
- Remove production `run_to_completion`, `run_on_worker`, and `run_on_worker_async` reachability.
  The headless adapter must itself be a caller-driven persistent batch session with one public
  `step/poll` opportunity. Any direct terminal oracle is strictly `#[cfg(test)]`.

### Terminal ownership and freshness

- All outcome variants are non-Clone exact owners. Publication observes operation, base revision,
  generation, step sequence, preview sequence, and cancellation before transfer. Sequence cursors
  use checked exhaustion; `u64::MAX` permanently exhausts the exact authority rather than wrapping
  or saturating into an alias.
- Provide public `take_rejected`, `take_terminal`, `resume`, `close_step`, and
  `terminal_is_empty` semantics. A close grant releases at most one exact fixed owner/page/root.
  The job shell, cancel token, child registry, result ticket, slot, and aggregate credit are distinct
  close units. No ordinary `Drop` path may recursively destroy retained payloads.
- Quiet jobs do not spin. One transition from idle/unpublished to ready/terminal raises one wake;
  redundant polling or duplicate intent raises none. A wake racing registration must be retained
  through a check/register/recheck or equivalent lost-wake-safe protocol.

## Required Fixtures

- payload items, per-operation bytes/pages, process bytes, children, sessions, and in-flight slots at
  max and max + 1, with exact pointer/page identity on every rejection;
- payload construction requiring multiple low-fuel turns, interrupted checkpoint/preview/candidate
  construction, separate state/output credits, and one-page-at-a-time close;
- release-mode parent completion with a live child, child slot exhaustion, child generation
  `u64::MAX`, duplicate/stale child completion, cancellation and panic at each child phase;
- overlapping session submission, pool admission rejection, closure panic, mutex/registry
  contention, shutdown, receiver discard before and after terminal, checked-out terminal Drop
  handback, resume contention, and session Drop in every authority phase;
- preview and step sequence `u64::MAX`, stale base revision/generation, cancel after complete, fault
  after output reservation, and exact last-valid publication preservation;
- native and Wasm-shaped quiet-wake/lost-wake/ABA schedules and a batch caller proving one external
  opportunity rather than a run-to-terminal drain.

## Permanent Verifier Requirements

Add faithful mutations that restore a public terminal loop, self-requeue, blocking mutex/channel,
overlapping submissions, panic-on-rejection, discarded pool closure, ignored send failure, cloneable
`Vec<u8>` outcome, post-allocation admission, decorative pages, wrapping/saturating sequence,
debug-only child completion, dynamic child registry, bulk clear/drop, missing take/resume/close,
missing terminal-empty witness, and missing mounted production caller cutover. Baseline predicates
alone are insufficient; each mutation must be rejected.

P2a1 is a prerequisite for later mounted layout and other job packets. P2a and Phase 2 remain open.
