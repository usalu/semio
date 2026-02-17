---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Successfully migrated chat and settings to navbar toggle buttons that render in the right panel position. TypeScript compiles cleanly.
## Plan

1. Remove Settings and Chat from all App Window Kind enums (Home, Kit, Type, Design, Quality, Docs)
2. Remove Settings and Chat from windowKinds arrays
3. Remove Settings and Chat from defaultLayout definitions
4. Restore Settings and Chat as side panel tabs via addSidePanelTab calls
5. Remove unused imports (Tree, TreeStateProvider) where no longer needed
6. Update TypeScript compilation
7. Run e2e tests to verify

## Todos

- [ ] Remove Settings/Chat from HomeAppWindowKind
- [ ] Remove Settings/Chat from KitAppWindowKind
- [ ] Remove Settings/Chat from TypeAppWindowKind
- [ ] Remove Settings/Chat from DesignAppWindowKind
- [ ] Remove Settings/Chat from QualityAppWindowKind
- [ ] Remove Settings/Chat from DocsAppWindowKind
- [ ] Add Settings/Chat as side panel tabs for each app
- [ ] Clean up imports
- [ ] Verify TypeScript compilation
- [ ] Run e2e tests

## Changes

## Log
