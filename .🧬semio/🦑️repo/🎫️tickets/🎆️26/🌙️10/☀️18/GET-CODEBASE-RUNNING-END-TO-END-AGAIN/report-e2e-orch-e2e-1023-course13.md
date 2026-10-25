# 🧭 e2e-orch report e2e-1023-course13

Created 2026-10-23T23:43:54+02:00, written 2026-10-24T01:14:25+02:00, orchestrator pid 67768. Argv: `course --go --run-id e2e-1023-course13 --no-verify-targets --phase cold,warm,e2e,probe`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1, PASS 1, SKIP-FAILED 11. Cancelled: False.

## Prerequisites

Verdict **OK** at 2026-10-23T23:43:54+02:00.

| status | check | detail |
|---|---|---|
| OK | tool python3 | /usr/bin/python3 |
| OK | tool bun | /Users/ueli/.bun/bin/bun |
| OK | tool node | /opt/homebrew/bin/node |
| OK | tool git | /opt/homebrew/bin/git |
| OK | pinned bun | 1.4.2 provisioned: /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun |
| OK | bun runs pinned | bun on PATH is 1.3.14 (/Users/ueli/.bun/bin/bun) and runs the checkout as 1.4.2 (pinned 1.4.2) |
| OK | bazel binary | 45e9388abf21d110 (pinned 8.4.2) bazel 8.4.2 |
| OK | playwright chromium | revision 1248 installed |
| OK | playwright chromium-headless-shell | revision 1248 installed |
| OK | playwright ffmpeg | revision 1013 installed |
| OK | disk | 123 GB free (need 60) |
| OK | memory | 11.6 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 14.0 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 54 running (shared fleet; the governor and the slots arbitrate): 3445       30:01 bun repo cargo --store=shared --configuration=prototype -- check --manife // 3446       30:01 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551 // 5892       29:08 bun repo cargo --store=shared --configuration=prototype -- test --no-fail |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 12124 MB available of 32768 MB (floor 6553 MB, -2386 MB spare), pressure normal, swap 8329 MB, load 13.03, open files 5670, attached images 19; 5 running, 12 waiting |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | FAIL 1h29m37s | SKIP 0s | SKIP 0s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | PASS | 0 | 51m59s | 2 | 1 | 0 | 1 | s2 | 5 |  |
| cold | typecheck | FAIL | 1 | 37m38s | 7 | 5 | 0 | 2 | s2 | 5 | exit code 1 |
| cold | test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | check | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | proctor-test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| warm | proctor-build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| warm | typecheck | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | check | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | proctor-test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| e2e | test-e2e | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | group 'core' it depends on failed |

## Warm assertions (Bazel cache hits)

| product | step | spawns | allowed | internal | cache hits | time | expected within | verdict |
|---|---|---|---|---|---|---|---|---|
| course | proctor-build | - | 0 | - | - | 0s | - | SKIP-FAILED an earlier step of group 'core' failed |
| course | typecheck | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | test | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | build | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | check | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | proctor-test | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |

## Queueing (slots, capacity)

- course proctor-build: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; -1316 MB spare above the 6553 MB floor, load 9.71; 5 running, 13 ahead)`
- course proctor-build: `[capacity] waiting 5s for run: an earlier request is still waiting (need 3584 MB; 193 MB spare above the 6553 MB floor, load 9.42; 5 running, 13 ahead)`
- course proctor-build: `[capacity] waiting 10s for run: an earlier request is still waiting (need 3584 MB; 391 MB spare above the 6553 MB floor, load 9.14; 6 running, 12 ahead)`
- course proctor-build: `[capacity] waiting 16s for run: an earlier request is still waiting (need 3584 MB; -1274 MB spare above the 6553 MB floor, load 9.29; 6 running, 12 ahead)`
- course proctor-build: `[capacity] waiting 21s for run: an earlier request is still waiting (need 3584 MB; -1232 MB spare above the 6553 MB floor, load 9.27; 6 running, 12 ahead)`
- course typecheck: `[capacity] waiting 0s for run: an earlier request is still waiting (need 5632 MB; 2733 MB spare above the 6553 MB floor, load 16.71; 6 running, 10 ahead)`
- course typecheck: `[capacity] waiting 6s for run: an earlier request is still waiting (need 5632 MB; -938 MB spare above the 6553 MB floor, load 16.01; 7 running, 9 ahead)`
- course typecheck: `[capacity] waiting 11s for run: an earlier request is still waiting (need 5632 MB; -1452 MB spare above the 6553 MB floor, load 16.01; 7 running, 9 ahead)`
- course typecheck: `[capacity] waiting 16s for run: an earlier request is still waiting (need 5632 MB; -3035 MB spare above the 6553 MB floor, load 15.97; 7 running, 9 ahead)`
- course typecheck: `[capacity] waiting 22s for run: an earlier request is still waiting (need 5632 MB; -959 MB spare above the 6553 MB floor, load 16.13; 7 running, 9 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-23T23:44:01+02:00, t30@2026-10-24T00:14:09+02:00, t60@2026-10-24T00:44:11+02:00, t90@2026-10-24T01:14:15+02:00, end@2026-10-24T01:14:25+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1023-course13`.

## Resources

Max load 36.1, min available memory 5.7 GB, min free disk 104.4 GB.

## Failures and warnings


### course cold typecheck: FAIL

- command: `bun repo --tool=bazel run @teaching/architecture-course:typecheck`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1023-course13/course/cold-2-typecheck.log`

```
                  ^
OwnedCommandFailure: bazel:build failed (1)
 outcome: {
  version: 1,
  status: 1,
  signal: null,
},
      at runOwnedCommand (semio-source:/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:324:13)
Bun v1.4.2 (macOS arm64)
167 |     if(auxiliary&&commandSignal?.aborted)throw commandSignal.reason;
168 |     if(auxiliaryFailed)throw auxiliaryFailure;
169 |     if(auxiliary)requireIssuedResourceControlV1(options.control!);
170 |     if (stopped&&stopped!=="output delivery refused") throw new Error(`${label} stopped: ${stopped}`);
171 |     if(stopped==="output delivery refused"&&status.signal!==null)throw deliveryFailure;
172 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
                                                                 ^
OwnedCommandFailure: repository:command failed (1)
 outcome: {
  version: 1,
  status: 1,
  signal: null,
},
      at runOwnedCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:172:60)
Bun v1.4.2 (macOS arm64)
error: script "repo" exited with code 1
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1023-course13 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
