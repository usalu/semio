# 🩹 Stale Stash-Pop Conflicts Resolved (Blocking Every `📜️script.ts` and nx Run)

At session start `git status` already showed four `UU` files from a `git stash pop` (markers
`<<<<<<< Updated upstream` / `>>>>>>> Stashed changes`, file times 2026-09-28 16:32 +0200, predating this
ticket). Bun refused to parse the library barrel graph, so `bun ./📜️script.ts …` and the nx project graph plugin
failed for everyone. Each hunk was resolved by keeping both sides' intent; the git index was not touched (no
`git add`), so the files still show as unmerged until their owner stages them.

| File | Upstream side | Stashed side | Resolution |
|---|---|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🛡️protection/🟦️.ts` | `CleanRemovalKind` with `root-transient` | with `empty-folder` | union of both kinds (both are produced elsewhere: `🔍️candidate-discovery` and `🔍️empty-folders`) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` (nx plugin) | `async createDependenciesImplementation` (awaits `collectImportEdges`) | sync with a win32 early return reading only the bun lock graph | async function with the win32 early return |
| same file, `cacheInternals` export | adds `createDependenciesImplementation, importTargetsFromSource, collectImportEdges, projectFilesToProcess, importEdgeCacheRoot` | adds `nxTrackedSourceFile, walkCargoToml` | union (all seven are defined in the file) |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts` | level-aware `lawTimeout` for the workspace contract test | new `test empty-folders` subcommand | both: the subcommand branch, then the level-aware run |
| `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts` | no log line | a `[DEBUG]` PASS log | upstream (temporary `[DEBUG]` logs are removed) |
