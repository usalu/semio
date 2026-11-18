# g1-m04 Round 28 (Linux, 16 GB, SteamOS)

Fourteen tries five minutes apart from 12:40 to 15:22 UTC on 2026-11-15, ended because `ROUND.md` named round 29. Marker absent, `ready: false`. Step 0 was not run again (exit 1 twice, see `round-27.md`). Round 29 runs in `run-loop.sh 29` from 15:23 UTC; the replica then had the code `32b23f76…` with nothing pending.

Files beside this report: `round-28.tries.txt`, `round-28[-try-k].service.txt`, `.bootstrap-last40.txt`, `.service-last60.txt` (stopped tries), `.p12.txt`, `.samples.txt`, `.monitor.txt`, `round-28.tail.txt`. Logs: `~/native-bootstrap-round-28[-try-k].log`.

## Tries

| Try | UTC | End | Slice worker peak | Largest process |
|---|---|---|---|---|
| 1 | 12:40 to 12:45 | SIGINT of my sampler above 8 GB, rc 7 | 9035 MB | 9053 MB |
| 2 | 12:51 to 13:02 | same | 8778 MB | 8773 MB |
| 3 | 13:08 to 13:15 | sampler stop and, at the same moment, the refusal below; no `native_rc` line written | 9624 MB | 9624 MB |
| 4 | 13:20 to 13:25 | sampler stop, rc 7 | 9142 MB | 9143 MB |
| 5 | 13:31 to 13:36 | same | 9170 MB | 9190 MB |
| 6 | 13:42 to 13:47 | same | 9041 MB | 9040 MB |
| 7 | 13:53 to 13:58 | same | 9066 MB | 9065 MB |
| 8 | 14:04 to 14:09 | `native_rc=1`, service memory limit | 9188 MB | 7021 MB (30 s sample missed the peak) |
| 9 | 14:14 to 14:19 | sampler stop, rc 7 | 8733 MB | 8785 MB |
| 10 | 14:25 to 14:31 | same | 8647 MB | 8648 MB |
| 11 | 14:37 to 14:43 | same | 8935 MB | 8938 MB |
| 12 | 14:49 to 15:00 | `native_rc=75`, refusal below (inside the server outage 14:50 to 15:01 that round 29 names) | 8506 MB | 7872 MB |
| 13 | 15:05 to 15:10 | sampler stop, rc 7 | 9051 MB | 9051 MB |
| 14 | 15:16 to 15:21 | same | 8217 MB | 8218 MB |

The fleet delivered 0 of 703 in every try. Most projects inferred here in one try: 74.

## The Three Tries That Ended by Themselves

Tries 3 and 12, the refusal round 28 expects (try 12 verbatim):

```
The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 9602 MB, more than the 6339 MB this machine gives a plan, and only 2676 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

Try 3: `grew to 9545 MB … only 2283 MB of memory were available`.

Try 8, the line round 28 says must be gone, now with the new limit:

```
The plan cannot continue: the service holds 9764 MB of its 9764 MB memory limit (safety margin 256 MB), so it was cancelled to keep the service up; try again later or ask for a smaller selection.
```

## Finding

The plan's own refusal acts at about 9.5 to 9.6 GB, above the 8 GB hard stop of round 23. On this machine the hard stop therefore ends nearly every try (eleven of fourteen) before the refusal is printed. I kept the rule as written. The growth itself is the same in all tries: the slice worker rises from 1 to 2 GB to above 8 GB within about a minute during plan stage `inference`.

## Setup Tasks

Not run in any try: succeeded 0, failed 0, not run all.
