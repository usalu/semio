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
- Only Windows dev environment is Semio.Grasshopper
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

**Legitimate Windows-only (Semio.Grasshopper - keep):**
- `net/Semio.Grasshopper/` - All .exe references for Rhino/Grasshopper
- `net/Semio/build.ts` - MSBuild.exe for .NET builds
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
8. `CLAUDE.md` - Changed Windows paths (`.\go\repo\repo.exe`) to Linux paths (`./go/repo/repo`)
9. `AGENTS.md` - Changed Windows paths (`.\go\repo\repo.exe`) to Linux paths (`./go/repo/repo`)
10. `.claude/settings.local.json` - Removed Windows-specific permission entries
11. `prompts/ueli.md` - Updated Windows paths to Linux paths, marked PowerShell migration as done
12. `js/vscode/extension.test.ts` - Made repo binary path detection cross-platform
