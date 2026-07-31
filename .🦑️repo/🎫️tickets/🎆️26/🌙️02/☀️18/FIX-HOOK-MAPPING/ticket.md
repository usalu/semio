---
goal: HOOKS
---

# Ticket

## Summary

All platform hook config generators now include ALL available events with NO matchers. Added native format generators for codex/droid/antigravity with all 13 agent events. configure command now writes 10 client configs + git hooks. All tests pass.

## Plan

1. Fix `generateCopilotConfig`: Add `SessionEnd`, `PostToolUseFailure`, `TaskCompleted`
2. Fix `generateClaudeCodeConfig`: Change `TaskCompleted` matcher from `"Task"` to `""`, remove `hookEntryWithMatcher`
3. Fix `generateDroidConfig`: Change `TaskCompleted` matcher from `"Task"` to `""`, add `SubagentStart`, `PostToolUseFailure`, remove `hookEntryWithMatcher`
4. Fix `generateCodexConfig`: Add `SubagentStart`, `SubagentStop`, `PostToolUseFailure`, `TaskCompleted`
5. Fix `generateAntigravityConfig`: Add `SubagentStart`, `SubagentStop`, `PostToolUseFailure`, `TaskCompleted`
6. Add `generateCodexNativeConfig`, `generateDroidNativeConfig`, `generateAntigravityNativeConfig` for hooks.json (repo event names as keys)
7. Update `getClientHookMappings` to include native format entries
8. Update all tests to match
9. Run configure command to regenerate all config files

## Todos

- [x] Fix generateCopilotConfig
- [x] Fix generateClaudeCodeConfig
- [x] Fix generateDroidConfig
- [x] Fix generateCodexConfig
- [x] Fix generateAntigravityConfig
- [x] Add native format generators
- [x] Update getClientHookMappings
- [x] Update tests
- [x] Run configure command
- [x] Run tests to verify

## Changes

- `repo/cli/main.go`: Fixed all 5 generators, added 3 native generators, updated mappings
- `repo/cli/main_test.go`: Updated all generator tests
- `.github/hooks/repo.json`: Regenerated with all events
- `.claude/settings.json`: Regenerated with no matcher on TaskCompleted
- `.cursor/hooks.json`: Regenerated
- `.windsurf/hooks.json`: Regenerated
- `.factory/settings.json`: Regenerated with no matcher on TaskCompleted
- `.codex/settings.json`: Regenerated with all events
- `.antigravity/settings.json`: Regenerated with all events
- `.codex/hooks.json`: Regenerated with all native events
- `.droid/hooks.json`: Regenerated with all native events
- `.antigravity/hooks.json`: Regenerated with all native events
