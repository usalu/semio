---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY
---

# Ticket

## Summary

Fixed MCP server GraphQL errors: added uri field to goalType, converted all MCP tool handlers from broken GraphQL mutations to direct Tool* calls, fixed 9 resource handler queries with wrong field names, fixed project detection for non-@ dirs, fixed emojiText U+FE0E, fixed ParseSections API compatibility, added 22 new MCP tool tests, updated dev docs
## Changes

- Added `uri` field to `goalType` GraphQL object definition with resolver calling `Goal.GetURI()`
- Converted all MCP tool handlers from broken GraphQL mutations to direct `Tool*` function calls: `contributorList`, `contributorRemove`, `projectList`, `projectTree`, `folderCreate`, `folderMove`, `folderList`, `folderTree`, `fileList`, `fileTree`, `sectionList`, `sectionTree`, `definitionList`
- Fixed 9 MCP resource handler GraphQL queries with wrong field names: `startLine`/`endLine` → `range { start end }`, `kind` removed from Section, `line` → `range { start end }` for Definition, `interaction` → `interactions` for Ticket, `email` → `emails` for Contributor, `contributions { count }` → `contributions { commits { id } tickets { id } }`, `oid`/`message`/`author` → `id`/`sha`/`title`/`date` for Commit
- Fixed `loadProjectsInternal` to detect both `@`-prefixed and non-prefixed project directories (excluding hidden dirs and `node_modules`)
- Fixed `emojiText` to add U+FE0E text presentation selector after stripping U+FE0F
- Fixed `ParseSections` API call to match new signature (concurrent change by other dev)
- Updated `TestUriToId` expected values to include U+FE0E
- Added 22 new MCP tool tests covering all `Tool*` functions
- Updated AGENTS.md and README.md with MCP architecture and project detection docs

## Log

- Root cause: `goalType` GraphQL object lacked `uri` field, MCP `goalList` query requested it
- MCP tool handlers were using broken GraphQL mutations with non-existent input types
- Systematic conversion of all MCP tool handlers to use direct `Tool*` function calls
- Resource handlers used wrong GraphQL field names (stale schema references)
- `loadProjectsInternal` only scanned `@`-prefixed directories but repo uses non-prefixed dirs
- `emojiText` stripped U+FE0F but didn't add U+FE0E per AGENTS.md requirement
- `ParseSections` API changed by concurrent developer (now takes `content string` instead of `filePath, lines`)

## Todos

- [x] Fix goalType: add uri field
- [x] Convert all MCP tool handlers to use Tool\* functions
- [x] Fix resource handler GraphQL queries (9 queries fixed)
- [x] Fix loadProjectsInternal for non-@ project dirs
- [x] Fix emojiText U+FE0E text presentation
- [x] Fix ParseSections API compatibility
- [x] Run existing tests, fix failures
- [x] Extend tests to cover MCP tools (22 new tests)
- [x] Update dev docs (AGENTS.md, README.md)

## Plan

1. Fix goalType uri field (done)
2. Convert all MCP tool handlers from GraphQL to Tool\* calls (done)
3. Fix resource handler GraphQL queries (done)
4. Fix loadProjectsInternal for non-@ dirs (done)
5. Fix emojiText U+FE0E (done)
6. Fix ParseSections API (done)
7. Run existing tests, fix failures (done)
8. Extend tests for MCP tools (done)
9. Update dev docs (done)
