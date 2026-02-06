---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-POLICY-MECHANISM
---

# Ticket

Improve the comment fix logic to ignore TODOs and their descriptions, ignore config files, and support partial line removal for comments.

## Summary

Improved comment fix logic to support TODO preservation, config file exclusion, and partial line removal of comments. Updated LanguagePlugin interface and multiple language providers.

## Changes

- Updated `LanguagePlugin` interface to include `BlockCommentStart()` and `BlockCommentEnd()`.
- Enhanced `TypeScriptLanguage.ScanComments` to:
  - Skip configuration files.
  - Track "TODO blocks": contiguous comment lines starting with `// TODO` are now preserved.
  - Correctly report the column position for all comment violations.
- Refactored `applyAutofixes` in `main.go` to support:
  - Partial removal of inline comments (preserving code before them).
  - Splice-based removal of block comments (preserving code before and after `/* ... */` blocks).
- Added `blockCommentStart` and `blockCommentEnd` metadata to Go, C#, Rust, and Ruby language providers.
- Added comprehensive unit tests in `main_test.go` covering TODO preservation, config file exclusion, and partial line comment removal.

## Log

- Ticket opened.
- Analyzed existing code and requirements.
- Refactored `Violation` struct and `CreateViolation` to support `Column` tracking across 26 callsites.
- Updated `LanguagePlugin` interface and base classes.
- Implemented TODO block detection in TypeScript scanner.
- Improved `applyAutofixes` with substring splicing logic for single-line block comments and partial inline comments.
- Verified changes with `go test`.
- Updated `README.md` and `AGENTS.md` docs.

## Todos

- [x] Add `InTodoBlock` to `CommentScanState` in `@semio-repo/cli/main.go`
- [x] Update `TypeScriptLanguage.ScanComments` to handle TODO blocks and ignore config files
- [x] Update `CreateViolation` to include `column` (or set it manually)
- [x] Update `applyAutofixes` to support partial line removal of comments
- [x] Extend `main_test.go` with new test cases
- [x] Verify all tests pass

## Plan

1. Modify `CommentScanState` to include `InTodoBlock`.
2. Update `TypeScriptLanguage.ScanComments`:
   - Skip files where `DeriveFileKind` returns `FileKindConfig`.
   - Implement TODO block detection: lines starting with `// TODO` enter `InTodoBlock` mode. Mode persists across subsequent `//` lines. Mode exits on blank line or non-comment content.
   - Record `Column` for comment violations.
3. Update `applyAutofixes`:
   - For `ViolationCodeCommentInline`, if `v.Column > 1`, only remove the comment portion of the line.
   - For blocks, handle partial lines on start/end if possible.
4. Add comprehensive tests in `main_test.go`.
