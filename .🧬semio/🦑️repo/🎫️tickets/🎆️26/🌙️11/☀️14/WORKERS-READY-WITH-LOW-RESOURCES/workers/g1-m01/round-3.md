# g1-m01 Round 3

## Before The Run (2026-11-14T17:27:21Z)

Replica code digest (`.cache/f/health.json`): `39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759`

`ps -axo pid,ppid,etime,rss,args` for `show projects` and `service host` (non-ASCII path characters removed):
```
 1517     1       36:01 162976 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /Users/ueli/Documents/semio/M-pM^_M-'M-0M-oM-8M^Oframework/M-pM^_M^[M^MM-oM-8M^Oproducts/M-pM^_M-&M^QM-oM-8M^Orepo
 1519  1517       35:59 402608 /Users/ueli/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun --eval \012const fs=await import('node:fs'),path=await import('node:path'),url=await import('node:url'),crypto=awa
```
Last 8 lines of `.cache/f/sidecars/build-agent.log`:
```
2026-11-14T16:59:46.036Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T16:59:46.112Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T16:59:46.115Z worker registered: 0a50c9ce-a5cb-4c12-a88c-f90dba69b33d cache http://192.168.178.70:6094
2026-11-14T17:23:45.897Z agent code changed, restarting
2026-11-14T17:23:45.900Z worker stopped: ueli-workstation-mac-mini-m6-16gb-256gb-01-lan.fritz.box-6181
2026-11-14T17:23:48.250Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T17:23:48.300Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T17:23:48.303Z worker registered: 2c610894-7783-449d-b950-ce9b0d6c8904 cache http://192.168.178.70:6094
```

## Result (written 2026-11-14T18:51:26Z)

- Machine: Mac mini, 16 GiB memory, macOS 27.2.1, free disk 87Gi of 228Gi.
- Start 2026-11-14T17:27:38Z, end 2026-11-14T18:50:24Z. Five tries; the round ended at try 5 because it failed with something that is not a fleet refusal.
- Final `native_rc=1`; `.cache/f/native-bootstrap.done` absent; not ready.
- `workspace:setup` never got past the graph step, so no nx task ran and no task failed: failed tasks = none.
- Memory pressure level: 1 in all 141 samples. No hard stop.
- Peak resident size of a process of the run: 3 289 440 KiB. Largest process on the machine: 3 526 896 KiB, pid 30181, ppid 1519 (child of the graph service), `bun -- …/repo/…` (full lines in `round-3.samples.txt`).
- A `git` process above 1 GiB was among the three largest in 117 of 141 samples, 1 108 448 to 2 664 224 KiB. It is not part of the run: every chain is `git … diff --ignore-submodules=dirty --submodule=short --no-textconv --numstat -z -M <commit>` (read-only) ← `Claude Helper` (node service, pid 63608) ← `/Applications/Claude.app/Contents/MacOS/Claude` (pid 63576) ← 1, that is the desktop app that hosts this agent session, which re-runs the diff of the worktree against a commit about every half minute. Chains verbatim (cut at 300 characters) in `round-3.samples.txt`; subcommand read from a live `ps` at 18:51Z. It takes up to 2.7 GiB of the 16 GiB while the bootstrap runs.

## Tries

| try | start (UTC) | end (UTC) | native_rc | wanted | arrived | missing | seconds | local |
|---|---|---|---|---|---|---|---|---|
| 1 | 17:27:38 | 17:35:50 | 75 | 509 | 0 | 509 | 300 | 192 |
| 2 | 17:39:10 | 17:46:32 | 75 | 509 | 0 | 509 | 301 | 192 |
| 3 | 17:49:41 | 18:18:55 | 75 | 509 | 207 | 302 | 1497 | 192 |
| 4 | 18:22:18 | 18:43:53 | 75 | 302 | 32 | 270 | 1020 | 192 |
| 5 | 18:46:54 | 18:49:56 | 1 | not reached | not reached | not reached | 3 | – |

Waits between tries: 180 s each.

First and last "arrived" line per try (service log, verbatim; the line repeats unchanged in between):
- Try 1: first and last `graph: waiting for the fleet: 0 of 509 projects arrived`
- Try 2: first and last `graph: waiting for the fleet: 0 of 509 projects arrived`
- Try 3: first `graph: waiting for the fleet: 0 of 509 projects arrived`, then 16, 64, 112, 160 (steps of 48), last `graph: waiting for the fleet: 207 of 509 projects arrived`
- Try 4: first `graph: waiting for the fleet: 0 of 302 projects arrived`, last `graph: waiting for the fleet: 32 of 302 projects arrived`
- Try 5: none

Try 4 kept the 207 projects of try 3 (`207 taken from the fleet`) and asked only for the remaining 302.

## Try 5: macOS Denial Right After a Service Restart

The graph service was replaced twice by new code during the round (`shutdown superseded` at 18:26:02Z and 18:50:51Z). The first request to the service that started about 18:49Z failed:
```
2026-11-14T18:49:56.026Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
macOS denied the resident service access to your Documents folder (reference e07192d7). The service runs without the file access consent of this session. Allow `repo` under System Settings, Privacy & Security, Files and Folders (or Full Disk Access), run `repo service restart`, or keep the checkout outside Documents, Desktop and Downloads. Details: /var/folders/gd/byw98s3s0p95zhdpzyv769740000gn/T/semio-repo/501/host-a561d8014168.log.
```
Same reference and same error as try 5 of round 2 (17:26:01Z), which was also the first request to a freshly started service. Tries 1 to 4 of this round ran against the service that had failed that way before and it then read the Documents folder without error, so the denial did not persist for that process.

## Lines Mentioning fleet, wanted, arrived, serving, restored, Restarting, adopted

Service logs (`host-f74179f31e9f.log`, `host-6feebb05efe3.log`, `host-a561d8014168.log`), consecutive repeats removed, prefix = line number:
```
# host-f74179f31e9f.log
934:graph: 509 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 509
935:graph: waiting for the fleet: 0 of 509 projects arrived
1093:2026-11-14T17:04:39.563Z graph-wanted-expired {"wanted":"509","arrived":"0","seconds":"676","missing":"509","local":"192"}
1711:graph: 509 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 509
1712:graph: waiting for the fleet: 0 of 509 projects arrived
1783:2026-11-14T17:17:52.623Z graph-wanted-expired {"wanted":"509","arrived":"0","seconds":"300","missing":"509","local":"192"}
1855:2026-11-14T17:26:01.371Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
2460:graph: 509 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 509
2461:graph: waiting for the fleet: 0 of 509 projects arrived
2532:2026-11-14T17:35:50.181Z graph-wanted-expired {"wanted":"509","arrived":"0","seconds":"300","missing":"509","local":"192"}
3096:graph: 509 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 509
3097:graph: waiting for the fleet: 0 of 509 projects arrived
3168:2026-11-14T17:46:32.247Z graph-wanted-expired {"wanted":"509","arrived":"0","seconds":"301","missing":"509","local":"192"}
3991:graph: 509 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 509
3992:graph: waiting for the fleet: 0 of 509 projects arrived
4057:graph: waiting for the fleet: 16 of 509 projects arrived
4081:graph: waiting for the fleet: 64 of 509 projects arrived
4111:graph: waiting for the fleet: 112 of 509 projects arrived
4146:graph: waiting for the fleet: 160 of 509 projects arrived
4192:graph: waiting for the fleet: 207 of 509 projects arrived
4344:2026-11-14T18:18:55.062Z graph-wanted-expired {"wanted":"509","arrived":"207","seconds":"1497","missing":"302","local":"192"}
4401:2026-11-14T18:26:02.779Z shutdown superseded
# host-6feebb05efe3.log
932:graph: 509 stale projects, 207 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 302
933:graph: waiting for the fleet: 0 of 302 projects arrived
961:graph: waiting for the fleet: 32 of 302 projects arrived
1172:2026-11-14T18:43:53.668Z graph-wanted-expired {"wanted":"302","arrived":"32","seconds":"1020","missing":"270","local":"192"}
1229:2026-11-14T18:50:51.157Z shutdown superseded
# host-a561d8014168.log
143:2026-11-14T18:49:56.026Z request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)
```
(The first file also holds round 2 lines before line 2400; they are in `round-2.md`.) No line with `serving`, `restored`, `Restarting` or `adopted` exists in these logs for the round.

Bootstrap logs, the refusal line of each try verbatim:
```
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 509 and delivered 0 within 300 s, 509 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 509 and delivered 0 within 301 s, 509 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 509 and delivered 207 within 1497 s, 302 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 302 and delivered 32 within 1020 s, 270 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

## `[capacity]` Lines (verbatim, distinct, per try)

Try 1:
```
[capacity] run: sized at 5% of the default from 13 earlier run(s) of its class, at least 2926 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3633 MB reserved (6817 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 13 earlier run(s) of its class
[capacity] dependencies: sized at 67% of the default from 13 earlier run(s) of its class, at least 4670 MB because of its plan phase
[capacity] admitted dependencies with 3990 of 4670 MB: only 3990 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] waiting 0s for run: not enough free memory (need 2048 MB; 1688 MB spare above the 2048 MB floor, load 9.23; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted run: 1 job(s), 2048 MB reserved (6311 MB available, 2048 MB floor)
```

Try 2:
```
[capacity] run: sized at 5% of the default from 14 earlier run(s) of its class, at least 2927 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3599 MB reserved (6573 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 13 earlier run(s) of its class
[capacity] dependencies: sized at 67% of the default from 13 earlier run(s) of its class, at least 4673 MB because of its plan phase
[capacity] waiting 0s for dependencies: not enough free memory (need 4673 MB; 2922 MB spare above the 2048 MB floor, load 7.26; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted dependencies with 4311 of 4673 MB: only 4311 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (5868 MB available, 2048 MB floor)
```

Try 3:
```
[capacity] run: sized at 5% of the default from 14 earlier run(s) of its class, at least 2928 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3600 MB reserved (6686 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 13 earlier run(s) of its class
[capacity] dependencies: sized at 67% of the default from 14 earlier run(s) of its class, at least 4676 MB because of its plan phase
[capacity] admitted dependencies with 3486 of 4676 MB: only 3486 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6729 MB available, 2048 MB floor)
```

Try 4:
```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class, at least 2932 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3604 MB reserved (6037 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 13 earlier run(s) of its class
[capacity] dependencies: sized at 67% of the default from 14 earlier run(s) of its class, at least 4683 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 4683 MB reserved (6827 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6318 MB available, 2048 MB floor)
```

Try 5:
```
[capacity] run: sized at 5% of the default from 15 earlier run(s) of its class, at least 2934 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3606 MB reserved (7153 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 13 earlier run(s) of its class
[capacity] dependencies: sized at 67% of the default from 15 earlier run(s) of its class, at least 4689 MB because of its plan phase
[capacity] admitted dependencies with 4289 of 4689 MB: only 4289 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6730 MB available, 2048 MB floor)
```

## Wall Time Per Phase

| try | before the graph step | graph step (wait for the fleet) | total |
|---|---|---|---|
| 1 | about 3 min | 330 s | 8 min 12 s |
| 2 | about 2 min | 318 s | 7 min 22 s |
| 3 | about 3 min | 1583 s | 29 min 14 s |
| 4 | about 3 min | 1131 s | 21 min 35 s |
| 5 | about 3 min | 3 s | 3 min 2 s |

"Before the graph step" is total minus the graph step; it was not measured separately.

## Files

- `round-3.samples.txt`: 30 s samples (three largest processes with pid, ppid and full command; pressure level; peak of the run).
- `round-3.tail.txt`: last 200 lines of the try 5 log (control lines of the script protocol removed).
- Logs on this machine: `~/native-bootstrap-round-3.log`, `~/native-bootstrap-round-3-try-{2,3,4,5}.log`.
