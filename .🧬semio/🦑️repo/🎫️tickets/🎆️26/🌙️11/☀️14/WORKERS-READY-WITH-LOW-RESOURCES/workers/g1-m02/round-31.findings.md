# Round 31 Findings of g1-m02 (written 2026-11-16 about 01:15Z)

Result of the machine's own `ready` child (read from `.cache/f/ready/state.json`, not started by this session):

- Stages `native` (2 s), `repo-mcp` (11 s), `dependencies` (9 s), `tools` (1253 s) ended with exit 0 and are recorded as done.
- Stage `workspace` stood still: `failure {stage: workspace, attempts: 1, exit: null, code: stage-timeout}` at 2026-11-16T01:12:07Z, `nextAttemptMs` 1794791558733 (retry about 01:12:38Z). Its log printed only `Reading the project graph slice (N s): flush 1 of 1.` every 2 s from about 1200 s up to 8981 s (and `[repo] still working after 14388 s; last phase started: door.script`), so the slice read stood in flush for about 2 h 10 min. No `cut smaller` line (round 30) and no `waiting for the fleet` line was printed in that stage.
- The same observation was sent by the course session of this machine at 01:12Z (a peer message, not verified by me beyond `state.json`, which I read myself at 01:12:34Z and which shows the same failure).
- At 01:12:34Z: memory free 80 % (`memory_pressure | tail -1`), largest process a `git` of the Claude app at 2.3 GB, graph host bun 640 MB (pid 3548), a second bun 635 MB (pid 67444). Not memory: the stall is in the flush step of the slice read.
- The retry started at about 01:12Z (the new `workspace.log` has 402 bytes: `[capacity]` lines, `Starting the repo graph service for this host.`).

What this session did: nothing. It started no setup command (round 31), only observes every 10 minutes (`round-31.md`).

## Attempt 2 of the `workspace` stage (read by me from `.cache/f/ready/state.json` at 2026-11-16T01:55:21Z)

- `failure {stage: workspace, attempts: 2, exit: 75, code: stage-failed}`, `nextAttemptMs` 1794794134141 (next attempt about 01:55:34Z).
- Last log lines: `Reading the project graph slice (666 s): flush 1 of 1.` (the flush stall of 0 to 666 s again), then `The project graph could not be read on this machine yet: the plan stage inference (during inference) grew to 10174 MB, more than the 6553 MB this machine gives a plan, and only 6872 MB of memory were available (at least 2048 MB are kept in reserve) after it gave back what it could. It was stopped be…` (tail cut by the state file).
- The tail of the state file shows no `cut smaller (step k of 6)` line, so the round 30 step-down did not run in this path, or this refusal still has the old form. Whether such a line exists earlier in `workspace.log` was not read here (the log is rewritten per attempt); not measured.
- Reported first by the course session, confirmed by me from the file.

### Correction (read from `.cache/f/ready/workspace.log` at 2026-11-16T01:55:30Z; the course session corrected its first message the same way)

The round 30 step-down did run. Verbatim lines `Reading the project graph slice (0 s): the slice worker grew to N MB, more than the 6553 MB a plan may take on this machine, so the work is cut smaller (step k of 6 …) and continues …` with N = 6972 MB (step 2 of 6), 10579 MB (step 3), 9322 MB (step 4), 7648 MB (step 5), 7339 MB (step 6). Then the refusal at 10174 MB after step 6 of 6 (one project at a time). So even one project at a time needs more than the 6553 MB plan limit on this 16 GB Mac; the graph child's peak resident size seen was 10579 MB. My earlier sentence that the step-down "did not run" was wrong and is withdrawn.

## Attempt 3 (read at 2026-11-16T03:39Z)

`failure {stage: workspace, attempts: 3, exit: 75, code: stage-failed}` at about 03:38Z: the plan stage inference grew to 10233 MB, more than the 6553 MB a plan may take, and only 4041 MB of memory were available. Flush stall `flush 1 of 1` again, 391 s. A fourth attempt started (`running.json`, pid 48774). `readyMs` is still null.

## Attempt 6 (read at 2026-11-16T09:39Z)

`failure {stage: workspace, attempts: 6, exit: 75, code: stage-failed}`; `readyMs` null; no marker. ROUND.md has not changed since round 31 (2026-11-15T20:55Z): no coordinator round in 12 hours. The failure is still the plan limit (round 31 text above: one project's inference above 6553 MB), which this session cannot change.

## Attempt 12 (read at 2026-11-16T15:59Z)

`failure {stage: workspace, attempts: 12, exit: 75, code: stage-failed}`; `readyMs` null; no marker. `ROUND.md` is still round 31 (2026-11-15T20:55Z): no new coordinator round for 19 hours. Nothing changed on this machine; this session only observes.

## Attempt 20 (read at 2026-11-16T21:59Z)

`stage-failed`, exit 75, `readyMs` null, `ROUND.md` still round 31 (25 hours without a new round). Unchanged; this session only observes.
