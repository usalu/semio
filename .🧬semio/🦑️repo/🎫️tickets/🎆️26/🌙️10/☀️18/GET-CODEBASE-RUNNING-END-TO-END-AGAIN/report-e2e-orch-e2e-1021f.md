# 🧭 e2e-orch report e2e-1021f

Created 2026-10-21T17:35:08+02:00, written 2026-10-21T20:06:25+02:00, orchestrator pid 47309. Argv: `course --go --run-id e2e-1021f`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1, SKIP-FAILED 12. Cancelled: False.

## Prerequisites

Verdict **WARN** at 2026-10-21T18:05:41+02:00.

| status | check | detail |
|---|---|---|
| OK | tool python3 | /usr/bin/python3 |
| OK | tool bun | /Users/ueli/.bun/bin/bun |
| OK | tool node | /opt/homebrew/bin/node |
| OK | tool git | /opt/homebrew/bin/git |
| WARN | bun version | 1.3.14 (packageManager 1.4.2) |
| OK | bazel binary | 45e9388abf21d110 (pinned 8.4.2) bazel 8.4.2 |
| OK | playwright chromium | revision 1248 installed |
| OK | playwright chromium-headless-shell | revision 1248 installed |
| OK | playwright ffmpeg | revision 1013 installed |
| OK | disk | 204 GB free (need 60) |
| OK | memory | 18.5 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| INFO | load | 25.9 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 10 running (shared fleet; the governor and the slots arbitrate): 2611       33:00 bazel(ws) --add-opens=java.base/java.lang=ALL-UNNAMED -Xverify:none -Djav // 24781       18:12 bazel(ws) --add-opens=java.base/java.lang=ALL-UNNAMED -Xverify:none -Dja // 35863    01:16:11 /bin/zsh .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-EN |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 18413 MB available of 32768 MB (floor 6553 MB, 2545 MB spare), pressure normal, swap 6173 MB, load 35.36, open files 6700, attached images 9; 3 running, 2 waiting |
| WARN | graph targets | could not read the published generation (exit 124: s is building the dependencies (852 s). Other commands keep running; waiting for that build instead of starting a second one. [DATA] Store dependency sync stale {"version":1,"purpose":"current-store-dependency-sync-stale","reason":"absent"}); target names not verified, the run itself will show a missing name |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | FAIL 2h00m01s | SKIP 0s | SKIP 0s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | FAIL | -15 | 2h00m01s | 0 | 0 | 0 | 0 | - | 5 | timeout after 2h00m00s |
| cold | typecheck | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | check | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | proctor-test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| warm | proctor-build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | typecheck | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | check | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | proctor-test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| e2e | test-e2e | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | group 'core' it depends on failed |

## Warm assertions (Bazel cache hits)

| product | step | spawns | allowed | internal | cache hits | time | expected within | verdict |
|---|---|---|---|---|---|---|---|---|
| course | proctor-build | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | typecheck | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | test | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | build | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | check | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | proctor-test | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |

## Queueing (slots, capacity)

- course proctor-build: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 419 MB spare above the 6553 MB floor, load 13.75; 6 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 6s for run: not enough free memory (need 3584 MB; 226 MB spare above the 6553 MB floor, load 13.37; 6 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 12s for run: not enough free memory (need 3584 MB; 850 MB spare above the 6553 MB floor, load 14.7; 6 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 17s for run: not enough free memory (need 3584 MB; 918 MB spare above the 6553 MB floor, load 14.65; 6 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 23s for run: not enough free memory (need 3584 MB; 824 MB spare above the 6553 MB floor, load 14.03; 6 running, 0 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-21T18:05:43+02:00, t30@2026-10-21T18:35:43+02:00, t60@2026-10-21T19:05:44+02:00, t90@2026-10-21T19:35:47+02:00, t120@2026-10-21T20:05:48+02:00, end@2026-10-21T20:06:25+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1021f`.

## Resources

Max load 18.2, min available memory 5.9 GB, min free disk 201.5 GB.

## Failures and warnings


### course cold proctor-build: FAIL

- command: `bun repo --tool=bazel run @teaching/proctor:build --configuration=release`
- note: timeout after 2h00m00s
- log: `🗑️generated/e2e-orch/e2e-1021f/course/cold-1-proctor-build.log`

```
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"repository-graph-facts:/Users/ueli/Documents/semio:shard:7","mode":"exclusive","elapsedMs":5704588}}
[script] repository:command remains active on repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive, 5705 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":2,\"queued\":2,\"holderMode\":\"exclusive\",\"holderPid\":49472,\"holderSeconds\":5926,\"r
[script] repository:command · Waiting for access to repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive): position 2 of 2, behind the request of process 49472 (exclusive, queued 5926 s ago)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"repository-graph-facts:/Users/ueli/Documents/semio:shard:7","mode":"exclusive","elapsedMs":5719662}}
[script] repository:command remains active on repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive, 5720 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":2,\"queued\":2,\"holderMode\":\"exclusive\",\"holderPid\":49472,\"holderSeconds\":5941,\"r
[script] repository:command · Waiting for access to repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive): position 2 of 2, behind the request of process 49472 (exclusive, queued 5941 s ago)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"repository-graph-facts:/Users/ueli/Documents/semio:shard:7","mode":"exclusive","elapsedMs":5734738}}
[script] repository:command remains active on repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive, 5735 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":2,\"queued\":2,\"holderMode\":\"exclusive\",\"holderPid\":49472,\"holderSeconds\":5956,\"r
[script] repository:command · Waiting for access to repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive): position 2 of 2, behind the request of process 49472 (exclusive, queued 5956 s ago)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"repository-graph-facts:/Users/ueli/Documents/semio:shard:7","mode":"exclusive","elapsedMs":5749814}}
[script] repository:command remains active on repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive, 5750 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":2,\"queued\":2,\"holderMode\":\"exclusive\",\"holderPid\":49472,\"holderSeconds\":5971,\"r
[script] repository:command · Waiting for access to repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive): position 2 of 2, behind the request of process 49472 (exclusive, queued 5971 s ago)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"repository-graph-facts:/Users/ueli/Documents/semio:shard:7","mode":"exclusive","elapsedMs":5764914}}
[script] repository:command remains active on repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive, 5765 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":2,\"queued\":2,\"holderMode\":\"exclusive\",\"holderPid\":49472,\"holderSeconds\":5986,\"r
[script] repository:command · Waiting for access to repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive): position 2 of 2, behind the request of process 49472 (exclusive, queued 5986 s ago)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"repository-graph-facts:/Users/ueli/Documents/semio:shard:7","mode":"exclusive","elapsedMs":5780017}}
[script] repository:command remains active on repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive, 5780 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":2,\"queued\":2,\"holderMode\":\"exclusive\",\"holderPid\":49472,\"holderSeconds\":6001,\"r
[script] repository:command · Waiting for access to repository-graph-facts:/Users/ueli/Documents/semio:shard:7 (exclusive): position 2 of 2, behind the request of process 49472 (exclusive, queued 6001 s ago)
error: script "repo" was terminated by signal SIGTERM (Polite quit request)
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1021f --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
