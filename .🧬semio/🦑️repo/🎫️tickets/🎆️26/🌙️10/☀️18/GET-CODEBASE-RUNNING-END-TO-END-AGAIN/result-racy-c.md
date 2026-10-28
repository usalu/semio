# result-racy-c (coordinator, 2026-10-27): hot course no-op never records a reusable plan

## Symptom
Every course hot no-op (g12-hot, 30 s) prints `[bazel] this plan cannot be reused later: a file read by the planner was modified moments before`, so the next hot no-op replans instead of printing `plan reused` → the < 10 s target cannot be met.

## Cause (law-proven)
`🔭️witness/🟨️.mjs` computes `identity(value)` for every `stat`/`lstat` result BEFORE `observeResult` checks `skipped(target)`. `identity` sets `racy` when mtime/ctime is younger than `racyMilliseconds` (3000). Skipped targets (infrastructure = the generated workspace and `<root>/.cache`, the computation's own output, probe files) are therefore never recorded but still make the witness racy. The planner stats freshly written `.cache` files on every run, so no plan is ever sealed.

## Fix (`racy-tools/edit.py <root> check|apply|revert`, anchored, backups in `racy-tools/base/`)
- `observeResult` takes a thunk for the outcome and computes it only for recorded targets; `identity(facts, target)` records the FIRST racy target (`racy: string | null`, null after `finish`/seal unchanged).
- Schema (`racy` string|null, minLength 1), `.d.mts`, reader (closed world), reason text EN/DE now name the file: `{detail} was modified moments before the planner read it` / `{detail} wurde kurz vor dem Lesen durch den Planer geaendert`.
- `recordBazelPlanV1` passes the racy file as `detail`.
- Laws (`🔭️witness/🧪️tests`): new "a file the witness skips ... never makes it racy, however fresh" (RED on base: `racy` = true), racy law asserts the named file; plan-reuse law uses `toBeNull`.

## Verification
Mirror `racy` (library + process): witness 10/10, plan-reuse 4/4, ignore-sources 15/15. Base code + new laws: 3 fail (incl. the skip law with `Received: true`).
