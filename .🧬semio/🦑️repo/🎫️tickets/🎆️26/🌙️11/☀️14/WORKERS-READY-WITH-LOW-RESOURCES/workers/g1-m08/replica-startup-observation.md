# Replica Startup Observation

Read-only observation after round 1, at 2026-11-14T12:37:55Z. No service was changed or restarted.

- LaunchAgent: running, `/bin/sh`, PID 25748, two runs, last exit code 0.
- Its sampled child: `repo`, PID 25755.
- The one-second stack sample showed its main thread in `open` / `__open` throughout the sample. Physical footprint was 6225 KiB; sampled report peak was 6257 KiB.
- The path being opened and the cause of that wait are **not measured** by this stack sample. No permission failure, network fault, or graph dependency is inferred as proven.
- The last replica status observation separately reported daemon not running and a stale attestation. The verified fleet-store check had not found this worker’s interim report.

Evidence: [replica-entry-observation.txt](replica-entry-observation.txt), [replica-entry-stack.sample.txt](replica-entry-stack.sample.txt), [round-1-status-after.txt](round-1-status-after.txt).

The coordinator control watcher remains active. No sudo, process termination, runtime repair, or code change was performed for this observation.
