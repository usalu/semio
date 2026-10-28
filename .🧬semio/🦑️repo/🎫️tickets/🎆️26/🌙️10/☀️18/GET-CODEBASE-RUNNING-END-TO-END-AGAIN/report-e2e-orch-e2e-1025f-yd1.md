# 🧭 e2e-orch report e2e-1025f-yd1

Created 2026-10-26T15:52:09+01:00, written 2026-10-26T16:21:01+01:00, orchestrator pid 89652. Argv: `course --phase e2e --course-topology dev --go --skip-preflight --run-id e2e-1025f-yd1`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: CANCELLED 1. Cancelled: True (SIGTERM).

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | FAIL 27m36s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | CANCELLED | 143 | 27m36s | 0 | 0 | 0 | 0 | - | 5 | cancelled: SIGTERM |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course test-e2e: `[capacity] waiting 0s for run: an earlier request is still waiting (need 6834 MB; 2746 MB spare above the 4096 MB floor, load 36.42; 2 running, 5 ahead)`
- course test-e2e: `[capacity] waiting 6s for run: an earlier request is still waiting (need 6834 MB; 2471 MB spare above the 4096 MB floor, load 33.63; 2 running, 5 ahead)`
- course test-e2e: `[capacity] waiting 11s for run: an earlier request is still waiting (need 6834 MB; 5701 MB spare above the 4096 MB floor, load 31.82; 2 running, 5 ahead)`
- course test-e2e: `[capacity] waiting 16s for run: an earlier request is still waiting (need 6834 MB; 6187 MB spare above the 4096 MB floor, load 31.82; 2 running, 5 ahead)`
- course test-e2e: `[capacity] waiting 21s for run: an earlier request is still waiting (need 6834 MB; 6321 MB spare above the 4096 MB floor, load 31.86; 2 running, 5 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-26T15:52:30+01:00, end@2026-10-26T16:21:01+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1025f-yd1`.

## Resources

Max load 44.5, min available memory 10.7 GB, min free disk 72.7 GB.

## Failures and warnings


### course e2e test-e2e: CANCELLED

- command: `bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh course --tool=bazel --topology=dev`
- note: cancelled: SIGTERM
- log: `🗑️generated/e2e-orch/e2e-1025f-yd1/course/e2e-1-test-e2e.log`

```
[capacity] waiting 1533s for run: not enough free memory (need 6834 MB; 2513 MB spare above the 4096 MB floor, load 32.96; 1 running, 0 ahead)
[capacity] waiting 1538s for run: not enough free memory (need 6834 MB; 2752 MB spare above the 4096 MB floor, load 32.88; 1 running, 0 ahead)
[capacity] waiting 1544s for run: not enough free memory (need 6834 MB; 1320 MB spare above the 4096 MB floor, load 34.57; 1 running, 0 ahead)
[capacity] waiting 1549s for run: not enough free memory (need 6834 MB; 1528 MB spare above the 4096 MB floor, load 35; 1 running, 0 ahead)
[capacity] waiting 1555s for run: not enough free memory (need 6834 MB; 1837 MB spare above the 4096 MB floor, load 35.89; 1 running, 0 ahead)
[capacity] waiting 1561s for run: not enough free memory (need 6834 MB; 2301 MB spare above the 4096 MB floor, load 34.85; 1 running, 0 ahead)
[capacity] waiting 1567s for run: not enough free memory (need 6834 MB; 6596 MB spare above the 4096 MB floor, load 33.55; 1 running, 0 ahead)
[capacity] waiting 1572s for run: not enough free memory (need 6834 MB; 752 MB spare above the 4096 MB floor, load 32.22; 3 running, 0 ahead)
[capacity] waiting 1578s for run: not enough free memory (need 6834 MB; -683 MB spare above the 4096 MB floor, load 38.29; 3 running, 0 ahead)
[capacity] waiting 1584s for run: not enough free memory (need 6834 MB; -1557 MB spare above the 4096 MB floor, load 39.47; 3 running, 0 ahead)
[capacity] waiting 1589s for run: not enough free memory (need 6834 MB; 1062 MB spare above the 4096 MB floor, load 38.63; 2 running, 0 ahead)
[capacity] waiting 1594s for run: not enough free memory (need 6834 MB; 1566 MB spare above the 4096 MB floor, load 38.34; 1 running, 1 ahead)
[capacity] waiting 1600s for run: not enough free memory (need 6834 MB; 1433 MB spare above the 4096 MB floor, load 38.63; 1 running, 1 ahead)
[capacity] waiting 1606s for run: not enough free memory (need 6834 MB; 1852 MB spare above the 4096 MB floor, load 36.58; 1 running, 1 ahead)
[capacity] waiting 1612s for run: not enough free memory (need 6834 MB; 1264 MB spare above the 4096 MB floor, load 41.91; 1 running, 1 ahead)
[capacity] waiting 1617s for run: not enough free memory (need 6834 MB; 590 MB spare above the 4096 MB floor, load 41.44; 1 running, 1 ahead)
[capacity] waiting 1622s for run: not enough free memory (need 6834 MB; 161 MB spare above the 4096 MB floor, load 40.52; 1 running, 1 ahead)
[capacity] waiting 1628s for run: not enough free memory (need 6834 MB; 277 MB spare above the 4096 MB floor, load 39.44; 1 running, 1 ahead)
[capacity] waiting 1634s for run: not enough free memory (need 6834 MB; 311 MB spare above the 4096 MB floor, load 38.36; 1 running, 1 ahead)
[capacity] waiting 1640s for run: not enough free memory (need 6834 MB; 3104 MB spare above the 4096 MB floor, load 36.81; 1 running, 2 ahead)
[capacity] waiting 1645s for run: an earlier request is still waiting (need 6834 MB; 4472 MB spare above the 4096 MB floor, load 37.23; 1 running, 2 ahead)
[capacity] waiting 1650s for run: not enough free memory (need 6834 MB; 4080 MB spare above the 4096 MB floor, load 39.69; 1 running, 2 ahead)
Terminated: 15
[run-e2e 16:20:25] summary for course (@teaching/architecture-course) tool=--tool=bazel
  logs: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/e2e-course-e2e-1025f-yd1-course-*.log
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1025f-yd1 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
