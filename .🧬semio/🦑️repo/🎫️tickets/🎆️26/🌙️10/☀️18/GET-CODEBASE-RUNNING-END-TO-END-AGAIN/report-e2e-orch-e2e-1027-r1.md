# 🧭 e2e-orch report e2e-1027-r1

Created 2026-10-27T20:19:03+01:00, written 2026-10-27T20:40:58+01:00, orchestrator pid 24358. Argv: `course --phase e2e --course-topology rehearsal --go --skip-preflight --run-id e2e-1027-r1`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1. Cancelled: False.

## Prerequisites

not run in this invocation.

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | - | - | FAIL 21m02s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| e2e | test-e2e | FAIL | 1 | 21m02s | 6 | 4 | 0 | 2 | s4 | 5 | exit code 1 |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course test-e2e: `[capacity] waiting 0s for run: not enough free memory (need 1241 MB; -2072 MB spare above the 4096 MB floor, load 28.15; 4 running, 0 ahead); behind check,build,test (running, 1 job(s), 3746 MB), grap`
- course test-e2e: `[capacity] waiting 6s for run: not enough free memory (need 1241 MB; -2621 MB spare above the 4096 MB floor, load 27.18; 4 running, 0 ahead); behind check,build,test (running, 1 job(s), 3840 MB), grap`
- course test-e2e: `[capacity] waiting 11s for run: not enough free memory (need 1241 MB; -1680 MB spare above the 4096 MB floor, load 26.77; 4 running, 0 ahead); behind check,build,test (running, 1 job(s), 3840 MB), gra`
- course test-e2e: `[capacity] waiting 16s for run: not enough free memory (need 1241 MB; -2797 MB spare above the 4096 MB floor, load 28.07; 4 running, 0 ahead); behind check,build,test (running, 1 job(s), 3840 MB), gra`
- course test-e2e: `[capacity] waiting 22s for run: not enough free memory (need 1241 MB; 546 MB spare above the 4096 MB floor, load 27.26; 4 running, 0 ahead); behind check,build,test (running, 1 job(s), 3919 MB), @semi`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-27T20:19:32+01:00, end@2026-10-27T20:40:58+01:00. Raw files: `🗑️generated/e2e-orch/e2e-1027-r1`.

## Resources

Max load 43.1, min available memory 10.3 GB, min free disk 136.7 GB.

## Failures and warnings


### course e2e test-e2e: FAIL

- command: `bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh course --tool=bazel --topology=rehearsal --release`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1027-r1/course/e2e-1-test-e2e.log`

```
  signal: null,
},
      at runOwnedCommand (semio-source:/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:342:13)
      at async <anonymous> (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🪵️failure-tail/🟦️.ts:115:20)
      at async retryOnConcurrentEditV1 (semio-source:/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts:152:20)
Bun v1.4.2 (macOS arm64)
183 |     if(auxiliary&&commandSignal?.aborted)throw commandSignal.reason;
184 |     if(auxiliaryFailed)throw auxiliaryFailure;
185 |     if(auxiliary)requireIssuedResourceControlV1(options.control!);
186 |     if (stopped&&stopped!=="output delivery refused") throw new Error(`${label} stopped: ${stopped}`);
187 |     if(stopped==="output delivery refused"&&status.signal!==null)throw deliveryFailure;
188 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
                                                                 ^
OwnedCommandFailure: repository:command failed (1)
 outcome: {
  version: 1,
  status: 1,
  signal: null,
},
      at runOwnedCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:188:60)
Bun v1.4.2 (macOS arm64)
error: script "repo" exited with code 1
[run-e2e 20:40:46] summary for course (@teaching/architecture-course) tool=--tool=bazel
  test-e2e           exit 1   1262s
  logs: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/e2e-course-e2e-1027-r1-course-*.log
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1027-r1 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
