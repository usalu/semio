# e2e-f (wave 1025f) result

FOR graph-f 00:05: my gate (run e2e-1025f-e1, wrapper started 21:52, i.e. after the 21:49 flip) is STILL queued for `repository-graph-facts` shard 7 at 00:05 (every `graph: waiting for the graph build of a peer (7341 s in the queue, generation 473 was the newest when this command queued)`; position 2 of 7-8 behind holder pid 21103, lstart 22:54:43, 99 % CPU, 70+ min CPU). Stamped generations 475/476 exist but my waiter was not served 'covered' while 21103 builds. Is 21103 an old-code process? Please check whether my queued command can be covered without waiting for 21103.

Status 23:47 (10/25): e1 passed the graph queue at ~23:25 and runs `build:graph-plan` (1 h 36 m wall since launch); no spec yet.

Status 23:20 (10/25): e1 in the graph-facts shard 7 queue (position 3 of 5 at 23:12, holders change every few minutes); no spec yet.

Status 22:54 (10/25): e1 still queued for the graph build (peer pid 64323 now 2 h+); nothing else of mine running (wrapper 68135).

BLOCKED 22:30 (> 30 min, not mine): e2e-1025f-e1 waits for `repository-graph-facts` shard 7 (exclusive), position 5 of 6, behind pid 64323 (lstart Sun Oct 25 20:38:45 2026, 97 % CPU, 57 min CPU, held 4784 s): `bun --eval` graph build script from the shared bun cache. Not killed.

Status 22:28 (10/25): e2e-1025f-e1 still waiting for the graph build of a peer (`graph: waiting for the graph build of a peer`, ~20 min in the queue); no spec yet. Disk 45 GiB.

ANSWER to coordinator (22:10): the SIGTERM to e2e-1025f-d1 at ~21:52 was SENT BY ME (`run.sh cancel --run-id e2e-1025f-d1`, after killing the retry wrapper pid 81800 so it would not loop), in response to your phase-2 note (my run had spent ~1 h 56 m in `build:graph-plan` and was started before 21:18). No peer or watchdog involved (orchestrator log shows the graceful cancel). The DEV topology run is ALREADY RELAUNCHED: `retry.sh e2e-1025f-e dev` (wrapper pid 68135), run e2e-1025f-e1 passed capacity and is in `build:discover` at 22:10.

Status 22:05 (10/25): coordinator 22:02: graph throughput fix live; my d1 (2 h in graph-plan behind peers' graph-facts queue) cancelled by me (SIGTERM, graceful) and relaunched as `retry.sh e2e-1025f-e dev` (wrapper pid 68135, log `🗑️generated/e2e-f/dev-retry2.log`, run ids e2e-1025f-e1..). e1 in the capacity queue (11 running, load 25).

Status 21:02 (10/25): d1 (pid 81800 retry wrapper; log `🗑️generated/e2e-f/dev-retry.log`) passed capacity at 20:25, now spends its time in `build:graph-plan` behind peers' `repository-graph-facts` shard 7 exclusive requests (position 8 of 9, holder pid 80316 `bun --eval` since 19:53); no spec has run yet. Static work meanwhile: the b1 failure's trace (`🗑️generated/e2e-f/trace-b1-en320`, from the crel2-d rehearsal) shows all assets 200 (fonts, chunks, proctor ws), no failing request, 320x1200 window, Show pets checked, free ground in the screenshot but no `svg.pet` for 480 s of the pets' time: not a load/CSP/asset fault.

Status 19:21 (10/25): DEV topology run e2e-1025f-2 launched 19:10 (`run.sh course --phase e2e --course-topology dev --go --skip-preflight`; orchestrator started by `detach.py`, log `🗑️generated/e2e-f/dev2.log`, gate log `🗑️generated/e2e-course-e2e-1025f-2-course-test-e2e.log`, report will be `report-e2e-orch-e2e-1025f-2.md`). Phase `e2e` only (not `all`): cold/warm are course-f / gen-f's lanes, a warm failure skips e2e (course-d 10/24). Run e2e-1025f-1 (`--phase all`) was cancelled by me at 19:10 (SIGTERM, graceful). Currently queued behind `repository-graph-facts` shard 7 (peers). Disk 62 GiB, swap 0.8 GB.
Reload question (static): `useAddress` (course react `🚏️navigation`) reads the hash on the first layout effect; a real reload keeps the URL and the restored screen, a hash set programmatically before the mount is the "arrival address" and is ignored when a run is restored (documented semantics). A learner who reloads mid-run is not affected; the spec set the hash right after `reload()` before the client mounted, hence the `#course-main` wait in `openGrid`.

Status 20:20 (10/25): e2e-1025f-2 ended 19:52 BEFORE any spec: graph plan took 1066 s (peer churn), then `Exact original parent child allocation required` (native-build storage `readNativeStorageMountedChildV1`, infra, transient). Relaunched via `e2e-f-tools/retry.sh e2e-1025f-d dev` (wrapper pid 81800 via detach.py, run ids e2e-1025f-d1.., log `🗑️generated/e2e-f/dev-retry.log`): d1 waits in the `[capacity]` queue for `run` (need 3584 MB, host short on memory by peers' 8 running jobs) since 19:55.
