# g1-m03 Round 1

- Machine: Mac mini M4, 16 GB memory (`hw.memsize` 17179869184), macOS 27.2.1 (26B131), free disk 96 GiB of 228 GiB before the run. Worktree `/Users/ueli/Documents/semio`.
- Replica daemon code digest (`.cache/f/health.json` at the end of the run): `c87388b552163f1c944d842d30bf2de297bd538ce1038efe75e81d300fcbb96e`, store `ok`. The replica `status` command was not run during the round (one graph command at a time).
- Command: `W/native-bootstrap.sh "$HOME/native-bootstrap-round-1.log"`, once, in its own process group, started by `workers/g1-m03/run-round.sh 1` (the run is longer than the agent's 10 minute command limit; the runner samples and applies the hard-stop rule of `ROUND.md`).
- Start `2026-11-14T10:48:09Z`, end `2026-11-14T11:26:49Z` (38 min 40 s).
- `native_rc=1`. Marker `.cache/f/native-bootstrap.done` absent. **Not a build failure: hard stop by the worker.**

## Hard stop

`2026-11-14T11:26:49Z: memory pressure level 2` (`sysctl kern.memorystatus_vm_pressure_level`, 1 = normal, 2 = warning), one SIGINT to the process group. The bootstrap answered at once:

```
SIGINT received: the running work is cancelled and its leases are released; this process exits after 10 s at the latest.
Cancelled (SIGINT); leases released.
Internal error: Original Script process interrupted
end 2026-11-14T11:26:49Z
native_rc=1
```

At that sample the three largest processes were 0.99 GB, 0.80 GB and 0.43 GB resident; 30 s earlier (pressure 1) 2.95 GB, 1.61 GB, 1.44 GB. After the stop: pressure level 1, swap `total = 3072.00M used = 2560.19M free = 511.81M`. 77 samples before it had pressure level 1.

The other hard-stop lines did not appear: no `graph: refreshing N projects`, no `Inferring projects: … of M projects`. The log has a differently worded line with a number above 300, which was **not** treated as a hard stop: `Reading the project graph slice (264 s): owners 1 of 656.` up to `Reading the project graph slice (1912 s): owners 656 of 656.` The coordinator may want that wording in the rule.

## Failed tasks

None reported by nx: the run was interrupted while tasks were running. Finished before the stop: `✔ nx run @semio-tech/ui-rs:generate:prototype`, `✔ nx run @semio-tech/assets:build:prototype`, `✔ nx run @semio-tech/ui-styling-tokens:generate:prototype`. Started and not finished: `> nx run workspace:deps-js-all:prototype`. The 15 lines before the interrupt (without the JSON progress lines):

```
✔  nx run @semio-tech/ui-rs:generate:prototype
✔  nx run @semio-tech/assets:build:prototype

> nx run workspace:deps-js-all:prototype

✔  nx run @semio-tech/ui-styling-tokens:generate:prototype
[script] cache:entry · Waiting for access to native-storage:[16777232,1582586,8588926976,16777296] (shared): position 2 of 2, behind the request of process 16774 (exclusive, queued 0 s ago) · holder: unknown (no live registry entry)
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Verifying build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
[script] cache:entry · Preparing build tools
```

## `[capacity]` and `graph:` lines (verbatim, all 19 `[capacity]` lines; there is no `graph:` line in the log)

```
[capacity] run: sized at 5% of the default from 10 earlier run(s) of its class, at least 4633 MB because of its assembly phase
[capacity] admitted run with 6 of 8 job(s): the host cannot carry more right now
[capacity] run: sized at 6% of the default from 10 earlier run(s) of its class, at least 4633 MB because of its assembly phase
[capacity] run: sized at 59% of the default from 8 earlier run(s) of its class
[capacity] run: sized at 59% of the default from 8 earlier run(s) of its class
[capacity] dependencies: sized at 85% of the default from 3 earlier run(s) of its class, at least 5863 MB because of its assembly phase
[capacity] admitted dependencies with 5033 of 5863 MB: only 5033 MB are free above the 2048 MB floor right now, and 3072 MB are enough to start
[capacity] dependencies: sized at 85% of the default from 3 earlier run(s) of its class, at least 5863 MB because of its assembly phase
[capacity] run: sized at 46% of the default from 2 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 2048 MB reserved (8033 MB available, 2048 MB floor)
[capacity] run: work has to run, waiting 0s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (1496 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 3647 MB)
[capacity] run: work has to run, waiting 6s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (799 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 3647 MB)
[capacity] run: work has to run, waiting 12s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (375 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 3542 MB)
[capacity] run: work has to run, waiting 18s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (557 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 3542 MB)
[capacity] run: work has to run, waiting 24s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (742 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 3532 MB)
[capacity] run: work has to run, waiting 29s to grow the reservation from 2048 MB to 8190 MB: not enough free memory (1014 MB spare above the 2048 MB floor; 1 running, 0 ahead); behind graph-plan (running, 1 job(s), 3532 MB)
[capacity] run: reservation grown from 2048 MB to 3510 MB with 2 of 8 job(s) after 30s: the host cannot carry more right now (3610 MB available, 2048 MB floor); the run grows to more jobs once memory and cores free up
[capacity] run: grown from 2 to 3 of 8 job(s) after 840s, reservation 3510 MB -> 3510 MB: memory freed up and nothing of ours waits (3471 MB available, 2048 MB floor); work that starts from now on uses the new jobs
[capacity] run: sized at 93% of the default from 2 earlier run(s) of its class
```

## Peak resident size

Sampled every 30 s with `ps -axo rss,command | sort -nr | head -3` (78 samples, `round-1.samples.txt`). Peak: **6016976 KiB (5.74 GiB)**, a `bun -- …` process of the project graph phase; next samples 5971424 KiB and 5954096 KiB of the same kind. The replica daemon itself was at 1539584 KiB shortly before the round. No process above 8 GB.

## Wall time per phase

The log has no absolute timestamps; the times below come from the `still working after N s` counters of the commands and the start and end lines.

| Phase | Wall time |
|---|---|
| launcher, rust toolchain, native repo command (`repo run repo-mcp:build`), dependency scopes | about 3 min 40 s (10:48:09Z to about 10:51:50Z, derived from the `workspace:setup` counter) |
| `workspace:setup`: until the first `Reading the project graph slice` line | 264 s |
| `workspace:setup`: `Reading the project graph slice`, owners 1 to 656 of 656 | 1648 s (264 s to 1912 s) |
| `workspace:setup`: tasks, until the SIGINT | about 180 s (counter at about 2090 s) |
| each phase separately for launcher / toolchain / repo command / dependency scopes | not measured |

## Left running

Two `bun` processes started 35 minutes before the end of the round (pids 93730 and 93740, 64 MB and 324 MB resident) are still there; they look like the resident graph service of the worktree. Not stopped.
