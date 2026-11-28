# Browser lane results (live end-to-end acceptance)

Status: BLOCKED before any browser step. No acceptance step was run; nothing below is claimed as passed.

## Steps
| Step | Result | Evidence |
|---|---|---|
| Start s shell (`repo run @semio-tech/framework-os-dev:dev-s-react-prototype --lang=en`, 6070) | FAIL (never served) | `🗑️generated/browser-s-shell.log`: launcher stayed in "Reading the project graph slice" from 15:19 to 16:13 (about 54 min). It never reached a cargo/rustc/wasm phase and port 6070 never answered. |
| gis3d (6083), gis2d (6040) | NOT RUN | Same planner gate; not attempted in parallel (memory). |
| verify-s-shell-terrain.mjs | NOT RUN | No server. |
| verify-terrain-maximum-grid-browser.mjs | NOT RUN | No server on 6083. |
| Map editor/viewer, middle-button pan, console | NOT RUN | No server. |
| e2e-*.png | none produced | |

## Why the launcher did not start (observed)
- Graph slice kept restarting from zero: attempt 1 reached 1655 completed owner checks, then "a new pass over the owners starts (1 path changed under the read)", later "reading the owners 581 of 3600", then a fresh "attempt 2" at 18 checks and "attempt 1" again at 16:12. The shared tree changes under the read, so the slice does not converge.
- Slice worker memory: log lines "the slice worker grew to 5862 MB / 5614 MB, more than the 0 MB a plan may take on this machine, so the work is cut smaller (step 2/3 of 6)". `repo fleet status` showed this host in warden "guard" pressure (free about 2.9 GB, swap 4.5 GB, interventions 37/34/0) and later graph-plan run 66820 at 6409 MB real. This matches the coordinator's digest (framework-surface-rs wasm "Slice worker exited with code null": the warden suspending/cancelling pool children under memory pressure, not a source fault).
- `repo leases status` at 15:46: graph builder pid 94543 held about 7 min, 10 recent lock-order violations (javascript-dependency-bindings / nx-tooling-provision requested while native-fixed-image held exclusive). Not investigated further.
- Compile lock: held 15:19 to about 15:35 by browser-lane, released at the coordinator's request (launcher was only reading the graph). Never re-acquired since no compile phase appeared.

## Fixes
None (no source failure was reached).

## Open
- Launcher cannot plan under current churn and 16 GB memory pressure; needs the graph-slice latency fix (other agent) or a quiet replica, then re-run the three steps above.
- I killed my own launcher process (pid in `🗑️generated/browser-s-shell.pid`) and its children; no dev servers remain running.
- Playwright (node 24, `chromium` channel chrome) is present in node_modules, so the verify scripts are runnable as soon as 6070/6083 answer; bun is not on PATH.

## Second attempt (21:31 to 22:23, after the coordinator's fixes)
Status: STILL BLOCKED before any browser step; no acceptance step run, no e2e-*.png.
- Command: `repo run @semio-tech/framework-os-dev:dev-s-react-prototype --lang=en`, log `🗑️generated/browser-s-shell-2.log`. Compile lock was not taken (no compile phase ever started; held by terrain-lane at the time).
- 21:31-21:57: first graph slice ran to completion pass 1 (1409 owner checks, then "reading the owners 96 of 3600"), then restarted at "attempt 2" (55964 observations) because the tree changed under the read (same churn as the first attempt).
- 22:07 onward: the next graph-plan stage queued for host capacity and never got it: "waiting for host capacity (not enough free memory); 2 active grants, 2 queued requests; the gate reports 3941 MB available above the 2048 MB floor; 6553 MB requested, minimum 5718 MB, queued for 929 s" (22:22). `repo fleet status` 22:13: host free about 5.0 GB of 16 GB, graph-plan pid 72415 blocked 7 min, a cargo source-admission run (pid 33015) also blocked. The minimum 5718 MB plus the 2048 MB floor needs about 7.8 GB free, which this 16 GB Mac does not reach while other agents, the Claude app and a 3 GB fseventsd (observed in `ps`) are resident.
- Root cause is the capacity gate versus machine memory (the graph-plan minimum 5718 MB is too large for this host while other work runs), not GIS source. Not fixed: needs either a smaller wave minimum for graph-plan or idle host memory (no other agents running). Existing note: `capacity-gate-starvation.md`.
- I killed my launcher (pid file `🗑️generated/browser-s-shell-2.pid`); no servers of mine are running.

## Third attempt (2026-11-27 00:07 to 03:50, fleet offload recipe, fast lock polling)
Status: STILL BLOCKED before any browser step; no acceptance step run, no e2e-*.png, no source change.
- Command: `env -u SEMIO_REMOTE_CACHE_TOKEN -u SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_OFFLOAD=distribute repo run @semio-tech/framework-os-dev:dev-s-react-prototype --lang=en` (launcher `🗑️generated/browser-lane-launch.sh`, log `🗑️generated/browser-s-react-3.log`).
- Lock: gismap-lane (`gismap-lane2.sh`) and ts-lane re-take the lock within milliseconds of each release; a 20 s poll never won (starved 00:08 to 00:58). A perl spin on `mkdir` (2 ms) won at 00:58 and again at 01:31 (held until abort at 03:50).
- 01:05 first plan failed fast: `BUILD_OWNER_FAILED`, 8 owners `invalid build.closure.provider-generated-import` (`@semio-tech/ui-rs:generate`). Cause (from `/tmp/dbg-pgi.log`): plugin store `packaging`/`installation` imported `manifest/localized-labels`, which imported the generated `ui-axes`; every provider port reaches the store. Another agent fixed `localized-labels` (reads the owned axes JSON) at 01:10; `provider-closures/🧪️tests/🛃️preflight` passes (28 s). Not my change.
- 01:32 to 03:50 the second plan did not converge: slice worker killed and restarted five times ("grew to 3372 MB, more than the 4169 MB a plan may take", "checking the file inventory" 1665 of about 1700 owner checks reached at 02:10, then a restart at 02:32), and between restarts the gate queued the next slice for 2 to 10 min ("waiting for host capacity (not enough free memory); 4 to 8 active grants, 3 to 6 queued; 6553 MB requested, minimum 6553 MB"). Two resident services (tags c7208dac666a and 241c2e22607a) each ran their own graph-plan queue, and other lanes' dry runs of gis-js/gisterrain/s-plugin-composition added more.
- No cargo/wasm phase and no port 6070/6083/6040 was ever reached; I started gis3d too early (queued behind s-react) and killed it. All my processes were killed, the lock removed, no servers remain.
- Root cause is unchanged: graph-slice restart-from-zero plus host capacity gate versus a 16 GB host shared by many lanes (`planner-slice-latency.md` addendum, `capacity-gate-starvation.md`). Needs the slice worker's retained rows capped or checkpointed per project (so a cut keeps progress), or a quiet machine.

## Fourth attempt (2026-11-27 04:40 to 06:12, own lock compile.lock5)
Status: STILL BLOCKED before any browser step; no acceptance step run, no e2e-*.png, no source change by me.
- Launcher `🗑️generated/browser-lane-launch5.sh` (s shell first, then gis3d, gis2d), logs `🗑️generated/browser5-*.log`. Lock compile.lock5 taken at once.
- 05:01 first plan died: slice worker exit 75 "Private Nx engine import missing" in `enforcement/🏗️compiler/🛠️native-compiler/🪟️windows/🟦️.ts` (relative import one `../` short, another agent's edit at 05:00); that agent corrected it at 05:01:35 (5 `../`). Not mine.
- 05:06 to 06:12 second plan: no slice restart this time, but the slice never got capacity: "waiting for host capacity (not enough free memory); 5 to 8 active grants, 2 to 5 queued; 4329 MB requested, minimum 2048 MB", queued 1456 s, one slice read ran 05:36 to 05:42 (166 owner checks) and was then requeued ("waiting for a host capacity decision on the 690 MB request", 05:38 to 05:55), then waiting again until I aborted. `repo capacity status` at 05:31: graph-plan pid 46370 held 4965 MB (growing to 5957 MB) while the same pid's second request (4329 MB) waited; host 3.7 GB available, swap 3.6 GB, Claude app 4.7 GB, 2 other agent sessions with grants.
- No cargo/wasm phase, no port 6070/6083/6040. All my processes killed, lock removed.
- Open: one pid should not queue a second graph-plan grant behind its own first one; and the gate needs idle host memory (about 6.5 GB above the floor for the plan plus a compile) which this 16 GB Mac does not have while the other lanes run.

## Fifth attempt (2026-11-27 06:12 to 08:57, lock compile.lock5, gated on the ts-lane plans)
Status: STILL BLOCKED before any browser step; no e2e-*.png, no GIS source change.
- Static readiness of the verify scripts (done while waiting): every selector, id and label that `verify-s-shell-terrain.mjs` and `verify-terrain-maximum-grid-browser.mjs` use still exists in the current tree (terrain-feature-list, action.clearOverlays/exportElevationGrid/importElevationGrid, importElevationGridFile, terrain-elevation.asc, Raise Terrain, s.gis.gisterrain@1 editor/viewer ids, semio-world-3d-host, playground.navbar.fixture/roles, window-engagement-overlay, s-home-create-space, s-space-create-artifact, Institut de Botanique, `.asc` accept in import-elevation). `terrain-browser-fixtures.mjs` resolves (maximum raster 4225 samples, 65536 bytes, 2 chunks; nodata fixture loads); playwright is installed. Ports match the registry table (6070 s, 6083 gis3d, 6040 gis2d). No drift, nothing changed.
- Launch gating: launcher `🗑️generated/browser-lane-launch8.sh` (restarts a dead run up to 6 times, logs `browser8-*`). Waiting for the ts-lane plans to end took until 06:40; then the s shell plan waited for a free slice worker (2 of 2 busy, 06:55 to 07:25), for host capacity (512 MB run admission up to 800 s), and for 846 MB / 509 MB capacity decisions.
- The `repo run` CLI exits after 3 hand-overs ("the repo graph service ends (superseded) before this graph read finished", `CUSTODY_POLICY_V1.maximumHandovers`), seen at 07:39, 08:00 and 08:40: every edit under `🏡️resident/🏛️service` (08:00 host `🌐️host/🟨️.mjs`) supersedes the resident generation again. The launcher restarted it and the read resumed from kept work, but did not finish before I stopped at 08:57.
- Nothing served on 6070/6083/6040. Processes killed, lock removed.
- Needed: a window of about 15 minutes with no edits under the repo service/resident modules and no ts-lane plan, or a run launched by the coordinator on a quiet host.

## Sixth attempt (2026-11-27 09:32 to 11:27, lock compile.lock5, no TS-lane gating, restart loop kept)
Status: STILL BLOCKED before any browser step; no e2e-*.png, no source change.
- Launcher `🗑️generated/browser-lane-launch9.sh`, logs `browser9-s-react-*.log`. The supersede fix works: no hand-over exits this time (one run, no restart needed).
- 09:32 waited for the previous service generation (pid 13577) to finish its slice read; 09:33 to 10:26 the slice request queued for host capacity (4655 MB graph-plan, "7 to 8 active grants, 3 to 6 queued, 0 MB available above the 2048 MB floor"); `repo capacity status` showed 4 other lanes' cargo-orchestration runs holding 1536 MB each (about 6 GB) plus two more graph-plan waiters (pid 62932, 13577) and a 3252 MB dependencies request.
- 10:26 to 10:53 the slice read ran 1 to 1700 owners admitted, 655 of about 1700 owner checks, then was cut and restarted twice (10:58 and 11:17; restart showed "1 completed owner checks" again, so the owner-vouch ledger did not carry the 655 completed checks across the restart) and queued again for capacity (556 to 846 MB decisions waited 10 min each).
- Nothing served on 6070/6083/6040; I stopped at 11:27, killed my processes, removed the lock.
- Needed: (a) restart resumption of completed owner checks (the second and third read pass restarted at 1 completed check), (b) the cargo lanes must leave about 6.5 GB so the plan can finish, or the plan run before the cargo lanes start.

## Seventh attempt (2026-11-27 11:41 to 14:14, coordinator's exclusive planner window, then own launch)
Status: STILL BLOCKED before any browser step; no e2e-*.png, no source change.
- 11:44 to 13:05 the coordinator's coalesced s shell + TS plan (`excl-s-react.log`) queued 20 min for capacity ("an earlier request still waiting holds the next reservation"), read about 1700 owners twice (restarted at 12:43 and 12:51, "75" and "60 completed owner checks" at the restart: ledger resume still not visible), and ended 12:59 with "Original Script process terminated (reference 557d4d71)". The coordinator stopped it.
- 13:32 the TS-only plan finished (`exclusive-plan.active` gone). My launcher `🗑️generated/browser-lane-launch11.sh` started the s shell on compile.lock5 at 13:33. Attempt 1 ended at 13:36:59: "The resident service met the same unexpected failure 4 times in a row (reference a4605e03): Original Script process terminated ... deterministic" (`🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:282`); the launcher's restart loop relaunched it at once.
- Attempt 2, 13:37 to 14:14: "waiting for a free slice worker (2 of 2 busy, 1 load ahead)" 13:37 to 13:55, then "waiting for host capacity (not enough free memory); 5 to 6 active grants, 1 to 3 queued; 306 to 1074 MB available above the floor" from 13:56 until I stopped at 14:14 (17 min).
- Nothing served on 6070/6083/6040. Processes killed, lock removed.
- Open: (a) the "Original Script process terminated" failure that the resident service counts as deterministic although it is not (it is the plan script being terminated while the generation changed or the warden acted; the service should not give up after 4); (b) host memory: the Mac needs about 6.5 GB free for a plan plus the compile.
