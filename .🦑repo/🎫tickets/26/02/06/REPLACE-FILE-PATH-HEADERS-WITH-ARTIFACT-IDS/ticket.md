---
goal: CODEBASE
---

# Ticket

## Summary

Replaced plain file paths in source code headers with emoji-prefixed artifact IDs. Renamed BreachCodeHeaderMissingFilename to BreachCodeHeaderWrongFileId (autofixable). Added FileHeaderId helper, updated headerPolicy detection, generateFileHeader, and applyAutofixes. Fixed ~155 file headers across the repo. Added 5 new tests. Fixed pre-existing BundleKindClient bug. Updated AGENTS.md and README.md documentation.

## Changes

- `repo/cli/cli.go`: Added `FileHeaderId()` helper, renamed statute, updated `headerPolicy` detection, updated `generateFileHeader`, added autofix case in `applyAutofixes`, fixed pre-existing `BundleKindClient` → `BundleKindUI` bug
- `repo/cli/cli_test.go`: Added `TestFileHeaderId`, `TestFixHeaderWrongFileId`, `TestFixHeaderWrongFileIdIdempotent`, `TestFixHeaderWrongFileIdDetection`, `TestFixHeaderWrongFileIdEndToEnd`; updated existing tests for new statute; fixed inline comment count expectation
- `assets/repo/some/folder/file.tsx`: Updated header to artifact ID
- `assets/repo/some/folder/file_fixed.tsx`: Updated header to artifact ID
- `assets/repo/some/folder/file_fixed.go`: Updated header to artifact ID
- `assets/repo/some/folder/file_fixed.cs`: Updated header to artifact ID
- `assets/repo/some/folder/file_fixed.py`: Updated header to artifact ID
- `assets/repo/some/folder/file_fixable_expected.tsx`: Updated header to artifact ID of source file
- ~155 source files across the repo: Headers replaced via `cli fix`
- `AGENTS.md`: Documented file header artifact ID requirement, autofixable statute, codebase sections
- `README.md`: Documented file header artifact ID policy in Breachs and Code Hygiene sections

## Log

- Explored current header system: `headerFmt`, `generateFileHeader`, `headerPolicy`, `BreachCodeHeaderMissingFilename`
- Current headers use plain paths like `// js/vscode/extension.ts` or `// ./repo/cli/cli_test.go`
- Replaced with artifact IDs like `// 💻 repo/cli/cli.go` using `fileKindEmoji` + `\uFE0E` + space + path
- Fixed pre-existing `BundleKindClient` undefined bug (should be `BundleKindUI`)
- Fixed pre-existing inline comment count expectation (2 not 1, blank lines reset grouping)
- Repo-wide fix needed extra pass for `.storybook` files not enumerated by bundle scope
- Test fixture `file_fixable_expected.tsx` needed manual correction after fix command modified it

## Todos

- [x] Understand current header system
- [x] Add `FileHeaderId(path)` helper function
- [x] Replace `BreachCodeHeaderMissingFilename` with `BreachCodeHeaderWrongFileId` (autofixable)
- [x] Update `headerPolicy` to check for correct file artifact ID
- [x] Update `generateFileHeader` to use file artifact ID
- [x] Add autofix logic in `applyAutofixes` for `BreachCodeHeaderWrongFileId`
- [x] Update test fixtures for new ID format
- [x] Extend existing tests
- [x] Run `cli fix` to fix all existing file headers
- [x] Verify all headers correct and all tests pass
- [x] Update AGENTS.md and README.md
- [x] Close ticket
