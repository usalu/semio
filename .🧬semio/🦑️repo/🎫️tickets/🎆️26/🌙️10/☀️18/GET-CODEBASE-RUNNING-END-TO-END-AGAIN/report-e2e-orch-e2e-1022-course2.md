# 🧭 e2e-orch report e2e-1022-course2

Created 2026-10-22T15:46:44+02:00, written 2026-10-22T16:04:29+02:00, orchestrator pid 28215. Argv: `course --go --run-id e2e-1022-course2`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1, PASS 1, SKIP-FAILED 11. Cancelled: False.

## Prerequisites

Verdict **WARN** at 2026-10-22T15:59:50+02:00.

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
| OK | disk | 147 GB free (need 60) |
| OK | memory | 13.2 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 12.1 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 27 running (shared fleet; the governor and the slots arbitrate): 13235       10:54 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 // 13535       10:30 bazel(e587c803008acc66) --add-opens=java.base/java.lang=ALL-UNNAMED -Xve // 15448       08:20 bazel(4d9d1194ea484fc7) --add-opens=java.base/java.lang=ALL-UNNAMED -Xve |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 13572 MB available of 32768 MB (floor 6553 MB, -1300 MB spare), pressure normal, swap 9288 MB, load 12.09, open files 5688, attached images 20; 5 running, 0 waiting |
| OK | graph targets | 2 project(s) and their 9 target name(s) found in lean `show projects/targets --json` (SEMIO_BUILD_GRAPH_POLICY=current) |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | FAIL 3m53s | SKIP 0s | SKIP 0s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | PASS | 0 | 2m08s | 2 | 1 | 0 | 1 | s1 | 3 |  |
| cold | typecheck | FAIL | 1 | 1m44s | 7 | 3 | 0 | 4 | s2 | 5 | exit code 1 |
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

- course proctor-build: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; -3735 MB spare above the 6553 MB floor, load 7.03; 5 running, 1 ahead)`
- course proctor-build: `[capacity] waiting 5s for run: not enough free memory (need 3584 MB; 2832 MB spare above the 6553 MB floor, load 6.78; 5 running, 1 ahead)`
- course proctor-build: `[capacity] waiting 10s for run: not enough free memory (need 3584 MB; 2897 MB spare above the 6553 MB floor, load 6.72; 4 running, 0 ahead)`
- course typecheck: `[capacity] waiting 0s for run: an earlier request is still waiting (need 5632 MB; 812 MB spare above the 6553 MB floor, load 6.16; 1 running, 3 ahead)`
- course typecheck: `[capacity] waiting 6s for run: an earlier request is still waiting (need 5632 MB; -821 MB spare above the 6553 MB floor, load 5.99; 2 running, 1 ahead)`
- course typecheck: `[capacity] waiting 11s for run: not enough free memory (need 5632 MB; 3228 MB spare above the 6553 MB floor, load 5.67; 1 running, 0 ahead)`
- course typecheck: `[capacity] waiting 16s for run: not enough free memory (need 5632 MB; 763 MB spare above the 6553 MB floor, load 5.45; 1 running, 0 ahead)`
- course typecheck: `[capacity] waiting 21s for run: not enough free memory (need 5632 MB; -970 MB spare above the 6553 MB floor, load 5.34; 1 running, 0 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-22T15:59:56+02:00, end@2026-10-22T16:04:29+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1022-course2`.

## Resources

Max load 8.7, min available memory 13.0 GB, min free disk 136.8 GB.

## Failures and warnings


### course cold typecheck: FAIL

- command: `bun repo --tool=bazel run @teaching/architecture-course:typecheck`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1022-course2/course/cold-2-typecheck.log`

```
OwnedCommandFailure: bazel:build failed (1)
 outcome: {
  version: 1,
  status: 1,
  signal: null,
},
      at runOwnedCommand (semio-source:/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:322:13)
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
      at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts:15:994)
Bun v1.4.2 (macOS arm64)
error: script "repo" exited with code 1
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1022-course2 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
