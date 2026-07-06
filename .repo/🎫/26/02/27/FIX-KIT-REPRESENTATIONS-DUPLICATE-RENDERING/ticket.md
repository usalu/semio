---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Bulk close
## Changes

- Updated `compose/js/sketchpad/Kit.tsx` so zip-backed rows no longer render from both the standalone file tree and the folder document at the same time.
- When folders are visible, imported root files now render through the folder document path and zip-backed folder children reuse the same document.
- Extended the existing Kit Playwright zip-entry block in `compose/js/sketchpad.test.ts` to assert that a zip-backed folder is not mirrored as a `file-*` row in mixed view.

## Log

- Inspected `compose/js/sketchpad/Kit.tsx` and `compose/js/sketchpad.test.ts`.
- Identified duplicate row generation: `allRows` emits imported zip paths from both the file tree branch and the folder document branch.
- Ran `cd compose/js && npx playwright test sketchpad.test.ts --grep "Kit" --reporter=line`.
- Initial sandboxed run failed before browser launch with Chromium sandbox permission errors.
- Escalated rerun launched successfully, then failed on an unrelated existing assertion in `expectNoLegacyWindowTabs` at `compose/js/sketchpad.test.ts:821` because legacy `settings` and `chat` tabs are still present in Kit.

## Todos

- Confirm the Kit flow after the unrelated legacy-tab failure is resolved upstream or in follow-up work.

## Plan

1. Refactor `Kit.tsx` row construction so zip-backed folder/file entries share one document instead of parallel file and folder trees.
2. Update the Kit Playwright test in `compose/js/sketchpad.test.ts` to catch duplicate folder/file rows in the existing zip-entry coverage.
3. Run the Kit test and capture the command and result in this ticket.
