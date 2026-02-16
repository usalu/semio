# Ticket

## Todos
# Previously

The `sketchpad.test.ts` had a nested structure with `test.describe` blocks:

- `Kit` describe with `beforeEach` that creates temporary kit
  - `Design` describe with `beforeEach` that creates design (2 tests)
  - `Type` describe with `beforeEach` that creates type (3 tests)
- `Kit Import Drag and Drop` describe (1 test)
- `Docs` describe (5 tests)
- `Settings Panel Hierarchy` describe (5 tests)

This structure caused unnecessary test isolation and didn't reflect the app hierarchy clearly.

# Plan

1. Create helper functions for app initialization: `initHome`, `initKit`, `initDesign`, `initType`, `initDocs`
2. Create helper functions for common operations: `openSettingsPanel`, `getSettingsSections`
3. Restructure to one test per app: Home, Kit, Design, Type, Docs
4. Each child app test calls parent initialization first
5. Consolidate all functionality from nested tests into single sequential tests

# Changes

- Extracted `openSettingsPanel` and `getSettingsSections` as module-level functions
- Created `initHome`, `initKit`, `initDesign`, `initType`, `initDocs` helper functions
- `initKit` calls `initHome` first, `initDesign` calls `initKit` first, `initType` calls `initKit` first
- Consolidated all tests into 5 single tests:
  - **Home**: Settings panel hierarchy test
  - **Kit**: Imports metabolism.zip, verifies content, tests files view, performance test for expand/collapse, settings panel hierarchy
  - **Design**: Windows visibility, drag-and-drop pieces functionality, settings panel hierarchy
  - **Type**: Toolbar visibility, selection/connector tool tests, settings panel hierarchy
  - **Docs**: Content loading, workbench panel sections, pages in sections, details panel, navigation

## Changes

## Log

## Summary
# Summary

Restructure sketchpad tests to one test per app with parent initialization
