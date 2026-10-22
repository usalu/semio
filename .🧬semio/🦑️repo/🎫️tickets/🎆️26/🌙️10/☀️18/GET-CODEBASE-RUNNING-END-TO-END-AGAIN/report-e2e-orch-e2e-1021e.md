# 🧭 e2e-orch report e2e-1021e

Created 2026-10-21T16:35:49+02:00, written 2026-10-21T17:31:15+02:00, orchestrator pid 65577. Argv: `all --go --run-id e2e-1021e`.

Host: Darwin 27.0.0, 10 cores, parallel 3, Bazel tool `bazel`. Counts: FAIL 4, SKIP-FAILED 21. Cancelled: False.

## Prerequisites

Verdict **WARN** at 2026-10-21T17:05:51+02:00.

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
| OK | disk | 212 GB free (need 60) |
| OK | memory | 15.0 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| INFO | load | 37.8 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 3 product(s) |
| OK | ports play | [6033] free |
| OK | applied: play-bzl E1: activate-prototype cached | present |
| OK | applied: pg-chain: component cache test | present |
| OK | ports demo | [6029] free |
| OK | applied: demo-bzl: activate-prototype cached | present |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 10 running (shared fleet; the governor and the slots arbitrate): 35863       16:24 /bin/zsh .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-EN // 58021       05:37 bun repo cargo --store=shared --configuration=prototype --parallelism=2  // 58022       05:37 bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 15299 MB available of 32768 MB (floor 6553 MB, 3276 MB spare), pressure normal, swap 4695 MB, load 37.82, open files 8297, attached images 9; 3 running, 0 waiting |
| WARN | graph targets | could not read the published generation (exit 124: nt-dependency-generation-receipt-deferral","scope":".","reason":"implementation-unsettled","path":"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/🧬️schema/🔣️.json"}); target names not verified, the run itself will show a missing name |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | FAIL 13m55s | SKIP 0s | SKIP 0s | - |
| demo | FAIL 24m59s | SKIP 0s | SKIP 0s | - |
| course | FAIL 21m50s | SKIP 0s | SKIP 0s | - |

## Semio Tech Play (`@semio-tech/semio-tech-play`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | activate | FAIL | 1 | 13m55s | 0 | 0 | 0 | 0 | - | 5 | exit code 1 |
| warm | activate | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| e2e | test-e2e | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | group 'activate' it depends on failed |

## Mit Bestand Demonstrator (`@semio-tech/mit-bestand-demonstrator`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | activate | FAIL | 1 | 17m24s | 0 | 0 | 0 | 0 | - | 5 | exit code 1 |
| cold | report-compact | FAIL | 1 | 7m35s | 0 | 0 | 0 | 0 | - | 0 | exit code 1 |
| cold | report-all | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'print' failed |
| cold | deck | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'print' failed |
| warm | activate | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | report-compact | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | report-all | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | deck | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| e2e | test-e2e | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | group 'activate' it depends on failed |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | FAIL | 1 | 21m50s | 0 | 0 | 0 | 0 | - | 5 | exit code 1 |
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
| play | activate | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | proctor-build | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | typecheck | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | test | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | build | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | check | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | proctor-test | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| demo | activate | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| demo | report-compact | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| demo | report-all | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| demo | deck | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |

## Queueing (slots, capacity)

- play activate: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; -244 MB spare above the 6553 MB floor, load 40.39; 6 running, 1 ahead)`
- play activate: `[capacity] waiting 5s for run: not enough free memory (need 3584 MB; -697 MB spare above the 6553 MB floor, load 39.48; 6 running, 1 ahead)`
- play activate: `[capacity] waiting 11s for run: not enough free memory (need 3584 MB; 3344 MB spare above the 6553 MB floor, load 40.48; 5 running, 1 ahead)`
- play activate: `[capacity] waiting 17s for run: not enough free memory (need 3584 MB; 2609 MB spare above the 6553 MB floor, load 38.36; 5 running, 1 ahead)`
- play activate: `[capacity] waiting 23s for run: an earlier request is still waiting (need 3584 MB; 2533 MB spare above the 6553 MB floor, load 38.48; 5 running, 1 ahead)`
- demo activate: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 2595 MB spare above the 6553 MB floor, load 39.05; 5 running, 2 ahead)`
- demo activate: `[capacity] waiting 5s for run: an earlier request is still waiting (need 3584 MB; 2452 MB spare above the 6553 MB floor, load 38.48; 5 running, 2 ahead)`
- demo activate: `[capacity] waiting 11s for run: an earlier request is still waiting (need 3584 MB; 2462 MB spare above the 6553 MB floor, load 37.4; 5 running, 2 ahead)`
- demo activate: `[capacity] waiting 16s for run: an earlier request is still waiting (need 3584 MB; 2336 MB spare above the 6553 MB floor, load 38.17; 5 running, 2 ahead)`
- demo activate: `[capacity] waiting 22s for run: an earlier request is still waiting (need 3584 MB; 2261 MB spare above the 6553 MB floor, load 38.96; 5 running, 2 ahead)`
- course proctor-build: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; 2296 MB spare above the 6553 MB floor, load 38.96; 5 running, 3 ahead)`
- course proctor-build: `[capacity] waiting 6s for run: an earlier request is still waiting (need 3584 MB; 2810 MB spare above the 6553 MB floor, load 41.93; 5 running, 3 ahead)`
- course proctor-build: `[capacity] waiting 11s for run: an earlier request is still waiting (need 3584 MB; 2794 MB spare above the 6553 MB floor, load 40.81; 5 running, 3 ahead)`
- course proctor-build: `[capacity] waiting 17s for run: an earlier request is still waiting (need 3584 MB; 3193 MB spare above the 6553 MB floor, load 45.07; 5 running, 3 ahead)`
- course proctor-build: `[capacity] waiting 22s for run: an earlier request is still waiting (need 3584 MB; 2928 MB spare above the 6553 MB floor, load 43.78; 5 running, 3 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-21T17:05:57+02:00, end@2026-10-21T17:31:15+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1021e`.

## Resources

Max load 57.8, min available memory 5.8 GB, min free disk 197.8 GB.

## Failures and warnings


### play cold activate: FAIL

- command: `bun repo --tool=bazel run @semio-tech/semio-tech-play:activate-prototype`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1021e/play/cold-1-activate.log`

```
[script] repository:command · Expanding input closures: 86 of 86 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:155","mode":"shared","elapsedMs":0}}
[script] repository:command · Expanding input closures: 0 of 155 closures
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:expansion:128:155","mode":"shared","elapsedMs":0}}
[script] repository:command · Expanding input closures: 128 of 155 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:expansion:155:155","mode":"shared","elapsedMs":0}}
[script] repository:command · Expanding input closures: 155 of 155 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
[script] {"owner":"repository:build","command":"build:prerequisite-closure","stage":"running"}
[script] {"owner":"repository:build","command":"build:prerequisite-closure","stage":"running"}
[script] {"owner":"repository:build","command":"build:prerequisite-closure","stage":"running"}
33 |  if(bytes>policy.maximumRetainedBytes)throw Error("Pinned build graphs exhaust the finite snapshot budget");
34 | }
35 | 
36 | /** 📌️ Retains immutable graph bytes and their enforcing storage through one backend consumer. */
37 | export async function withRepositoryBuildGraphArtifactV1<T>(workspace:string,graph:BuildGraphV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(artifact:RepositoryBuildGraphArtifactV1,actualControl:ResourceLeaseCont
38 |  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkScriptInvocation(invocation);if(validateJsonSchemaSubset(schema,policy).length||policy.maximumSnapshotBytes>policy.maximumRetainedBytes)throw Error("Closed graph a
RangeError: Out of memory
      at withRepositoryBuildGraphArtifactV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts:38:280)
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts:213:11)
Bun v1.3.14 (macOS arm64)
error: script "repo" exited with code 1
```

### demo cold activate: FAIL

- command: `bun repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:activate-prototype`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1021e/demo/cold-1-activate.log`

```
[script] repository:command · Expanding input closures: 0 of 28 closures
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:expansion:28:28","mode":"shared","elapsedMs":0}}
[script] repository:command · Expanding input closures: 28 of 28 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:expansion:0:155","mode":"shared","elapsedMs":0}}
[script] repository:command · Expanding input closures: 0 of 155 closures
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:expansion:128:155","mode":"shared","elapsedMs":0}}
[script] repository:command · Expanding input closures: 128 of 155 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:expansion:155:155","mode":"shared","elapsedMs":0}}
[script] repository:command · Expanding input closures: 155 of 155 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
[script] {"owner":"repository:build","command":"build:prerequisite-closure","stage":"running"}
33 |  if(bytes>policy.maximumRetainedBytes)throw Error("Pinned build graphs exhaust the finite snapshot budget");
34 | }
35 | 
36 | /** 📌️ Retains immutable graph bytes and their enforcing storage through one backend consumer. */
37 | export async function withRepositoryBuildGraphArtifactV1<T>(workspace:string,graph:BuildGraphV1,invocation:ScriptInvocation,phaseControl:ResourceLeaseControl,body:(artifact:RepositoryBuildGraphArtifactV1,actualControl:ResourceLeaseCont
38 |  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkScriptInvocation(invocation);if(validateJsonSchemaSubset(schema,policy).length||policy.maximumSnapshotBytes>policy.maximumRetainedBytes)throw Error("Closed graph a
RangeError: Out of memory
      at withRepositoryBuildGraphArtifactV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts:38:280)
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts:213:11)
Bun v1.3.14 (macOS arm64)
error: script "repo" exited with code 1
```

### course cold proctor-build: FAIL

- command: `bun repo --tool=bazel run @teaching/proctor:build --configuration=release`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1021e/course/cold-1-proctor-build.log`

```
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"javascript-preserving-source:2","mode":"exclusive","elapsedMs":1.1699999999254942}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:store-ownership:state:1009984","mode":"exclusive","elapsedMs":104674.34224999999}}
[script] repository:command remains active on dependencies:store-ownership:state:1009984 (exclusive, 105 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:store-ownership:list:1015296","mode":"exclusive","elapsedMs":105112.12883400009}}
[script] repository:command remains active on dependencies:store-ownership:list:1015296 (exclusive, 105 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:store-ownership:selection:1015872","mode":"exclusive","elapsedMs":105126.17529199994}}
[script] repository:command remains active on dependencies:store-ownership:selection:1015872 (exclusive, 105 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:store-ownership:readText:1016192","mode":"exclusive","elapsedMs":105264.30541700008}}
[script] repository:command remains active on dependencies:store-ownership:readText:1016192 (exclusive, 105 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"javascript-preserving-source:322","mode":"exclusive","elapsedMs":121.98475000006147}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:store-input-replay","mode":"shared","elapsedMs":0}}
[DATA] Store dependency installer failed {"version":1,"purpose":"current-dependency-installer-failure","scope":".","status":1,"signal":null,"stdoutBytes":31,"stderrBytes":193,"stdoutSha256":"2d879def92fdd8299c1b8dea63dfee2e9d195f3c0a48c784b
[DATA] Store dependency sync incomplete {"version":2,"purpose":"current-store-dependency-sync-failure","completedBindingsChanged":0,"sourceTransitionMayBeUnresolved":false,"activeSourceDirectory":null,"activeIntentSha256":null,"retainedCaps
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:store-input-replay","mode":"shared","elapsedMs":0}}
561 | /** 🩺 Projects bounded credential-redacted terminal DATA without exporting raw output, environment or arguments. */
562 | export function javascriptDependencyInstallerFailureDiagnosticV1(scope:string,terminal:OwnedCommandCaptureV1){
563 |  const policy=installSchema.$defs.InstallerFailureDiagnosticPolicy.const,bytes=Buffer.from(terminal.stderr),offset=Math.max(0,bytes.length-policy.maximumBytes*2);let selected=bytes.subarray(offset).toString("utf8");if(offset>0){const 
564 | }
565 | /** 📣 Publishes failure DATA under the original bounded output owner while preserving the installer primary. */
566 | async function failJavascriptDependencyInstallerV1(scope:string,terminal:OwnedCommandCaptureV1,control:ResourceLeaseControl):Promise<never>{const cause=javascriptDependencyInstallerFailureDiagnosticV1(scope,terminal);try{await writeOw
                                                                                                                                                                                                                                                
error: Fixed Store dependency installer failed
      at failJavascriptDependencyInstallerV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:566:363)
Bun v1.3.14 (macOS arm64)
error: script "repo" exited with code 1
```

### demo cold report-compact: FAIL

- command: `bun repo --tool=bazel run @semio-tech/mit-bestand-bericht:build-kompaktbericht`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1021e/demo/cold-2-report-compact.log`

```
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:store-ownership:list:1779776","mode":"exclusive","elapsedMs":156824.132041}}
[script] repository:command remains active on dependencies:store-ownership:list:1779776 (exclusive, 157 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:store-ownership:state:1785088","mode":"exclusive","elapsedMs":157394.601708}}
[script] repository:command remains active on dependencies:store-ownership:state:1785088 (exclusive, 157 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:store-ownership:selection:1785664","mode":"exclusive","elapsedMs":157419.3285}}
[script] repository:command remains active on dependencies:store-ownership:selection:1785664 (exclusive, 157 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:store-ownership:readText:1785984","mode":"exclusive","elapsedMs":157514.66737500002}}
[script] repository:command remains active on dependencies:store-ownership:readText:1785984 (exclusive, 158 s)
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"javascript-preserving-source:322","mode":"exclusive","elapsedMs":266.7393339999835}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:store-input-replay","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:store-input-replay","mode":"shared","elapsedMs":0}}
[DATA] Store dependency installer failed {"version":1,"purpose":"current-dependency-installer-failure","scope":".","status":1,"signal":null,"stdoutBytes":31,"stderrBytes":193,"stdoutSha256":"2d879def92fdd8299c1b8dea63dfee2e9d195f3c0a48c784b
[DATA] Store dependency sync incomplete {"version":2,"purpose":"current-store-dependency-sync-failure","completedBindingsChanged":0,"sourceTransitionMayBeUnresolved":false,"activeSourceDirectory":null,"activeIntentSha256":null,"retainedCaps
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:store-input-replay","mode":"shared","elapsedMs":0}}
561 | /** 🩺 Projects bounded credential-redacted terminal DATA without exporting raw output, environment or arguments. */
562 | export function javascriptDependencyInstallerFailureDiagnosticV1(scope:string,terminal:OwnedCommandCaptureV1){
563 |  const policy=installSchema.$defs.InstallerFailureDiagnosticPolicy.const,bytes=Buffer.from(terminal.stderr),offset=Math.max(0,bytes.length-policy.maximumBytes*2);let selected=bytes.subarray(offset).toString("utf8");if(offset>0){const 
564 | }
565 | /** 📣 Publishes failure DATA under the original bounded output owner while preserving the installer primary. */
566 | async function failJavascriptDependencyInstallerV1(scope:string,terminal:OwnedCommandCaptureV1,control:ResourceLeaseControl):Promise<never>{const cause=javascriptDependencyInstallerFailureDiagnosticV1(scope,terminal);try{await writeOw
                                                                                                                                                                                                                                                
error: Fixed Store dependency installer failed
      at failJavascriptDependencyInstallerV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:566:363)
Bun v1.3.14 (macOS arm64)
error: script "repo" exited with code 1
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1021e --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
