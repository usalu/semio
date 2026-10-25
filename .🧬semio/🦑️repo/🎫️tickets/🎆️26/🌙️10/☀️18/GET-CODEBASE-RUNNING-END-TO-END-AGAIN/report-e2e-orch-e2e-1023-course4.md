# 🧭 e2e-orch report e2e-1023-course4

Created 2026-10-23T03:43:01+02:00, written 2026-10-23T06:29:09+02:00, orchestrator pid 98775. Argv: `course --go --run-id e2e-1023-course4 --no-verify-targets --phase cold || course --go --run-id e2e-1023-course4 --no-verify-targets --phase cold`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: PASS 1, SKIP-FAILED 5. Cancelled: False.

## Prerequisites

Verdict **OK** at 2026-10-23T06:28:16+02:00.

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
| OK | disk | 119 GB free (need 60) |
| OK | memory | 17.1 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 9.3 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 36 running (shared fleet; the governor and the slots arbitrate): 46697    01:22:02 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 // 61324       19:23 /Users/ueli/Documents/semio/.cache/b/t/m/bazel/install/8.4.2/darwin-arm6 // 61325       19:23 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 18159 MB available of 32768 MB (floor 6553 MB, -44 MB spare), pressure normal, swap 12155 MB, load 9.34, open files 5128, attached images 15; 5 running, 11 waiting |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | WARN 1h44m51s | - | - | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | PASS | 0 | 1h44m51s | 2 | 1 | 0 | 1 | s2 | 5 |  |
| cold | typecheck | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | check | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | proctor-test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course proctor-build: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; 3726 MB spare above the 6553 MB floor, load 7.09; 4 running, 12 ahead)`
- course proctor-build: `[capacity] waiting 5s for run: an earlier request is still waiting (need 3584 MB; 3431 MB spare above the 6553 MB floor, load 7.64; 4 running, 12 ahead)`
- course proctor-build: `[capacity] waiting 11s for run: an earlier request is still waiting (need 3584 MB; 3536 MB spare above the 6553 MB floor, load 7.69; 4 running, 12 ahead)`
- course proctor-build: `[capacity] waiting 17s for run: an earlier request is still waiting (need 3584 MB; 3363 MB spare above the 6553 MB floor, load 7.71; 4 running, 12 ahead)`
- course proctor-build: `[capacity] waiting 23s for run: an earlier request is still waiting (need 3584 MB; 3172 MB spare above the 6553 MB floor, load 8.06; 4 running, 12 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-23T03:43:09+02:00, t30@2026-10-23T04:13:19+02:00, t60@2026-10-23T04:43:22+02:00, t90@2026-10-23T05:13:10+02:00, t120@2026-10-23T05:43:14+02:00, t150@2026-10-23T06:13:15+02:00, end@2026-10-23T06:26:53+02:00, start@2026-10-23T06:28:31+02:00, end@2026-10-23T06:29:09+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1023-course4`.

## Resources

Max load 26.7, min available memory 5.6 GB, min free disk 100.2 GB.

## Failures and warnings

none.

## Re-run

Continue this run: `run.sh all --run-id e2e-1023-course4 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
