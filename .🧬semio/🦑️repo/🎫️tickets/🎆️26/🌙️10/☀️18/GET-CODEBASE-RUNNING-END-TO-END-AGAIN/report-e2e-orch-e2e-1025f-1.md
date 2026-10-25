# 🧭 e2e-orch report e2e-1025f-1

Created 2026-10-25T19:09:08+01:00, written 2026-10-25T19:09:48+01:00, orchestrator pid 11708. Argv: `course --phase all --course-topology dev --go --run-id e2e-1025f-1`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: no steps. Cancelled: True (SIGTERM).

## Prerequisites

Verdict **WARN** at 2026-10-25T19:09:28+01:00.

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
| OK | disk | 62 GB free (need 60) |
| OK | memory | 17.6 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 10.4 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 7 running (shared fleet; the governor and the slots arbitrate): 11504       00:19 bun repo --tool=bazel run-many --projects=@semio-tech/repo-dashboard-rs  // 11505       00:19 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 // 11546       00:15 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 17971 MB available of 32768 MB (floor 6553 MB, 11418 MB spare), pressure normal, swap 796 MB, load 10.44, open files 5565, attached images 24; 1 running, 0 waiting |
| WARN | graph targets | could not read the published projects (exit 1); target names not verified |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | - | - |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

no waiting lines recorded.

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-25T19:09:39+01:00, end@2026-10-25T19:09:48+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1025f-1`.

## Resources

Max load 10.5, min available memory 18.3 GB, min free disk 61.7 GB.

## Failures and warnings

none.

## Re-run

Continue this run: `run.sh all --run-id e2e-1025f-1 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
