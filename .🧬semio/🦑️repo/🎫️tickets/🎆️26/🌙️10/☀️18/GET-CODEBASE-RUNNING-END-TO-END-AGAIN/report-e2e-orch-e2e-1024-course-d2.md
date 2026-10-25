# 🧭 e2e-orch report e2e-1024-course-d2

Created 2026-10-24T20:42:02+02:00, written 2026-10-24T22:54:11+02:00, orchestrator pid 40776. Argv: `course --phase cold,warm,e2e,probe --go --run-id e2e-1024-course-d2 --no-verify-targets`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 3, PASS 6, SKIP-FAILED 4. Cancelled: False.

## Prerequisites

Verdict **OK** at 2026-10-24T20:42:02+02:00.

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
| OK | disk | 84 GB free (need 60) |
| OK | memory | 15.7 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 11.3 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 21 running (shared fleet; the governor and the slots arbitrate): 28242       10:47 sh .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-E // 28243       10:47 bun repo cargo --store=shared --configuration=prototype -- test --manife // 28244       10:47 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 16092 MB available of 32768 MB (floor 6553 MB, 7919 MB spare), pressure normal, swap 2446 MB, load 11.32, open files 6216, attached images 15; 4 running, 0 waiting |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | PASS 1h26m18s | FAIL 19m05s | SKIP 0s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | PASS | 0 | 20m30s | 2 | 1 | 0 | 1 | s1 | 5 |  |
| cold | typecheck | PASS | 0 | 13m07s | 8 | 7 | 0 | 1 | s0,s3 | 5 |  |
| cold | test | PASS | 0 | 8m54s | 12 | 8 | 1 | 3 | s0 | 5 |  |
| cold | build | PASS | 0 | 27m40s | 8 | 7 | 0 | 1 | s1 | 5 |  |
| cold | check | PASS | 0 | 6m08s | 3 | 3 | 0 | 1 | s0 | 5 |  |
| cold | proctor-test | PASS | 0 | 10m00s | 5 | 4 | 0 | 2 | s0 | 0 |  |
| warm | proctor-build | FAIL | 0 | 9m45s | 2 | 1 | 0 | 1 | s2 | 5 | 1 spawns, at most 0 allowed; settle run 1 after a first warm run with work (1 spawns, at most 0 allowed) |
| warm | typecheck | FAIL | 0 | 8m38s | 4 | 7 | 0 | 1 | s2 | 5 | 7 spawns, at most 0 allowed; settle run 1 after a first warm run with work (7 spawns, at most 0 allowed) |
| warm | test | FAIL | 1 | 42s | 0 | 0 | 0 | 0 | - | 5 | exit code 1; settle run 1 after a first warm run with work (10 spawns, at most 0 allowed) |
| warm | build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| warm | check | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| warm | proctor-test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| e2e | test-e2e | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | group 'core' it depends on failed |

## Warm assertions (Bazel cache hits)

| product | step | spawns | allowed | internal | cache hits | time | expected within | verdict |
|---|---|---|---|---|---|---|---|---|
| course | proctor-build | 1 | 0 | 1 | 0 | 9m45s | 10m00s | FAIL 1 spawns, at most 0 allowed; settle run 1 after a first warm run with work (1 spawns, at most 0 allowed) |
| course | typecheck | 7 | 0 | 1 | 0 | 8m38s | 10m00s | FAIL 7 spawns, at most 0 allowed; settle run 1 after a first warm run with work (7 spawns, at most 0 allowed) |
| course | test | 0 | 0 | 0 | 0 | 42s | 10m00s | FAIL exit code 1; settle run 1 after a first warm run with work (10 spawns, at most 0 allowed) |
| course | build | - | 0 | - | - | 0s | - | SKIP-FAILED an earlier step of group 'core' failed |
| course | check | - | 0 | - | - | 0s | - | SKIP-FAILED an earlier step of group 'core' failed |
| course | proctor-test | - | 0 | - | - | 0s | - | SKIP-FAILED an earlier step of group 'core' failed |

## Queueing (slots, capacity)

- course proctor-build: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 455 MB spare above the 6553 MB floor, load 13.18; 4 running, 1 ahead)`
- course proctor-build: `[capacity] waiting 6s for run: not enough free memory (need 3584 MB; 135 MB spare above the 6553 MB floor, load 12.84; 4 running, 1 ahead)`
- course proctor-build: `[capacity] waiting 11s for run: not enough free memory (need 3584 MB; 226 MB spare above the 6553 MB floor, load 12.29; 4 running, 1 ahead)`
- course proctor-build: `[capacity] waiting 16s for run: an earlier request is still waiting (need 3584 MB; -1190 MB spare above the 6553 MB floor, load 11.95; 4 running, 1 ahead)`
- course proctor-build: `[capacity] waiting 22s for run: an earlier request is still waiting (need 3584 MB; -1288 MB spare above the 6553 MB floor, load 11.27; 4 running, 1 ahead)`
- course typecheck: `[capacity] waiting 0s for run: an earlier request is still waiting (need 5632 MB; -8719 MB spare above the 6553 MB floor, load 8.86; 6 running, 2 ahead)`
- course typecheck: `[capacity] waiting 5s for run: an earlier request is still waiting (need 5632 MB; -9892 MB spare above the 6553 MB floor, load 10.87; 6 running, 2 ahead)`
- course typecheck: `[capacity] waiting 10s for run: the host is swapping out too fast (need 5632 MB; -4868 MB spare above the 6553 MB floor, load 10.72; 6 running, 2 ahead)`
- course typecheck: `[capacity] waiting 16s for run: an earlier request is still waiting (need 5632 MB; -3383 MB spare above the 6553 MB floor, load 10.34; 6 running, 2 ahead)`
- course typecheck: `[capacity] waiting 21s for run: an earlier request is still waiting (need 5632 MB; -4845 MB spare above the 6553 MB floor, load 10.4; 6 running, 2 ahead)`
- course test: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; 5518 MB spare above the 6553 MB floor, load 10.17; 5 running, 3 ahead)`
- course test: `[capacity] waiting 5s for run: an earlier request is still waiting (need 3584 MB; -974 MB spare above the 6553 MB floor, load 9.68; 6 running, 2 ahead)`
- course test: `[capacity] waiting 11s for run: an earlier request is still waiting (need 3584 MB; -2202 MB spare above the 6553 MB floor, load 9.38; 6 running, 2 ahead)`
- course test: `[capacity] waiting 16s for run: an earlier request is still waiting (need 3584 MB; 2389 MB spare above the 6553 MB floor, load 9.51; 6 running, 2 ahead)`
- course test: `[capacity] waiting 22s for run: an earlier request is still waiting (need 3584 MB; 2659 MB spare above the 6553 MB floor, load 9.13; 6 running, 2 ahead)`
- course build: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; 5406 MB spare above the 6553 MB floor, load 9.9; 5 running, 3 ahead)`
- course build: `[capacity] waiting 6s for run: an earlier request is still waiting (need 3584 MB; 2729 MB spare above the 6553 MB floor, load 9.66; 5 running, 2 ahead)`
- course build: `[capacity] waiting 11s for run: an earlier request is still waiting (need 3584 MB; 836 MB spare above the 6553 MB floor, load 9.45; 5 running, 2 ahead)`
- course build: `[capacity] waiting 17s for run: an earlier request is still waiting (need 3584 MB; 1363 MB spare above the 6553 MB floor, load 9.37; 6 running, 1 ahead)`
- course build: `[capacity] waiting 23s for run: an earlier request is still waiting (need 3584 MB; 642 MB spare above the 6553 MB floor, load 9.18; 6 running, 1 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-24T20:42:09+02:00, t30@2026-10-24T21:12:11+02:00, t60@2026-10-24T21:42:11+02:00, t90@2026-10-24T22:12:11+02:00, t120@2026-10-24T22:42:13+02:00, end@2026-10-24T22:54:11+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1024-course-d2`.

## Resources

Max load 17.7, min available memory 5.7 GB, min free disk 75.2 GB.

## Failures and warnings


### course warm proctor-build: FAIL

- command: `bun repo --tool=bazel run @teaching/proctor:build --configuration=release`
- note: 1 spawns, at most 0 allowed; settle run 1 after a first warm run with work (1 spawns, at most 0 allowed)
- log: `🗑️generated/e2e-orch/e2e-1024-course-d2/course/warm-1-proctor-build.log`

```
[bazel:build] running elapsedMs=100017
[repository:command] running elapsedMs=380061
[bazel:build] running elapsedMs=110019
INFO: From @teaching/proctor:build:release:
stderr (/Users/ueli/Documents/semio/.cache/b/p/m/bazel/outputs/server/s2/execroot/_main/bazel-out/_tmp/actions/stderr-2) 1446326 exceeds maximum size of --experimental_ui_max_stdouterr_bytes=1048576 bytes; skipping
[@teaching/proctor:build:release] running elapsedMs=10003
[@teaching/proctor:build:release] running elapsedMs=20003
[@teaching/proctor:build:release] running elapsedMs=30005
[@teaching/proctor:build:release] running elapsedMs=40006
[@teaching/proctor:build:release] running elapsedMs=50008
[@teaching/proctor:build:release] running elapsedMs=60010
[@teaching/proctor:build:release] running elapsedMs=70011
[@teaching/proctor:build:release] running elapsedMs=80013
[@teaching/proctor:build:release] running elapsedMs=90015
INFO: Found 1 target...
Target //:t_7d24f6432dc80bf5 up-to-date:
  /Users/ueli/Documents/semio/.cache/b/p/m/bazel/outputs/server/s2/execroot/_main/bazel-out/darwin_arm64-fastbuild/bin/t_7d24f6432dc80bf5.snapshot
INFO: Elapsed time: 110.586s, Critical Path: 97.11s
INFO: 2 processes: 1 internal, 1 local.
INFO: Build completed successfully, 2 total actions
INFO: 
INFO: Build Event Protocol files produced successfully.
[repository:command] running elapsedMs=390061
[script] {"owner":"repository:build","command":"bazel:snapshot-restore","stage":"running"}
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
```

### course warm typecheck: FAIL

- command: `bun repo --tool=bazel run @teaching/architecture-course:typecheck`
- note: 7 spawns, at most 0 allowed; settle run 1 after a first warm run with work (7 spawns, at most 0 allowed)
- log: `🗑️generated/e2e-orch/e2e-1024-course-d2/course/warm-2-typecheck.log`

```
[script] {"owner":"repository:build","command":"bazel:snapshot-restore","stage":"running"}
[script] {"owner":"repository:build","command":"bazel:snapshot-restore","stage":"running"}
[script] {"owner":"repository:build","command":"bazel:snapshot-restore","stage":"running"}
[script] {"owner":"repository:build","command":"bazel:snapshot-restore","stage":"running"}
[script] {"owner":"repository:build","command":"owner-command","stage":"loading"}
[script] {"owner":"repository:build","command":"owner-command","stage":"running"}
[script] {"owner":"repository:build","command":"typecheck","stage":"loading"}
[script] {"owner":"repository:build","command":"typecheck","stage":"running"}
[script] {"owner":"repository:build","command":"typecheck:entry","stage":"running","activity":{"resource":"resource-retirement-inventory:128","mode":"shared","elapsedMs":1}}
[script] {"owner":"repository:build","command":"typecheck:entry","stage":"running","activity":{"resource":"claim-retirement-scan:128","mode":"exclusive","elapsedMs":69}}
[script] {"owner":"repository:build","command":"typecheck","stage":"running"}
[script] {"owner":"repository:build","command":"typecheck","stage":"running"}
[script] {"owner":"repository:build","command":"typecheck","stage":"complete"}
[script] {"owner":"repository:build","command":"typecheck","stage":"complete"}
[script] {"owner":"repository:build","command":"owner-command","stage":"complete"}
INFO: Found 1 target...
Target //:t_0d2cec7ea080488d up-to-date:
  /Users/ueli/Documents/semio/.cache/b/p/m/bazel/outputs/server/s2/execroot/_main/bazel-out/darwin_arm64-fastbuild/bin/t_0d2cec7ea080488d.snapshot
INFO: Elapsed time: 33.972s, Critical Path: 21.64s
INFO: 4 processes: 4 action cache hit, 1 internal, 1 local, 2 worker.
INFO: Build completed successfully, 4 total actions
INFO: 
INFO: Build Event Protocol files produced successfully.
[script] {"owner":"repository:build","command":"bazel:snapshot-restore","stage":"running"}
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
```

### course warm test: FAIL

- command: `bun repo --tool=bazel run @teaching/architecture-course:test -- quick`
- note: exit code 1; settle run 1 after a first warm run with work (10 spawns, at most 0 allowed)
- log: `🗑️generated/e2e-orch/e2e-1024-course-d2/course/warm-3-test.log`

```
60 |   requireIssuedResourceControlRefinementV1(options.control, custody.control);
61 |   const backing = await readNativePlatformImageBackingV1(image, custody.control), allocation = readNativeStorageImageReservationV1(reservation, reservationControl, image.port, backing), current = journal(image.root, image.maximumPhysic
62 |   requireNativePlatformImageWriterV1(image, custody);
63 |     throw Error("Sole exact committed recomputable image allocation required");
               ^
error: Sole exact committed recomputable image allocation required
      at captureNativeRecomputableRetirementV1 (semio-source:/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️whole/🟦️.ts:59:11)
Bun v1.4.2 (macOS arm64)
170 |     if(auxiliary&&commandSignal?.aborted)throw commandSignal.reason;
171 |     if(auxiliaryFailed)throw auxiliaryFailure;
172 |     if(auxiliary)requireIssuedResourceControlV1(options.control!);
173 |     if (stopped&&stopped!=="output delivery refused") throw new Error(`${label} stopped: ${stopped}`);
174 |     if(stopped==="output delivery refused"&&status.signal!==null)throw deliveryFailure;
175 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
                                                                 ^
OwnedCommandFailure: repository:command failed (1)
 outcome: {
  version: 1,
  status: 1,
  signal: null,
},
      at runOwnedCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:175:60)
      at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts:15:994)
Bun v1.4.2 (macOS arm64)
error: script "repo" exited with code 1
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1024-course-d2 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
