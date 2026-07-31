---
goal: R26-03
---

# Ticket

## Summary

Fixed tool.searching patterns to display line numbers correctly in CLI (e.g., `#L532` for lines and `#L532-L771` for ranges).

## Changes

- Extracted `startLine` and `endLine` from `tool_input` in `extractCodeSearchFromInput` inside `repo/cli/main.go`.
- Added logic to append `#L[start]` or `#L[start]-L[end]` formatting to the query string if the tool input contains line numbers.
- Added test coverage in `TestExtractCodeSearchFromInputLineNumbers` in `repo/cli/main_test.go`.

## Log

## Todos

## Plan
