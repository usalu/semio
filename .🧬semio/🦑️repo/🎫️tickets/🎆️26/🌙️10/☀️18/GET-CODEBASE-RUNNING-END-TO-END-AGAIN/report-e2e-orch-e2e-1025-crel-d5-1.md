# 🧭 e2e-orch report e2e-1025-crel-d5-1

Created 2026-10-25T03:22:41+01:00, written 2026-10-25T03:24:42+01:00, orchestrator pid 10553. Argv: `course --phase e2e --course-topology rehearsal --go --skip-preflight --run-id e2e-1025-crel-d5-1`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1. Cancelled: False.

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | FAIL 1m10s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | FAIL | 1 | 1m10s | 0 | 0 | 0 | 0 | - | 5 | exit code 1 |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course test-e2e: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; 3159 MB spare above the 6553 MB floor, load 5.6; 3 running, 2 ahead)`
- course test-e2e: `[capacity] waiting 6s for run: an earlier request is still waiting (need 3584 MB; 3142 MB spare above the 6553 MB floor, load 5.47; 3 running, 2 ahead)`
- course test-e2e: `[capacity] waiting 11s for run: an earlier request is still waiting (need 3584 MB; 3099 MB spare above the 6553 MB floor, load 5.43; 3 running, 2 ahead)`
- course test-e2e: `[capacity] waiting 16s for run: an earlier request is still waiting (need 3584 MB; 3123 MB spare above the 6553 MB floor, load 5.24; 3 running, 2 ahead)`
- course test-e2e: `[capacity] waiting 22s for run: an earlier request is still waiting (need 3584 MB; 3111 MB spare above the 6553 MB floor, load 5.06; 3 running, 2 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-25T03:22:46+01:00, end@2026-10-25T03:24:42+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1025-crel-d5-1`.

## Resources

Max load 8.9, min available memory 10.3 GB, min free disk 107.3 GB.

## Failures and warnings


### course e2e test-e2e: FAIL

- command: `bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh course --tool=bazel --topology=rehearsal --release`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1025-crel-d5-1/course/e2e-1-test-e2e.log`

```
error: Current parser Source-owner custody changed
      at <anonymous> (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚀️bootstrap/🟦️.ts:238:13)
      at async settleConsumers (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts:229:19)
      at async <anonymous> (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts:431:25)
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
      at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts:15:994)
Bun v1.4.2 (macOS arm64)
error: script "repo" exited with code 1
[run-e2e 03:24:31] summary for course (@teaching/architecture-course) tool=--tool=bazel
  test-e2e           exit 1     70s
  logs: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/e2e-course-e2e-1025-crel-d5-1-course-*.log
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1025-crel-d5-1 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
