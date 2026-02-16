---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI/REPO-CLI-FILTERS
---

# Ticket

## Summary

Corrected ScanComments to ignore shebang headers.

## Changes

- Modified `semio-repo/cli/main.go`:
  - Updated `BaseLanguage.ScanComments` to skip line if `i == 0` and line starts with `#!`.
  - Updated `TypeScriptLanguage.ScanComments` to skip line if `i == 0` and line starts with `#!`.
- Modified `semio-repo/cli/main_test.go`:
  - Added `TestFixHeaderWithShebang` to reproduce the issue (Python).
  - Added `TestFixHeaderWithShebangTS` to reproduce the issue (TypeScript).

## Log

- Reproduced the issue where shebangs were flagged as inline comments.
- Implemented logic to skip first line if it starts with `#!`.
- Applied fix to both BaseLanguage (used by Python) and TypeScriptLanguage.
- Resolved debugging issues where file edits were not persisting or syntax errors were introduced.

## Todos

- [x] Reproduce invalid inline comment breach on shebang headers.
- [x] Modify `BaseLanguage.ScanComments` to ignore `#!` lines at index 0.
- [x] Modify `TypeScriptLanguage.ScanComments` to ignore `#!` lines at index 0.
- [x] Verify fix with tests.

## Plan

1. Create reproduction tests (Done).
2. Implement fix in `main.go` (Done).
3. Verify (Done).
