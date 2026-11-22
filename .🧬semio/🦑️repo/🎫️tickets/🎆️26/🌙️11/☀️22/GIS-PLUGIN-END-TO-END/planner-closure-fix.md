# Planner closure fix

## Changes (graph, under `build/🕸️graph/`)

1. `📥️load/🛟️owner-isolation/🟦️.ts`: new `stableOwnerFailureHoldsV1(error,holds)` (named owner failure with `stable` whose consumed rows still hold).
   `📥️load/🟦️.ts` (repair catch branch of `loadBuildGraphAttemptV1`) now throws at once on it, before the unchanged-tree-digest condition, so no settle/closure re-walks follow a reproduced failure.
2. `🚪️admission/🟦️.ts` (`closeBuildOwnerAdmissionV1`): admission edges are typed by `origin` kind (`selection`, `task`, `caret`, `dependency`, `input`, `file`).
   Owners admitted only by `file` go into an `ownership` set: they stay in `owners`, their manifest is added to `files`, the importing owner keeps the imported file in `files` (ownership and input hashing unchanged), but `inspect` (observation and inference) is skipped.
   Any later `task`, `input`, `dependency` or `caret` edge (`admit` with a non-file kind) removes it from the set and re-queues it for the full observation. `all` and named selections admit with kind `selection`, so they are untouched.

## Tests (all via `repo test-file`)

- Added in `🚪️admission/🧪️tests/🟦️.ts`: file-edge owners are ownership-only (not observed, manifest and imported files present) and are observed once a dependency edge needs them.
- Added in `📥️load/🛟️owner-isolation/🧪️tests/🟦️.ts`: stable decision table for `stableOwnerFailureHoldsV1`.
- Changed in the same file: the old "withheld" test reached its failing owner through a file edge; it now asserts ownership-only (never observed, no failure), and a new twin reaches the owner through a dependency edge and keeps the withheld-and-named behaviour.
- Result: admission 10 pass; owner-isolation 16 pass; settle-churn, closure-first, task-level-failure, observed-files, frontier all pass (0 fail).

## Timing

`repo run-many --dry-run --lang=en --targets=test --projects=@semio-tech/gis-js` did not finish within about 10 minutes: the host was memory-starved by other agents ("waiting for host capacity" for 160 s, then "reading the owners (round 1) 371 of 3600"). This is the generation-slice path (`loadGenerationGraphV1`), which re-infers planner-stale records after the planner source change. No `[graph-closure]` line was reached, so the closure size after the change is not measured. Not verified end to end; rerun on a quiet host and read `[graph-closure]` kinds (`file` should no longer drive observations).
Residual risk: the inference side still builds a light project node for each ownership-only owner (no targets), which was not changed.
