# 🧭 e2e-orch report e2e-1021c

Created 2026-10-21T14:42:19+02:00, written 2026-10-21T14:49:56+02:00, orchestrator pid 7933. Argv: `all --go --run-id e2e-1021c`.

Host: Darwin 27.0.0, 10 cores, parallel 3, Bazel tool `bazel`. Counts: FAIL 4, SKIP-FAILED 21. Cancelled: False.

## Prerequisites

Verdict **WARN** at 2026-10-21T14:44:49+02:00.

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
| OK | disk | 198 GB free (need 60) |
| OK | memory | 20.3 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 16.1 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 3 product(s) |
| OK | ports play | [6033] free |
| OK | applied: play-bzl E1: activate-prototype cached | present |
| OK | applied: pg-chain: component cache test | present |
| OK | ports demo | [6029] free |
| OK | applied: demo-bzl: activate-prototype cached | present |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 9 running (shared fleet; the governor and the slots arbitrate): 4529    03:01 bun repo cargo --store=shared --configuration=prototype --parallelism=2 -- t // 4530    03:01 bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts  // 7897    00:01 cargo test --manifest-path Cargo.toml -p semio-framework-os-vcs -p semio-fra |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 21036 MB available of 32768 MB (floor 6553 MB, -877 MB spare), pressure normal, swap 6922 MB, load 16.1, open files 6078, attached images 9; 5 running, 3 waiting |
| WARN | graph targets | could not read the published generation (exit 1: /🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts:192:3) at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts:330:357) Bun v1.3.14 (macOS arm64) error: script "repo" exited with code 1); target names not verified, the run itself will show a missing name |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | FAIL 4m00s | SKIP 0s | SKIP 0s | - |
| demo | FAIL 4m43s | SKIP 0s | SKIP 0s | - |
| course | FAIL 3m22s | SKIP 0s | SKIP 0s | - |

## Semio Tech Play (`@semio-tech/semio-tech-play`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | activate | FAIL | 1 | 4m00s | 0 | 0 | 0 | 0 | - | 5 | exit code 1 |
| warm | activate | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| e2e | test-e2e | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | group 'activate' it depends on failed |

## Mit Bestand Demonstrator (`@semio-tech/mit-bestand-demonstrator`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | activate | FAIL | 1 | 3m38s | 0 | 0 | 0 | 0 | - | 5 | exit code 1 |
| cold | report-compact | FAIL | 1 | 1m05s | 0 | 0 | 0 | 0 | - | 3 | exit code 1 |
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
| cold | proctor-build | FAIL | 1 | 3m22s | 0 | 0 | 0 | 0 | - | 4 | exit code 1 |
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

- demo activate: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 3487 MB spare above the 6553 MB floor, load 23.41; 4 running, 2 ahead)`
- demo activate: `[capacity] waiting 6s for run: not enough free memory (need 3584 MB; 1397 MB spare above the 6553 MB floor, load 24.1; 4 running, 1 ahead)`
- demo activate: `[capacity] waiting 11s for run: not enough free memory (need 3584 MB; 1634 MB spare above the 6553 MB floor, load 23.21; 4 running, 1 ahead)`
- demo activate: `[capacity] waiting 16s for run: not enough free memory (need 3584 MB; 1958 MB spare above the 6553 MB floor, load 21.99; 4 running, 1 ahead)`
- demo activate: `[capacity] waiting 21s for run: not enough free memory (need 3584 MB; 3176 MB spare above the 6553 MB floor, load 24.99; 4 running, 1 ahead)`
- play activate: `[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared): position 2 of 2, behind the request of process 95797 (exclusive, queued 28 s ago`
- play activate: `[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared): position 2 of 3, behind the request of process 95797 (exclusive, queued 43 s ago`
- play activate: `[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared): position 2 of 3, behind the request of process 95797 (exclusive, queued 59 s ago`
- play activate: `[script] repository:command · Waiting for access to native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g (shared): position 2 of 4, behind the request of process 95797 (exclusive, queued 74 s ago`
- play activate: `[script] repository:command · Waiting for access to repository-cache-charge:/Users/ueli/Documents/semio/.cache/b/g/m (exclusive): position 2 of 2, behind the request of process 12767 (exclusive, queue`
- course proctor-build: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 2798 MB spare above the 6553 MB floor, load 24.55; 4 running, 2 ahead)`
- course proctor-build: `[capacity] waiting 5s for run: not enough free memory (need 3584 MB; 2573 MB spare above the 6553 MB floor, load 24.99; 4 running, 1 ahead)`
- course proctor-build: `[capacity] waiting 10s for run: not enough free memory (need 3584 MB; 2532 MB spare above the 6553 MB floor, load 27.07; 4 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 16s for run: not enough free memory (need 3584 MB; 1947 MB spare above the 6553 MB floor, load 25.62; 4 running, 0 ahead)`
- demo report-compact: `[capacity] waiting 0s for run: swap use is above its ceiling (need 3584 MB; 3806 MB spare above the 6553 MB floor, load 20.25; 3 running, 4 ahead)`
- demo report-compact: `[capacity] waiting 6s for run: not enough free memory (need 3584 MB; 2676 MB spare above the 6553 MB floor, load 21.67; 5 running, 0 ahead)`
- demo report-compact: `[capacity] waiting 12s for run: the host is swapping out too fast (need 3584 MB; 2431 MB spare above the 6553 MB floor, load 20.34; 5 running, 0 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-21T14:44:56+02:00, end@2026-10-21T14:49:56+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1021c`.

## Resources

Max load 25.8, min available memory 5.7 GB, min free disk 157.8 GB.

## Failures and warnings


### demo cold activate: FAIL

- command: `bun repo --tool=bazel run @semio-tech/mit-bestand-demonstrator:activate-prototype`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1021c/demo/cold-1-activate.log`

```
[script] repository:command · Expanding input closures: 256 of 271 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:expansion:271:271","mode":"shared","elapsedMs":0}}
[script] repository:command · Expanding input closures: 271 of 271 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
14 | /** 🧷️ Rejects a retained helper whose Source or privately captured policies changed. */
15 | export function requireCurrentInferenceInventoryReuseV1(){if(implementationPinnedV1())return;if(epochURLs.some((url,index)=>revision(url,index)!==loadedEpoch[index]))throw Error("Stale inference inventory reuse implementation; restart 
16 | 
17 | /** 📋️ Captures finite inert compact references without retaining mutable caller DATA. */
18 | export function captureInferenceInventoryReferencesV1(value){
19 |  let values=0,text=0;const active=new Set(),names=new Set(),charge=value=>{text+=Buffer.byteLength(value);if(text>policy.maximumTextBytes)throw Error("Inference reference text capacity exceeded");};
                                                                                                                                                     ^
error: Inference reference text capacity exceeded
      at charge (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:19:145)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:22:33)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at captureInferenceInventoryReferencesV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:26:4)
      at portableInputs (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts:192:3)
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts:330:357)
Bun v1.3.14 (macOS arm64)
error: script "repo" exited with code 1
```

### play cold activate: FAIL

- command: `bun repo --tool=bazel run @semio-tech/semio-tech-play:activate-prototype`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1021c/play/cold-1-activate.log`

```
[script] repository:command · Expanding input closures: 256 of 271 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:expansion:271:271","mode":"shared","elapsedMs":0}}
[script] repository:command · Expanding input closures: 271 of 271 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
14 | /** 🧷️ Rejects a retained helper whose Source or privately captured policies changed. */
15 | export function requireCurrentInferenceInventoryReuseV1(){if(implementationPinnedV1())return;if(epochURLs.some((url,index)=>revision(url,index)!==loadedEpoch[index]))throw Error("Stale inference inventory reuse implementation; restart 
16 | 
17 | /** 📋️ Captures finite inert compact references without retaining mutable caller DATA. */
18 | export function captureInferenceInventoryReferencesV1(value){
19 |  let values=0,text=0;const active=new Set(),names=new Set(),charge=value=>{text+=Buffer.byteLength(value);if(text>policy.maximumTextBytes)throw Error("Inference reference text capacity exceeded");};
                                                                                                                                                     ^
error: Inference reference text capacity exceeded
      at charge (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:19:145)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:22:33)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at captureInferenceInventoryReferencesV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:26:4)
      at portableInputs (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts:192:3)
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts:330:357)
Bun v1.3.14 (macOS arm64)
error: script "repo" exited with code 1
```

### course cold proctor-build: FAIL

- command: `bun repo --tool=bazel run @teaching/proctor:build --configuration=release`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1021c/course/cold-1-proctor-build.log`

```
[script] repository:command · Expanding input closures: 256 of 271 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:expansion:271:271","mode":"shared","elapsedMs":0}}
[script] repository:command · Expanding input closures: 271 of 271 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
14 | /** 🧷️ Rejects a retained helper whose Source or privately captured policies changed. */
15 | export function requireCurrentInferenceInventoryReuseV1(){if(implementationPinnedV1())return;if(epochURLs.some((url,index)=>revision(url,index)!==loadedEpoch[index]))throw Error("Stale inference inventory reuse implementation; restart 
16 | 
17 | /** 📋️ Captures finite inert compact references without retaining mutable caller DATA. */
18 | export function captureInferenceInventoryReferencesV1(value){
19 |  let values=0,text=0;const active=new Set(),names=new Set(),charge=value=>{text+=Buffer.byteLength(value);if(text>policy.maximumTextBytes)throw Error("Inference reference text capacity exceeded");};
                                                                                                                                                     ^
error: Inference reference text capacity exceeded
      at charge (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:19:145)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:22:33)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at captureInferenceInventoryReferencesV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:26:4)
      at portableInputs (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts:192:3)
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts:330:357)
Bun v1.3.14 (macOS arm64)
error: script "repo" exited with code 1
```

### demo cold report-compact: FAIL

- command: `bun repo --tool=bazel run @semio-tech/mit-bestand-bericht:build-kompaktbericht`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1021c/demo/cold-2-report-compact.log`

```
[script] repository:command · Expanding input closures: 256 of 271 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"running"}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"build:graph-phase:expansion:271:271","mode":"shared","elapsedMs":0}}
[script] repository:command · Expanding input closures: 271 of 271 closures
[script] {"owner":"repository:build","command":"build:portable-inputs","stage":"complete"}
14 | /** 🧷️ Rejects a retained helper whose Source or privately captured policies changed. */
15 | export function requireCurrentInferenceInventoryReuseV1(){if(implementationPinnedV1())return;if(epochURLs.some((url,index)=>revision(url,index)!==loadedEpoch[index]))throw Error("Stale inference inventory reuse implementation; restart 
16 | 
17 | /** 📋️ Captures finite inert compact references without retaining mutable caller DATA. */
18 | export function captureInferenceInventoryReferencesV1(value){
19 |  let values=0,text=0;const active=new Set(),names=new Set(),charge=value=>{text+=Buffer.byteLength(value);if(text>policy.maximumTextBytes)throw Error("Inference reference text capacity exceeded");};
                                                                                                                                                     ^
error: Inference reference text capacity exceeded
      at charge (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:19:145)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:22:33)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at visit (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:25:635)
      at captureInferenceInventoryReferencesV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/📥️reuse/🟨️.mjs:26:4)
      at portableInputs (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts:192:3)
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🟦️.ts:330:357)
Bun v1.3.14 (macOS arm64)
error: script "repo" exited with code 1
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1021c --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
