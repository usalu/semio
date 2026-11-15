# g1-m04 Round 7 (covers round 6; logs `…-round-7…`)

Machine: `ueli-workstation-lin-1`, SteamOS (Linux 7.2.7-valve2-2-neptune-72-g31504582a073), 15848 MB memory, 25331 MB swap, 146 GiB free on `/home`. Runner: `run-tries-7.sh`. Linux: the macOS checks of `ROUND-NOTE-P6.md` do not apply; no exit 4, no ENOENT of `build-input-closures`, no `source-differs`, no `closureRepaired`, no `restored`.

## Result

`native_rc=75` in all 8 tries, not ready, `workspace:setup` never got past the graph step, so there is no list of setup tasks. **Wanted 516, arrived 0, missing 516 in every try**: 26 tries in a row since 16:51 UTC. No hard stop; largest process 1906360 KiB; PSI memory `full avg10` maximum 4.12 %.

New in tries 7 and 8: the refusal names the too heavy projects, and try 7 was refused after **0 s** of waiting.

| Try | Start (UTC) | End (UTC) | Seconds | Wanted | Arrived | Missing | Waited | Peak RSS (KiB) |
|---|---|---|---|---|---|---|---|---|
| 1 | 20:22:57Z | 20:32:53Z | 596 | 516 | 0 | 516 | within 302 s | 1793884 |
| 2 | 20:34:59Z | 20:44:09Z | 550 | 516 | 0 | 516 | within 301 s | 1805744 |
| 3 | 20:46:30Z | 20:55:12Z | 522 | 516 | 0 | 516 | within 301 s | 1481856 |
| 4 | 20:57:31Z | 21:07:49Z | 618 | 516 | 0 | 516 | within 301 s | 1906360 |
| 5 | 21:10:02Z | 21:18:42Z | 520 | 516 | 0 | 516 | within 300 s | 1560416 |
| 6 | 21:21:03Z | 21:29:46Z | 523 | 516 | 0 | 516 | within 300 s | 1274480 |
| 7 | 21:32:04Z | 21:36:53Z | 289 | 516 | 0 | 516 | within 0 s | 1650604 |
| 8 | 21:39:05Z | 21:47:44Z | 519 | 516 | 0 | 516 | within 300 s | 1475428 |

Refusal of try 1, verbatim:

```
The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 516 and delivered 0 within 302 s, 516 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

Refusal of try 7 (0 s), verbatim:

```
The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 516 and delivered 0 within 0 s, 516 are still missing, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 510 more (this machine can give about 0 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

Refusal of try 8, verbatim:

```
The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 516 and delivered 0 within 300 s, 516 are still missing, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 510 more (this machine can give about 0 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

## Before the Run

```
pre-run facts, taken 2026-11-14T20:22:57Z
replica code digest: "code":"d72ff30ebb826bfabac6ee5f126041ee5f833672897443b9666c557ce85335c1"
repo service host processes (pid ppid etime rss command):
2588030    1977       08:21 145440 … 📜️script.ts repo service host run
(end of list)
last 8 lines of .cache/f/sidecars/build-agent.log:
2026-11-14T20:10:33.242Z worker registered: 68bc763a-15f7-4075-b84f-ab50f9b4835a cache http://192.168.178.70:6094
2026-11-14T20:15:28.279Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T20:20:28.491Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T20:20:33.063Z agent code changed, restarting
2026-11-14T20:20:33.069Z worker stopped: ueli-workstation-lin-1-2579635
2026-11-14T20:20:33.838Z joining http://192.168.178.70:8981 offering rustc for 4 target(s)
2026-11-14T20:20:33.876Z worker sandbox: linux-sandbox confined network denied
2026-11-14T20:20:33.884Z worker registered: 9e6e5e65-ef29-45bc-915f-7c2897647d03 cache http://192.168.178.70:6094
```

## Graph Lines per Try (graph service log; files `round-7[-try-k].service.txt` hold them with the last 30 lines)

```
try 1:
graph: 516 stale projects, 0 taken from the fleet, this machine can infer 0 (0 per wave); asking the fleet for 516
graph: nothing to wait for any longer: asked the fleet for 516 stale projects and received 0 within 302 s, 516 remain and this machine can infer only 0; try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: waiting for the fleet: 0 of 516 projects arrived
try 2:
graph: 516 stale projects, 0 taken from the fleet, this machine can infer 0 (0 per wave); asking the fleet for 516
graph: nothing to wait for any longer: asked the fleet for 516 stale projects and received 0 within 301 s, 516 remain and this machine can infer only 0; try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: waiting for the fleet: 0 of 516 projects arrived
try 3:
graph: 516 stale projects, 0 taken from the fleet, this machine can infer 0 (0 per wave); asking the fleet for 516
graph: nothing to wait for any longer: asked the fleet for 516 stale projects and received 0 within 301 s, 516 remain and this machine can infer only 0; try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: waiting for the fleet: 0 of 516 projects arrived
try 4:
graph: 516 stale projects, 0 taken from the fleet, this machine can infer 0 (0 per wave); asking the fleet for 516
graph: nothing to wait for any longer: asked the fleet for 516 stale projects and received 0 within 301 s, 516 remain and this machine can infer only 0; try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: waiting for the fleet: 0 of 516 projects arrived
try 5:
graph: 516 stale projects, 0 taken from the fleet, this machine can infer 0 (0 per wave); asking the fleet for 516
graph: nothing to wait for any longer: asked the fleet for 516 stale projects and received 0 within 300 s, 516 remain and this machine can infer only 0; try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: waiting for the fleet: 0 of 516 projects arrived
try 6:
graph: 516 stale projects, 0 taken from the fleet, this machine can infer 0 (0 per wave); asking the fleet for 516
graph: nothing to wait for any longer: asked the fleet for 516 stale projects and received 0 within 300 s, 516 remain and this machine can infer only 0; try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: waiting for the fleet: 0 of 516 projects arrived
try 7:
graph: 516 stale projects, 0 taken from the fleet, 516 too heavy for this machine (@semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 510 more; it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 516
graph: 516 stale projects, 0 taken from the fleet, this machine can infer 0 (0 per wave); asking the fleet for 516
graph: nothing to wait for any longer: the fleet was asked for 516 stale projects and delivered 0 within 0 s; 516 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 510 more, which this machine must never infer itself (it can give about 0 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machine
graph: the fleet cache cannot be reached, so 516 stale projects cannot be requested from it; this machine can infer 0
graph: waiting for the fleet: 0 of 516 projects arrived
try 8:
graph: 516 stale projects, 0 taken from the fleet, 516 too heavy for this machine (@semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 510 more; it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 516
graph: nothing to wait for any longer: the fleet was asked for 516 stale projects and delivered 0 within 300 s; 516 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 510 more, which this machine must never infer itself (it can give about 0 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machi
graph: waiting for the fleet: 0 of 516 projects arrived
```

## Sidecar of This Machine, Last Lines

```
2026-11-14T21:36:48.310Z worker reconnecting: Unable to connect. Is the computer able to access the url?
2026-11-14T21:36:50.563Z load report sent: reachable again
2026-11-14T21:36:53.315Z worker registered: 57f13384-0fb1-46ca-aa6e-42d3ac354e61 cache http://192.168.178.70:6094
2026-11-14T21:37:35.290Z agent code changed, restarting
2026-11-14T21:37:35.295Z worker stopped: ueli-workstation-lin-1-2751020
2026-11-14T21:37:36.078Z joining http://192.168.178.70:8981 offering rustc for 4 target(s)
2026-11-14T21:37:36.111Z worker sandbox: linux-sandbox confined network denied
2026-11-14T21:37:36.121Z worker registered: 47b7f18c-496a-4f98-9771-1dd29986188f cache http://192.168.178.70:6094
2026-11-14T21:42:31.099Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T21:47:31.352Z graph-server: alive, 0 machines seen, 0 requests held, read none
```

## `[capacity]` Lines (try 8)

```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 14028 MB because of its plan phase
[capacity] admitted run: 8 job(s), 7247 MB reserved (9849 MB available, 1981 MB floor)
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 14028 MB because of its plan phase
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class, at least 14028 MB because of its plan phase
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class, at least 14028 MB because of its plan phase
[capacity] dependencies: sized at 40% of the default from 16 earlier run(s) of its class, at least 14056 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 6933 MB reserved (9405 MB available, 1981 MB floor)
[capacity] dependencies: sized at 40% of the default from 16 earlier run(s) of its class, at least 14056 MB because of its plan phase
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (10187 MB available, 1981 MB floor)
```

Wall time per phase: not measured.

## For the Coordinator

Unchanged since round 3: this machine's requests are not answered (0 of 516 in 26 tries, each wait ending at the silence limit), while it cannot infer any of them itself ("can infer 0"). Replica code digest changed during round 5 to `d72ff30ebb82…`. I continue with round 8.

## Addendum: Probable Cause of "0 of 516" (read in the code, not changed)

`📚️library/🏗️build/🪪️graph-state/🙋️wanted/🟦️.ts`: a request lives in one slot **per requester and platform** (`wantedStateV1(requester, platform)`), and a serving machine reads the requests of the other machines **with its own platform** (`claimWantedV1` → `readWantedV1({client, requester, platform: input.platform})`, called from `🛠️server/🟦️.ts` line 117) and only for its own `graphKey`. g1-m04 is, as far as the worker folders show, the only Linux machine of the group; the group server and the other workers are macOS. So no machine ever looks into the slot where g1-m04 publishes, which fits what is measured: 0 arrived in 26 tries, every wait ends at the silence limit, the sidecar here says `0 machines seen, 0 requests held`, and this machine asks for 516 projects of graph `d09b06e73429` while the Macs ask for 703 of theirs. If project records are platform-specific on purpose, a Linux worker needs a Linux machine with enough memory to serve it (or a way to infer its 516 projects in bounded waves itself: `this machine can infer 0 (0 per wave)`); if they are not, the server could read requests of every platform. That decision is the coordinator's; I change nothing.
