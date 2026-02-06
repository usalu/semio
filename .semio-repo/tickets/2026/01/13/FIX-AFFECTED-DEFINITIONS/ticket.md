# Ticket

## Todos

# Plan

## Problem Analysis

The affected definitions list in ticket file metrics includes local variables (like `result`, `affectedDefs`, `diff`, `currentFile`) in addition to actual top-level function definitions.

Root cause in `./semio-repo/cli/cli.go`:

- The Go language `definitionRegexp` (line 1062) is `(?:^|\s)(?:func|type|var|const)\s+...`
- The `(?:^|\s)` pattern matches both start-of-line AND after-whitespace, which means it matches indented local variable declarations like `var result = ...`

## Solution

Change the Go language `definitionRegexp` to only match declarations at the start of a line (column 0), which in Go signifies top-level declarations:

- Change `(?:^|\s)` to `^`
- This ensures only `func`, `type`, `var`, `const` at the beginning of a line are matched

Adjust section line metrics so removed lines are attributed using base-commit section ranges while added lines keep current section ranges, and keep affected definitions driven by added lines in the current file.

## Files to Modify

- `./semio-repo/cli/cli.go` - line 1062, update `definitionRegexp`
- `./semio-repo/cli/cli.go` - ticket section line metrics
- `README.md` and `AGENTS.md` - ticket metrics documentation

## Changes

## Log

# Log

## Iteration 1: Fix local variable matching

### Analysis

The Go language `definitionRegexp` uses `(?:^|\s)` which matches both start-of-line AND after-whitespace. This causes local variable declarations (which are indented) to be matched as definitions.

### Implementation

Changed the Go `definitionRegexp` from:

```
(?:^|\s)(?:func|type|var|const)\s+(?:\([^)]+\)\s+)?([A-Za-z_][A-Za-z0-9_]*)
```

to:

```
^(?:func|type|var|const)\s+(?:\([^)]+\)\s+)?([A-Za-z_][A-Za-z0-9_]*)
```

This ensures only definitions at column 0 (top-level) are matched.

## Iteration 2: Fix false positives from removed lines

### Analysis

Still too many definitions being marked as affected. The issue is that removed lines reference OLD file line numbers, but we were comparing them against NEW file definition ranges. This caused definitions that weren't actually modified to be incorrectly included.

### Implementation

Changed `computeAffectedSections` to only use ADDED lines (which correctly reference new file positions) for determining affected definitions. Removed lines are still counted for the section total but not used for definition matching.

## Iteration 3: Attribute removed lines to base sections

### Analysis

Removed line totals were being mapped to current file section ranges, which can shift after edits. This misattributed deletions across sections (e.g. removals in Schema Builder showing under GraphQL Executor).
User report: Schema Builder removed 10 lines and added 6.

### Implementation

Computed removed line metrics using base commit section ranges via `git show`, while added line metrics and affected definitions continue using current file sections.

## Summary

# Summary

Fixed issues causing incorrect affected definitions and section line attribution in ticket file metrics:

1. **Local variables**: Updated Go `definitionRegexp` to only match at column 0 (top-level declarations)
2. **False positives from removed lines**: Changed `computeAffectedSections` to only use added lines for determining affected definitions, since removed lines reference old file positions that don't map to new file definition ranges
3. **Removed line attribution**: Mapped removed line totals using base-commit section ranges while keeping added lines and affected definitions tied to current sections
