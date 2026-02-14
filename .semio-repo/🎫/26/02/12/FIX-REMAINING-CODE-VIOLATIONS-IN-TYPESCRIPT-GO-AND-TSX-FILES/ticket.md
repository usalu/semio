---
goal: fix-code-breachs
---

# Ticket

## Summary

Fixed 41 code breachs across 17 files: added shebang skip to orphan detection in analyzer, added section summaries to 14 TS files, wrapped Go kit_sqlite.go in section, removed TSX inline comment. All 17 files now pass with 0 breachs.

## Changes

1. **semio-repo/cli/main.go**: Added shebang skip in orphan detection (line 1 `#!` lines are now excluded like they are in ScanComments).
2. **14 TypeScript files**: Added section summary comments after `//#region 🔖SectionName` markers:
   - `semio/engine/build.ts` - Build section summary
   - `semio/engine/generate-schemas.ts` - Schema Generation section summary
   - `semio/engine/post-build.ts` - Post Build section summary
   - `semio/engine/sqliteschema.ts` - Schema Export section summary
   - `semio/gh/Semio.Grasshopper/build-value-lists.ts` - Value List Generation section summary
   - `semio/gh/Semio.Grasshopper/build.ts` - Build section summary
   - `semio/gh/Semio.Grasshopper/yak/build.ts` - Build section summary
   - `semio/gh/Semio.Grasshopper/yak/login.ts` - Login section summary
   - `semio/gh/Semio.Grasshopper/yak/publish.ts` - Publish section summary
   - `semio/gh/Semio.Grasshopper/yak/test-push.ts` - Test Push section summary
   - `semio/gh/Semio.Grasshopper/yak/unyank.ts` - Unyank section summary
   - `semio/gh/Semio.Grasshopper/yak/yank.ts` - Yank section summary
   - `semio/jsonschema/build.ts` - Schema Export section summary
   - `semio/net/Semio/build.ts` - Build section summary
3. **semio/go/kit_sqlite.go**: Wrapped all code after header in `// #region 🔖SQLite Kit Operations` / `// #endregion 🔖SQLite Kit Operations` section with summary.
4. **semio/js/sketchpad/Type.tsx**: Removed inline comment `// Default export of the TypeApp component.` before `export default TypeApp;`.

## Log

- Read all 17 files, confirmed breach patterns
- Found that shebang orphan detection is a bug in analyzer: `ScanComments` already ignores shebangs but orphan detection did not
- Fixed analyzer to skip shebangs at line 1 in orphan detection
- Added section summaries with MUST keywords to all 14 TS files
- Wrapped Go file kit_sqlite.go orphan definitions in a section
- Removed inline comment in Type.tsx
- Verified all 17 files pass with 0 breachs
- CLI tests pass

## Todos

- [x] Fix analyzer shebang orphan detection
- [x] Add section summaries to 14 TS files
- [x] Wrap Go file in section
- [x] Fix TSX inline comment
- [x] Verify all files pass analysis
- [x] Run CLI tests

## Plan

Fix orphan-at-line-1 by skipping shebangs in analyzer orphan detection. Add missing section summaries. Wrap Go orphan definitions in section. Remove TSX inline comment.
