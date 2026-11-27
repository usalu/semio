# F122: transaction-v2 runs under `repo test-file` (run identity issued by the fixture)

Lane F122, 2026-11-26 18:30. Library = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. No graph, cargo, sub-agents or git writes.

## 1. Where the run identity comes from
`🧪️tests/🔄️transaction-v2/🏗️fixture-builders/🟦️.ts` `fixtureRunRoot()` demanded `SEMIO_TEST_ARTIFACT_DIR` + `SEMIO_TRANSACTION_V2_RUN_ROOT` (run id: `SEMIO_TRANSACTION_V2_RUN_ID`, which `boundary-contracts` already defaults to `<pid>-<uuid4>`). Only the aggregate launcher `🔄️transactions/🧪️verification/🏃️shard-execution/🟦️.ts` sets them: it allocates the run owner with `transactionV2BundleRoot` (`📁️run-allocation`) and spawns 8 filtered `bun test` shards with the env. `repo test-file` routes this file as "plain" (no suite row), so nothing allocated a run root: every fixture call threw "Invalid transaction fixture run identity" (not an env gap in the tool, a fixture that could only work under its launcher).

## 2. Fix (fixture issues its own identity, same allocator as the launcher)
`fixtureRunRoot()` keeps the exact propagated path (env run root -> `assertFixtureRunRoot`, unchanged, so the language-neutral harness law that extracts that function still holds). Without a propagated root it issues one: `issueStandaloneRunRoot()` allocates `<artifact dir>/🧾️runs/🔖️<uuid>` through `transactionV2BundleRoot` and validates it with `assertFixtureRunRoot`; without `SEMIO_TEST_ARTIFACT_DIR` the artifact dir is a private `realpath` temp dir removed at process exit. Memoised per process. One file touched (sha 8f424e7d -> f1c822f0), both entry points use the same allocator and validator. `repo typecheck` passed (6 files), `dangling-imports.py` exit 0.

## 3. Real counts
- `repo test-file` default: the process tree is stopped at the 3072 MB default ceiling (held 3191-3641 MB: 62 concurrent cases plus the spawned crash-driver children), so no totals. The file is not in the test file budgets table (`🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/📏️file-budgets/🔣️.json`).
- `SEMIO_TEST_FILE_CEILING=off repo test-file ...`: **62 tests, 55 pass, 7 fail**, 35 s. Direct `bun test` of the file gives the same 55/7 (main process 661 MB); the 8 launcher filters give 1/4, 4/3, 4/0, 6/0, 10/0 x4 (same 7 failures, each reproducible alone).
- Staged (quiet window 18:15-19:00 forbids a process-area landing): `T/🧰️scripts/F122/land-budget.py` declares the file at 4000 MB (atomic, path order kept). Land after 19:00 so plain `repo test-file` reports totals.

## 4. The 7 failures are not F120's (ownWriter path is covered)
F120's change only acts for a file name declared in `✍️own-file-writers` (`📋️build-owners.json`); no transaction-v2 fixture has one, and 55 passing cases include every apply, rollback, kill and recovery shard through the changed `transaction-steps` and `recovery` modules (shards 3-8 all green, plus after-staging, after-moves, after-edits, before-verify). Failures, all in planner/golden territory:
1. Generator fixture (3: `rolls back after-regenerations`, `process-tree-killed mixed generator`, `rejects stale generator`): the plan has 0 regenerations; blocking violation `generator-preview-invalid ... Generator target is not the exact owner JSON preview command`. The fixture's generator contract (`@fixture/generator:generate` / `preview-generated`) no longer satisfies the preview rule. Fix is in `generatorFixture` (builder) once the exact owner preview command shape is chosen.
2. Golden digests (3: `after-embedded-root-staging`, `after-relocations`, `after-symlink-retargeting`): captured and diffed against `🧪️tests/🧫️fixtures/📒️transaction-ledger-boundaries/🔣️.json` (written 2026-10-22): the workspace tree is identical node for node; only the plan digest (and with it `planDigest`, `stagingRoot`, `backupRoot`) differs, so the planner output for these fixtures changed since the golden. Needs a deliberate golden recapture (`SEMIO_CAPTURE_TRANSACTION_BOUNDARIES`), not done here because the plan digest follows every taxonomy edit.
3. `keeps the language-neutral golden aligned ...`: `staticTitles` found 0 of 14 test titles, the AST check only reads the top `🟦️.ts` but the cases moved into sibling modules; the check needs the module list.

Kept: `T/🧰️scripts/F122/land-budget.py`.
