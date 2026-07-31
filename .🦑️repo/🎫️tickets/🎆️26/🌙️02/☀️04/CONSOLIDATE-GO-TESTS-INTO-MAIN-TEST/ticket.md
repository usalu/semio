# Ticket

## Summary

Consolidated repo-tooling Go tests into repo/cli/main_test.go, removed redundant \_test.go files, fixed path/root assumptions in folder/file tool tests, and verified go test ./... passes.

## Changes

- Consolidated all Go tests under `repo/cli/main_test.go`.
- Updated folder/file tool tests to resolve repo root deterministically and use stable existing paths.
- Updated developer docs to reflect the consolidated test organization.

## Log

- Moved test functions from `format_test.go`, `format_more_test.go`, `main_commands_test.go`, and `nogithub_test.go` into `repo/cli/main_test.go`.
- Deleted redundant test files to avoid redeclarations.
- Fixed failing folder/file tool tests by setting repo root reliably and using `repo/...` paths.
- Relaxed `update` command assertion to avoid expecting JSONL done events from non-streaming operational commands.
- Verified `go test ./...` passes in `repo/cli`.
- Verified `go test` at repo root must target modules listed in `go.work` (e.g. `./repo/cli/...`).
- Noted `compose/go` tests currently fail due missing `assets/compose/*` fixtures; this is unrelated to the repo-tooling test consolidation.

## Todos

- [x] Consolidate Go tests into `repo/cli/main_test.go`.
- [x] Ensure `go test ./...` passes for `repo/cli`.
- [x] Update `README.md` and `AGENTS.md` with consolidated test organization.
- [x] Close ticket via `repo ticket close`.

## Plan

1. Consolidate Go tests into `repo/cli/main_test.go` and remove redundant files.
2. Fix any newly surfaced test failures and re-run Go tests.
3. Update dev docs and close the ticket.
