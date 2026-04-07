---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-MECHANISMS/REPO-FILE-MECHANISM
---

# Ticket

## Summary

Added glob support for .repo/files.json in file policy with exact+glob matcher and tests.

## Changes

- Updated `repo/cli/main.go`:
  - Added `Godfile` structure with exact-path and glob-pattern collections.
  - Added `isGlobPattern` to detect wildcard entries.
  - Refactored `loadGodfile` to classify entries as exact or glob.
  - Added `godfileMatchesPath` using `doublestar.Match` for glob entries.
  - Updated `filePolicy` to validate files via exact or glob match.
- Updated `repo/cli/main_test.go`:
  - Added `TestFilePolicyGodfileSupportsGlobPatterns` to verify `src/**/*.ts` and `docs/*.md` entries allow matching files while still flagging unlisted files.

## Log

- Opened ticket for REPO-FILE-MECHANISM scope.
- Located existing `files.json` policy enforcement in `filePolicy`.
- Implemented glob-aware matcher with exact-match fast path.
- Added/extended tests in existing test file.
- Ran targeted policy tests successfully.

## Todos

- [x] Locate `.repo/files.json` consumer.
- [x] Implement glob pattern support without regressing exact path behavior.
- [x] Extend existing tests (no new test files).
- [x] Run relevant test coverage.

## Plan

1. Inspect current godfile loading and policy matching implementation.
2. Refactor matcher to support exact entries and glob entries.
3. Extend existing `main_test.go` file with glob-pattern test coverage.
4. Execute targeted tests for file policy behavior.
