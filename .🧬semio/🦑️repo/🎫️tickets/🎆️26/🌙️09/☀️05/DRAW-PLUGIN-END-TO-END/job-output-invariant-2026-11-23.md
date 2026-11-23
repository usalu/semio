# Job Output Invariant (2026-11-23)

## Root cause
`step_on_source` (`retained-turn/💼️job/🏠️state/🦀️.rs`) refuses a step while the guest holds an unacknowledged output (`ack` set, or an output whose identity differs from the stepped job). The host `OriginalJobFunds.step` (`retained-turn/🟦️.ts`) always issued a fresh `stepJob` on every request, even when an earlier request of the same job had stopped inside its collection (read pages, or the final acknowledgement stalled or failed after the guest set `ack`). The retried step, or the first step of another job after a cancel, then met the held output/closing phase and the guest faulted with `InvariantViolated`, which fences the producer (`original guest failure fences a registered job producer`) and fails every later turn. `cancel` also left such output held in the guest (the guest only flags cancellation when output is held).

The runtime trigger was inferred from the guest and host code, not observed live: the journey report only holds the fault chain, so which exact crossing stalled first is not confirmed.

## Fix
`retained-turn/🟦️.ts` (`OriginalJobFunds`, embedded into the bridge via `originalOperationWorkerSource`):
- The job output hold now records the guest's `outcome` (state, byteLength) of the step that produced it.
- `step` skips `stepJob` when a hold with an outcome exists and resumes collection/acknowledgement (the guest then answers the closed page and accepts the ack).
- `cancel` first drains an unfinished collection of its job (read to close and ack) and only crosses `cancelJob` when the job is still running.

## Verification
- New tests in `retained-turn/🧪️tests/💼️job-funds/🟦️.ts` use a strict mock guest enforcing the Rust invariant: a retried step after a stalled acknowledgement must not step again; cancel must drain before the next job steps.
- `repo test-file` (SEMIO_BUILD_MODE=prototype SEMIO_LANGUAGE=en): job-funds 15/15 pass; wit-lowering test run as well.
- Not verified: the Draw journey in the browser (component and bridge need rebuild by the owner). If the fault persists after rebuild, the first stalled crossing is the next thing to log with `[DEBUG]`.
