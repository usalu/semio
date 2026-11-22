# Worker Build Verification at 19:00 UTC

Nine of twelve workers had fresh attestations for the exact coordinator stage `7d8c0b059bcaed05be881279fe59b4fa8305d514`. g1-m02 stopped attesting for over seven minutes while its workspace inventory kept advancing. It resumed attesting at 18:43 UTC, passed the previously pending source PUT, and eventually caught up with the other fresh workers. All nine cursors cover the 18 current file digests already present in the latest committed journal sample, out of 23 edited paths. The latest bootstrap snapshot retry receipt is pending publication for five paths in that sample. A current stage and a cursor both remain distinct from a loaded implementation or a Ready receipt. m04 and m06 still have stale reports. m09 still has no operational report.

m07's 18:36 UTC failed dependency attempt confirmed an intact Source symlink that resolved to generation `011ae1e34068d5343961` while the current binding receipt expected `a775b1d53e5d57cf7d81`. At 18:54 UTC it entered a new dependency attempt and the exact pressure fix arrived in the journal during the launch window. The running attempt advanced into generation collection. Its latest sampled per-tree counter reached 20,992 entries at 306.9 seconds. The process is still under the owning Ready supervisor. There is no new terminal receipt or measured confirmation that it has freed the full admission gap or repaired the binding. m03 and m10 had moved into their workspace stage by 19:00. [Dependency pressure admission](Dependency%20pressure%20admission.md)

| Verified change | Local evidence |
| --- | --- |
| Dependency pressure tries protected normal reclamation before strict retirement | Twenty-one tests passed. Live and pinned generations remain protected, including indirect Source targets. The strict retirement path still runs if measured capacity falls short. |
| Selected graph matching uses direct files for flat patterns sharing a recursive walk | Twenty-five tests passed. The fixture reduced the flat predicate from 721 calls to one and removed redundant directory-existence probes. |
| Graph lookup skips waits for a transport the existing breaker already refuses | Fifty-six tests passed. Authentic wire failures retain bounded recovery and peers can still be adopted on a later successful poll. |
| Boot retains the last valid owned heartbeat and retries refused snapshot publication | All 24 installed-entry tests passed. The final complete direct boot suite passed 23 tests after correcting a stack-sampling fixture and fixing the publication defect. Boot version 8 takes effect on a new boot-owner launch. |

[Selected graph source walk](audit-selected-graph-direct-source-walk.md) · [Owner cache breaker](audit-owner-cache-breaker-retries.md) · [Replica heartbeat ownership](audit-replica-owned-watchdog-silence.md) · [Snapshot publication retry](audit-bootstrap-snapshot-publication-retry.md)

## Remaining physical acceptance

- Zero of twelve workers report current Ready.
- The coordinator had 19,532,185,600 bytes free. The fixed 41 GiB isolated APFS lab requires 60,129,542,144 physical bytes before launching the 8,192 MiB, four-core all-stage attempt.
- The coordinator still reports no complete graph. Native PID 24613 remains live and queued for 14,336 MiB. A recent capacity sample reported 4,071 MiB above the 4,096 MiB floor. The authenticated adoption verifier returned `receipt-missing` for all twelve workers. [Coordinator native admission](audit-coordinator-swap-and-disk-1852.md) · [Exact graph authority](audit-graph-derivation-and-recovery-authority.md)
