---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY
---

# Ticket

## Summary

Fix the MCP server which fails with `Cannot query field "uri" on type "Goal"`. Convert all MCP tool handlers from broken GraphQL queries to direct Tool\* function calls. Ensure MCP tools return same output as CLI. Extend tests.

## Changes

- Added `uri` field to `goalType` GraphQL object definition with resolver calling `Goal.GetURI()`
- Converted `contributorList` MCP handler to use `ToolContributorList()`
- Converted `contributorRemove` MCP handler to use `ToolContributorRemove()`
- Converted `projectList` MCP handler to use `ToolProjectList()`
- Converted `projectTree` MCP handler to use `ToolProjectTree()`
- Converted `folderCreate` MCP handler to use `ToolFolderCreate()`
- Converted `folderMove` MCP handler to use `ToolFolderMove()`
- Converted `folderList` MCP handler to use `ToolFolderList()`
- Converted `folderTree` MCP handler to use `ToolFolderTree()`
- Converted `fileList` MCP handler to use `ToolFileList()`
- Converted `fileTree` MCP handler to use `ToolFileTree()`
- Converted `sectionList` MCP handler to use `ToolSectionList()`
- Converted `sectionTree` MCP handler to use `ToolSectionTree()`
- Converted `definitionList` MCP handler to use `ToolDefinitionList()`

## Log

- Root cause: `goalType` GraphQL object lacked `uri` field, MCP `goalList` query requested it
- MCP tool handlers were using broken GraphQL mutations with non-existent input types
- Systematic conversion of all MCP tool handlers to use direct `Tool*` function calls
- Resource handlers still use GraphQL queries (valid since they query read-only fields)

## Todos

- [x] Fix goalType: add uri field
- [x] Convert all MCP tool handlers to use Tool\* functions
- [ ] Verify resource handler GraphQL queries are valid
- [ ] Extend tests to cover MCP tools
- [ ] Make all tests pass
- [ ] Update dev docs

## Plan

1. Fix goalType uri field (done)
2. Convert all MCP tool handlers from GraphQL to Tool\* calls (done)
3. Verify resource handlers work correctly
4. Run existing tests, fix failures
5. Extend tests for MCP tools
6. Update dev docs
