# 🧭 e2e-orch report e2e-1023-course10

Created 2026-10-23T20:08:28+02:00, written 2026-10-23T21:32:49+02:00, orchestrator pid 2875. Argv: `course --go --run-id e2e-1023-course10 --no-verify-targets --phase cold,warm,e2e,probe`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1, SKIP-FAILED 12. Cancelled: False.

## Prerequisites

Verdict **OK** at 2026-10-23T20:08:29+02:00.

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
| OK | disk | 110 GB free (need 60) |
| OK | memory | 8.5 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 6.5 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 63 running (shared fleet; the governor and the slots arbitrate): 470    01:21:01 bun repo --tool=bazel run @semio-tech/trinity-jack-shell:test // 471    01:21:01 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e // 1217    01:20:28 bun repo cargo --store=shared --configuration=prototype -- test --manifes |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 8871 MB available of 32768 MB (floor 6553 MB, 2286 MB spare), pressure normal, swap 18852 MB, load 6.46, open files 7941, attached images 19; 12 running, 10 waiting |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | FAIL 1h23m35s | SKIP 0s | SKIP 0s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | FAIL | 1 | 1h23m35s | 0 | 0 | 0 | 0 | - | 5 | exit code 1 |
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

- course proctor-build: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; 2114 MB spare above the 6553 MB floor, load 6.19; 11 running, 10 ahead)`
- course proctor-build: `[capacity] waiting 6s for run: an earlier request is still waiting (need 3584 MB; 1377 MB spare above the 6553 MB floor, load 6.26; 11 running, 10 ahead)`
- course proctor-build: `[capacity] waiting 12s for run: an earlier request is still waiting (need 3584 MB; 430 MB spare above the 6553 MB floor, load 7.01; 11 running, 10 ahead)`
- course proctor-build: `[capacity] waiting 17s for run: an earlier request is still waiting (need 3584 MB; 296 MB spare above the 6553 MB floor, load 7.65; 11 running, 10 ahead)`
- course proctor-build: `[capacity] waiting 22s for run: an earlier request is still waiting (need 3584 MB; -936 MB spare above the 6553 MB floor, load 7.44; 11 running, 10 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-23T20:08:34+02:00, t30@2026-10-23T20:38:36+02:00, t60@2026-10-23T21:08:37+02:00, end@2026-10-23T21:32:49+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1023-course10`.

## Resources

Max load 10.0, min available memory 5.7 GB, min free disk 100.3 GB.

## Failures and warnings


### course cold proctor-build: FAIL

- command: `bun repo --tool=bazel run @teaching/proctor:build --configuration=release`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1023-course10/course/cold-1-proctor-build.log`

```
[repository:command] running elapsedMs=680107
[bazel] all 3 Bazel server slots are in use; waiting for one (589 s)
[bazel] all 3 Bazel server slots are in use; waiting for one (594 s)
[repository:command] running elapsedMs=690109
[bazel] all 3 Bazel server slots are in use; waiting for one (599 s)
[bazel] all 3 Bazel server slots are in use; waiting for one (604 s)
[repository:command] running elapsedMs=700109
[bazel] all 3 Bazel server slots are in use; waiting for one (609 s)
This process (repository:build) made no progress for 10 min and has no live child: it exits now so that the leases it holds are released.
167 |     if(auxiliary&&commandSignal?.aborted)throw commandSignal.reason;
168 |     if(auxiliaryFailed)throw auxiliaryFailure;
169 |     if(auxiliary)requireIssuedResourceControlV1(options.control!);
170 |     if (stopped&&stopped!=="output delivery refused") throw new Error(`${label} stopped: ${stopped}`);
171 |     if(stopped==="output delivery refused"&&status.signal!==null)throw deliveryFailure;
172 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
                                                                 ^
OwnedCommandFailure: repository:command failed (124)
 outcome: {
  version: 1,
  status: 124,
  signal: null,
},
      at runOwnedCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:172:60)
Bun v1.4.2 (macOS arm64)
error: script "repo" exited with code 1
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1023-course10 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
