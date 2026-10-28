# 🧭 e2e-orch report e2e-1025f-zd5

Created 2026-10-26T08:29:11+01:00, written 2026-10-26T08:54:07+01:00, orchestrator pid 12926. Argv: `course --phase e2e --course-topology dev --go --skip-preflight --run-id e2e-1025f-zd5`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1. Cancelled: False.

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | FAIL 23m39s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | FAIL | 1 | 23m39s | 0 | 0 | 0 | 0 | - | 5 | exit code 1 |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course test-e2e: `[capacity] waiting 0s for exec: not enough free memory (need 3584 MB; -1509 MB spare above the 4096 MB floor, load 9.55; 1 running, 3 ahead)`
- course test-e2e: `[capacity] waiting 5s for exec: an earlier request is still waiting (need 3584 MB; -2151 MB spare above the 4096 MB floor, load 9.26; 1 running, 3 ahead)`
- course test-e2e: `[capacity] waiting 12s for exec: an earlier request is still waiting (need 3584 MB; -4952 MB spare above the 4096 MB floor, load 9.88; 1 running, 3 ahead)`
- course test-e2e: `[capacity] waiting 17s for exec: an earlier request is still waiting (need 3584 MB; -799 MB spare above the 4096 MB floor, load 11.65; 2 running, 2 ahead)`
- course test-e2e: `[capacity] waiting 23s for exec: an earlier request is still waiting (need 3584 MB; 726 MB spare above the 4096 MB floor, load 12.48; 3 running, 1 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-26T08:29:12+01:00, end@2026-10-26T08:54:07+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1025f-zd5`.

## Resources

Max load 51.4, min available memory 6.2 GB, min free disk 63.1 GB.

## Failures and warnings


### course e2e test-e2e: FAIL

- command: `bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh course --tool=bazel --topology=dev`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1025f-zd5/course/e2e-1-test-e2e.log`

```
  version: 1,
  status: 1,
  signal: null,
},
      at runOwnedCommand (semio-source:/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:328:13)
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
[run-e2e 08:53:31] summary for course (@teaching/architecture-course) tool=--tool=bazel
  test-e2e           exit 1   1419s
  logs: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/e2e-course-e2e-1025f-zd5-course-*.log
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1025f-zd5 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
