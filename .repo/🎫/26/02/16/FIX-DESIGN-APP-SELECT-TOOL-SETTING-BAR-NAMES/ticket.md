---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fixed the Design app select tool setting bar names. The DesignSelectSettings component used incorrect element IDs and i18n label keys that didn't match the expected naming convention. Updated IDs from flat naming (`select.additive`) to hierarchical naming (`select.mode.additive`, `select.mode.subtractive`, `select.mode.intersect`, `select.shape.rectangular`, `select.shape.lasso`, `select.navigation.hand`). Added missing toggles: intersect mode, rectangular/lasso shape, and hand navigation. Also imported IntersectIcon. Fixed merge conflict in Sketchpad.tsx toolbar condition.

## Changes

- `compose/js/sketchpad/Design.tsx`: 
  - Added `IntersectIcon` import
  - Rewrote `DesignSelectSettings` with correct IDs: `mode.additive`, `mode.subtractive`, `mode.intersect`, `shape.rectangular`, `shape.lasso`, `navigation.hand`
  - Added missing Intersect, Rectangular, Lasso, and Hand toggles
  - Fixed i18n label keys to match en.json and de.json structure
- `compose/js/sketchpad/Sketchpad.tsx`:
  - Resolved merge conflict in toolbar visibility condition

## Log

- Identified mismatch between element IDs in DesignSelectSettings and test expectations
- Verified en.json and de.json have all required translations at correct paths
- Imported IntersectIcon from @compose/assets
- Rewrote DesignSelectSettings with 6 toggles matching test IDs
- Resolved merge conflict in Sketchpad.tsx

## Todos

- [x] Fix merge conflict in Sketchpad.tsx
- [x] Fix DesignSelectSettings IDs and labels
- [x] Add missing toggles (intersect, rectangular, lasso, hand)
- [x] Verify i18n keys match en.json
- [x] Check German locale translations

## Plan

1. Identify incorrect IDs in DesignSelectSettings
2. Update element IDs with hierarchical naming
3. Add missing toggles
4. Verify i18n translations exist
