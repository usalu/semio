# 🧭 e2e-orch report e2e-1025f-h1

Created 2026-10-26T05:55:57+01:00, written 2026-10-26T06:33:59+01:00, orchestrator pid 19413. Argv: `course --phase e2e --course-topology dev --go --skip-preflight --run-id e2e-1025f-h1`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: PASS 1. Cancelled: False.

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | PASS 37m12s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | PASS | 0 | 37m12s | 4 | 2 | 2 | 1 | s0 | 0 |  |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

no waiting lines recorded.

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-26T05:56:03+01:00, t30@2026-10-26T06:26:05+01:00, end@2026-10-26T06:33:59+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1025f-h1`.

## Resources

Max load 11.3, min available memory 7.6 GB, min free disk 63.7 GB.

## Failures and warnings

none.

## Re-run

Continue this run: `run.sh all --run-id e2e-1025f-h1 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
