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
