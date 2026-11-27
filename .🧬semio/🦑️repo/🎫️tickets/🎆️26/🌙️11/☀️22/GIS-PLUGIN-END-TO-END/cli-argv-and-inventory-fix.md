# CLI argv order, inventory fixtures, debug block

## 1. Flag position (fixed)
Cause: global options (`--lang`, `--tool`, `--workers`) are consumed by `buildOperationArgumentsV1`, leaving `["run","--dry-run","@x:test"]`; every reader (`buildInvocationGraphSelectionV1` line 277 in `build/🕸️graph/🚪️admission/🟦️.ts`, `resolveBuildInvocationV1`, the plan, mode resolution) takes `args[1]` as the task.
Fix at the single choke point, `build/⚙️operations/🔤️options/🟦️.ts`: new `hoistBuildRunTaskV1` (used by `buildOperationArgumentsV1`) moves the one `project:target` positional of `run` directly behind the command; value-taking options keep their value (catalogue `required` flags), `--graph` never takes one, everything after `--` is untouched, idempotent. Applies to all commands via the shared parser; only `run` has a positional at `args[1]` (`test` is a `run` of a `test` target; other commands unchanged).
Tests: `⚙️operations/🔤️options/🧪️tests/🟦️.ts` + language-agnostic rows `runTaskCases` in `🔣️.json` (7 rows), checked with yargs-parser as independent oracle and against `buildInvocationGraphSelectionV1`. Result: 22 pass, 0 fail (`🗑️generated/cli-argv.log`).

## Race of two simultaneous `repo run` (not reproduced)
Started two `repo run @semio-tech/gis-js:test --lang=en --dry-run` in the same second (task and flags in the fixed order) and let them run 280 s: neither raised "Canonical build task identity required"; both queued behind the resident service ("holds this request", `🗑️generated/cli-race-*.log`). With the task first the identity check cannot fail from argv; no shared argv handoff state was found in the invocation path, so the reported dual failure is most likely the same flag-order defect (or an agent running with a stale kernel from before the fix). No regression test for a race that did not reproduce; re-open with the exact failing command line and its log if it recurs.

## 2. Inventory suite (fixed)
Cause: `buildGraphPathV1(root)` now requires an absolute canonical workspace (`repositoryCachePartitionRootDataV1`); the fixture passed `root:"fixture"`. Fix: fixture uses `resolve("fixture-workspace")` (`load/🧪️tests/📁️inventory/🟦️.ts`), loader untouched. Result: 10 pass, 0 fail (`🗑️generated/cli-inventory.log`).

## 3. Debug block
`[DEBUG] [owner-declaration]` and all `dbg` marks removed from `inference/🔎️owner-discovery/🟨️.mjs` (mtime was 12:55, 3 h old; `node --check` ok).
