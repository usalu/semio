# 🧭 e2e-orch report e2e-1021g

Created 2026-10-21T20:13:27+02:00, written 2026-10-21T20:14:18+02:00, orchestrator pid 22115. Argv: `course --go --run-id e2e-1021g`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1, SKIP-FAILED 12. Cancelled: False.

## Prerequisites

Verdict **WARN** at 2026-10-21T20:13:32+02:00.

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
| OK | disk | 206 GB free (need 60) |
| OK | memory | 21.9 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 7.7 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 1 running (shared fleet; the governor and the slots arbitrate): 24781    02:56:03 bazel(ws) --add-opens=java.base/java.lang=ALL-UNNAMED -Xverify:none -Dja |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 22397 MB available of 32768 MB (floor 6553 MB, 15844 MB spare), pressure normal, swap 5431 MB, load 7.7, open files 6021, attached images 9; 1 running, 0 waiting |
| WARN | graph targets | could not read the published generation (exit 1:  failed (1) outcome: { version: 1, status: 1, signal: null, }, at runOwnedCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:168:60) Bun v1.3.14 (macOS arm64) error: script "repo" exited with code 1); target names not verified, the run itself will show a missing name |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | FAIL 2s | SKIP 0s | SKIP 0s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | FAIL | 1 | 2s | 0 | 0 | 0 | 0 | - | 0 | exit code 1 |
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

no waiting lines recorded.

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-21T20:13:36+02:00, end@2026-10-21T20:14:18+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1021g`.

## Resources

Max load 8.7, min available memory 21.9 GB, min free disk 206.5 GB.

## Failures and warnings


### course cold proctor-build: FAIL

- command: `bun repo --tool=bazel run @teaching/proctor:build --configuration=release`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1021g/course/cold-1-proctor-build.log`

```
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"resource-retirement-inventory:128","mode":"shared","elapsedMs":0}}
SyntaxError: Export named 'createBunWorkspaceOwnershipV1' not found in module 'semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs'.
Bun v1.3.14 (macOS arm64)
163 |     if(auxiliary&&commandSignal?.aborted)throw commandSignal.reason;
164 |     if(auxiliaryFailed)throw auxiliaryFailure;
165 |     if(auxiliary)requireIssuedResourceControlV1(options.control!);
166 |     if (stopped&&stopped!=="output delivery refused") throw new Error(`${label} stopped: ${stopped}`);
167 |     if(stopped==="output delivery refused"&&status.signal!==null)throw deliveryFailure;
168 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
                                                                 ^
OwnedCommandFailure: repository:command failed (1)
 outcome: {
  version: 1,
  status: 1,
  signal: null,
},
      at runOwnedCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:168:60)
Bun v1.3.14 (macOS arm64)
error: script "repo" exited with code 1
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1021g --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
