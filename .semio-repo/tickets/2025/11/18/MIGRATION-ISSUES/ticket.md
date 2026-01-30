# Ticket

## Todos
# Migration Problems

**Date:** 2025-01-18  
**Status:** Migration Complete - All Assets Fully Compliant

## Overview

The migration from name-based references to GUID-based ID objects is complete. All TypeScript code compiles without errors. All assets in the `assets/semio` folder are compliant with the new schema with:

- GUID Consistency: Types and designs have the same GUID across all files (kit, standalone, piece references)
- Clean Schema: Deprecated `view` field removed from all designs
- Normalized JSON: All JSON files have alphabetically sorted keys recursively

## Problems Resolved

### 1. Duplicate Type Names

**Solution:** Migration script uses `name|variant` as key to distinguish types during migration, then converts to new hierarchy where variant becomes the name and parent references are created.

### 2. Standalone Design Files

**Solution:** Migration script now pre-loads all type files in the same directory to build type name→GUID mapping for standalone design files.

### 3. Connection Side Structure

**Solution:** Verified connections have complete side structures: `{ guid, piece: { guid }, connector: { guid } }`.

### 4. Kit.authors Collection

**Solution:** Migration script now collects unique Author objects (deduplicated by email) from all types and designs into `Kit.authors` array with full `name` and `email` fields.

### 5. TypeScript Schema Mismatch

**Solution:** Fixed `TypeSchema` and `DesignSchema` to use ID references for `parent` and `location` fields:

- `TypeSchema.parent`: `z.string().optional()` → `TypeIdSchema.optional()`
- `TypeSchema.location`: `LocationSchema.optional()` → `LocationIdSchema.optional()`
- `DesignSchema.parent`: `z.string().optional()` → `DesignIdSchema.optional()`
- `DesignSchema.location`: `LocationSchema.optional()` → `LocationIdSchema.optional()`

### 6. GUID Consistency Across Assets

**Solution:** Migration script now processes kit files first to establish authoritative GUIDs, then standalone files use those GUIDs via pre-loading. Types and designs maintain consistent GUIDs across:

- Standalone type/design JSON files
- Kit type/design entries
- Piece type/design references

### 7. Deprecated View Field

**Solution:** The `view` field (deprecated schema field) is explicitly excluded from migration output for all design files.

### 8. JSON Normalization ✅

**Solution:** Created `normalize-json.ps1` script that recursively sorts all JSON keys alphabetically for consistent formatting and easier diffing.

### 9. Type and Connector GUID Consistency Between Kit and Standalone Files ✅

**Problem:** Type GUIDs referenced in design files must exist in BOTH the kit and standalone type files for data integrity. Same requirement applies to connector GUIDs.

**Solution:**

- **Kit is the authoritative source**: Kit file is always migrated first to establish canonical GUIDs
- **Standalone type files load from kit**: When migrating standalone `type_*.json` files, they pre-load type GUIDs from `kit_metabolism.json` and reuse them
- **Standalone design files load from kit**: When migrating standalone `design_*.json` files, they pre-load both type and connector GUIDs from the kit
- **Abstract parent types**: Types created as abstract parents (e.g., "Capsule") exist only in the kit, not as standalone files (expected behavior)

**Verification Results:**

- ✅ All non-abstract type GUIDs exist in BOTH kit and standalone type files
- ✅ Abstract parent types (e.g., "Capsule", "Box", "Ellipsoid") exist in kit only
- ✅ All connector GUIDs from kit types are preserved in standalone type files

## Migration Statistics

- **Total Files:** 56
  - Types: 45
  - Designs: 10
  - Kits: 1
- **Files Migrated:** 56 (100%)
- **Compilation Errors:** 0
- **Data Quality Problems:** See above

## Completed Steps

1. ✅ All TypeScript compiles
2. ✅ Fixed TypeScript schema (TypeSchema.parent, TypeSchema.location, DesignSchema.parent, DesignSchema.location)
3. ✅ All pieces have type references with proper `{ guid }` structure
4. ✅ All connections have complete side structures with `piece`, `connector`, and side `guid`
5. ✅ Kit.authors properly collects unique Author objects with `name` and `email`
6. ✅ Standalone designs have type references (loaded from type files in same directory)
7. ✅ All 56 files migrated successfully
8. ✅ **GUID consistency across all assets** - Kit processed first to establish authoritative GUIDs
9. ✅ **Deprecated `view` field removed** from all design files
10. ✅ **All JSON normalized** with alphabetically sorted keys recursively
11. ✅ **Type/Connector GUIDs verified** - All non-abstract type GUIDs exist in BOTH kit and standalone files

## Schema Changes Completed

All entity references converted from strings to ID objects:

- `Piece.type`: `string` → `TypeId` (`{ guid: string }`)
- `Piece.design`: `string` → `DesignId` (`{ guid: string }`)
- `Side.piece`: `string` → `PieceId` (`{ guid: string }`)
- `Side.connector`: `string` → `ConnectorId` (`{ guid: string }`)
- `Side.designPiece`: `string?` → `PieceId?` (`{ guid: string }`)
- `Group.pieces`: `string[]` → `PieceId[]` (`{ guid: string }[]`)
- `Type.authors`: `string[]` → `AuthorId[]` (`{ guid: string }[]`)
- `Type.location`: `Location?` → `LocationId?` (`{ guid: string }`)
- `Design.authors`: `string[]` → `AuthorId[]` (`{ guid: string }[]`)
- `Design.location`: `Location?` → `LocationId?` (`{ guid: string }`)
- `Design.activeLayer`: `string?` → `LayerId?` (`{ guid: string }`)
- `File.folder`: `string?` → `FolderId?` (`{ guid: string }`)
- `Folder.parent`: `string?` → `FolderId?` (`{ guid: string }`)
- `Prop.key`: `string` → `Prop.quality`: `QualityId` (`{ guid: string }`)
- `Stat.key`: `string` → `Stat.quality`: `QualityId` (`{ guid: string }`)

All diffs, stores, and UI components updated accordingly.

## Changes

## Log

## Summary
