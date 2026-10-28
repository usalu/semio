# 🧭 e2e-orch report e2e-1025f-r1

Created 2026-10-26T06:34:20+01:00, written 2026-10-26T07:02:05+01:00, orchestrator pid 39996. Argv: `course --phase e2e --course-topology rehearsal --go --skip-preflight --run-id e2e-1025f-r1`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: PASS 1. Cancelled: False.

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | PASS 26m53s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | PASS | 0 | 26m53s | 5 | 4 | 0 | 1 | s1 | 4 |  |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course test-e2e: `[script] repository:command · Waiting for access to native-storage-mapping:cffa6f632709e3bdeca7766b4a4b265dec0de0582d52ebbaa3d1ed868cc74b8f (shared): position 2 of 2, behind the request of process 409`
- course test-e2e: `[script] repository:command · Waiting for access to native-storage-mapping:cffa6f632709e3bdeca7766b4a4b265dec0de0582d52ebbaa3d1ed868cc74b8f (shared): position 2 of 2, behind the request of process 411`
- course test-e2e: `[script] repository:command · Waiting for access to native-storage-mapping:cffa6f632709e3bdeca7766b4a4b265dec0de0582d52ebbaa3d1ed868cc74b8f (shared): position 2 of 2, behind the request of process 413`
- course test-e2e: `[script] bazel:task-admission · Waiting for access to repository-cache-charge:/Users/ueli/Documents/semio/.cache/b/g/m (exclusive): position 2 of 2, behind the request of process 42494 (exclusive, que`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-26T06:34:30+01:00, end@2026-10-26T07:02:05+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1025f-r1`.

## Resources

Max load 16.4, min available memory 6.9 GB, min free disk 64.2 GB.

## Failures and warnings

none.

## Re-run

Continue this run: `run.sh all --run-id e2e-1025f-r1 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
