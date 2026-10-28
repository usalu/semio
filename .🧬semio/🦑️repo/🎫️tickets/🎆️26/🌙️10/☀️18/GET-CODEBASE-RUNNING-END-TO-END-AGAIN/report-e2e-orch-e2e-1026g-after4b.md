# 🧭 e2e-orch report e2e-1026g-after4b

Created 2026-10-26T16:21:55+01:00, written 2026-10-26T16:53:18+01:00, orchestrator pid 44734. Argv: `course --phase e2e --go --skip-preflight --no-verify-targets --run-id e2e-1026g-after4b --course-topology dev`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1. Cancelled: False.

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | FAIL 30m34s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | FAIL | 1 | 30m34s | 5 | 2 | 1 | 2 | s0 | 0 | exit code 1 |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

no waiting lines recorded.

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-26T16:22:15+01:00, t30@2026-10-26T16:52:31+01:00, end@2026-10-26T16:53:18+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1026g-after4b`.

## Resources

Max load 64.3, min available memory 9.6 GB, min free disk 72.3 GB.

## Failures and warnings


### course e2e test-e2e: FAIL

- command: `bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh course --tool=bazel --topology=dev`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1026g-after4b/course/e2e-1-test-e2e.log`

```
  signal: null,
},
      at runOwnedCommand (semio-source:/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:328:13)
      at async <anonymous> (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🪵️failure-tail/🟦️.ts:115:20)
      at async retryOnConcurrentEditV1 (semio-source:/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts:151:20)
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
[run-e2e 16:53:10] summary for course (@teaching/architecture-course) tool=--tool=bazel
  test-e2e           exit 1   1834s
  logs: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/e2e-course-e2e-1026g-after4b-course-*.log
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1026g-after4b --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
