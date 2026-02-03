# Ticket

## Plan

The ticket mechanism should change to no longer create `plan.md` files when no plan is provided:

### Current Behavior
1. When creating a ticket with no planPath: creates an empty `plan.md` file
2. When creating a ticket with a planPath: moves the plan file to the ticket folder, keeps the filename
3. In both cases, the `Plan` field in the iteration is set to the filename

### Target Behavior
1. When a plan is provided: move the original plan file (keep the filename) to the ticket folder and add the plan filename to `ticket.json` iterations
2. When no plan is provided: **don't create any `plan.md` file** - the iteration's `Plan` field should be empty
3. No `plan_ITERATION.md` files should be created (already the case)
4. Everything else goes in `ticket.md`

### Changes Required

1. **Modify `CreateTicket` function** (line ~8561-8567):
   - Remove the else branch that creates empty `plan.md`
   - When no planPath provided, set `finalPlanFilename = ""` and skip file creation
   - The iteration.Plan will be empty in this case

2. **Verify `ReopenTicket` function** (line ~9776-9783):
   - Already only creates plan entry when planPath is provided - no change needed

3. **Update tests** if any rely on plan.md being created

## Todos

### Iteration 1: Plan file handling
- [x] Write plan to ticket.md
- [x] Modify CreateTicket to not create plan.md when no planPath provided
- [x] Verify ReopenTicket handles no plan correctly
- [x] Update any related tests

### Iteration 2: Author lookup from contributors
- [x] Explore contributor structure and current author logic
- [x] Fix GetGitAuthorGithub to use ListContributors and contributor.json

### Iteration 3: Test cleanup
- [x] Find all ticket tests that need cleanup
- [x] Add cleanup logic to TestTicketOpenContinueKeyword

### Iteration 4: Verify title validation
- [x] Verify title validation only rejects lowercase/uppercase slugs
- [x] Verify tests have cleanup and noIssue:true

### Iteration 5: Flexible CLI argument parsing
- [x] Add helper functions for extracting LLM/UI from args
- [x] Add boolean flags for all allowed LLMs and UIs
- [x] Support positional, boolean flag, and named flag styles
- [x] Test all argument styles

### Iteration 6: MCP CLI-style output
- [x] Add toolResultToMCP helper function
- [x] Update ticketOpen to use ToolTicketOpen
- [x] Update ticketList to use ToolTicketList
- [x] Update ticketRead to use ToolTicketRead
- [x] Update ticketClose to use ToolTicketClose
- [x] Update ticketReopen to use ToolTicketReopen

## Changes

1. **[go/repo/main.go](go/repo/main.go)**:
   - Modified `CreateTicket` function to no longer create empty `plan.md` files when no planPath is provided
   - Fixed `GetGitAuthorGithub` function to use `ListContributors()` instead of reading a non-existent `config.json` file
   - Added `extractLLMFromArgs` and `extractUIFromArgs` helper functions
   - Added `addLLMFlags` and `addUIFlags` helper functions
   - Modified `ticket open` command to support flexible argument parsing
   - Modified `ticket reopen` command to support flexible argument parsing
   - Added `toolResultToMCP` helper function to convert ToolResult to MCP CallToolResult
   - Modified MCP ticket functions (ticketOpen, ticketList, ticketRead, ticketClose, ticketReopen) to use Tool* functions instead of GraphQL, producing CLI-style output

2. **[go/repo/main_test.go](go/repo/main_test.go)**:
   - Fixed `ToolTicketOpen` calls in tests to include the missing `goal` and `parent` parameters
   - Added cleanup logic to `TestTicketOpenContinueKeyword` using `defer os.RemoveAll()` to remove created ticket folder after test

## Log

### Iteration 1
- Analyzed current implementation in `CreateTicket` (line ~8548-8567) and `ReopenTicket` (line ~9776-9783)
- Found that `ReopenTicket` already correctly handles no plan - only creates plan entry when planPath is provided
- Removed the else branch in `CreateTicket` that was creating an empty `plan.md` file
- Fixed test compilation errors: `TestTicketOpenNoticketKeyword` and `TestTicketOpenContinueKeyword` were missing `goal` and `parent` parameters
- All ticket tests pass

### Iteration 2
- Found that `GetGitAuthorGithub` was looking for `config.json` but contributor files are named `contributor.json`
- Simplified the function to use `ListContributors()` which already correctly reads `contributor.json` files
- The function now:
  1. Gets git user name and email
  2. Creates fallback string in `NAME <EMAIL>` format
  3. Searches contributors for matching email
  4. Returns GitHub username if match found, otherwise returns fallback
- All tests pass

### Iteration 3
- Reviewed all ticket tests for cleanup needs:
  - `TestTicketsNonEmpty` - just queries, no cleanup needed
  - `TestTicketListCommand` - just queries, no cleanup needed
  - `TestTicketOpenNoticketKeyword` - uses NOTICKET, no ticket created
  - `TestTicketTitleValidation` - already has cleanup logic
  - `TestTicketOpenContinueKeyword` - was missing cleanup
- Added `defer os.RemoveAll(seed.FolderPath)` to `TestTicketOpenContinueKeyword`
- All tests pass

### Iteration 4
- Verified title validation logic in `CreateTicket` function is already correct:
  - Allows: "Refactor Resource ID System to Bundle-Based Hierarchy" (title with spaces and hyphens)
  - Rejects: "REFACTOR-RESOURCE-ID-SYSTEM-TO-BUNDLE-BASED-HIERARCHY" (uppercase slug)
  - Rejects: "refactor-resource-id-system-to-bundle-based-hierarchy" (lowercase slug)
- Verified `TestTicketTitleValidation` test cases cover these scenarios
- Verified all tests use `noIssue: true` to prevent GitHub issue creation
- Verified all tests have cleanup logic
- All tests pass

### Iteration 5
- Added flexible argument parsing for `ticket open` and `ticket reopen` commands
- Now supports three styles of specifying LLM and UI:
  1. Positional args: `ticket open "Title" "Prompt" gemini-3-pro copilot-chat`
  2. Boolean flags: `ticket open "Title" "Prompt" --gemini-3-pro --copilot-chat`
  3. Named flags: `ticket open "Title" --prompt "Prompt" --llm gemini-3-pro --ui copilot-chat`
- Mixtures of all three styles are supported
- Added helper functions: `extractLLMFromArgs`, `extractUIFromArgs`, `addLLMFlags`, `addUIFlags`
- All tests pass

### Iteration 6
- Changed MCP ticket functions to use CLI-style output instead of JSON GraphQL responses
- Added `toolResultToMCP` helper that converts ToolResult to MCP CallToolResult
- Updated all MCP ticket functions to use their corresponding Tool* functions:
  - `ticketOpen` → `ToolTicketOpen`
  - `ticketList` → `ToolTicketList`
  - `ticketRead` → `ToolTicketRead`
  - `ticketClose` → `ToolTicketClose`
  - `ticketReopen` → `ToolTicketReopen`
- MCP output now shows human-readable text with emojis instead of JSON
- All tests pass

## Summary

Bulk close
