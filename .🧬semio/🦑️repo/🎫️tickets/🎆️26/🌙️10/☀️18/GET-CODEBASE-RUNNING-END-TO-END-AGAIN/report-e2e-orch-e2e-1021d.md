# 🧭 e2e-orch report e2e-1021d

Created 2026-10-21T16:28:03+02:00, written 2026-10-21T16:34:48+02:00, orchestrator pid 54246. Argv: `all --go --run-id e2e-1021d`.

Host: Darwin 27.0.0, 10 cores, parallel 3, Bazel tool `bazel`. Counts: no steps. Cancelled: False.

## Prerequisites

Verdict **FAIL** at 2026-10-21T16:34:48+02:00.

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
| OK | tectonic binary | aarch64-apple-darwin 0.16.9 sha256 matches the recorded extraction |
| OK | tex bundle | 2 populated bundle directories |
| OK | disk | 213 GB free (need 60) |
| OK | memory | 16.4 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| INFO | load | 28.3 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 3 product(s) |
| OK | ports play | [6033] free |
| OK | applied: play-bzl E1: activate-prototype cached | present |
| OK | applied: pg-chain: component cache test | present |
| OK | ports demo | [6029] free |
| OK | applied: demo-bzl: activate-prototype cached | present |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 12 running (shared fleet; the governor and the slots arbitrate): 1433       32:12 bun repo --tool=bazel run @teaching/proctor:build --configuration=release // 1435       32:12 bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script. // 35863       08:37 /bin/zsh .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-EN |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 16740 MB available of 32768 MB (floor 6553 MB, 2569 MB spare), pressure normal, swap 4711 MB, load 28.24, open files 7286, attached images 9; 5 running, 1 waiting |
| FAIL | graph targets | 20 of 20 target(s) missing from the generation: @semio-tech/mit-bestand-bericht:build, @semio-tech/mit-bestand-bericht:build-kompaktbericht, @semio-tech/mit-bestand-demonstrator:activate-prototype, @semio-tech/mit-bestand-demonstrator:prepare-prototype, @semio-tech/mit-bestand-demonstrator:serve, @semio-tech/mit-bestand-demonstrator:test-e2e, @semio-tech/mit-bestand-praesentation-projektetage:build, @semio-tech/semio-tech-play:activate-prototype, @semio-tech/semio-tech-play:prepare-prototype, @semio-tech/semio-tech-play:serve, @semio-tech/semio-tech-play:test-e2e, @teaching/architecture-course:build, @teaching/architecture-course:check, @teaching/architecture-course:dev, @teaching/architecture-course:dev-site, @teaching/architecture-course:test, @teaching/architecture-course:test-e2e, @teaching/architecture-course:typecheck, @teaching/proctor:build, @teaching/proctor:test |

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

Snapshots: none. Raw files: `🗑️generated/e2e-orch/e2e-1021d`.

## Resources

Max load None, min available memory None GB, min free disk None GB.

## Failures and warnings

none.

## Re-run

Continue this run: `run.sh all --run-id e2e-1021d --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
