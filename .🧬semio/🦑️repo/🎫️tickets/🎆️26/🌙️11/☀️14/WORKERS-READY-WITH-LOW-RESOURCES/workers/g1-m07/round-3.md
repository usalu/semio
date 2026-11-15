# Round 3 on g1-m07

## Machine
- Mac mini, 17179869184 bytes memory (16 GiB), macOS 27.2.1 (Darwin 27.2.0), 109 GiB free of 228 GiB on the data volume (measured 16:52 UTC).

## Before the run (2026-11-14T16:52:04Z)
- Round 2 was not running any more when round 3 was read; no SIGINT was needed.
- Replica code digest (`.cache/f/health.json`, written 16:52:03Z): `39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759`, `store: ok`, sidecar `build-agent` running (pid 99403), `build-scheduler` and `build-cache` off.
- `repo show projects` process: none.
- `repo service host` processes (pid, parent pid, age, resident KiB):
  - 88716, parent 1, 47:49, 88064 KiB: `bun …/🏗️build/🚪️door/📜️script.ts repo service host run` (started by try 1 of round 2 at 16:04Z)
  - 88717, parent 88716, 47:47, 482592 KiB: `bun --eval … service host run` (its loader child)
  - Left running: round 3 names no stop for it.
- Last 8 lines of `.cache/f/sidecars/build-agent.log`:

```
2026-11-14T16:46:12.688Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T16:46:12.767Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T16:46:12.771Z worker registered: c61af8b3-3d2f-4785-8b3c-1a81de7c8fdc cache http://192.168.178.70:6094
2026-11-14T16:47:12.295Z agent code changed, restarting
2026-11-14T16:47:12.305Z worker stopped: ueli-workstation-mac-mini-m4-16gb-256gb-02-lan.fritz.box-9418
2026-11-14T16:47:13.410Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T16:47:13.498Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T16:47:13.514Z worker registered: 92e0b012-bfba-40e9-938f-2209bce0a8e9 cache http://192.168.178.70:6094
```

- Memory pressure level before the run: 1.

## Tries
| try | log | start (UTC) | end (UTC) | native_rc | seconds waited | wanted | arrived | missing | peak RSS of one process | samples |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `~/native-bootstrap-round-3.log` | 16:52:30 | 17:05:09 | 75 | 302 | 703 | 0 | 703 | 3600144 KiB | 26, all level 1 |
| 2 | `…-round-3-try-2.log` | 17:08:36 | 17:15:14 | 75 | 301 | 703 | 0 | 703 | 3011408 KiB | 14, all level 1 |
| 3 | `…-round-3-try-3.log` | 17:18:38 | 17:44:40 | 75 | 1480 | 703 | 502 | 201 | 2998176 KiB | 52, all level 1 |
| 4 | `…-round-3-try-4.log` | 17:47:45 | 18:06:50 | 7 | not measured (no refusal) | 201 | 10 (last count seen) | 191 | 5996400 KiB | 39: 36 level 1, then 3 level 2 |

- Pause between tries: 180 s each (measured from the end of my runner, about 25 s after the `end` line of the log).
- Tries 5 to 8 were not run: try 4 ended with a hard stop, and when I read the result (18:07Z) `ROUND.md` said round 5.
- Not ready: marker `.cache/f/native-bootstrap.done` absent. No nx task ran in any try: `workspace:setup` never got past the graph step.

### Refusal text in the bootstrap log (new in this round, verbatim)
```
try 1: The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 703 and delivered 0 within 302 s, 703 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
try 2: The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 703 and delivered 0 within 301 s, 703 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
try 3: The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 703 and delivered 502 within 1480 s, 201 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```
The refusal and `native_rc=75` now reach the bootstrap log, as announced.

### Fleet lines of the graph service log (`$TMPDIR/semio-repo/501/host-f74179f31e9f.log`, service pid 15153 started 16:57:16Z by try 1), verbatim, repeat counts in brackets
Tries 1 and 2:
```
graph: 703 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 703
graph: waiting for the fleet: 0 of 703 projects arrived
graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 302 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T17:05:09.698Z graph-wanted-expired {"wanted":"703","arrived":"0","seconds":"302","missing":"703","local":"192"}
graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 301 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T17:15:14.264Z graph-wanted-expired {"wanted":"703","arrived":"0","seconds":"301","missing":"703","local":"192"}
```
Try 3 (first "arrived" line: `0 of 703`, last: `502 of 703`):
```
graph: 703 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 703
graph: waiting for the fleet: 0 of 703 projects arrived      [44]
graph: waiting for the fleet: 262 of 703 projects arrived    [9]
graph: waiting for the fleet: 310 of 703 projects arrived    [11]
graph: waiting for the fleet: 358 of 703 projects arrived    [8]
graph: waiting for the fleet: 406 of 703 projects arrived    [16]
graph: waiting for the fleet: 454 of 703 projects arrived    [21]
graph: waiting for the fleet: 502 of 703 projects arrived    [41]
graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 502 within 1480 s, 201 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T17:44:40.708Z graph-wanted-expired {"wanted":"703","arrived":"502","seconds":"1480","missing":"201","local":"192"}
```
Try 4 (first "arrived" line: `0 of 201`, last: `10 of 201`):
```
graph: No verified change journal cursor is recorded for generation 72; verifying the tree once.
graph: 703 stale projects, 502 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 201
graph: waiting for the fleet: 0 of 201 projects arrived      [59]
graph: waiting for the fleet: 10 of 201 projects arrived     [1]
graph: refreshing 703 projects (generation 73)
graph: 636 stale facts (analysis 251, file 249, field 75, readdir 46, digest 14, readdirNames 1) leave 703 projects stale
graph: asking the host for up to 8 worker processes for 48 projects; projects are inferred in this process meanwhile      [2]
graph: inferring projects with 1 worker processes (1536 MB each)      [8]
graph: worker processes were not used because they differ from this process (identity differs: records (file:/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs, …      [8]
graph: 0 of 48 projects inferred by 8 worker processes in 55 s (0 failed, 0 lost, 0 restarts)
graph: waiting for memory headroom (243 of 211 MB reserved, 7 projects running)
graph: waiting for memory headroom (243 of -396 MB reserved, 7 projects running)
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
```
- No line with `serving`, `restored`, `Restarting` or `adopted` in any bootstrap log of this round.

## Hard stop in try 4
- 2026-11-14T18:06:49Z: `kern.memorystatus_vm_pressure_level` was 2 in three samples in a row (18:05:49Z, 18:06:19Z, 18:06:49Z). One SIGINT to the process group of the run; one minute later the run was gone; `native_rc=7`.
- What preceded it: after 10 of the 201 missing projects had arrived, the service did not go on waiting and did not use the 502 it had taken: it started `graph: refreshing 703 projects (generation 73)` and inferred on this machine. The service's loader child (pid 15166) grew from 3619632 KiB (18:04:49Z) to 5996400 KiB (18:05:19Z). No process of the run was above 8 GB in any sample.
- `Inferring projects: … of 48 projects` only (M = 48, fleet lines preceded it).
- `[capacity]` lines of the service log in the last minute, verbatim:

```
[capacity] waiting 34s for exec: not enough free memory (need 1536 MB; -7688 MB spare above the 2048 MB floor, load 5.1; 2 running, 0 ahead); behind graph-plan (running, 1 job(s), 21975 MB), workspace:setup (quiet, 1 job(s), 49 MB)
[capacity] waiting 40s for exec: the host is swapping out too fast (need 1536 MB; -7688 MB spare above the 2048 MB floor, load 5.16; 2 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 46s for exec: the host is swapping out too fast (need 1536 MB; -9541 MB spare above the 2048 MB floor, load 4.99; 2 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 52s for exec: memory pressure is critical (need 1536 MB; -9541 MB spare above the 2048 MB floor, load 4.83; 2 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 59s for exec: the host is swapping out too fast (need 1536 MB; -11031 MB spare above the 2048 MB floor, load 4.6; 2 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 65s for exec: memory pressure is critical (need 1536 MB; -11031 MB spare above the 2048 MB floor, load 4.35; 2 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 71s for exec: memory pressure is critical (need 1536 MB; -11031 MB spare above the 2048 MB floor, load 5.36; 2 running, 0 ahead); behind the host itself (nothing of ours)
```
  A `graph-plan` job is booked there with 21975 MB on a 16 GiB machine.

## The large `git` process (step 5)
- A `git` process appears among the three largest in every try of this round (103 samples with a recorded chain), largest sample 2484832 KiB (about 2.37 GiB), 2324768 KiB right before the pressure samples of try 4.
- Command (verbatim): `/Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.hooksPath=/dev/null -c filter.lfs.clean= -c filter.lfs.smudge= -c filter.lfs.process= -c filter.lfs.required=false -c diff.ignoreSubmodules=dirty -c diff.submodule=short -c status.submoduleSummary=false -c submodule.recurse=false -c log.showSignature=false -c alias.remote-denied_by_claude_desktop.command= -c help.autocorrect=0 -c core.splitIndex=false ls-files --others --exclude-standard --full-name`
- Parent chain (verbatim, pids of try 4):

```
45003 44969 /Applications/Claude.app/Contents/Frameworks/Claude Helper.app/Contents/MacOS/Claude Helper --type=utility --utility-sub-type=node.mojom.NodeService …
44969     1 /Applications/Claude.app/Contents/MacOS/Claude
    1     0 /sbin/launchd
```
- So the parent is the Claude desktop app that hosts the worker agent: it lists untracked files of the worktree again and again (a new pid in every sample). It is not a process of the build and not one I started, so I left it alone. It takes up to 2.4 GiB of the 16 GiB while the run is going.

## Other observations
- The graph service of round 2 (pid 88716) was replaced at 16:57:16Z by a new one (pid 15153, log `host-f74179f31e9f.log`); it is still up after the round (1 h 11 min old at 18:08Z, 58928 KiB, child 15166 87856 KiB).
- Pressure level one minute after the stop: 1.
- Wall time per phase: not measured one by one; `workspace:setup` waited 302 s, 301 s, 1480 s for the fleet in tries 1 to 3.
- Files: `round-3.samples.txt`, `round-3-try-2.samples.txt`, `round-3-try-3.samples.txt`, `round-3-try-4.samples.txt`, the four `monitor.txt`, `round-3.tries.txt`, `round-3.tail.txt` (last 200 lines of try 4).
