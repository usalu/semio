# g1-m04 Round 2

Machine: `ueli-workstation-lin-1`, SteamOS (Linux 7.2.7-valve2-2-neptune-72-g31504582a073), 15848 MB memory, 25331 MB swap (10064 MB used before the run), 146 GiB free on `/home`. Runner: `run-tries.sh 2` (calls `run-round.sh`, sample every 30 s).

## Result

`native_rc=1`, not ready, one try only. **Hard stop** at 16:07:40Z: PSI memory `full avg10` 27.87 % (limit 20 %). One SIGINT to the process group of the run; the run had ended three seconds later (`end 2026-11-14T16:07:43Z`). No retry, because the try did not end with the fleet refusal: the log has **no line** about the fleet, `wanted`, `waiting for the fleet`, `restored` or `Restarting with the current implementation` (0 matches; the only `fleet` matches are help text of `repo`).

**The process that caused the pressure was not started by the run.** It is `repo show projects` of the build agent sidecar of the replica daemon (`graph-server: refreshing the fleet generation in the background`, logged 16:03:59Z), ancestry `2090280 <- 2090245 (📜️script.ts repo show projects) <- 1661222 (sidecar agent) <- 1658988 <- 1657737 (replica daemon)`. It grew from 0.75 GB (16:04:10Z) to 7571404 KiB resident (16:07:40Z) while `workspace:setup` of the run waited for memory for its own graph slice. So two graph commands ran on the machine at once, one of them outside the worker's control. It is not mine, so I did not stop it; state when this report was written: pid etime rss(KiB) cpu = `2090280       06:15 4300564 91.5`. The three sidecar attempts before it (13:17, 14:12, 15:07 UTC) each ended after 2701 s with `Internal error: Original Script process interrupted`.

The graph service the run started (`repo service host run`, pid 2090136/2090206, 225 MB, log `/run/user/1000/semio-repo/1000/host-e90c7512b77f.log`) is still running and answers `repo service host status`: "running (pid 2090206, 1 workspaces, 225.2 MB) … Plans: 1 asked, 0 from memory, 0 computed". I left it.

## Before the Run (step 1 of the round)

```
pre-run facts, taken 2026-11-14T16:00:09Z
replica code digest (.cache/f/health.json): "code":"39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759"
repo service host process (ps -eo pid,etime,rss,args | grep "service host"): none, so nothing was stopped
last 5 lines of .cache/f/sidecars/build-agent.log:
2026-11-14T14:02:49.892Z graph-server: the graph read ended with status 1 after 2702 s: Internal error: Original Script process interrupted
2026-11-14T14:12:51.973Z graph-server: refreshing the fleet generation in the background (5 s quiet, 602 s since the last)
2026-11-14T14:57:52.730Z graph-server: the graph read ended with status 1 after 2701 s: Internal error: Original Script process interrupted
2026-11-14T15:07:55.019Z graph-server: refreshing the fleet generation in the background (1 s quiet, 602 s since the last)
2026-11-14T15:52:55.620Z graph-server: the graph read ended with status 1 after 2701 s: Internal error: Original Script process interrupted
memory: Mem:           15848        5718        2463         318        8322       10130
```

The stale service of round 1 (pid 569455) no longer existed; nothing was stopped.

## Times

- start 2026-11-14T16:00:09Z, end 2026-11-14T16:07:43Z (454 s).
- Wall time per phase: not measured (the log carries no timestamps). Tools and dependencies were through before 16:03:50Z (start time of the graph service); `workspace:setup` then waited at least 128 s for memory ("Reading the project graph slice (128 s)" is the last line).
- Peak resident size seen: 7571404 KiB (the sidecar's `repo show projects`, see above). Largest process of the run itself: 4680992 KiB at 16:07:10Z is the same sidecar process; the run's own processes stayed below 0.6 GB in the samples. Samples: `round-2.samples.txt`.
- PSI `full avg10` per sample from 16:05:10Z: 0.53, 0.76, 1.06, 0.05, 0.14, 27.87. MemAvailable fell from 8990 MB (16:02:40Z) to 2951 MB (16:07:40Z).

## Failed Task

`workspace:setup`: no error line; it was interrupted by the hard stop while waiting. Last distinct lines before the end:

```
[repository:command] still running after 190 s
[repo] still working after 200 s; last phase started: source.served
[DATA] Store dependency concurrency {"version":1,"purpose":"dependency-concurrency-event","event":"kept","count":5}
Kept 5 superseded dependency generation(s) that are pinned, recent or the rollback target.
[script] {"owner":"repository:build","command":"repo","stage":"complete"}
[semio] workspace:setup…
[capacity] run: sized at 46% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (9992 MB available, 1981 MB floor)
[script] {"owner":"repository:build","command":"repo","stage":"loading"}
[script] {"owner":"repository:build","command":"repo","stage":"running"}
Starting the repo graph service for this host.
Waiting for the repo graph service (25 ms).
[repo] still working after 5 s; last phase started: door.script
Loading the project graph of d09b06e73429.
Planning 1 targets.
Reading the project graph slice (2 s): reservation 0 of 3239; running (name, kind, seconds, rows): ; waiting for memory: 3239 MB are asked for (2048 MB are enough to start), 4386 MB are free above the 1981 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and not even 2048 MB are free.
```

The wait line in full (last one):

```
Reading the project graph slice (128 s): reservation 0 of 3239; running (name, kind, seconds, rows): ; waiting for memory: 3239 MB are asked for (2048 MB are enough to start), 859 MB are free above the 1981 MB floor of this machine; the request is refused after 120 s if nothing of ours runs and not even 2048 MB are free.
```

## `[capacity]` and `graph:` Lines of the Run (verbatim, all)

```
[capacity] run: sized at 5% of the default from 6 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] admitted run: 8 job(s), 7275 MB reserved (10028 MB available, 1981 MB floor)
[capacity] run: sized at 5% of the default from 6 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] run: sized at 46% of the default from 4 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] run: sized at 46% of the default from 4 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] dependencies: sized at 51% of the default from 4 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] waiting 0s for dependencies: swap use is above its ceiling (need 6933 MB; 7354 MB spare above the 1981 MB floor, load 3.56; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 5s for dependencies: swap use is above its ceiling (need 6933 MB; 7767 MB spare above the 1981 MB floor, load 3.51; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted dependencies: 1 job(s), 6933 MB reserved (9753 MB available, 1981 MB floor)
[capacity] dependencies: sized at 51% of the default from 4 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] run: sized at 46% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (9992 MB available, 1981 MB floor)
```

## `[capacity]` Lines of the Graph Service (`host-e90c7512b77f.log`)

```
[capacity] waiting 66s for graph-plan: not enough free memory (need 3239 MB; -7233 MB spare above the 1981 MB floor, load 4.78; 2 running, 0 ahead); behind projects (running, 1 job(s), 13867 MB), workspace:setup (quiet, 1 job(s), 2048 MB)
[capacity] waiting 71s for graph-plan: swap use is above its ceiling (need 3239 MB; -7083 MB spare above the 1981 MB floor, load 4.56; 2 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 76s for graph-plan: not enough free memory (need 3239 MB; -7891 MB spare above the 1981 MB floor, load 4.51; 2 running, 0 ahead); behind projects (running, 1 job(s), 13867 MB), workspace:setup (quiet, 1 job(s), 2048 MB)
[capacity] waiting 81s for graph-plan: not enough free memory (need 3239 MB; -8140 MB spare above the 1981 MB floor, load 4.39; 2 running, 0 ahead); behind projects (running, 1 job(s), 13867 MB), workspace:setup (quiet, 1 job(s), 2048 MB)
[capacity] waiting 86s for graph-plan: swap use is above its ceiling (need 3239 MB; -7943 MB spare above the 1981 MB floor, load 4.36; 2 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 91s for graph-plan: swap use is above its ceiling (need 3239 MB; -8328 MB spare above the 1981 MB floor, load 4.14; 3 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 97s for graph-plan: not enough free memory (need 3239 MB; -8914 MB spare above the 1981 MB floor, load 4.13; 3 running, 0 ahead); behind projects (running, 1 job(s), 13867 MB), graph-farm (running, 4 job(s), 6144 MB), workspace:setup (quiet, 1 job(s), 2048 MB)
[capacity] waiting 102s for graph-plan: not enough free memory (need 3239 MB; -7448 MB spare above the 1981 MB floor, load 4.12; 3 running, 0 ahead); behind projects (running, 1 job(s), 13867 MB), graph-farm (running, 4 job(s), 6144 MB), workspace:setup (quiet, 1 job(s), 2048 MB)
[capacity] waiting 108s for graph-plan: swap use is above its ceiling (need 3239 MB; -9436 MB spare above the 1981 MB floor, load 4.11; 3 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 113s for graph-plan: swap use is above its ceiling (need 3239 MB; -9312 MB spare above the 1981 MB floor, load 4.26; 3 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 119s for graph-plan: swap use is above its ceiling (need 3239 MB; -6835 MB spare above the 1981 MB floor, load 5.37; 3 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] waiting 125s for graph-plan: swap use is above its ceiling (need 3239 MB; -6915 MB spare above the 1981 MB floor, load 5.82; 3 running, 0 ahead); behind the host itself (nothing of ours)
```

Also in that log: `[lease-order] Lock order violated: nx-tooling-provision (tool-singleton, rank 2) is requested while native-fixed-image:/home/deck/semio/.cache/b/t (image, rank 7) is held exclusive.`

## What This Means for the Coordinator

1. On a 16 GB worker the background fleet-generation refresh of the build agent (`repo show projects`, booked as `projects (running, 1 job(s), 13867 MB)`) and the bootstrap cannot run together. The graph service saw it: `waiting … for graph-plan: not enough free memory (need 3239 MB; -7879 MB spare …); behind projects (running, 1 job(s), 13867 MB)`. The worker cannot avoid it under the protocol (no stop of foreign processes, the sidecar restarts it every ten minutes after a failure).
2. The path "ask the fleet instead of inferring" was not reached in this try, neither by the run nor (as far as its log shows) by the sidecar: its read infers locally to more than 7 GB.
3. Swap: 10 GB were in use before the run (`swap use is above its ceiling` delayed `dependencies` and `graph-plan`).

Files: `round-2.pre.txt`, `round-2.samples.txt`, `round-2.monitor.txt`, `round-2.tries.txt`, `round-2.tail.txt`, `state.json`; full log `~/native-bootstrap-round-2.log` on the machine.
