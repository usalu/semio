# 🧭 e2e-orch report e2e-1025f-zd6

Created 2026-10-26T08:55:08+01:00, written 2026-10-26T08:57:24+01:00, orchestrator pid 74315. Argv: `course --phase e2e --course-topology dev --go --skip-preflight --run-id e2e-1025f-zd6`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: CANCELLED 1. Cancelled: True (SIGTERM).

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | FAIL 1m03s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | CANCELLED | 143 | 1m03s | 0 | 0 | 0 | 0 | - | 5 | cancelled: SIGTERM |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course test-e2e: `[capacity] waiting 0s for run: not enough free memory (need 3311 MB; 136 MB spare above the 4096 MB floor, load 21.58; 7 running, 0 ahead)`
- course test-e2e: `[capacity] waiting 6s for run: not enough free memory (need 3311 MB; 164 MB spare above the 4096 MB floor, load 20.73; 7 running, 0 ahead)`
- course test-e2e: `[capacity] waiting 12s for run: not enough free memory (need 3311 MB; 1304 MB spare above the 4096 MB floor, load 20.43; 7 running, 0 ahead)`
- course test-e2e: `[capacity] waiting 17s for run: not enough free memory (need 3311 MB; -13 MB spare above the 4096 MB floor, load 21.6; 7 running, 0 ahead)`
- course test-e2e: `[capacity] waiting 22s for run: not enough free memory (need 3311 MB; -373 MB spare above the 4096 MB floor, load 20.59; 7 running, 0 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-26T08:55:32+01:00, end@2026-10-26T08:57:24+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1025f-zd6`.

## Resources

Max load 29.3, min available memory 12.1 GB, min free disk 68.5 GB.

## Failures and warnings


### course e2e test-e2e: CANCELLED

- command: `bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh course --tool=bazel --topology=dev`
- note: cancelled: SIGTERM
- log: `🗑️generated/e2e-orch/e2e-1025f-zd6/course/e2e-1-test-e2e.log`

```
[script] repository:command · Verifying graph facts: 61440 of 113567 facts
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:facts:65536:113567","mode":"shared","elapsedMs":0}}
[script] repository:command · Verifying graph facts: 65536 of 113567 facts
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:facts:69632:113567","mode":"shared","elapsedMs":0}}
[script] repository:command · Verifying graph facts: 69632 of 113567 facts
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:facts:73728:113567","mode":"shared","elapsedMs":0}}
[script] repository:command · Verifying graph facts: 73728 of 113567 facts
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:facts:77824:113567","mode":"shared","elapsedMs":0}}
[script] repository:command · Verifying graph facts: 77824 of 113567 facts
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:facts:81920:113567","mode":"shared","elapsedMs":0}}
[script] repository:command · Verifying graph facts: 81920 of 113567 facts
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:facts:86016:113567","mode":"shared","elapsedMs":0}}
[script] repository:command · Verifying graph facts: 86016 of 113567 facts
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:facts:106496:113567","mode":"shared","elapsedMs":0}}
[script] repository:command · Verifying graph facts: 106496 of 113567 facts
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:facts:113567:113567","mode":"shared","elapsedMs":0}}
[script] repository:command · Verifying graph facts: 113567 of 113567 facts
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"repository-graph-facts:/Users/ueli/Documents/semio:shard:7","mode":"shared","elapsedMs":6}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":7,\"queued\":7,\"holderMode\":\"shared\",\"holderPid\":56449,\"holderSeconds\":189,\"resou
[script] repository:command · Waiting for access to repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (shared): position 7 of 7, behind the request of process 56449 (shared, queued 189 s ago)
[script] {"owner":"repository:build","command":"build:graph-plan","stage":"running"}
[repository:command] running elapsedMs=30005
Terminated: 15
[run-e2e 08:56:50] summary for course (@teaching/architecture-course) tool=--tool=bazel
  logs: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/e2e-course-e2e-1025f-zd6-course-*.log
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1025f-zd6 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
