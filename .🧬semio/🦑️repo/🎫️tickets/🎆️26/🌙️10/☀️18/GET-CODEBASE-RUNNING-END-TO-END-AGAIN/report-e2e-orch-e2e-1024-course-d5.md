# 🧭 e2e-orch report e2e-1024-course-d5

Created 2026-10-24T23:13:00+02:00, written 2026-10-24T23:25:10+02:00, orchestrator pid 13605. Argv: `course --phase e2e,probe --go --run-id e2e-1024-course-d5 --no-verify-targets`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1. Cancelled: False.

## Prerequisites

Verdict **OK** at 2026-10-24T23:13:00+02:00.

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
| OK | disk | 84 GB free (need 15) |
| OK | memory | 9.8 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 4.8 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 30 running (shared fleet; the governor and the slots arbitrate): 11919       02:26 bun repo --tool=bazel run @semio-tech/presentation:test // 11923       02:26 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 // 12088       02:15 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 10030 MB available of 32768 MB (floor 6553 MB, -13494 MB spare), pressure normal, swap 2022 MB, load 4.78, open files 4831, attached images 15; 4 running, 2 waiting |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | FAIL 11m18s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | FAIL | 1 | 11m18s | 6 | 4 | 0 | 2 | s0,s1 | 5 | exit code 1 |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course test-e2e: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 1706 MB spare above the 6553 MB floor, load 5.2; 3 running, 1 ahead)`
- course test-e2e: `[capacity] waiting 5s for run: not enough free memory (need 3584 MB; 857 MB spare above the 6553 MB floor, load 5.03; 3 running, 1 ahead)`
- course test-e2e: `[capacity] waiting 11s for run: not enough free memory (need 3584 MB; 899 MB spare above the 6553 MB floor, load 4.7; 3 running, 1 ahead)`
- course test-e2e: `[capacity] waiting 17s for run: not enough free memory (need 3584 MB; 766 MB spare above the 6553 MB floor, load 4.49; 3 running, 1 ahead)`
- course test-e2e: `[capacity] waiting 22s for run: an earlier request is still waiting (need 3584 MB; 832 MB spare above the 6553 MB floor, load 4.13; 3 running, 1 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-24T23:13:07+02:00, end@2026-10-24T23:25:10+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1024-course-d5`.

## Resources

Max load 15.0, min available memory 9.8 GB, min free disk 83.0 GB.

## Failures and warnings


### course e2e test-e2e: FAIL

- command: `bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh course --tool=bazel --release`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1024-course-d5/course/e2e-1-test-e2e.log`

```
  signal: null,
},
      at runOwnedCommand (semio-source:/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:328:13)
      at async <anonymous> (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🪵️failure-tail/🟦️.ts:115:20)
      at async retryOnConcurrentEditV1 (semio-source:/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts:129:20)
Bun v1.4.2 (macOS arm64)
170 |     if(auxiliary&&commandSignal?.aborted)throw commandSignal.reason;
171 |     if(auxiliaryFailed)throw auxiliaryFailure;
172 |     if(auxiliary)requireIssuedResourceControlV1(options.control!);
173 |     if (stopped&&stopped!=="output delivery refused") throw new Error(`${label} stopped: ${stopped}`);
174 |     if(stopped==="output delivery refused"&&status.signal!==null)throw deliveryFailure;
175 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
                                                                 ^
OwnedCommandFailure: repository:command failed (1)
 outcome: {
  version: 1,
  status: 1,
  signal: null,
},
      at runOwnedCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:175:60)
Bun v1.4.2 (macOS arm64)
error: script "repo" exited with code 1
[run-e2e 23:24:59] summary for course (@teaching/architecture-course) tool=--tool=bazel
  test-e2e           exit 1    678s
  logs: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/e2e-course-e2e-1024-course-d5-course-*.log
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1024-course-d5 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
