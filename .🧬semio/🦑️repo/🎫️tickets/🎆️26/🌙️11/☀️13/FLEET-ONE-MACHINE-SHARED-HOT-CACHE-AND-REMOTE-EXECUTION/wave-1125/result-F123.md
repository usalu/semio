# F123: transaction-v2 62/0 under plain `repo test-file`

Lane F123, 2026-11-26 18:25-19:05. Library = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`. Final: `repo test-file 🧪️tests/🔄️transaction-v2/🟦️.ts` (plain, no ceiling override): 62 pass / 0 fail, 32 s. `repo typecheck` passed (13 files), `dangling-imports.py` exit 0. No graph, cargo, sub-agents, git writes.

Process note: my first four test-only edits (items 1 to 3 below, before I defined the lock) were written without `landing.lock.tests`; all later writes took a lock. They were test-only files, inside the quiet window rules. I also briefly landed a budget row with a reason over 240 chars (table invalid for about 1 min); fixed in place.

## 1. Generator fixture (3 tests)
Production rule (`🔍️discovery/✅️owned-file-approvals` `generatorPreviewExecution`): target executor must be `repo:script`, command exactly `repo ./📜️script.ts preview-generated` for cwd = owner (script path and args derive from the contract). Production untouched. Fixture changes (`🏗️fixture-builders`):
- preview target executor `repo:script` (was `nx:run-commands`).
- `checkTarget: "@fixture/generator:check"` in the contract: without it `assertRegenerationContract` calls `canonicalJson(undefined)` and throws.
- fixture `📜️script.ts` rewritten to the admitted command-router shape (`@semio-tech/process/routing`, `receiveScriptProcessInvocation(process.env, original => runScriptMain(router, {invocation: original}))`); old import `@semio-tech/repo-lib/routing` no longer exists; fixture root gets `📋️project.json` + `workspaces: []` so `findWorkspaceRoot` resolves.
- generation/check run `repo run <project>:generate|check` (fixed by the plan). Real `repo run` boots a graph service for the temp checkout and took over 2 min under load 20, so `installFixtureRepoShim()` puts a fixture-local POSIX `repo` ahead on PATH (runs the owner script directly; skipped on win32, where the real `repo` is used).

## 2. Goldens decoupled from the live taxonomy
- Live taxonomy is no longer read: `FIXTURE_SCHEMA` is the pinned snapshot `🧪️tests/🧫️fixtures/🌳️transaction-taxonomy-snapshot/🔣️.json` (copy of the taxonomy, sha256 805c3c1c..., 1.3 MB). The env override `SEMIO_TRANSACTION_V2_SCHEMA` is gone. It must be refreshed deliberately only when the engine's taxonomy validation changes (older snapshots do not load).
- Finding: only some taxonomy sections feed the plan; the plan digest was stable under unrelated live edits, so the earlier "follows every edit" is only partly true. A full recapture with the pinned snapshot reproduced 92 of 98 keys byte for byte.
- Symlink golden: not a digest change. `cpSync` turned the relative link into an absolute one (then rebased); fix `verbatimSymlinks: true` in both copies; `after-symlink-retargeting` matches the October golden again.
- Recaptured once (`T/🧰️scripts/F123/recapture-golden.py`, capture via `SEMIO_CAPTURE_TRANSACTION_BOUNDARIES`), 6 keys, all transaction side digests only except generator ones:
  - `rolledback:after-embedded-root-staging`, `rolledback:after-relocations`: only the plan digest (f42ed46b -> 775232be); workspace tree identical. Cause: the planner output changed since the 10-23 golden (inference from the diff of `🧹️normalization` since then: reference-inventory, typescript-reference-authority, inventory-planning, plan-parsing); journal differs only in `planDigest`, `stagingRoot`, `backupRoot`.
  - `rolledback:after-regenerations`, `killed/recovered/committed:process-tree-mixed-generator`: new plan digest and new workspace ledgers, because the generator fixture changed (script, project manifest, contract) and the old golden never ran (3 old workspace ledgers were also internally inconsistent: digest key != canonicalJson of nodes).
  - Unreferenced ledgers dropped; boundary key set unchanged (98).

## 3. Language-neutral golden scan
`🔶️keeps-language-neutral-golden`: the AST scan now also reads the case modules that the top file imports locally (`./...`), requires at least one, and still expects 14 titles. The launcher (`📋️orchestration`) had the same blind regex scan and would have thrown the "manifest count changed" error: it now follows the suite's local imports too.

## 4. Budget row and launcher
- Measured peak of the process tree (F94 sampler, ceiling off): 2999 MB, 3411 MB (62/0 run); F122 saw 3641 MB. Row = 4600 MB (3641 + 25 percent) in `🧰️framework/🔨️modules/🏃️process/🧪️testing/👷️workers/📏️file-budgets/🔣️.json` (locks `process`, `transactions` taken together, released at once). Note `T/🧰️scripts/F123/land-budget.py` carries a longer reason than the 240 character limit; the table has the shortened one.
- Launcher patch (`T/🧰️scripts/F123/land-launcher.py`): provenance identifies the pinned snapshot, `schemaSnapshot` option, copy and env removed, static title scan follows case modules.
- Per-case timeout: new `CASE_TIMEOUT_MS = 90_000` (was bun's 15 s); at load 20 the 62-concurrent kill cases timed out at 15 s although each passes alone (14, 6, 1 pass in filtered runs).

Kept: `T/🧰️scripts/F123/{recapture-golden.py,land-budget.py,land-launcher.py}`.
