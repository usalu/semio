# Owned SQLite Source Test Groups

## Contract and Initial Evidence

The existing artifact `test-snapshot-sqlite source` command accepts an optional explicit list of source groups, `snapshotSqliteTestGroups`. Every source in the existing `snapshotSqliteTests` ownership list must appear exactly once across nonempty groups. Empty, missing, duplicate, outside-owned and path-equivalent duplicate entries are rejected before any subprocess. Owners without explicit groups retain one complete invocation. Groups execute sequentially, with the owner’s same finite `snapshotSqliteTestBudgetMs` on each invocation. No new executable, runtime dependency, global deadline or skipped law is introduced.

The language-neutral fixture and strict JSON schema are authored under repository library `🧫️fixtures/🪶️snapshot-test-groups` and `🧬️schema/🪶️snapshot-test-groups`. The existing registered process-budget route exercises the real package router inside independent Execa/Bun subprocesses, checks Ajv validation and SQLite’s ordered distinct ownership table, observes invocation arguments and budgets, and checks invalid groups do not execute any source subprocess.

The authentic old-runner RED selected one law (987 ms; four assertions): the requested `[owner-a]` and `[owner-b, owner-c]` groups were observed as one `[owner-a, owner-b, owner-c]` invocation. The failure is retained in generated `snapshot-source-groups-red.log`. The full seventeen-law registered process-budget regression is pending after implementation.

Semio now hand-authors four groups containing its exact twenty retained suites: inventory plus base; eight simple subsets; six composed/geometry subsets; four document/drawing/presentation/BRep subsets. Each group remains bounded by 120000 ms. Only the three separately measured aggregate base/CAD/Document laws receive explicit 30000 ms quick scopes; all assertions and fixtures remain. The original twenty-suite aggregate timeout is documented separately as a harness/resource limit.

## Files

- Repository library Rust artifact runner `⚡️caching/📦️artifacts/🦀️rust/🟦️.ts`.
- Repository library process-budget suite `🧪️tests/⏱️process-budgets/🟦️.ts`.
- Neutral fixture and schema `🪶️snapshot-test-groups/🔣️.json`.
- Semio Rust package `📜️script.ts`, base/CAD/Document SQLite source suites.

All permanent invocation remains within existing `📜️script.ts` and its registered Nx/launch routes.

## First Implemented Regression and Fixture Authority

The new group law passed all valid/default/invalid/path-equivalent/empty cases (8082 ms) in the first full invocation. The existing captured Nextest and two coverage laws then failed because their empty selected-package lists resolved the now workspace-only root Cargo manifest; the canonical execution policy correctly requires an actual package manifest. These are concrete fixture-authority failures, separate from the enclosing 30000 ms command timeout. Their three calls now select the same real package already authored by the neutral native-profile corpus; every compilation/assertion budget check is retained.

The group law now imports and observes the real router once in an independent Execa/Bun subprocess, exercising all eight configurations and collecting each invocation/refusal independently. This removes repeated library startup overhead without omitting a case or replacing the runtime assertions. A fresh full seventeen-law regression is pending; no deadline changed.

## Current Verified Focused Result

The exact registered feature-selection and source-group laws now pass together: two laws, 48 assertions, 8.22 s Bun / 11.4 s uncached Nx, via `@semio-tech/repo-lib:test-process-budgets -- -t 'owned snapshot'`. Every neutral group case and Cargo feature case executed. The full seventeen-law route remains unverified: its current older captured Nextest fixture hits its 5000 ms outer Execa/test cap under the shared native-policy preparation workload, then the enclosing 30000 ms cap stops the aggregate. No global or older fixture timeout changed. This focused green does not claim the full process-budget suite passed.
