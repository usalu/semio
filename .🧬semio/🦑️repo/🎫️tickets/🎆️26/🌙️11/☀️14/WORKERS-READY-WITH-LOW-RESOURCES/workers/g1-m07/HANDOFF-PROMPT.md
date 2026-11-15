# Handoff prompt for the worker agent on g1-m07 (written 2026-11-15T03:50Z)

Paste everything below the line into a new session whose working directory is `/Users/ueli/Documents/semio`.

---

You are the agent on the semio fleet worker `g1-m07` (Mac mini, 16 GiB, macOS 27.2.1, worktree `/Users/ueli/Documents/semio`, uid 501). A previous session of yours followed the worker protocol up to round 17; you continue from there.

## Your task
Read `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES/PROTOCOL.md` (call that folder `W`, and `W/workers/g1-m07/` `M`). Follow it exactly and keep following it until it tells you to stop: you run the native bootstrap when a round says so and report into `M`. Re-read `PROTOCOL.md` and `W/ROUND.md` (or `M/next.md` if it exists) at the start of every round; the coordinator changes `ROUND.md` often and skips round numbers (9, 14 and 16 were never seen here; a round file contains the older rounds below the newest one and they still apply where the newer text says "as in round …").

## Hard rules
- Change no file outside `M`. No build code, no fixes: observe and report. (Exception named by a round: logs under `$HOME`, e.g. `~/native-bootstrap-round-<n>[-try-<k>].log`, `~/tools-setup-round-16.log`.)
- No git command that changes anything, no git worktrees, no `pkill`/`killall`, no `sudo`. Kill only processes you started, with one SIGINT to their process group.
- Never print tokens: `~/.zprofile` holds `SEMIO_REMOTE_CACHE_TOKEN`; do not copy values of an `environment` block into a report.
- One graph command at a time on the machine (`repo run`, `repo show`, `repo cargo`, `repo cache gc`, the bootstrap).
- You cannot click macOS consent dialogs or change System Settings; the owner must. Tell them plainly when a try ends with `native_rc=4`.
- Write numbers as measured, "not measured" otherwise. Reports are files in `M`; in chat give a short status and point to the file.

## State when this was written
- `W/ROUND.md` says `round: 17`. Reported so far: rounds 1, 2, 3, 5, 6, 7, 8, 10, 11, 12, 13, 15 (`M/round-<n>.md`, each with `.tail.txt`, samples and monitor files). `M/round-17.md` does NOT exist yet.
- The machine has never been ready: `.cache/f/native-bootstrap.done` is absent and no setup task has ever run; every try failed inside `workspace:setup` while reading the project graph.
- Last finished try (round 15 try 2, 03:37:55Z): `native_rc=4`. The graph service under the launcher was denied by macOS, was started again under the access of the calling app and was denied again (reference da146983, `fstatat failed (EPERM)`, no `where: root=` line in the service log). Receipt `~/Library/Application Support/semio/access/3f08bbea4d32/service.json` said `"state":"denied"`. The owner was asked to switch on Documents Folder for `repo` (`/Users/ueli/.local/share/semio/launcher/bin/repo`, identifier `tech.semio.repo`) under System Settings, Privacy & Security, Files & Folders; whether they did is not known. Check the receipts (`client.json`, `launcher.json`, `service.json`) before the next try and report them.
- Launcher since round 8 try 1, unchanged in every later try: link `~/.local/share/semio/launcher/bin/repo -> versions/cf25f2b751f83336/repo`, target inode 3496898, mtime 2026-11-14T22:55:52 (CET), CDHash `6de7547a8b8ff384bbfbfc45e962462063ff0357`. Keep reporting whether it changed.
- Other failures seen so far, for comparison: fleet refusals (`native_rc=75`, few or no projects delivered), `graph-plan-too-heavy` in stage `inference` (6570 and 6586 MB against the 6553 MB limit), missing `.cache/b/g/m/build-input-closures/<hash>.json` (ENOENT in `readInventoryRecordLeanV1`; one `repo cache gc --area=graph-inference` did not help and ended with exit 124), slice worker exit 124 after 48 minutes without progress, "the code on this machine kept changing under it".

## What was in flight for round 17
The previous session started, as one background shell, `bash "$M/tools-setup-round-16.sh"` (step 0 of round 16: `repo tools setup all` once, log `~/tools-setup-round-16.log`, notes in `M/round-17.tools-setup.txt`; it had been running since 03:39:04Z) followed by `bash "$M/drive-round.sh" 17 1 5`. That shell may have died with the old session. First find out what is true:
1. `ps -axo pid,ppid,etime,args | grep -E 'tools-setup-round-16|drive-round|run-round|native-bootstrap|p12-sampler' | grep -v grep` — if any of these still run, do NOT start a second copy; wait for them (they are yours: same account, started by the previous session).
2. Read `M/round-17.tools-setup.txt` (has an `end … exit_code=… wall_seconds=…` line when step 0 finished), `M/round-17.tries.txt` (one line per finished try), `M/state.json`, and `ls ~/native-bootstrap-round-17*.log`.
3. If step 0 never finished, run it again once before the next try; if it finished, do not repeat it (it is "once per machine"). Report its exit code, wall time and last 15 lines.
4. Then run or continue the tries of round 17 with the next free try number, or go to the newest round if `ROUND.md` has moved on (write the report of the round you leave first).

## Tools of the previous session in `M` (yours to reuse and edit)
- `run-round.sh <round> [try]`: one try of `W/native-bootstrap.sh` in its own process group, log `~/native-bootstrap-round-<n>[-try-<k>].log`; samples every 30 s (`ps`, `kern.memorystatus_vm_pressure_level`) into `round-<n>[-try-<k>].samples.txt`; one SIGINT on: a process of the run above 8 GB resident, pressure level 4 in two samples in a row, no new progress line for 15 minutes in "Reading the project graph slice" (writes `….stall.txt` with the last 60 service log lines and `ps`), the dependency store wait standing 5 minutes (writes `….store-wait.txt`). Level 2 is recorded only. Result in `….monitor.txt`.
- `drive-round.sh <round> <first try> <last try>`: around every try the launcher identity (`round-<n>.launcher.txt`) and the P12 sampler (`round-<n>[-try-<k>].p12-samples.txt`), `state.json` after every try, 3 minutes between tries; ends on `native_rc` 0 or 4, or after two tries in a row with the same other error; one line per try in `round-<n>.tries.txt`. Run it in the background and give it at most 4 or 5 tries per call so that you read the results in between (limit of the round: 12 tries).
- `p12-sampler.sh <out>`: the sampler of `W/ROUND-NOTE-P12.md` section 1.
- `round-13-service-sections.sh <since UTC> <reference>`: prints the service-log sections of a report (restore line with A and S, plan stage lines, fleet / too heavy / `closure-missing` / `graph-plan-too-heavy` with the 20 lines before it, the stack of a reference). Service logs: `$TMPDIR/semio-repo/501/host-*.log` and `exit.log`.
- `wait-round.sh <reported round>`: polls every 60 s, at most 6 hours, for a higher round (exit 0) else exit 2.
- `round-13.md` and `round-15.md` are the templates for a report of the current kind.

## Per round
- Report `M/round-<n>.md`: machine, replica state and code digest (`.cache/f/health.json`, `launchctl print gui/501/tech.semio.fleet.replica`), launcher identity, a table of the tries (start, end, `native_rc`, peak resident, pressure samples), the final client line of every try verbatim, and what the round asks for. Round 17 asks in addition for the first plan line of every try verbatim (`… about 3512 MB per project), it can infer 192 of the others (48 per wave)`; if it says 0 MB add `vm_stat` and `memory_pressure | tail -3`), the restore line (`restored … from a state A min old with S projects stale`), `taken from the fleet` / `inferred here` / `too heavy` lines, plan stage lines, the sampler's peak with its progress line, and for setup tasks the succeeded / failed / not-run lists with the first error line and 15 lines before it.
- `M/round-<n>.tail.txt`: last 200 lines of the last log. `M/state.json`: `{"origin":"g1-m07","round":<n>,"native_rc":<code>,"ready":<bool>,"failed":[…],"at":"<UTC>"}`.
- `native_rc`: 0 ready when the marker exists; 75 refusal (fleet, too heavy, plan too heavy) → wait 3 minutes and try again; 4 → your own process cannot read the worktree either: stop the series, report "consent", tell the owner; 7 after a hard stop; 1 unexpected.
- When `native_rc=0` and `.cache/f/native-bootstrap.done` exists: `ready: true`, run `repo preflight --lang en` and the replica `status`, add both to the report, and stop for good.
- After a reported round: `bash "$M/wait-round.sh" <n>` in the background; if nothing higher comes within 6 hours, stop and tell the owner.

## Background goal of the owner (still open)
"Finish setting up this machine as worker. Make sure there is only one canonical semio folder etc, same state as the others and all local changes of the old semio are merged into the canonical folder." The folder work is done (`/Users/ueli/Documents/semio` is the one canonical worktree); what remains is this machine becoming ready through the protocol above.
