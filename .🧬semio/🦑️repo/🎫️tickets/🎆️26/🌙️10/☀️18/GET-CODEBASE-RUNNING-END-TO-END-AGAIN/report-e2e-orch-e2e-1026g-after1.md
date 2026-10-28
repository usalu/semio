# 🧭 e2e-orch report e2e-1026g-after1

Created 2026-10-26T13:26:10+01:00, written 2026-10-26T14:10:34+01:00, orchestrator pid 96526. Argv: `course --phase e2e --go --run-id e2e-1026g-after1 --course-topology dev`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: no steps. Cancelled: True (SIGTERM).

## Prerequisites

Verdict **WARN** at 2026-10-26T14:10:27+01:00.

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
| OK | disk | 77 GB free (need 15) |
| OK | memory | 14.2 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 11.9 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 10 running (shared fleet; the governor and the slots arbitrate): 24384    01:20:49 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 // 87319       08:04 /private/var/folders/pm/zdg449hs6r33w_2gtsn23byc0000gn/T/reuse-ecRZj0/ch // 88445       07:04 bun repo --tool=bazel run @semio-tech/print-rs:test |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 6266 MB available of 32768 MB (floor 4096 MB, -6807 MB spare), pressure normal, swap 3148 MB, load 11.91, open files 6136, attached images 28; 2 running, 2 waiting |
| WARN | graph targets | could not read the published projects (exit 130); target names not verified |

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

Snapshots: start@2026-10-26T14:10:31+01:00, end@2026-10-26T14:10:34+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1026g-after1`.

## Resources

Max load 10.6, min available memory 18.2 GB, min free disk 73.6 GB.

## Failures and warnings

none.

## Re-run

Continue this run: `run.sh all --run-id e2e-1026g-after1 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
