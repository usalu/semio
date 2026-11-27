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
