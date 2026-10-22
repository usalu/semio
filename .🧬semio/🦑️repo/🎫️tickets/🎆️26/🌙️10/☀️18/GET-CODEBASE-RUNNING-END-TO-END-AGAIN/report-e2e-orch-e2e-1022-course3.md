# 🧭 e2e-orch report e2e-1022-course3

Created 2026-10-22T20:03:50+02:00, written 2026-10-22T21:42:52+02:00, orchestrator pid 6342. Argv: `course --go --run-id e2e-1022-course3`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1, PASS 3, SKIP-FAILED 9. Cancelled: False.

## Prerequisites

Verdict **WARN** at 2026-10-22T20:04:06+02:00.

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
| OK | disk | 106 GB free (need 60) |
| OK | memory | 14.3 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 6.2 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 10 running (shared fleet; the governor and the slots arbitrate): 5265       00:46 bun repo cargo --store=shared --configuration=prototype -- test -p semio- // 5266       00:46 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551 // 5416       00:39 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551 |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 14629 MB available of 32768 MB (floor 6553 MB, 8076 MB spare), pressure normal, swap 3238 MB, load 6.2, open files 5062, attached images 26; 3 running, 0 waiting |
| OK | graph targets | 2 project(s) and their 9 target name(s) found in lean `show projects/targets --json` (SEMIO_BUILD_GRAPH_POLICY=current) |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | FAIL 1h37m47s | SKIP 0s | SKIP 0s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | PASS | 0 | 8m59s | 2 | 1 | 0 | 1 | s0 | 5 |  |
| cold | typecheck | PASS | 0 | 12m58s | 8 | 7 | 0 | 1 | s2 | 5 |  |
| cold | test | PASS | 0 | 17m03s | 11 | 10 | 0 | 2 | s2 | 5 |  |
| cold | build | FAIL | 1 | 58m47s | 0 | 0 | 0 | 0 | - | 5 | exit code 1 |
| cold | check | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | proctor-test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| warm | proctor-build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| warm | typecheck | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| warm | test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| warm | build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | check | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | proctor-test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| e2e | test-e2e | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | group 'core' it depends on failed |

## Warm assertions (Bazel cache hits)

| product | step | spawns | allowed | internal | cache hits | time | expected within | verdict |
|---|---|---|---|---|---|---|---|---|
| course | proctor-build | - | 0 | - | - | 0s | - | SKIP-FAILED an earlier step of group 'core' failed |
| course | typecheck | - | 0 | - | - | 0s | - | SKIP-FAILED an earlier step of group 'core' failed |
| course | test | - | 0 | - | - | 0s | - | SKIP-FAILED an earlier step of group 'core' failed |
| course | build | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | check | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | proctor-test | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |

## Queueing (slots, capacity)

- course proctor-build: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 2006 MB spare above the 6553 MB floor, load 6.15; 3 running, 2 ahead)`
- course proctor-build: `[capacity] waiting 5s for run: not enough free memory (need 3584 MB; 28 MB spare above the 6553 MB floor, load 6.21; 3 running, 2 ahead)`
- course proctor-build: `[capacity] waiting 11s for run: not enough free memory (need 3584 MB; 402 MB spare above the 6553 MB floor, load 6.44; 4 running, 1 ahead)`
- course proctor-build: `[capacity] waiting 17s for run: not enough free memory (need 3584 MB; 212 MB spare above the 6553 MB floor, load 6.76; 4 running, 1 ahead)`
- course proctor-build: `[capacity] waiting 22s for run: an earlier request is still waiting (need 3584 MB; -4 MB spare above the 6553 MB floor, load 6.7; 4 running, 1 ahead)`
- course typecheck: `[capacity] waiting 0s for run: an earlier request is still waiting (need 5632 MB; 1456 MB spare above the 6553 MB floor, load 18.41; 4 running, 7 ahead)`
- course typecheck: `[capacity] waiting 5s for run: an earlier request is still waiting (need 5632 MB; 2617 MB spare above the 6553 MB floor, load 17.66; 5 running, 6 ahead)`
- course typecheck: `[capacity] waiting 10s for run: an earlier request is still waiting (need 5632 MB; 1494 MB spare above the 6553 MB floor, load 18.09; 5 running, 6 ahead)`
- course typecheck: `[capacity] waiting 16s for run: an earlier request is still waiting (need 5632 MB; 2198 MB spare above the 6553 MB floor, load 17.12; 4 running, 6 ahead)`
- course typecheck: `[capacity] waiting 22s for run: an earlier request is still waiting (need 5632 MB; 1045 MB spare above the 6553 MB floor, load 16.47; 5 running, 5 ahead)`
- course test: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; 401 MB spare above the 6553 MB floor, load 13.22; 4 running, 11 ahead)`
- course test: `[capacity] waiting 5s for run: an earlier request is still waiting (need 3584 MB; -319 MB spare above the 6553 MB floor, load 13.04; 6 running, 9 ahead)`
- course test: `[capacity] waiting 10s for run: an earlier request is still waiting (need 3584 MB; -3185 MB spare above the 6553 MB floor, load 14.88; 6 running, 9 ahead)`
- course test: `[capacity] waiting 16s for run: an earlier request is still waiting (need 3584 MB; 3050 MB spare above the 6553 MB floor, load 14.57; 6 running, 9 ahead)`
- course test: `[capacity] waiting 21s for run: an earlier request is still waiting (need 3584 MB; 1425 MB spare above the 6553 MB floor, load 14.2; 6 running, 8 ahead)`
- course build: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 2791 MB spare above the 6553 MB floor, load 16.38; 5 running, 9 ahead)`
- course build: `[capacity] waiting 5s for run: an earlier request is still waiting (need 3584 MB; 1339 MB spare above the 6553 MB floor, load 15.79; 6 running, 8 ahead)`
- course build: `[capacity] waiting 11s for run: an earlier request is still waiting (need 3584 MB; -873 MB spare above the 6553 MB floor, load 15.32; 6 running, 8 ahead)`
- course build: `[capacity] waiting 16s for run: an earlier request is still waiting (need 3584 MB; 3310 MB spare above the 6553 MB floor, load 15.63; 6 running, 8 ahead)`
- course build: `[capacity] waiting 22s for run: an earlier request is still waiting (need 3584 MB; 3481 MB spare above the 6553 MB floor, load 15.66; 6 running, 8 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-22T20:04:11+02:00, t30@2026-10-22T20:34:13+02:00, t60@2026-10-22T21:04:18+02:00, t90@2026-10-22T21:34:24+02:00, end@2026-10-22T21:42:52+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1022-course3`.

## Resources

Max load 46.1, min available memory 5.5 GB, min free disk 98.5 GB.

## Failures and warnings


### course cold build: FAIL

- command: `bun repo --tool=bazel run @teaching/architecture-course:build --configuration=release`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1022-course3/course/cold-4-build.log`

```
      at async inferProjectBodyV1 (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:73:27)
      at async <anonymous> (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs:160:35)
      at async observeInferenceComputationV1 (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs:1081:153)
      at async observeProjectInferenceV1 (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs:157:23)
      at async tagProjectFailureV1 (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:45:18)
      at async <anonymous> (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:189:35)
      at async <anonymous> (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:211:19)
      at async run (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs:179:22)
Bun v1.4.2 (macOS arm64)
164 |     if(auxiliary&&commandSignal?.aborted)throw commandSignal.reason;
165 |     if(auxiliaryFailed)throw auxiliaryFailure;
166 |     if(auxiliary)requireIssuedResourceControlV1(options.control!);
167 |     if (stopped&&stopped!=="output delivery refused") throw new Error(`${label} stopped: ${stopped}`);
168 |     if(stopped==="output delivery refused"&&status.signal!==null)throw deliveryFailure;
169 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
                                                                 ^
OwnedCommandFailure: repository:command failed (1)
 outcome: {
  version: 1,
  status: 1,
  signal: null,
},
      at runOwnedCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:169:60)
Bun v1.4.2 (macOS arm64)
error: script "repo" exited with code 1
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1022-course3 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
