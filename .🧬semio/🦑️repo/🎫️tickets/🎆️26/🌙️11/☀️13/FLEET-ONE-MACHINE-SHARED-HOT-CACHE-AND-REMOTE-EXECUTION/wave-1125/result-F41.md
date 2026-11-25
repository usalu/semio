# F41 result: pid reuse on g1-m05

## 1. What produced the judgement
`changedLineV1` in `🚦️ready/🟦️.ts` (watcher, `readyIdentityV1` + `readyConfirmProcessReplacementV1`). The stage root was recorded as `{pid, startMs: Date.now() before spawn}`, and the process table reports the real creation time. On busy Windows (Defender, load) the spawn took 17 s, so "seen 17:25:26.9 vs recorded 17:25:10.4, tolerance 15 s" was judged a reused pid in two reads and the stage was ended with `stage-failed`. It was NOT a reuse: the same process, a lower-bound clock mistaken for its creation time. The failed run also hit the adoption (supervisor 2444 adopted at 18:16:45).
Note: `javascript-installed-paths:N` is an entry counter in a progress resource name, not a pid-keyed hold; kernel-held leases (SQLite) never use pids.

## 2. Fix
- One shared identity: new `🧰️framework/🔨️modules/🏃️process/🔖️identity/` (schema first: `ProcessIdentity {pid,startMs}`, policy tolerance 3 s / spawn grace 300 s). `readProcessStartV1`: Windows `Win32_Process.CreationDate`, macOS/Linux `ps -o lstart=` (C locale, UTC). Pure verdict same/gone/reused/unknown; unreadable never frees a holder.
- Ready sidecar: root is pinned from the first table observation inside the spawn window (persisted in `<stage>.tree.json`, restored by an adopting supervisor); a row outside the window is a real reuse. Adoption (`running.json` record) and the `inflight` port judge by pid + creation time (`stageProcessAliveV1`). `readyIdentityV1` and `readyProcessStartSyncV1` now delegate to the shared module (single definition).
- Windows holes closed: warden lock start was `""` on win32 (pid only) and grant start was `undefined` on win32 (reuse never detected); both now use the shared reader.
## 3. Tests
New `🔖️identity/🧪️tests/🟦️.ts` (6/0): schema, reuse simulation (same/other start/gone/unknown), the m05 17 s slow spawn (pinned, then a later process on the pid is reused), adoption verdicts, POSIX parse vs independent Python `calendar.timegm`, real child vs independent `ps etime`. Also process-tree 16/0, pid-reuse 3/0, warden 49/0. Landing in `T/landings.md` ("Wave 1125 F41").

## Not done / not verified
- `🪜️step-overdue` has one failing test (verified-memo step watch, slice-report logic), not caused by this landing but not proven pre-existing.
- Not converted (still pid-only `process.kill(pid,0)`, ~50 sites listed by `grep "process.kill(.*, *0)"`): e.g. leases ticket probe, replica/journal/supervisor, unit-cache lock, graph admission, remote-cache store, cas pins/collector, bazel sessions, cache-gc, agent sessions. Each needs its persisted record to gain `startMs` (schema first); the shared function is ready for them. Highest value next: unit-cache `🔒️lock`, cas `📌️pins`, graph `🚏️admission`, replica `🛟️supervisor`.
- Windows reader costs one powershell spawn (about 1 s) per call; used only at adoption and stale-lease/grant checks, not per tick. Not run on Windows; m05 behaviour after the landing is unobserved (live m05 attempt 49 runs old code until its next start).
