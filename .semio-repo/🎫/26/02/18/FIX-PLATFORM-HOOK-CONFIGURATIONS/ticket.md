---
goal: AI-OPTIMIZED-REPO
---

# Ticket

## Summary

Fixed platform hook configurations to use only native hook event names per spec. Removed unsupported platforms (antigravity, codex). Fixed droid to use .factory/hooks.json with simple string dict format. Updated windsurf (removed post_run_command) and cursor (added afterAgentResponse, afterAgentThought, beforeTabFileRead, afterTabFileEdit). All tests pass.
## Plan

Fix platform hook configs to use only native event names. Remove unsupported platforms. Fix droid path.

Platforms and their supported hooks:
- VSCode: `.github/hooks/semio-repo.json` → `SessionStart`, `UserPromptSubmit`, `PreToolUse`, `PostToolUse`, `PreCompact`, `SubagentStart`, `SubagentStop`, `Stop`
- Windsurf: `.windsurf/hooks.json` → `pre_read_code`, `post_read_code`, `pre_write_code`, `post_write_code`, `pre_run_command`, `pre_mcp_tool_use`, `post_mcp_tool_use`, `pre_user_prompt`, `post_cascade_response`, `post_setup_worktree`
- Cursor: `.cursor/hooks.json` → `sessionStart`, `sessionEnd`, `preToolUse`, `postToolUse`, `postToolUseFailure`, `subagentStart`, `subagentStop`, `beforeShellExecution`, `afterShellExecution`, `beforeMCPExecution`, `afterMCPExecution`, `beforeReadFile`, `afterFileEdit`, `beforeSubmitPrompt`, `preCompact`, `stop`, `afterAgentResponse`, `afterAgentThought`, `beforeTabFileRead`, `afterTabFileEdit`
- Claude Code: `.claude/settings.json` → `SessionStart`, `UserPromptSubmit`, `PreToolUse`, `PermissionRequest`, `PostToolUse`, `PostToolUseFailure`, `Notification`, `SubagentStart`, `SubagentStop`, `Stop`, `TeammateIdle`, `TaskCompleted`, `PreCompact`, `SessionEnd`
- Droid: `.factory/hooks.json` → `PreToolUse`, `PostToolUse`, `UserPromptSubmit`, `Notification`, `Stop`, `SubagentStop`, `PreCompact`, `SessionStart`, `SessionEnd`
- Antigravity: no hook support → delete `.antigravity/hooks.json`, delete `.antigravity/settings.json`
- Codex: no hook support → delete `.codex/hooks.json`, delete `.codex/settings.json`

## Todos

- [ ] Update `getClientHookMappings`: remove antigravity/codex entries, change droid path to `.factory/hooks.json`, remove droid-hooks entry
- [ ] Remove `generateDroidNativeConfig`, `generateCodexConfig`, `generateCodexNativeConfig`, `generateAntigravityConfig`, `generateAntigravityNativeConfig` functions
- [ ] Update `generateWindsurfConfig`: remove `post_run_command`
- [ ] Update `generateCursorConfig`: add `afterAgentResponse`, `afterAgentThought`, `beforeTabFileRead`, `afterTabFileEdit`
- [ ] Update `generateDroidConfig`: change path to write `.factory/hooks.json` format (simple string dict format matching droid native events)
- [ ] Update tests: remove obsolete test functions, update cursor/windsurf/droid tests, update TestGetClientHookMappings
- [ ] Delete `.antigravity/hooks.json`, `.antigravity/settings.json`, `.codex/hooks.json`, `.codex/settings.json`, `.droid/hooks.json`, `.factory/settings.json`
- [ ] Build and test
- [ ] Run `configure` to regenerate all config files

## Changes

## Log
