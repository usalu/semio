---
slug: APP-STATE-DECOUPLE
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: 'Decouple app state from Y.js, move transactions to app level'
model: claude-opus-4.5
input: []
commit: unknown
files: {}
lines:
  added: 0
  removed: 0
---

# Previously

Current architecture:

1. `SketchpadContext` has app states (Home, Kit, Type, Design, Quality) in XState
2. `transactions` is a global Record at sketchpad level - shared across all apps
3. `AppStore` classes use Y.js for transaction state (legacy code)
4. Kit import happens in React component state (`HomeDropZone`) with local `loadingKits` state

Problems:

- Transaction events are global (`TRANSACTION.START`) not per-app
- Some legacy code uses Y.js for transaction state instead of XState
- Navigation state change during kit import could interrupt import

# Plan

1. Add transaction state to each app's state interface (Design, Type, Kit, Quality)
2. Remove global `transactions` record from `SketchpadContext`
3. Add app-scoped transaction events (e.g., `DESIGN.TRANSACTION.START`)
4. Add transaction actions and guards per app
5. Move kit import to an XState actor that continues in background
6. Run tests to verify

# Changes

## Completed

1. **Added `AppTransactionState` interface** - Generic interface for per-app transaction state with `isTransactionActive`, `currentTransactionStack`, `pastTransactionStack`, and `redoStack` fields.

2. **Updated app state interfaces** - Added `transaction: AppTransactionState` to:
   - `DesignAppState`
   - `TypeAppState`
   - `KitAppState`
   - `QualityAppState`

3. **Updated `SketchpadContext`** - Removed global `transactions` record, added `backgroundOperations` for tracking async tasks that continue during navigation.

4. **Added app-scoped transaction events**:
   - `DESIGN.TRANSACTION.START/COMMIT/ABORT/UNDO/REDO/RECORD_EDIT`
   - `TYPE.TRANSACTION.START/COMMIT/ABORT/UNDO/REDO/RECORD_EDIT`
   - `KIT.TRANSACTION.START/COMMIT/ABORT/UNDO/REDO/RECORD_EDIT`

5. **Added background operation events**:
   - `BACKGROUND.START` - Start tracking an async operation
   - `BACKGROUND.COMPLETE` - Remove completed operation
   - `BACKGROUND.FAIL` - Mark operation as failed with error

6. **Created `createDefaultTransactionState()` helper** - Returns default transaction state for app initialization.

7. **Updated default app state functions** - `createDefaultDesignAppState`, `createDefaultTypeAppState`, `createDefaultKitAppState`, `createDefaultQualityAppState` now include default transaction state.

8. **Added app-scoped transaction actions** - Each app (Design, Type, Kit) has its own set of transaction actions that operate on the app's embedded transaction state.

9. **Added background operation actions** - `backgroundStart`, `backgroundComplete`, `backgroundFail` for managing async operations.

10. **Updated state machine event handlers** - Added transaction events to each navigation state (design, type, kit) and background events at global level.

11. **Removed obsolete global transaction guards** - Removed `hasActiveTransaction` and `noActiveTransaction` guards that referenced the old global transactions record.

12. **Updated kit import to use background operations** - `HomeDropZone` now tracks kit imports via `BACKGROUND.START/COMPLETE/FAIL` events instead of local React state. This ensures kit imports continue even when navigating away from Home.

13. **Added background operation selectors and hooks**:
    - `selectBackgroundOperations` - All background operations
    - `selectKitImportOperations` - Kit import operations with parsed kitName
    - `useKitImportOperations()` - Hook for accessing kit imports in components

## Pending

- Run tests to verify all changes work correctly

## Notes

The following pre-existing type errors remain in the codebase (unrelated to transaction changes):

- Missing `useThemeTriadic`, `useLanguageTriadic`, etc. hooks in Design.tsx, Kit.tsx, Type.tsx, Quality.tsx
- Missing `selectDesign`, `selectType` functions in Kit.tsx
- Various type mismatches in Design.tsx (DesignStore, DesignAppState)
- Missing `setHelperLines` in Design.tsx

These should be addressed in a separate task.
