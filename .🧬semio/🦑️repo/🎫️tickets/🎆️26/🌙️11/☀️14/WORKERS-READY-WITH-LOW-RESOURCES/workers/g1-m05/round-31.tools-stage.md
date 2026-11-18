# g1-m05 (Windows): `dependencies` Done, `tools` Busy Behind Identical Lines

Written 2026-11-16T19:20Z by the agent session on the machine. Observation only (round 31): no setup command, no `repo` command, nothing stopped.

## `dependencies` Is Done (Correction of My Projection)

- `ready-timings.json`: the run of 12:31:53Z ended after 3 810 s with exit 0, outcome `ok`; `state.json` has `dependencies.doneMs` = 13:35:23Z.
- The last counter line was `dependencies:publish-binding:320`. My projection in `round-31.publish-rate.md` (about 1 500 bindings, four runs, 18:30Z) was wrong: the run had about 320 bindings left, the earlier stopped runs had published the others. So published bindings are kept between runs, as R77 says.
- Totals for the stage on this machine: 1 ok, 15 stalled, 4 stage-timeout, 2 stage-failed rows (the file keeps the last rows only).

## `tools` (Stage 4 of 5)

- 7 rows `stage-failed` of 10 to 15 s (13:37Z to 14:10Z), the mirror ends with `Internal error: Exact safe dependency physical identity required`.
- Then 11 rows `stalled` (14:30Z to 18:16Z): 3 345 s, 1 392 s, eight of 617 to 627 s, 1 148 s, 3 076 s.
- The run of 18:16:32Z (wrapper pid 81548, worker `bun.exe` pid 7508) was at 3 560 s when I looked.

## Measurement: The Worker Is Busy in That Phase

10 s sample at 19:15:56Z (`Win32_Process`, read-only):

| Process | Resident | CPU s | Read operations | Other operations | Read KB | Written KB |
|---|---|---|---|---|---|---|
| `bun.exe` 7508 (worker) | 1 021 MB | 5.2 | 8 328 | 348 448 | 817 | 0 |
| `bun.exe` 34888 (door) | 59 MB | 0 | 634 | 190 | 5 | 1 |
| wrapper 81548, `repo.exe` 66712 | 23 MB, 7 MB | 0 | 0 | 0 | 0 | 0 |

Lines of the last 400 of `tools.log` that are not heartbeats: 291 times `native-artifact-input-verification`, 37 times `native-artifact-input-stage`, every one with `"elapsedMs":0` and no counter.

Reading (inference from the two facts above): this is the shape of the `dependencies` case before R77b. The worker does about 35 000 file operations per second, but the only lines it prints in these two phases are identical, so after the normalisation of R83 the stall rule sees nothing new and stops a working stage. The eight stops at about 620 s fit a phase that is silent from its start. A counter on `native-artifact-input-stage` and `native-artifact-input-verification` (as `publish-binding:<n>`) would let the rule tell this from a hang. Whether the phase would end inside the cap I cannot say: no run got through it.

## State

- `ready: false`, stage `tools` (4 of 5), `attempts` 7. `workspace` has not started.
- Observer restarted 19:15Z for six more hours.
- Not done: `repo preflight --lang en` (only at `ready: true`), any setup or elevated command.
