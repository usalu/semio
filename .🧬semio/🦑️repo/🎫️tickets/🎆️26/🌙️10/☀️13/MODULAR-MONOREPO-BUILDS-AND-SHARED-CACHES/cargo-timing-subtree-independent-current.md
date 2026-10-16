# Cargo Timing and Owned Subtree Independent Audit

Source-only review, 2026-10-15. No Actor/cohort run, probes, tests or images.

No concrete deadline renewal, preparation exclusion, injected execution authority or undrained accepted sampler established in the inspected Source.

`library/🏃️process/🦀️cargo/⏱️timing/🟦️.ts` contains pure observational parsing/formatting. Unix snapshot parsing limits incoming bytes and row count, refuses malformed/duplicate process rows, selects root plus descendants through linear BFS, bounds state/name fields, and computes deltas only with matching PID/birth/parent and nondecreasing CPU/elapsed clocks. It grants no CPU/storage/process control authority. CPU/elapsed clock parser handles days, hours, minutes and millisecond fractions with explicit limits and safe integer checks. Actual lstart has second-level resolution, so delta identity is an OS observation rather than cryptographic proof against every possible very rapid descendant PID reuse; it must remain diagnostic.

Schema policy bounds snapshots to 1 MiB/8192 rows, output subtree to 64 rows/16384 bytes, at most 120 samples and 1 MiB emitted sampler JSON. Oversized complete snapshots refuse rather than claiming truncated census completeness. The record helper removes tail rows to meet its encoded byte bound. Non-Unix platforms emit typed unavailable status instead of pretending native observations exist.

Ticket `actor-current/📜️script.ts:16-19` registers the observer under the carried original control, narrows each actual ps call to 2000 ms without renewing parent time, and uses the exact current actor process as root. The observer reads only lstat metadata for recency. Stop wakes its pending timer; finally awaits the accepted observer and closes its image. Observer failure is observed, propagated and combined with a primary compile failure rather than discarded. A failure after initial ready can be reported only when compilation reaches the finally drain; it does not authorize early process termination or a new deadline.

Cargo launcher phase timing is opt-in, bounded by per-process aggregate bytes and phase event count, writes debug stderr and samples the existing remaining control. The inspected command still uses the same preparation/input/graph/source/writer/profile/capacity/scheduler/child flow; timing markers do not select an alternative execution path or excluded dependency set. Storage is adding throttled existing publish observations within the same budget; final added lines require review if they change beyond formatting.

The fixture/parser oracle checks and actual host sample results reported by Storage were not rerun here. Source establishes bounded diagnostic intent, not current host utilization, process ownership custody, scheduler correctness or all resource limits. Root-authorized next Actor execution remains separate from this audit.

## Final Progress Addition

Current launcher lines 105-110 still await the unchanged throttled admission.publish before diagnostic output. Phase/resource event ceilings (96/256) share the per-process 1 MiB emitted-byte budget, with a 16 KiB record ceiling. Resource names truncate at 256 characters with an explicit flag. Malformed progress formatting is caught and emits typed unavailable status, adding no new refusal to the underlying operation. Remaining-time reads do not replace the control or its deadline. No new actionable omission established in this final addition.
