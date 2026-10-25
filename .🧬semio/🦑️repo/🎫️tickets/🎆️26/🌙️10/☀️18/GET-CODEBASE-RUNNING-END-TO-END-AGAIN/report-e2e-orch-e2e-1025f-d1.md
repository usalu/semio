# 🧭 e2e-orch report e2e-1025f-d1

Created 2026-10-25T19:54:16+01:00, written 2026-10-25T21:52:16+01:00, orchestrator pid 81806. Argv: `course --phase e2e --course-topology dev --go --skip-preflight --run-id e2e-1025f-d1`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: CANCELLED 1. Cancelled: True (SIGTERM).

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | FAIL 1h56m56s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | CANCELLED | 143 | 1h56m56s | 0 | 0 | 0 | 0 | - | 5 | cancelled: SIGTERM |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course test-e2e: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; -985 MB spare above the 6553 MB floor, load 15.59; 4 running, 6 ahead)`
- course test-e2e: `[capacity] waiting 6s for run: an earlier request is still waiting (need 3584 MB; -2215 MB spare above the 6553 MB floor, load 15.54; 4 running, 6 ahead)`
- course test-e2e: `[capacity] waiting 11s for run: an earlier request is still waiting (need 3584 MB; -1924 MB spare above the 6553 MB floor, load 15.5; 4 running, 6 ahead)`
- course test-e2e: `[capacity] waiting 16s for run: an earlier request is still waiting (need 3584 MB; -2148 MB spare above the 6553 MB floor, load 14.98; 4 running, 6 ahead)`
- course test-e2e: `[capacity] waiting 23s for run: an earlier request is still waiting (need 3584 MB; -1451 MB spare above the 6553 MB floor, load 15.14; 4 running, 6 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-25T19:54:27+01:00, t30@2026-10-25T20:24:26+01:00, t60@2026-10-25T20:55:16+01:00, t90@2026-10-25T21:24:54+01:00, end@2026-10-25T21:52:16+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1025f-d1`.

## Resources

Max load 52.1, min available memory 5.6 GB, min free disk 45.3 GB.

## Failures and warnings


### course e2e test-e2e: CANCELLED

- command: `bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh course --tool=bazel --topology=dev`
- note: cancelled: SIGTERM
- log: `🗑️generated/e2e-orch/e2e-1025f-d1/course/e2e-1-test-e2e.log`

```
[script] repository:command remains active on repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive, 2565 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":3,\"queued\":9,\"holderMode\":\"exclusive\",\"holderPid\":64323,\"holderSeconds\":2565,\"r
[script] repository:command · Waiting for access to repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive): position 3 of 9, behind the request of process 64323 (exclusive, queued 2565 s ago)
[repository:command] running elapsedMs=5320827
[repository:command] running elapsedMs=5310808
[script] {"owner":"repository:build","command":"build:graph-plan","stage":"running"}
[repository:command] running elapsedMs=5330829
[repository:command] running elapsedMs=5320811
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"repository-graph-facts:/Users/ueli/Documents/semio:shard:7","mode":"exclusive","elapsedMs":2580098}}
[script] repository:command remains active on repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive, 2580 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":3,\"queued\":9,\"holderMode\":\"exclusive\",\"holderPid\":64323,\"holderSeconds\":2580,\"r
[script] repository:command · Waiting for access to repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive): position 3 of 9, behind the request of process 64323 (exclusive, queued 2580 s ago)
[script] {"owner":"repository:build","command":"build:graph-plan","stage":"running"}
[repository:command] running elapsedMs=5340834
[repository:command] running elapsedMs=5330813
[script] {"owner":"repository:build","command":"build:graph-plan","stage":"running"}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"repository-graph-facts:/Users/ueli/Documents/semio:shard:7","mode":"exclusive","elapsedMs":2595239}}
[script] repository:command remains active on repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive, 2595 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":3,\"queued\":9,\"holderMode\":\"exclusive\",\"holderPid\":64323,\"holderSeconds\":2595,\"r
[script] repository:command · Waiting for access to repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive): position 3 of 9, behind the request of process 64323 (exclusive, queued 2595 s ago)
[repository:command] running elapsedMs=5350837
[repository:command] running elapsedMs=5340816
Terminated: 15
[run-e2e 21:51:51] summary for course (@teaching/architecture-course) tool=--tool=bazel
  logs: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/e2e-course-e2e-1025f-d1-course-*.log
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1025f-d1 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
