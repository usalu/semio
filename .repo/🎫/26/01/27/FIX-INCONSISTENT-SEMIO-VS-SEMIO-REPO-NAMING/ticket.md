# Ticket

## Todos

- [x] Find all occurrences of 'semio' that should be 'repo'
- [x] Fix output channel name
- [x] Fix command prefixes in package.json (39 commands)

## Changes

- `js/vscode/extension.ts` - Changed output channel name from "semio" to "repo"
- `js/vscode/extension.ts` - Updated activation log message
- `js/vscode/package.json` - Changed all 39 command titles from "semio:" to "repo:"

## Log

1. Found output channel created with name "semio" at line 3853
2. Found 39 command titles with "semio:" prefix in package.json
3. Changed output channel name to "repo"
4. Used replace_all to change all command title prefixes to "repo:"

## Summary

Changed output channel and 39 command titles from 'semio' to 'repo' for consistent naming
