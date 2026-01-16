# Summary: Fix VS Code 'kill dev' Task & Remove Windows Legacy Configs

## Problem
1. The VS Code task "kill dev" in `.vscode/tasks.json` was causing an error due to an empty Windows command
2. Multiple Windows-specific legacy configurations (PowerShell, .exe paths) were present throughout the codebase

## Solution
Removed all Windows legacy configurations since the only Windows dev environment is Semio.Grasshopper.

## Files Modified
- `.vscode/tasks.json` - Removed the "kill dev" task
- `.vscode/settings.json` - Removed Windows-specific Python interpreter path
- `.vscode/mcp.json` - Changed `mcp.exe` to `mcp` (Linux executable)
- `.vscode/launch.json` - Removed PowerShell launch configurations
- `assets/logo/package.json` - Changed build script from PowerShell to TypeScript
- `assets/icons/package.json` - Replaced broken PowerShell build script with placeholder

## Not Modified (Legitimate Windows-only for Semio.Grasshopper)
- `net/Semio.Grasshopper/` - Rhino/Grasshopper paths
- `net/Semio/build.ts` - MSBuild.exe for .NET builds
- `yak/` folder - Yak.exe for Rhino package management
