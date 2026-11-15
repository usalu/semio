# g1-m04 Round 8

Machine: `ueli-workstation-lin-1`, SteamOS (Linux 7.2.7-valve2-2-neptune-72-g31504582a073), 15848 MB memory, 25331 MB swap, 146 GiB free on `/home`. Runner: `run-tries-8.sh`. Linux: no consent, no exit 4, no codesign.

## Result

`native_rc=75` in all 10 tries, not ready; `workspace:setup` never got past the graph step, so no setup task ran. **Wanted 516 (517 from try 8), arrived 0 in every try**: 36 tries in a row since 2026-11-14 16:51 UTC. No hard stop; largest process 1945936 KiB; PSI memory `full avg10` maximum 2.59 %. No `source-differs`, `closure.moving`, `restored`, `closureRepaired` or `inferred here` line.

The cause I read in the code is in the addendum of `round-7.md`: a serving machine reads requests of its own platform only, and this is the group's Linux machine. Nothing in rounds 8 text addresses it, and the measurements did not change.

| Try | Start (UTC) | End (UTC) | Seconds | Wanted | Arrived | Missing | Waited | Peak RSS (KiB) |
|---|---|---|---|---|---|---|---|---|
| 1 | 21:49:34Z | 21:58:07Z | 513 | 516 | 0 | 516 | within 300 s | 1945936 |
| 2 | 22:00:35Z | 22:09:44Z | 549 | 516 | 0 | 516 | within 300 s | 1652256 |
| 3 | 22:12:06Z | 22:20:46Z | 520 | 516 | 0 | 516 | within 301 s | 1471580 |
| 4 | 22:23:07Z | 22:32:01Z | 534 | 516 | 0 | 516 | within 301 s | 1723744 |
| 5 | 22:34:08Z | 22:43:02Z | 534 | 516 | 0 | 516 | within 300 s | 1457316 |
| 6 | 22:45:09Z | 22:53:56Z | 527 | 516 | 0 | 516 | within 301 s | 1747036 |
| 7 | 22:56:10Z | 23:04:53Z | 523 | 516 | 0 | 516 | within 301 s | 1374224 |
| 8 | 23:07:11Z | 23:17:08Z | 597 | 517 | 0 | 517 | within 300 s | 1723976 |
| 9 | 23:19:12Z | 23:27:54Z | 522 | 517 | 0 | 517 | within 300 s | 1437920 |
| 10 | 23:30:13Z | 23:38:52Z | 519 | 517 | 0 | 517 | within 300 s | 1712668 |

Refusal of try 10, verbatim:

```
The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 517 and delivered 0 within 300 s, 517 are still missing, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 511 more (this machine can give about 0 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

## Before the Run, Replica Agent, Launcher

```
pre-run facts, taken 2026-11-14T21:49:34Z
replica agent: running, not restarted by me (0 times); .cache/f/health.json age 1 s
replica code digest: "code":"590fdef8b75853d124d6f615cd2612d607146e27aaf2b94715a5d7b7606ad2f1"; "bootVersion":6
launcher: 1771439 -rwxr-xr-x 1 deck deck 20516143 2026-11-14 21:46:46.384754297 +0100 /home/deck/.local/bin/repo
repo service host processes (pid ppid etime rss):
2748475    1977       14:12 143624 /home
```

Replica agent started by me: 0 times (it was running). Launcher after the round: `1771439 -rwxr-xr-x 1 deck deck 20516143 2026-11-14 21:46:46.384754297 +0100 /home/deck/.local/bin/repo` (before: see above; say plainly: inode and mtime unchanged over the ten runs).

## Graph Lines (graph service log, try 1 and try 10; all tries in `round-8[-try-k].service.txt`)

```
graph: 516 stale projects, 0 taken from the fleet, 516 too heavy for this machine (@semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 510 more; it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 516
graph: nothing to wait for any longer: the fleet was asked for 516 stale projects and delivered 0 within 300 s; 516 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 510 more, which this machine must never infer itself (it can give about 0 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machi
graph: waiting for the fleet: 0 of 516 projects arrived
graph: 517 stale projects, 0 taken from the fleet, 517 too heavy for this machine (@semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 511 more; it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 517
graph: nothing to wait for any longer: the fleet was asked for 517 stale projects and delivered 0 within 300 s; 517 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 511 more, which this machine must never infer itself (it can give about 0 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machi
graph: waiting for the fleet: 0 of 517 projects arrived
```

## Sidecar, Last Lines

```
2026-11-14T23:09:48.060Z worker sandbox: linux-sandbox confined network denied
2026-11-14T23:09:48.066Z worker registered: 0f23375a-b90d-4786-9d88-f30674a6a5c7 cache http://192.168.178.70:6094
2026-11-14T23:14:43.138Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T23:19:43.327Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T23:24:43.586Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T23:29:43.837Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T23:34:44.075Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T23:39:44.337Z graph-server: alive, 0 machines seen, 0 requests held, read none
```

## `[capacity]` Lines (try 10)

```
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 13843 MB because of its plan phase
[capacity] admitted run: 8 job(s), 7254 MB reserved (9766 MB available, 1981 MB floor)
[capacity] run: sized at 5% of the default from 16 earlier run(s) of its class, at least 13843 MB because of its plan phase
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class, at least 13843 MB because of its plan phase
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class, at least 13843 MB because of its plan phase
[capacity] dependencies: sized at 36% of the default from 16 earlier run(s) of its class, at least 13916 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 6933 MB reserved (9423 MB available, 1981 MB floor)
[capacity] dependencies: sized at 36% of the default from 16 earlier run(s) of its class, at least 13916 MB because of its plan phase
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (10267 MB available, 1981 MB floor)
```

Wall time per phase: not measured.
