# 🧭 e2e-orch report e2e-1023-course5

Created 2026-10-23T06:29:54+02:00, written 2026-10-23T08:47:11+02:00, orchestrator pid 3871. Argv: `course --go --run-id e2e-1023-course5 --no-verify-targets --phase cold`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1, PASS 1, SKIP-FAILED 4. Cancelled: False.

## Prerequisites

Verdict **OK** at 2026-10-23T06:29:54+02:00.

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
| OK | disk | 119 GB free (need 60) |
| OK | memory | 13.6 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 17.1 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 35 running (shared fleet; the governor and the slots arbitrate): 1424       00:51 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551 // 46697    01:23:40 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 // 61324       21:01 /Users/ueli/Documents/semio/.cache/b/t/m/bazel/install/8.4.2/darwin-arm6 |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 13917 MB available of 32768 MB (floor 6553 MB, 301 MB spare), pressure normal, swap 11987 MB, load 17.08, open files 5163, attached images 15; 6 running, 10 waiting |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | FAIL 2h16m29s | - | - | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | PASS | 0 | 1h26m30s | 2 | 1 | 0 | 1 | s2 | 5 |  |
| cold | typecheck | FAIL | 1 | 49m59s | 0 | 0 | 0 | 0 | - | 5 | exit code 1 |
| cold | test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | check | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | proctor-test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |

## Warm assertions (Bazel cache hits)

no warm step recorded.

## Queueing (slots, capacity)

- course proctor-build: `[capacity] waiting 0s for run: an earlier request is still waiting (need 3584 MB; -227 MB spare above the 6553 MB floor, load 16.97; 5 running, 11 ahead)`
- course proctor-build: `[capacity] waiting 5s for run: an earlier request is still waiting (need 3584 MB; -994 MB spare above the 6553 MB floor, load 16.49; 5 running, 11 ahead)`
- course proctor-build: `[capacity] waiting 11s for run: an earlier request is still waiting (need 3584 MB; -1464 MB spare above the 6553 MB floor, load 15.81; 5 running, 11 ahead)`
- course proctor-build: `[capacity] waiting 17s for run: an earlier request is still waiting (need 3584 MB; -1997 MB spare above the 6553 MB floor, load 15.58; 5 running, 11 ahead)`
- course proctor-build: `[capacity] waiting 22s for run: an earlier request is still waiting (need 3584 MB; -2208 MB spare above the 6553 MB floor, load 15.58; 5 running, 11 ahead)`
- course typecheck: `[capacity] waiting 0s for run: an earlier request is still waiting (need 5632 MB; -2046 MB spare above the 6553 MB floor, load 13.48; 4 running, 10 ahead)`
- course typecheck: `[capacity] waiting 5s for run: an earlier request is still waiting (need 5632 MB; 971 MB spare above the 6553 MB floor, load 13.84; 4 running, 10 ahead)`
- course typecheck: `[capacity] waiting 11s for run: an earlier request is still waiting (need 5632 MB; -318 MB spare above the 6553 MB floor, load 14.33; 4 running, 10 ahead)`
- course typecheck: `[capacity] waiting 16s for run: an earlier request is still waiting (need 5632 MB; -266 MB spare above the 6553 MB floor, load 14.23; 4 running, 10 ahead)`
- course typecheck: `[capacity] waiting 22s for run: an earlier request is still waiting (need 5632 MB; -1079 MB spare above the 6553 MB floor, load 14.66; 4 running, 10 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-23T06:30:09+02:00, t30@2026-10-23T07:00:05+02:00, t60@2026-10-23T07:30:03+02:00, t90@2026-10-23T08:00:09+02:00, t120@2026-10-23T08:30:05+02:00, end@2026-10-23T08:47:11+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1023-course5`.

## Resources

Max load 38.7, min available memory 5.7 GB, min free disk 105.4 GB.

## Failures and warnings


### course cold typecheck: FAIL

- command: `bun repo --tool=bazel run @teaching/architecture-course:typecheck`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1023-course5/course/cold-2-typecheck.log`

```
      at async tagProjectFailureV1 (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:45:18)
      at async <anonymous> (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:192:30)
      at async <anonymous> (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:218:19)
      at async run (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🟨️.mjs:179:22)
      at async <anonymous> (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:214:24)
      at async awaitGrowingV1 (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🟨️.mjs:60:19)
      at async emojiProjectJsonNodes (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:229:11)
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

Continue this run: `run.sh all --run-id e2e-1023-course5 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
