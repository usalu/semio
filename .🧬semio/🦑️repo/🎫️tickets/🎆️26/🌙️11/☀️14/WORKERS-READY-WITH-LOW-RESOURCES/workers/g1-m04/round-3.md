# g1-m04 Round 3

Machine: `ueli-workstation-lin-1`, SteamOS (Linux 7.2.7-valve2-2-neptune-72-g31504582a073), 15848 MB memory, 25331 MB swap (about 9.9 GB in use before the run), 146 GiB free on `/home`. Runner: `run-tries.sh 3 8 180`.

## Result

`native_rc=75` in all 8 tries, not ready. Every try ended with the designed refusal and the same numbers: **wanted 516, arrived 0, missing 516**, after 300 to 302 s. No "arrived" line exists in any log (0 matches), so there is no first or last one to quote. No hard stop; no memory problem (largest process of all tries 1923348 KiB, PSI memory `full avg10` maximum 0.80 %). No `git` process above 1 GB appeared (no `chain` file was written). The background graph build of the sidecar did not come back.

| Try | Start (UTC) | End (UTC) | Seconds | Wanted | Arrived | Missing | Waited for fleet | Peak RSS (KiB) |
|---|---|---|---|---|---|---|---|---|
| 1 | 16:51:44Z | 17:01:53Z | 609 | 516 | 0 | 516 | within 301 s | 1868592 |
| 2 | 17:05:15Z | 17:14:43Z | 568 | 516 | 0 | 516 | within 301 s | 1515056 |
| 3 | 17:17:46Z | 17:26:24Z | 518 | 516 | 0 | 516 | within 301 s | 1771804 |
| 4 | 17:29:47Z | 17:38:25Z | 518 | 516 | 0 | 516 | within 301 s | 1765180 |
| 5 | 17:41:48Z | 17:50:41Z | 533 | 516 | 0 | 516 | within 300 s | 1600648 |
| 6 | 17:53:50Z | 18:03:56Z | 606 | 516 | 0 | 516 | within 301 s | 1918848 |
| 7 | 18:07:21Z | 18:16:07Z | 526 | 516 | 0 | 516 | within 302 s | 1377852 |
| 8 | 18:19:22Z | 18:27:54Z | 512 | 516 | 0 | 516 | within 300 s | 1923348 |

Refusal, verbatim (try 1; the others differ only in the seconds):

```
The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 516 and delivered 0 within 301 s, 516 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

## Before the Run (step 1)

```
pre-run facts, taken 2026-11-14T16:51:37Z
replica code digest: "code":"39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759"
repo show projects / repo service host processes (pid ppid etime rss command):
(end of list)
last 8 lines of .cache/f/sidecars/build-agent.log:
2026-11-14T16:46:08.077Z joining http://192.168.178.70:8981 offering rustc for 4 target(s)
2026-11-14T16:46:08.118Z worker sandbox: linux-sandbox confined network denied
2026-11-14T16:46:08.151Z worker registered: 932cd337-4706-417b-ad31-c3625781c3e4 cache http://192.168.178.70:6094
2026-11-14T16:47:07.942Z agent code changed, restarting
2026-11-14T16:47:07.944Z worker stopped: ueli-workstation-lin-1-2170640
2026-11-14T16:47:08.620Z joining http://192.168.178.70:8981 offering rustc for 4 target(s)
2026-11-14T16:47:08.659Z worker sandbox: linux-sandbox confined network denied
2026-11-14T16:47:08.666Z worker registered: ea6f755d-94aa-48f8-ae2a-c453569f0877 cache http://192.168.178.70:6094
memory: Mem:           15848        5804        3979          50        6456       10043 | swap: Swap:          25331        9891       15440
psi: full avg10=0.00 avg60=0.00 avg300=0.00 total=21077643045
```

No `repo show projects` and no `repo service host` process existed, so there was nothing to wait for.

## Every Line About the Fleet, `wanted`, `arrived`, `serving`, `restored`, `Restarting`, `adopted` (try 1, help text of `repo` left out, repeated lines once)

```
[script] {"owner":"repository:build","command":"bun:dependency-storage","stage":"running","activity":{"resource":"javascript-preserving-source:323","mode":"exclusive","elapsedMs":397.7079169999997}}
The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 516 and delivered 0 within 301 s, 516 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

## Lines of `workspace:setup` Before the Refusal (try 1, repeated lines once, numbers folded)

```
[capacity] run: sized at 46% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (8877 MB available, 1981 MB floor)
Starting the repo graph service for this host.
Waiting for the repo graph service (23 ms).
[repo] still working after 5 s; last phase started: door.script
Loading the project graph of d09b06e73429.
Planning 1 targets.
Reading the project graph slice (2 s).
Reading the project graph slice (10 s): directories 710 of 710.
Reading the project graph slice (12 s): files 0 of 0.
Reading the project graph slice (14 s): doubt 450803 of 450803.
Reading the project graph slice (16 s): facts 40626 of 40626.
Reading the project graph slice (18 s): verification 0 of 1048.
[capacity] run: work has to run, waiting 0s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (4110 MB spare above the 1981 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 5616 MB)
[capacity] run: reservation grown from 2048 MB to 8190 MB after 14s (8 job(s), 8170 MB available, 1981 MB floor)
Reading the project graph slice (78 s): flush 1 of 1.
The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 516 and delivered 0 within 301 s, 516 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

## `[capacity]` and `graph:` Lines (verbatim, try 1 and what differs later)

```
[capacity] run: sized at 5% of the default from 7 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] admitted run: 8 job(s), 7275 MB reserved (9779 MB available, 1981 MB floor)
[capacity] run: sized at 5% of the default from 7 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] run: sized at 46% of the default from 4 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] run: sized at 46% of the default from 4 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] dependencies: sized at 51% of the default from 5 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] waiting 0s for dependencies: swap use is above its ceiling (need 6933 MB; 7962 MB spare above the 1981 MB floor, load 3.57; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted dependencies: 1 job(s), 6933 MB reserved (9916 MB available, 1981 MB floor)
[capacity] dependencies: sized at 51% of the default from 5 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] run: sized at 46% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (8877 MB available, 1981 MB floor)
[capacity] run: work has to run, waiting 0s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (4110 MB spare above the 1981 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 5616 MB)
[capacity] run: work has to run, waiting 5s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (4237 MB spare above the 1981 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 5616 MB)
[capacity] run: work has to run, waiting 11s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (4279 MB spare above the 1981 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 5616 MB)
[capacity] run: reservation grown from 2048 MB to 8190 MB after 14s (8 job(s), 8170 MB available, 1981 MB floor)
try 6:
[capacity] waiting 0s for run: swap use is above its ceiling (need 6932 MB; 7791 MB spare above the 1981 MB floor, load 2.19; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] dependencies: sized at 40% of the default from 10 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 6933 MB reserved (9771 MB available, 1981 MB floor)
```

## Times

Wall time per phase: not measured (no timestamps in the log). A try takes 512 to 632 s, of which 300 to 302 s are the wait for the fleet; tools and dependencies together therefore about 3.5 to 5.5 minutes.

## For the Coordinator

1. The worker side works here too: the machine decides it can infer 0 of the stale projects itself and asks for 516; memory stays calm.
2. In 97 minutes (16:51 to 18:28 UTC) and 8 requests the fleet delivered nothing to g1-m04, and each wait ended at the 5 minute silence limit, so the group server did not answer at all (not: answered slowly).
3. `ROUND.md` went to round 4 and then 5 while this round ran; I continue with the newest.

Files: `round-3.pre.txt`, `round-3.tries.txt`, `round-3[-try-k].samples.txt`, `round-3[-try-k].monitor.txt`, `round-3.tail.txt` (try 8), `state.json`; logs `~/native-bootstrap-round-3[-try-k].log`.
