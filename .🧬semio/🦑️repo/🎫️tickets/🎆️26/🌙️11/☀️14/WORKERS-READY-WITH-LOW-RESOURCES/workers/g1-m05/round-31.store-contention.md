# g1-m05 (Windows): `tools` Has Not Held the Dependency Store Since 20:34Z; the Agent's Graph Read Holds It for Hours

Written 2026-11-18T03:07Z by the agent session on the machine. Observation only: no setup command run, nothing under `.cache/f/ready` touched. All times UTC.

## What Is Seen

- `ready-timings.json`: every `tools` try since 2026-11-17T20:34:52Z ended as `stalled`, 30 in a row up to 02:56:59Z, most after 611 s to 660 s, the longest 2128 s. Before the reboot of 20:13Z the tries ran long (13 355 s and 11 273 s, see `round-31.tools-capacity-error.md`); since then none got past the wait.
- `tools.log` of the running try (wrapper 36684, started 02:57:00Z) repeats: `Process 9036 (… show projects) holds the dependency store since 2026-11-18T01:32:05.962Z; waiting for it (474 s of at most 21600 s)`.
- Process 9036 is the graph read of the build agent: runner 16084 (`… graph-server\read.log … read-watch 1800000 2000 C:\semio refresh repo show projects`) → `repo.exe` 34340 → `🚪️door/script.ts` 10008 → 9036. Started 01:31:00Z, 1909 s processor time, 653 MB, still running at 03:05Z (`read-record.json`: `reason: refresh`, `status: null`).
- The reads before it, from the rotated logs in `.cache/f/graph-server/` (the number in the name is the end time):
  - ended 22:39:19Z: last lines `[repo] still working after 8214 s; last phase started: source.served`, no result line
  - ended 01:09:52Z: last lines `still working after 7762 s; last phase started: source.served`, no result line
  - current: at 5670 s, same phase; `Ownership walk: 970880 steps in 4437 s, 219 per second since the start and 186 per second lately (readText)`; 1504 `dependencies:workspace-edges` activities so far
- So a graph read takes more than two hours on this machine, its log ends without a result, and the next one starts within about 20 minutes. The store is held almost without a gap.
- `kills.log` for the stage: `the stage showed no output and no processor time for 600 second(s) and was stopped as a whole` (for example 02:44:47Z), and since the code change `the stage waited for a holder while its code changed and was stopped` (02:56:59Z). The log itself does get a line every 10 s while waiting.

## What Follows From It

- The wait is allowed 21 600 s by the store, but the stage runner stops the stage after 600 s, so the stage never outlasts a read of 7 000 s to 8 000 s. Each stop puts the stage at the back again.
- The read and the stage do the same slow work on Windows (the per-file ownership walk at about 200 steps per second) and exclude each other. Neither was seen to finish tonight.
- I do not know what ends the reads without a result line (the two rotated logs just stop); I did not see a kill record for them in `ready/kills.log`.

## What Would Help (Your Decision)

- Do not count a wait for a named, live holder as a stall, or give the stage the same 21 600 s as the store's wait.
- Or: while `ready` is false, let the agent's refresh read wait for the stage instead of the other way round.
- Tell me if I should end the running read once so the stage gets the store; I will not do it without your word, round 31 says observe only.

## Other State at 03:05Z

- Terminal windows: 1 `OpenConsole.exe` (from 20:56:01Z) for more than six hours; the flood of `round-31.console-windows.md` has not come back. The scheduled tasks are still registered in the old form (needs an elevated shell).
- Memory: 40 048 MB free of 65 447 MB. Replica: `store: ok`, build agent running (pid 29668, 0 restarts). `head.json` last moved 20:53Z, which matches my last write.
- `round-31.md` got no entry between 2026-11-17T15:06Z and now: the observer of the last session only watched for `ready`, a round change and `next.md`. The full observer (`round-31-observe.ps1`) runs again since 03:06Z.
- Leftovers in `S:\v1\origins\g1-m05\`: about 80 `pool.json.<pid>.tmp`, 11 `ready-log-tools.txt.<pid>.tmp`, 1 `ready.json.<pid>.tmp` from interrupted writes.
