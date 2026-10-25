# 🧭 e2e-orch report e2e-1025f-e1

Created 2026-10-25T21:52:24+01:00, written 2026-10-26T00:24:37+01:00, orchestrator pid 68142. Argv: `course --phase e2e --course-topology dev --go --skip-preflight --run-id e2e-1025f-e1`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: RUNNING 1. Cancelled: False.

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | RUNNING 0s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | RUNNING | None | - | - | - | - | - | - | 0 |  |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

no waiting lines recorded.

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `t150`
- capacity: 0 changed counter(s) between `start` and `t150`
- leases: 0 changed counter(s) between `start` and `t150`

Snapshots: start@2026-10-25T21:52:40+01:00, t30@2026-10-25T22:22:42+01:00, t60@2026-10-25T22:52:38+01:00, t90@2026-10-25T23:22:39+01:00, t120@2026-10-25T23:52:45+01:00, t150@2026-10-26T00:22:47+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1025f-e1`.

## Resources

Max load 35.0, min available memory 5.7 GB, min free disk 45.4 GB.

## Failures and warnings

none.

## Re-run

Continue this run: `run.sh all --run-id e2e-1025f-e1 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
