---
goal: AI-OPTIMIZED-REPO
---

# Ticket

## Summary

Extend CLI with `hook` and `configure` commands. Implement git hooks (commit.starting/ended) and agent hooks (agent.starting/ended, prompt.submit, compacting, tool.calling/ended, code.reading/edited, notification). Tool call blocking for `git checkout`/`git stash`. Auto-configure pre-commit and all agent clients.

## Changes

- `repo/cli/main.go`: Added Hooks region (HookEvent types, HookContext, HookResult, BlockedToolPatterns, IsToolBlocked, RunHook, hookCommand) and Configure region (configureCommand, configureGitHooks, getClientHookMappings, generateCopilotConfig, generateCursorConfig, generateWindsurfConfig, generateClaudeCodeConfig, generateCodexConfig, generateDroidConfig, generateAntigravityConfig)
- `repo/cli/main_test.go`: Extended with 19 hook/configure tests (TestValidateHookEvent, TestHookEventKind, TestIsToolBlocked, TestRunHookAgentEvents, TestRunHookToolBlocking, TestRunHookToolAllowed, TestRunHookUnknownEvent, TestAllHookEventsCompleteness, TestHookCommandCLI, TestHookCommandToolBlocking, TestHookCommandJSONOutput, TestGenerateCopilotConfig, TestGenerateCursorConfig, TestGenerateWindsurfConfig, TestGenerateClaudeCodeConfig, TestGenerateCodexConfig, TestConfigureGitHooks, TestGetClientHookMappings, TestBlockedToolPatterns)
- `package.json`: Updated pre-commit to use `repo/cli/cli hook commit.starting`, removed old `pre-commit:global:install`, added `configure` script
- `.devcontainer/post-create.sh`: Updated GitHooks region to use `./repo/cli/cli configure`

## Log

- Researched existing CLI structure, command patterns, client list, MCP tools
- Identified insertion points for hook and configure commands
- Implemented HookEvent types, HookContext, HookResult, BlockedToolPatterns
- Implemented hookCommand with tool blocking support
- Implemented configureCommand that generates configs for all 7 clients + git hooks
- Updated package.json and devcontainer post-create to use new hook system
- Added 19 tests, all passing
- Build succeeds

## Todos

- [x] Research CLI and repo structure
- [x] Open ticket
- [x] Implement hook types and hookCommand in main.go
- [x] Implement configureCommand in main.go
- [x] Update pre-commit git hook setup
- [x] Create agent hook configs for all clients
- [x] Implement tool call blocking
- [x] Add tests (19 tests, all passing)
- [x] Build and verify
- [ ] Close ticket

## Plan

1. Add Hooks region in main.go with event types, hook context, and hook handler
2. Add hookCommand (cobra) that handles `hook <event> <client>` dispatching
3. Add configureCommand that generates all agent configs and git hook configs
4. Implement tool call blocking in hook handler (deny git checkout/stash)
5. Generate client-specific configs: .github/copilot-instructions.md, .cursor/rules, .windsurfrules, CLAUDE.md hooks section
6. Update pre-commit to use CLI hook command
7. Add tests for hook and configure commands
8. Build and verify everything works
