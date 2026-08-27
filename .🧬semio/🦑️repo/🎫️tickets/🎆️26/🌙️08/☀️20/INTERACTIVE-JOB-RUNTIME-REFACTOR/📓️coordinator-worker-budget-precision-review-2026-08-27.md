# Worker Budget Precision Review

## Source Finding

The full command census's remaining worker-limit predicate still extracts `dispatch_typed_command_inner`, although the fuel, step-budget and output-allocation setup now resides in the exact `start_typed_command_operation` helper. This explains a stale source-location requirement, but does not justify declaring the complete worker-budget gate green.

Actual production setup converts `max_step_micros` to `step_budget_ms` with integer division by 1,000. The action-bus contract accepts every nonzero microsecond value below 8,000. `drive_worker_job_authority` sets the absolute deadline to the current millisecond clock plus that converted value, and `StepContext` treats `now >= deadline` as expired. Thus a valid 1–999 microsecond contract receives a zero-duration grant; a cooperative job checking `should_yield` cannot progress under that grant. The coordinator read these actual paths; this is source reasoning, not a new executed native test or a claim that a particular registered app currently declares such a value.

## Required Correction

Preserve genuine submillisecond timing through the retained worker/session budget seam. Do not round up beyond the declared contract or prohibit the planned 0.5 ms steps. A strict language-neutral fake-clock fixture and native/registered-path tests must cover submillisecond progress, exact expiration, cancellation, zero fuel and checked deadline overflow. The verifier must then bind the exact helper and require the corrected units, without relaxing independent decode/output/cleanup obligations.

This packet is queued with the publication executor after the current CAD gate. No coordinator production edits were made. The gate remains open while its source predicate and real precision defect are repaired together.
