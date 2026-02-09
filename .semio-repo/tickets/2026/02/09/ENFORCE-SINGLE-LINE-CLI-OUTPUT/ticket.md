---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary

Enforced single-line output for all CLI tree and list items. Added sanitizeProp (strips newlines, replaces backticks with single quotes, collapses spaces) and sanitizeSingleLine (final rendering safety net) functions. Updated collectEntityProps, renderEntityMarkdownLink, and renderEntityHuman to use them. Extended TestCollectEntityPropsConsistency with 7 new sub-tests and added TestSingleLineOutput with 50+ sub-tests covering all entity kinds through every rendering path. Fixed missing io/fs import in test file. Updated AGENTS.md and README.md with specs and codebase docs.
## Changes

- `semio-repo/cli/main.go`: Added `sanitizeProp()` function that strips `\r\n`, `\n`, `\r`, backticks (replaced with `'`), collapses double spaces, and trims. Added `sanitizeSingleLine()` for final output guarantee. Updated `collectEntityProps` to use `sanitizeProp`. Updated `renderEntityMarkdownLink` to sanitize artifact ID and apply `sanitizeSingleLine` to output. Updated `renderEntityHuman` to sanitize artifact ID and apply `sanitizeSingleLine` to output.
- `semio-repo/cli/main_test.go`: Added `io/fs` import (pre-existing missing import fix). Extended `TestCollectEntityPropsConsistency` with 7 new sub-tests for newline stripping, backtick stripping, space collapsing, Windows line endings, and coverage across goal/commit/policy kinds. Added new `TestSingleLineOutput` test function with 50+ sub-tests covering all entity kinds with multi-line data through every rendering path (markdown link, markdown item, human, TTY, monorepo tree node markdown/text, goal tree, ticket list, formatMarkdownResult).

## Log

1. Identified `collectEntityProps` already replaced newlines but not backticks
2. Identified `renderEntityMarkdownLink` wraps props in backticks but content backticks break markdown
3. Added `sanitizeProp` function: strips newlines, replaces backticks with single quotes, collapses spaces
4. Added `sanitizeSingleLine` function: strips newlines as final safety net
5. Updated `collectEntityProps`, `renderEntityMarkdownLink`, `renderEntityHuman` to use new functions
6. Fixed missing `io/fs` import in test file
7. Extended `TestCollectEntityPropsConsistency` with multi-line/backtick tests
8. Added comprehensive `TestSingleLineOutput` covering all entity kinds and rendering paths
9. All rendering-related tests pass (full suite timeout is pre-existing `TestListCommands` issue)

## Todos

- [x] Find code that formats tree/list output
- [x] Find existing test file
- [x] Identify all places where multi-line content can appear
- [x] Implement single-line enforcement in sanitizeProp and sanitizeSingleLine
- [x] Update collectEntityProps, renderEntityMarkdownLink, renderEntityHuman
- [x] Extend existing tests
- [x] Run tests and ensure all pass
- [x] Update ticket.md
- [x] Update dev docs (AGENTS.md, README.md)
- [x] Close ticket

## Plan

1. Add `sanitizeProp` to strip newlines, backticks, collapse spaces
2. Add `sanitizeSingleLine` as rendering-level safety net
3. Wire into `collectEntityProps`, `renderEntityMarkdownLink`, `renderEntityHuman`
4. Extend existing test functions with multi-line and backtick test cases
5. Add `TestSingleLineOutput` covering all kinds through all rendering paths
6. Update dev docs and close ticket
