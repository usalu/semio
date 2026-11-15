# Round 2 on g1-m07

## Machine
- Mac mini, 17179869184 bytes memory (16 GiB), macOS 27.2.1 (Darwin 27.2.0), 110 GiB free of 228 GiB on the data volume (measured 15:59 UTC).

## Before the run (2026-11-14T15:59:37Z to 16:00Z)
- Replica code digest (`.cache/f/health.json`, written 15:59:54Z): `39e3688eb315d48831772ca305d78bcb6f35975794444da47750bc5fa7459759`, `store: ok`, daemon pid 23834, sidecar `build-agent` running (pid 23931, 0 restarts), `build-scheduler` and `build-cache` off, trust scope `shared-source`.
- `ps -axo pid,etime,rss,args | grep "service host"`: no line. No `repo service host run` process exists, of any age, so `repo service host stop` was not run.
- Last 5 lines of `.cache/f/sidecars/build-agent.log`:

```
2026-11-14T12:41:26.835Z worker finished: 310718cc-3c21-441f-af5d-6676050e6eb8.2 failed 9
2026-11-14T12:41:31.163Z worker started: 8bc2bd2a-a650-4c8d-8aae-71d52796c42c.1 ae4d560b9f3a
2026-11-14T12:41:34.154Z worker finished: 8bc2bd2a-a650-4c8d-8aae-71d52796c42c.1 completed 0
2026-11-14T12:41:36.430Z worker started: 2e458149-1e1c-4bf7-8d9c-38e89e8b09c4.1 7c5d19adf255
2026-11-14T12:41:38.969Z worker finished: 2e458149-1e1c-4bf7-8d9c-38e89e8b09c4.1 completed 0
```

- Memory pressure level before the run: 1.

## Try 1 (`~/native-bootstrap-round-2.log`, 16818 lines)
- Start 2026-11-14T16:00:08Z, end 2026-11-14T16:22:49Z (1361 s), `native_rc=1`. Marker `.cache/f/native-bootstrap.done` absent.
- Result: fleet refusal. Seconds waited for the fleet: 901. Projects wanted: 703, arrived: 0, missing: 703, this machine can infer: 192 (48 per wave).
- Hard stop: none. 46 samples, all `pressure_level=1`. Peak resident size of one process: 4176464 KiB (about 3.98 GiB), which was also the largest process of the run. No `Inferring projects` line appeared.
- Observation: the bootstrap log itself contains neither `graph-wanted` nor a fleet line nor exit 75. What the terminal shows is the generic text below; the designed refusal is only in the service log. The inner exit code of `repo run workspace:setup` was not measured (the bootstrap reports only `native_rc=1`).

Last lines of the bootstrap log, verbatim:

```
[semio] workspace:setup…
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (8677 MB available, 2048 MB floor)
Starting the repo graph service for this host.
Waiting for the repo graph service (2 ms).
Waiting for the repo graph service (2013 ms).
Waiting for the repo graph service (4040 ms).
Loading the project graph of 440cffa5563d.
Planning 1 targets.
Reading the project graph slice (… s): flush 1 of 1.        (repeated up to 1107 s)
The resident service met an unexpected failure (reference 80ba70d1); try again, the details under that reference are in the service log /var/folders/xq/z6t8hmcd0qj871bx0sj7pbn80000gn/T/semio-repo/501/host-38cdf415c7fb.log.
end 2026-11-14T16:22:49Z
native_rc=1
```

Fleet lines of the service log (`$TMPDIR/semio-repo/501/host-38cdf415c7fb.log`), verbatim:

```
graph: 703 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 703
graph: waiting for the fleet: 0 of 703 projects arrived        (90 times, never another count)
graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 901 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T16:22:49.031Z request-failed 80ba70d1 RepoError: timeout graph-wanted.expired {"wanted":703,"arrived":0,"seconds":901,"missing":703,"local":192}
    at refuse (…/🏗️build/🪪️graph-state/🙋️wanted/📖️read/🟦️.ts:20:232)
    at fleetReadV1 (…/🏗️build/🪪️graph-state/🙋️wanted/📖️read/🟦️.ts:29:39)
    at async fleetReadStaleGenerationV1 (unknown)
    at async loadGenerationGraphV1 (unknown)
    at async loadBuildGraphSliceV1 (unknown)
```

- No line with `restored` or `Restarting with the current implementation`, and no slice worker exit 75, in the bootstrap log or the service log of this try. Before the fleet request the slice worker verified 3491 of 3491 input closures.
- `[capacity]` lines of the bootstrap log, verbatim:

```
[capacity] run: sized at 7% of the default from 9 earlier run(s) of its class, at least 4093 MB because of its plan phase
[capacity] admitted run with 6 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 9% of the default from 9 earlier run(s) of its class, at least 4093 MB because of its plan phase
[capacity] run: sized at 59% of the default from 5 earlier run(s) of its class
[capacity] run: sized at 59% of the default from 5 earlier run(s) of its class
[capacity] dependencies: sized at 76% of the default from 7 earlier run(s) of its class
[capacity] admitted dependencies: 1 job(s), 4677 MB reserved (7541 MB available, 2048 MB floor)
[capacity] dependencies: sized at 76% of the default from 7 earlier run(s) of its class
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (8677 MB available, 2048 MB floor)
```

- Wall time per phase: `workspace:setup` at least 1107 s (its last progress line) of the 1361 s; launcher, rust toolchain, `repo-mcp:install` and `dependencies sync` together about 250 s, not measured one by one (the log carries no time stamps). No nx task ran, so there is no failed nx task.
- The graph service this try started (pid 88716) stayed up after the try; it is younger than one hour and was left alone.

## Try 2 (`~/native-bootstrap-round-2-try-2.log`, 16841 lines)
- Waited 300 s after try 1. Start 2026-11-14T16:27:49Z, end 2026-11-14T16:51:07Z (1398 s), `native_rc=1`. Marker absent.
- Result: fleet refusal again. Seconds waited for the fleet: 902. Projects wanted: 703, arrived: 0, missing: 703, this machine can infer: 192.
- Hard stop: none. 47 samples, all `pressure_level=1`. Peak resident size of one process: 3910176 KiB (about 3.73 GiB).
- The graph service of try 1 (pid 88716) was reused: no "Starting the repo graph service" line in this try.

End of the bootstrap log, verbatim:

```
[semio] workspace:setup…
[capacity] run: sized at 46% of the default from 3 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (7579 MB available, 2048 MB floor)
Loading the project graph of 440cffa5563d.
Planning 1 targets.
Reading the project graph slice (… s): flush 1 of 1.        (repeated up to 1079 s)
The resident service met an unexpected failure (reference 20faee18); try again, the details under that reference are in the service log /var/folders/xq/z6t8hmcd0qj871bx0sj7pbn80000gn/T/semio-repo/501/host-38cdf415c7fb.log.
end 2026-11-14T16:51:07Z
native_rc=1
```

Fleet lines of the service log, verbatim:

```
graph: No verified change journal cursor is recorded for generation 72; verifying the tree once.
graph: 703 stale projects, 0 taken from the fleet, this machine can infer 192 (48 per wave); asking the fleet for 703
graph: waiting for the fleet: 0 of 703 projects arrived        (90 times, never another count)
graph: nothing to wait for any longer: asked the fleet for 703 stale projects and received 0 within 902 s, 703 remain and this machine can infer only 192; try again when a machine with free memory serves the graph, or read the graph on a larger machine
2026-11-14T16:51:07.490Z request-failed 20faee18 RepoError: timeout graph-wanted.expired {"wanted":703,"arrived":0,"seconds":902,"missing":703,"local":192}
```

- No `restored`, no `Restarting with the current implementation`, no exit 75 of a slice worker.

## Why the round ended after two tries
- Tries 3 to 6 were not run. My retry driver looked for `graph-wanted` in the wrong part of the service logs after try 2 and ended (an error of my script, not of the build). When I read the result at 16:51Z, `ROUND.md` already said round 3 ("write its report, then start round 3"), so round 2 closes here with two tries.

## Summary of round 2
| try | start (UTC) | end (UTC) | native_rc | waited for fleet | wanted | arrived | missing | peak RSS |
|---|---|---|---|---|---|---|---|---|
| 1 | 16:00:08 | 16:22:49 | 1 | 901 s | 703 | 0 | 703 | 4176464 KiB |
| 2 | 16:27:49 | 16:51:07 | 1 | 902 s | 703 | 0 | 703 | 3910176 KiB |

- Not ready: `native_rc=1`, marker absent. Failed step: `workspace:setup` at the graph step (fleet refusal). No nx task ran in either try.
- Worker side as designed: no learned-peak wait, no memory pressure, the request to the fleet was made. The fleet answered nothing in 1803 s over both tries.
- Files: `round-2.samples.txt`, `round-2-try-2.samples.txt`, `round-2.monitor.txt`, `round-2-try-2.monitor.txt`, `round-2.tail.txt` (last 200 lines of try 2).
