# Previously

The repo CLI had multiple output modes (--json, plain text), dry-run support, and interactive flags. This created complexity and inconsistency.

# Plan

1. Remove --json, --dry-run, --scope global flags from repo CLI
2. Make all commands output JSON by default for programmatic consumption
3. Simplify ToolFix to always apply fixes (no dry-run mode)
4. Update MCP server to call repo binary instead of importing tools package
5. Update VSCode extension with interactive wizards:
   - pickTicket() - QuickPick to select from ticket list
   - pickPolicy() - QuickPick to select from policy list
   - Add semio.ticketOpen command to open ticket file directly
6. Update tests for new commands
7. Update documentation in AGENTS.md

# Changes

## go/repo/main.go

- Removed global flags: `jsonOutput`, `scopeFlag`, `dryRun`
- All commands now output JSON via `outputResult()`
- `ToolFix()` signature changed from `(string, bool)` to `(string)`
- Commands take scope as positional argument instead of flag
- `ticket list` now takes optional year/month/day positional args

## go/mcp/main.go

- Removed import of non-existent `tools` package
- Added `runRepo()` helper to execute repo binary
- All handlers now call repo binary and return JSON output
- Removed `dry_run` parameter from fix tool

## js/vscode/extension.ts

- Added `runRepoCommandJson<T>()` for parsing JSON from repo commands
- Added `ToolResult`, `TicketData`, `PolicyData` interfaces
- Added `pickTicket()` - interactive QuickPick for ticket selection
- Added `pickPolicy()` - interactive QuickPick for policy selection
- Added `semio.ticketOpen` command to open ticket file in editor
- Updated ticket commands to use picker instead of manual date/slug input
- Updated policy check command to use picker

## js/vscode/extension.test.ts

- Added test for `semio.ticketOpen` command registration

## AGENTS.md

- Updated Repo CLI architecture description
- Documented JSON output structure
- Removed --json, --dry-run, --scope flags from documentation
- Updated command syntax for ticket operations
