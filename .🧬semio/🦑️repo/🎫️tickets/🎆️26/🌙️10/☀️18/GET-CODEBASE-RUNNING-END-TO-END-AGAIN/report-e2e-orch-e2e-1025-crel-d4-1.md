# 🧭 e2e-orch report e2e-1025-crel-d4-1

Created 2026-10-25T02:33:39+01:00, written 2026-10-25T02:59:24+01:00, orchestrator pid 72700. Argv: `course --phase e2e --course-topology rehearsal --go --skip-preflight --run-id e2e-1025-crel-d4-1`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1. Cancelled: False.

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | FAIL 24m55s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | FAIL | 1 | 24m55s | 6 | 4 | 0 | 2 | s0 | 5 | exit code 1 |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course test-e2e: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 2328 MB spare above the 6553 MB floor, load 7.65; 7 running, 0 ahead)`
- course test-e2e: `[capacity] waiting 6s for run: not enough free memory (need 3584 MB; -624 MB spare above the 6553 MB floor, load 8.72; 7 running, 0 ahead)`
- course test-e2e: `[capacity] waiting 11s for run: not enough free memory (need 3584 MB; -1644 MB spare above the 6553 MB floor, load 9.14; 7 running, 0 ahead)`
- course test-e2e: `[capacity] waiting 18s for run: not enough free memory (need 3584 MB; -3472 MB spare above the 6553 MB floor, load 9.61; 7 running, 0 ahead)`
- course test-e2e: `[capacity] waiting 23s for run: not enough free memory (need 3584 MB; -4290 MB spare above the 6553 MB floor, load 10.6; 7 running, 0 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-25T02:33:44+01:00, end@2026-10-25T02:59:24+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1025-crel-d4-1`.

## Resources

Max load 27.7, min available memory 7.8 GB, min free disk 98.4 GB.

## Failures and warnings


### course e2e test-e2e: FAIL

- command: `bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh course --tool=bazel --topology=rehearsal --release`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1025-crel-d4-1/course/e2e-1-test-e2e.log`

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
[run-e2e 02:59:14] summary for course (@teaching/architecture-course) tool=--tool=bazel
  test-e2e           exit 1   1495s
  logs: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/e2e-course-e2e-1025-crel-d4-1-course-*.log
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1025-crel-d4-1 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
