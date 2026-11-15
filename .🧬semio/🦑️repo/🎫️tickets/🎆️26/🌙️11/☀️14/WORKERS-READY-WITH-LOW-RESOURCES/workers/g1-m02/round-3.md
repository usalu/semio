# g1-m02 Round 3

round: 3

Result: not ready. Try 1 was refused by the fleet (0 of 703 arrived in 301 s, `native_rc=75`). In try 2 the fleet delivered 551 of 703 projects, then the graph service on this machine began `refreshing 703 projects (generation 18)` itself and drove the machine into memory pressure; I stopped the run with one SIGINT under the round's rule (level 2 in three samples in a row), `native_rc=7`. No nx task ran. 2 of 8 tries used: a hard stop ends the round.

## Machine

Mac mini `Mac16,10`, Apple M4, 10 cores, 16 GB (17179869184 bytes), macOS 27.2.1 (26B131). Free disk: not measured in this round (106 GiB of 228 GiB after round 2).

## Before the Run (2026-11-14T17:05:44Z)

Replica code digest (`.cache/f/health.json`): `39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759`

`repo show projects` and `repo service host` processes (pid, ppid, age, rss KiB; command shortened to its tail):
```
10135     1       11:21 268336 … repo service host run
(end of list)
```

Last 8 lines of `.cache/f/sidecars/build-agent.log`:
```
2026-11-14T16:47:10.893Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T16:47:11.083Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T16:47:11.088Z worker registered: 381e9525-39a1-4324-bb54-99588cbe76ba cache http://192.168.178.70:6094
2026-11-14T16:59:39.611Z agent code changed, restarting
2026-11-14T16:59:39.613Z worker stopped: ueli-workstation-mac-mini-m4-16gb-256gb-01-wlan.fritz.box-7533
2026-11-14T16:59:41.100Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T16:59:41.151Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T16:59:41.154Z worker registered: 5af064f4-7796-442b-82a0-a6353f32e957 cache http://192.168.178.70:6094
```

The `repo service host run` process (pid 10135, parent 1) was left by round 2 try 3; it is the graph service the bootstrap starts and it served both tries of this round. No `repo show projects` process ran.

## Tries

| Try | Log | Start (UTC) | End (UTC) | `native_rc` | Wanted | Arrived | Missing | Seconds | Largest process on the machine | Largest in the run's process group |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `~/native-bootstrap-round-3.log` | 17:06:01 | 17:15:28 | 75 | 703 | 0 | 703 | 301 | 3552528 KiB | 490256 KiB |
| 2 | `~/native-bootstrap-round-3-try-2.log` | 17:18:33 | 17:43:38 | 7 (my SIGINT) | 703 | 551 (last line seen) | 152 (derived) | about 1130 (slice counter 36 s to 1167 s; no closing line with seconds) | 4198144 KiB (pid 22848, child of the graph service 10149) | 358848 KiB |

Try 2 started 3 min 5 s after try 1 ended.

"arrived" lines, first and last, verbatim (all from the service log `host-f74179f31e9f.log`; the bootstrap logs contain no `arrived` line):

- Try 1: `graph: waiting for the fleet: 0 of 703 projects arrived` (only value, repeated), then
  `graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 301 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine`
- Try 2: first `graph: waiting for the fleet: 0 of 703 projects arrived`, last `graph: waiting for the fleet: 551 of 703 projects arrived` (108 such lines; values seen 0, 263, 311, 359, 407, 455, 503, 551, i.e. steps of 48).

## Try 2: From the Fleet Wait to a Local Refresh of All 703

Bootstrap log, distinct lines of `workspace:setup` (counters differ, repeats left out):

```
[semio] workspace:setup…
Loading the project graph of 440cffa5563d.
Planning 1 targets.
Reading the project graph slice (36 s): flush 1 of 1.
Reading the project graph slice (1167 s): files 0 of 0.
Reading the project graph slice (1205 s): owners 0 of 703; running (name, kind, seconds, rows): @semio-tech/energy-oracle-py (project, 1, 0), @semio-tech/async-timer-rs (language:rust, 1, 0), @semio-tech/async-macros-rs (language:cargo, 1, 0).
Reading the project graph slice (1245 s): owners 33 of 703; running (name, kind, seconds, rows): @semio-tech/canvas-react-renderer (project, 1, 0).
Reading the project graph slice (1261 s): owners 34 of 703; running (name, kind, seconds, rows): @semio-tech/dsl-syntax-rs (language:rust, 9, 0); waiting for memory headroom (13 of -60 MB reserved).
Reading the project graph slice (1275 s): owners 34 of 703; running (name, kind, seconds, rows): @semio-tech/dsl-syntax-rs (language:rust, 23, 0); waiting for memory headroom (168 of -603 MB reserved).
Reading the project graph slice (1311 s): owners 34 of 703; running (name, kind, seconds, rows): @semio-tech/dsl-syntax-rs (language:rust, 60, 0); waiting for memory headroom (207 of -587 MB reserved).
end 2026-11-14T17:43:38Z
native_rc=7
```

Service log, same time, verbatim (the 16 `… is stale: the workspace file … was added` lines between the second and third line left out):

```
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (6968 MB available, 2048 MB floor)
graph: waiting for the fleet: 551 of 703 projects arrived
graph: No verified change journal cursor is recorded for generation 17; verifying the tree once.
graph: refreshing 703 projects (generation 18)
graph: 527 stale facts (file 224, analysis 115, readdir 88, field 77, digest 20, readdirNames 2, exists 1) leave 703 projects stale
graph: 527 more stale inputs are not listed
graph: asking the host for up to 8 worker processes for 48 projects; projects are inferred in this process meanwhile
[capacity] admitted exec with 2 of 8 job(s): the host cannot carry more right now
graph: inferring projects with 2 worker processes (1536 MB each)
[capacity] waiting 0s for exec: not enough free memory (need 1536 MB; 787 MB spare above the 2048 MB floor, load 5.07; 3 running, 0 ahead); behind graph-farm (running, 2 job(s), 3072 MB), graph-plan (running, 1 job(s), 1887 MB), workspace:setup (quiet, 1 job(s), 45 MB)
[capacity] waiting 16s for exec: not enough free memory (need 1536 MB; -641 MB spare above the 2048 MB floor, load 4.61; 3 running, 0 ahead); behind graph-farm (running, 2 job(s), 3072 MB), graph-plan (running, 1 job(s), 2002 MB), workspace:setup (quiet, 1 job(s), 45 MB)
graph: waiting for memory headroom (100 of 100 MB reserved, 7 projects running)
[capacity] waiting 28s for exec: not enough free memory (need 1536 MB; -3516 MB spare above the 2048 MB floor, load 4.68; 3 running, 0 ahead); behind graph-plan (running, 1 job(s), 10792 MB), graph-farm (running, 2 job(s), 3072 MB), workspace:setup (quiet, 1 job(s), 46 MB)
graph: waiting for memory headroom (5 of -67 MB reserved, 1 projects running)
graph: headroom of -72 MB is below one project (5 MB), inferring one project at a time
graph: headroom of 20 MB again (5 MB taken), resuming
graph: waiting for memory headroom (10 of 20 MB reserved, 2 projects running)
graph: headroom of 20 MB is below one project (69 MB), inferring one project at a time
[capacity] waiting 40s for exec: the host is swapping out too fast (need 1536 MB; -3506 MB spare above the 2048 MB floor, load 4.51; 3 running, 0 ahead); behind the host itself (nothing of ours)
graph: waiting for memory headroom (13 of -60 MB reserved, 1 projects running)
[capacity] waiting 55s for exec: memory pressure is critical (need 1536 MB; -13396 MB spare above the 2048 MB floor, load 5.14; 3 running, 0 ahead); behind the host itself (nothing of ours)
graph: waiting for memory headroom (207 of -603 MB reserved, 1 projects running)
```

Observations, as measured:

1. After 551 of 703 had arrived the service printed `refreshing 703 projects (generation 18)` and `leave 703 projects stale`, not 152. Whether the 551 arrived records were used is not visible in either log; no line says "taken from the fleet" with a number above 0 and no line with `restored`, `adopted`, `serving` or `Restarting` exists in the logs of this round.
2. There is no refusal and no closing line of the fleet wait in try 2: the wait went straight into the local refresh. The `Inferring projects: … of 48 projects` lines (service log only) come after the fleet lines, so by the round's rule they are not a stop.
3. Memory: the capacity ledger showed `graph-plan (running, 1 job(s), 10792 MB)` against its 2048 MB reservation 28 s after the workers started; my 30 s samples saw at most 4198144 KiB resident for the largest process (pid 22848). Pressure level was 1 in 67 samples and 2 in the last three (17:42:37Z, 17:43:08Z, 17:43:38Z); SIGINT to the run's process group at 17:43:38Z, the run ended at once. Level 1 again when checked at 17:45Z. The graph service (pids 10135, 10149) survived the SIGINT (it is not in the run's group) and is idle at 54 and 96 MB; I did not stop it.
4. Wall time per phase, try 2: before `workspace:setup` about 190 s (derived); graph slice 36 s until the fleet wait, about 1130 s fleet wait, 146 s local inference (1167 s to 1313 s) until the stop. No task phase.

## The Large `git` Process and Its Parent

43 distinct `git` processes above 500 MB resident were seen in the 30 s samples of this round (16 in try 1, 27 in try 2), between 568976 and 2576560 KiB each, a new pid nearly every sample. All have the same command and parent chain:

```
git … diff --ignore-submodules=dirty --submodule=short --no-textconv --numstat -z -M 277881dca4ea640e85beffcc5527b3d7e3feae94
  parent 35208  /Applications/Claude.app/Contents/Frameworks/Claude Helper.app/Contents/MacOS/Claude Helper --type=utility --utility-sub-type=node.mojom.NodeService …
  parent 35172  /Applications/Claude.app/Contents/MacOS/Claude
  parent 1      /sbin/launchd
```

So on this machine the 2 GB `git` is the Claude desktop app itself (the app both agent sessions here run in) recomputing the diff of the worktree against a fixed commit about twice a minute. It is not started by the replica, the build or an agent command. It takes up to 2.5 GB of the 16 GB during every run. I cannot switch it off from here; it is not a process I started and I did not touch it.

## `[capacity]` Lines of the Bootstrap Logs, Verbatim

Try 1:
```
[capacity] run: sized at 5% of the default from 10 earlier run(s) of its class, at least 2303 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 2968 MB reserved (6897 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 49% of the default from 9 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 3020 MB reserved (6864 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7054 MB available, 2048 MB floor)
```
Try 2:
```
[capacity] run: sized at 5% of the default from 11 earlier run(s) of its class, at least 2305 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 2970 MB reserved (6916 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 49% of the default from 10 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 3020 MB reserved (7854 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] waiting 0s for run: not enough free memory (need 2048 MB; 1722 MB spare above the 2048 MB floor, load 6.15; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted run: 1 job(s), 2048 MB reserved (6968 MB available, 2048 MB floor)
```

The refusal of try 1 in the bootstrap log, verbatim:
```
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 703 and delivered 0 within 301 s, 703 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

## Files

`round-3.tail.txt` (last 200 lines of try 2), `round-3.samples.txt` (three largest processes with pid and parent pid every 30 s, commands cut at 700 characters, and every parent chain of a large `git`), `run-round-3.py`.
