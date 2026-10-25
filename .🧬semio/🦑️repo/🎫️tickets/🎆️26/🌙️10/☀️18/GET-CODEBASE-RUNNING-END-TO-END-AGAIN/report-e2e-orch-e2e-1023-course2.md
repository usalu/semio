# 🧭 e2e-orch report e2e-1023-course2

Created 2026-10-23T00:56:51+02:00, written 2026-10-23T02:48:17+02:00, orchestrator pid 32733. Argv: `course --go --run-id e2e-1023-course2 --no-verify-targets`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1, PASS 1, SKIP-FAILED 11. Cancelled: False.

## Prerequisites

Verdict **WARN** at 2026-10-23T00:56:51+02:00.

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
| OK | disk | 107 GB free (need 60) |
| OK | memory | 12.8 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| INFO | load | 21.8 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 28 running (shared fleet; the governor and the slots arbitrate): 28728       02:48 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 // 28964       02:35 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 // 29852       01:54 /Users/ueli/Documents/semio/.cache/b/t/m/r/toolchains/nightly-2026-07-20 |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 13036 MB available of 32768 MB (floor 6553 MB, -2189 MB spare), pressure normal, swap 7608 MB, load 21.77, open files 7397, attached images 11; 10 running, 1 waiting |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | FAIL 1h50m37s | SKIP 0s | SKIP 0s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | PASS | 0 | 50m36s | 2 | 1 | 0 | 1 | s0 | 5 |  |
| cold | typecheck | FAIL | 1 | 1h00m01s | 0 | 0 | 0 | 0 | s0 | 5 | timeout after 1h00m00s |
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

- course proctor-build: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; 1643 MB spare above the 6553 MB floor, load 20.61; 9 running, 1 ahead)`
- course proctor-build: `[capacity] waiting 5s for run: an earlier request is still waiting (need 3584 MB; 1805 MB spare above the 6553 MB floor, load 20.72; 9 running, 1 ahead)`
- course proctor-build: `[capacity] waiting 11s for run: an earlier request is still waiting (need 3584 MB; 1050 MB spare above the 6553 MB floor, load 21.7; 9 running, 1 ahead)`
- course proctor-build: `[capacity] waiting 16s for run: an earlier request is still waiting (need 3584 MB; 1096 MB spare above the 6553 MB floor, load 22.21; 9 running, 1 ahead)`
- course proctor-build: `[capacity] waiting 22s for run: an earlier request is still waiting (need 3584 MB; 1033 MB spare above the 6553 MB floor, load 21.95; 9 running, 1 ahead)`
- course typecheck: `[capacity] waiting 0s for run: an earlier request is still waiting (need 5632 MB; 5811 MB spare above the 6553 MB floor, load 21.12; 5 running, 3 ahead)`
- course typecheck: `[capacity] waiting 5s for run: an earlier request is still waiting (need 5632 MB; 341 MB spare above the 6553 MB floor, load 19.82; 6 running, 2 ahead)`
- course typecheck: `[capacity] waiting 11s for run: an earlier request is still waiting (need 5632 MB; 395 MB spare above the 6553 MB floor, load 18.8; 5 running, 2 ahead)`
- course typecheck: `[capacity] waiting 16s for run: an earlier request is still waiting (need 5632 MB; -1210 MB spare above the 6553 MB floor, load 17.69; 5 running, 2 ahead)`
- course typecheck: `[capacity] waiting 21s for run: an earlier request is still waiting (need 5632 MB; 1858 MB spare above the 6553 MB floor, load 16.76; 5 running, 2 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-23T00:57:01+02:00, t30@2026-10-23T01:27:11+02:00, t60@2026-10-23T01:57:01+02:00, t90@2026-10-23T02:27:03+02:00, end@2026-10-23T02:48:17+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1023-course2`.

## Resources

Max load 33.6, min available memory 5.7 GB, min free disk 102.3 GB.

## Failures and warnings


### course cold typecheck: FAIL

- command: `bun repo --tool=bazel run @teaching/architecture-course:typecheck`
- note: timeout after 1h00m00s
- log: `🗑️generated/e2e-orch/e2e-1023-course2/course/cold-2-typecheck.log`

```
[bazel:build] running elapsedMs=1490252
[repository:command] running elapsedMs=3330819
[bazel:build] running elapsedMs=1500252
[2 / 8] @semio-tech/ui-contract-rs:generate:prototype; 1491s disk-cache, worker
[repository:command] running elapsedMs=3340821
[bazel:build] running elapsedMs=1510253
[repository:command] running elapsedMs=3350824
[bazel:build] running elapsedMs=1520255
[repository:command] running elapsedMs=3360827
[bazel:build] running elapsedMs=1530256
[repository:command] running elapsedMs=3370830
[bazel:build] running elapsedMs=1540257
[repository:command] running elapsedMs=3380832
[bazel:build] running elapsedMs=1550260
208 | }
209 | /** 🔐️ Refuses structural copies and retired process scopes. */
210 | export function requireIssuedScriptInvocationV1(invocation:ScriptInvocation):asserts invocation is IssuedScriptInvocationV1{birthOf(invocation);checkScriptInvocation(invocation);}
211 | /** 🎛️ Retains canonical cancellation and accepted asynchronous work through actual settlement. */
212 | export function withScriptProcessEnvelope<T>(envelope:ScriptProcessEnvelope,run:(invocation:IssuedScriptInvocationV1)=>Promise<T>):Promise<T>{
213 |  try{const admitted=readScriptProcessEnvelope(envelope,Date.now()),incoming=context.getStore();
                                                                                                                                                                                                                                                
error: Original Script process terminated
      at terminate (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:213:268)
Bun v1.4.2 (macOS arm64)
error: script "repo" exited with code 1
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1023-course2 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
