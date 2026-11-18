# g1-m08 Round 30: Report (automatic ready runner; workspace stage refused after step 6 of 6)

Written 2026-11-15T19:07Z. Machine and replica facts: `round-30.pre.md`. No manual `native-bootstrap.sh` was started; `native_rc` is therefore not measured. Everything below is the automatic runner (sidecar role `ready`), observed read-only by `round-30-observe.sh`.

## Stages (from `.cache/f/ready/state.json`)

| Stage | Ended (UTC) | Wall time | Result |
|---|---|---|---|
| native | 17:42:04 | not measured (before this session) | done |
| repo-mcp | 17:49:33 | not measured | done |
| dependencies | 17:53:54 | about 4.5 min (from 17:49:33) | done, attempts 0 |
| tools | 18:48:22 | about 54.5 min | done, attempts 0 |
| workspace | still running | started 18:48:23, 19 min at the time of writing | step 5 of 6 reached |

Runner peak (`peakMegabytes`): 3820 MB up to the end of `tools`.

## Workspace Stage: `[capacity]` and `cut smaller` Lines (verbatim, seconds normalised)

```
[capacity] run: sized at 33% of the default from 1 earlier run(s) of its class
[capacity] admitted run: 1 job(s), 512 MB reserved (10106 MB available, 2048 MB floor)
Reading the project graph slice (0 s): the slice worker grew to 10180 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 2 of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.
Reading the project graph slice (0 s): the slice worker grew to 8282 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 3 of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.
Reading the project graph slice (0 s): the slice worker grew to 9463 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 4 of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.
Reading the project graph slice (0 s): the slice worker grew to 10038 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 5 of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.
```

- Highest step so far: step 5 of 6. The machine started at step 1 and was cut four times; no exit 75 so far.
- Slice worker, 5 s samples (`round-30.p12-auto.txt`): peak resident 7754 MB, peak physical footprint 9366 MB. The `cut smaller` lines themselves name 10180, 8282, 9463 and 10038 MB, all above the 8 GB of the old hard stop of the rounds; the runner did not stop and I stopped nothing.
- Memory pressure in the 30 s samples so far:  120 pressure=1   27 pressure=2    3 pressure=4 .
- Newest progress line: `Reading the project graph slice (231 s): verification 653 of 3622.`
- `sample` of the slice worker above 4 GB: `round-30.p12-auto.sample-20261115T185026Z.txt`.

## Runner Restarted While the Stage Runs (19:06Z)

At 19:06:41Z the runner changed from pid 63361 to pid 88172 and its digest from `34b0264f…` to `5c04858e…` (setup inputs changed in the shared tree). `ready.json` now shows `stage native, 1 of 5`, while its `tail` still carries the graph lines of the workspace stage, and the workspace process tree (pid 72166, `.cache/f/ready/running.json`) is still alive and progressing. `state.json` still lists the four stages as done under the old digests. Not yet seen: whether the new digest makes the runner repeat the stages after this workspace run ends.

Replica health at 19:06Z: failures `unstable` count 4 (new since 17:50Z), store ok.

## Result of the Workspace Stage (added 2026-11-15T19:12Z): Refused After Step 6 of 6

The workspace stage that began at 18:48:23Z ended at 19:11:48Z (23.4 min, modification time of `workspace.log`) with the typed refusal, after all six steps. This is the case round 30 asks to copy. Exit code: not measured (`.cache/f/ready/workspace.exit` does not exist, and the runner that started the stage had been replaced at 19:06Z, so `state.json` records neither a failure nor an attempt for it).

```
Reading the project graph slice (0 s): the slice worker grew to 10180 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 2 of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.
Reading the project graph slice (0 s): the slice worker grew to 8282 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 3 of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.
Reading the project graph slice (0 s): the slice worker grew to 9463 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 4 of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.
Reading the project graph slice (0 s): the slice worker grew to 10038 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 5 of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.
Reading the project graph slice (0 s): the slice worker grew to 9847 MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step 6 of 6: fewer projects at once and smaller waves) and continues; projects already inferred are kept and nothing is repeated.
Reading the project graph slice (227 s): verification 900 of 3622.
The project graph could not be read on this machine yet: the plan stage verification (during verification) grew to 10230 MB, more than the 6553 MB this machine gives a plan, and only 2825 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped before the host ran short of memory. Nothing is wrong with your tree; run the command again when a machine with free memory serves the graph, or read the graph on a larger machine.
```

- Highest step: 6 of 6. Sizes named at the cuts: 10180, 8282, 9463, 10038, 9847 MB; final refusal 10230 MB in plan stage `verification (during verification)`, 2825 MB available.
- The last progress before the refusal stood at `verification 900 of 3622`: the growth is in the verification of input closures, before any project inference of this try, so cutting the number of projects per wave did not lower it (each step grew to 8 to 10 GB again).
- Slice worker, 5 s samples: peak resident 8021 MB, peak physical footprint 9366 MB (`round-30.p12-auto.txt`; one `sample` in `round-30.p12-auto.sample-20261115T185026Z.txt`).
- Memory pressure over the whole observation (30 s samples):  131 pressure=1   27 pressure=2    3 pressure=4 .
- Service log lines `graph-plan-too-heavy` / `plan stop detail`: not copied; no `host-*.log` exists under `$TMPDIR/semio-repo/` on this machine and I did not find where this service writes.

## After It

The replaced runner (pid 93052, digest `5c04858e…`) started the stages again by itself: `native` 2 s exit 0 (19:11:55Z), `repo-mcp` 21 s exit 0, peak 1684 MB (`.cache/f/ready/ready-timings.json`). It will reach `workspace` again; I keep observing and add each further workspace result below. I do not start `native-bootstrap.sh` beside it: it would meet the same refusal and double the memory use on a machine that already reached pressure level 4.

Ready: false. Marker `.cache/f/native-bootstrap.done` absent. `repo preflight` not run (only asked for when ready).
