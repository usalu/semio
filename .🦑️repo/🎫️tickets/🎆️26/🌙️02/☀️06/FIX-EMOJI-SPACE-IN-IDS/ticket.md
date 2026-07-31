---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary

All tests pass. ToolProjectTree rewritten with toolResultFromEvents event pipeline with events sorted by formatMarkdownResult output for stable CLI/MCP parity. ToolBundleList added for bundle-specific listing using LoadBundles. TestBundleListCommand updated to use ToolBundleList. runProjectTree updated to sort events by rendered output matching ToolProjectTree ordering. AGENTS.md updated with ToolBundleList, ToolProjectTree event sorting, and test coverage documentation.

## Changes

- `repo/cli/main.go`: Rewrote `ToolProjectTree` to use `toolResultFromEvents` pattern matching CLI's event-based rendering pipeline. Sort events by `formatMarkdownResult` output for stable alphabetical ordering. Changed `runProjectTree` to sort events by rendered output instead of by project name for CLI/MCP parity. Added `ToolBundleList` function for bundle-specific listing.
- `repo/cli/main_test.go`: Updated `TestBundleListCommand` to use `ToolBundleList()` instead of `ToolProjectList()` since the latter now correctly returns `[]Project`.

## Log

- Rewrote `ToolProjectTree` from `output.Plain(renderEntityMarkdown(...))` to `toolResultFromEvents(events, projects)` pattern, fixing CLI/MCP output parity (TestParityProjectTree/output_matches_CLI_markdown)
- Changed sort in both `ToolProjectTree` and `runProjectTree` from `projects[i].Name < projects[j].Name` to `formatMarkdownResult(events[i]...) < formatMarkdownResult(events[j]...)` so rendered lines are in ascending string order (TestParityProjectTree/projects_are_sorted_alphabetically)
- Created `ToolBundleList()` using `LoadBundles()` + `toolResultFromEvents` for bundle listing
- Fixed `TestBundleListCommand` to call `ToolBundleList()` instead of `ToolProjectList()` (type mismatch: `[]Bundle` vs `[]Project`)

## Todos

- [x] Rewrite ToolProjectTree with events pattern
- [x] Sort events by rendered output
- [x] Add ToolBundleList function
- [x] Fix TestBundleListCommand
- [x] Run tests
- [ ] Update AGENTS.md

## Plan
