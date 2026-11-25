# F5 result: readiness pipeline time and stalls

Measured 16:30-16:45 from `S/origins/*/ready-timings.json`, `ready-log-workspace*.txt`, `ready.json` (505 recorded runs).

## Where the time goes (workspace stage)
- Outcomes (workspace runs): 155 stalled (median 1161 s), 160 stage-failed (median 920 s), 10 stage-timeout (median 8685 s), 10 ok (median 1418 s, 798-3725 s, peak 2.6-5.6 GB). A stalled run lasts about as long as an ok run, so stalls happen late, after minutes of work, not at the start.
- One workspace attempt on a 16 GB Mac: ~7 min before the slice read prints anything (the `door.script` phase counter in the log already stands at 440-680 s when the first slice line appears), then the slice read (`taken N of 612`, 6-10 min) which restarts a full owner pass whenever a path under the read changes ("a new pass over the owners starts (round N): 1 path(s) changed", exit 75 "authority changed during inference"), then planning, then Nx admission (`nx:source-verification`, the only watchdog activity of every ok run).
- Slice restarts are driven by the live replica: this wave's ~14 lanes edit the tree continuously, so rounds restart. Not fixable inside readiness; the quiet fleet needs no restarts.
- Failures: the stage-failed share (about 50 %) is the F1 crate-refused class and older ones; not touched.

## Why attempts stall (false positive found and fixed)
- Stalled logs (m06, m07, m10, m12, m13) end with `Planning the selected targets and what they need.` and, on m10, `The graph service holds this request and is working on it (1170 s).`: the graph service is working, the stage prints nothing new, and after 10 min the stage tree is killed and the whole read is redone.
- Cause: `readingPatterns` of the process-tree policy listed a dead text (`Planning every target`) and none of the lines the resident service prints today. The phase stayed `null`, so `readyReadingLiveV1` (phase reading + live runner in the read record = stall exempt, bounded by the stage hard cap) never applied.
- Fix: `readingPatterns` now lists the current plan/hold/identify/scope/wait lines in English and German (see `T/landings.md`, "Wave 1125 F5"). Test: stale-stage "every line the graph service prints ..." (15/0), process-tree 16/0, graph-progress 3/0.

## Per-worker, per-stage time (minutes total over all recorded runs)
| worker | stage | runs | total min | stalled | failed | timeout | ok | median ok s | peak MB (max) |
|---|---|---|---|---|---|---|---|---|---|
| g1-m01 | workspace | 39 | 933 | 20 | 17 | 1 | 1 | 1456 | 5379 |
| g1-m01 | dependencies | 1 | 5 | 0 | 0 | 0 | 1 | 339 | 4768 |
| g1-m02 | workspace | 21 | 2261 | 0 | 19 | 0 | 2 | 2029 | 5492 |
| g1-m02 | dependencies | 4 | 18 | 0 | 0 | 0 | 4 | 176 | 2910 |
| g1-m02 | tools | 15 | 800 | 0 | 14 | 0 | 1 | 15613 | 2578 |
| g1-m03 | workspace | 34 | 1091 | 15 | 16 | 2 | 1 | 3070 | 5641 |
| g1-m03 | dependencies | 6 | 25 | 0 | 4 | 0 | 2 | 312 | 2637 |
| g1-m04 | tools | 24 | 1829 | 0 | 20 | 4 | 0 | - | 1542 |
| g1-m04 | native | 4 | 0 | 0 | 0 | 0 | 4 | 2 | 0 |
| g1-m04 | dependencies | 12 | 39 | 0 | 12 | 0 | 0 | - | 1928 |
| g1-m05 | dependencies | 40 | 2300 | 22 | 15 | 3 | 0 | - | 1985 |
| g1-m06 | workspace | 37 | 1126 | 16 | 19 | 1 | 1 | 1380 | 6206 |
| g1-m06 | dependencies | 3 | 12 | 0 | 1 | 0 | 2 | 319 | 2647 |
| g1-m07 | workspace | 38 | 1340 | 30 | 6 | 1 | 1 | 1197 | 4439 |
| g1-m07 | dependencies | 2 | 16 | 0 | 0 | 0 | 2 | 503 | 2852 |
| g1-m08 | workspace | 38 | 1695 | 25 | 11 | 2 | 0 | - | 4151 |
| g1-m08 | dependencies | 2 | 14 | 0 | 0 | 0 | 2 | 429 | 2537 |
| g1-m10 | workspace | 38 | 1474 | 20 | 16 | 2 | 0 | - | 4642 |
| g1-m10 | dependencies | 2 | 7 | 0 | 0 | 0 | 2 | 232 | 3152 |
| g1-m11 | workspace | 12 | 492 | 4 | 6 | 1 | 1 | 3725 | 8209 |
| g1-m11 | dependencies | 28 | 18 | 0 | 26 | 0 | 2 | 178 | 2381 |
| g1-m12 | workspace | 39 | 689 | 24 | 14 | 0 | 1 | 1179 | 5871 |
| g1-m12 | dependencies | 1 | 3 | 0 | 0 | 0 | 1 | 227 | 3077 |
| g1-m13 | dependencies | 13 | 55 | 0 | 12 | 0 | 1 | 433 | 2216 |
| g1-m13 | workspace | 27 | 616 | 1 | 25 | 0 | 1 | 3597 | 5130 |
| g1-m14 | native | 1 | 2 | 0 | 0 | 0 | 1 | 126 | 732 |
| g1-m14 | repo-mcp | 1 | 1 | 0 | 0 | 0 | 1 | 73 | 2029 |
| g1-m14 | dependencies | 8 | 8 | 0 | 7 | 0 | 1 | 105 | 2668 |
| g1-m14 | tools | 3 | 34 | 0 | 2 | 0 | 1 | 1002 | 3040 |
| g1-m14 | workspace | 12 | 253 | 0 | 11 | 0 | 1 | 798 | 4707 |

## Not verified / for others
- After-fix live measurement: supervisors restart on the replicated schema change; no worker had completed a post-fix attempt when this was written, so the stall-count drop is unmeasured. Check `ready-timings.json` outcome `stalled` counts over the next hours.
- Not done: work resumed per attempt (the slice read reuses the on-disk memo, `282560 verified memo rows`, but each round re-checks the 31k-file inventory ~7 min), and fleet hits of m07 outputs (cache hit rates per unit are not in `ready-timings.json`; the slice report has no restored/compiled split). The biggest remaining lever is the 7 min pre-slice phase and the round restarts from replica churn.
- Windows (m05) stalls (22 on dependencies) and m04 EACCES are other lanes.

## Round 2
### Ready-stays-Ready on adoption
- Not a regression of adoption. Code check: the adopted stage is announced through the same `status.update` that wraps every patch in `readyRefreshingPatchV1(patch, refreshStage)`; `refreshStage` comes from `readyRefreshV1`, which returns `null` only without marker/binary/readyMs, with a stale stage never verified or older than 24 h, or with a recorded failure whose code is real evidence (`stage-failed`, `tooling-compiler`).
- g1-m07 live: `ready-log-workspace-failed.txt` (15:43Z = 16:43 CET) holds a failed re-verification (`build:nx-command failed (1)` / `native:owner-command failed (1)`, the F1 crate-refused class), recorded as `stage-failed`, `attempts:1`. That failure legitimately cleared Ready; the adoption at 16:47 ran with a failure on record (`readyMs` is kept). It returns to Ready once F1's fix lands and one refresh passes. m11 and m06 are `ready:true refreshing:false` at 16:49 (m11 ok 1418 s, m06 ok 2181 s), so nothing is stuck by adoption.
- New test (adoption of a refresh with no failure keeps `ready:true refreshing:true`, never `ready:false`; a failed adopted re-verification does clear Ready): refresh tests 10/0. Landing recorded in `T/landings.md`. No code edit was needed, so no lock was taken.
- Open design question for the coordinator: a `stage-failed` of a refresh that fails deterministically on environment-wide code (F1) clears Ready on every worker at once; if that is unwanted, add `stage-failed` of a refresh to `refreshKeepingFailureCodes` only after the Ready worker already passed with the same stage digest lineage.

### Re-measure of the readingPatterns fix
- Workspace runs ended since 16:40 CET (window 10 min, 7 runs): 0 stalled (m11 ok, m06 ok, 5 stage-failed). Before: 1 stalled of 15 in the hour before 16:40, 1 of 27 in 2 h, 1 of 55 in 4 h, 2 of 71 in 8 h (the 155 stalls are concentrated on 11-24).
- The window is shorter than the 10 min stall rule plus an attempt, so this is inconclusive. Recount in an hour: `outcome == "stalled"` in `ready-timings.json` with `endedMs` after 16:40 CET.
