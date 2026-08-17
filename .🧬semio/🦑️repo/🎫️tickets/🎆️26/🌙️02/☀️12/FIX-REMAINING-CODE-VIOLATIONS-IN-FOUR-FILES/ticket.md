---
goal: AI-OPTIMIZED-REPO/SINGLE-FILE-REPO/CONSISTENT-SECTIONS
---

# Ticket

## Summary

Fixed 25 code breachs across 4 files to 0. assets/index.ts: added MUST to section summary, removed 8 non-definition import comments. build.py: removed shebang, added Build section with MUST summary, removed 4 non-definition variable comments, fixed file ID emoji. queries.ts: moved import inside Queries section, added MUST to summary. Home.tsx: removed 3 re-export comments.

## Changes

- **assets/index.ts**: Added MUST keyword to Exports section summary; removed 8 comment lines before non-definition imports/re-exports
- **assets/grasshopper/build.py**: Removed shebang; wrapped all code after header in `# region 🔖️Build` section with MUST summary; removed 4 comment lines before non-definition variable assignments; fixed file ID emoji (📜️→💻️)
- **repo/vscode/queries.ts**: Moved import inside Queries section; added MUST keyword to section summary
- **compose/js/sketchpad/Home.tsx**: Removed 3 comment lines before non-definition re-exports/default export

## Log

## Todos

## Plan

1. **assets/index.ts** (9 inline comments):
   - Line 29: section summary missing MUST keyword → add MUST
   - Lines 31,33,35,37,39,41,43,45: comments before non-definition imports/re-exports → remove
2. **assets/grasshopper/build.py** (8 orphan defs + 4 inline comments):
   - Remove shebang on line 1 (other Python files don't use it)
   - Wrap all code after header in `# region 🔖️Build` / `# endregion 🔖️Build` with MUST summary
   - Remove 4 comment lines before non-definition variable assignments (lines 256,258,260,263)
3. **repo/vscode/queries.ts** (1 orphan + 1 inline):
   - Move import into Queries section (move section start before import)
   - Add MUST keyword to section summary
4. **compose/js/sketchpad/Home.tsx** (3 inline comments):
   - Lines 221,224: comments before re-exports → remove
   - Line 1717: comment before `export default Home;` → remove
