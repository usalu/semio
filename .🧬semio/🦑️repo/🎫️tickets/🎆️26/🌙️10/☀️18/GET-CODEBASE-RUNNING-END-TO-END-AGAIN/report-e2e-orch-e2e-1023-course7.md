# 🧭 e2e-orch report e2e-1023-course7

Created 2026-10-23T10:59:23+02:00, written 2026-10-23T13:57:22+02:00, orchestrator pid 9997. Argv: `course --go --run-id e2e-1023-course7 --no-verify-targets --phase cold`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1, PASS 1, SKIP-FAILED 4. Cancelled: False.

## Prerequisites

Verdict **OK** at 2026-10-23T10:59:24+02:00.

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
| OK | disk | 123 GB free (need 60) |
| OK | memory | 11.1 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| INFO | load | 20.0 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 49 running (shared fleet; the governor and the slots arbitrate): 6777       02:57 bun repo cargo --store=shared --configuration=prototype -- test --manifes // 6778       02:57 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551 // 8502       01:02 sh .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-EN |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 11251 MB available of 32768 MB (floor 6553 MB, 2415 MB spare), pressure normal, swap 6642 MB, load 20.01, open files 6441, attached images 16; 5 running, 9 waiting |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | FAIL 2h57m11s | - | - | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | PASS | 0 | 52m15s | 2 | 1 | 0 | 1 | s0 | 5 |  |
| cold | typecheck | FAIL | 1 | 2h04m56s | 0 | 0 | 0 | 0 | - | 5 | exit code 1 |
| cold | test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | check | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | proctor-test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course proctor-build: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; -2054 MB spare above the 6553 MB floor, load 21.02; 5 running, 9 ahead)`
- course proctor-build: `[capacity] waiting 5s for run: not enough free memory (need 3584 MB; -2488 MB spare above the 6553 MB floor, load 19.82; 5 running, 9 ahead)`
- course proctor-build: `[capacity] waiting 11s for run: not enough free memory (need 3584 MB; -2577 MB spare above the 6553 MB floor, load 18.71; 5 running, 9 ahead)`
- course proctor-build: `[capacity] waiting 16s for run: not enough free memory (need 3584 MB; -2391 MB spare above the 6553 MB floor, load 18.1; 5 running, 9 ahead)`
- course proctor-build: `[capacity] waiting 22s for run: an earlier request is still waiting (need 3584 MB; -2494 MB spare above the 6553 MB floor, load 17.13; 5 running, 9 ahead)`
- course typecheck: `[capacity] waiting 0s for run: an earlier request is still waiting (need 5632 MB; -1541 MB spare above the 6553 MB floor, load 11.07; 6 running, 6 ahead)`
- course typecheck: `[capacity] waiting 5s for run: an earlier request is still waiting (need 5632 MB; 2000 MB spare above the 6553 MB floor, load 10.74; 4 running, 6 ahead)`
- course typecheck: `[capacity] waiting 11s for run: an earlier request is still waiting (need 5632 MB; -2124 MB spare above the 6553 MB floor, load 13.16; 5 running, 5 ahead)`
- course typecheck: `[capacity] waiting 16s for run: an earlier request is still waiting (need 5632 MB; -3268 MB spare above the 6553 MB floor, load 14.51; 5 running, 5 ahead)`
- course typecheck: `[capacity] waiting 23s for run: an earlier request is still waiting (need 5632 MB; -4234 MB spare above the 6553 MB floor, load 15.35; 5 running, 5 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-23T10:59:31+02:00, t30@2026-10-23T11:29:38+02:00, t60@2026-10-23T11:59:42+02:00, t90@2026-10-23T12:29:36+02:00, t120@2026-10-23T12:59:36+02:00, t150@2026-10-23T13:29:39+02:00, end@2026-10-23T13:57:22+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1023-course7`.

## Resources

Max load 49.5, min available memory 5.6 GB, min free disk 108.6 GB.

## Failures and warnings


### course cold typecheck: FAIL

- command: `bun repo --tool=bazel run @teaching/architecture-course:typecheck`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1023-course7/course/cold-2-typecheck.log`

```
118 |   request.push(...buildOperationArgumentsV1(selected));
119 |     throw Error("Undeclared original Repo build selection");
                ^
error: Undeclared original Repo build selection
      at readOwnedBuildInvocationV1 (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🟦️.ts:119:11)
      at runAdmittedBuildInvocationV1 (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/⌨️command/⚙️runtime/🟦️.ts:19:3)
      at async withImplementationPinV1 (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔏️revision/🟨️.mjs:139:31)
Bun v1.4.2 (macOS arm64)
167 |     if(auxiliary&&commandSignal?.aborted)throw commandSignal.reason;
168 |     if(auxiliaryFailed)throw auxiliaryFailure;
169 |     if(auxiliary)requireIssuedResourceControlV1(options.control!);
170 |     if (stopped&&stopped!=="output delivery refused") throw new Error(`${label} stopped: ${stopped}`);
171 |     if(stopped==="output delivery refused"&&status.signal!==null)throw deliveryFailure;
172 |     if (status.signal !== null || status.code !== 0) throw new OwnedCommandFailure(label, { version: 1, status: status.code, signal: status.signal });
                                                                 ^
OwnedCommandFailure: repository:command failed (1)
 outcome: {
  version: 1,
  status: 1,
  signal: null,
},
      at runOwnedCommand (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts:172:60)
      at async <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🚪️command/🟦️.ts:15:994)
Bun v1.4.2 (macOS arm64)
error: script "repo" exited with code 1
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1023-course7 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
