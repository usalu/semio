# g1-m05 (Windows): R77b Seen Here, and the Publication Rate Against the 3 600 s Cap

Written 2026-11-16T13:10Z by the agent session on the machine. Observation only (round 31).

## R77b Works Here

- The run that started 12:31:53Z (wrapper pid 33140) prints `dependencies:publish-binding:<n>` every 8 bindings. It is the first run since 04:26Z that passed 10 silent minutes without being stopped: at 13:08Z it stands at 2 180 s.
- Before it, `ready-timings.json` holds 15 rows `outcome: stalled` (04:26Z to about 12:31Z), the last three of 1 353 s, 1 251 s and 1 374 s.

## Rate (From `dependencies.log` of This Run)

| Line | Stage second |
|---|---|
| `publish-binding:1` | 780 |
| `publish-binding:216` | 2 140 |

- 215 bindings in 1 360 s = 6.3 s per binding, the same as the "about 6 s" of R77.
- The stage cap is still 3 600 000 ms (`🚦️ready/🧬️schema/🔣️.json` line 69). This run will be cut at about binding 450 (780 s lead, then 2 820 s at 6.3 s each), which is 14:31:53Z.
- If the counter starts at 1 in every run and counts only what the run publishes (I did not read the code), and published bindings stay durable as R77 says, about 1 500 bindings need 4 runs of one hour each, with a 20 minute back-off after every `stage-timeout`: ready for stage `tools` at about 18:30Z at the earliest. With the cap at 14 400 000 ms (proposal 1 of `pool-failures-and-windows-timeout.md` Q2) one run of about 2 h 50 min would do it.
- What the next run shows decides it: if its `publish-binding` phase starts earlier than 780 s or ends before the cap, the bindings were kept. `round-31.md` has the log tail every 10 minutes.

## State

- `ready: false`, stage `dependencies` (3 of 5), `attempts` 10.
- Observer restarted 13:08Z for six more hours.
- Not done: `repo preflight --lang en` (only at `ready: true`), any setup or elevated command.
