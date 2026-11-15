# g1-m04 Round 5

Machine: `ueli-workstation-lin-1`, SteamOS (Linux 7.2.7-valve2-2-neptune-72-g31504582a073), 15848 MB memory, 25331 MB swap (10271 MB in use before the run), 146 GiB free on `/home`. Runner: `run-tries-5.sh`.

## Result

`native_rc=75` in all 10 tries, not ready. **Wanted 516, arrived 0, missing 516 in every try** (18 tries in a row with round 3, 16:51 to 20:21 UTC). No hard stop, no code-change end. Largest process over all tries 1879164 KiB; PSI memory `full avg10` maximum 4.47 %. Other workers received projects in the same hours (g1-m02: 551 of 703), so the missing answer is specific to this machine or to its request: it asks for **516** stale projects of graph `d09b06e73429`, the others for 703.

| Try | Start (UTC) | End (UTC) | Seconds | Wanted | Arrived | Missing | Waited | Peak RSS (KiB) |
|---|---|---|---|---|---|---|---|---|
| 1 | 18:29:18Z | 18:38:10Z | 532 | 516 | 0 | 516 | within 302 s | 1321528 |
| 2 | 18:40:19Z | 18:49:04Z | 525 | 516 | 0 | 516 | within 300 s | 1485024 |
| 3 | 18:51:20Z | 19:00:00Z | 520 | 516 | 0 | 516 | within 301 s | 1486536 |
| 4 | 19:02:21Z | 19:10:55Z | 514 | 516 | 0 | 516 | within 300 s | 1879164 |
| 5 | 19:13:22Z | 19:22:02Z | 520 | 516 | 0 | 516 | within 302 s | 1407192 |
| 6 | 19:24:23Z | 19:33:00Z | 517 | 516 | 0 | 516 | within 300 s | 1567356 |
| 7 | 19:35:24Z | 19:45:24Z | 600 | 516 | 0 | 516 | within 300 s | 1649360 |
| 8 | 19:47:25Z | 19:57:15Z | 590 | 516 | 0 | 516 | within 302 s | 1762188 |
| 9 | 19:59:27Z | 20:09:00Z | 573 | 516 | 0 | 516 | within 301 s | 1861740 |
| 10 | 20:11:28Z | 20:21:15Z | 587 | 516 | 0 | 516 | within 301 s | 1744936 |

Refusal (try 10): `The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 516 and delivered 0 within 301 s, 516 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.`

## Before the Run

```
pre-run facts, taken 2026-11-14T18:29:11Z
replica code digest: "code":"39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759"
repo show projects / repo service host processes (pid ppid etime rss command):
2312203    1977       31:58 142872 … 📜️script.ts repo service host run
(end of list)
last 8 lines of .cache/f/sidecars/build-agent.log:
2026-11-14T18:13:24.871Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T18:18:25.148Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T18:23:25.427Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T18:26:58.927Z agent code changed, restarting
2026-11-14T18:26:58.936Z worker stopped: ueli-workstation-lin-1-2304179
2026-11-14T18:26:59.789Z joining http://192.168.178.70:8981 offering rustc for 4 target(s)
2026-11-14T18:26:59.841Z worker sandbox: linux-sandbox confined network denied
2026-11-14T18:26:59.851Z worker registered: 911124a3-d275-405c-b024-50a3e6afb505 cache http://192.168.178.70:6094
memory: Mem:           15848        5934        1975          24        8299        9913 | Swap:          25331       10271       15060
psi: full avg10=0.01 avg60=0.12 avg300=0.04 total=21082423125
```

## Graph Service Log (step 5)

The asked-for lines are the same two in every try (files `round-5[-try-k].service.txt` hold them with the last 30 lines per try):

```
graph: 516 stale projects, 0 taken from the fleet, this machine can infer 0 (0 per wave); asking the fleet for 516
graph: waiting for the fleet: 0 of 516 projects arrived
```

No line with `inferred here`, `refreshing N projects`, `asking the host for` or `stale facts` in any try. Services seen: /run/user/1000/semio-repo/1000/host-3926e83b2907.log (7 tries) /run/user/1000/semio-repo/1000/host-6a6d5c233697.log (4 tries) /run/user/1000/semio-repo/1000/host-6b3804922737.log (1 tries) /run/user/1000/semio-repo/1000/host-c9106d915737.log (1 tries) 

Last 30 lines of the service log after try 10 (the newer service):

```
graph: waiting for the fleet: 0 of 516 projects arrived
[repository:command] still running after 290 s
graph: waiting for the fleet: 0 of 516 projects arrived
[repository:command] still running after 300 s
graph: waiting for the fleet: 0 of 516 projects arrived
[repository:command] still running after 310 s
[repo] still working after 320 s; last phase started: source.served
graph: waiting for the fleet: 0 of 516 projects arrived
[repository:command] still running after 320 s
graph: waiting for the fleet: 0 of 516 projects arrived
[repository:command] still running after 330 s
graph: waiting for the fleet: 0 of 516 projects arrived
[repository:command] still running after 340 s
[repo] still working after 350 s; last phase started: source.served
graph: waiting for the fleet: 0 of 516 projects arrived
[repository:command] still running after 350 s
[repository:command] still running after 360 s
graph: waiting for the fleet: 0 of 516 projects arrived
[repository:command] still running after 370 s
graph: waiting for the fleet: 0 of 516 projects arrived
[repo] still working after 380 s; last phase started: source.served
[repository:command] still running after 380 s
graph: waiting for the fleet: 0 of 516 projects arrived
[repository:command] still running after 390 s
graph: waiting for the fleet: 0 of 516 projects arrived
graph: nothing to wait for any longer: asked the fleet for 516 stale projects and received 0 within 301 s, 516 remain and this machine can infer only 0; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T20:21:15.495Z graph-wanted-expired {"wanted":"516","arrived":"0","seconds":"301","missing":"516","local":"0"}
[repository:command] still running after 400 s
[repo] still working after 410 s; last phase started: source.served
[repository:command] still running after 410 s
```

## Sidecar of This Machine During the Round

`.cache/f/sidecars/build-agent.log` says every five minutes `graph-server: alive, 0 machines seen, 0 requests held, read none` and restarted five times for `agent code changed`. Last lines:

```
2026-11-14T20:10:33.242Z worker registered: 68bc763a-15f7-4075-b84f-ab50f9b4835a cache http://192.168.178.70:6094
2026-11-14T20:15:28.279Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T20:20:28.491Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T20:20:33.063Z agent code changed, restarting
2026-11-14T20:20:33.069Z worker stopped: ueli-workstation-lin-1-2579635
2026-11-14T20:20:33.838Z joining http://192.168.178.70:8981 offering rustc for 4 target(s)
2026-11-14T20:20:33.876Z worker sandbox: linux-sandbox confined network denied
2026-11-14T20:20:33.884Z worker registered: 9e6e5e65-ef29-45bc-915f-7c2897647d03 cache http://192.168.178.70:6094
```

## `[capacity]` Lines (try 10; the earlier tries differ only in numbers)

```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] admitted run: 8 job(s), 7254 MB reserved (9796 MB available, 1981 MB floor)
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] dependencies: sized at 34% of the default from 16 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 6933 MB reserved (9161 MB available, 1981 MB floor)
[capacity] dependencies: sized at 34% of the default from 16 earlier run(s) of its class, at least 14144 MB because of its plan phase
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (9200 MB available, 1981 MB floor)
[capacity] run: reservation grown from 2048 MB to 8190 MB after 0s (8 job(s), 8729 MB available, 1981 MB floor)
```

Wall time per phase: not measured. A try takes 514 to 600 s, 300 to 302 s of it waiting for the fleet.

## For the Coordinator

A request from g1-m04 has never been answered (0 of 516, 18 tries). Please check on the group server whether requests of origin `g1-m04` arrive at all, and whether its graph `d09b06e73429` with 516 stale projects is one the server can serve. I continue with round 7.
