# 🧭 e2e-orch report e2e-1021i

Created 2026-10-21T22:00:45+02:00, written 2026-10-21T22:01:52+02:00, orchestrator pid 54844. Argv: `course --go --run-id e2e-1021i`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: no steps. Cancelled: False.

## Prerequisites

Verdict **FAIL** at 2026-10-21T22:01:52+02:00.

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
| OK | disk | 207 GB free (need 60) |
| OK | memory | 18.1 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| INFO | load | 29.9 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 1 running (shared fleet; the governor and the slots arbitrate): 53776       01:47 bazel(7951b2611b401cb2) --add-opens=java.base/java.lang=ALL-UNNAMED -Xve |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 18467 MB available of 32768 MB (floor 6553 MB, 11914 MB spare), pressure normal, swap 5152 MB, load 28.14, open files 6527, attached images 9; 1 running, 0 waiting |
| FAIL | graph targets | 9 of 9 target(s) missing from the generation: @teaching/architecture-course:build, @teaching/architecture-course:check, @teaching/architecture-course:dev, @teaching/architecture-course:dev-site, @teaching/architecture-course:test, @teaching/architecture-course:test-e2e, @teaching/architecture-course:typecheck, @teaching/proctor:build, @teaching/proctor:test |

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

Snapshots: none. Raw files: `🗑️generated/e2e-orch/e2e-1021i`.

## Resources

Max load None, min available memory None GB, min free disk None GB.

## Failures and warnings

none.

## Re-run

Continue this run: `run.sh all --run-id e2e-1021i --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
