# g1-m04 Round 10 (covers round 9)

Machine: `ueli-workstation-lin-1`, SteamOS (Linux 7.2.7-valve2-2-neptune-72-g31504582a073), 15848 MB memory, 25331 MB swap, 146 GiB free on `/home`. Runner: `run-tries-10.sh`. Linux: no consent, no exit 4, no `fstatat failed (EPERM)`.

## Result

`native_rc=75` in all 10 tries, not ready; no setup task ran. **Wanted 517, arrived 0 in every try**: 46 tries in a row since 2026-11-14 16:51 UTC. No hard stop; largest process 1940648 KiB; PSI memory `full avg10` maximum 1.38 %. No window in which the fleet was unreachable was seen (no store or cache timeout in the logs). **Read `BLOCKER.md` in this folder**: the cause is the platform of the request, not this round's fixes.

| Try | Start (UTC) | End (UTC) | Seconds | Wanted | Arrived | Missing | Waited | Peak RSS (KiB) |
|---|---|---|---|---|---|---|---|---|
| 1 | 23:40:05Z | 23:48:42Z | 517 | 517 | 0 | 517 | within 300 s | 1548908 |
| 2 | 23:51:06Z | 00:00:58Z | 592 | 517 | 0 | 517 | within 301 s | 1774872 |
| 3 | 00:03:07Z | 00:11:46Z | 519 | 517 | 0 | 517 | within 300 s | 1470296 |
| 4 | 00:14:08Z | 00:22:40Z | 512 | 517 | 0 | 517 | within 301 s | 1940648 |
| 5 | 00:25:09Z | 00:33:41Z | 512 | 517 | 0 | 517 | within 300 s | 1938136 |
| 6 | 00:36:10Z | 00:44:49Z | 519 | 517 | 0 | 517 | within 300 s | 1449868 |
| 7 | 00:47:11Z | 00:55:46Z | 515 | 517 | 0 | 517 | within 302 s | 1884304 |
| 8 | 00:58:12Z | 01:06:54Z | 522 | 517 | 0 | 517 | within 300 s | 1295624 |
| 9 | 01:09:13Z | 01:18:02Z | 529 | 517 | 0 | 517 | within 301 s | 1824328 |
| 10 | 01:20:14Z | 01:29:06Z | 532 | 517 | 0 | 517 | within 300 s | 1439628 |

Refusal of try 10, verbatim (with the names of the too heavy projects):

```
The project graph could not be read on this machine: it can infer only 0 of the stale projects, the fleet was asked for 517 and delivered 0 within 300 s, 517 are still missing, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 511 more (this machine can give about 0 MB per project). Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

## Before the Run, Replica Agent, Launcher

```
pre-run facts, taken 2026-11-14T23:40:05Z
replica agent: running, started by me 0 times; health.json age 4 s
replica code digest: "code":"eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc"; "bootVersion":6
launcher: 1771439 -rwxr-xr-x 1 deck deck 20516143 2026-11-14 21:46:46.384754297 +0100 /home/deck/.local/bin/repo
repo service host processes (pid ppid etime rss):
2937655    1977       29:42 143724 /home
last 8 lines of build-agent.log:
2026-11-14T23:09:48.060Z worker sandbox: linux-sandbox confined network denied
2026-11-14T23:09:48.066Z worker registered: 0f23375a-b90d-4786-9d88-f30674a6a5c7 cache http://192.168.178.70:6094
2026-11-14T23:14:43.138Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T23:19:43.327Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T23:24:43.586Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T23:29:43.837Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T23:34:44.075Z graph-server: alive, 0 machines seen, 0 requests held, read none
2026-11-14T23:39:44.337Z graph-server: alive, 0 machines seen, 0 requests held, read none
```

Launcher after the round: `1771439 -rwxr-xr-x 1 deck deck 20516143 2026-11-14 21:46:46.384754297 +0100 /home/deck/.local/bin/repo`: inode and mtime unchanged over the ten runs. Replica digest now: "code":"eabbc92267329dbd39d8d05e0c8eceb1bcad083c0eea58c44805408a7d547bcc"
"code":"52e46eee5c577cf8425b43f243681f24326d6645807f938982368d8c64cbfc7f".

## Graph Lines (graph service log, try 10; all tries in `round-10[-try-k].service.txt`)

```
graph: 517 stale projects, 0 taken from the fleet, 517 too heavy for this machine (@semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 511 more; it can give about 0 MB per project), it can infer 0 of the others (0 per wave); asking the fleet for 517
graph: nothing to wait for any longer: the fleet was asked for 517 stale projects and delivered 0 within 300 s; 517 remain, among them the too heavy project(s) @semio-tech/s-dev-composition (about 27952 MB), @semio-tech/repo-lib (about 9652 MB), @semio-tech/framework-os-dev (about 7676 MB), @semio-tech/framework-os-kernel (about 7676 MB), @semio-tech/framework-rs (about 6518 MB), workspace (about 6200 MB), and 511 more, which this machine must never infer itself (it can give about 0 MB per project); try again when a machine with free memory serves the graph, or read the graph on a larger machi
graph: waiting for the fleet: 0 of 517 projects arrived
```

## Sidecar, Last Lines

```
2026-11-15T01:29:30.778Z worker reconnecting: Unable to connect. Is the computer able to access the url?
2026-11-15T01:29:31.809Z worker reconnecting: Unable to connect. Is the computer able to access the url?
2026-11-15T01:29:32.829Z worker reconnecting: Unable to connect. Is the computer able to access the url?
2026-11-15T01:29:33.866Z worker reconnecting: Unable to connect. Is the computer able to access the url?
2026-11-15T01:29:34.885Z worker reconnecting: Unable to connect. Is the computer able to access the url?
2026-11-15T01:29:35.918Z worker reconnecting: Unable to connect. Is the computer able to access the url?
2026-11-15T01:29:36.953Z worker reconnecting: Unable to connect. Is the computer able to access the url?
2026-11-15T01:29:37.969Z worker reconnecting: Unable to connect. Is the computer able to access the url?
```

Wall time per phase: not measured.
