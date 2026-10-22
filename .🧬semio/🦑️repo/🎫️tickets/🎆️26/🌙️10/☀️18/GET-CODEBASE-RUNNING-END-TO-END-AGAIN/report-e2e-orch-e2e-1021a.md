# 🧭 e2e-orch report e2e-1021a

Created 2026-10-21T12:08:28+02:00, written 2026-10-21T12:08:32+02:00, orchestrator pid 45097. Argv: `all --go --run-id e2e-1021a`.

Host: Darwin 27.0.0, 10 cores, parallel 3, Bazel tool `bazel`. Counts: no steps. Cancelled: False.

## Prerequisites

Verdict **FAIL** at 2026-10-21T12:08:32+02:00.

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
| OK | disk | 215 GB free (need 60) |
| OK | memory | 21.3 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| INFO | load | 32.9 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 3 product(s) |
| OK | ports play | [6033] free |
| OK | applied: play-bzl E1: activate-prototype cached | present |
| OK | applied: pg-chain: component cache test | present |
| OK | ports demo | [6029] free |
| OK | applied: demo-bzl: activate-prototype cached | present |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 3 running (shared fleet; the governor and the slots arbitrate): 10990 02:06:56 /bin/zsh .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-T // 45037    00:05 bun repo cargo --store=shared --configuration=prototype --parallelism=2 --  // 45039    00:05 bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 21651 MB available of 32768 MB (floor 6553 MB, 11514 MB spare), pressure normal, swap 7265 MB, load 32.95, open files 5975, attached images 9; 2 running, 0 waiting |
| FAIL | graph project @semio-tech/mit-bestand-bericht | exit 1; $ bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts repo "--tool=bazel" show project @semio-tech/mit-bestand-bericht "--graph-policy=current" 74 / } 75 / 76 / /** 🪪️ Admits o |
| FAIL | graph project @semio-tech/mit-bestand-demonstrator | exit 1; $ bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts repo "--tool=bazel" show project @semio-tech/mit-bestand-demonstrator "--graph-policy=current" 74 / } 75 / 76 / /** 🪪️ Adm |
| FAIL | graph project @semio-tech/mit-bestand-praesentation-projektetage | exit 1; $ bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts repo "--tool=bazel" show project @semio-tech/mit-bestand-praesentation-projektetage "--graph-policy=current" 74 / } 75 / 7 |
| FAIL | graph project @semio-tech/semio-tech-play | exit 1; $ bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts repo "--tool=bazel" show project @semio-tech/semio-tech-play "--graph-policy=current" 74 / } 75 / 76 / /** 🪪️ Admits only  |
| FAIL | graph project @teaching/architecture-course | exit 1; $ bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts repo "--tool=bazel" show project @teaching/architecture-course "--graph-policy=current" 74 / } 75 / 76 / /** 🪪️ Admits onl |
| FAIL | graph project @teaching/proctor | exit 1; $ bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts repo "--tool=bazel" show project @teaching/proctor "--graph-policy=current" 74 / } 75 / 76 / /** 🪪️ Admits only an already |

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

Snapshots: none. Raw files: `🗑️generated/e2e-orch/e2e-1021a`.

## Resources

Max load None, min available memory None GB, min free disk None GB.

## Failures and warnings

none.

## Re-run

Continue this run: `run.sh all --run-id e2e-1021a --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
