# 🧭 e2e-orch report e2e-1023-course2

Created 2026-10-23T00:56:51+02:00, written 2026-10-23T01:42:54+02:00, orchestrator pid 32733. Argv: `course --go --run-id e2e-1023-course2 --no-verify-targets`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: RUNNING 1. Cancelled: False.

## Prerequisites

Verdict **WARN** at 2026-10-23T00:56:51+02:00.

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
| OK | disk | 107 GB free (need 60) |
| OK | memory | 12.8 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| INFO | load | 21.8 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 28 running (shared fleet; the governor and the slots arbitrate): 28728       02:48 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 // 28964       02:35 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 // 29852       01:54 /Users/ueli/Documents/semio/.cache/b/t/m/r/toolchains/nightly-2026-07-20 |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 13036 MB available of 32768 MB (floor 6553 MB, -2189 MB spare), pressure normal, swap 7608 MB, load 21.77, open files 7397, attached images 11; 10 running, 1 waiting |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | RUNNING 0s | - | - | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | RUNNING | None | - | - | - | - | - | - | 0 |  |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

no waiting lines recorded.

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `t30`
- capacity: 0 changed counter(s) between `start` and `t30`
- leases: 0 changed counter(s) between `start` and `t30`

Snapshots: start@2026-10-23T00:57:01+02:00, t30@2026-10-23T01:27:11+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1023-course2`.

## Resources

Max load 31.5, min available memory 5.7 GB, min free disk 102.3 GB.

## Failures and warnings

none.

## Re-run

Continue this run: `run.sh all --run-id e2e-1023-course2 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
