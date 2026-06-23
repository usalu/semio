---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-VSCODE-EXTENSION
---

# Ticket

## Summary

Refactored the repo/vscode extension sidebar into exactly two sections (Monorepo and Filter) with emoji-prefixed labels, 🆔 copy-to-clipboard support, contributor expandable children (Emails, Links, Contributions), filter none/all toggles, and comprehensive test coverage. Removed legacy Todos section and all backwards-incompatible APIs. All 63 tests passing.

## Changes

### package.json

- Removed `compose.todos` view, kept only `compose.monorepo` and `compose.filter` in `repo` container
- Added commands: `compose.copyId`, `compose.refreshMonorepo`, `compose.mailto`, `compose.openLink`, `compose.copyCommitSha`, `compose.openCommitInGitHub`, `compose.ticketReopen`
- Added ~40 filter toggle commands with none/all variants for bundle, folder, definition, ticket categories
- Completely rewrote menus section with proper view references and contextValue regex patterns

### extension.ts

- Rewrote `FilterTreeDataProvider`: emoji-prefixed labels, time hierarchy from available data, none/all toggles, setTimeMode
- Rewrote `MonorepoTreeDataProvider`: emoji-prefixed root categories, kind icons, contributor expandable children, nodeId for copy support
- Added `MonorepoTreeItem.nodeId` parameter for clipboard copy
- Rewrote `registerCommands`: copyId, mailto, openLink, refreshMonorepo, copyCommitSha, openCommitInGitHub, ticketClose, ticketReopen, policyCheck, filter none/all toggles
- Fixed `navigateToFolder` path parameter shadowing

### queries.ts

- Removed `TodosDocument`, `TodoCreateDocument`, `TodoDeleteDocument`

### extension.test.ts

- Updated EXPECTED_COMMANDS to match new command set
- Updated Filter Provider tests: emoji labels, time hierarchy with available values, none/all toggle tests, setTimeMode tests, search state tests
- Updated Monorepo Provider tests: emoji labels, contextValues, nodeId tests
- Updated Sidebar View tests: replaced runCommand test with new command tests (copyId, mailto, openLink, refreshMonorepo, copyCommitSha, openCommitInGitHub, ticketReopen)

## Log

- Explored codebase, identified all files to change
- Refactored package.json (views, commands, menus)
- Rewrote providers (FilterTreeDataProvider, MonorepoTreeDataProvider, MonorepoTreeItem)
- Rewrote commands (registerCommands)
- Removed todo queries
- Verified build (221 modules, 1,311.21 kB)
- Updated all tests to match new implementation
- All 63 tests passing

## Todos

- [x] Analyze codebase
- [x] Refactor package.json
- [x] Rewrite providers
- [x] Rewrite commands
- [x] Remove todo queries
- [x] Verify build
- [x] Update tests
- [x] Run tests (63 passing)

## Plan

N/A - Completed
