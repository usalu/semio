# 🧭 e2e-orch report e2e-1024-course-d6

Created 2026-10-24T23:26:19+02:00, written 2026-10-25T00:12:42+02:00, orchestrator pid 29039. Argv: `course --phase e2e,probe --go --run-id e2e-1024-course-d6 --no-verify-targets --course-topology dev`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: PASS 1. Cancelled: False.

## Prerequisites

Verdict **OK** at 2026-10-24T23:26:19+02:00.

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
| OK | disk | 83 GB free (need 15) |
| OK | memory | 9.7 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 10.9 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 22 running (shared fleet; the governor and the slots arbitrate): 12988    01:49:16 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 // 14618       11:44 sh .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-E // 14621       11:44 bun repo --tool=bazel run @semio-tech/mit-bestand-bericht:test |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 9912 MB available of 32768 MB (floor 6553 MB, -2273 MB spare), pressure normal, swap 2364 MB, load 10.93, open files 5158, attached images 14; 4 running, 1 waiting |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | PASS 45m33s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | PASS | 0 | 45m33s | 6 | 5 | 0 | 1 | s1,s0 | 5 |  |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course test-e2e: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; 2124 MB spare above the 6553 MB floor, load 9; 4 running, 1 ahead)`
- course test-e2e: `[capacity] waiting 5s for run: an earlier request is still waiting (need 3584 MB; 2325 MB spare above the 6553 MB floor, load 8.38; 4 running, 1 ahead)`
- course test-e2e: `[capacity] waiting 11s for run: an earlier request is still waiting (need 3584 MB; 2274 MB spare above the 6553 MB floor, load 8.03; 4 running, 1 ahead)`
- course test-e2e: `[capacity] waiting 16s for run: an earlier request is still waiting (need 3584 MB; 2215 MB spare above the 6553 MB floor, load 7.71; 4 running, 1 ahead)`
- course test-e2e: `[capacity] waiting 22s for run: an earlier request is still waiting (need 3584 MB; 2172 MB spare above the 6553 MB floor, load 7.41; 4 running, 1 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-24T23:26:26+02:00, t30@2026-10-24T23:56:33+02:00, end@2026-10-25T00:12:42+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1024-course-d6`.

## Resources

Max load 18.4, min available memory 5.8 GB, min free disk 72.9 GB.

## Failures and warnings

none.

## Re-run

Continue this run: `run.sh all --run-id e2e-1024-course-d6 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
