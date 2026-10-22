# 🧭 e2e-orch report e2e-1023-course1

Created 2026-10-23T00:55:17+02:00, written 2026-10-23T00:56:28+02:00, orchestrator pid 30315. Argv: `course --go --run-id e2e-1023-course1 --no-verify-targets`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: CANCELLED 1. Cancelled: True (CANCEL file).

## Prerequisites

Verdict **WARN** at 2026-10-23T00:55:17+02:00.

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
| OK | memory | 13.4 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| INFO | load | 27.5 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 26 running (shared fleet; the governor and the slots arbitrate): 27019       01:56 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 // 28728       01:14 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 // 28944       01:03 /Users/ueli/Documents/semio/.cache/b/t/m/r/toolchains/nightly-2026-07-20 |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 13732 MB available of 32768 MB (floor 6553 MB, 2802 MB spare), pressure normal, swap 7624 MB, load 27.53, open files 6368, attached images 11; 9 running, 2 waiting |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | FAIL 21s | - | - | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | CANCELLED | 1 | 21s | 0 | 0 | 0 | 0 | - | 5 | cancelled: CANCEL file |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course proctor-build: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; 1781 MB spare above the 6553 MB floor, load 23.65; 9 running, 1 ahead)`
- course proctor-build: `[capacity] waiting 5s for run: an earlier request is still waiting (need 3584 MB; 1595 MB spare above the 6553 MB floor, load 26.08; 9 running, 1 ahead)`
- course proctor-build: `[capacity] waiting 11s for run: not enough free memory (need 3584 MB; 489 MB spare above the 6553 MB floor, load 25.59; 10 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 17s for run: not enough free memory (need 3584 MB; -473 MB spare above the 6553 MB floor, load 24.66; 10 running, 0 ahead)`
- course proctor-build: `51 |  const controller=new AbortController(),handlers=(["SIGINT","SIGTERM"] as const).map(name=>[name,()=>controller.abort(Error(`Host capacity wait cancelled by ${name}`))] as const);`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-23T00:55:25+02:00, end@2026-10-23T00:56:28+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1023-course1`.

## Resources

Max load 27.5, min available memory 12.8 GB, min free disk 107.3 GB.

## Failures and warnings


### course cold proctor-build: CANCELLED

- command: `bun repo --tool=bazel run @teaching/proctor:build --configuration=release`
- note: cancelled: CANCEL file
- log: `🗑️generated/e2e-orch/e2e-1023-course1/course/cold-1-proctor-build.log`

```
# bun repo --tool=bazel run @teaching/proctor:build --configuration=release
# cwd /Users/ueli/Documents/semio
$ repo ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts repo "--tool=bazel" run @teaching/proctor:build "--configuration=release"
[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; 1781 MB spare above the 6553 MB floor, load 23.65; 9 running, 1 ahead)
[capacity] waiting 5s for run: an earlier request is still waiting (need 3584 MB; 1595 MB spare above the 6553 MB floor, load 26.08; 9 running, 1 ahead)
[capacity] waiting 11s for run: not enough free memory (need 3584 MB; 489 MB spare above the 6553 MB floor, load 25.59; 10 running, 0 ahead)
[capacity] waiting 17s for run: not enough free memory (need 3584 MB; -473 MB spare above the 6553 MB floor, load 24.66; 10 running, 0 ahead)
46 |  return [...kept,`${name}=${jobs}`,...tail];
47 | }
48 | 
49 | /** 🛑️ A control for the wait before any invocation exists: it is cancelled by an interrupt or termination signal and has no deadline of its own, so the invocation's budget starts when the grant does. */
50 | export function createCapacityControlV1(process_:Pick<NodeJS.Process,"once"|"removeListener">=process):Readonly<{control:ResourceLeaseControl;dispose:()=>void}>{
51 |  const controller=new AbortController(),handlers=(["SIGINT","SIGTERM"] as const).map(name=>[name,()=>controller.abort(Error(`Host capacity wait cancelled by ${name}`))] as const);
                                                                                                                           ^
error: Host capacity wait cancelled by SIGTERM
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts:51:119)
Bun v1.4.2 (macOS arm64)
error: script "repo" exited with code 1
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1023-course1 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
