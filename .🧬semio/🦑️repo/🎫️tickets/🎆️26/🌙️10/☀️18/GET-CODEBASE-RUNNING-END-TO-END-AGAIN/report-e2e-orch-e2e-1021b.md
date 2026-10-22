# 🧭 e2e-orch report e2e-1021b

Created 2026-10-21T12:45:10+02:00, written 2026-10-21T14:42:09+02:00, orchestrator pid 83460. Argv: `all --go --run-id e2e-1021b`.

Host: Darwin 27.0.0, 10 cores, parallel 3, Bazel tool `bazel`. Counts: CANCELLED 3. Cancelled: True (SIGTERM).

## Prerequisites

Verdict **WARN** at 2026-10-21T12:47:56+02:00.

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
| OK | tectonic binary | aarch64-apple-darwin 0.16.9 sha256 matches the recorded extraction |
| OK | tex bundle | 2 populated bundle directories |
| OK | disk | 213 GB free (need 60) |
| OK | memory | 18.0 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| INFO | load | 21.7 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 3 product(s) |
| OK | ports play | [6033] free |
| OK | applied: play-bzl E1: activate-prototype cached | present |
| OK | applied: pg-chain: component cache test | present |
| OK | ports demo | [6029] free |
| OK | applied: demo-bzl: activate-prototype cached | present |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 14 running (shared fleet; the governor and the slots arbitrate): 10990 02:43:38 /bin/zsh .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-T // 77324    06:17 /bin/zsh .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-T // 81702    01:34 bun repo cargo --store=shared --configuration=prototype --parallelism=4 --  |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 18339 MB available of 32768 MB (floor 6553 MB, 3195 MB spare), pressure normal, swap 5705 MB, load 21.7, open files 7398, attached images 9; 4 running, 3 waiting |
| WARN | graph targets | could not read the published generation (exit 1: o/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts:191:3) at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts:329:88) Bun v1.3.14 (macOS arm64) error: script "repo" exited with code 1); target names not verified, the run itself will show a missing name |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | FAIL 1h54m09s | - | - | - |
| demo | FAIL 1h53m48s | - | - | - |
| course | FAIL 1h53m28s | - | - | - |

## Semio Tech Play (`@semio-tech/semio-tech-play`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | activate | CANCELLED | 1 | 1h54m09s | 0 | 0 | 0 | 0 | - | 4 | cancelled: SIGTERM |

## Mit Bestand Demonstrator (`@semio-tech/mit-bestand-demonstrator`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | activate | CANCELLED | 1 | 1h53m48s | 0 | 0 | 0 | 0 | - | 5 | cancelled: SIGTERM |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | CANCELLED | 1 | 1h53m28s | 0 | 0 | 0 | 0 | - | 5 | cancelled: SIGTERM |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course proctor-build: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 1914 MB spare above the 6553 MB floor, load 25.21; 4 running, 1 ahead)`
- course proctor-build: `[capacity] waiting 5s for run: not enough free memory (need 3584 MB; 1547 MB spare above the 6553 MB floor, load 27.76; 3 running, 1 ahead)`
- course proctor-build: `[capacity] waiting 10s for run: not enough free memory (need 3584 MB; 1138 MB spare above the 6553 MB floor, load 27.3; 4 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 16s for run: not enough free memory (need 3584 MB; 1775 MB spare above the 6553 MB floor, load 27.43; 4 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 21s for run: not enough free memory (need 3584 MB; 1260 MB spare above the 6553 MB floor, load 32.04; 4 running, 0 ahead)`
- demo activate: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 2109 MB spare above the 6553 MB floor, load 20.28; 4 running, 0 ahead)`
- demo activate: `[capacity] waiting 6s for run: not enough free memory (need 3584 MB; 1505 MB spare above the 6553 MB floor, load 24.67; 4 running, 0 ahead)`
- demo activate: `[capacity] waiting 12s for run: not enough free memory (need 3584 MB; 2324 MB spare above the 6553 MB floor, load 24.71; 4 running, 0 ahead)`
- demo activate: `[capacity] waiting 18s for run: not enough free memory (need 3584 MB; 2042 MB spare above the 6553 MB floor, load 25.21; 4 running, 0 ahead)`
- demo activate: `[capacity] waiting 24s for run: not enough free memory (need 3584 MB; 1612 MB spare above the 6553 MB floor, load 27.76; 4 running, 0 ahead)`
- play activate: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 618 MB spare above the 6553 MB floor, load 17.58; 3 running, 1 ahead)`
- play activate: `[capacity] waiting 5s for run: not enough free memory (need 3584 MB; 843 MB spare above the 6553 MB floor, load 17.94; 4 running, 0 ahead)`
- play activate: `[capacity] waiting 11s for run: not enough free memory (need 3584 MB; 188 MB spare above the 6553 MB floor, load 17.86; 4 running, 0 ahead)`
- play activate: `[script] repository:command · Waiting for access to repository-cache-charge:/Users/ueli/Documents/semio/.cache/b/o/m (exclusive): position 2 of 2, behind the request of process 87657 (exclusive, queue`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-21T12:48:00+02:00, t30@2026-10-21T13:18:50+02:00, t60@2026-10-21T13:48:55+02:00, t90@2026-10-21T14:19:24+02:00, end@2026-10-21T14:42:09+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1021b`.

## Resources

Max load 37.8, min available memory 5.6 GB, min free disk 184.3 GB.

## Failures and warnings


### course cold proctor-build: CANCELLED

- command: `bun repo --tool=bazel run @teaching/proctor:build --configuration=release`
- note: cancelled: SIGTERM
- log: `🗑️generated/e2e-orch/e2e-1021b/course/cold-1-proctor-build.log`

```
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":5,\"queued\":7,\"holderMode\":\"shared\",\"holderPid\":81882,\"holderSeconds\":6801,\"reso
[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared): position 5 of 7, behind the request of process 81882 (shared, queued 6801 s ago)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g","mode":"shared","elapsedMs":1140659}}
[script] repository:command remains active on native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared, 1141 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":5,\"queued\":7,\"holderMode\":\"shared\",\"holderPid\":81882,\"holderSeconds\":6817,\"reso
[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared): position 5 of 7, behind the request of process 81882 (shared, queued 6817 s ago)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g","mode":"shared","elapsedMs":1156260}}
[script] repository:command remains active on native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared, 1156 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":5,\"queued\":7,\"holderMode\":\"shared\",\"holderPid\":81882,\"holderSeconds\":6832,\"reso
[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared): position 5 of 7, behind the request of process 81882 (shared, queued 6832 s ago)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g","mode":"shared","elapsedMs":1171427}}
[script] repository:command remains active on native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared, 1171 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"lease-queue:{\"version\":1,\"position\":5,\"queued\":7,\"holderMode\":\"shared\",\"holderPid\":81882,\"holderSeconds\":6847,\"reso
[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared): position 5 of 7, behind the request of process 81882 (shared, queued 6847 s ago)
198 | export function requireIssuedScriptInvocationV1(invocation:ScriptInvocation):asserts invocation is IssuedScriptInvocationV1{birthOf(invocation);checkScriptInvocation(invocation);}
199 | /** 🎛️ Retains canonical cancellation and accepted asynchronous work through actual settlement. */
200 | export function withScriptProcessEnvelope<T>(envelope:ScriptProcessEnvelope,run:(invocation:IssuedScriptInvocationV1)=>Promise<T>):Promise<T>{
201 |  try{const admitted=readScriptProcessEnvelope(envelope,Date.now()),incoming=context.getStore();
202 |  if(incoming){const birth=admit(incoming);if(JSON.stringify(admitted)!==JSON.stringify(birth.envelope))throw Error("Nested Original process envelope must preserve its actual parent");return track(birth,()=>run(incoming as IssuedScript
203 |  const started=performance.now(),deadline=admitted.deadlineEpochMilliseconds,initial=deadline===null?null:deadline-Date.now(),controller=new AbortController(),interrupt=()=>controller.abort(Error("Original Script process interrupted")
                                                                                                                                                                                                                                                
error: Original Script process terminated
      at terminate (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:203:268)
Bun v1.3.14 (macOS arm64)
error: script "repo" exited with code 1
```

### demo cold activate: CANCELLED

- command: `bun repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:activate-prototype`
- note: cancelled: SIGTERM
- log: `🗑️generated/e2e-orch/e2e-1021b/demo/cold-1-activate.log`

```
[script] {"owner":"repository:build","command":"build:selected-catalog-targets","stage":"running"}
[script] {"owner":"repository:build","command":"build:selected-command-sources","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-imports","stage":"running"}
[script] {"owner":"repository:build","command":"build:selected-native-closure","stage":"running"}
[script] {"owner":"repository:build","command":"build:selected-normalization","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-imports","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-packages","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-packages","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-imports","stage":"running"}
[script] {"owner":"repository:build","command":"build:selected-native-preparation","stage":"running"}
[script] {"owner":"repository:build","command":"build:selected-source-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"build:selected-source-closure","stage":"running"}
[script] {"owner":"repository:build","command":"build:selected-compile-closure","stage":"running"}
[script] {"owner":"repository:build","command":"build:selected-compile-imports","stage":"running"}
198 | export function requireIssuedScriptInvocationV1(invocation:ScriptInvocation):asserts invocation is IssuedScriptInvocationV1{birthOf(invocation);checkScriptInvocation(invocation);}
199 | /** 🎛️ Retains canonical cancellation and accepted asynchronous work through actual settlement. */
200 | export function withScriptProcessEnvelope<T>(envelope:ScriptProcessEnvelope,run:(invocation:IssuedScriptInvocationV1)=>Promise<T>):Promise<T>{
201 |  try{const admitted=readScriptProcessEnvelope(envelope,Date.now()),incoming=context.getStore();
202 |  if(incoming){const birth=admit(incoming);if(JSON.stringify(admitted)!==JSON.stringify(birth.envelope))throw Error("Nested Original process envelope must preserve its actual parent");return track(birth,()=>run(incoming as IssuedScript
203 |  const started=performance.now(),deadline=admitted.deadlineEpochMilliseconds,initial=deadline===null?null:deadline-Date.now(),controller=new AbortController(),interrupt=()=>controller.abort(Error("Original Script process interrupted")
                                                                                                                                                                                                                                                
error: Original Script process terminated
      at terminate (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:203:268)
Bun v1.3.14 (macOS arm64)
error: script "repo" exited with code 1
```

### play cold activate: CANCELLED

- command: `bun repo --tool=bazel run @semio-tech/semio-tech-play:activate-prototype`
- note: cancelled: SIGTERM
- log: `🗑️generated/e2e-orch/e2e-1021b/play/cold-1-activate.log`

```
[script] {"owner":"repository:build","command":"build:command-source-packages","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-imports","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-packages","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-imports","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-packages","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-imports","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-packages","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-imports","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-packages","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-imports","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-packages","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-imports","stage":"running"}
[script] {"owner":"repository:build","command":"build:selected-native-preparation","stage":"running"}
[script] {"owner":"repository:build","command":"build:selected-source-inputs","stage":"running"}
198 | export function requireIssuedScriptInvocationV1(invocation:ScriptInvocation):asserts invocation is IssuedScriptInvocationV1{birthOf(invocation);checkScriptInvocation(invocation);}
199 | /** 🎛️ Retains canonical cancellation and accepted asynchronous work through actual settlement. */
200 | export function withScriptProcessEnvelope<T>(envelope:ScriptProcessEnvelope,run:(invocation:IssuedScriptInvocationV1)=>Promise<T>):Promise<T>{
201 |  try{const admitted=readScriptProcessEnvelope(envelope,Date.now()),incoming=context.getStore();
202 |  if(incoming){const birth=admit(incoming);if(JSON.stringify(admitted)!==JSON.stringify(birth.envelope))throw Error("Nested Original process envelope must preserve its actual parent");return track(birth,()=>run(incoming as IssuedScript
203 |  const started=performance.now(),deadline=admitted.deadlineEpochMilliseconds,initial=deadline===null?null:deadline-Date.now(),controller=new AbortController(),interrupt=()=>controller.abort(Error("Original Script process interrupted")
                                                                                                                                                                                                                                                
error: Original Script process terminated
      at terminate (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:203:268)
Bun v1.3.14 (macOS arm64)
error: script "repo" exited with code 1
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1021b --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
