# 🧭 e2e-orch report e2e-1025-crel2-d-b1

Created 2026-10-25T12:26:37+01:00, written 2026-10-25T14:54:06+01:00, orchestrator pid 83858. Argv: `course --phase e2e --course-topology rehearsal --go --skip-preflight --run-id e2e-1025-crel2-d-b1`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1. Cancelled: False.

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | FAIL 2h26m25s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | FAIL | 1 | 2h26m25s | 6 | 4 | 0 | 2 | s0 | 5 | exit code 1 |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course test-e2e: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; -1650 MB spare above the 6553 MB floor, load 10.82; 3 running, 14 ahead)`
- course test-e2e: `[capacity] waiting 6s for run: an earlier request is still waiting (need 3584 MB; -1821 MB spare above the 6553 MB floor, load 10.2; 3 running, 14 ahead)`
- course test-e2e: `[capacity] waiting 11s for run: an earlier request is still waiting (need 3584 MB; -1509 MB spare above the 6553 MB floor, load 9.7; 3 running, 14 ahead)`
- course test-e2e: `[capacity] waiting 16s for run: an earlier request is still waiting (need 3584 MB; -1709 MB spare above the 6553 MB floor, load 8.91; 3 running, 14 ahead)`
- course test-e2e: `[capacity] waiting 22s for run: an earlier request is still waiting (need 3584 MB; -1433 MB spare above the 6553 MB floor, load 8.52; 3 running, 14 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-25T12:26:42+01:00, t30@2026-10-25T12:56:57+01:00, t60@2026-10-25T13:27:09+01:00, t90@2026-10-25T13:57:17+01:00, t120@2026-10-25T14:27:34+01:00, end@2026-10-25T14:54:06+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1025-crel2-d-b1`.

## Resources

Max load 47.0, min available memory 5.8 GB, min free disk 56.2 GB.

## Failures and warnings


### course e2e test-e2e: FAIL

- command: `bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh course --tool=bazel --topology=rehearsal --release`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1025-crel2-d-b1/course/e2e-1-test-e2e.log`

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
[run-e2e 14:53:42] summary for course (@teaching/architecture-course) tool=--tool=bazel
  test-e2e           exit 1   8785s
  logs: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/e2e-course-e2e-1025-crel2-d-b1-course-*.log
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1025-crel2-d-b1 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
