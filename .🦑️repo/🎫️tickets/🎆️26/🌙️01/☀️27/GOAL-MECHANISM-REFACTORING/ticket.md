# Ticket

## Todos

- [x] Define `Goal` struct and storage mechanism in `./repo/cli`
- [x] Implement `goal` commands (`open`, `close`, `reopen`, `list`, `tree`)
- [x] Update `Ticket` struct to support `Goal` and `ParentTicket`
- [x] Update `ticket open` command to support `--goal` and `--parent`
- [x] Implement GitHub milestone synchronization for goals
- [x] Update documentation (`AGENTS.md`)

## Changes

## Log

## Summary

Implemented comprehensive refactoring of the ticket and goal mechanisms:

**Iteration 1 - Ticket Plan Handling:**

- Modified CreateTicket to not create empty plan.md when no planPath provided
- Plan files are now moved to ticket folder preserving original filename
- Removed plan_ITERATION.md file creation

**Iteration 2 - Author Lookup:**

- Fixed GetGitAuthorGithub to use ListContributors() to match contributor by email
- Returns GitHub username if email matches a contributor, otherwise falls back to "NAME <EMAIL>" format

**Iteration 3 - Test Cleanup:**

- Added cleanup to TestTicketOpenContinueKeyword using defer os.RemoveAll()

**Iteration 4 - Title Validation:**

- Modified title validation to only throw if title exactly equals lowercase or uppercase slug
- Added tests with cleanup and noGithub flag

**Iteration 5 - Flexible CLI Argument Parsing:**

- Added extractLLMFromArgs and extractUIFromArgs helper functions
- Added addLLMFlags and addUIFlags for consistent flag handling
- Updated ticket open/reopen commands to support positional, boolean, and named flags

**Iteration 6 - MCP Output Format:**

- Added toolResultToMCP helper function
- Modified MCP ticket functions to use Tool\* functions instead of JSON GraphQL responses
- Output now matches CLI-style format

**Iteration 7 - Goal Requirements:**

- Updated GoalCreateInput struct with required fields: title, description, prompt, dueDate, LLM, UI
- Added validation for all required fields in GoalCreate
- Added ToolGoalCreate and ToolGoalList functions
- Updated goal CLI command with flexible argument parsing
- Added comprehensive tests: TestGoalCreateValidation, TestGoalCreateAndCleanup, TestGoalList
