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
