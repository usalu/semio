# g1-m02 Round 2

round: 2

Result: three tries, all refused by the fleet wait (0 of 703 projects arrived each time), not ready. The memory wait of round 1 is gone: the graph plan was admitted at once with 2048 MB. No nx task ran. I ran 3 of the 6 allowed tries; round 3 was published meanwhile.

## Machine

Mac mini `Mac16,10`, Apple M4, 10 cores, 16 GB (17179869184 bytes), macOS 27.2.1 (26B131), 106 GiB free of 228 GiB on the data volume after the round.

## Before the Run (2026-11-14T15:59:21Z)

Replica code digest (`.cache/f/health.json`):
```
{'pid': 82534, 'code': '39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759', 'store': 'ok'}
```

`ps -axo pid,etime,rss,args | grep "service host"`: no `repo service host run` process exists (0 lines besides the grep itself; the host of last night, pid 33840, is gone). Nothing to stop, `repo service host stop` not run.

Last 5 lines of `.cache/f/sidecars/build-agent.log`:
```
2026-11-14T15:45:32.195Z worker registered: 4758b1b8-a805-4770-a9c7-fcad1be5cff5 cache http://192.168.178.70:6094
2026-11-14T15:46:10.076Z worker stopped: ueli-workstation-mac-mini-m4-16gb-256gb-01-wlan.fritz.box-82363
2026-11-14T15:46:44.614Z joining http://192.168.178.70:8981 offering rustc for 3 target(s)
2026-11-14T15:46:44.660Z worker sandbox: darwin-sandbox confined network denied
2026-11-14T15:46:44.683Z worker registered: 4cb5aefb-35c3-4719-8426-1b410e8648f5 cache http://192.168.178.70:6094
```

## Tries

| Try | Log | Start (UTC) | End (UTC) | `native_rc` | Graph slice counter at the end | Wanted | Arrived | Missing | Seconds waited for the fleet | This machine can infer |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `~/native-bootstrap-round-2.log` | 15:59:39 | 16:21:46 | 1 | 1065 s | 703 | 0 | 703 | 901 | 192 |
| 2 | `~/native-bootstrap-round-2-try-2.log` | 16:22:44 | 16:45:12 | 1 | 1083 s | 703 | 0 | 703 | 901 | 192 |
| 3 | `~/native-bootstrap-round-2-try-3.log` | 16:50:18 | 17:04:23 | 75 | 591 s | 703 | 0 | 703 | 450 | 192 |

- Try 2 started 32 s after try 1 ended, not after 5 minutes: see the first finding. Try 3 started 5 min 6 s after try 2.
- Hard stops: none. `kern.memorystatus_vm_pressure_level` was 1 in all 119 samples. Largest process of the run's own process group: 461408, 466144, 489776 KiB. Largest process on the machine: 4153616, 4251216, 3898608 KiB (the graph service and its workers are not in the run's process group; which process held the peak is in `round-2.samples.txt`).
- Wall time per phase, try 1: everything before `[semio] workspace:setup…` about 260 s (derived, not measured separately); graph slice 1065 s, of which 901 s waiting for the fleet. No task phase.
- Failed nx tasks: none, no task started.

## Findings

1. In tries 1 and 2 the refusal did not reach the bootstrap log. The log ended with:
   ```
   Reading the project graph slice (1065 s): flush 1 of 1.
   The resident service met an unexpected failure (reference 80ba70d1); try again, the details under that reference are in the service log /var/folders/1s/pfk8j2p90t94v1nx_gf8zq3w0000gn/T/semio-repo/501/host-38cdf415c7fb.log.
   end 2026-11-14T16:21:46Z
   native_rc=1
   ```
   No line with `fleet`, `wanted` or `graph-wanted` is in those two logs (only help text of `repo fleet status`), so the test of step 3 did not match and my runner stopped after try 1. I found the refusal in the service log, extended `run-round-2.py` to look the reference up there, and restarted it at try 2. Both tries carry the same reference `80ba70d1`.
2. In try 3 (code had changed in between) the refusal reaches the log with numbers and exit 75:
   ```
   Reading the project graph slice (591 s): flush 1 of 1.
   The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 703 and delivered 0 within 450 s, 703 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
   end 2026-11-14T17:04:23Z
   native_rc=75
   ```
3. The bootstrap log shows during the whole wait only `Reading the project graph slice (N s): flush 1 of 1.`; the `graph:` lines exist only in the service log.

## Fleet, `wanted`, `graph:` Lines, Verbatim

None in the bootstrap logs except the refusal of try 3 above. From the service log `host-38cdf415c7fb.log` (tries 1 and 2):

```
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (7678 MB available, 2048 MB floor)
graph: No verified change journal cursor is recorded for generation 17; verifying the tree once.
graph: 703 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 703
graph: waiting for the fleet: 0 of 703 projects arrived
graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 901 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T16:21:46.787Z request-failed 80ba70d1 RepoError: timeout graph-wanted.expired {"wanted":703,"arrived":0,"seconds":901,"missing":703,"local":192}
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (7731 MB available, 2048 MB floor)
2026-11-14T16:45:12.162Z request-failed 80ba70d1 RepoError: timeout graph-wanted.expired {"wanted":703,"arrived":0,"seconds":901,"missing":703,"local":192}
```

From `host-f74179f31e9f.log` (try 3):

```
[capacity] waiting 0s for graph-plan: not enough free memory (need 2048 MB; 2019 MB spare above the 2048 MB floor, load 9.48; 1 running, 0 ahead); behind workspace:setup (quiet, 1 job(s), 2048 MB)
[capacity] admitted graph-plan: 1 job(s), 2048 MB reserved (6123 MB available, 2048 MB floor)
graph: No verified change journal cursor is recorded for generation 17; verifying the tree once.
graph: 703 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 703
graph: waiting for the fleet: 0 of 703 projects arrived
graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 450 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
```

No line with `restored` or `Restarting with the current implementation`, and no exit 75 of a slice worker, in any of the logs.

## `[capacity]` Lines of the Bootstrap Logs, Verbatim

Try 1:
```
[capacity] run: sized at 9% of the default from 7 earlier run(s) of its class, at least 2289 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 3528 MB reserved (7035 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 87% of the default from 6 earlier run(s) of its class
[capacity] admitted dependencies with 5326 of 5370 MB: only 5326 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (8398 MB available, 2048 MB floor)
```
Try 2:
```
[capacity] run: sized at 9% of the default from 8 earlier run(s) of its class, at least 2294 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 3533 MB reserved (6089 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 87% of the default from 7 earlier run(s) of its class
[capacity] waiting 0s for dependencies: not enough free memory (need 5370 MB; 3058 MB spare above the 2048 MB floor, load 9.76; 0 running, 0 ahead); behind the host itself (nothing of ours)
[capacity] admitted dependencies with 4231 of 5370 MB: only 4231 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7706 MB available, 2048 MB floor)
```
Try 3:
```
[capacity] run: sized at 5% of the default from 9 earlier run(s) of its class, at least 2300 MB because of its assembly phase
[capacity] admitted run: 8 job(s), 2965 MB reserved (6759 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 6 earlier run(s) of its class
[capacity] dependencies: sized at 87% of the default from 8 earlier run(s) of its class
[capacity] admitted dependencies with 4673 of 5370 MB: only 4673 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6579 MB available, 2048 MB floor)
```

## Files

`round-2.tail.txt` (last 200 lines of try 3), `round-2.samples.txt` (30 s samples of all tries), `run-round-2.py`.
