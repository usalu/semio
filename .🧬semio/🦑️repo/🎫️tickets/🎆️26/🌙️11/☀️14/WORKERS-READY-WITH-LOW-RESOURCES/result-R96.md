# R96: Monotonic graph phase progress required

## Verdict
Neither (a) nor (b) as stated: the evidence predates R95. Clock: the host is CET; `service-failure.txt` mtime 21:19 is CET, and its own header says `2026-11-16T20:18:59Z`. R95 landed 21:35 CET (20:35Z), 16 minutes later. The first failure `service-failure-3a9fba88.txt` is `2026-11-16T18:47:59Z`. Both traces match the pre-R95 file: `begin` at `🔌️load/🟨️.mjs:274:92` (3a9fba88, `Closed graph phase and finite total required`) and `advance` at `:275:15` (79312331). In the post-R95 file line 275 is `else if(done<seen.get(phase))return;`. No later `Monotonic` failure file exists in the store (the 21:34Z `workspace-failed` is a memory refusal, unrelated).

Slice loads run in a fresh process per load (`👷️slice-pool`, `createProcessWorkerV1`), so a worker does not keep stale library code across a landing; residents were not changed.

## But R95 was incomplete (b, in part)
R95 only dropped smaller counts. `NaN`, fractional, negative, oversized or foreign-named phases, and a throwing clock or publish still rejected into the load (this is exactly failure 3a9fba88, the first one).

## Fix (landed 2026-11-16 22:03 UTC, lock R96, entry in landings.md)
`createGenerationProgressV1` (`🕰️generations/🔌️load/🟨️.mjs`): `handle` validates each report and runs `apply` behind a guard that resets the phase (and ends it in the clock) on any failure, so a report can never reject.
Test `no ordering, shape or failure of progress reports can reject a report and end the load` (300 seeded random interleavings with bad names, NaN, negatives, huge values, failing publish): red before (reproduces `Closed graph phase and finite total required` at 274:92), green after.

## Proof
Harness runs after the change: load tests 101 pass; standing preflight (`provider-closures/.../🛃️preflight`) 2 pass; `🪆️heavy-questions` 10 pass. Script: `🧰️scripts/R96/land.py`.

## Unverified
No worker was restarted to confirm g1-m10 now reads the slice end to end; `⏱️phases` unit tests were not re-run (unchanged file).
