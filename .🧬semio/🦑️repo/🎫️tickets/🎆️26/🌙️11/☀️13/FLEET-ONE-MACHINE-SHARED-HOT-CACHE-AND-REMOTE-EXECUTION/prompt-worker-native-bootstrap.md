# Prompt for an agent on a worker: retry the native bootstrap

## Status written by the coordinator, 2026-11-14 10:10 (supersedes the 09:05 status below)

Can a worker run the native bootstrap now: EXPECTED YES, not yet seen on a 16 GB machine.
- The group server completed a graph build (generation 79, 5 100 projects, 2 202 s, uploaded 09:45) and keeps publishing (generation 83 at about 10:05).
- Restore from the fleet is proven live on the group server: after `repo reset`, `repo show projects` printed `restored from the remote cache, 5104 documents (47 MB) downloaded in 8 s`, refreshed 4 stale projects, total 261 s, rc 0 (package X).
- A read that infers nothing peaked at about 300 MB in the parent (package V). A read that must infer a few projects still verifies the tree for 2 to 4 minutes; that is slow, not a failure.
- Not fixed: a cold build (thousands of stale projects) still puts 12 to 13 GB in the parent. The hard stop in step 4 stays, with the limit given there.

## Status written by package U, 2026-11-14 09:05 (history)

Can a worker run the native bootstrap to the end now: NO.
- The graph step has no fleet generation to restore. Since package S every machine publishes its generation head under its own origin key, and no machine has completed a graph build since; the earlier head (key `7240a80f…`) is no longer read, so the restore answers silently "absent" (measured 09:00 after `repo reset`: `graph: no usable generation (invalidated), building the first one`, no `restored` line, 4775 projects to infer).
- On this machine (32 GB, 4 731 of 5 100 projects stale because the build system's own scripts changed all night) one refresh is 97 waves of 48 projects, each 1 to 3 minutes (about 2 to 3 hours), and the parent process holds 7.1 GB physical footprint in the first wave and 15.8 GB after ten waves (measured; peak resident 12 to 14 GB). Package R's rule books 4.1 to 6.5 GB on a 16 GB machine. A 16 GB worker would swap or be killed in the first wave.
- Therefore: give the prompt below to a worker only as a bounded probe (hard stop in step 4), not as a bootstrap that is expected to finish. The bootstrap can finish only after the group server (or any machine with 32 GB) has completed one `repo show projects` and published its generation (look for `uploaded to the remote cache` in its log), and after the parent's footprint in a wave is fixed (package U report: the farm workers are starved by the 22 GB rebuild reservation, so the parent infers in-process).

Numbers a probe log should show on the new code: after a restart of an interrupted refresh no line `No verified change journal cursor is recorded` (package U's landing keeps the cursor of every published wave, `~/Library/Caches/semio/repo/<hash>/journal/frontier-cursors.json` exists); first project inferred 150 to 280 s after the start; the verification pass appears three times (shared read, prefetch or restore, builder) at 40 to 90 s each.


Give the text between the two lines to an agent that runs on the worker (a 16 GB Mac mini of the fleet) once the replica has delivered package R's code (about one minute after it landed; the daemon restarts itself when its code digest changes). Written 2026-11-14 03:25 by package R; the numbers below are the ones a worker log must show.

---

You work on a fleet worker (account `ueli`, worktree `/Users/ueli/Documents/semio`, replicated live from the group server; never edit files there, never run a git command that changes anything, never kill a process you did not start, no sudo). Your job is to run the native bootstrap once more and report. The earlier runs failed because the project-graph step waited for ever for a fixed 6627 to 8192 MB memory reservation that a 16 GB machine never has. That is fixed: the step now asks for at most 40 percent of the physical memory (6553 MB on 16 GB), starts with the memory that is free once 2048 MB are free above the machine's reserve, and refuses with a message after 120 seconds when nothing else of ours is running and not even 2048 MB are free. The graph itself should come from the fleet (per-project records in the fleet cache), not from your CPU.

1. Check that the new code arrived (all four lines must print a number of at least 1; if one prints 0, wait two minutes and check again, at most three times, then report "code not arrived" with the output):
   ```
   cd /Users/ueli/Documents/semio
   B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
   grep -c "sliceBookingV1" "$B/🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs"
   grep -c "memoryStarved" "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json"
   grep -c "shrinkable" "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts"
   grep -c "host-slice-owners-reserving" "$B/🏡️resident/🧬️schema/🔣️.json"
   ```
   Package S added four more checks for the graph step (all four must print at least 1 as well; the first two make a graph read O(changed) instead of two full passes, the third bounds the memory of the restore of the fleet generation, the fourth stops a 7 GB spike in every pass that finds stale rows):
   ```
   grep -c "manifestJournalPortV1" "$B/🪪️graph-state/📍️cursor/🟦️.ts"
   grep -c "transcodeFactIndexV1" "$B/🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs"
   grep -c "snapshotFor" "$B/🕸️graph/🕰️generations/🌐️remote-exchange/🟦️.ts"
   grep -c "rowOwnerNamesV1" "$B/🕸️graph/🕰️generations/🟨️.mjs"
   ```
   Package U added three more checks (each at least 1):
   ```
   grep -c "maximumCarriedCursors" "$B/🏡️resident/🕰️graph-hooks/🧬️schema/🔣️.json"
   grep -c "published(generation)" "$B/🏡️resident/🕰️graph-hooks/🔌️host/🟨️.mjs"
   grep -c "request.published" "$B/🕸️graph/🕰️generations/🟨️.mjs"
   ```
   Then check the daemon runs the new code: `repo ./📜️script.ts status` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica` (the replica status command; `--lang en`). It must say the daemon is running with no lag; write down its code digest line. A restart in the last two minutes is normal (new code).

2. Look at the machine before you start and write it down: `vm_stat | head -4` (page size 16384 bytes), `sysctl hw.memsize`, `uptime`, `ps -axo rss,command | sort -nr | head -8`. Quit applications of the user that you may close (browsers, editors) only if the user's instructions allow it; otherwise only report. A resident graph service that grew above 1 GB (`ps` shows a `repo service host run` process) may be stopped with `repo service host stop` (it starts again by itself).

3. Run the bootstrap exactly once, in the foreground, and keep the whole log:
   ```
   cd /Users/ueli/Documents/semio
   bash ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FINISH-WORKER-SETUP-AND-MERGE-PRE-FLEET-FOLDER-ON-MAC-MINI-02/native-bootstrap.sh" "$HOME/native-bootstrap-retry.log"
   ```
   Do not set `SEMIO_GRAPH_MEMORY_MB` (it does not change the reservation). Do not start any other `repo` graph command beside it (`repo graph`, `repo show projects`, `repo run`): they queue behind each other. The script prints `native_rc=<code>` as its last line and writes `.cache/f/native-bootstrap.done` only after exit 0; the script's own exit status is not the bootstrap's: read the last line.

4. What you should see, in order:
   - `Installing the dependency scopes`: the install pack is restored from the fleet in about 12 seconds per scope (not 30 minutes). If it instead downloads or installs for minutes, report the lines.
   - `workspace:setup`, then lines `[capacity] admitted graph: ... MB reserved` or `[capacity] admitted graph-plan with N of M MB: only X MB are free above the Y MB floor right now, and Z MB are enough to start` (the second one is the normal case on a 16 GB machine; N is what the machine can give).
   - `Reading the project graph slice (N s): ...`. Either a line `graph: generation ... restored from the remote cache` or `[fleet] seeded=...` counters (the fleet supplied the graph or its projects: good, this is the expected fast path), or `waiting for memory: W MB wanted, M MB are enough to start, F MB are free above the R MB this machine keeps in reserve ... it stops after L s`.
   - Expected: a line `restored from the remote cache, N documents (… MB) downloaded in … s`, then `graph: refreshing N projects` with a small N (the projects changed since the fleet's generation) and a few minutes of work.
   - HARD STOP: when `graph: refreshing N projects` shows N above 300, or `Inferring projects: … of M projects` shows M above 300, or after 12 minutes without a `restored` line, send ONE SIGINT (`Ctrl-C`, wait a minute) and report. Do not let a 16 GB machine infer thousands of projects: the parent holds 7 GB in the first wave (measured), 15 GB after ten. Also stop when `ps -axo rss,command | sort -nr | head -3` shows a process above 6 GB resident or the machine reports memory pressure. A line `graph: refreshing N projects (generation G)` tells how far behind the machine is (N near 4 700: nothing usable came from the fleet).

5. Exit codes and messages:
   - `native_rc=0` and the marker file: success. Run `repo capacity status` and `ps -axo rss,command | sort -nr | head -8` once more for the report.
   - `native_rc=130`: interrupted (by you or the user).
   - A capacity refusal (the log contains `needs at least` / `braucht mindestens`) is the designed answer, not a crash: `... It cannot run here` / `Es kann hier nicht laufen` (the machine is too small for that step: report total memory and the numbers) or `... after 120 s only N MB are free ... and nothing of ours is running` / `... sind nur N MB ... frei, und nichts von uns läuft` (free memory was below the minimum for two minutes: report what held the memory, `ps -axo rss,command | sort -nr | head -12`, close what the user allows, run step 3 once more at most once).
   - `stale build.closure.source-differs` (another machine changed a build source while the run was in flight): run step 3 again once; this is not a memory problem.
   - Any other non-zero `native_rc`: report the first error line and the 40 lines before it.

6. Report (in a file `native-bootstrap-retry-report.md` next to `native-bootstrap.sh` in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FINISH-WORKER-SETUP-AND-MERGE-PRE-FLEET-FOLDER-ON-MAC-MINI-02/`, and four lines in the chat): the code digest from step 1, memory before (free MB above the floor, total), `native_rc`, wall time per step (dependency scopes, graph step, whole run), the `[capacity]` lines of the graph step verbatim, any `restored`/`seeded`/`computed` counters verbatim, peak resident size of the largest process seen (sample `ps -axo rss,command | sort -nr | head -3` every 30 seconds during the graph step), and the last 40 lines of the log. Write numbers as measured; say "not measured" where you did not.

---

## What the owner reads in the answer

- `native_rc=0`, marker present: the worker is set up. The report's graph-step time tells whether the fleet supplied the graph (`seeded`/`restored`, minutes) or the machine computed it (`computed` large: the fleet cache has no records yet; the group server has to finish and publish one graph first, package M's path).
- `admitted graph-plan with N of M MB`: the shrink rule worked on the real machine; N should be about (free memory above the reserve).
- A refusal text: the rule worked; the numbers in it are the machine's.
- Still waiting after 3 minutes with `running: (none)` and no refusal: the code did not arrive or the rule failed on the real machine; ask for `repo capacity status` and the daemon status.
