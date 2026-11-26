# F59 result: g1-m05 readiness beat stopped (ready.json 23:30, memory.json 23:59)

## Evidence (S/origins/g1-m05, read 02:30 CET)
- ready.json: pid 31964, state `running`, `ready:false`, stage dependencies attempt 49 ("adopted"), updatedMs 23:29:58, `beatMilliseconds` 30000. The supervisor log's last line is 23:29:51. memory.json (written by the same readiness process: warden mirror) stopped at 23:59:56. attest.json 01:47, head.json 02:29 (replica alive, publishing).
- attest.json `failures`: `loop-blocked-restart` count 83. refused.json: current code `bfeebe98` refused 7 times at 02:22 with "the watchdog ended this code while it was new" [transient 7]. So the replica daemon of m05 is being killed over and over while the readiness supervisor is dead or wedged.
- attest lists no services or pids; there is no sidecar health in the store (worker-side `.cache/f/sidecars` is not readable from here): the exact state of pid 31964 (dead vs. alive but blocked) is NOT provable from the store.

## Root cause (code-proven, two defects that feed each other)
1. Nothing ever ends a wedged readiness child. The launcher (`runRemoteExecutionSidecarLauncherV1`) keeps its own record fresh every 2 s whether or not its child works, and a restarted daemon adopts a launcher whose record is fresh and whose entry hash is unchanged (`claim` in `🧷️sidecars`). A readiness supervisor that is alive but blocked (m05 load, F41 Windows process-start reads are synchronous `powershell` spawns of about 1 s, F14 descriptor sessions) is therefore adopted again and again; only a crash restarted it.
2. The only watchdog (`💔️silence`, used by the replica boot supervisor) killed the whole replica daemon when ready.json was silent: it fixed nothing (the launcher survives and is re-adopted) and it read the STALE ready.json of the wedged sidecar as soon as a fresh daemon became ready. A daemon younger than `earlyMs` (60 s) killed this way counts as a failed code: the code is refused and an older sealed snapshot runs, which is the `refused.json` rotation and the 83 `loop-blocked-restart` / "watchdog ended this code while it was new" entries seen on m05. The watchdog was the loop, not the cure.

## Fix (schema first, en/de, atomic, `T/🧰️scripts/F59/apply.py`)
- Readiness schema: `Policy.silenceChildGraceMilliseconds` 180000, `heartbeatNames` now `pool.json, attest.json, head.json` (a machine whose replica still publishes its head is alive), new `SilenceTexts` (en/de `childRestart`).
- `🚦️ready/💔️silence/🟦️.ts`: `readySilenceChildWatchV1` (a child younger than the grace is never judged, one kill per cooldown across the children of a launcher), `readySilenceChildNoticeV1` (bilingual), `readySilenceStatusFileV1`, `readySilencePolicyV1`.
- `🛰️sidecar/🟦️.ts` launcher of role `ready`: every 5 s reads `<store>/v1/origins/<origin>/ready.json`; when a not-ready record owes beats and missed 3 (90 s declared beat) and the child is older than 3 min, logs the bilingual line and `SIGKILL`s the child only (TerminateProcess on Windows); the launcher stays, restarts a child after its back-off and the new child adopts the running stage. The store being unreadable never kills anything. Env overrides for tests: `SEMIO_SIDECAR_SILENCE_GRACE_MILLISECONDS`, `SEMIO_SIDECAR_SILENCE_POLL_MILLISECONDS`.
- `🔁️replica/🛟️supervisor/🟦️.ts`: the daemon-level silence kill and its helpers are removed (single owner: the launcher).
- Fleet status: `workers.state.silent` is now "stale readiness beat" / "Bereitschaftsschlag veraltet", finding `worker-silent` says the beat is stale (en/de), golden fixture line updated. With `head.json` as heartbeat m05 shows `stale readiness beat`, not `offline`.

## Proof
- New: `💔️silence/🧪️tests/🔪️child` 4/0 (incl. 500 seeded cases against the independent Python oracle `🐍️.py` with its own cooldown), `🧪️tests/🔪️silent-child` 1/0 (a real launcher with a wedged stub child and a stale ready.json: the child is killed, a second pid starts, the launcher lives, 1.5 s), `🩺️workers/🧪️tests/💓️head` 1/0 (live head + stale beat = silent, no head = offline), `fleet-status/🧪️tests/🫀️stale-beat` 1/0.
- Existing green: silence 6/0, workers 3/0, fleet-status 30/0 (after fixture text), replica `🛟️boot.ts` 24/0. Ready main 58/0, replica entry 24/0, sidecar main 6/1 (the F47 signed-scheduler test, failing independently). dangling-imports clean for the touched folders (one pre-existing dangling `🪜️step-overdue-f57` test import of F57).

## NOT verified
- Nothing ran on Windows. That the m05 supervisor is wedged rather than dead is inferred (launcher design), not observed; if the launcher itself is dead the launcher/daemon adoption path (`claim`) already restarts it after 15 s.
- The first `SIGKILL` on m05 happens only after the new replica code runs there; m05's replica is flapping (loop-blocked-restart), so it may take a few cycles. Check `ready.json` `updatedMs` moves and the sidecar log for "the readiness supervisor wrote nothing".

## USER (physical Windows box g1-m05)
1. If `ready.json` of m05 still does not move 10 min after this landing: in Task Manager end `bun.exe` processes whose command line contains `sidecar` + `ready` (the launcher), the replica task restarts it within 15 s (`schtasks /Run /TN semio-fleet-replica` if it does not).
2. Exclude `C:\semio` from Defender real-time scanning (F4/F14 still open): the synchronous process-start reads and 9-14 s per binding come from there.
3. Read `.cache\f\sidecars\ready.log` (last 50 lines) and send the line "wrote nothing" if present; it names the age and missed beats.
