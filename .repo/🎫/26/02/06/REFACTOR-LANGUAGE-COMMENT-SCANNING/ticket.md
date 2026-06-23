---
goal: CODEHEALTH/POLICIES
---

# Ticket

## Summary

Refactored comment scanning to be language-agnostic via BaseLanguage.ScanComments with configurable primitives per language. Removed TypeScript-specific ScanComments override. Updated all language constructors with string literal flavors and skip directives. Added 48 per-language test subtests covering Go, Python, C#, TypeScript, Shell, Rust. Fixed fixture header ID formats. All tests pass.

## Plan

1. Add language-specific string literal fields to `BaseLanguage` (templates, raw backticks, triple quotes, verbatim strings, JSDoc, skip directives)
2. Add corresponding state fields to `CommentScanState`
3. Add `SkipDirectives()` method to `LanguagePlugin` interface
4. Move `ScanComments` from `TypeScriptLanguage` to `BaseLanguage` with generic logic
5. Remove TypeScript-specific `ScanComments` override
6. Update all language constructors with new fields
7. Fix `ParseIgnoreDirectives` to be language-aware (use comment prefix)
8. Update `applyAutofixes` inline comment fix to use language-aware region/directive checks
9. Update fixture files (remove stale comments in clean fixtures)
10. Add per-language comment scanning tests in existing test file
11. Run all tests and fix failures
12. Update dev docs

## Todos

- [x] Explore codebase and understand the mechanism
- [x] Add language-specific fields to BaseLanguage (hasJSDoc, hasTemplates, hasRawBackticks, hasTripleQuotes, hasVerbatimStrings, skipDirectives)
- [x] Add corresponding state fields to CommentScanState (InTripleDouble, InTripleSingle, InRawBacktick, InVerbatimString)
- [x] Add SkipDirectives() method to LanguagePlugin interface and BaseLanguage
- [x] Implement generic BaseLanguage.ScanComments replacing no-op
- [x] Remove TypeScriptLanguage.ScanComments override
- [x] Update TypeScript constructor (hasJSDoc, hasTemplates, skipDirectives: eslint-, @ts-, noinspection)
- [x] Update Go constructor (hasRawBackticks, skipDirectives: nolint)
- [x] Update Python constructor (hasTripleQuotes, skipDirectives: noqa, type: ignore, pylint:, pragma:)
- [x] Update C# constructor (hasVerbatimStrings, skipDirectives: pragma)
- [x] Fix ParseIgnoreDirectives to accept commentPrefix parameter
- [x] Fix applyAutofixes to use language-aware region/directive checks
- [x] Fix fixture header IDs (remove space between emoji and path)
- [x] Fix TestFileHeaderId expectations to match current FileHeaderId format
- [x] Restore file_fixable.tsx fixture to original state with breachs
- [x] Remove bare # comment from file_fixed.py
- [x] Fix TestFixImprovedCommentLogic and TestFixConfigIgnored to use NewTypeScriptLanguage()
- [x] Add TestScanCommentsGo (12 subtests)
- [x] Add TestScanCommentsPython (11 subtests)
- [x] Add TestScanCommentsCSharp (8 subtests)
- [x] Add TestScanCommentsTypeScript (7 subtests)
- [x] Add TestScanCommentsShell (3 subtests)
- [x] Add TestScanCommentsRust (3 subtests)
- [x] Add TestScanCommentsAutofix (4 subtests: Python inline, Python trailing, Go block, C# inline)
- [x] All tests pass
- [x] Update dev docs
- [ ] Close ticket

## Changes

- `repo/cli/main.go`: Added language-specific fields to BaseLanguage, CommentScanState. Added SkipDirectives() to LanguagePlugin interface. Replaced no-op BaseLanguage.ScanComments with generic implementation. Removed TypeScriptLanguage.ScanComments override. Updated all language constructors. Made ParseIgnoreDirectives and applyAutofixes language-aware.
- `repo/cli/main_test.go`: Fixed TestFileHeaderId expectations. Fixed TestFixImprovedCommentLogic and TestFixConfigIgnored to use constructors. Added 7 new test functions with 48 subtests covering all languages.
- `compose/assets/repo/some/folder/file_fixed.tsx`: Fixed header ID format.
- `compose/assets/repo/some/folder/file_fixed.py`: Fixed header ID format, removed bare # comment.
- `compose/assets/repo/some/folder/file_fixed.go`: Fixed header ID format.
- `compose/assets/repo/some/folder/file_fixed.cs`: Fixed header ID format.
- `compose/assets/repo/some/folder/file_fixable.tsx`: Restored original fixture with breachs.
- `compose/assets/repo/some/folder/file_fixable_expected.tsx`: Fixed header ID format.

## Log

- Explored codebase: BaseLanguage.ScanComments was no-op, only TypeScriptLanguage had override with hardcoded comment logic.
- Added 6 new fields to BaseLanguage for language-specific string literal and comment handling.
- Added 4 new fields to CommentScanState for tracking triple quotes, raw backticks, verbatim strings.
- Implemented generic ScanComments (~290 lines) that handles: char-level block comments (/\* \*/), line-level block comments, inline comments with configurable prefix, string literals (single/double quotes), template literals (JS/TS), raw backtick strings (Go), triple-quoted strings (Python), verbatim strings (C#), URL scheme skipping, debug marker skipping, TODO blocks, language-specific skip directives, header section exclusion, region marker exclusion, JSDoc detection (TypeScript only), comment grouping.
- Removed 185-line TypeScript-specific ScanComments override.
- Fixed pre-existing header ID format issue across all fixture files (space between emoji and path).
- Fixed pre-existing TestFileHeaderId expectations.
- All 48 new subtests pass covering Go, Python, C#, TypeScript, Shell, Rust scanning and cross-language autofix.
