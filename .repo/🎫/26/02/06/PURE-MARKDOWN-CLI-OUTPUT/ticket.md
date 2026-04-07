---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI
---

# Ticket

## Summary

Eliminated all JSON/NDJSON fallback from markdown and text CLI output. All commands return pure markdown or human-readable text via generic inferEntityKind dispatch. Added renderEntityMarkdownLink, formatMarkdownFile, isList streaming detection. Comprehensive unit tests (130+ subtests) covering all entity kinds, mutations, lists, and edge cases.

## Changes

- `repo/cli/cli.go`: Added `inferEntityKind()`, `renderEntityMarkdownLink()`, `formatMarkdownFile()`. Rewrote `formatMarkdownResult()` and `formatResult()` to use generic entity-kind dispatch with no JSON fallback. Added `isList` streaming detection for list/tree commands.
- `repo/cli/cli_test.go`: Added `assertValidMarkdownLink`, `TestFormatMarkdownResult_MutationKeys` (21), `TestFormatMarkdownResult_SingleEntities` (12), `TestFormatMarkdownResult_Lists` (12), `TestFormatMarkdownResult_Analyze`, `TestFormatMarkdownResult_Fix`, `TestFormatMarkdownResult_FileWithSections`, `TestFormatMarkdownResult_NoJSONFallback`, `TestFormatResult_MutationKeys` (12), `TestRenderEntityMarkdownLink_AllKinds` (15), `TestInferEntityKind` (27).

## Log

- Analyzed 27,000-line cli.go renderer architecture (NDJSONRenderer, HumanRenderer, MarkdownRenderer)
- Identified GraphQL mutation result keys falling through to JSON code-block fallback
- Implemented `inferEntityKind()` mapping operation names to entity kinds via prefix matching
- Implemented `renderEntityMarkdownLink()` with switch cases for all 15 entity kinds
- Rewrote `formatMarkdownResult()` eliminating all JSON fallback paths
- Rewrote `formatResult()` (HumanRenderer) eliminating JSON dump fallback
- Fixed streaming list regression: items lacked `- ` prefix because each arrives as individual single-entity event
- Added `isList` detection (`strings.HasSuffix(command, " list") || " tree"`)
- Added comprehensive unit tests (130+ subtests)
- Pre-existing `goal tree` failure (author deserialization bug) not caused by these changes

## Todos

- [x] Add `inferEntityKind()` function
- [x] Add `renderEntityMarkdownLink()` function
- [x] Rewrite `formatMarkdownResult()` - no JSON fallback
- [x] Add `formatMarkdownFile()` helper
- [x] Rewrite `formatResult()` (HumanRenderer) - no JSON dump
- [x] Fix streaming list `- ` prefix regression
- [x] Add comprehensive unit tests
- [x] Verify all tests pass

## Plan

1. Analyze rendering architecture and identify all JSON fallback paths
2. Create generic `inferEntityKind` for mutation key → entity kind mapping
3. Create `renderEntityMarkdownLink` for consistent `[id](uri) - props` format
4. Rewrite `formatMarkdownResult` to use generic dispatch, no JSON
5. Rewrite `formatResult` to use generic dispatch, no JSON dump
6. Fix streaming list item prefix issue
7. Add comprehensive unit tests for all paths
