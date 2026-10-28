# 🧭 e2e-orch report e2e-1026g-after5

Created 2026-10-26T17:40:58+01:00, written 2026-10-26T17:45:22+01:00, orchestrator pid 38213. Argv: `course --phase e2e --go --skip-preflight --no-verify-targets --run-id e2e-1026g-after5 --course-topology dev`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1, RUNNING 1. Cancelled: False.

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
| pipeline | crash | FAIL | 1 | 0s | - | - | - | - | - | 0 | FileNotFoundError(2, 'No such file or directory') |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

no waiting lines recorded.

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-26T17:41:15+01:00, end@2026-10-26T17:45:22+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1026g-after5`.

## Resources

Max load 27.7, min available memory 11.4 GB, min free disk 75.2 GB.

## Failures and warnings


### course pipeline crash: FAIL

- command: ``
- note: FileNotFoundError(2, 'No such file or directory')
- log: ``

## Re-run

Continue this run: `run.sh all --run-id e2e-1026g-after5 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
