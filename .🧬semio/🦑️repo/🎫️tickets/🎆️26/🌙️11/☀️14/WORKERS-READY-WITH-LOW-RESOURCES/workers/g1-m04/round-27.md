# g1-m04 Round 27 (Linux, 16 GB, SteamOS), Run on the Code of Round 28

Step 0 once more (10:05 to 10:45 UTC), then twelve tries from 10:46 to 12:40 UTC on 2026-11-15. Round 28 was written at 10:40 UTC, before try 1 began, so all twelve tries ran its code; the files are labelled `round-27` because the series had started. Marker absent, `ready: false`. Round 28 continues in `run-loop-28.sh` (five minutes apart, no try limit) from 12:41 UTC.

**All twelve tries were ended by my sampler, not by the build:** the slice worker of the run went above 8 GB resident (hard stop of round 23), one SIGINT each, `native_rc=7`. The refusal round 28 expects (`grew to N MB, more than the 6339 MB this machine gives a plan`, exit 75) did not appear before 8 GB in any try.

Files beside this report: `round-27.step0.txt`, `round-27.tries.txt`, `round-27[-try-k].service.txt`, `.service-last60.txt`, `.p12.txt`, `.samples.txt`, `.monitor.txt`, `round-27.tail.txt`. Logs: `~/tools-setup-round-27.log`, `~/native-bootstrap-round-27[-try-k].log`.

## Step 0: `repo tools setup all`

Exit 1 again, wall time 2413 s, same end as in round 24, no `restored` / `installed` line per scope:

```
[script] {"owner":"repository:build","command":"repository:command","stage":"running","activity":{"resource":"dependencies:selected-plan:5","mode":"exclusive","elapsedMs":0}}
Internal error: Exact selected dependency fields required
repository:command failed with exit code 1
```

It began at 10:05 UTC, after the 16-role fix of 08:05 UTC. Not run a third time.

## What Round 28 Asked About

| Asked | Measured |
|---|---|
| `The plan cannot continue: the service holds N MB of its N MB memory limit` gone | gone: 0 occurrences in twelve logs |
| `graph: memory readings: machine 15848 MB (physical 15848 MB, cgroup limit none), available <n> MB` | present in every try, available 8943 to 9405 MB |
| `plan stage inference … settled` | present, twelve different lines, for example `graph: plan stage inference settled from 6775 to 7616 MB`, `… from 7075 to 7467 MB`, `… from 6874 to 7062 MB`: the value after settling is higher than before |
| refusal with exit 75 (`grew to N MB, more than the 6339 MB …`) | not printed; no `graph-plan-too-heavy`, no `graph-plan-settled` line, no `could not be read` line |
| `The resident service met an unexpected failure` | not seen |

First plan line (42 of 59 plan lines read like this; the others name 5, 6, 7, 16 or 18 too heavy):

```
graph: 703 stale projects, 0 taken from the fleet, 10 too heavy for this machine (…; it can give about 3298 MB per project), it can infer 192 of the others (48 per wave); asking the fleet for 703
```

## Tries

| Try | UTC | Slice worker peak | Stop |
|---|---|---|---|
| 1 | 10:46 to 10:55 | 9190 MB | above 8 GB |
| 2 | 10:59 to 11:06 | 8970 MB | above 8 GB |
| 3 | 11:10 to 11:15 | 9002 MB | above 8 GB |
| 4 | 11:19 to 11:24 | 8776 MB | above 8 GB |
| 5 | 11:28 to 11:33 | 9035 MB | above 8 GB |
| 6 | 11:37 to 11:42 | 8696 MB | above 8 GB |
| 7 | 11:46 to 11:52 | 9102 MB | above 8 GB |
| 8 | 11:56 to 12:01 | 8298 MB | above 8 GB |
| 9 | 12:05 to 12:10 | 8862 MB | above 8 GB |
| 10 | 12:14 to 12:19 | 8197 MB | above 8 GB |
| 11 | 12:23 to 12:30 | 8925 MB | above 8 GB |
| 12 | 12:34 to 12:39 | 9094 MB | above 8 GB |

Course of try 12 (the others are alike): `plan stage inference starts (managed 904 MB, resident 1176 MB)`; `0 of 703 projects arrived from the fleet, 16 inferred here, 687 left; the server last progressed 1046 s ago`; `plan stage inference settled from 7075 to 7467 MB`; the slice worker (pid 523349) went from 1270 MB at 12:38:02 to 8174 MB at 12:38:33 and 9094 MB at 12:39:03 (30 s and 5 s samplers), no swap. `MemAvailable` fell from 9229 MB to 3150 MB in the same 30 s; PSI full avg10 2.58 %. The last task line before the stop: `build:projects · Inferring projects: 15 of 681 projects · 1 running, longest 0 s: semio-os-infinite (language:cargo, 0 s, 0 rows)`.

- Most projects inferred here in one try: 24. The fleet delivered 0 of 703 in every try.
- Closure entries: 17380 before try 1, then 16331 (eight tries) and 16336 (four tries); so about 1050 fewer than in round 24.
- Access proof present at every service start (route direct), no EPERM line.

## Setup Tasks

Not run in any try: succeeded 0, failed 0, not run all.

## Open for the Coordinator

1. Plan stage `inference` grows from about 1.3 GB to above 8 GB within 30 to 60 s after 15 to 24 inferred projects; the 6339 MB plan limit does not refuse before my 8 GB hard stop. If the limit is meant to act first (ceiling 9508 MB), the 8 GB stop rule of round 23 hides it on this machine: say whether the stop should move to the 9508 MB ceiling for g1-m04.
2. `settled` lines report more memory after settling than before.
3. Step 0 exit 1, `Internal error: Exact selected dependency fields required`, twice.
4. Fleet delivery 0 of 703 (named known in round 28).
