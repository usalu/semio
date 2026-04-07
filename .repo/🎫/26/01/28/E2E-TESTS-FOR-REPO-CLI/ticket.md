# Ticket

## Todos

- [x] Analyze ./repo/cli/main.go for commands and flags
- [x] Extend ./repo/cli/main_test.go with helper functions for CLI testing
- [x] Implement E2E tests for `ticket open` (positional, flags, named values)
- [x] Implement E2E tests for `ticket close`
- [x] Implement E2E tests for `ticket reopen`
- [x] Implement E2E tests for `ticket list`
- [x] Implement E2E tests for `goal` commands
- [x] Implement E2E tests for `folder` commands
- [x] Implement E2E tests for `file` commands
- [x] Implement E2E tests for `policy` commands
- [x] Implement E2E tests for `project` (bundle) commands
- [x] Ensure all tests cleanup artifacts
- [x] Verify no GitHub interaction during tests

## Changes

- GraphQL `node(id:)` accepts the canonical ID formats emitted by the schema (`semio/...`, `repo/...`) and keeps a fallback for legacy `kind:id` IDs.
- E2E ticket-close tests use a non-gitignored file path so ticket file filtering does not drop the list to empty.
- E2E ticket status parsing normalizes status values for case-insensitive comparisons.

## Log

- `go test ./...` (./repo/cli): PASS

## Summary

E2E CLI E2E tests stabilized; node(id:) accepts canonical IDs; ticket close file filtering handled; all ./repo/cli tests pass.
