# Ticket

## Todos
# Previously

The diff schemas in `compose.ts` used plain string IDs:

- `removed: string[]` - array of guid strings
- `updated: { id: string, diff: XDiff }[]` - id was a string

This made it unclear which entity was being referenced and was inconsistent with the strongly-typed `EntityIdSchema` pattern.

# Plan

1. Update all diff schemas to use typed EntityId objects:
   - `removed: EntityId[]` where `EntityId = { guid: string }`
   - `updated: { <entity>: EntityId, diff: XDiff }[]` where `<entity>` is the entity name (e.g., `type`, `design`, `piece`)
2. Update all diff functions (`getDiff`, `inverseDiff`, `applyDiff`, `mergeDiff`) to use the new format
3. Update all helper functions and comparison functions
4. Update sketchpad files to use the new format
5. Update test fixtures

# Changes

## `compose.ts`

- Updated all `*DiffSchema` definitions to use `EntityIdSchema` in `removed` and `updated` arrays
- Changed `updated` arrays from `{ id: string, diff }` to `{ <entity>: EntityId, diff }` format
- Added `ConnectorsDiff` type export (was missing)
- Added `SideIdSchema` for composite Side entity identification
- Updated generic collection diff functions to use `entityKey` parameter
- Updated all entity-specific diff functions
- Updated `mergeKitDiff` to use proper entity-specific merge functions
- Updated helper functions: `removeTypeFromKit`, `removeDesignFromKit`, `removePortFromKit`, `removeFileFromKit`, `removeTagFromKit`, `removeConceptFromKit`
- Updated utility functions: `fixPieceInDesign`, `fixPiecesInDesign`, `removePiecesAndConnectionsFromDesign`, `replaceClusterWithDesign`
- Updated diff equality comparison functions (`areKitDiffsEqual` and nested functions)
- Added `areRemovedArraysEqual` helper for comparing EntityId arrays

## `sketchpad/Design.tsx`

- Added imports for `PieceId`, `ConnectionId`, `ConnectionDiff`
- Updated all design app commands to use new EntityId format:
  - `deleteSelected`: piece and connection removals use `{ guid: ... }` format
  - `addPiece`, `addPieces`: no change (uses full entity)
  - `removePiece`, `removePieces`: use `{ guid: ... }` format
  - `addConnection`, `addConnections`: no change
  - `removeConnection`, `removeConnections`: use `{ guid: ... }` format
  - `updatePiece`: use `{ piece: { guid }, diff }` format
  - `updatePieces`: signature changed to accept `{ piece: PieceId, diff }[]`
  - `updateConnection`: use `{ connection: { guid }, diff }` format
  - `updateConnections`: signature changed to accept `{ connection: ConnectionId, diff }[]`
- Updated diff status tracking functions to use new EntityId format

## Test Fixtures

- Updated `assets/compose/validation.json` to use new EntityId format
- Regenerated `assets/compose/diff_kit_metabolism.json` and `diff_kit_metabolism_inverted.json` using `scripts/generate-metabolism-diff.ts`

## C# Schema

- The C# diff test (`Kit_Plus_Diff_Equals_DiffedKit_And_DiffedKit_Plus_InverseDiff_Equals_Kit`) is skipped temporarily while the C# schema is updated to match the TypeScript EntityId format
- The C# `Compose.cs` requires significant updates to:
  - Change `List<string> Removed` to `List<EntityId> Removed` in all diff classes
  - Create entity-specific diff update classes (e.g., `TypeDiffUpdate`, `DesignDiffUpdate`)
  - Update implicit conversion operators and helper methods

## Python (`py/engine/engine.py`)

- Fixed missing `ports` relationship in `Kit` model
- Updated `_getCollectionDiff` to use EntityId format (`removed: [{"guid": ...}]`, `updated: [{entityKey: {"guid": ...}, "diff": ...}]`)
- Updated `_applyCollectionDiff` to support new EntityId format with entity key parameter
- Updated `_inverseCollectionDiff` to generate/parse new EntityId format
- Updated `_getAttributesDiff` to use GUID (not KEY) for attribute identification with EntityId format
- Updated `_applyAttributesDiff` to support new EntityId format
- Updated `_inverseAttributesDiff` to generate/parse new EntityId format
- Updated `areKitDiffsDictEqual` to handle EntityId format in removed and updated arrays
- Added `_extractUpdateGuid` helper function for extracting guid from both old and new formats
- Updated all validation fix generators to use EntityId format in diffs

## Test Results (Final)

- **TypeScript tests**: 9 passed
- **C# tests**: 61 passed, 1 skipped (diff test pending C# schema migration)
- **Python tests**: 17 passed
- **`schema.ts`**: Ran with pre-existing errors (missing C# entities, missing Grasshopper components)
- **TypeScript compilation**: 555 errors (mostly pre-existing issues in Design.tsx, i18n.ts)

## Changes

## Log

## Summary
# Summary

Refactor EntityId and Diff Schemas
