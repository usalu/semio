# 🧭 e2e-orch report e2e-1022-course3b

Created 2026-10-22T21:44:48+02:00, written 2026-10-22T22:57:04+02:00, orchestrator pid 42003. Argv: `course --go --run-id e2e-1022-course3b`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: CANCELLED 1, PASS 3. Cancelled: True (SIGTERM).

## Prerequisites

Verdict **WARN** at 2026-10-22T22:08:47+02:00.

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
| OK | disk | 105 GB free (need 60) |
| OK | memory | 17.2 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| INFO | load | 23.6 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 40 running (shared fleet; the governor and the slots arbitrate): 607       31:03 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e // 2424       29:11 bun repo cargo --store=shared --configuration=prototype -- check -p semio // 2425       29:11 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551 |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 17514 MB available of 32768 MB (floor 6553 MB, 1220 MB spare), pressure normal, swap 9615 MB, load 23.63, open files 4828, attached images 27; 5 running, 9 waiting |
| OK | graph targets | 2 project(s) and their 9 target name(s) found in lean `show projects/targets --json` (SEMIO_BUILD_GRAPH_POLICY=current) |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | FAIL 47m29s | - | - | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | PASS | 0 | 15m15s | 2 | 1 | 0 | 1 | s1 | 5 |  |
| cold | typecheck | PASS | 0 | 10m43s | 8 | 6 | 1 | 1 | s1 | 5 |  |
| cold | test | PASS | 0 | 3m10s | 5 | 10 | 0 | 2 | s1 | 5 |  |
| cold | build | CANCELLED | -15 | 18m21s | 0 | 0 | 0 | 0 | s1 | 0 | cancelled: SIGTERM |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course proctor-build: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 582 MB spare above the 6553 MB floor, load 25.2; 5 running, 2 ahead)`
- course proctor-build: `[capacity] waiting 6s for run: not enough free memory (need 3584 MB; 763 MB spare above the 6553 MB floor, load 24.8; 5 running, 2 ahead)`
- course proctor-build: `[capacity] waiting 12s for run: an earlier request is still waiting (need 3584 MB; -650 MB spare above the 6553 MB floor, load 24.17; 5 running, 2 ahead)`
- course proctor-build: `[capacity] waiting 18s for run: an earlier request is still waiting (need 3584 MB; 3233 MB spare above the 6553 MB floor, load 23.28; 5 running, 2 ahead)`
- course proctor-build: `[capacity] waiting 23s for run: an earlier request is still waiting (need 3584 MB; 2398 MB spare above the 6553 MB floor, load 22.13; 5 running, 2 ahead)`
- course typecheck: `[capacity] waiting 0s for run: an earlier request is still waiting (need 5632 MB; 837 MB spare above the 6553 MB floor, load 10.03; 3 running, 2 ahead)`
- course typecheck: `[capacity] waiting 6s for run: an earlier request is still waiting (need 5632 MB; -968 MB spare above the 6553 MB floor, load 9.71; 4 running, 1 ahead)`
- course typecheck: `[capacity] waiting 11s for run: an earlier request is still waiting (need 5632 MB; -2360 MB spare above the 6553 MB floor, load 12.3; 4 running, 1 ahead)`
- course typecheck: `[capacity] waiting 17s for run: an earlier request is still waiting (need 5632 MB; 90 MB spare above the 6553 MB floor, load 13.63; 4 running, 1 ahead)`
- course typecheck: `[capacity] waiting 22s for run: an earlier request is still waiting (need 5632 MB; -1089 MB spare above the 6553 MB floor, load 13.74; 4 running, 1 ahead)`
- course test: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 3054 MB spare above the 6553 MB floor, load 21.24; 3 running, 2 ahead)`
- course test: `[capacity] waiting 6s for run: not enough free memory (need 3584 MB; 174 MB spare above the 6553 MB floor, load 20.82; 5 running, 0 ahead)`
- course test: `[capacity] waiting 11s for run: not enough free memory (need 3584 MB; -4214 MB spare above the 6553 MB floor, load 20.03; 5 running, 0 ahead)`
- course test: `[capacity] waiting 17s for run: not enough free memory (need 3584 MB; 1452 MB spare above the 6553 MB floor, load 18.91; 5 running, 0 ahead)`
- course test: `[capacity] waiting 22s for run: not enough free memory (need 3584 MB; 1631 MB spare above the 6553 MB floor, load 18.12; 5 running, 0 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-22T22:08:55+02:00, t30@2026-10-22T22:39:15+02:00, end@2026-10-22T22:57:04+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1022-course3b`.

## Resources

Max load 31.0, min available memory 5.8 GB, min free disk 103.6 GB.

## Failures and warnings


### course cold build: CANCELLED

- command: `bun repo --tool=bazel run @teaching/architecture-course:build --configuration=release`
- note: cancelled: SIGTERM
- log: `🗑️generated/e2e-orch/e2e-1022-course3b/course/cold-4-build.log`

```
[repository:command] running elapsedMs=980384
[bazel:build] running elapsedMs=780194
[repository:command] running elapsedMs=990397
[bazel:build] running elapsedMs=790197
[repository:command] running elapsedMs=1000399
[bazel:build] running elapsedMs=800200
[repository:command] running elapsedMs=1010402
[5 / 8] @semio-tech/ui-contract-rs:generate:release; 101s disk-cache, local
[bazel:build] running elapsedMs=810202
[repository:command] running elapsedMs=1020404
[bazel:build] running elapsedMs=820204
[repository:command] running elapsedMs=1030406
[bazel:build] running elapsedMs=830207
[repository:command] running elapsedMs=1040409
[bazel:build] running elapsedMs=840209
[repository:command] running elapsedMs=1050410
[bazel:build] running elapsedMs=850210
[repository:command] running elapsedMs=1060413
[bazel:build] running elapsedMs=860211
[repository:command] running elapsedMs=1070415
[5 / 8] @semio-tech/ui-contract-rs:generate:release; 162s disk-cache, local
[bazel:build] running elapsedMs=870212
[repository:command] running elapsedMs=1080416
[bazel:build] running elapsedMs=880221
[repository:command] running elapsedMs=1090419
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1022-course3b --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
