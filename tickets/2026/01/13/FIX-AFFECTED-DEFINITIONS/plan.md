# Plan

## Problem Analysis
The affected definitions list in ticket file metrics includes local variables (like `result`, `affectedDefs`, `diff`, `currentFile`) in addition to actual top-level function definitions.

Root cause in `go/repo/repo.go`:
- The Go language `definitionRegexp` (line 1062) is `(?:^|\s)(?:func|type|var|const)\s+...`
- The `(?:^|\s)` pattern matches both start-of-line AND after-whitespace, which means it matches indented local variable declarations like `var result = ...`

## Solution
Change the Go language `definitionRegexp` to only match declarations at the start of a line (column 0), which in Go signifies top-level declarations:
- Change `(?:^|\s)` to `^`
- This ensures only `func`, `type`, `var`, `const` at the beginning of a line are matched

Adjust section line metrics so removed lines are attributed using base-commit section ranges while added lines keep current section ranges, and keep affected definitions driven by added lines in the current file.

## Files to Modify
- `go/repo/repo.go` - line 1062, update `definitionRegexp`
- `go/repo/repo.go` - ticket section line metrics
- `README.md` and `AGENTS.md` - ticket metrics documentation
