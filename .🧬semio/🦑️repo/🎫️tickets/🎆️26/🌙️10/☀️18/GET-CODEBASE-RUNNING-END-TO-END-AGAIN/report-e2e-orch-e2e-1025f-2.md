# 🧭 e2e-orch report e2e-1025f-2

Created 2026-10-25T19:09:51+01:00, written 2026-10-25T19:52:58+01:00, orchestrator pid 12406. Argv: `course --phase e2e --course-topology dev --go --skip-preflight --run-id e2e-1025f-2`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1. Cancelled: False.

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | FAIL 42m05s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | FAIL | 1 | 42m05s | 0 | 0 | 0 | 0 | - | 5 | exit code 1 |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course test-e2e: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; 1753 MB spare above the 6553 MB floor, load 25.61; 3 running, 1 ahead)`
- course test-e2e: `[capacity] waiting 6s for run: an earlier request is still waiting (need 3584 MB; 1896 MB spare above the 6553 MB floor, load 28.21; 3 running, 1 ahead)`
- course test-e2e: `[capacity] waiting 12s for run: not enough free memory (need 3584 MB; 822 MB spare above the 6553 MB floor, load 27.36; 4 running, 0 ahead)`
- course test-e2e: `[capacity] waiting 18s for run: not enough free memory (need 3584 MB; 94 MB spare above the 6553 MB floor, load 29.97; 4 running, 0 ahead)`
- course test-e2e: `[capacity] waiting 23s for run: not enough free memory (need 3584 MB; -259 MB spare above the 6553 MB floor, load 29.33; 4 running, 0 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-25T19:10:45+01:00, t30@2026-10-25T19:40:06+01:00, end@2026-10-25T19:52:58+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1025f-2`.

## Resources

Max load 30.8, min available memory 5.3 GB, min free disk 50.1 GB.

## Failures and warnings


### course e2e test-e2e: FAIL

- command: `bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh course --tool=bazel --topology=dev`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1025f-2/course/e2e-1-test-e2e.log`

```
1107 |   const observed = await readCleanupJournal(parent.root, parent.maximumPhysicalBytes, check, () => control.remainingMilliseconds() ?? Number.MAX_SAFE_INTEGER), candidates = observed.state.allocations.filter((row) => row.child?.direct
1108 |     throw Error("Exact original parent child allocation required");
                 ^
error: Exact original parent child allocation required
      at readNativeStorageMountedChildV1 (semio-source:/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts:1101:11)
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
[run-e2e 19:52:37] summary for course (@teaching/architecture-course) tool=--tool=bazel
  test-e2e           exit 1   2524s
  logs: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/e2e-course-e2e-1025f-2-course-*.log
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1025f-2 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
