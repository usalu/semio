---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-POLICY-MECHANISM
---

# Ticket

## Summary

Implemented Fix() method and applyAutofixes() function with full autofix logic for all 6 autofixable statutes (empty section, missing end name, name mismatch, inline/block/JSDoc comments). Added 16 comprehensive tests. All 35 tests pass.

## Changes

- `semio-repo/cli/main.go`: Implemented `repoContext.Fix()` with full autofix logic, `applyAutofixes()` per-file fix application, `findMatchingSectionStartName()` helper for section end name resolution. Added blank line collapse post-processing.
- `semio-repo/cli/main_test.go`: Added 16 new fix tests (TestFixApplyAutofixes, TestFixSectionMissingEndName, TestFixSectionNameMismatch, TestFixSectionEmpty, TestFixInlineComment, TestFixBlockComment, TestFixJSDocComment, TestFixMultipleBreachsSameFile, TestFixNonAutofixableNotFixed, TestFixViaGraphQL, TestFixViaRepoContext, TestFixIdempotent, TestFixNestedSections, TestFixExtractFileFromScope, TestFixStatuteMeta, TestFindMatchingSectionStartName). Scoped existing GraphQL fix mutation test to avoid fixture side effects.
- `semio/assets/repo/some/folder/file_fixable.tsx`: New fixture with all autofixable breachs.
- `semio/assets/repo/some/folder/file_fixable_expected.tsx`: Expected output after autofix.

## Log

- Explored breach system: Statute, StatuteMeta, Autofixable flag, PolicyContext, CheckPoliciesWithContext.
- Implemented Fix() method replacing the stub with full autofix pipeline: detect breachs, filter autofixable, group by file, apply fixes per file.
- Implemented applyAutofixes() with per-breach-kind handlers: empty section removal (with surrounding blank cleanup), missing end name (walk back to find matching start), name mismatch (replace end with matching start name), inline comment removal (contiguous block with blank line tracking), block/JSDoc comment removal.
- Added blank line collapse post-processing to prevent double blank lines after line removal.
- Fixed empty section blank line handling to only remove one surrounding blank line (prefer preceding).
- Scoped GraphQL fix mutation tests to prevent fixture file modification side effects.
- Created fixture files for end-to-end fix validation.
- All 35 tests pass (18 fix tests + 17 other tests).

## Todos

- [x] Explore breach system codebase
- [x] Understand current breach types and autofix logic
- [x] Run existing tests to see current state
- [x] Implement Fix() with autofix for code breachs
- [x] Write comprehensive tests for all fix statutes
- [x] Fix blank line handling and test side effects
- [x] Run all tests to confirm everything passes
- [x] Update dev docs (README.md, AGENTS.md)
- [x] Close ticket with summary

## Plan

1. Detect breachs via CheckPoliciesWithContext
2. Filter to autofixable breachs only
3. Group breachs by file path (extracted from scope)
4. For each file, apply fixes bottom-up to avoid line number shifts
5. Handle each statute: empty sections, missing end names, name mismatches, inline/block/JSDoc comments
6. Collapse consecutive blank lines after removal
7. Write fixed content back to file
8. Return FixResult with fixed count, remaining count, and remaining breachs
