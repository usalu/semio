# 🧭 e2e-orch report e2e-1027-1d1

Created 2026-10-27T17:14:38+01:00, written 2026-10-27T18:16:36+01:00, orchestrator pid 80007. Argv: `course --phase e2e --course-topology dev --go --skip-preflight --run-id e2e-1027-1d1`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1. Cancelled: False.

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | FAIL 1h00m44s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | FAIL | 1 | 1h00m44s | 6 | 1 | 3 | 2 | s4 | 5 | exit code 1 |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course test-e2e: `[capacity] waiting 0s for exec: an earlier request is still waiting (need 3584 MB; 3236 MB spare above the 4096 MB floor, load 58.02; 3 running, 1 ahead)`
- course test-e2e: `[capacity] waiting 6s for exec: an earlier request is still waiting (need 3584 MB; 3337 MB spare above the 4096 MB floor, load 58.5; 3 running, 1 ahead)`
- course test-e2e: `[capacity] waiting 12s for exec: an earlier request is still waiting (need 3584 MB; 3158 MB spare above the 4096 MB floor, load 59.58; 3 running, 1 ahead)`
- course test-e2e: `[script] repository:command · Waiting for access to build-graph-state:/Users/ueli/Documents/semio/.cache/b/g/m:selected:7 (exclusive): position 2 of 2, behind the request of process 59839 (exclusive, `
- course test-e2e: `[capacity] keeping the run grant (1 job(s), 1241 MB reserved) for the command that replaces this one: it does not queue again`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-27T17:15:43+01:00, t30@2026-10-27T17:45:45+01:00, t60@2026-10-27T18:15:43+01:00, end@2026-10-27T18:16:36+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1027-1d1`.

## Resources

Max load 78.9, min available memory 7.5 GB, min free disk 136.9 GB.

## Failures and warnings


### course e2e test-e2e: FAIL

- command: `bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh course --tool=bazel --topology=dev`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1027-1d1/course/e2e-1-test-e2e.log`

```
  signal: null,
},
      at runOwnedCommand (semio-source:/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:329:13)
      at async <anonymous> (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🪵️failure-tail/🟦️.ts:115:20)
      at async retryOnConcurrentEditV1 (semio-source:/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts:152:20)
Bun v1.4.2 (macOS arm64)
173 |     if(auxiliary&&commandSignal?.aborted)throw commandSignal.reason;
174 |     if(auxiliaryFailed)throw auxiliaryFailure;
175 |     if(auxiliary)requireIssuedResourceControlV1(options.control!);
176 |     if (stopped&&stopped!=="output delivery refused") throw new Error(`${label} stopped: ${stopped}`);
177 |     if(stopped==="output delivery refused"&&status.signal!==null)throw deliveryFailure;
178 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
                                                                 ^
OwnedCommandFailure: repository:command failed (1)
 outcome: {
  version: 1,
  status: 1,
  signal: null,
},
      at runOwnedCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:178:60)
Bun v1.4.2 (macOS arm64)
error: script "repo" exited with code 1
[run-e2e 18:16:04] summary for course (@teaching/architecture-course) tool=--tool=bazel
  test-e2e           exit 1   3642s
  logs: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/e2e-course-e2e-1027-1d1-course-*.log
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1027-1d1 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
