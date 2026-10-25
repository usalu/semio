# 🧭 e2e-orch report e2e-1023-course17

Created 2026-10-24T02:23:30+02:00, written 2026-10-24T14:29:15+02:00, orchestrator pid 48171. Argv: `course --go --run-id e2e-1023-course17 --no-verify-targets --phase cold,warm,e2e,probe || course --go --run-id e2e-1023-course17 --no-verify-targets --phase warm,e2e,probe || course --go --run-id e2e-1023-course17 --no-verify-targets --phase e2e,probe || course --go --run-id e2e-1023-course17 --no-verify-targets --phase e2e,probe || course --go --run-id e2e-1023-course17 --no-verify-targets --phase e2e,probe || course --go --run-id e2e-1023-course17 --no-verify-targets --phase e2e,probe || course --go --run-id e2e-1023-course17 --no-verify-targets --phase e2e,probe || course --go --run-id e2e-1023-course17 --no-verify-targets --phase e2e,probe || course --go --run-id e2e-1023-course17 --no-verify-targets --phase e2e,probe || course --go --run-id e2e-1023-course17 --no-verify-targets --phase e2e,probe || course --go --run-id e2e-1023-course17 --no-verify-targets --phase e2e,probe`.

Host: Darwin 27.0.0, 10 cores, parallel 1, Bazel tool `bazel`. Counts: FAIL 7, PASS 6. Cancelled: False.

## Prerequisites

Verdict **OK** at 2026-10-24T13:21:05+02:00.

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
| OK | disk | 127 GB free (need 15) |
| OK | memory | 14.4 GB available of 32.0 GB (the live capacity gate queues heavy commands below its floor; FAIL only under 2 GB) |
| OK | load | 19.4 on 10 cores (informational: foreign load no longer blocks, the capacity gate admits by memory and its own grants) |
| OK | bazel build slots | 3 slot(s) from the host budget for 1 product(s) |
| OK | ports course | [6061, 6063, 8791, 8793] free |
| OK | applied: x-site: course build cached | present |
| INFO | foreign heavy processes | 39 running (shared fleet; the governor and the slots arbitrate): 14933       18:45 bazel(e3f4c231e29f92ec) --add-opens=java.base/java.lang=ALL-UNNAMED -Xve // 16332       14:55 bun repo cargo --store=shared --configuration=prototype -- test --no-fai // 16333       14:55 /Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d55 |
| OK | bun repo --help | exit 0 |
| OK | bun repo capacity status | [capacity] 14814 MB available of 32768 MB (floor 6553 MB, 4976 MB spare), pressure normal, swap 2267 MB, load 19.39, open files 8563, attached images 20; 5 running, 6 waiting |

## Matrix

| product | cold | warm | e2e | probe |
|---|---|---|---|---|
| play | - | - | - | - |
| demo | - | - | - | - |
| course | PASS 40m58s | FAIL 1h47m36s | FAIL 1h07m00s | - |

## Architecture Course (`@teaching/architecture-course`)

| phase | step | status | rc | time | processes | spawns | cache hits | internal | slot | waits | note |
|---|---|---|---|---|---|---|---|---|---|---|---|
| cold | proctor-build | PASS | 0 | 1m36s | 2 | 1 | 0 | 1 | s2 | 5 |  |
| cold | typecheck | PASS | 0 | 7m08s | 8 | 7 | 0 | 1 | s2 | 5 |  |
| cold | test | PASS | 0 | 4m56s | 10 | 11 | 0 | 1 | s1 | 1 |  |
| cold | build | PASS | 0 | 1m41s | 8 | 7 | 0 | 1 | s2 | 0 |  |
| cold | check | PASS | 0 | 15m49s | 4 | 3 | 0 | 1 | s1 | 5 |  |
| cold | proctor-test | PASS | 0 | 9m49s | 6 | 3 | 0 | 3 | s1 | 0 |  |
| warm | proctor-build | FAIL | 0 | 15m44s | 4 | 1 | 0 | 3 | s0 | 5 | 1 spawns, at most 0 allowed |
| warm | typecheck | FAIL | 0 | 18m20s | 8 | 7 | 0 | 1 | s0 | 5 | 7 spawns, at most 0 allowed |
| warm | test | FAIL | 0 | 16m48s | 11 | 10 | 0 | 2 | s2 | 5 | 10 spawns, at most 0 allowed |
| warm | build | FAIL | 0 | 31m43s | 8 | 7 | 0 | 1 | s0 | 5 | 7 spawns, at most 0 allowed |
| warm | check | FAIL | 0 | 12m55s | 4 | 3 | 0 | 1 | s0 | 5 | 3 spawns, at most 0 allowed |
| warm | proctor-test | FAIL | 0 | 12m06s | 5 | 4 | 0 | 2 | s2 | 5 | 4 spawns, at most 0 allowed |
| e2e | test-e2e | FAIL | 1 | 1h07m00s | 6 | 4 | 0 | 2 | s0 | 5 | exit code 1 |

## Warm assertions (Bazel cache hits)

| product | step | spawns | allowed | internal | cache hits | time | expected within | verdict |
|---|---|---|---|---|---|---|---|---|
| course | proctor-build | 1 | 0 | 3 | 0 | 15m44s | 10m00s | FAIL 1 spawns, at most 0 allowed |
| course | typecheck | 7 | 0 | 1 | 0 | 18m20s | 10m00s | FAIL 7 spawns, at most 0 allowed |
|  |  |  |  |  |  |  |  | evidence: `Compiling proc-macro2 v1.0.107` |
|  |  |  |  |  |  |  |  | evidence: `Compiling quote v1.0.47` |
|  |  |  |  |  |  |  |  | evidence: `Compiling syn v3.0.9` |
|  |  |  |  |  |  |  |  | evidence: `Compiling serde_core v1.0.229` |
|  |  |  |  |  |  |  |  | evidence: `Compiling serde_derive v1.0.229` |
| course | test | 10 | 0 | 2 | 0 | 16m48s | 10m00s | FAIL 10 spawns, at most 0 allowed |
| course | build | 7 | 0 | 1 | 0 | 31m43s | 10m00s | FAIL 7 spawns, at most 0 allowed |
|  |  |  |  |  |  |  |  | evidence: `Compiling proc-macro2 v1.0.107` |
|  |  |  |  |  |  |  |  | evidence: `Compiling quote v1.0.47` |
|  |  |  |  |  |  |  |  | evidence: `Compiling unicode-ident v1.0.27` |
|  |  |  |  |  |  |  |  | evidence: `Compiling serde_core v1.0.229` |
|  |  |  |  |  |  |  |  | evidence: `Compiling syn v3.0.9` |
| course | check | 3 | 0 | 1 | 0 | 12m55s | 10m00s | FAIL 3 spawns, at most 0 allowed |
| course | proctor-test | 4 | 0 | 2 | 0 | 12m06s | 10m00s | FAIL 4 spawns, at most 0 allowed |

## Queueing (slots, capacity)

- course proctor-build: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 1781 MB spare above the 6553 MB floor, load 16.62; 5 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 5s for run: not enough free memory (need 3584 MB; 1817 MB spare above the 6553 MB floor, load 16.01; 5 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 11s for run: not enough free memory (need 3584 MB; 2266 MB spare above the 6553 MB floor, load 15.29; 5 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 16s for run: not enough free memory (need 3584 MB; 2915 MB spare above the 6553 MB floor, load 15.11; 5 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 22s for run: not enough free memory (need 3584 MB; 2300 MB spare above the 6553 MB floor, load 14.78; 5 running, 0 ahead)`
- course typecheck: `[capacity] waiting 0s for run: not enough free memory (need 5632 MB; 5015 MB spare above the 6553 MB floor, load 11.41; 4 running, 1 ahead)`
- course typecheck: `[capacity] waiting 5s for run: not enough free memory (need 5632 MB; 1021 MB spare above the 6553 MB floor, load 11.06; 5 running, 0 ahead)`
- course typecheck: `[capacity] waiting 10s for run: not enough free memory (need 5632 MB; -621 MB spare above the 6553 MB floor, load 10.81; 5 running, 0 ahead)`
- course typecheck: `[capacity] waiting 16s for run: not enough free memory (need 5632 MB; 2916 MB spare above the 6553 MB floor, load 10.58; 5 running, 0 ahead)`
- course typecheck: `[capacity] waiting 21s for run: not enough free memory (need 5632 MB; 2292 MB spare above the 6553 MB floor, load 10.78; 5 running, 0 ahead)`
- course test: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 976 MB spare above the 6553 MB floor, load 8.63; 4 running, 0 ahead)`
- course check: `[capacity] waiting 0s for run: not enough free memory (need 5632 MB; 3163 MB spare above the 6553 MB floor, load 8.34; 4 running, 0 ahead)`
- course check: `[capacity] waiting 6s for run: not enough free memory (need 5632 MB; 1870 MB spare above the 6553 MB floor, load 7.99; 4 running, 0 ahead)`
- course check: `[capacity] waiting 11s for run: not enough free memory (need 5632 MB; 835 MB spare above the 6553 MB floor, load 7.83; 4 running, 0 ahead)`
- course check: `[capacity] waiting 17s for run: not enough free memory (need 5632 MB; 964 MB spare above the 6553 MB floor, load 7.7; 4 running, 0 ahead)`
- course check: `[capacity] waiting 23s for run: not enough free memory (need 5632 MB; 1229 MB spare above the 6553 MB floor, load 7.49; 4 running, 0 ahead)`
- course proctor-build: `[capacity] waiting 0s for run: not enough free memory (need 3584 MB; 2504 MB spare above the 6553 MB floor, load 20.16; 2 running, 8 ahead)`
- course proctor-build: `[capacity] waiting 6s for run: an earlier request is still waiting (need 3584 MB; -127 MB spare above the 6553 MB floor, load 18.43; 3 running, 7 ahead)`
- course proctor-build: `[capacity] waiting 11s for run: an earlier request is still waiting (need 3584 MB; 59 MB spare above the 6553 MB floor, load 18.43; 3 running, 7 ahead)`
- course proctor-build: `[capacity] waiting 16s for run: an earlier request is still waiting (need 3584 MB; 575 MB spare above the 6553 MB floor, load 18.5; 4 running, 6 ahead)`

## Cache and capacity statistics

- cache: 0 changed counter(s) between `start` and `end`
- capacity: 0 changed counter(s) between `start` and `end`
- leases: 0 changed counter(s) between `start` and `end`

Snapshots: start@2026-10-24T02:23:37+02:00, t30@2026-10-24T02:53:37+02:00, t60@2026-10-24T03:23:40+02:00, end@2026-10-24T03:45:09+02:00, start@2026-10-24T03:47:06+02:00, t30@2026-10-24T04:17:00+02:00, t60@2026-10-24T04:47:03+02:00, t90@2026-10-24T05:17:08+02:00, end@2026-10-24T05:45:54+02:00, start@2026-10-24T05:51:38+02:00, t30@2026-10-24T06:21:44+02:00, end@2026-10-24T06:21:55+02:00, start@2026-10-24T06:28:12+02:00, t30@2026-10-24T06:58:32+02:00, end@2026-10-24T07:05:01+02:00, start@2026-10-24T07:06:12+02:00, t30@2026-10-24T07:36:13+02:00, end@2026-10-24T07:44:34+02:00, start@2026-10-24T07:44:55+02:00, end@2026-10-24T07:45:05+02:00, start@2026-10-24T07:53:15+02:00, t30@2026-10-24T08:23:17+02:00, end@2026-10-24T08:23:29+02:00, start@2026-10-24T09:08:51+02:00, end@2026-10-24T09:09:04+02:00, start@2026-10-24T09:09:14+02:00, t30@2026-10-24T09:39:16+02:00, end@2026-10-24T10:07:18+02:00, start@2026-10-24T12:35:42+02:00, t30@2026-10-24T13:05:53+02:00, end@2026-10-24T13:21:03+02:00, start@2026-10-24T13:21:13+02:00, t30@2026-10-24T13:51:35+02:00, t60@2026-10-24T14:21:51+02:00, end@2026-10-24T14:29:14+02:00. Raw files: `🗑️generated/e2e-orch/e2e-1023-course17`.

## Resources

Max load 43.0, min available memory 5.7 GB, min free disk 112.2 GB.

## Failures and warnings


### course warm proctor-build: FAIL

- command: `bun repo --tool=bazel run @teaching/proctor:build --configuration=release`
- note: 1 spawns, at most 0 allowed
- log: `🗑️generated/e2e-orch/e2e-1023-course17/course/warm-1-proctor-build.log`

```
[@teaching/proctor:build:release] running elapsedMs=320069
[@teaching/proctor:build:release] running elapsedMs=330071
[@teaching/proctor:build:release] running elapsedMs=340072
[@teaching/proctor:build:release] running elapsedMs=350074
[@teaching/proctor:build:release] running elapsedMs=360076
[@teaching/proctor:build:release] running elapsedMs=370082
[@teaching/proctor:build:release] running elapsedMs=380084
[@teaching/proctor:build:release] running elapsedMs=390086
[@teaching/proctor:build:release] running elapsedMs=400086
[@teaching/proctor:build:release] running elapsedMs=410088
[@teaching/proctor:build:release] running elapsedMs=420090
[@teaching/proctor:build:release] running elapsedMs=430091
[@teaching/proctor:build:release] running elapsedMs=440094
[@teaching/proctor:build:release] running elapsedMs=450094
[@teaching/proctor:build:release] running elapsedMs=460096
INFO: Found 1 target...
Target //:t_7d24f6432dc80bf5 up-to-date:
  /Users/ueli/Documents/semio/.cache/b/p/m/bazel/outputs/server/s0/execroot/_main/bazel-out/darwin_arm64-fastbuild/bin/t_7d24f6432dc80bf5.snapshot
INFO: Elapsed time: 478.572s, Critical Path: 474.56s
INFO: 2 processes: 1 internal, 1 local.
INFO: Build completed successfully, 2 total actions
INFO: 
INFO: Build Event Protocol files produced successfully.
[script] {"owner":"repository:build","command":"bazel:snapshot-restore","stage":"running"}
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
```

### course warm typecheck: FAIL

- command: `bun repo --tool=bazel run @teaching/architecture-course:typecheck`
- note: 7 spawns, at most 0 allowed
- log: `🗑️generated/e2e-orch/e2e-1023-course17/course/warm-2-typecheck.log`

```
[script] {"owner":"repository:build","command":"owner-command","stage":"loading"}
[script] {"owner":"repository:build","command":"owner-command","stage":"running"}
[script] {"owner":"repository:build","command":"typecheck","stage":"loading"}
[script] {"owner":"repository:build","command":"typecheck","stage":"running"}
[script] {"owner":"repository:build","command":"typecheck","stage":"running"}
[script] {"owner":"repository:build","command":"typecheck","stage":"running"}
[@teaching/architecture-course:typecheck:prototype] running elapsedMs=10002
[process:owner-command] running elapsedMs=10002
[architecture-course:typescript] running elapsedMs=10001
[@teaching/architecture-course:typecheck:prototype] running elapsedMs=20004
[process:owner-command] running elapsedMs=20004
[architecture-course:typescript] running elapsedMs=20003
[script] {"owner":"repository:build","command":"typecheck","stage":"complete"}
[script] {"owner":"repository:build","command":"typecheck","stage":"complete"}
[script] {"owner":"repository:build","command":"owner-command","stage":"complete"}
INFO: Found 1 target...
Target //:t_0d2cec7ea080488d up-to-date:
  /Users/ueli/Documents/semio/.cache/b/p/m/bazel/outputs/server/s0/execroot/_main/bazel-out/darwin_arm64-fastbuild/bin/t_0d2cec7ea080488d.snapshot
INFO: Elapsed time: 210.647s, Critical Path: 103.54s
INFO: 8 processes: 1 internal, 2 local, 5 worker.
INFO: Build completed successfully, 8 total actions
INFO: 
INFO: Build Event Protocol files produced successfully.
[script] {"owner":"repository:build","command":"bazel:snapshot-restore","stage":"running"}
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
```

### course warm test: FAIL

- command: `bun repo --tool=bazel run @teaching/architecture-course:test -- quick`
- note: 10 spawns, at most 0 allowed
- log: `🗑️generated/e2e-orch/e2e-1023-course17/course/warm-3-test.log`

```
[11 / 12] Testing //:t_59ab860e3b26c2af; 11s local, disk-cache
[bazel:test] running elapsedMs=140022
[repository:command] running elapsedMs=740134
[bazel:test] running elapsedMs=150022
[repository:command] running elapsedMs=750134
[bazel:test] running elapsedMs=160023
[repository:command] running elapsedMs=760136
[11 / 12] Testing //:t_59ab860e3b26c2af; 41s local, disk-cache
[bazel:test] running elapsedMs=170026
[repository:command] running elapsedMs=770137
[bazel:test] running elapsedMs=180029
[repository:command] running elapsedMs=780139
[11 / 12] 1 / 1 tests; Testing //:t_59ab860e3b26c2af; 61s local, disk-cache
INFO: Found 1 test target...
Target //:t_59ab860e3b26c2af up-to-date:
  /Users/ueli/Documents/semio/.cache/b/p/m/bazel/outputs/server/s2/execroot/_main/bazel-out/darwin_arm64-fastbuild/bin/t_59ab860e3b26c2af
INFO: Elapsed time: 183.619s, Critical Path: 112.89s
INFO: 11 processes: 1 action cache hit, 2 internal, 9 local.
INFO: Build completed successfully, 11 total actions
INFO: 
//:t_59ab860e3b26c2af                                                    PASSED in 61.3s
Executed 1 out of 1 test: 1 test passes.
There were tests whose specified size is too big. Use the --test_verbose_timeout_warnings command line option to see which ones these are.
INFO: Build Event Protocol files produced successfully.
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
```

### course warm build: FAIL

- command: `bun repo --tool=bazel run @teaching/architecture-course:build --configuration=release`
- note: 7 spawns, at most 0 allowed
- log: `🗑️generated/e2e-orch/e2e-1023-course17/course/warm-4-build.log`

```
../../../.cache/b/p/m/vite/release/e483dc668ec8bf18/tmp/v-7xdx1E/assets/🟦️-C8YoPNSa.js       4.47 kB
../../../.cache/b/p/m/vite/release/e483dc668ec8bf18/tmp/v-7xdx1E/assets/🟦️-ek1q6GBH.js      11.24 kB
../../../.cache/b/p/m/vite/release/e483dc668ec8bf18/tmp/v-7xdx1E/assets/🟦️-Cnw25x0Y.js      11.90 kB
../../../.cache/b/p/m/vite/release/e483dc668ec8bf18/tmp/v-7xdx1E/assets/🟦️-C9IA7yf6.js      14.82 kB
../../../.cache/b/p/m/vite/release/e483dc668ec8bf18/tmp/v-7xdx1E/assets/🟦️-b-Z0iMYP.js      47.67 kB
../../../.cache/b/p/m/vite/release/e483dc668ec8bf18/tmp/v-7xdx1E/assets/🟦️-DB04vBf7.js      54.62 kB
../../../.cache/b/p/m/vite/release/e483dc668ec8bf18/tmp/v-7xdx1E/assets/🟦️-houqbNPu.js     160.77 kB
../../../.cache/b/p/m/vite/release/e483dc668ec8bf18/tmp/v-7xdx1E/assets/🟦️-BZeL9p_s.js     189.82 kB
../../../.cache/b/p/m/vite/release/e483dc668ec8bf18/tmp/v-7xdx1E/assets/🟦️-CedvYNDo.js   1,123.85 kB
../../../.cache/b/p/m/vite/release/e483dc668ec8bf18/tmp/v-7xdx1E/assets/🌐️-DToGpdde.js   1,223.50 kB
✓ built in 5.94s
INFO: Found 1 target...
Target //:t_3ba32c26490207b2 up-to-date:
  /Users/ueli/Documents/semio/.cache/b/p/m/bazel/outputs/server/s0/execroot/_main/bazel-out/darwin_arm64-fastbuild/bin/t_3ba32c26490207b2.snapshot
INFO: Elapsed time: 263.269s, Critical Path: 167.76s
INFO: 8 processes: 1 internal, 7 local.
INFO: Build completed successfully, 8 total actions
INFO: 
INFO: Build Event Protocol files produced successfully.
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"resource-retirement-inventory:128","mode":"shared","elapsedMs":2}}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"claim-retirement-scan:128","mode":"exclusive","elapsedMs":535}}
[script] {"owner":"repository:build","command":"bazel:snapshot-restore","stage":"running"}
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"resource-directory-admission:v1","mode":"shared","elapsedMs":0}}
[repository:command] running elapsedMs=990157
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
```

### course warm check: FAIL

- command: `bun repo --tool=bazel run @teaching/architecture-course:check`
- note: 3 spawns, at most 0 allowed
- log: `🗑️generated/e2e-orch/e2e-1023-course17/course/warm-5-check.log`

```
[script] {"owner":"repository:build","command":"bazel:owned-action-child","stage":"running"}
[script] {"owner":"repository:build","command":"bazel:snapshot-restore","stage":"running"}
[script] {"owner":"repository:build","command":"bazel:snapshot-restore","stage":"running"}
[script] {"owner":"repository:build","command":"owner-command","stage":"loading"}
[script] {"owner":"repository:build","command":"owner-command","stage":"running"}
[script] {"owner":"repository:build","command":"check","stage":"loading"}
[script] {"owner":"repository:build","command":"check","stage":"running"}
course architecture (/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/📚️course/🔣️.json): 5 quizzes, 8 badges, fingerprint e9aadcf92857930586f0dbb2f039c6b3982006f84cff5422a8e1055a8980e631
  quiz physics (3 tasks) ../⚡️energy/🧲️physics/❓️quiz/🔣️.json revision 1795e5c9491d9f46f5bd591e4e10d45faea6e13188a61522e913dc811a5bae26
  quiz ventilation (3 tasks) ../⚡️energy/🌬️ventilation/❓️quiz/🔣️.json revision 9f256c7b64ca73c124633d320d76632424ec562713546ee6fa9abf53fe46762d
  quiz heating (2 tasks) ../⚡️energy/🔥️heating/❓️quiz/🔣️.json revision db04b89a6218b9296f2f129290c0d675562cac470f7b46e8d30749e2ffc5a319
  quiz cooling (1 tasks) ../⚡️energy/❄️cooling/❓️quiz/🔣️.json revision 82320ffc1602ca2f2c29bdf768c59ab490acfd4c68a9469329e3c00350ba55c4
  quiz demand (2 tasks) ../⚡️energy/📊️demand/❓️quiz/🔣️.json revision 73da95631f43d2034bc2911178e50575ad66c2cc1de47b3a4d1364d8c23b1f4c
[script] {"owner":"repository:build","command":"check","stage":"complete"}
[script] {"owner":"repository:build","command":"owner-command","stage":"complete"}
INFO: Found 1 target...
Target //:t_64acdcd6a369b08c up-to-date:
  /Users/ueli/Documents/semio/.cache/b/p/m/bazel/outputs/server/s0/execroot/_main/bazel-out/darwin_arm64-fastbuild/bin/t_64acdcd6a369b08c.snapshot
INFO: Elapsed time: 224.976s, Critical Path: 210.49s
INFO: 4 processes: 1 internal, 1 local, 2 worker.
INFO: Build completed successfully, 4 total actions
INFO: 
INFO: Build Event Protocol files produced successfully.
[script] {"owner":"repository:build","command":"bazel:snapshot-restore","stage":"running"}
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
```

### course warm proctor-test: FAIL

- command: `bun repo --tool=bazel run @teaching/proctor:test`
- note: 4 spawns, at most 0 allowed
- log: `🗑️generated/e2e-orch/e2e-1023-course17/course/warm-6-proctor-test.log`

```
[repository:command] running elapsedMs=650114
[bazel:test] running elapsedMs=170022
[repository:command] running elapsedMs=660116
[5 / 6] Testing //:t_7203536f29be45b0; 161s local, disk-cache
[bazel:test] running elapsedMs=180023
[repository:command] running elapsedMs=670116
[bazel:test] running elapsedMs=190026
[repository:command] running elapsedMs=680119
[bazel:test] running elapsedMs=200028
[repository:command] running elapsedMs=690120
[bazel:test] running elapsedMs=210029
[5 / 6] 1 / 1 tests; Testing //:t_7203536f29be45b0; 198s local, disk-cache
INFO: Found 1 test target...
Target //:t_7203536f29be45b0 up-to-date:
  /Users/ueli/Documents/semio/.cache/b/p/m/bazel/outputs/server/s2/execroot/_main/bazel-out/darwin_arm64-fastbuild/bin/t_7203536f29be45b0
INFO: Elapsed time: 213.051s, Critical Path: 208.19s
INFO: 5 processes: 1 action cache hit, 2 internal, 3 local.
INFO: Build completed successfully, 5 total actions
INFO: 
//:t_7203536f29be45b0                                                    PASSED in 198.7s
Executed 1 out of 1 test: 1 test passes.
There were tests whose specified size is too big. Use the --test_verbose_timeout_warnings command line option to see which ones these are.
INFO: Build Event Protocol files produced successfully.
[repository:command] running elapsedMs=700122
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
```

### course e2e test-e2e: FAIL

- command: `bash /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/e2e-tools/run-e2e.sh course --tool=bazel --topology=dev`
- note: exit code 1
- log: `🗑️generated/e2e-orch/e2e-1023-course17/course/e2e-1-test-e2e.log`

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
[run-e2e 14:28:46] summary for course (@teaching/architecture-course) tool=--tool=bazel
  test-e2e           exit 1   4020s
  logs: /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/e2e-course-e2e-1023-course17-course-*.log
```

## Re-run

Continue this run: `run.sh all --run-id e2e-1023-course17 --go --phase warm,e2e` (add `--rerun` to repeat steps that already passed). Exact launch commands: result-e2e-orch.md.
