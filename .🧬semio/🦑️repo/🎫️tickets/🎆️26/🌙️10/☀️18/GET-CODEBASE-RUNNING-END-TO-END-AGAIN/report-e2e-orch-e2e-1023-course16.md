# 🧭 e2e-orch report e2e-1023-course16

Created 2026-10-24T02:14:04+02:00, written 2026-10-24T02:23:17+02:00, orchestrator pid 30966. Argv: `course --go --run-id e2e-1023-course16 --no-verify-targets --phase cold,warm,e2e,probe`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1, PASS 1, SKIP-FAILED 11. Cancelled: False.

## Prerequisites

Verdict **OK** at 2026-10-24T02:14:04+02:00.

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
| OK | disk | 127 GB free (need 60) |
| OK | memory | 14.8 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 9.8 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 16 running (shared fleet; the governor and the slots arbitrate): 23195       06:04 sh .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-E // 23196       06:04 bun repo cargo --store=shared --configuration=prototype -- check --manif // 23198       06:04 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 14866 MB available of 32768 MB (floor 6553 MB, 1861 MB spare), pressure normal, swap 2900 MB, load 9.76, open files 6198, attached images 20; 3 running, 0 waiting |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | FAIL 8m26s | SKIP 0s | SKIP 0s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | PASS | 0 | 7m34s | 2 | 1 | 0 | 1 | s2 | 5 |  |
| cold | typecheck | FAIL | 1 | 52s | 0 | 0 | 0 | 0 | - | 0 | exit code 1 |
| cold | test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | check | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| cold | proctor-test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| warm | proctor-build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| warm | typecheck | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | check | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | proctor-test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| e2e | test-e2e | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | group 'core' it depends on failed |

## Warm assertions (Bazel cache hits)

| product | step | spawns | allowed | internal | cache hits | time | expected within | verdict |
|---|---|---|---|---|---|---|---|---|
| course | proctor-build | - | 0 | - | - | 0s | - | SKIP-FAILED an earlier step of group 'core' failed |
| course | typecheck | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | test | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | build | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | check | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | proctor-test | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |

## Queueing (slots, capacity)

- course proctor-build: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 1168 MB spare above the 6553 MB floor, load 9.89; 3 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 6s for run: not enough free memory (need 3584 MB; 2731 MB spare above the 6553 MB floor, load 11.02; 3 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 11s for run: not enough free memory (need 3584 MB; 2134 MB spare above the 6553 MB floor, load 10.77; 3 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 16s for run: not enough free memory (need 3584 MB; 2515 MB spare above the 6553 MB floor, load 12.55; 3 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 22s for run: not enough free memory (need 3584 MB; 615 MB spare above the 6553 MB floor, load 12.03; 3 running, 0 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-24T02:14:14+02:00, end@2026-10-24T02:23:17+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1023-course16`.

## Resources

Max load 15.5, min available memory 10.5 GB, min free disk 127.2 GB.

## Failures and warnings


### course cold typecheck: FAIL

- command: `bun repo --tool=bazel run @teaching/architecture-course:typecheck`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1023-course16/course/cold-2-typecheck.log`

```
118 |     const source = bytes.toString("utf8");
119 |       return tomlParsing.parse(source);
                               ^
SyntaxError: TOML Parse error: Cannot redefine key 'semio-framework-value-native-owned'
      at <anonymous> (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs:113:26)
      at collectPlaygroundCatalog (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs:30:22)
      at emojiProjectJsonNodes (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:108:495)
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

Continue this run: `run.sh all --run-id e2e-1023-course16 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
