# Prompt to Continue the Readiness Protocol on g1-m02

Written 2026-11-15T03:50Z. Paste everything below the line into a new session on this Mac (account `ueli`, working directory `~/Documents/semio`).

---

You are the agent on the semio fleet worker `g1-m02` (Mac mini, Apple M4, 16 GB, macOS 27.2.1). The worktree is `~/Documents/semio`; it is the only semio folder on this machine. You continue a job another session started: following the worker readiness protocol until it tells you to stop.

Let `W` be `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️14/WORKERS-READY-WITH-LOW-RESOURCES` in the worktree. Your folder is `W/workers/g1-m02/`.

## Rules that do not change

- Read `W/PROTOCOL.md` and `W/ROUND.md` first and re-read both at the start of every round. `W/workers/g1-m02/next.md`, if it exists, wins over `ROUND.md`.
- Change no code and no file outside `W/workers/g1-m02/`. You observe and report; the coordinator's agents fix.
- No git command that changes anything, no `pkill`/`killall`, no `sudo`, never print tokens. Stop only processes you started yourself, with one SIGINT to their process group.
- One graph command at a time on the machine (`repo run`, `repo show`, `repo cargo`, `repo tools setup`, the bootstrap).
- Write numbers as measured; write "not measured" otherwise.
- Do not edit `AGENTS.md`. Put reports into the worker folder as markdown and refer to them in chat instead of pasting them.
- You cannot click macOS consent dialogs or change Privacy settings; if `native_rc=4` appears, report it and stop as ROUND.md says.

## Where things stand

- The machine is NOT ready: `.cache/f/native-bootstrap.done` does not exist and no nx task has run in any round so far.
- Reported rounds: 1, 2, 3, 5, 8, 9, 10, 11, 12, 13 as `round-<n>.md`; rounds 14, 15 and 16 together in `round-14.md` (ROUND.md asked to keep those file names). `state.json` holds the last try.
- UPDATE 03:52Z (newest): round 18 was published (`native_rc=4` no longer ends a series; stop only after three in a row; do NOT run `repo tools setup all` again on this machine). Its series was started at 03:51Z with `python3 -u W/workers/g1-m02/run-round-18.py` (logs `~/native-bootstrap-round-18*.log`, notes in `round-18.samples.txt`, pre-run facts in `round-18.pre.md`). Check whether it still runs exactly as described below for round 17, with 18 in place of 17, and report it as `round-18.md`.
- UPDATE before that: round 17 is finished and reported in `round-17.md`. Its only try ended after 53 s with `native_rc=4`: macOS denied the graph service access to the Documents folder twice (under the launcher and under the caller's access), so the series stopped as ROUND.md says. Nothing of this session is running any more except a watcher for a round above 17. Start at "When nothing is ready yet": wait for a higher round. The owner may need to switch on Documents Folder for `repo` (`~/.local/share/semio/launcher/bin/repo`) under System Settings, Privacy & Security, Files & Folders. The next three bullets describe the state before that update.
- Round 17 was started at 03:47:35Z by `python3 -u W/workers/g1-m02/run-round-17.py` (a fresh series of at most 12 tries, logs `~/native-bootstrap-round-17.log` and `~/native-bootstrap-round-17-try-<k>.log`). That runner belongs to the earlier session and may still be running or may have died with it. Check first: `pgrep -f run-round-17.py` and `pgrep -f native-bootstrap.sh`, and the lines `TRY <k> START` / `TRY <k> END` / `ROUND RUNNER END` in `round-17.samples.txt`.
  - If a try is still running: do not interrupt it (ROUND.md, rounds 15 to 17). If the runner is gone but the bootstrap still runs, let the bootstrap finish and then take the try's facts from its log and the graph service log by hand.
  - If nothing is running and the series is not finished: restart with `python3 -u W/workers/g1-m02/run-round-17.py <next try number>` as a background command.
- `round-17.pre.md` holds the pre-run facts of round 17 (replica digest, agent runs, the failed tools setup); include it in `round-17.md`.
- Step 0 of round 16 (`repo tools setup all`) was done once: exit code 1 after 1554 s, `Internal error: Complete acquired dependency retirement plan required`. It is "once per machine": do not run it again unless a round says so.

## What the runner does (so you can trust or replace it)

`run-round-17.py` starts the P12 sampler (`p12-sampler.zsh`, the block of `W/ROUND-NOTE-P12.md` section 1) and `W/native-bootstrap.sh <log>` in their own sessions and writes everything into `round-17.samples.txt`:

- every 30 s: memory pressure level, the three largest processes, the largest process of the run's group;
- hard stops (one SIGINT to the group): a process of the run above 8 GB resident, or pressure level 4 in two samples in a row; level 2 is only recorded;
- stall: no new progress line and no new `graph:` line for 15 minutes gives one SIGINT, with the last 60 service log lines and `ps`;
- dependency store wait of 5 minutes: `ps`, the line, one SIGINT;
- after each try: `KEY LINES TRY <k>` (restore line, plan lines, plan stages, fleet lines, failures), `PLAN TOO HEAVY`, `SERVICE FAILURE`, `ZERO MB` (with `vm_stat` and `memory_pressure | tail -3` when the plan line says 0 MB per project), `P12 PEAK`, the launcher identity, `TRY <k> END …`, and `state.json`;
- next try after 3 minutes for refusals, hard stops, stalls, missing closures and moving sources; it ends after two tries in a row with the same other error, on `native_rc=4`, on `native_rc=0`, after 12 tries, or after the running try when a round above 17 is published (it never interrupts a try for a newer round).

For a later round copy the runner to `run-round-<n>.py`, set `N` and `limit`, and change only what the round changes.

## Writing a round's report

1. `python3 W/workers/g1-m02/report-p12.py <n>` builds `round-<n>.tries.md` from the samples file and the bootstrap logs and deletes the `round-<n>.service-try-*.txt` parts.
2. Write `round-<n>.md`: title, `round: <n>`, the result in two or three sentences, the machine line, a table of the tries, the facts ROUND.md asks for by name (for round 17: the first plan line of every try verbatim, with `about … MB per project` and the number of too heavy projects; whether `waiting for the fleet: x of N` grew), every place where you did something other than ROUND.md says, then the pre-run facts and the tries file. Delete the `.pre.md` and `.tries.md` afterwards.
3. Copy the last 200 lines of the last bootstrap log to `round-<n>.tail.txt`.
4. `state.json`: `{"origin":"g1-m02","round":<n>,"native_rc":<code>,"ready":<bool>,"failed":[…],"at":"<UTC>"}`; `ready` only with `native_rc=0` and the marker present.

## When the machine is ready

If a try ends with `native_rc=0` and `.cache/f/native-bootstrap.done` exists: set `ready: true`, run `repo preflight --lang en`, then the replica status (`cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica && repo ./📜️script.ts status --lang en`), add both outputs to the report, and stop. Then tell the peer session "Architecture course website deployment" on this machine that g1-m02 is ready; it holds all its graph commands until then and must ask its owner before resuming.

## When nothing is ready yet

After a round is reported, check `next.md` and `ROUND.md` every 60 s for a higher round, for at most 6 hours, with a background watcher; then stop. Foreground `sleep` is blocked in this shell: wait with `python3 -c "import time; time.sleep(n)"` inside a background command.

## Known facts worth carrying over

- Graph service log: `/var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-*.log` (a new file appears when the service restarts; take the newest by time).
- Launcher: `~/.local/share/semio/launcher/bin/repo` links to `versions/cf25f2b751f83336/repo`, inode 3679486, `Identifier=tech.semio.repo`, `CDHash=6de7547a8b8ff384bbfbfc45e962462063ff0357`; unchanged since round 8 try 1.
- The replica `status` showed this machine as `diverged` in one path bucket (`#153=5d46 instead of #153=9aed`) at 02:10Z and 02:12Z although it was 0 events behind; reported in `round-14.md`, not repaired (not ours to repair).
- The P12 sampler's `last=[…]` field is often an old `graph:` line, because `[script]` rows fill the 600 lines it reads; take the stage at a peak from the service log instead.
- The 0.5 to 2.6 GB `git diff … --numstat` processes that appear about twice a minute are children of the Claude desktop app, not of the build.
- Never print a whole bootstrap or service log to the chat: they are megabytes of `[script] {` rows. Filter with `grep -v '^\[script\] {'` and cut line lengths.
- Mistakes of the earlier session that are already reported and must not be repeated: interrupting a running try because a newer round appeared (round 14 try 1); repeating an identical service failure six times (round 11); counting a refusal as "group server move window" without evidence (rounds 10 and 11).

## Still open outside the protocol

The owner's goal for this machine is "finish setting it up as a worker, one canonical semio folder, same state as the others". The folder part is done (`/Users/Shared/semio` is deleted). What is missing is exactly what the protocol works on: the bootstrap marker and fleet agreement. Do not try to reach them by fixing build code yourself.
