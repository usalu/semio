# g1-m03 Round 2

- Machine: Mac mini M4, 16 GB memory, macOS 27.2.1 (26B131), free disk 100 GiB of 228 GiB before the run. Worktree `/Users/ueli/Documents/semio`.
- Result: three tries, all `native_rc=1`, marker absent, `ready` false. Tries 1 and 2 ended with the fleet refusal (0 of 703 projects arrived). Try 3 was stopped with one SIGINT after 4 min 57 s because `ROUND.md` changed to round 3 ("If your round 2 is still running: stop it with one SIGINT"). No hard stop in any try. `workspace:setup` never got past the graph step, so there are no nx task failures to report.

## 1. Before the run (2026-11-14T16:00:16Z)

    Pre-run facts, 2026-11-14T16:00:16Z
    - replica code digest (.cache/f/health.json): 39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759, store ok
    - `ps -axo pid,etime,rss,args | grep "service host"`: no process (nothing to stop; `repo service host stop` was not run)
    - last 5 lines of .cache/f/sidecars/build-agent.log (file last written 2026-11-14 12:41 UTC):
    2026-11-14T12:40:56.618Z worker finished: 4a7a2173-282d-42b5-b885-7be5638be7ad.2 failed 9
    2026-11-14T12:41:03.525Z worker started: 539c4618-9ea9-4f67-8d94-5379112a68da.1 78efb837fdf1
    2026-11-14T12:41:05.282Z worker finished: 539c4618-9ea9-4f67-8d94-5379112a68da.1 completed 0
    2026-11-14T12:41:23.232Z worker started: 141f9cfe-afdd-439c-b34d-1170276d4bab.1 6fd9d8c2f08c
    2026-11-14T12:41:24.846Z worker finished: 141f9cfe-afdd-439c-b34d-1170276d4bab.1 completed 0
    - memory pressure level 1, free disk 100 GiB of 228 GiB

## 2. and 3. Tries

| Try | Log | Start | End | `native_rc` | Seconds waited for the fleet | Wanted | Arrived | Missing | Local |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `~/native-bootstrap-round-2.log` | 16:00:38Z | 16:21:12Z | 1 | 902 | 703 | 0 | 703 | 192 |
| 2 | `~/native-bootstrap-round-2-try-2.log` | 16:26:12Z | 16:46:05Z | 1 | 901 | 703 | 0 | 703 | 192 |
| 3 | `~/native-bootstrap-round-2-try-3.log` | 16:51:17Z | 16:56:14Z | 1 (SIGINT, round 3 posted) | not measured (stopped before the graph wait ended) | not measured | not measured | not measured | not measured |

Wait between the tries: 300 s after the end of try 1 (16:21:12Z to 16:26:12Z) and 300 s after try 2 (runner timestamps 16:46:17Z to 16:51:17Z).

**The refusal is not in the bootstrap log.** The log of tries 1 and 2 ends with (try 1 verbatim):

```
Reading the project graph slice (1037 s): flush 1 of 1.
The resident service met an unexpected failure (reference 20faee18); try again, the details under that reference are in the service log /var/folders/wg/5s85qzsj2pg6689nck2j3xbm0000gn/T/semio-repo/501/host-38cdf415c7fb.log.
end 2026-11-14T16:21:12Z
native_rc=1
```

and contains neither `graph-wanted` nor a fleet line. The numbers are in the log of the resident graph service (`$TMPDIR/semio-repo/501/host-38cdf415c7fb.log`). Because of that the first runner of this worker did not recognise try 1 as a refusal; it was recognised by reading the service log, and try 2 started exactly 300 s after try 1 ended.

## 5. Lines that mention the fleet, `wanted`, `waiting for the fleet`, `restored`, `Restarting with the current implementation`

In the three bootstrap logs: none (apart from the `repo` help text that the launcher build prints, e.g. `fleet status …`). No `restored` and no `Restarting with the current implementation` line in any log.

Service log, try 1 (first occurrence of each line, with its line number):

```
141: graph: No verified change journal cursor is recorded for generation 44; verifying the tree once.
929: graph: 703 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 703
930: graph: waiting for the fleet: 0 of 703 projects arrived
1141: graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 902 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
1142: 2026-11-14T16:21:12.521Z request-failed 20faee18 RepoError: timeout graph-wanted.expired {"wanted":703,"arrived":0,"seconds":902,"missing":703,"local":192}
```

Service log, try 2:

```
graph: No verified change journal cursor is recorded for generation 44; verifying the tree once.
graph: the replica manifest cannot vouch for the tree (the replica daemon is not healthy); verifying the tree once.
graph: 703 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 703
graph: waiting for the fleet: 0 of 703 projects arrived
graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 901 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T16:46:05.424Z request-failed 80ba70d1 RepoError: timeout graph-wanted.expired {"wanted":703,"arrived":0,"seconds":901,"missing":703,"local":192}
```

Try 3: the graph service wrote a new log (`host-f74179f31e9f.log`); until the SIGINT it holds only:

```
graph: No verified change journal cursor is recorded for generation 44; verifying the tree once.
```

Note for try 2: `graph: the replica manifest cannot vouch for the tree (the replica daemon is not healthy)`. The replica daemon restarted at about 16:29Z (pid 96303); `.cache/f/health.json` at 16:46Z: store `ok`, failures `loop-blocked-restart` count 7, `git-broken-config-unreadable` count 1, `unstable` count 1.

## `[capacity]` and `graph:` lines of the bootstrap logs (verbatim; no `graph:` line in them)

Try 1:

```
[capacity] run: sized at 5% of the default from 11 earlier run(s) of its class, at least 4582 MB because of its assembly phase
[capacity] admitted run with 3 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 12% of the default from 11 earlier run(s) of its class, at least 4582 MB because of its assembly phase
[capacity] run: sized at 100% of the default from 9 earlier run(s) of its class
[capacity] run: sized at 100% of the default from 9 earlier run(s) of its class
[capacity] dependencies: sized at 85% of the default from 4 earlier run(s) of its class, at least 5877 MB because of its assembly phase
[capacity] admitted dependencies with 4556 of 5877 MB: only 4556 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 85% of the default from 4 earlier run(s) of its class, at least 5877 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (8497 MB available, 2048 MB floor)
```

Try 2:

```
[capacity] run: sized at 6% of the default from 12 earlier run(s) of its class, at least 4578 MB because of its assembly phase
[capacity] admitted run with 3 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 14% of the default from 12 earlier run(s) of its class, at least 4578 MB because of its assembly phase
[capacity] run: sized at 100% of the default from 9 earlier run(s) of its class
[capacity] run: sized at 100% of the default from 9 earlier run(s) of its class
[capacity] dependencies: sized at 85% of the default from 5 earlier run(s) of its class, at least 5878 MB because of its assembly phase
[capacity] admitted dependencies with 5511 of 5878 MB: only 5511 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 85% of the default from 5 earlier run(s) of its class, at least 5878 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7809 MB available, 2048 MB floor)
```

Try 3:

```
[capacity] run: sized at 6% of the default from 12 earlier run(s) of its class, at least 4574 MB because of its assembly phase
[capacity] admitted run with 6 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 8% of the default from 12 earlier run(s) of its class, at least 4574 MB because of its assembly phase
[capacity] run: sized at 59% of the default from 9 earlier run(s) of its class
[capacity] run: sized at 59% of the default from 9 earlier run(s) of its class
[capacity] dependencies: sized at 85% of the default from 6 earlier run(s) of its class, at least 5879 MB because of its assembly phase
[capacity] admitted dependencies with 3319 of 5879 MB: only 3319 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 85% of the default from 6 earlier run(s) of its class, at least 5879 MB because of its assembly phase
[capacity] run: sized at 70% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6803 MB available, 2048 MB floor)
```

## Peak resident size (sampled every 30 s, `ps -axo rss,command | sort -nr | head -3`)

| Try | Samples | Pressure level other than 1 | Largest process |
|---|---|---|---|
| 1 | 42 | 0 | 4386880 KiB (4.18 GiB), `bun --eval …` |
| 2 | 40 | 0 | 4905936 KiB (4.68 GiB), `bun --eval …` |
| 3 | 10 | 0 | 2613984 KiB (2.49 GiB), `bun --eval …` |

A `git` process of 2.3 to 2.6 GB appears in the samples of all three tries (2604688 KiB, 2510624 KiB, 2371840 KiB at the largest). It is not part of the run. Its command begins `/Library/Developer/CommandLineTools/usr/bin/git -c core.quotepath=false -c safe.directory=* -c core.fsmonitor= -c branch.autoSetupMerge=false -c core.useReplaceRefs=false -c core.hooksPath=/dev/null …`; at 16:57Z one such process (pid 9149, 2202752 KiB, 10 s old) had parent 47709 = `/Applications/Claude.app/Contents/Frameworks/Claude Helper.app/Contents/MacOS/Claude Helper --type=utility --utility-sub-type=node.mojom.NodeService`, whose parent is 47677 = `/Applications/Claude.app/Contents/MacOS/Claude`, parent 1 (launchd). It is the git status watcher of the Claude desktop app in which this agent runs; it starts a new process every few seconds on the worktree.

## Wall time per phase

| Phase | Try 1 | Try 2 | Try 3 |
|---|---|---|---|
| whole run | 20 min 34 s | 19 min 53 s | 4 min 57 s |
| launcher, toolchain, native repo command, dependency scopes (until `[semio] workspace:setup…`) | not measured separately | not measured separately | not measured separately |
| `workspace:setup` until the service gave up (counter of `[repo] still working after N s`) | about 1040 s, of which 902 s waiting for the fleet | not measured from the bootstrap log; 901 s waiting for the fleet | stopped |

## Runner

`workers/g1-m03/run-round-2.sh` (try 1), `run-round-2b.sh` (tries 2 and 3, reads the service log for the refusal), `round-2.monitor.txt` (timestamps), `round-2.try-<k>.samples.txt`, `round-2.try-<k>.service-lines.txt`, `round-2.tail.txt` (last 200 lines of try 3), `round-2.try-1.tail.txt`, `round-2.try-2.tail.txt`.
