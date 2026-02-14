---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MCP/REPO-MCP-TOOLS
---

# Ticket

## Summary

Removed all 15 granular list/tree MCP tools (policy_list, policy_tree, ticket_list, draft_list, goal_list, contributor_list, project_list, project_tree, folder_list, folder_tree, file_list, file_tree, section_list, section_tree, definition_list) and their handler functions. Added a single unified `tree` MCP tool that delegates to the existing monorepo tree infrastructure with query filtering. Updated canonical commands list in repoPolicy. All MCP schema tests pass (29 tools).
## Changes

- `semio-repo/cli/main.go`: Removed 15 list/tree MCP tool registrations and handler functions, added `tree` MCP tool, updated canonical commands list

## Log

## Todos

## Plan

Remove all list and tree MCP tools except the main `tree` tool:

### MCP Tool Registrations to Remove (in `createMcpServer`):
- `policy_list` (line 29428)
- `policy_tree` (line 29434)
- `ticket_list` (line 29464)
- `draft_list` (line 29519)
- `goal_list` (line 29532)
- `contributor_list` (line 29583)
- `project_list` (line 29603)
- `project_tree` (line 29609)
- `folder_list` (line 29637)
- `folder_tree` (line 29644)
- `file_list` (line 29673)
- `file_tree` (line 29680)
- `section_list` (line 29712)
- `section_tree` (line 29719)
- `definition_list` (line 29753)

### Handler Functions to Remove:
- `policyList` (line 30068)
- `policyTree` (line 30073)
- `ticketList` (line 30121)
- `draftList` (line 30275)
- `goalList` (line 30290)
- `contributorList` (line 30389)
- `projectList` (line 30404)
- `projectTree` (line 30409)
- `folderList` (line 30448)
- `folderTree` (line 30461)
- `fileList` (line 30508)
- `fileTree` (line 30521)
- `sectionList` (line 30580)
- `sectionTree` (line 30590)
- `definitionList` (line 30697)

### Add:
- `tree` MCP tool that delegates to the existing monorepo tree logic

### Update:
- Tests referencing deleted tools
