# 🧭 e2e-orch report e2e-1026g-after5b

Created 2026-10-26T17:51:06+01:00, written 2026-10-26T18:11:47+01:00, orchestrator pid 51823. Argv: `course --phase e2e --go --skip-preflight --no-verify-targets --run-id e2e-1026g-after5b --course-topology dev`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: CANCELLED 1. Cancelled: True (SIGTERM).

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | FAIL 18m56s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | CANCELLED | 143 | 18m56s | 0 | 0 | 0 | 0 | s2 | 1 | cancelled: SIGTERM |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course test-e2e: `[script] bazel:task-admission · Waiting for access to repository-cache-charge:/Users/ueli/Documents/semio/.cache/b/g/m (exclusive): position 2 of 2, behind the request of process 95447 (exclusive, que`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-26T17:52:26+01:00, end@2026-10-26T18:11:47+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1026g-after5b`.

## Resources

Max load 68.6, min available memory 8.3 GB, min free disk 132.3 GB.

## Failures and warnings


### course e2e test-e2e: CANCELLED

- command: `bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh course --tool=bazel --topology=dev`
- note: cancelled: SIGTERM
- log: `🗑️generated/e2e-orch/e2e-1026g-after5b/course/e2e-1-test-e2e.log`

```
[bazel:build] running elapsedMs=280102
[repository:command] running elapsedMs=1030722
[bazel:build] running elapsedMs=290104
[3 / 5] @teaching/proctor:build:prototype; 282s disk-cache, worker
[repository:command] running elapsedMs=1040729
[bazel:build] running elapsedMs=300105
[repository:command] running elapsedMs=1050733
[bazel:build] running elapsedMs=310106
[repository:command] running elapsedMs=1060740
[bazel:build] running elapsedMs=320107
[repository:command] running elapsedMs=1070744
[bazel:build] running elapsedMs=330108
[repository:command] running elapsedMs=1080748
[bazel:build] running elapsedMs=340110
[repository:command] running elapsedMs=1090750
[bazel:build] running elapsedMs=350113
[repository:command] running elapsedMs=1100753
[3 / 5] @teaching/proctor:build:prototype; 343s disk-cache, worker
[bazel:build] running elapsedMs=360118
[repository:command] running elapsedMs=1110754
[bazel:build] running elapsedMs=370119
[repository:command] running elapsedMs=1120766
Terminated: 15
[run-e2e 18:10:43] summary for course (@teaching/architecture-course) tool=--tool=bazel
  logs: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/e2e-course-e2e-1026g-after5b-course-*.log
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1026g-after5b --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
