---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI/REPO-CLI-FILTERS
---

# Ticket

## Summary

Default markdown output is enforced for top-level tree, project tool output typing is corrected, and tree/markdown behavior is fully covered in existing tests.

## Changes

- Updated `repo/cli/main.go`:
- `treeCommand` now honors `Config.Format` directly for top-level tree output.
- Added `RenderMonorepoTreeMarkdown` and recursive markdown tree renderer.
- Kept connector renderer for `--text` via `RenderMonorepoTree`.
- Added JSON output mode for top-level `tree`.
- Fixed `ToolProjectList` to use sorted `LoadProjects()` (`[]Project`) instead of `GetProjects()` (`[]Bundle`).
- Updated `repo/cli/main_test.go`:
- Extended `TestTreeCommands` to assert default markdown output and explicit `--text` / `--json` behavior.
- Extended `TestMarkdownOutput` to validate markdown defaults without requiring explicit `--md`.
- Extended `TestRenderMonorepoTree` with markdown renderer assertions.
- Updated `README.md` under `# 📦 Bundles` with default `tree` markdown behavior and project list source contract.
- Updated `AGENTS.md`:
- SRS business-logic requirement now states markdown default for `tree` and ASCII via `--text`.
- Codebase docs updated for `tree` render-path selection and `ToolProjectList` project-typed output.
- Test coverage documentation updated to include markdown/default tree tests.

## Log

- Listed goal tree and opened ticket under `AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI/REPO-CLI-FILTERS`.
- Located root cause: top-level `tree` bypassed stream renderer and always used connector-style output.
- Implemented dedicated markdown tree renderer and format switch in `treeCommand`.
- Extended existing test file only (`main_test.go`) per request.
- Ran focused tests for tree/markdown/rendering.
- Ran full verbose suite once, found failing `TestToolProjectList`, fixed project tool data type mismatch.
- Re-ran focused coverage including `TestTool*`, tree, and markdown tests.

## Todos

- [x] Open ticket and track work in ticket workspace.
- [x] Make top-level tree output respect default markdown behavior.
- [x] Ensure explicit `--text` and `--json` still work for tree.
- [x] Extend existing test file with default markdown and mode checks.
- [x] Fix additional test failures discovered during full suite validation.
- [x] Update `README.md` and `AGENTS.md` documentation.
- [x] Prepare ticket close summary and changed file list.

## Plan

1. Reproduce output mismatch on top-level `tree` and find non-rendered code paths.
2. Refactor tree rendering to be format-aware with markdown default semantics.
3. Extend existing tests in `repo/cli/main_test.go` to validate defaults and mode overrides.
4. Run tests, fix regressions, then update dev docs and close ticket.
