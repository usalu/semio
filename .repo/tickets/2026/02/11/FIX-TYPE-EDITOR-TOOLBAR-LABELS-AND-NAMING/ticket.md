---
goal: SKETCHPAD-FIXES
---

# Ticket

## Summary

Fixed Type Editor toolbar labels to match Design and Kit app patterns. Updated subtool label from 'selection' to 'select' and added missing 'connector' subtool translations in both English and German locale files. Also completed German locale with all missing subtool entries.
## Changes

- /workspaces/semio/semio/js/sketchpad/Type.tsx
- /workspaces/semio/semio/js/sketchpad/locales/en.json
- /workspaces/semio/semio/js/sketchpad/locales/de.json

## Log

- Started ticket for fixing Type Editor toolbar labels
- Analyzed toolbar implementation in Type.tsx, Design.tsx, and Kit.tsx
- Found that Type.tsx uses incorrect subtool label ID "selection" instead of "select"
- Found that Type.tsx uses "connector" subtool which doesn't exist in translations
- Updated Type.tsx to change subToolLabelId from "selection" to "select"
- Added "connector" subtool to en.json
- Added all missing subtool entries to de.json (select, hand, additive, subtractive, intersect, connector)
- Verified no errors in modified files
- All changes complete

## Todos

- [x] Update Type.tsx selection subtool label from "selection" to "select"
- [x] Add "connector" subtool translation to en.json
- [x] Add "connector" subtool translation to de.json
- [x] Verify fixes work correctly

## Plan

1. Update Type.tsx to use correct subtool label IDs:
   - Change "semio.sketchpad.toolbar.subtool.selection" to "semio.sketchpad.toolbar.subtool.select"
   - Add "connector" subtool translation (currently missing)

2. Add missing "connector" translation to locale files:
   - Add to en.json: "connector": "Connector"
   - Add to de.json: "connector": "Konnektor"
