# 🧭 e2e-orch report e2e-1025f-e1

Created 2026-10-25T21:52:24+01:00, written 2026-10-26T02:20:42+01:00, orchestrator pid 68142. Argv: `course --phase e2e --course-topology dev --go --skip-preflight --run-id e2e-1025f-e1`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1. Cancelled: False.

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | FAIL 4h27m28s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | FAIL | 1 | 4h27m28s | 6 | 4 | 0 | 2 | s2 | 5 | exit code 1 |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course test-e2e: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; -2161 MB spare above the 6553 MB floor, load 22.9; 11 running, 2 ahead)`
- course test-e2e: `[capacity] waiting 5s for run: not enough free memory (need 3584 MB; -3240 MB spare above the 6553 MB floor, load 23.79; 11 running, 2 ahead)`
- course test-e2e: `[capacity] waiting 10s for run: not enough free memory (need 3584 MB; -3652 MB spare above the 6553 MB floor, load 23.88; 11 running, 2 ahead)`
- course test-e2e: `[capacity] waiting 16s for run: not enough free memory (need 3584 MB; -3837 MB spare above the 6553 MB floor, load 25.64; 11 running, 2 ahead)`
- course test-e2e: `[capacity] waiting 21s for run: not enough free memory (need 3584 MB; -5099 MB spare above the 6553 MB floor, load 25.11; 11 running, 2 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-25T21:52:40+01:00, t30@2026-10-25T22:22:42+01:00, t60@2026-10-25T22:52:38+01:00, t90@2026-10-25T23:22:39+01:00, t120@2026-10-25T23:52:45+01:00, t150@2026-10-26T00:22:47+01:00, t180@2026-10-26T00:52:45+01:00, t210@2026-10-26T01:22:46+01:00, t240@2026-10-26T01:52:49+01:00, end@2026-10-26T02:20:42+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1025f-e1`.

## Resources

Max load 35.0, min available memory 5.7 GB, min free disk 45.4 GB.

## Failures and warnings


### course e2e test-e2e: FAIL

- command: `bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh course --tool=bazel --topology=dev`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1025f-e1/course/e2e-1-test-e2e.log`

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
[run-e2e 02:20:33] summary for course (@teaching/architecture-course) tool=--tool=bazel
  test-e2e           exit 1   16047s
  logs: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/e2e-course-e2e-1025f-e1-course-*.log
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1025f-e1 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
