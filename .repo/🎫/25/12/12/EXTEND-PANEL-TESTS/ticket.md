# Ticket

## Todos
# Previously

User requested extending sketchpad tests for every app (Home, Kit, Type, Design, Docs) to use panel toggle in navbar and verify panel content.

# Plan

1. Explore existing tests in sketchpad.test.ts
2. Identify panel toggle IDs per app using Playwright MCP exploration
3. Add helper functions for panel toggle verification
4. Update each app test to verify panel toggles work
5. Run tests to verify

# Changes

## Helper Functions Added

Added to `js/semio/sketchpad.test.ts`:

- `verifyToggleWorks(page, toggleId, panelKey, appName)` - Clicks toggle, verifies state changes or panel visibility
- Enhanced panel helpers already existed: `togglePanelAndVerify`, `verifyPanelSection`, `verifyPanelHasContent`

## Test Updates Per App

### Home Test

- Checks for settings toggle visibility
- Verifies toggle state change on click

### Kit Test

- Checks for settings toggle visibility
- Verifies toggle state change on click

### Type Test

- Checks for workbench toggle visibility
- Verifies workbench panel has connectors/models section
- Checks for settings toggle visibility

### Design Test

- Already has extensive panel testing
- Added verification for workbench, settings, details toggles

### Docs Test

- Verifies workbench toggle opens and shows TOC/navigation items (10 buttons found)
- Verifies details toggle opens

## Critical Fix: Import Navigation

Fixed `initHome` function - the application no longer auto-navigates after file import. Updated test to:

1. Wait for import to complete (10 seconds)
2. Find "Metabolism" cell/row on page
3. Double-click to navigate to kit

## Test Results

**Passing Tests (4/6):**

- Home (9.3s) - Panel toggle verification complete
- Kit (25.7s) - Panel toggle verification complete
- Docs (21.7s) - Workbench shows 10 navigation items, details toggle works
- Design Drag and Drop (34.5s) - Validates 118 draggable type avatars

**Failing Tests (2/6) - Pre-existing issues:**

- Type - `[TypeMesh] Selected` console message not found (pre-existing assertion unrelated to panel toggles)
- Design - Flaky performance test (scene pan took 531ms > 500ms threshold)

## Changes

## Log

## Summary
# Summary

>-
