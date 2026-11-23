# W1 Ready diagnosis and fix (2026-11-23 12:50 CET)

Sources: `/Users/Shared/semio-store/v1/origins/g1-m*/ready.json`, `ready-timings.json`, `ready-log-workspace-failed.txt`; code `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready/🟦️.ts` (+ schema), Nx front `🌐️remote-cache/🧶️nx/🚦️single-flight/🟦️.ts`.

## Findings
(a) Ready was not lost to a digest change. All machines carry the same digest and sit in the `workspace` stage (5/5). They lost Ready because the re-verification of `workspace` FAILED or timed out, and E79 treated every recorded failure (timeout, stall exhaustion, full disk, exit 1) as "failed re-verification": `readyRefreshV1` returned null for any `failure !== null`, and a `backoff` patch always set `ready:false`. Stages that merely stall do not record a failure, but m07/m08 were stale beyond the 24 h bound (readyMs 5 d / 19 h). The workspace stage is `repo run-many norm-din4108,norm-din4108-rs,repo-mcp --targets=build` (1 h timeout): 15-110 min on a 16 GB Mac (graph slice read 1000-6000 s, 131 cargo units compiled in ~950 s on m01).
Cause of exit 1 on m02/m06/m13/m01 (their `-failed.txt` tails): Nx prints "Successfully ran target build" and then "Misconfigured remote cache endpoint: Unexpected response status: 503 Service Unavailable ... admission closed" - the Nx front passed the fleet cache server's 503/507/502 straight to Nx, which treats it as fatal. The fleet cache is full (CAS1/R3 lane), so the stage failed although every task succeeded.
(b) The 31184 MB need = one `ok` workspace run on m01 whose free-space delta was 15592 MB (includes every other writer on the volume) x CAS_RESERVE_FACTOR 2; nothing bounded it except 25 % of the volume (about 31 GB on 121 GB). m01 has 21 GB free, so it would never start. Fleet tier: the stage runs `repo run-many` -> native-build admission -> `unitCacheChainEnvironmentV1` (admission member line 417), not a hand-built env; logs show `unit:lookup-fleet-miss` counts (121 on m01, 13 on m06). Misses are real (fleet store full: "0 stored, 23 not shared"), so units are compiled locally; fix belongs to R3 (server bound, LRU eviction).
(c) m12: exit 1 in 18-30 s every attempt (45 attempts): `Original closure read child refused: TypeError: Cannot read properties of undefined (reading 'path')` from `...🧾️closures/📡️session/🟦️.ts:33` (graph closure read worker cohort, in the callback that processes rows/links). Only m12; real product bug, NOT fixed (minified one-line callbacks in closures/🟦️.ts:97 and 🗂️members/🔎️scan/🟦️.ts:419; needs the service log `host-659f709f2f10.log` on m12). Open item.
(d) m10: stage-timeout (SIGTERM at 7000 s, 4826 s, ...) while "Loading the project graph"/slice read; same graph-read slowness as m02/m06/m07/m08 (9 of 3600 owners after 1043 s on m07).

## Per machine
| machine | state at 12:39 | why not Ready | note |
|---|---|---|---|
| m01 | waiting, disk-collecting | last workspace run exit 1 (503 from cache); need 31184 MB vs 21155 free | need now capped 16384 |
| m02 | running workspace, attempt 4 | 4 failures: exit 1 (Nx 503), readyMs 8.6 h old | transient now kept |
| m03 | running tools, attempt 4 | tools stage 4 h, `repository:command` active | readyMs 5 d old (age bound) |
| m04 | waiting, dependencies, disk-consumers | needs 6144 MB, 4421 free after collection (Linux, 18 attempts) | disk-short, real |
| m05 | observing dependencies, lease-held | Windows, 28 attempts, pid-reuse verdict kills stage; never Ready | open (E6 manual restart) |
| m06 | running workspace, attempt 6 | exit 1 x5 (Nx 503), timeout once | readyMs 8.6 h |
| m07 | running workspace, attempt 1 | 10 stalls in a row (graph read 9/3600), readyMs 5 d | age bound |
| m08 | running workspace, attempt 3 | stalls (graph read), readyMs 18 h | stall, not failure |
| m09 | no ready.json | daemon never ran | open |
| m10 | backoff, stage-timeout | 3 timeouts, 1 exit 1; 4.1-7 ks graph read | now keeps Ready if still fresh |
| m11 | stale since 08:15 (4.3 h), offline | sidecar silent; last log `door.script` 4267 s | not Ready by silence |
| m12 | backoff, stage-failed attempt 45 | closure-read TypeError (c) | open product bug |
| m13 | running workspace, attempt 8 | exit 1 x4 (Nx 503), stalls; never Ready (readyMs null) | |

## Changes (landing in `T/landings.md` "Wave 1122-dist W1")
1. Nx front fails open: upstream non-404 non-200 on GET -> 404; PUT / CAS PUT upstream >= 500 or error -> 200 "Stored"; counter `dropped` (schema + type). Fleet cache pressure can never fail a build. Test: new law "a busy or full cache server never fails a build"; two former `502` expectations are now `200`.
2. Ready stickiness: Policy `refreshKeepingFailureCodes` (stage-timeout, stage-stall-exhausted, store-full, interrupted, spawn-failed, loop-failed). `readyRefreshV1` keeps Ready for those; only `stage-failed` and `tooling-compiler` leave it. `readyRefreshingPatchV1` maps a `backoff` of a kept failure to ready+refreshing with the refusal retained. The 24 h evidence age bound still ends a never-finishing refresh.
3. Disk need: Policy `diskGrowth.measuredCeilingMegabytes` 16384 bounds a measured need (free-space delta counts all writers); existing volume ceiling stays.

## Tests (one at a time via `repo test-file`)
nx single-flight 16/0; ready refresh 9/0 (grid with timeout/store-full/tooling-compiler vs Python oracle); disk-recovery 8/0 (oracle extended with the cap); ready main 56/1 (the known load-sensitive CPU-liveness test, as in E79), early-admission 4/0, stale-stage 14/0, stage-completion 8/0.

## Not verified / open
- Live effect needs replica + supervisor restart of workers (not forced); a worker staying Ready through a failing refresh is not yet observed.
- m12 TypeError, m05 Windows pid-reuse, m09 missing daemon, m04 disk (6 GB need on 4.4 GB free), workspace graph-read duration (1-2 h on 16 GB hosts; stage timeout 1 h) and fleet-cache misses (R3) remain.
- E77 finding stands: the disk gate measures the host volume while stages write into images.
