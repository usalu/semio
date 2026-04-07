---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT
---

# Ticket

## Summary

All --json CLI commands now return pure data without {"data": ...} GraphQL wrapper. Errors emitted to stderr. Cobra SilenceUsage/SilenceErrors set. Wrong-argument tests added for all command categories. TestCliJsonPureData and TestCliJsonErrorsToStderr verify output contract. Full test suite passes.

## Changes

- `repo/cli/main.go`: Removed `{"data": result}` wrapper from `Engine.runGraphQL`, removed corresponding unwrap logic in `formatResult` and `formatMarkdownResult`, set `SilenceUsage`/`SilenceErrors` on root cobra command.
- `repo/cli/main_test.go`: Updated `executeCommand` to separate stdout/stderr (3 return values). Replaced `findFirstResultData`/`parseJSONOutput`/`hasExitCode`/`mustHaveExitCode` with `firstJSONLine`. Updated parse helpers to not expect `{"data": ...}` wrapper. Updated all callers. Added 13 wrong-argument test functions covering ticket, goal, policy, folder, file, section, definition, contributor, and graphql commands. Added `TestCliJsonPureData` and `TestCliJsonErrorsToStderr` tests.

## Log

- Explored codebase, identified `Engine.runGraphQL` wrapping result in `{"data": result}` as the source of extra wrapping.
- Identified `formatResult` and `formatMarkdownResult` unwrapping `{"data": ...}` as downstream consumers.
- Removed wrapper from engine, removed unwrap from renderers.
- Updated `executeCommand` test helper to separate stdout/stderr buffers.
- Replaced event-based test helpers with pure data line parsing.
- Updated all E2E test callers to use 3-return `executeCommand`.
- Added `SilenceUsage`/`SilenceErrors` to cobra root command.
- Added wrong argument tests for all command categories.
- Added `TestCliJsonPureData` (verifies no event wrappers or `{"data": ...}` in JSON output).
- Added `TestCliJsonErrorsToStderr` (verifies stdout is empty on errors).
- Full test suite: ALL PASS (230s).

## Todos

- [x] Remove `{"data": ...}` wrapper from `Engine.runGraphQL`
- [x] Remove `{"data": ...}` unwrapping in `formatResult` and `formatMarkdownResult`
- [x] Update `executeCommand` test helper to separate stdout/stderr
- [x] Update test parse helpers to not expect `{"data": ...}` wrapper
- [x] Fix all callers of `executeCommand` and remove legacy helpers
- [x] Update `TestLifecycleCommands` to parse pure data lines
- [x] Add tests with semantically wrong arguments for all commands
- [x] Run full test suite - ALL PASS
- [x] Update ticket.md, AGENTS.md, README.md and close ticket

## Plan

1. Remove `{"data": ...}` wrapper from `Engine.runGraphQL` (line 225 in main.go)
2. Remove `{"data": ...}` unwrapping in `formatResult` and `formatMarkdownResult`
3. Update `executeCommand` test helper to separate stdout/stderr
4. Update test parse helpers (`parseTicketOpenResult`, `parseGoalCreateID`, `parseTicketCloseStatus`, `parseTicketReopenStatus`) to not expect `{"data": ...}` wrapper
5. Add tests with semantically wrong arguments for all CLI commands
6. Run all tests, fix failures
7. Update ticket.md and close ticket
