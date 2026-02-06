---
goal: CODEBASE
---

# Ticket

## Summary

Replace file path in source code headers with artifact IDs from the semio repo ID system. Change `ViolationCodeHeaderMissingFilename` to `ViolationCodeHeaderWrongFileId` and make it autofixable. Fix all existing headers via `cli fix`.

## Changes

## Log

- Explored current header system: `headerFmt`, `generateFileHeader`, `headerPolicy`, `ViolationCodeHeaderMissingFilename`
- Current headers use plain paths like `// js/vscode/extension.ts` or `// go/repo/repo_test.go`
- Need to replace with artifact IDs like `// 💻︎ @semio-repo/cli/cli.go` using `fileKindEmoji` + `\uFE0E` + space + path

## Todos

- [x] Understand current header system
- [ ] Add `FileHeaderId(path)` helper function
- [ ] Replace `ViolationCodeHeaderMissingFilename` with `ViolationCodeHeaderWrongFileId` (autofixable)
- [ ] Update `headerPolicy` to check for correct file artifact ID
- [ ] Update `generateFileHeader` to use file artifact ID
- [ ] Add autofix logic in `applyAutofixes` for `ViolationCodeHeaderWrongFileId`
- [ ] Update test fixtures for new ID format
- [ ] Extend existing tests
- [ ] Run `cli fix` to fix all existing file headers
- [ ] Verify all headers correct and all tests pass
- [ ] Update AGENTS.md and README.md
- [ ] Close ticket

## Plan

1. Add `FileHeaderId(path string) string` helper that returns the file artifact ID for headers
2. Rename `ViolationCodeHeaderMissingFilename` → `ViolationCodeHeaderWrongFileId`, mark autofixable
3. Update `headerPolicy` to detect wrong/missing file IDs
4. Update `generateFileHeader` to use `FileHeaderId`
5. Add autofix case in `applyAutofixes`
6. Update fixture files for new ID format
7. Extend tests
8. Run `cli fix` to fix all files
9. Verify everything works
