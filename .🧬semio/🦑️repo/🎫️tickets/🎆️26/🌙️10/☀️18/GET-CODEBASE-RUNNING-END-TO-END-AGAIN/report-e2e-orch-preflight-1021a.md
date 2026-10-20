# 🧭 e2e-orch report preflight-1021a

Created 2026-10-21T00:00:07+02:00, written 2026-10-21T00:00:11+02:00, orchestrator pid 58926. Argv: `all --phase preflight --run-id preflight-1021a`.

Host: Darwin 27.0.0, 10 cores, parallel 3, Bazel tool `bazel`. Counts: no steps. Cancelled: False.

## Prerequisites

Verdict **WARN** at 2026-10-21T00:00:11+02:00.

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
| OK | disk | 231 GB free (need 15) |
| OK | memory | 13.3 GB available of 32.0 GB (need 6) |
| WARN | load | 48.9 on 10 cores (warn above 20, fail above 50) |
| OK | bazel build slots | 3 slot(s) from the host budget for 3 product(s) |
| OK | ports play | [6033] free |
| OK | applied: play-bzl E1: activate-prototype cached | present |
| OK | applied: pg-chain: component cache test | present |
| OK | ports demo | [6029] free |
| OK | applied: demo-bzl: activate-prototype cached | present |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 11 running (shared fleet; the governor and the slots arbitrate): 3696 06:19:49 /Applications/Claude.app/Contents/Frameworks/Electron Framework.framework/He // 3712 06:19:49 /Applications/Claude.app/Contents/Frameworks/Claude Helper (Renderer).app/Co // 5897 06:13:24 /Applications/Claude.app/Contents/Helpers/disclaimer --pgroup -- /Users/ueli |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 14093 MB available of 32768 MB (floor 6553 MB, 7540 MB spare), pressure normal, swap 2485 MB, load 48.46, open files 8133, attached images 8; 2 running, 3 waiting |

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

Snapshots: none. Raw files: `🗑️generated/e2e-orch/preflight-1021a`.

## Resources

Max load None, min available memory None GB, min free disk None GB.

## Failures and warnings

none.
## Re-run

Continue this run: `run.sh all --run-id preflight-1021a --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
