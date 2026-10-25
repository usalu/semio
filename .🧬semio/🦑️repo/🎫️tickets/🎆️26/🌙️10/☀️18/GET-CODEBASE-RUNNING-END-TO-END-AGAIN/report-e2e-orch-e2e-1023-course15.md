# 🧭 e2e-orch report e2e-1023-course15

Created 2026-10-24T01:48:05+02:00, written 2026-10-24T02:12:46+02:00, orchestrator pid 98388. Argv: `course --go --run-id e2e-1023-course15 --no-verify-targets --phase cold,warm,e2e,probe`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 1, PASS 4, SKIP-FAILED 8. Cancelled: False.

## Prerequisites

Verdict **OK** at 2026-10-24T01:48:05+02:00.

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
| OK | disk | 124 GB free (need 60) |
| OK | memory | 18.4 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 12.5 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 17 running (shared fleet; the governor and the slots arbitrate): 13341    01:35:17 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 // 58007    01:16:32 bazel(4d9d1194ea484fc7) --add-opens=java.base/java.lang=ALL-UNNAMED -Xve // 58073    01:16:30 /Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/baze |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 19023 MB available of 32768 MB (floor 6553 MB, 4909 MB spare), pressure normal, swap 4570 MB, load 12.5, open files 5358, attached images 20; 3 running, 0 waiting |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | FAIL 23m53s | SKIP 0s | SKIP 0s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | PASS | 0 | 8m00s | 2 | 1 | 0 | 1 | s2 | 5 |  |
| cold | typecheck | PASS | 0 | 5m23s | 8 | 7 | 0 | 1 | s2 | 3 |  |
| cold | test | PASS | 0 | 1m20s | 11 | 4 | 6 | 2 | s1 | 0 |  |
| cold | build | PASS | 0 | 6m17s | 8 | 7 | 0 | 1 | s2 | 5 |  |
| cold | check | FAIL | 1 | 2m51s | 0 | 0 | 0 | 0 | - | 5 | exit code 1 |
| cold | proctor-test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| warm | proctor-build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| warm | typecheck | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| warm | test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| warm | build | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | an earlier step of group 'core' failed |
| warm | check | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| warm | proctor-test | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | the cold run of this step did not pass |
| e2e | test-e2e | SKIP-FAILED | None | 0s | - | - | - | - | - | 0 | group 'core' it depends on failed |

## Warm assertions (Bazel cache hits)

| product | step | spawns | allowed | internal | cache hits | time | expected within | verdict |
|---|---|---|---|---|---|---|---|---|
| course | proctor-build | - | 0 | - | - | 0s | - | SKIP-FAILED an earlier step of group 'core' failed |
| course | typecheck | - | 0 | - | - | 0s | - | SKIP-FAILED an earlier step of group 'core' failed |
| course | test | - | 0 | - | - | 0s | - | SKIP-FAILED an earlier step of group 'core' failed |
| course | build | - | 0 | - | - | 0s | - | SKIP-FAILED an earlier step of group 'core' failed |
| course | check | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |
| course | proctor-test | - | 0 | - | - | 0s | - | SKIP-FAILED the cold run of this step did not pass |

## Queueing (slots, capacity)

- course proctor-build: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 1898 MB spare above the 6553 MB floor, load 12.88; 4 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 6s for run: not enough free memory (need 3584 MB; 2378 MB spare above the 6553 MB floor, load 12.97; 4 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 12s for run: not enough free memory (need 3584 MB; 2036 MB spare above the 6553 MB floor, load 12.94; 4 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 17s for run: not enough free memory (need 3584 MB; 812 MB spare above the 6553 MB floor, load 12.71; 4 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 22s for run: not enough free memory (need 3584 MB; 549 MB spare above the 6553 MB floor, load 12.25; 4 running, 0 ahead)`
- course typecheck: `[capacity] waiting 0s for exec: not enough free memory (need 3584 MB; 955 MB spare above the 6553 MB floor, load 16.01; 5 running, 0 ahead)`
- course typecheck: `[capacity] waiting 6s for exec: not enough free memory (need 3584 MB; 1077 MB spare above the 6553 MB floor, load 16.17; 5 running, 0 ahead)`
- course typecheck: `[capacity] waiting 12s for exec: not enough free memory (need 3584 MB; 1577 MB spare above the 6553 MB floor, load 15.99; 5 running, 0 ahead)`
- course build: `[script] cargo:typegen · Waiting for access to repo:compiler-writer:/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m (exclusive): position 4 of 4, behind the request of process 8581 (sh`
- course build: `[script] cargo:typegen · Waiting for access to repo:compiler-writer:/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m (exclusive): position 4 of 4, behind the request of process 8581 (sh`
- course build: `[script] cargo:typegen · Waiting for access to repo:compiler-writer:/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m (exclusive): position 4 of 4, behind the request of process 8581 (sh`
- course build: `[script] cargo:typegen · Waiting for access to repo:compiler-writer:/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m (exclusive): position 4 of 4, behind the request of process 8581 (sh`
- course build: `[script] cargo:typegen · Waiting for access to repo:compiler-writer:/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m (exclusive): position 4 of 4, behind the request of process 8581 (sh`
- course check: `[capacity] waiting 0s for run: not enough free memory (need 5632 MB; 3836 MB spare above the 6553 MB floor, load 14.7; 3 running, 0 ahead)`
- course check: `[capacity] waiting 5s for run: not enough free memory (need 5632 MB; 3702 MB spare above the 6553 MB floor, load 14.56; 3 running, 0 ahead)`
- course check: `[capacity] waiting 11s for run: not enough free memory (need 5632 MB; 4039 MB spare above the 6553 MB floor, load 14.2; 3 running, 0 ahead)`
- course check: `[capacity] waiting 16s for run: not enough free memory (need 5632 MB; 4682 MB spare above the 6553 MB floor, load 13.7; 3 running, 0 ahead)`
- course check: `[capacity] waiting 0s for exec: not enough free memory (need 3584 MB; 2094 MB spare above the 6553 MB floor, load 11; 3 running, 0 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-24T01:48:21+02:00, end@2026-10-24T02:12:46+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1023-course15`.

## Resources

Max load 18.2, min available memory 10.6 GB, min free disk 124.4 GB.

## Failures and warnings


### course cold check: FAIL

- command: `bun repo --tool=bazel run @teaching/architecture-course:check`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1023-course15/course/cold-5-check.log`

```
73 | }
74 | const unpack = (bytes, limit) => JSON.parse(brotliDecompressSync(bytes, { maxOutputLength: limit }).toString("utf8"));
75 | const fail = (message, code) => Object.assign(Error(message), code ? { code } : {});
                                                                                                                                                                                                                    ^
error: Edge refresh requires a published owner
 code: "GENERATION_INCOMPLETE"
      at fail (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs:70:208)
      at build (semio-source:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs:812:17)
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
Bun v1.4.2 (macOS arm64)
error: script "repo" exited with code 1
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1023-course15 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
