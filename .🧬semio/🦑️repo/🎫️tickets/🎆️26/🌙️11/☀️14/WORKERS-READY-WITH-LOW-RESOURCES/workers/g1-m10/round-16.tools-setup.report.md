# Step 0 (`repo tools setup all`) on g1-m10: no exit 0

| attempt | start (UTC) | end (UTC) | wall s | rc | end |
|---|---|---|---|---|---|
| 1 | not measured here (see round-16.tools-progress.md) | 04:05 | about 3400 per earlier notes (not re-measured) | killed | external SIGTERM (not by this agent), no metrics |
| 2 | 04:06:42 | 04:18:06 | 684 | 1 | `Current complete Source binding or whole receipt changed` (fixed in round 20) |
| 3 | 04:33:23 | 07:53:45 | 12022 | 1 | `Internal error: Resource lease timed out: native-storage:[16777233,1112611,38653698048,16777255]` |
| 4 | 07:55:01 | 08:51:10 | 3369 | 1 | same lease timeout |
| 5 | 08:51:29 | 08:57:07 | 338 | 1 | same lease timeout (replica code 0dc4d5abc2e3, unchanged since attempt 4) |

Each ends with `[DATA] Dependency sync incomplete {"completedBindingsChanged":0,...,"wholeAllocationRetired":false,...}` then the lease timeout and `repository:command failed with exit code 1`. No `restored` / `installed` per-scope line appears; only `Funding the dependency store ...` and `javascript-installed-paths` scans (attempt 4 and 5 logs). Last 20 lines: `round-16.tools-setup.last20*.txt`. Logs: `round-16.tools-setup.attempt<n>-*.log`.
Not retried further: identical error three times on identical code. Continuing with the bootstrap try (protocol: go on with the try).
