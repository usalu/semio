# 🧭 e2e-orch report e2e-1023-course12

Created 2026-10-23T23:34:23+02:00, written 2026-10-23T23:43:49+02:00, orchestrator pid 53879. Argv: `course --go --run-id e2e-1023-course12 --no-verify-targets --phase cold,warm,e2e,probe`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: CANCELLED 1. Cancelled: True (SIGTERM).

## Prerequisites

Verdict **OK** at 2026-10-23T23:34:25+02:00.

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
| OK | disk | 121 GB free (need 60) |
| OK | memory | 13.0 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 8.2 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 44 running (shared fleet; the governor and the slots arbitrate): 3445       20:31 bun repo cargo --store=shared --configuration=prototype -- check --manife // 3446       20:31 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551 // 5892       19:38 bun repo cargo --store=shared --configuration=prototype -- test --no-fail |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 13231 MB available of 32768 MB (floor 6553 MB, 1046 MB spare), pressure normal, swap 10062 MB, load 8.17, open files 4812, attached images 18; 4 running, 11 waiting |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | FAIL 8m37s | - | - | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | CANCELLED | 1 | 8m37s | 0 | 0 | 0 | 0 | - | 5 | cancelled: SIGTERM |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course proctor-build: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; 1844 MB spare above the 6553 MB floor, load 11.42; 4 running, 13 ahead)`
- course proctor-build: `[capacity] waiting 5s for run: an earlier request is still waiting (need 3584 MB; 1750 MB spare above the 6553 MB floor, load 10.98; 4 running, 13 ahead)`
- course proctor-build: `[capacity] waiting 11s for run: an earlier request is still waiting (need 3584 MB; 2277 MB spare above the 6553 MB floor, load 10.74; 4 running, 13 ahead)`
- course proctor-build: `[capacity] waiting 17s for run: an earlier request is still waiting (need 3584 MB; 2022 MB spare above the 6553 MB floor, load 10.6; 4 running, 13 ahead)`
- course proctor-build: `[capacity] waiting 23s for run: an earlier request is still waiting (need 3584 MB; 1989 MB spare above the 6553 MB floor, load 10.15; 4 running, 13 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-23T23:34:36+02:00, end@2026-10-23T23:43:49+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1023-course12`.

## Resources

Max load 17.4, min available memory 10.1 GB, min free disk 120.6 GB.

## Failures and warnings


### course cold proctor-build: CANCELLED

- command: `bun repo --tool=bazel run @teaching/proctor:build --configuration=release`
- note: cancelled: SIGTERM
- log: `🗑️generated/e2e-orch/e2e-1023-course12/course/cold-1-proctor-build.log`

```
[capacity] waiting 445s for run: an earlier request is still waiting (need 3584 MB; -3979 MB spare above the 6553 MB floor, load 16.44; 5 running, 11 ahead)
[capacity] waiting 450s for run: an earlier request is still waiting (need 3584 MB; -4310 MB spare above the 6553 MB floor, load 16.16; 5 running, 11 ahead)
[capacity] waiting 455s for run: an earlier request is still waiting (need 3584 MB; -3690 MB spare above the 6553 MB floor, load 15.67; 5 running, 11 ahead)
[capacity] waiting 462s for run: an earlier request is still waiting (need 3584 MB; -3784 MB spare above the 6553 MB floor, load 15.7; 5 running, 11 ahead)
[capacity] waiting 467s for run: an earlier request is still waiting (need 3584 MB; -2844 MB spare above the 6553 MB floor, load 15.88; 5 running, 10 ahead)
[capacity] waiting 472s for run: an earlier request is still waiting (need 3584 MB; -3054 MB spare above the 6553 MB floor, load 15.17; 5 running, 10 ahead)
[capacity] waiting 478s for run: an earlier request is still waiting (need 3584 MB; -3254 MB spare above the 6553 MB floor, load 14.51; 5 running, 10 ahead)
[capacity] waiting 484s for run: an earlier request is still waiting (need 3584 MB; -2686 MB spare above the 6553 MB floor, load 13.91; 5 running, 10 ahead)
[capacity] waiting 489s for run: an earlier request is still waiting (need 3584 MB; -2854 MB spare above the 6553 MB floor, load 13.32; 5 running, 10 ahead)
[capacity] waiting 494s for run: an earlier request is still waiting (need 3584 MB; -2751 MB spare above the 6553 MB floor, load 12.82; 5 running, 10 ahead)
[capacity] waiting 500s for run: an earlier request is still waiting (need 3584 MB; -3221 MB spare above the 6553 MB floor, load 12.51; 5 running, 10 ahead)
[capacity] waiting 505s for run: an earlier request is still waiting (need 3584 MB; -3087 MB spare above the 6553 MB floor, load 11.99; 5 running, 10 ahead)
[capacity] waiting 511s for run: an earlier request is still waiting (need 3584 MB; -2607 MB spare above the 6553 MB floor, load 12.71; 5 running, 10 ahead)
[capacity] waiting 516s for run: an earlier request is still waiting (need 3584 MB; -2134 MB spare above the 6553 MB floor, load 13.29; 5 running, 10 ahead)
49 |  return [...kept,`${name}=${jobs}`,...tail];
50 | }
51 | 
52 | /** 🛑️ A control for the wait before any invocation exists: it is cancelled by an interrupt or termination signal and has no deadline of its own, so the invocation's budget starts when the grant does. */
53 | export function createCapacityControlV1(process_:Pick<NodeJS.Process,"once"|"removeListener">=process):Readonly<{control:ResourceLeaseControl;dispose:()=>void}>{
54 |  const controller=new AbortController(),handlers=(["SIGINT","SIGTERM"] as const).map(name=>[name,()=>controller.abort(Error(`Host capacity wait cancelled by ${name}`))] as const);
                                                                                                                           ^
error: Host capacity wait cancelled by SIGTERM
      at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts:54:119)
Bun v1.4.2 (macOS arm64)
error: script "repo" exited with code 1
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1023-course12 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
