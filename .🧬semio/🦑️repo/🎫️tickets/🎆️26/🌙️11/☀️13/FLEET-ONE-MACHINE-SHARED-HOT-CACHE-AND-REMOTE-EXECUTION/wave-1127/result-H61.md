# Result H61: end-to-end profile of `repo inputs repo-graph-maintenance:test` (08:51-09:31, load 38-46 on 10 cores)

Run: `SEMIO_REPO_PROFILE=1 nice -n 10 repo inputs repo-graph-maintenance:test` (scratch `H61/run1.err`), stopped by me at 2330 s after three full attempts (4th was in its slice at 135 s).

## Headline: the 45 minutes are three to four whole-command restarts, not one slow run
Each attempt ends in `Another process kept editing …/🔏️authorizers/📦️runtime/🟦️.ts … (its bytes changed). Waiting N ms, then starting the command again (k of 6)` and the outermost command restarts from zero: attempt 1 = 725 s, attempt 2 = about 790 s, attempt 3 = about 800 s (+ 4th started). 2692 s of H56 = 3.4 attempts. A single attempt is 12-13 min, of which the slice is 520-590 s and the tool stage 150-180 s; the restart throws away the finished slice AND the finished plan.
The reported path is only the ENTRY label of the authorizer closure. `ensureNxAuthorizerBundleV1` re-captures the 2084-file runtime closure and 2296-file compiler closure after the 60-100 s bundle compile (`nxAuthorizerSourcesHoldV1`, full walk, no memo) and throws a `build`-rank concurrent edit if any source moved; nothing catches the `build` layer, so it escapes to the process restart. The closure is live product code: 2 of its files were modified within the last minute and 15 within 30 min at 09:26 (peers: shard image, lane pool, plan, reload). The same bytes error 3 of 3 times.

## Phase table, attempt 1 (wall seconds from client start; CPU = not measured per phase, see note)
| # | phase | start-end | wall | waiting for what |
|---|---|---|---|---|
| 1 | front door / launch (`door.script`) | 0-6 | 6 | process boot |
| 2 | slice worker boot + module load | 6-13 | 7 | CPU (module load) |
| 3 | prepare slice read (backend, compiler runtime, native recheck) | 13-35 | 22 | CPU + disk under load |
| 4 | open graph store + quota ledger + graph state lease | 35-94 | **59** | image opens (hdiutil roster 1.7-3.8 s each), ledger, capacity (not a lease wait per H38/H43) |
| 5 | seed owner facts from the memo | 94-137 | **43** | CPU (memo 41,777 rows, journal vouch) |
| 6 | scan owners = rediscovery of the ONE owner (about 50k rows) | 137-392 | **255** | CPU (cold single owner is 19-21 s CPU per H60: stretched 12x by load 40 on 10 cores), closure graph load (10 s CPU), module parse |
| 7 | publish admission memo | 392-424 | **32** | disk + CPU |
| 8 | read owners, taken | 424-445 | 21 | |
| 9 | inventory attempt 2 (read memo, seed, hash membership) | 445-475 | **30** | the first inventory attempt is repeated whole once peers edited during the 255 s scan |
| 10 | verify observed facts | 475-505 | 30 | |
| 11 | publish sealed graph to the fleet | 505-534 | 29 | |
| 12 | source admission + dependencies.ensure (client child) | 534-560 | 26 | dependencies.ensure 2.8-9.8 s, verify 1.1-7.4 s, recheck 0.8-1.1 s |
| 13 | tool.nx: planning storage + reading build tool index (11,648 entries) + verifying build tools | ~560-650 | **~85** | CPU/lstat: the SDK verify recorded 11,701 lstat calls totalling 52 s (4.5 ms each under load); `nx.sdk-source.verify` 14-18 s |
| 14 | authorizer bundle: capture 2 closures (about 20+22 s) + compile + `Preparing build tools` | ~650-690 | ~40 | bundler, per-source shared leases (3,767 `nx:authorizers:release` stderr lines) |
| 15 | authorizer hold re-walks (runtime 25 s, compiler 45 s standalone) -> concurrent edit -> `exit 75` | ~690-725 | ~35 | CPU (full walk without memo) |
| 16 | restart sleep 250 ms, whole command again | 725 | | |
Attempt 2: slice 593 s (owner scan 127-378 s again, memo not reused: peers edited), tool stage 176 s, restart at about 790 s. Attempt 3: same shape, 4th started.

Phases over 30 s and their cause: 4 (59 s store/lease/ledger), 5 (43 s memo seed), 6 (255 s owner rediscovery), 7 (32 s publish memo), 9 (30 s repeated inventory), 10 (30 s), 13 (~85 s tool index/verify), and, above all, the repetition of 1-15 three to four times (restart). No lease wait, no capacity wait was visible in the progress text this time (the capacity step passed in under 5 s); the preflight of tool closures H30 measured (186 s) is phase 13-15 here (150-180 s).
Machine context: 10 cores, load 38-46 (4x), swap 14.9 of 16 GB; top CPU outside the command: Activity Monitor 100 % (1687 CPU-min, the coordinator's UI: close it), replica supervisor boot-child 97 %, remote-execution cache sidecar 90 % (group server serving the fleet), H9's whole read (90730, 4 GB, 48 CPU-min), archivist git signing. Per-phase CPU was not captured (service-side phases run in a worker whose profile prints only at exit; `--cpu-prof` of the worker was not attempted to avoid a second 12 min run under the same load).

## Fix staged for the 10:00 slot (outside graph code; `🧰️scripts/H61/land.sh`)
Ready to land: `patch.py` (anchors verified by dry run on the live files; scratch copy parses with bun), `test-addition.ts` (passes on the scratch copy, 1/0), lock `landing.lock.bootstrap`.
1. A bundle refused for a peer edit is built again INSIDE the command: `ready()` wraps `ensureNxAuthorizerBundleV1` in `retryOnConcurrentEditV1("build")` (existing layer: 4 attempts, 2/8/30 s backoff, bilingual announcement) and attempt >= 2 re-captures the closure pair through the new `captureNxAuthorizerPairV1` (factored from `captureNxToolingInputsV1`). Cost per retry about 40 s capture + 60-100 s compile instead of a 12 min restart that redoes the slice.
2. The refusal names the moved SOURCE (`nxAuthorizerSourcesMovedV1`, first differing path, or the failing path of the walk), not the entry label; `nxAuthorizerSourcesHoldV1` stays as `Moved===undefined`. This also makes the third-party cause visible in the next run (today a swallowed walk error and a real edit look the same: the old `catch{return false}`).
Not proven live (needs the slot + a run); expected: edit during the tool stage costs about 2-3 min instead of about 12-13 min per occurrence.

## Not fixable outside graph code (staged by others / after the seal window)
- Phase 6/5/7/9: any edit of any row of the 1-owner closure drops the whole memo, so every attempt rediscovers 50k rows (255 s under load). Needs the per-row delta of H56 round 2 (journal-vouch analysis/field rows with parser identity) and cheaper closure-graph load (H60 finding 2). With the tree edited every few minutes by 10 lanes, the clean narrow target (< 60 s) is not reachable until those land AND the tool stage no longer restarts the command.
- Phase 4 (59 s) and 11 (29 s) are graph-state/storage code (H43 left them) and were not touched.

## Unverified
- Landing, the retry effect and any "after" number (none exists; the after column is empty on purpose).
- Why the hold fails every time: a real source edit is the proven possibility (files modified in the closure within minutes), a walk exception is not excluded; the diagnostic in fix 2 answers it.
- Per-phase CPU split. The 4th attempt and the final total of the profiled run were cut at 2330 s by me (my pid only).
Scripts (kept): `🧰️scripts/H61/authorizer-drift.ts` (captures the closure, diffs against a previous capture; 2084 files, 12-15 s), `authorizer-hold.ts` (hold check standalone: both closures hold when quiet, 20+25 s and 22+44 s), `patch.py`, `land.sh`, `test-addition.ts`. Raw: scratchpad `H61/run1.lines`.

## Update 10:01: LANDED (slot 10:00, lock bootstrap, landings.md "Wave 1127 H61")
bundle-settled 4/0, dangling-imports clean; `repo typecheck` did not run (typescript missing in node_modules on this host), bun parse ok. After-number: none yet (needs a run once workers restart on the landing).
