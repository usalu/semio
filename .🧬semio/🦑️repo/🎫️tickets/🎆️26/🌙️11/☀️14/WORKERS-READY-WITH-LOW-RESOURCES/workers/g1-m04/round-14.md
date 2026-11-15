# g1-m04 Round 14 (Linux, 16 GB, SteamOS)

Twelve tries from 02:09 to 06:36 UTC on 2026-11-15, all `native_rc=75`, marker absent, `ready: false`. No hard stop, no SIGINT, no setup task ran (the graph was never read). Rounds 15 to 24 arrived while this series ran; it was not interrupted and kept the round 14 rules. Step 0 of round 16 was not run in this series; it runs first in round 24.

Files beside this report: `round-14.tries.txt` (one line per try), `round-14[-try-k].service.txt` (graph lines of every service log touched), `.p12.txt` (5 s sampler), `.samples.txt` (30 s, three largest processes), `.monitor.txt`, `round-14.tail.txt` (last 200 log lines of try 12), `round-14.pre.txt`. Bootstrap logs: `~/native-bootstrap-round-14[-try-k].log`.

## What Round-Note P14 Asked For

| Asked | Expected | Measured |
|---|---|---|
| too heavy `K` in the first plan line | at most 14 | 68 (tries 1 to 5), 75 (try 6), 77 (tries 7 to 12), 5 in the second request of try 12 |
| grant `G` per project | about 3300 MB | **0 MB in every plan line of all twelve tries** |
| `it can infer` | 192 (48 per wave) | **0 (0 per wave)** in every plan line |
| `waiting for the fleet: x of R` grows within a minute | yes | only in tries 3, 4, 5 and 6; 0 in tries 1, 2 and 7 to 12 |
| restore line `restored … from a state A min old with S projects stale` | present | not found in any bootstrap or service log of this round |
| plan stage `environment` | under 10 s, under 700 MB | never reached; only `plan stage inference` was logged (tries 4 and 5) |

The Linux grant of 0 MB is still there on this machine although round 14 names it fixed, and round 17 names the same line (`0 MB per project … infer 0`) fixed for Macs. The last plan line here is from 06:31 UTC (code digest of the replica then `5455ef8abeb4…`, no pending code).

## Tries

| Try | UTC | First plan line (stale / taken / too heavy / asked) | Fleet delivered | End | Peak RSS of largest process | Slice worker peak |
|---|---|---|---|---|---|---|
| 1 | 02:09 to 02:29 | 517 / 0 / 68 / 517 | 0 within 901 s | 75 | 1825 MB | 1856 MB |
| 2 | 02:32 to 02:52 | 517 / 0 / 68 / 517 | 0 within 902 s | 75 | 1944 MB | 1942 MB |
| 3 | 02:55 to 03:24 | 517 / 0 / 68 / 517 | 246 within 1493 s | 75 | 1795 MB | 2094 MB |
| 4 | 03:27 to 03:49 | 517 / 246 / 1 / 271 | 271 of 271 | 75, local inference refused | 3257 MB | 3298 MB |
| 5 | 03:52 to 04:10 | 517 / 271 / 72 / 246 | 246 of 246 | 75, local inference refused | 3116 MB | 3191 MB |
| 6 | 04:13 to 04:41 | 517 / 0 / 75 / 517 | 24 within 1369 s | 75 | 1963 MB | 1936 MB |
| 7 | 04:44 to 04:57 | 540 / 0 / 77 / 540 | 0 within 414 s | 75 | 1844 MB | 1946 MB |
| 8 | 05:00 to 05:17 | 540 / 0 / 77 / 540 | 0 within 667 s | 75 | 1754 MB | 2022 MB |
| 9 | 05:21 to 05:38 | 540 / 0 / 77 / 540 | 0 within 302 s | 75 | 6722 MB | 1941 MB |
| 10 | 05:41 to 06:00 | 540 / 0 / 77 / 540 | 0 within 301 s | 75 | 6100 MB | 1905 MB |
| 11 | 06:03 to 06:18 | 540 / 0 / 77 / 540 | 0 within 301 s | 75 | 5729 MB | 1905 MB |
| 12 | 06:21 to 06:36 | 540 / 0 / 77 / 540, then 703 / 0 / 5 / 703 | 0 within 301 s | 75 | 1805 MB | 1874 MB |

Closure entries: 17458 before try 1, 17371 after try 4, 17372 after try 5, 17377 from try 9 on.

First plan line verbatim (try 1, the heavy list shortened by me):

```
graph: 517 stale projects, 0 taken from the fleet, 68 too heavy for this machine (…; it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 517
```

Refusal lines present in every try except 5:

```
graph: this machine is linux-x64 (class posix-shared); only machines of that class serve its request, and their graph server logs 'serving' for it
```

From try 8 on additionally:

```
graph: no machine of this class gave a sign of life for 5 min, so no one holds the request
graph: nothing to wait for any longer: asked the fleet for 540 stale projects and received 0 within 301 s, 540 remain and this machine can infer only 0; try again when a machine with free memory serves the graph, or read the graph on a larger machine
```

Last waiting line of the round (try 12): `graph: waiting for the fleet: 0 of 703 projects arrived; the server last progressed 586 s ago, estimated time left: about 1 min`.

## Tries 3 to 6: the Fleet Did Deliver, Then the State Was Lost

- Try 3: the server delivered 246 of 517 (first delivery ever on this machine), the command still ended with 75 after 1493 s.
- Try 4: second plan line `517 stale projects, 246 taken from the fleet, 1 too heavy … asking the fleet for 271`, then `271 of 271 projects arrived`. The plan went on to local inference: `graph: plan stage inference took 842.2 s (managed 2439 MB, 1134 MB more than at its start, resident 2867 MB)`, with many `graph: waiting for memory headroom (… of … MB reserved, n projects running)` lines. End: `The project graph could not be read on this machine yet: @semio-tech/energy-oracle-py (about 2052 MB) need(s) more memory than this machine can give one project (1024 MB). The inference was stopped before the host ran short of memory and the project(s) are marked too heavy here.`
- Try 5: `517 stale projects, 271 taken from the fleet, 72 too heavy … asking the fleet for 246`, `246 of 246 projects arrived`, `plan stage inference took 352.8 s (managed 1935 MB, 605 MB more than at its start, resident 2327 MB)`. End: the same refusal for `@semio-tech/framework-os-kernel (about 2334 MB)`, again with the limit 1024 MB. One retry line in the bootstrap log: `Store dependency sync retrying over the delta … "Bun current ownership changed after refreshed continuation"`.
- Try 6: the plan line is back at `0 taken from the fleet, 75 too heavy … asking the fleet for 517`; the 517 projects received in tries 4 and 5 were not counted any more. 24 arrived in 1369 s.

So with a served request the machine gets every project it asks for, and then fails on single projects above a 1024 MB limit per project while 8 to 10 GB are available (`[capacity] admitted run: 8 job(s), 7275 MB reserved (9675 MB available, 1981 MB floor)`).

## Memory

- Largest process of the round: 6722 MB at 05:27 UTC in try 9 (`bun --eval …`, pid 3727214, parent 3726756), 6100 MB in try 10, 5729 MB in try 11. The process running now with the same command line is a child of `repo service host run`; for the three peaks the parent's command line was not recorded. In these tries the bootstrap log has 74 to 91 `[capacity]` lines such as `waiting 6s for run: not enough free memory (need 6932 MB; 1182 MB spare above the 1981 MB floor, load 4.24; 1 running, 0 ahead); behind projects (running, 1 job(s), 6933 MB)`: a `projects` run with a 6933 MB reservation was active beside the bootstrap.
- Lowest `MemAvailable` in a 30 s sample at that peak: 3878 MB; PSI full avg10 8.66 %. Never under 1 GB.
- `[capacity]` first lines of the other tries: `run: sized at 5% of the default from 16 earlier run(s) of its class, at least 13585 MB because of its plan phase`, `admitted run: 8 job(s), 7261 MB reserved (9640 MB available, 1981 MB floor)`.
- Try 2: the service wrote `graph: the replica manifest cannot vouch for the tree (the replica daemon is not healthy); verifying the tree once.` `health.json` at 06:37 UTC lists `loop-blocked-restart` 18 times and `unstable` 3 times; the daemon had been restarted by its supervisor 9 minutes before.

## Setup Tasks

Not run in any try: succeeded 0, failed 0, not run all. No `source-differs`, `closure.moving`, `closure-missing`, `graph-plan-too-heavy` or dependency-store wait line in this round.

## Open for the Coordinator

1. `it can give about 0 MB per project … infer 0` on Linux in every plan line up to 06:31 UTC.
2. The per-project limit of 1024 MB that ended tries 4 and 5 after the fleet had delivered everything.
3. From 04:44 UTC on nobody of class `posix-shared` served the request (`no machine of this class gave a sign of life for 5 min`).
4. Projects taken from the fleet in tries 4 and 5 were asked for again in try 6.
