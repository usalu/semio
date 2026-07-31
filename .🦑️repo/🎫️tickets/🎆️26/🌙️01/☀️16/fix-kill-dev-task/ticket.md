# Ticket

## Todos

# Plan: Fix VS Code 'kill dev' Task

## Problem

The VS Code task "kill dev" in `.vscode/tasks.json` has an empty `command` property in the Windows block and no default command at the top level. This causes VS Code to show an error: "the task 'kill dev' neither specifies a command nor a dependsOn property".

## Solution

Add a valid command to the task. Since the task has a TODO comment indicating the feature is not yet fully implemented, add a placeholder echo command that:

1. Makes VS Code recognize the task as valid
2. Informs the user that the feature is not yet implemented
3. Provides cross-platform compatibility (Linux and Windows)

## Steps

1. Add a top-level `command` with echo message for Linux/default
2. Update Windows block with a PowerShell echo command
3. Verify the JSON is valid

## Files to modify

- `.vscode/tasks.json`

## Changes

## Log

# Log: Fix VS Code 'kill dev' Task

## 2026-01-16

### Investigation

- Found the "kill dev" task at lines 1128-1147 in `.vscode/tasks.json`
- The task has a Windows-specific block with an empty command: `"command": ""`
- There's a TODO comment: `// TODO: Find a reliable way to kill all dev tasks`
- No default/top-level command is specified
- VS Code requires either a `command` or `dependsOn` property for tasks

### Fix Applied

- Added a top-level `command` property with an echo message for Linux/default systems
- Updated the Windows block with a non-empty PowerShell Write-Host command
- Both commands inform the user that the feature is not yet implemented

### Follow-up: Remove Windows legacy configs

- User requested removal of all Windows-specific legacy configurations (exe, powershell, etc.)
- Only Windows dev environment is Compose.Grasshopper
- Removing the entire "kill dev" task as it was a Windows-only placeholder
- The task was never properly implemented and is not needed for the current dev environment

### Scanning monorepo for Windows legacy configs

- Searching for: powershell, .exe, cmd.exe patterns

Found Windows legacy configs:

**To remove (not needed for Linux dev):**

1. `.vscode/settings.json` - `python.defaultInterpreterPath.windows`
2. `.vscode/mcp.json` - `go/mcp/mcp.exe` (needs Linux path)
3. `.vscode/launch.json` - PowerShell launch configurations
4. `assets/logo/package.json` - PowerShell build script
5. `assets/icons/package.json` - PowerShell build script

**Legitimate Windows-only (Compose.Grasshopper - keep):**

- `net/Compose.Grasshopper/` - All .exe references for Rhino/Grasshopper
- `net/Compose/build.ts` - MSBuild.exe for .NET builds
- `yak/` folder - Yak.exe for Rhino package management
- `py/engine/post-build.ts` - .exe references for cross-platform distribution

**Code patterns (not configs - keep):**

- `.exec()`, `.execute()`, `.executeCommand()` - These are method calls, not Windows configs

### Changes Made

1. `.vscode/tasks.json` - Removed "kill dev" task
2. `.vscode/settings.json` - Removed `python.defaultInterpreterPath.windows` and `python.defaultInterpreterPath.linux` (keeping only the default)
3. `.vscode/mcp.json` - Changed `go/mcp/mcp.exe` to `go/mcp/mcp`
4. `.vscode/launch.json` - Removed "build icons" PowerShell configuration and generic "PowerShell" configuration
5. `assets/logo/package.json` - Changed build script from PowerShell to `tsx logo.ts`
6. `assets/icons/package.json` - Replaced broken PowerShell build script with placeholder echo
7. `package.json` - Changed all `repo.exe` references to `repo`
8. `CLAUDE.md` - Changed Windows paths (`.\go\repo\repo.exe`) to Linux paths (`./repo/cli/cli`)
9. `AGENTS.md` - Changed Windows paths (`.\go\repo\repo.exe`) to Linux paths (`./repo/cli/cli`)
10. `.claude/settings.local.json` - Removed Windows-specific permission entries
11. `prompts/ueli.md` - Updated Windows paths to Linux paths, marked PowerShell migration as done
12. `js/vscode/extension.test.ts` - Made repo binary path detection cross-platform

## Summary

# Summary: Fix VS Code 'kill dev' Task & Remove Windows Legacy Configs

## Problem

1. The VS Code task "kill dev" in `.vscode/tasks.json` was causing an error due to an empty Windows command
2. Multiple Windows-specific legacy configurations (PowerShell, .exe paths) were present throughout the codebase

## Solution

Removed all Windows legacy configurations since the only Windows dev environment is Compose.Grasshopper.

## Files Modified

- `.vscode/tasks.json` - Removed the "kill dev" task
- `.vscode/settings.json` - Removed Windows-specific Python interpreter path
- `.vscode/mcp.json` - Changed `mcp.exe` to `mcp` (Linux executable)
- `.vscode/launch.json` - Removed PowerShell launch configurations
- `assets/logo/package.json` - Changed build script from PowerShell to TypeScript
- `assets/icons/package.json` - Replaced broken PowerShell build script with placeholder
- `package.json` - Changed all `repo.exe` to `repo`
- `CLAUDE.md` - Changed Windows paths to Linux paths
- `AGENTS.md` - Changed Windows paths to Linux paths
- `.claude/settings.local.json` - Removed Windows-specific permissions
- `prompts/ueli.md` - Updated Windows paths to Linux paths
- `js/vscode/extension.test.ts` - Made repo binary path detection cross-platform

## Not Modified (Legitimate Windows-only for Compose.Grasshopper)

- `net/Compose.Grasshopper/` - Rhino/Grasshopper paths
- `net/Compose/build.ts` - MSBuild.exe for .NET builds
- `yak/` folder - Yak.exe for Rhino package management
