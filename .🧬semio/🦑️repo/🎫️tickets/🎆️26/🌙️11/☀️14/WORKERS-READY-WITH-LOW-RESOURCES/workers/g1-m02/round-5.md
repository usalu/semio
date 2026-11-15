# g1-m02 Round 5

round: 5

Result: not ready after 10 tries (17:54Z to 21:01Z). No nx task ran in any try. The worker now takes fleet records ("N taken from the fleet" is above 0 from try 3 on), but every time it starts inferring locally it reaches memory pressure level 4 within minutes (tries 1, 3, 5, 7), and the fleet delivers less than is missing before its wait ends (tries 2, 8, 9, 10). Rounds 6 to 8 were published while this ran; I did not watch `ROUND.md` during the run and so did not interrupt it. From the next round on the runner checks for a newer round every 30 s.

## Machine

Mac mini `Mac16,10`, Apple M4, 10 cores, 16 GB (17179869184 bytes), macOS 27.2.1 (26B131), 105 GiB free of 228 GiB on the data volume after the round.

## Before the Run (2026-11-14T17:54:06Z)

Replica code digest (`.cache/f/health.json`): `39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759`

`repo show projects` and `repo service host` processes (pid, ppid, age, rss KiB; command shortened to its tail):
```
(end of list)
```

Last 8 lines of `.cache/f/sidecars/build-agent.log`:
```
2026-11-14T17:51:00.142Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T17:51:00.191Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T17:51:00.195Z worker registered: e0b7b758-a150-4511-8455-129c5ee78de3 cache http://192.168.178.70:6094
2026-11-14T17:53:29.757Z agent code changed, restarting
2026-11-14T17:53:29.759Z worker stopped: ueli-workstation-mac-mini-m4-16gb-256gb-01-wlan.fritz.box-35790
2026-11-14T17:53:31.058Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T17:53:31.106Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T17:53:31.110Z worker registered: 913619ba-be6c-4d3b-b10f-b6dcc93dcb2a cache http://192.168.178.70:6094
```

## Summary of the Tries

| Try | Start (UTC) | End (UTC) | `native_rc` | End reason | Stale | Taken from the fleet at start | Wanted | Arrived | Missing | Seconds | Largest process on the machine (KiB) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | 17:54:11 | 18:05:45 | 7 | hard stop, pressure level 4 | 703 | no line (closing line: 10 taken) | no line | no line | not measured | not measured | 6959920 |
| 2 | 18:09:14 | 18:39:57 | 75 | fleet refusal | 650 | 0 | 650 | 274 (550 seen earlier) | 376 | 1499 | 3144992 |
| 3 | 18:42:20 | 18:53:23 | 7 | hard stop, pressure level 4 | 650 | 274 | 376 | 192 (last line seen) | not measured | not measured | 3272336 |
| 4 | 18:56:53 | 19:00:20 | 1 | macOS denied the graph service access to Documents (`EPERM`) | - | - | - | - | - | - | 3684368 |
| 5 | 19:02:24 | 19:15:27 | 7 | hard stop, pressure level 4 | 650 | 376 | 274 | see section | not measured | not measured | 7703616 |
| 6 | 19:18:57 | 19:24:27 | 1 | `The resident service met an unexpected failure (reference dad6d735)` | 2 (`refreshing 2 projects (generation 61)`) | closing line: 2 taken | - | - | - | - | 3952240 |
| 7 | 19:26:29 | 19:35:32 | 7 | hard stop, pressure level 4 | 5 (`refreshing 5 projects (generation 61)`) | closing line: 1 taken, 3 computed here | - | - | - | - | 6196816 |
| 8 | 19:39:02 | 20:10:06 | 75 | fleet refusal | 657 | 234 | 423 | 96 | 327 | 1488 | 3654768 |
| 9 | 20:12:08 | 20:33:17 | 75 | fleet refusal | 658 | 95 | 563 | 56 | 507 | 910 | 3742448 |
| 10 | 20:35:42 | 21:01:02 | 75 | fleet refusal | 658 | 48 | 610 | 96 | 514 | 1141 | 3699040 |

Pressure samples of the whole round (30 s apart): level 1 in 322, level 2 in 3, level 4 in 4. Each of the four level 4 samples ended a try at once with one SIGINT; no try reached five samples of 2 in a row. The largest process of the run's own process group never exceeded 471216 KiB; the large processes are the graph service's children, which are not in the run's group.

## Findings, as Measured

1. Local inference is what hits the memory limit, in four of four tries that reached it. The capacity ledger printed the graph plan at `10002 MB` (try 1) and `17528 MB` (try 3) against 2048 MB reserved, then `the host is swapping out too fast` and `memory pressure is critical` within 6 to 40 s. Try 7 was refreshing only 5 projects when it reached level 4. Try 6 printed `waiting for memory headroom (17182 of 1024 MB reserved)` while inferring 2 projects.
2. What the fleet has delivered goes down between tries, not up: taken at start 274 (try 3), 376 (try 5), then 234, 95, 48 (tries 8, 9, 10), while the stale count stays at 650 to 658. In try 2 the `arrived` count itself fell during the wait, from 550 to 274 of 650.
3. Tries 6 and 7 saw a different graph (`generation 61`, 2 and 5 stale projects) than tries 1 to 5 (`generation 18`/`19`, 650 to 703 stale) and tries 8 to 10 (657 to 658 stale). Why is not visible in the logs.
4. Try 4: `macOS denied the resident service access to your Documents folder (reference e07192d7). The service runs without the file access consent of this session. Allow repo under System Settings, Privacy & Security, Files and Folders (or Full Disk Access), run repo service restart, or keep the checkout outside Documents, Desktop and Downloads.` Service log: `request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)`. It happened once in ten tries; the tries before and after it read the tree. I changed no setting.
5. Try 6 ended with `The resident service met an unexpected failure (reference dad6d735)`; the service log has `request-failed dad6d735 Error` with no message text on that line.
6. The Claude desktop app's `git diff --numstat` (see round 3) kept running during the round at up to about 2.5 GB resident; the parent chains are in `round-5.samples.txt`.

## Per Try, Verbatim

The service log part of each try was cut from `$TMPDIR/semio-repo/501/host-*.log` by byte offset at try start and end. Repeats that differ only in numbers are left out.

## Try 1

- Log `/Users/ueli/native-bootstrap-round-5.log`: `start 2026-11-14T17:54:11Z`, `end 2026-11-14T18:05:45Z`, `native_rc=7`
- Runner: `native_rc=7 peak_rss_kib=6959920 run_peak_rss_kib=459744 hard_stop=memory pressure level 4 (0 samples of 2 in a row) fleet_refusal=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 5% of the default from 12 earlier run(s) of its class, at least 2313 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 2978 MB reserved (8622 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 49% of the default from 11 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 3020 MB reserved (6682 MB available, 2048 MB floor)
[capacity] run: work has to run, waiting 0s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (-3419 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 10002 MB)
[capacity] run: work has to run, waiting 11s to grow the reservation from 2048 MB to 8190 MB: the host is swapping out too fast (-10941 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] run: work has to run, waiting 22s to grow the reservation from 2048 MB to 8190 MB: memory pressure is raised (-17858 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] run: work has to run, waiting 38s to grow the reservation from 2048 MB to 2574 MB: memory pressure is critical (-23146 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind the host itself (nothing of ours)
```

Last distinct lines of the bootstrap log:
```
Reading the project graph slice (412 s): owners 49 of 703; running (name, kind, seconds, rows): @semio-tech/framework (project, 15, 72), @semio-tech/framework-async-rs (language:cargo, 11, 0).
Reading the project graph slice (422 s): owners 50 of 703; running (name, kind, seconds, rows): @semio-tech/framework-async-rs (language:cargo, 21, 0); waiting for memory headroom (1846 of -786 MB reserved).
[capacity] run: work has to run, waiting 0s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (-3419 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 10002 MB)
Reading the project graph slice (432 s): owners 52 of 703; running (name, kind, seconds, rows): @semio-tech/framework-diagnostic-rs (language:cargo, 5, 0); waiting for memory headroom (4359 of -6046 MB reserved).
[capacity] run: work has to run, waiting 11s to grow the reservation from 2048 MB to 8190 MB: the host is swapping out too fast (-10941 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind the host itself (nothing of ours)
Reading the project graph slice (449 s): owners 53 of 703; running (name, kind, seconds, rows): @semio-tech/framework-editor-rs (language:cargo, 5, 0); waiting for memory headroom (6093 of -5346 MB reserved).
[capacity] run: work has to run, waiting 22s to grow the reservation from 2048 MB to 8190 MB: memory pressure is raised (-17858 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind the host itself (nothing of ours)
Reading the project graph slice (461 s): owners 55 of 703; running (name, kind, seconds, rows): @semio-tech/framework-kernel (project, 5, 0).
Reading the project graph slice (465 s): owners 55 of 703; running (name, kind, seconds, rows): @semio-tech/framework-kernel (project, 10, 0); waiting for memory headroom (5287 of -2939 MB reserved).
[capacity] run: work has to run, waiting 38s to grow the reservation from 2048 MB to 2574 MB: memory pressure is critical (-23146 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind the host itself (nothing of ours)
end 2026-11-14T18:05:45Z
native_rc=7
```

Service log, wanted lines (0 `arrived` lines; first `-`, last `-`):
```
graph: refreshing 703 projects (generation 18)
graph: 537 stale facts (file 224, analysis 121, readdir 92, field 77, digest 20, readdirNames 2, exists 1) leave 703 projects stale
graph: asking the host for up to 8 worker processes for 48 projects; projects are inferred in this process meanwhile
[capacity] waiting 70s for exec: memory pressure is critical (need 1536 MB; -23146 MB spare above the 2048 MB floor, load 17.87; 2 running, 1 ahead); behind the host itself (nothing of ours)
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[script] build:projects · Inferring projects: 7 of 48 projects
[script] {"owner":"repository:build","command":"build:projects","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:57:703:1:0:%40semio-tech%2Fframework-actor-retained-context-rs|language%3Arust|0|0","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:8:48","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 8 of 48 projects
[script] {"owner":"repository:build","command":"build:projects","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
graph: waiting for memory headroom (1322 of -10099 MB reserved, 1 projects running)
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:9:48:1:0:%40semio-tech%2Fframework-actor-retained-turn-rs|language%3Arust|0|0:waiting|1322|0|","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 9 of 48 projects · 1 running, longest 0 s: @semio-tech/framework-actor-retained-turn-rs (language:rust, 0 s, 0 rows) · waiting for memory headroom (1322 of 0 MB reserved)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:57:703:1:0:%40semio-tech%2Fframework-actor-retained-turn-rs|language%3Arust|0|0:waiting|1322|0|","mode":"shared","elapsedMs":0}}
[capacity] waiting 88s for exec: memory pressure is critical (need 1536 MB; -25452 MB spare above the 2048 MB floor, load 14.03; 2 running, 1 ahead); behind the host itself (nothing of ours)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:58:703:1:0:%40semio-tech%2Fframework-actor-retained-turn-rs|language%3Arust|0|0:waiting|1322|0|","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:9:48","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 9 of 48 projects
[script] {"owner":"repository:build","command":"build:projects","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
[fleet] projects from the fleet: 10 taken, 0 stored, 0 offered, 0 waited for another machine, 0 computed here | Projekte aus der Flotte: 10 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 0 hier berechnet
Cancelled (SIGINT); leases released.
[repository:command] still running after 490 s
[repo] still working after 500 s; last phase started: source.served
[repository:command] still running after 500 s
[repository:command] still running after 510 s
```

## Try 2

- Log `/Users/ueli/native-bootstrap-round-5-try-2.log`: `start 2026-11-14T18:09:14Z`, `end 2026-11-14T18:39:57Z`, `native_rc=75`
- Runner: `native_rc=75 peak_rss_kib=3144992 run_peak_rss_kib=357360 hard_stop=None fleet_refusal=True`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 5% of the default from 13 earlier run(s) of its class, at least 2316 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 2981 MB reserved (8659 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 7 earlier run(s) of its class
[capacity] dependencies: sized at 49% of the default from 12 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 3020 MB reserved (8675 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Reading the project graph slice (791 s): flush 1 of 1.
[repo] still working after 920 s; last phase started: door.script
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 650 and delivered 274 within 1499 s, 376 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-14T18:39:57Z
native_rc=75
```

Service log, wanted lines (152 `arrived` lines; first `graph: waiting for the fleet: 550 of 650 projects arrived`, last `graph: waiting for the fleet: 274 of 650 projects arrived`):
```
graph: 650 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 650
graph: waiting for the fleet: 550 of 650 projects arrived
graph: nothing to wait for any longer: asked the fleet for 650 stale projects and received 274 within 1499 s, 376 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T18:39:57.205Z graph-wanted-expired {"wanted":"650","arrived":"274","seconds":"1499","missing":"376","local":"192"}
```

Service log, last 30 lines of the try (cut at 300 characters):
```
graph: waiting for the fleet: 274 of 650 projects arrived
[repository:command] still running after 2440 s
graph: waiting for the fleet: 274 of 650 projects arrived
[repo] still working after 2450 s; last phase started: source.served
[repository:command] still running after 2450 s
graph: waiting for the fleet: 274 of 650 projects arrived
[repository:command] still running after 2460 s
graph: waiting for the fleet: 274 of 650 projects arrived
[repository:command] still running after 2470 s
graph: waiting for the fleet: 274 of 650 projects arrived
[repo] still working after 2480 s; last phase started: source.served
[repository:command] still running after 2480 s
graph: waiting for the fleet: 274 of 650 projects arrived
[repository:command] still running after 2490 s
graph: waiting for the fleet: 274 of 650 projects arrived
[repository:command] still running after 2500 s
graph: waiting for the fleet: 274 of 650 projects arrived
[repo] still working after 2510 s; last phase started: source.served
[repository:command] still running after 2510 s
graph: waiting for the fleet: 274 of 650 projects arrived
[repository:command] still running after 2520 s
graph: waiting for the fleet: 274 of 650 projects arrived
[repository:command] still running after 2530 s
graph: waiting for the fleet: 274 of 650 projects arrived
[repo] still working after 2540 s; last phase started: source.served
[repository:command] still running after 2540 s
graph: nothing to wait for any longer: asked the fleet for 650 stale projects and received 274 within 1499 s, 376 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T18:39:57.205Z graph-wanted-expired {"wanted":"650","arrived":"274","seconds":"1499","missing":"376","local":"192"}
[repository:command] still running after 2550 s
[repository:command] still running after 2560 s
```

## Try 3

- Log `/Users/ueli/native-bootstrap-round-5-try-3.log`: `start 2026-11-14T18:42:20Z`, `end 2026-11-14T18:53:23Z`, `native_rc=7`
- Runner: `native_rc=7 peak_rss_kib=3272336 run_peak_rss_kib=467680 hard_stop=memory pressure level 4 (0 samples of 2 in a row) fleet_refusal=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 14 earlier run(s) of its class, at least 2322 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 2910 MB reserved (7774 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 7 earlier run(s) of its class
[capacity] dependencies: sized at 49% of the default from 13 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 3020 MB reserved (6093 MB available, 2048 MB floor)
[capacity] run: work has to run, waiting 0s to grow the reservation from 2048 MB to 14336 MB: not enough free memory (535 MB spare above the 2048 MB floor; 2 running, 0 ahead); behind graph-plan (running, 1 job(s), 2255 MB), graph-farm (running, 1 job(s), 0 MB)
[capacity] run: work has to run, waiting 6s to grow the reservation from 2048 MB to 14336 MB: not enough free memory (-10571 MB spare above the 2048 MB floor; 2 running, 0 ahead); behind graph-plan (running, 1 job(s), 17528 MB), graph-farm (running, 1 job(s), 0 MB)
[capacity] run: work has to run, waiting 12s to grow the reservation from 2048 MB to 14336 MB: the host is swapping out too fast (-10571 MB spare above the 2048 MB floor; 2 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] run: work has to run, waiting 23s to grow the reservation from 2048 MB to 14336 MB: memory pressure is critical (-17748 MB spare above the 2048 MB floor; 2 running, 0 ahead); behind the host itself (nothing of ours)
```

Last distinct lines of the bootstrap log:
```
Reading the project graph slice (356 s): owners 13 of 650; running (name, kind, seconds, rows): @semio-tech/framework-actor-rs (language:cargo, 2, 0).
Reading the project graph slice (364 s): owners 29 of 650; running (name, kind, seconds, rows): @semio-tech/framework-dsl-rs (language:cargo, 2, 0), @semio-tech/framework-dsl-record-derive-rs (language:cargo, 2, 0), @semio-tech/framework-dsl-record-rs (language:cargo, 2, 0).
Reading the project graph slice (374 s): owners 40 of 650; running (name, kind, seconds, rows): @semio-tech/framework-job-rs (language:cargo, 1, 0), @semio-tech/framework-job-fault-publication-rs (language:rust, 0, 0).
Reading the project graph slice (380 s): owners 46 of 650; running (name, kind, seconds, rows): @semio-tech/framework-os (project, 0, 0).
Reading the project graph slice (386 s): owners 46 of 650; running (name, kind, seconds, rows): @semio-tech/framework-os (project, 7, 0); waiting for memory headroom (737 of 933 MB reserved).
[capacity] run: work has to run, waiting 0s to grow the reservation from 2048 MB to 14336 MB: not enough free memory (535 MB spare above the 2048 MB floor; 2 running, 0 ahead); behind graph-plan (running, 1 job(s), 2255 MB), graph-farm (running, 1 job(s), 0 MB)
[capacity] run: work has to run, waiting 6s to grow the reservation from 2048 MB to 14336 MB: not enough free memory (-10571 MB spare above the 2048 MB floor; 2 running, 0 ahead); behind graph-plan (running, 1 job(s), 17528 MB), graph-farm (running, 1 job(s), 0 MB)
[capacity] run: work has to run, waiting 12s to grow the reservation from 2048 MB to 14336 MB: the host is swapping out too fast (-10571 MB spare above the 2048 MB floor; 2 running, 0 ahead); behind the host itself (nothing of ours)
Reading the project graph slice (418 s): owners 47 of 650; running (name, kind, seconds, rows): @semio-tech/framework-os-dev (project, 5, 0).
[capacity] run: work has to run, waiting 23s to grow the reservation from 2048 MB to 14336 MB: memory pressure is critical (-17748 MB spare above the 2048 MB floor; 2 running, 0 ahead); behind the host itself (nothing of ours)
end 2026-11-14T18:53:23Z
native_rc=7
```

Service log, wanted lines (22 `arrived` lines; first `graph: waiting for the fleet: 0 of 376 projects arrived`, last `graph: waiting for the fleet: 192 of 376 projects arrived`):
```
graph: 650 stale projects, 274 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 376
graph: waiting for the fleet: 0 of 376 projects arrived
graph: refreshing 650 projects (generation 19)
graph: 349 stale facts (file 218, analysis 91, field 38, digest 2) leave 650 projects stale
graph: asking the host for up to 8 worker processes for 48 projects; projects are inferred in this process meanwhile
[capacity] waiting 95s for exec: memory pressure is critical (need 1536 MB; -21162 MB spare above the 2048 MB floor, load 6.84; 3 running, 1 ahead); behind the host itself (nothing of ours)
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[script] build:projects · Inferring projects: 46 of 48 projects
[script] {"owner":"repository:build","command":"build:projects","stage":"running"}
[capacity] waiting 77s for exec: the host is swapping out too fast (need 1536 MB; -10571 MB spare above the 2048 MB floor, load 6.47; 3 running, 1 ahead); behind the host itself (nothing of ours)
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:47:48:1:5:%40semio-tech%2Fframework-os-dev|project|5|0","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 47 of 48 projects · 1 running, longest 5 s: @semio-tech/framework-os-dev (project, 5 s, 0 rows)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:47:650:1:5:%40semio-tech%2Fframework-os-dev|project|5|0","mode":"shared","elapsedMs":0}}
[repository:command] still running after 420 s
[capacity] waiting 83s for exec: the host is swapping out too fast (need 1536 MB; -13390 MB spare above the 2048 MB floor, load 6.99; 3 running, 1 ahead); behind the host itself (nothing of ours)
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:47:48:1:10:%40semio-tech%2Fframework-os-dev|project|10|0","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 47 of 48 projects · 1 running, longest 10 s: @semio-tech/framework-os-dev (project, 10 s, 0 rows)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:47:650:1:10:%40semio-tech%2Fframework-os-dev|project|10|0","mode":"shared","elapsedMs":0}}
[capacity] waiting 88s for exec: the host is swapping out too fast (need 1536 MB; -17748 MB spare above the 2048 MB floor, load 7.17; 3 running, 1 ahead); behind the host itself (nothing of ours)
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:47:48:1:15:%40semio-tech%2Fframework-os-dev|project|15|0","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 47 of 48 projects · 1 running, longest 15 s: @semio-tech/framework-os-dev (project, 15 s, 0 rows)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:47:650:1:15:%40semio-tech%2Fframework-os-dev|project|15|0","mode":"shared","elapsedMs":0}}
[repository:command] still running after 430 s
[capacity] waiting 95s for exec: memory pressure is critical (need 1536 MB; -21162 MB spare above the 2048 MB floor, load 6.84; 3 running, 1 ahead); behind the host itself (nothing of ours)
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:47:48:1:20:%40semio-tech%2Fframework-os-dev|project|20|0","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 47 of 48 projects · 1 running, longest 20 s: @semio-tech/framework-os-dev (project, 20 s, 0 rows)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:47:650:1:20:%40semio-tech%2Fframework-os-dev|project|20|0","mode":"shared","elapsedMs":0}}
[repo] still working after 440 s; last phase started: source.served
[capacity] waiting 101s for exec: the host is swapping out too fast (need 1536 MB; -25977 MB spare above the 2048 MB floor, load 7.33; 3 running, 1 ahead); behind the host itself (nothing of ours)
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:47:48:1:25:%40semio-tech%2Fframework-os-dev|project|25|0","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 47 of 48 projects · 1 running, longest 25 s: @semio-tech/framework-os-dev (project, 25 s, 0 rows)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:47:650:1:25:%40semio-tech%2Fframework-os-dev|project|25|0","mode":"shared","elapsedMs":0}}
[repository:command] still running after 440 s
[repository:command] still running after 450 s
[repository:command] still running after 460 s
[repo] still working after 470 s; last phase started: source.served
[repository:command] still running after 470 s
```

## Try 4

- Log `/Users/ueli/native-bootstrap-round-5-try-4.log`: `start 2026-11-14T18:56:53Z`, `end 2026-11-14T19:00:20Z`, `native_rc=1`
- Runner: `native_rc=1 peak_rss_kib=3684368 run_peak_rss_kib=361728 hard_stop=None fleet_refusal=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 13 earlier run(s) of its class, at least 2325 MB because of its assembly phase
[capacity] admitted run with 6 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 59% of the default from 7 earlier run(s) of its class
[capacity] dependencies: sized at 49% of the default from 13 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 3020 MB reserved (9024 MB available, 2048 MB floor)
[capacity] admitted run: 1 job(s), 2048 MB reserved (9143 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
[script] bun:dependency-storage remains active on dependencies:store-ownership:frontier:506688 (exclusive, 17 s)
[script] bun:dependency-storage · Checking declared package providers: 1 edges
[DATA] Store dependency concurrency {"version":1,"purpose":"dependency-concurrency-event","event":"kept","count":6}
Kept 6 superseded dependency generation(s) that are pinned, recent or the rollback target.
[semio] workspace:setup…
[capacity] run: sized at 95% of the default from 4 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (9143 MB available, 2048 MB floor)
Loading the project graph of 440cffa5563d.
Planning 1 targets.
macOS denied the resident service access to your Documents folder (reference e07192d7). The service runs without the file access consent of this session. Allow `repo` under System Settings, Privacy & Security, Files and Folders (or Full Disk Access), run `repo service restart`, or keep the checkout outside Documents, Desktop and Downloads. Details: /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-
end 2026-11-14T19:00:20Z
native_rc=1
```

Service log, wanted lines (0 `arrived` lines; first `-`, last `-`):
```
2026-11-14T19:00:20.265Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[repository:command] still running after 720 s
[repository:command] still running after 730 s
[repo] still working after 740 s; last phase started: source.served
[repository:command] still running after 740 s
[repository:command] still running after 750 s
[repository:command] still running after 760 s
[repo] still working after 770 s; last phase started: source.served
[repository:command] still running after 770 s
[repository:command] still running after 780 s
[repository:command] still running after 790 s
[repo] still working after 800 s; last phase started: source.served
[repository:command] still running after 800 s
[repository:command] still running after 810 s
[repository:command] still running after 820 s
[repo] still working after 830 s; last phase started: source.served
[repository:command] still running after 830 s
[repository:command] still running after 840 s
[repository:command] still running after 850 s
[repo] still working after 860 s; last phase started: source.served
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (9143 MB available, 2048 MB floor)
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] build:source-admission · Admitting the sources of the build
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
[script] {"owner":"repository:build","command":"build:source-admission","stage":"running"}
2026-11-14T19:00:20.265Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
    at failure (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148:307)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:153:768)
    at processTicksAndRejections (native:7:39)
[repository:command] still running after 860 s
```

## Try 5

- Log `/Users/ueli/native-bootstrap-round-5-try-5.log`: `start 2026-11-14T19:02:24Z`, `end 2026-11-14T19:15:27Z`, `native_rc=7`
- Runner: `native_rc=7 peak_rss_kib=7703616 run_peak_rss_kib=369536 hard_stop=memory pressure level 4 (0 samples of 2 in a row) fleet_refusal=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 14 earlier run(s) of its class, at least 2326 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 2921 MB reserved (8849 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 7 earlier run(s) of its class
[capacity] dependencies: sized at 49% of the default from 14 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 3020 MB reserved (9063 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Reading the project graph slice (34 s): flush 1 of 1.
Reading the project graph slice (356 s): rederivation 256 of 256.
Reading the project graph slice (368 s): owners 0 of 650; running (name, kind, seconds, rows): @semio-tech/framework (project, 0, 0).
Reading the project graph slice (372 s): owners 2 of 650; running (name, kind, seconds, rows): @semio-tech/framework-async-rs (language:cargo, 0, 0).
Reading the project graph slice (376 s): owners 3 of 650.
Reading the project graph slice (382 s): owners 14 of 650; running (name, kind, seconds, rows): @semio-tech/framework-actor-rs (language:cargo, 2, 0), @semio-tech/framework-mesh-engine-rs (language:cargo, 0, 0).
Reading the project graph slice (386 s): owners 28 of 650; running (name, kind, seconds, rows): @semio-tech/framework-deflate-rs (language:cargo, 0, 0), @semio-tech/framework-dsl-rs (language:cargo, 0, 0).
Reading the project graph slice (388 s): owners 30 of 650; running (name, kind, seconds, rows): @semio-tech/framework-dsl-record-derive-rs (language:cargo, 1, 0), @semio-tech/framework-dsl-record-rs (language:cargo, 1, 0), @semio-tech/framework-artifact-reference-rs (language:cargo, 1, 0).
Reading the project graph slice (398 s): owners 46 of 650; running (name, kind, seconds, rows): @semio-tech/framework-os (project, 1, 0).
Reading the project graph slice (404 s): owners 47 of 650; running (name, kind, seconds, rows): @semio-tech/framework-os-dev (project, 2, 0).
end 2026-11-14T19:15:27Z
native_rc=7
```

Service log, wanted lines (27 `arrived` lines; first `graph: waiting for the fleet: 0 of 274 projects arrived`, last `graph: waiting for the fleet: 274 of 274 projects arrived`):
```
graph: 650 stale projects, 376 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 274
graph: waiting for the fleet: 0 of 274 projects arrived
graph: refreshing 650 projects (generation 19)
graph: 349 stale facts (file 218, analysis 91, field 38, digest 2) leave 650 projects stale
graph: asking the host for up to 8 worker processes for 48 projects; projects are inferred in this process meanwhile
[capacity] waiting 63s for exec: memory pressure is critical (need 1536 MB; -10909 MB spare above the 2048 MB floor, load 9.03; 5 running, 0 ahead); behind the host itself (nothing of ours)
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:47:650:1:175:%40semio-tech%2Fframework-os-dev|project|175|1100","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[repo] still working after 1760 s; last phase started: source.served
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[capacity] waiting 74s for exec: memory pressure is critical (need 1536 MB; -10909 MB spare above the 2048 MB floor, load 8.74; 5 running, 0 ahead); behind the host itself (nothing of ours)
[repository:command] still running after 1761 s
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[capacity] waiting 80s for exec: memory pressure is critical (need 1536 MB; -16299 MB spare above the 2048 MB floor, load 8.44; 5 running, 0 ahead); behind the host itself (nothing of ours)
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:47:48:1:185:%40semio-tech%2Fframework-os-dev|project|185|1113","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 47 of 48 projects · 1 running, longest 185 s: @semio-tech/framework-os-dev (project, 185 s, 1113 rows)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:47:650:1:185:%40semio-tech%2Fframework-os-dev|project|185|1113","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
graph: 1 of 48 projects inferred by 4 worker processes in 226 s (0 failed, 0 lost, 0 restarts)
[fleet] projects from the fleet: 0 taken, 0 stored, 0 offered, 0 waited for another machine, 1 computed here | Projekte aus der Flotte: 0 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 1 hier berechnet
Cancelled (SIGINT); leases released.
[repository:command] still running after 1771 s
[repository:command] still running after 1781 s
[repo] still working after 1790 s; last phase started: source.served
[repository:command] still running after 1791 s
```

## Try 6

- Log `/Users/ueli/native-bootstrap-round-5-try-6.log`: `start 2026-11-14T19:18:57Z`, `end 2026-11-14T19:24:27Z`, `native_rc=1`
- Runner: `native_rc=1 peak_rss_kib=3952240 run_peak_rss_kib=356816 hard_stop=None fleet_refusal=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 14 earlier run(s) of its class, at least 2330 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 2946 MB reserved (10192 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 7 earlier run(s) of its class
[capacity] dependencies: sized at 34% of the default from 14 earlier run(s) of its class, at least 2506 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2506 MB reserved (8281 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Reading the project graph slice (2 s).
[repo] still working after 5 s; last phase started: door.script
Reading the project graph slice (36 s): files 0 of 0.
Reading the project graph slice (40 s): facts 28672 of 268459.
Reading the project graph slice (60 s): verification 0 of 2.
Reading the project graph slice (72 s): flush 1 of 1.
Reading the project graph slice (80 s): owners 0 of 2; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 0, 0).
Reading the project graph slice (86 s): owners 0 of 2; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 5, 0); waiting for memory headroom (17182 of 1024 MB reserved).
Reading the project graph slice (124 s): owners 0 of 2; running (name, kind, seconds, rows): workspace (root:language:cargo, 4, 0).
The resident service met an unexpected failure (reference dad6d735); try again, the details under that reference are in the service log /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-a561d8014168.log.
end 2026-11-14T19:24:27Z
native_rc=1
```

Service log, wanted lines (0 `arrived` lines; first `-`, last `-`):
```
graph: generation 60 (5099 projects) restored from the remote cache, 5103 documents (48 MB) downloaded in 12 s
graph: refreshing 2 projects (generation 61)
graph: 6 stale facts (directoryExists 3, readdir 2, exists 1) leave 2 projects stale
2026-11-14T19:24:27.380Z request-failed dad6d735 Error
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-imports","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[repository:command] still running after 2301 s
[script] {"owner":"repository:build","command":"build:closure-entry-rows","stage":"running"}
[script] {"owner":"repository:build","command":"build:project-assembly","stage":"running"}
[script] {"owner":"repository:build","command":"build:command-source-imports","stage":"running"}
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:0:2:1:20:workspace|root%3Alanguage%3Acargo|20|18748","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 0 of 2 projects · 1 running, longest 20 s: workspace (root:language:cargo, 20 s, 18748 rows)
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:0:2:1:20:workspace|root%3Alanguage%3Acargo|20|18748","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:1:2:1:20:workspace|root%3Alanguage%3Acargo|20|18748","mode":"shared","elapsedMs":0}}
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:0:2","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 0 of 2 projects
[fleet] projects from the fleet: 2 taken, 0 stored, 0 offered, 0 waited for another machine, 0 computed here | Projekte aus der Flotte: 2 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 0 hier berechnet
2026-11-14T19:24:27.380Z request-failed dad6d735 Error
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs:1030:284)
    at readInferencePhysicalRecordV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs:1055:1234)
    at <anonymous> (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs:929:25)
    at observeInferenceComputationV1 (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs:937:98)
    at async observeProjectInferenceV1 (unknown)
    at async tagProjectFailureV1 (unknown)
    at async <anonymous> (unknown)
    at async <anonymous> (unknown)
    at async run (unknown)
    at async <anonymous> (unknown)
    at async awaitGrowingV1 (unknown)
    at async emojiProjectJsonNodes (/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:283:13)
    at processTicksAndRejections (unknown:7:39)
```

## Try 7

- Log `/Users/ueli/native-bootstrap-round-5-try-7.log`: `start 2026-11-14T19:26:29Z`, `end 2026-11-14T19:35:32Z`, `native_rc=7`
- Runner: `native_rc=7 peak_rss_kib=6196816 run_peak_rss_kib=370384 hard_stop=memory pressure level 4 (0 samples of 2 in a row) fleet_refusal=False`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 14 earlier run(s) of its class, at least 2331 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 2947 MB reserved (8579 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 7 earlier run(s) of its class
[capacity] dependencies: sized at 34% of the default from 15 earlier run(s) of its class, at least 2511 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2511 MB reserved (8086 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Reading the project graph slice (18 s): facts 20480 of 271628.
Reading the project graph slice (40 s): verification 0 of 2.
Reading the project graph slice (50 s): flush 1 of 1.
Reading the project graph slice (100 s): doubt 451523 of 451523.
Reading the project graph slice (138 s): owners 0 of 5; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 0, 0).
Reading the project graph slice (142 s): owners 0 of 5; running (name, kind, seconds, rows): @semio-tech/s-dev-composition (project, 5, 0); waiting for memory headroom (17182 of 1024 MB reserved).
Reading the project graph slice (202 s): owners 1 of 5.
Reading the project graph slice (204 s): owners 1 of 5; running (name, kind, seconds, rows): workspace (root:language:cargo, 2, 0).
Reading the project graph slice (210 s): owners 1 of 5; running (name, kind, seconds, rows): workspace (root:language:cargo, 7, 0); waiting for memory headroom (1300 of 296 MB reserved).
Reading the project graph slice (226 s): owners 3 of 5; running (name, kind, seconds, rows): @semio-tech/ui-contract-rs (language:cargo, 2, 0).
end 2026-11-14T19:35:32Z
native_rc=7
```

Service log, wanted lines (0 `arrived` lines; first `-`, last `-`):
```
graph: refreshing 5 projects (generation 61)
graph: 9 stale facts (directoryExists 3, exists 3, readdir 3) leave 5 projects stale
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[script] {"owner":"repository:build","command":"build:projects","stage":"running","activity":{"resource":"build:graph-phase:inference:5:5","mode":"shared","elapsedMs":0}}
[script] build:projects · Inferring projects: 5 of 5 projects
[fleet] projects from the fleet: 1 taken, 0 stored, 0 offered, 0 waited for another machine, 3 computed here | Projekte aus der Flotte: 1 übernommen, 0 gespeichert, 0 angeboten, 0 auf eine andere Maschine gewartet, 3 hier berechnet
[script] {"owner":"repository:build","command":"build:infer","stage":"running"}
[script] build:infer · Inferring projects
[script] {"owner":"repository:build","command":"build:generator-producers","stage":"running"}
[script] build:generator-producers · Collecting generator producers
[script] {"owner":"repository:build","command":"build:modules","stage":"running"}
[repository:command] still running after 2861 s
[script] {"owner":"repository:build","command":"build:graph","stage":"complete"}
[script] {"owner":"repository:build","command":"graph:slice-worker","stage":"running","activity":{"resource":"build:graph-phase:owners:5:5","mode":"shared","elapsedMs":0}}
[repo] still working after 2870 s; last phase started: source.served
[repository:command] still running after 2871 s
[repository:command] still running after 2881 s
[repository:command] still running after 2891 s
[repo] still working after 2900 s; last phase started: source.served
[repository:command] still running after 2901 s
[repository:command] still running after 2911 s
[repository:command] still running after 2921 s
[repo] still working after 2930 s; last phase started: source.served
[repository:command] still running after 2931 s
[repository:command] still running after 2941 s
[repository:command] still running after 2951 s
[repo] still working after 2960 s; last phase started: source.served
[repository:command] still running after 2961 s
[repository:command] still running after 2971 s
[repository:command] still running after 2981 s
[repo] still working after 2990 s; last phase started: source.served
[repository:command] still running after 2991 s
[repository:command] still running after 3001 s
```

## Try 8

- Log `/Users/ueli/native-bootstrap-round-5-try-8.log`: `start 2026-11-14T19:39:02Z`, `end 2026-11-14T20:10:06Z`, `native_rc=75`
- Runner: `native_rc=75 peak_rss_kib=3654768 run_peak_rss_kib=469760 hard_stop=None fleet_refusal=True`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 15 earlier run(s) of its class, at least 2334 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 2929 MB reserved (7993 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 7 earlier run(s) of its class
[capacity] dependencies: sized at 34% of the default from 15 earlier run(s) of its class, at least 2518 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2518 MB reserved (7283 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Reading the project graph slice (911 s): flush 1 of 1.
[repo] still working after 920 s; last phase started: door.script
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 423 and delivered 96 within 1488 s, 327 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-14T20:10:06Z
native_rc=75
```

Service log, wanted lines (148 `arrived` lines; first `graph: waiting for the fleet: 0 of 423 projects arrived`, last `graph: waiting for the fleet: 96 of 423 projects arrived`):
```
graph: 657 stale projects, 234 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 423
graph: waiting for the fleet: 0 of 423 projects arrived
graph: nothing to wait for any longer: asked the fleet for 423 stale projects and received 96 within 1488 s, 327 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T20:10:06.164Z graph-wanted-expired {"wanted":"423","arrived":"96","seconds":"1488","missing":"327","local":"192"}
```

Service log, last 30 lines of the try (cut at 300 characters):
```
graph: waiting for the fleet: 96 of 423 projects arrived
[repo] still working after 1550 s; last phase started: source.served
[repository:command] still running after 1550 s
graph: waiting for the fleet: 96 of 423 projects arrived
[repository:command] still running after 1560 s
graph: waiting for the fleet: 96 of 423 projects arrived
[repository:command] still running after 1570 s
graph: waiting for the fleet: 96 of 423 projects arrived
[repo] still working after 1580 s; last phase started: source.served
[repository:command] still running after 1580 s
graph: waiting for the fleet: 96 of 423 projects arrived
[repository:command] still running after 1590 s
graph: waiting for the fleet: 96 of 423 projects arrived
[repository:command] still running after 1600 s
graph: waiting for the fleet: 96 of 423 projects arrived
[repo] still working after 1610 s; last phase started: source.served
[repository:command] still running after 1610 s
graph: waiting for the fleet: 96 of 423 projects arrived
[repository:command] still running after 1620 s
graph: waiting for the fleet: 96 of 423 projects arrived
[repository:command] still running after 1630 s
graph: waiting for the fleet: 96 of 423 projects arrived
[repo] still working after 1640 s; last phase started: source.served
[repository:command] still running after 1640 s
graph: waiting for the fleet: 96 of 423 projects arrived
[repository:command] still running after 1650 s
graph: waiting for the fleet: 96 of 423 projects arrived
graph: nothing to wait for any longer: asked the fleet for 423 stale projects and received 96 within 1488 s, 327 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T20:10:06.164Z graph-wanted-expired {"wanted":"423","arrived":"96","seconds":"1488","missing":"327","local":"192"}
[repository:command] still running after 1660 s
```

## Try 9

- Log `/Users/ueli/native-bootstrap-round-5-try-9.log`: `start 2026-11-14T20:12:08Z`, `end 2026-11-14T20:33:17Z`, `native_rc=75`
- Runner: `native_rc=75 peak_rss_kib=3742448 run_peak_rss_kib=471216 hard_stop=None fleet_refusal=True`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 15 earlier run(s) of its class, at least 2340 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 2935 MB reserved (7773 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 7 earlier run(s) of its class
[capacity] dependencies: sized at 33% of the default from 15 earlier run(s) of its class, at least 2537 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2537 MB reserved (7085 MB available, 2048 MB floor)
```

Last distinct lines of the bootstrap log:
```
Reading the project graph slice (324 s): flush 1 of 1.
[repo] still working after 350 s; last phase started: door.script
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 563 and delivered 56 within 910 s, 507 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-14T20:33:17Z
native_rc=75
```

Service log, wanted lines (90 `arrived` lines; first `graph: waiting for the fleet: 0 of 563 projects arrived`, last `graph: waiting for the fleet: 56 of 563 projects arrived`):
```
graph: 658 stale projects, 95 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 563
graph: waiting for the fleet: 0 of 563 projects arrived
graph: nothing to wait for any longer: asked the fleet for 563 stale projects and received 56 within 910 s, 507 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T20:33:17.020Z graph-wanted-expired {"wanted":"563","arrived":"56","seconds":"910","missing":"507","local":"192"}
```

Service log, last 30 lines of the try (cut at 300 characters):
```
graph: waiting for the fleet: 56 of 563 projects arrived
[repository:command] still running after 970 s
graph: waiting for the fleet: 56 of 563 projects arrived
[repo] still working after 980 s; last phase started: source.served
[repository:command] still running after 980 s
graph: waiting for the fleet: 56 of 563 projects arrived
[repository:command] still running after 990 s
graph: waiting for the fleet: 56 of 563 projects arrived
[repository:command] still running after 1000 s
graph: waiting for the fleet: 56 of 563 projects arrived
[repo] still working after 1010 s; last phase started: source.served
[repository:command] still running after 1010 s
graph: waiting for the fleet: 56 of 563 projects arrived
[repository:command] still running after 1020 s
graph: waiting for the fleet: 56 of 563 projects arrived
[repository:command] still running after 1030 s
graph: waiting for the fleet: 56 of 563 projects arrived
[repo] still working after 1040 s; last phase started: source.served
[repository:command] still running after 1040 s
graph: waiting for the fleet: 56 of 563 projects arrived
[repository:command] still running after 1050 s
graph: waiting for the fleet: 56 of 563 projects arrived
[repository:command] still running after 1060 s
graph: waiting for the fleet: 56 of 563 projects arrived
[repo] still working after 1070 s; last phase started: source.served
[repository:command] still running after 1070 s
graph: nothing to wait for any longer: asked the fleet for 563 stale projects and received 56 within 910 s, 507 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T20:33:17.020Z graph-wanted-expired {"wanted":"563","arrived":"56","seconds":"910","missing":"507","local":"192"}
[repository:command] still running after 1080 s
[repository:command] still running after 1090 s
```

## Try 10

- Log `/Users/ueli/native-bootstrap-round-5-try-10.log`: `start 2026-11-14T20:35:42Z`, `end 2026-11-14T21:01:02Z`, `native_rc=75`
- Runner: `native_rc=75 peak_rss_kib=3699040 run_peak_rss_kib=470048 hard_stop=None fleet_refusal=True`
- nx task lines: 0

`[capacity]` lines of the bootstrap log:
```
[capacity] run: sized at 4% of the default from 15 earlier run(s) of its class, at least 2345 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 2940 MB reserved (7863 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 33% of the default from 15 earlier run(s) of its class, at least 2551 MB because of its assembly phase
[capacity] admitted dependencies: 1 job(s), 2551 MB reserved (6969 MB available, 2048 MB floor)
[capacity] waiting 0s for run: not enough free memory (need 2048 MB; 1226 MB spare above the 2048 MB floor, load 6.04; 0 running, 0 ahead); behind the host itself (nothing of ours)
```

Last distinct lines of the bootstrap log:
```
Reading the project graph slice (557 s): flush 1 of 1.
[repo] still working after 560 s; last phase started: door.script
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 610 and delivered 96 within 1141 s, 514 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
end 2026-11-14T21:01:02Z
native_rc=75
```

Service log, wanted lines (115 `arrived` lines; first `graph: waiting for the fleet: 0 of 610 projects arrived`, last `graph: waiting for the fleet: 96 of 610 projects arrived`):
```
graph: 658 stale projects, 48 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 610
graph: waiting for the fleet: 0 of 610 projects arrived
graph: nothing to wait for any longer: asked the fleet for 610 stale projects and received 96 within 1141 s, 514 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T21:01:02.004Z graph-wanted-expired {"wanted":"610","arrived":"96","seconds":"1141","missing":"514","local":"192"}
```

Service log, last 30 lines of the try (cut at 300 characters):
```
[repository:command] still running after 2631 s
graph: waiting for the fleet: 96 of 610 projects arrived
[repository:command] still running after 2641 s
graph: waiting for the fleet: 96 of 610 projects arrived
[repository:command] still running after 2651 s
graph: waiting for the fleet: 96 of 610 projects arrived
[repo] still working after 2660 s; last phase started: source.served
[repository:command] still running after 2661 s
graph: waiting for the fleet: 96 of 610 projects arrived
[repository:command] still running after 2671 s
graph: waiting for the fleet: 96 of 610 projects arrived
[repository:command] still running after 2681 s
graph: waiting for the fleet: 96 of 610 projects arrived
[repo] still working after 2690 s; last phase started: source.served
[repository:command] still running after 2691 s
graph: waiting for the fleet: 96 of 610 projects arrived
[repository:command] still running after 2701 s
graph: waiting for the fleet: 96 of 610 projects arrived
[repository:command] still running after 2711 s
graph: waiting for the fleet: 96 of 610 projects arrived
[repo] still working after 2720 s; last phase started: source.served
[repository:command] still running after 2721 s
graph: waiting for the fleet: 96 of 610 projects arrived
[repository:command] still running after 2731 s
graph: waiting for the fleet: 96 of 610 projects arrived
graph: nothing to wait for any longer: asked the fleet for 610 stale projects and received 96 within 1141 s, 514 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T21:01:02.004Z graph-wanted-expired {"wanted":"610","arrived":"96","seconds":"1141","missing":"514","local":"192"}
[repository:command] still running after 2741 s
[repo] still working after 2750 s; last phase started: source.served
[repository:command] still running after 2751 s
```


## Files

`round-5.tail.txt` (last 200 lines of try 10), `round-5.samples.txt`, `run-round-5.py`, `report-round.py`.
