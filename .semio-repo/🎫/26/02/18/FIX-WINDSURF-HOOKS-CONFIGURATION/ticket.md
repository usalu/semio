---
goal: AI-OPTIMIZED-REPO
---

# Ticket

## Summary

Fixed two issues preventing Windsurf hooks from calling semio-repo CLI: (1) Created missing .windsurf/hooks.json with all 10 Windsurf hook events mapped to CLI commands. (2) Fixed exit code 2 bug: hookCommand returned pointer &ExitError{Code:2} but main() used errors.As with value type ExitError, causing match failure and fallthrough to os.Exit(1). Changed to ExitError{Code:2}. (3) Updated generateWindsurfConfig with show_output fields. All 27+ hook tests pass. Blocked commands (git checkout, git stash, git reset --hard, git clean -fd) verified returning exit code 2.
## Changes

- **Created** `.windsurf/hooks.json` — 10 hook events (pre/post read/write code, pre/post run command, pre/post MCP tool use, pre user prompt, post cascade response) mapped to CLI hook commands. `pre_run_command` has `show_output: true`.
- **Fixed** `semio-repo/cli/main.go` line 32975 — Changed `&ExitError{Code: 2}` to `ExitError{Code: 2}` so `errors.As` in `main()` matches and `os.Exit(2)` is called (required by Windsurf to block pre-hook actions).
- **Updated** `semio-repo/cli/main.go` `generateWindsurfConfig` — Added `show_output` fields to all hook entries.

## Log

- Discovered `.windsurf/hooks.json` was missing entirely
- Analyzed `generateWindsurfConfig` and `hookCommand` in CLI source
- Found exit code bug: pointer vs value `ExitError` mismatch
- Verified fix: `git checkout`, `git stash`, `git reset --hard` all return exit code 2
- All 27+ hook tests pass

## Todos

- [x] Create `.windsurf/hooks.json`
- [x] Fix `ExitError` pointer vs value bug
- [x] Update `generateWindsurfConfig` with `show_output` fields
- [x] Rebuild CLI binary
- [x] Verify all tests pass
- [x] Verify blocked commands return exit code 2

## Plan

1. [x] Investigate missing `.windsurf/hooks.json`
2. [x] Analyze CLI hook command and `extractCommandFromStdin`
3. [x] Create hooks.json with correct Windsurf format
4. [x] Fix exit code 2 propagation bug
5. [x] Update generator to match created file
6. [x] Rebuild and test
