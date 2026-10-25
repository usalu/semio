# 🧭 e2e-orch report e2e-1023-course9

Created 2026-10-23T18:07:31+02:00, written 2026-10-23T20:08:20+02:00, orchestrator pid 83397. Argv: `course --go --run-id e2e-1023-course9 --no-verify-targets --phase cold,warm,e2e,probe`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1, SKIP-FAILED 12. Cancelled: False.

## Prerequisites

Verdict **OK** at 2026-10-23T18:07:31+02:00.

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
| OK | disk | 113 GB free (need 60) |
| OK | memory | 11.8 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 3.3 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 61 running (shared fleet; the governor and the slots arbitrate): 2447    02:23:04 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551 // 2448    02:23:04 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551 // 24347    01:57:17 bun repo --tool=bazel run @semio-tech/s-dev-composition:typecheck |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 12026 MB available of 32768 MB (floor 6553 MB, 3905 MB spare), pressure normal, swap 17402 MB, load 3.31, open files 7085, attached images 17; 11 running, 0 waiting |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | FAIL 2h00m01s | SKIP 0s | SKIP 0s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | FAIL | 1 | 2h00m01s | 0 | 0 | 0 | 0 | - | 5 | timeout after 2h00m00s |
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

- course proctor-build: `[script] repository:command · Waiting for access to bazel-slot-picker (exclusive): position 4 of 4, behind the request of process 51884 (exclusive, queued 5403 s ago)`
- course proctor-build: `[script] repository:command · Waiting for access to bazel-slot-picker (exclusive): position 4 of 4, behind the request of process 51884 (exclusive, queued 5418 s ago)`
- course proctor-build: `[script] repository:command · Waiting for access to bazel-slot-picker (exclusive): position 4 of 4, behind the request of process 51884 (exclusive, queued 5433 s ago)`
- course proctor-build: `[script] repository:command · Waiting for access to bazel-slot-picker (exclusive): position 4 of 4, behind the request of process 51884 (exclusive, queued 5448 s ago)`
- course proctor-build: `[script] repository:command · Waiting for access to bazel-slot-picker (exclusive): position 4 of 4, behind the request of process 51884 (exclusive, queued 5463 s ago)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-23T18:07:36+02:00, t30@2026-10-23T18:37:37+02:00, t60@2026-10-23T19:07:38+02:00, t90@2026-10-23T19:37:39+02:00, t120@2026-10-23T20:07:40+02:00, end@2026-10-23T20:08:20+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1023-course9`.

## Resources

Max load 19.7, min available memory 5.6 GB, min free disk 96.3 GB.

## Failures and warnings


### course cold proctor-build: FAIL

- command: `bun repo --tool=bazel run @teaching/proctor:build --configuration=release`
- note: timeout after 2h00m00s
- log: `🗑️generated/e2e-orch/e2e-1023-course9/course/cold-1-proctor-build.log`

```
[script] repository:command remains active on bazel-slot-picker (exclusive, 7038 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":2,\"queued\":5,\"holderMode\":\"exclusive\",\"holderPid\":83278,\"holderSeconds\":7206,\"r
[script] repository:command · Waiting for access to bazel-slot-picker (exclusive): position 2 of 5, behind the request of process 83278 (exclusive, queued 7206 s ago)
[repository:command] running elapsedMs=7161073
[repository:command] running elapsedMs=7171075
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"bazel-slot-picker","mode":"exclusive","elapsedMs":7053222}}
[script] repository:command remains active on bazel-slot-picker (exclusive, 7053 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":2,\"queued\":5,\"holderMode\":\"exclusive\",\"holderPid\":83278,\"holderSeconds\":7221,\"r
[script] repository:command · Waiting for access to bazel-slot-picker (exclusive): position 2 of 5, behind the request of process 83278 (exclusive, queued 7221 s ago)
[repository:command] running elapsedMs=7181076
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"bazel-slot-picker","mode":"exclusive","elapsedMs":7068337}}
[script] repository:command remains active on bazel-slot-picker (exclusive, 7068 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":2,\"queued\":5,\"holderMode\":\"exclusive\",\"holderPid\":83278,\"holderSeconds\":7236,\"r
[script] repository:command · Waiting for access to bazel-slot-picker (exclusive): position 2 of 5, behind the request of process 83278 (exclusive, queued 7236 s ago)
[repository:command] running elapsedMs=7191080
209 | export function requireIssuedScriptInvocationV1(invocation:ScriptInvocation):asserts invocation is IssuedScriptInvocationV1{birthOf(invocation);checkScriptInvocation(invocation);}
210 | /** 🎛️ Retains canonical cancellation and accepted asynchronous work through actual settlement. */
211 | export function withScriptProcessEnvelope<T>(envelope:ScriptProcessEnvelope,run:(invocation:IssuedScriptInvocationV1)=>Promise<T>):Promise<T>{
212 |  try{const admitted=readScriptProcessEnvelope(envelope,Date.now()),incoming=context.getStore();
213 |  if(incoming){const birth=admit(incoming);if(JSON.stringify(admitted)!==JSON.stringify(birth.envelope))throw Error("Nested Original process envelope must preserve its actual parent");return track(birth,()=>run(incoming as IssuedScript
214 |  const started=performance.now(),deadline=admitted.deadlineEpochMilliseconds,initial=deadline===null?null:deadline-Date.now(),controller=new AbortController(),endProcess=createScriptTerminationGuardV1({exit:code=>process.exit(code),no
error: Original Script process terminated
      at terminate (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:214:466)
Bun v1.4.2 (macOS arm64)
error: script "repo" exited with code 1
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1023-course9 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
