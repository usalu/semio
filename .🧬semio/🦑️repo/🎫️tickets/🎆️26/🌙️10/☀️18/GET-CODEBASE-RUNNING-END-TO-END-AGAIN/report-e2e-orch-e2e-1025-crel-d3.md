# 🧭 e2e-orch report e2e-1025-crel-d3

Created 2026-10-25T02:53:46+02:00, written 2026-10-25T02:02:11+01:00, orchestrator pid 39390. Argv: `course --phase e2e --course-topology rehearsal --go --run-id e2e-1025-crel-d3`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: no steps. Cancelled: False.

## Prerequisites

Verdict **FAIL** at 2026-10-25T02:02:11+01:00.

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
| OK | disk | 108 GB free (need 15) |
| OK | memory | 12.3 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 17.9 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| FAIL | ports course | in use: [6061, 8791] (stop the other serve; lsof -nP -iTCP:<port> -sTCP:LISTEN) |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 29 running (shared fleet; the governor and the slots arbitrate): 25235       12:06 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 // 32173       06:16 bazel(bd2dcb4e9d97a5c8) --add-opens=java.base/java.lang=ALL-UNNAMED -Xve // 32640       05:57 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 12794 MB available of 32768 MB (floor 6553 MB, 794 MB spare), pressure normal, swap 2797 MB, load 17.88, open files 7019, attached images 14; 4 running, 1 waiting |
| OK | graph targets | 2 project(s) and their 9 target name(s) found in lean `show projects/targets --json` (SEMIO_BUILD_GRAPH_POLICY=current) |

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

fewer than two snapshots (use the `start` and `end` snapshot files under the run directory when present).

Snapshots: none. Raw files: `🗑️generated/e2e-orch/e2e-1025-crel-d3`.

## Resources

Max load None, min available memory None GB, min free disk None GB.

## Failures and warnings

none.

## Re-run

Continue this run: `run.sh all --run-id e2e-1025-crel-d3 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
