# g1-m01 Round 2

## Before The Run (2026-11-14T15:59:35Z)

Replica code digest (`.cache/f/health.json`):
```
"code":"39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759"
```
`ps -axo pid,etime,rss,args | grep "service host"`:
```
```
Last 5 lines of `.cache/f/sidecars/build-agent.log`:
```
2026-11-14T12:40:55.474Z worker finished: c04275c7-9726-4387-9837-c08fe0e7e925.1 completed 0
2026-11-14T12:41:22.581Z worker started: a79c37b0-df90-4ee1-a810-2b4e5761883a.1 892c3eca3e38
2026-11-14T12:41:24.307Z worker finished: a79c37b0-df90-4ee1-a810-2b4e5761883a.1 completed 0
2026-11-14T12:41:32.054Z worker started: cc8ced67-70a5-4afb-89c7-151ddae82585.2 a4bc5bc2e18f
2026-11-14T12:41:32.373Z worker finished: cc8ced67-70a5-4afb-89c7-151ddae82585.2 failed 9
```

No `service host` process existed before the run (the output above is empty), so nothing was stopped. The bootstrap started the graph service itself in try 1 (`Starting the repo graph service for this host.`) and again in try 3.

## Result

- Machine: Mac mini, 16 GiB memory, macOS 27.2.1; 87 GiB free on the data volume after the round.
- Replica code digest after the round: `39e3688eb315` (unchanged).
- Five tries, `2026-11-14T15:59:51Z` to `2026-11-14T17:26:01Z`. Not ready: marker `.cache/f/native-bootstrap.done` absent. No hard stop in any try.
- Tries 1 to 4 ended with the fleet refusal. Try 5 ended with a different failure (macOS file access), so the round stopped there (step 3 repeats only after a fleet refusal); a sixth try was not run.
- `workspace:setup` never got past the graph step: no nx task ran, so there is no failed task to list.

| Try | Start (UTC) | End (UTC) | `native_rc` | Seconds waited for the fleet | Wanted | Arrived | Missing | Can infer here | Peak resident of the run |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 15:59:51 | 16:18:03 | 1 | 901 | 509 | 0 | 509 | 192 | 3 249 488 KiB |
| 2 | 16:23:05 | 16:43:00 | 1 | 900 | 509 | 0 | 509 | 192 | 4 108 144 KiB |
| 3 | 16:48:09 | 17:04:39 | 75 | 676 | 509 | 0 | 509 | 192 | 3 582 832 KiB |
| 4 | 17:09:44 | 17:17:52 | 75 | 300 | 509 | 0 | 509 | 192 | 2 679 296 KiB |
| 5 | 17:23:16 | 17:26:01 | 1 | none (failed 2 s into the graph step) | not measured | not measured | not measured | not measured | 3 376 560 KiB |

Memory pressure: `kern.memorystatus_vm_pressure_level` 1 in all 133 samples. Largest process of any kind seen: 4 108 144 KiB (try 2).

## Findings For The Coordinator

1. **The fleet delivered 0 of 509 projects in four tries** (waits 901, 900, 676 and 300 s; the wait got shorter from try to try, not measured why).
2. **In tries 1 and 2 the refusal was not visible in the bootstrap log and the exit was 1, not 75.** The log ends with `The resident service met an unexpected failure (reference 876d69f7); try again, the details under that reference are in the service log …` (try 2: reference `74c82088`). Only the service log names it: `request-failed 876d69f7 RepoError: timeout graph-wanted.expired {"wanted":509,"arrived":0,"seconds":901,"missing":509,"local":192}`. Tries 3 and 4, after the graph service had been restarted by the bootstrap (new service log `host-f74179f31e9f.log`), printed the designed text and `native_rc=75`. So the older resident service ran code without the designed refusal; a service that keeps running across a code change answers with the old behaviour.
3. **The lines `graph: …` are only in the service log**, not in the bootstrap log (0 lines starting with `graph:` in all five logs). The detection rule of `ROUND.md` step 3 (`graph-wanted` or a fleet line in the log) therefore matched nothing in tries 1 and 2; my runner stopped after try 1 and again after try 3 until I taught it the service log and the wording of try 3. The five-minute waits were kept (see table).
4. **Try 5 failed on macOS file access**, 2 s into the graph step: `macOS denied the resident service access to your Documents folder (reference e07192d7). The service runs without the file access consent of this session. Allow \`repo\` under System Settings, Privacy & Security, Files and Folders (or Full Disk Access), run \`repo service restart\`, or keep the checkout outside Documents, Desktop and Downloads.` Service log: `request-failed e07192d7 Error: Relative filesystem fstatat failed (EPERM)` at `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:148`. The same service (pids 1517/1519, started 16:51Z by try 3) had read the tree in tries 3 and 4. Nothing was changed: a consent needs the user, and restarting the service is not among this round's instructions. The service is still running.

## Fleet Lines, Verbatim

Bootstrap logs (tries 3 and 4, the only lines mentioning the fleet apart from the help text):
```
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 509 and delivered 0 within 676 s, 509 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
The project graph could not be read on this machine: it can infer only 192 of the stale projects, the fleet was asked for 509 and delivered 0 within 300 s, 509 are still missing. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```
Service logs (`$TMPDIR/semio-repo/501/host-38cdf415c7fb.log` for tries 1 and 2, `host-f74179f31e9f.log` for tries 3 to 5):
```
graph: No verified change journal cursor is recorded for generation 28; verifying the tree once.
graph: 509 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 509
graph: waiting for the fleet: 0 of 509 projects arrived        (repeated, 246 lines in all, always 0 of 509)
graph: nothing to wait for any longer: asked the fleet for 509 stale projects and received 0 within 901 s, 509 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T16:18:03.425Z request-failed 876d69f7 RepoError: timeout graph-wanted.expired {"wanted":509,"arrived":0,"seconds":901,"missing":509,"local":192}
2026-11-14T16:43:00.656Z request-failed 74c82088 RepoError: timeout graph-wanted.expired {"wanted":509,"arrived":0,"seconds":900,"missing":509,"local":192}
2026-11-14T17:04:39.563Z graph-wanted-expired {"wanted":"509","arrived":"0","seconds":"676","missing":"509","local":"192"}
2026-11-14T17:17:52.623Z graph-wanted-expired {"wanted":"509","arrived":"0","seconds":"300","missing":"509","local":"192"}
```
`restored`, `Restarting with the current implementation`, `Inferring projects`: no line in any of the five logs.

## `[capacity]` Lines, Verbatim

Try 1:
```
[capacity] run: sized at 5% of the default from 9 earlier run(s) of its class, at least 2917 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3624 MB reserved (6845 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 13 earlier run(s) of its class
[capacity] dependencies: sized at 67% of the default from 9 earlier run(s) of its class, at least 4650 MB because of its plan phase
[capacity] admitted dependencies with 4342 of 4650 MB: only 4342 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (4560 MB available, 2048 MB floor)
```
Try 4:
```
[capacity] run: sized at 5% of the default from 12 earlier run(s) of its class, at least 2924 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3631 MB reserved (6243 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 13 earlier run(s) of its class
[capacity] dependencies: sized at 67% of the default from 12 earlier run(s) of its class, at least 4666 MB because of its plan phase
[capacity] admitted dependencies with 4327 of 4666 MB: only 4327 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6707 MB available, 2048 MB floor)
```
Try 5:
```
[capacity] run: sized at 5% of the default from 13 earlier run(s) of its class, at least 2926 MB because of its plan phase
[capacity] admitted run: 8 job(s), 3633 MB reserved (6180 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 13 earlier run(s) of its class
[capacity] dependencies: sized at 67% of the default from 12 earlier run(s) of its class, at least 4669 MB because of its plan phase
[capacity] admitted dependencies: 1 job(s), 4669 MB reserved (6846 MB available, 2048 MB floor)
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (6671 MB available, 2048 MB floor)
```
Tries 2 and 3: not copied here; the logs are `~/native-bootstrap-round-2-try-2.log` and `-try-3.log` on g1-m01.

## Phases

| Try | Before `workspace:setup` (launcher, toolchain, `repo` install, dependency scopes) | Graph step |
|---|---|---|
| 1 | about 2 min (not measured exactly) | 961 s, of it 901 s waiting for the fleet |
| 2 | not measured | about 960 s, of it 900 s waiting |
| 3 | not measured | 791 s, of it 676 s waiting |
| 4 | about 2.5 min (not measured exactly) | 332 s, of it 300 s waiting; local part reached `rederivation 256 of 256` after 30 s |
| 5 | about 2.7 min | 2 s, then the file access failure |

## Files

- `round-2.tail.txt`: last 200 lines of try 5. `round-2.try-4.tail.txt`: last 200 lines of try 4 (the last fleet refusal).
- `round-2.samples.txt`: 30 s samples with pressure level and the peak of the run's processes, and the start and end of each try.
- `run-round-2.sh`: the runner (argument: try to start with).
