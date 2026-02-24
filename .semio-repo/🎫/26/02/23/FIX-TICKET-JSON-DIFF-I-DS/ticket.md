---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Fix Ticket Json Diff Ids

## Summary

Fixed ticket.json diff IDs to use proper emoji-based identifiers instead of raw paths. Rewrote applyHookPathToSessionDiffModified, added setSectionsFilePath/applyCodeBlockRefsToSessionDiff, fixed session lifecycle, restored concurrent agent deletions. All tests pass.
## Changes

- **`semio-repo/cli/main.go`**:
  - Rewrote `applyHookPathToSessionDiffModified` to use `buildFileID`, `buildFolderID`, `GetBundleByPath().GetID()`, and `DeriveProjectKind` for proper emoji-based IDs
  - Added `applyCodeBlockRefsToSessionDiff` function to propagate section/definition refs from code edit events into session diff
  - Added `setSectionsFilePath` helper to recursively set `FilePath` on parsed sections so `GetID()` returns full qualified IDs
  - Updated `resolveCodeBlockRefs` to call `setSectionsFilePath` and `HydrateSectionsWithDefinitions` after parsing
  - Updated `trackHookInOpenTicket` code edit case to call `applyCodeBlockRefsToSessionDiff`
  - Fixed session fallback in `trackHookInOpenTicket` to generate new session ID when no sessions exist
  - Added initial session creation in `CreateTicket` and `ReopenTicket`
  - Fixed unreachable code in `UriToId` statutes case
  - Restored 13 MCP handler function definitions removed by concurrent agent (`ticketClose`, `ticketReopen`, `draftCreate`, `draftDelete`, `goalOpen`, `goalClose`, `goalReopen`, `sectionCreate`, `sectionMove`, `sectionDelete`, `sectionIntegrate`, `sectionExtract`, `mcpTree`)
- **`semio-repo/cli/main_test.go`**:
  - Updated `TestTicketLifecycle_NoManagement` to expect 1 session after OpenTicket and 2 after ReopenTicket
  - Updated `TestTrackHookInOpenTicketUsesStableSessionIDs` session count assertions
  - Fixed `entry.Event` struct access (14 replacements from `json.RawMessage` to struct fields)
  - Fixed literal `\n` character corruption

## Log

- Investigated root cause: `applyHookPathToSessionDiffModified` used raw path segments (`parts[0]`, `parts[0]+"/"+parts[1]`) instead of proper ID builder functions
- Identified missing `FilePath` propagation in `resolveCodeBlockRefs` causing incomplete section IDs
- Identified missing section/definition diff propagation from code edit events
- Implemented all fixes, resolved concurrent agent conflicts (function deletions, test corruptions)
- All 104+ hook/ticket/native hook tests pass

## Todos

- [x] Root cause analysis
- [x] Rewrite applyHookPathToSessionDiffModified with proper IDs
- [x] Add setSectionsFilePath and applyCodeBlockRefsToSessionDiff
- [x] Update resolveCodeBlockRefs
- [x] Fix trackHookInOpenTicket session management
- [x] Add initial session to CreateTicket and ReopenTicket
- [x] Fix concurrent agent breakages (missing functions, test corruptions)
- [x] Validate all tests pass

## Plan

1. Analyzed the diff section ID generation pipeline
2. Rewrote `applyHookPathToSessionDiffModified` to use `buildFileID`, `buildFolderID`, `GetBundleByPath`, `DeriveProjectKind`
3. Added helper functions for section/definition propagation
4. Updated `resolveCodeBlockRefs` to set `FilePath` on sections
5. Fixed session lifecycle (create on ticket open/reopen, fallback on hook without session ID)
6. Restored functions deleted by concurrent agent
7. Updated test assertions to match new behavior
8. Validated all tests pass
