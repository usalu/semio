# 🧭 e2e-orch report e2e-1023-course8

Created 2026-10-23T13:58:01+02:00, written 2026-10-23T18:06:35+02:00, orchestrator pid 17037. Argv: `course --go --run-id e2e-1023-course8 --no-verify-targets --phase cold`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1, PASS 1, SKIP-FAILED 4. Cancelled: False.

## Prerequisites

Verdict **OK** at 2026-10-23T13:58:01+02:00.

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
| OK | disk | 122 GB free (need 60) |
| OK | memory | 17.2 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 13.9 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 39 running (shared fleet; the governor and the slots arbitrate): 2258    01:44:33 bun repo cargo --store=shared --configuration=prototype -- check --keep-g // 2263    01:44:33 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551 // 5586       21:23 /Users/ueli/Documents/semio/.cache/b/t/m/bazel/install/8.4.2/darwin-arm64 |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 17502 MB available of 32768 MB (floor 6553 MB, -2162 MB spare), pressure normal, swap 7935 MB, load 13.9, open files 5278, attached images 17; 5 running, 6 waiting |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | FAIL 4h07m49s | - | - | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | PASS | 0 | 1h07m48s | 2 | 1 | 0 | 1 | s1 | 5 |  |
| cold | typecheck | FAIL | 1 | 3h00m01s | 0 | 0 | 0 | 0 | - | 5 | timeout after 3h00m00s |
| cold | test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | check | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | proctor-test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course proctor-build: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; 2131 MB spare above the 6553 MB floor, load 9.33; 5 running, 6 ahead)`
- course proctor-build: `[capacity] waiting 6s for run: an earlier request is still waiting (need 3584 MB; 2150 MB spare above the 6553 MB floor, load 9.14; 5 running, 6 ahead)`
- course proctor-build: `[capacity] waiting 11s for run: an earlier request is still waiting (need 3584 MB; 2156 MB spare above the 6553 MB floor, load 9.21; 5 running, 6 ahead)`
- course proctor-build: `[capacity] waiting 17s for run: an earlier request is still waiting (need 3584 MB; 1977 MB spare above the 6553 MB floor, load 8.47; 5 running, 6 ahead)`
- course proctor-build: `[capacity] waiting 22s for run: an earlier request is still waiting (need 3584 MB; 1986 MB spare above the 6553 MB floor, load 8.11; 5 running, 5 ahead)`
- course typecheck: `[capacity] waiting 0s for run: an earlier request is still waiting (need 5632 MB; -1572 MB spare above the 6553 MB floor, load 16.1; 8 running, 5 ahead)`
- course typecheck: `[capacity] waiting 6s for run: the host is swapping out too fast (need 5632 MB; -201 MB spare above the 6553 MB floor, load 16.81; 8 running, 5 ahead)`
- course typecheck: `[capacity] waiting 11s for run: an earlier request is still waiting (need 5632 MB; -318 MB spare above the 6553 MB floor, load 16.66; 8 running, 5 ahead)`
- course typecheck: `[capacity] waiting 17s for run: an earlier request is still waiting (need 5632 MB; 672 MB spare above the 6553 MB floor, load 19.8; 8 running, 5 ahead)`
- course typecheck: `[capacity] waiting 22s for run: an earlier request is still waiting (need 5632 MB; 585 MB spare above the 6553 MB floor, load 18.93; 8 running, 5 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-23T13:58:07+02:00, t30@2026-10-23T14:28:08+02:00, t60@2026-10-23T14:58:07+02:00, t90@2026-10-23T15:28:08+02:00, t120@2026-10-23T15:58:09+02:00, t150@2026-10-23T16:28:10+02:00, t180@2026-10-23T16:58:11+02:00, t210@2026-10-23T17:28:10+02:00, t240@2026-10-23T17:58:12+02:00, end@2026-10-23T18:06:35+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1023-course8`.

## Resources

Max load 21.6, min available memory 5.7 GB, min free disk 99.7 GB.

## Failures and warnings


### course cold typecheck: FAIL

- command: `bun repo --tool=bazel run @teaching/architecture-course:typecheck`
- note: timeout after 3h00m00s
- log: `🗑️generated/e2e-orch/e2e-1023-course8/course/cold-2-typecheck.log`

```
[script] repository:command remains active on bazel-slot-picker (exclusive, 5969 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":2,\"queued\":4,\"holderMode\":\"exclusive\",\"holderPid\":85285,\"holderSeconds\":10131,\"
[script] repository:command · Waiting for access to bazel-slot-picker (exclusive): position 2 of 4, behind the request of process 85285 (exclusive, queued 10131 s ago)
[repository:command] running elapsedMs=6100991
[repository:command] running elapsedMs=6110992
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"bazel-slot-picker","mode":"exclusive","elapsedMs":5983606}}
[script] repository:command remains active on bazel-slot-picker (exclusive, 5984 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":2,\"queued\":4,\"holderMode\":\"exclusive\",\"holderPid\":85285,\"holderSeconds\":10146,\"
[script] repository:command · Waiting for access to bazel-slot-picker (exclusive): position 2 of 4, behind the request of process 85285 (exclusive, queued 10146 s ago)
[repository:command] running elapsedMs=6120993
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"bazel-slot-picker","mode":"exclusive","elapsedMs":5998674}}
[script] repository:command remains active on bazel-slot-picker (exclusive, 5999 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":2,\"queued\":4,\"holderMode\":\"exclusive\",\"holderPid\":85285,\"holderSeconds\":10161,\"
[script] repository:command · Waiting for access to bazel-slot-picker (exclusive): position 2 of 4, behind the request of process 85285 (exclusive, queued 10161 s ago)
[repository:command] running elapsedMs=6130994
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

Continue this run: `run.sh all --run-id e2e-1023-course8 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
