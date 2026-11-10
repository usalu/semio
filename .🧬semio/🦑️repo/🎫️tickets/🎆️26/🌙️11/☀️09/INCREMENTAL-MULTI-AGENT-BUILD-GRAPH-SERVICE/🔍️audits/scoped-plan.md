# Lane `scoped-plan`: demand-driven, target-scoped planning

`BUILD` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`.

## Findings (what already existed)

- `repo run <p>:<t>`, literal `run-many` and literal `exec` already select `BuildGraphSelectionV1` `tasks`/`owners` (`BUILD/🕸️graph/🚪️admission`, `buildInvocationGraphSelectionV1`). `loadBuildGraphSliceV1` -> `refreshClosure` verifies only the facts of the closure (`sliceClosureV1`), and the Nx plugin reads that selected graph (`readPrivateNxBuildGraphV1`). `show project <n>` already uses `closure`. `kind:"all"` remains only for roster commands (`graph`, `show projects|targets`, `affected`, `run-many` with patterns/`--all`, `watch`), which need the roster by definition.
- What was missing: a tool-neutral, testable definition of the scope and plan (closure semantics, tool closure, counted reads), proof that the scoped plan equals the restriction of the whole plan, and a backend capability flag. Whole-workspace planning is not on the default `run` path; no code removal was needed there.

## Delivered

- New `BUILD/🧭️selection/🎯️scope/` (project `repo-task-scope`):
  - `🧬️schema/🔣️.json`: `TargetScopeV1 {version, targets[], includeToolClosure}` and `ScopedPlanV1 {scope, projects, tasks, edges, projectsRead}` (draft-07).
  - `🟦️.ts`: `planScopedTargetsV1(source, scope, {signal, maximumTasks, progress})`, lazy `ScopeSourceV1` (`select`, counted `node` reads), `scopeSourceOfGraphV1`, `readTargetScopeV1`. Semantics: `dependsOn` strings (`dep`, `proj:dep`, `^dep` with pass-through of projects lacking the target), object forms (`projects`, `dependencies`), cycles, optional targets, required root targets refuse. Project set = task owners plus their transitive project dependencies, plus with `includeToolClosure` the transitive tool projects (declared by `tools`) and their dependencies, without their tasks. Cancellable (signal), progress callback, bounded (`maximumTasks`), EN+DE errors.
  - `🧪️tests/🔣️.json` (language-agnostic fixture, 10 cases), `🧪️tests/🐍️oracle.py` (independent Python reachability), `🧪️tests/🟦️.ts`, `🧪️tests/🧶️nx/🟦️.ts` (actual Nx `createTaskGraph`, third-party validation), `📜️script.ts`, `📋️project.json`.
- `BUILD/🔌️backends`: required registry field `plansScoped` (schema, rows nx/bazel/semio = true, `buildBackendPlansScopedV1`), test added.

## Verified

- `repo --service=off tools exec --light bun -- test ./…/🧭️selection/🎯️scope/🧪️tests/🟦️.ts`: 5 pass, 0 fail (schema, closure + reads bound, refusals/cancel/progress/capacity, restriction of whole plan, Python oracle).
- `…/🔌️backends/🧪️tests/🟦️.ts`: 9 pass, 1 fail; the failure is another lane's in-flight change (`dirtyPathsFromJournalV1` export missing in `🏡️resident/🕰️graph-hooks/🟨️.mjs`), not this lane.
- Nx cross-check (`repo run repo-task-scope:test` via build-slot, includes `🧪️tests/🧶️nx/🟦️.ts`): NOT yet verified. Run 1 timed out waiting for the global slot; run 2 failed in `build:source-admission` with ENOENT `lstat BUILD/⚡️caching` (a relative import in some file outside this lane resolves one level too shallow; all imports in `🎯️scope/**` were checked to resolve); run 3 was still queued for the slot at handback (`🗑️generated/scoped-plan/3.txt`). Re-run `bash …/build-slot.sh 'repo-task-scope:test' <log>` once the slot is free and the bad import is fixed.

## Not done / handoff

- `ScopedPlanV1.inputsRef`/`actionKeys` belong to revision-cas/derived-dag; fields are not added yet. Feeding `ScopeSourceV1` from generation fragments (instead of the existing `sliceClosureV1`) is for the integration lane; the cold-plan RSS/row-count acceptance needs the bench lane measuring `rowsChecked` on `run` of an artifact project.
