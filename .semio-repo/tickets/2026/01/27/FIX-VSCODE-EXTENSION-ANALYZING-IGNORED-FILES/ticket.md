# Ticket

## Todos

- [x] Find where analyzeFile is called in extension.ts
- [x] Understand how files are selected for analysis
- [x] Add ignore patterns matching repo binary behavior

## Changes

- `js/vscode/extension.ts` - Added `ignoredDirectories` set and `allowedDotDirectories` set with `isInIgnoredDirectory()` function that matches the repo binary behavior

## Log

1. Found `analyzeFile()` function at line 1125 which calls the repo binary to analyze files
2. Found `shouldAnalyzeFile()` only checks language ID, not file path
3. Repo binary (`./semio-repo/cli/main.go:5787-5788`) skips directories starting with `.` and uses .gitignore patterns
4. Added `isInIgnoredDirectory()` function that:
   - Checks against explicit ignore list (node_modules, **pycache**, site-packages, etc.)
   - Skips any directory starting with `.` unless explicitly allowed (.github, .devcontainer, .semio-repo)
5. Added check in `analyzeFile()` to return early if file is in ignored directory

## Summary

Added ignore patterns to VS Code extension matching repo binary behavior - skips dot-prefixed directories and common dependency/build folders
