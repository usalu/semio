# 🧭 e2e-orch report e2e-1026g-after2

Created 2026-10-26T14:10:42+01:00, written 2026-10-26T15:04:34+01:00, orchestrator pid 59570. Argv: `course --phase e2e --go --skip-preflight --no-verify-targets --run-id e2e-1026g-after2 --course-topology dev`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: PASS 1. Cancelled: False.

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | PASS 52m41s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | PASS | 0 | 52m41s | 5 | 3 | 1 | 1 | s0 | 5 |  |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course test-e2e: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3169 MB; -1385 MB spare above the 4096 MB floor, load 10.63; 2 running, 1 ahead)`
- course test-e2e: `[capacity] waiting 5s for run: an earlier request is still waiting (need 3169 MB; -1369 MB spare above the 4096 MB floor, load 10.26; 2 running, 1 ahead)`
- course test-e2e: `[capacity] waiting 11s for run: an earlier request is still waiting (need 3169 MB; -2250 MB spare above the 4096 MB floor, load 10.72; 2 running, 1 ahead)`
- course test-e2e: `[capacity] waiting 16s for run: an earlier request is still waiting (need 3169 MB; -2916 MB spare above the 4096 MB floor, load 11.87; 2 running, 1 ahead)`
- course test-e2e: `[capacity] waiting 21s for run: an earlier request is still waiting (need 3169 MB; -3139 MB spare above the 4096 MB floor, load 12.12; 2 running, 1 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-26T14:10:49+01:00, t30@2026-10-26T14:41:17+01:00, end@2026-10-26T15:04:34+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1026g-after2`.

## Resources

Max load 45.9, min available memory 6.3 GB, min free disk 69.8 GB.

## Failures and warnings

none.

## Re-run

Continue this run: `run.sh all --run-id e2e-1026g-after2 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
