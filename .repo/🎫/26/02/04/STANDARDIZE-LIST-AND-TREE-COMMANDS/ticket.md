# Ticket

## Summary

## Changes

## Log

- Re-established context for `STANDARDIZE-LIST-AND-TREE-COMMANDS` and confirmed ticket workspace location.
- Mapped where GraphQL execution is invoked (`gql(...)` wrapper) and where schema is built (`buildSchema(...)`) to prepare for shared ID/URI and standardized list rendering refactor.
- Fixed `TestLifecycleCommands` goal handling: create a unique goal, capture returned `goalID`, pass it into `ticket open`, and cleanup only that goal directory (removed unsafe deletion of all goals).
- Ran `go test ./...` (green).
- Attempted `ticket reopen 2026/02/04/STANDARDIZE-LIST-AND-TREE-COMMANDS` to follow workflow; command returned `ticket is already open` (expected).
- Refactored CLI list rendering: removed list headers (`found N ...`) for repo list outputs and standardized human list lines to start with the semantic `<id>` and follow with properties separated by spaces.
- Updated list rendering to handle `repo.goals` in the same flat list pipeline as other repo list kinds.
- Cleaned up stray `DEBUG: Markdown=...` output in `tree` command.
- Ran `go test ./...` (green).
- Standardized `--md` list output further by removing remaining headings for nested file sections/definitions and for todo/draft lists.
- Improved section ID/URI derivation to be stable when GraphQL does not provide `path` by deriving `filePath#section` forms.
- Added TTY-only terminal width truncation for human list lines that preserves ANSI color sequences.
- Implemented human list property colorization by position (property1/property2/...) in `renderEntityHuman`.
- Ran `go test ./...` (green).
- Removed stray `DEBUG: Markdown=...` stdout logging from `tree` command.
- Restored `goal` due-date rendering in human output and fixed `ticket` created-date extraction to read `dates.created`.
- Ran `go test ./...` (green).

## Todos

- Reopen/confirm active ticket context and keep `ticket.md` updated while working.
- Unify ID/URI generation behind one implementation and ensure GraphQL uses `id` while MCP uses `uri`.
- Standardize all `list` outputs (human + `--md`) to the specified formats with property coloring and width truncation.
- Extend existing Go tests (`repo/cli/main_test.go`) to cover ID/URI + list output behavior.
- Update root `README.md` and `AGENTS.md` documentation sections for the new mechanism.

## Plan

1. Centralize ID/URI generation (one shared implementation) and update all producers/consumers to use it.
2. Standardize `list` output formatting across commands and renderers (terminal + markdown) with truncation.
3. Extend existing tests and run the suite until green.
4. Update dev docs (`README.md`, `AGENTS.md`) to reflect the new system.
