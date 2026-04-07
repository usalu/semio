---
goal: AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY/REPO-CLI/REPO-CLI-FILTERS
---

# Ticket

## Summary

Fixed full tree hierarchy rendering by adding folders root category, ensuring bundle-root path inclusion, forcing tree section inclusion, and restructuring policy tree to policy->entitykind->statute. Extended existing tree tests and validated with targeted go test runs.

## Changes

- Updated tree command build options to always include sections for tree rendering.
- Added top-level `folders` category assembly from streamed folders/files with recursive file/section/definition attachment.
- Fixed bundle matching so bundle-root folder/file paths are included (exact-root matches were previously skipped).
- Reworked policy subtree construction to emit entitykind grouping between policy and statutes.
- Extended `TestBuildMonorepoTree` coverage for:
  - required top-level categories including `folders`
  - folder/file/section/definition presence under folders tree
  - `policy -> entitykind -> statute` shape

## Log

- Investigated tree command flow and tree builder in `repo/cli/main.go`.
- Implemented hierarchy fixes in builder and policy grouping path.
- Updated existing tree tests in `repo/cli/main_test.go`.
- Verified with targeted test runs:
  - `go test ./... -run TestTreeCommandFlags -count=1`
  - `go test ./... -run TestBuildMonorepoTree -count=1 -timeout=30m`
  - `go test ./... -run TestTreeCommands -count=1 -timeout=30m`

## Todos

- [x] Locate tree rendering/build logic gap.
- [x] Implement complete hierarchy fix in tree builder.
- [x] Extend existing tests for full hierarchy and policy grouping.
- [x] Run targeted tests and confirm pass.

## Plan

1. Inspect current tree build/filter/render paths and reproduce missing hierarchy conditions.
2. Fix tree assembly to guarantee complete hierarchy and expected root entity categories.
3. Extend existing tree tests for regression coverage.
4. Execute targeted tests and finalize ticket.
