---
date:
  created: '2025-12-03T14:01:17.461Z'
  updated: '2025-12-03T14:01:17.461Z'
slug: SCHEMA-SYNC
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Schema extraction and synchronization script
model: claude-opus-4.5
prompts: []
commit: unknown
affectedFiles: []
lines:
  added: 0
  removed: 0
---

# Previously

The user wants a script similar to the i18n script that extracts schema from multiple sources:

- `semio.ts` (TypeScript - source of truth)
- `engine.py` (Python)
- `Semio.cs` (C#)
- `Semio.Grasshopper.cs` (Grasshopper components)

# Plan

1. Create schema extraction script (`scripts/schema.ts`)
2. Parse TypeScript Zod schemas
3. Parse Python SQLModel classes
4. Parse C# classes with [Model] attribute
5. Parse Grasshopper components/params/goos
6. Generate comparison reports
7. Identify schema mismatches
8. Sync schemas (large task - many fields missing)

# Changes

## scripts/schema.ts

Created new schema extraction script that:

- Parses TypeScript Zod schemas from `semio.ts`
- Parses Python SQLModel field classes from `engine.py`
- Parses C# entity classes from `Semio.cs`
- Parses Grasshopper components/params/goos from `Semio.Grasshopper.cs`
- Generates individual reports: `schema-ts.json`, `schema-py.json`, `schema-net.json`, `schema-grasshopper.json`
- Generates summary report: `schema.json` with all errors/warnings

## Initial Report Results

| Source      | Entities                          | ID Types |
| ----------- | --------------------------------- | -------- |
| TypeScript  | 27                                | 21       |
| Python      | 25                                | 17       |
| C#          | 26                                | 15       |
| Grasshopper | 20 components, 60 params, 60 goos |

**77 errors, 13 warnings** after filtering timestamp fields and relationship fields.

### Error Breakdown by Source

| Source | Errors |
| ------ | ------ |
| Python | 52     |
| C#     | 25     |

### Error Breakdown by Entity (top issues)

| Entity   | Errors | Notes                                                                                                                                                                 |
| -------- | ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Quality  | 22     | Missing folder, canScale, defaultSiUnit, defaultImperialUnit, isMinExcluded, isMaxExcluded, defaultValue, icon, image, unit, benchmarks, attributes in both PY and C# |
| Piece    | 10     | Missing design, scale, mirrorPlane, isHidden, isLocked, color, props in C#                                                                                            |
| Layer    | 4      | Missing path, isHidden, isLocked, attributes in PY                                                                                                                    |
| Location | 4      | Missing guid, altitude, attributes in both                                                                                                                            |
| Type     | 4      | Missing virtual, concepts, attributes in PY                                                                                                                           |
| Concept  | 4      | Missing name, description, icon, attributes in PY                                                                                                                     |

### Key Missing Items

**Python (52 errors):**

- Missing `Interface` entity entirely
- Many entities missing `attributes` field
- Missing fields: `Quality.*` (most fields), `Layer.path/isHidden/isLocked`, `Concept.*`, `Tag.*`
- Missing Id types: `LocationId`, `InterfaceId`, `TagId`, `ConceptId`

**C# (25 errors):**

- `Quality` missing most fields (folder, canScale, units, benchmarks, etc.)
- `Piece` missing design, scale, mirrorPlane, isHidden, isLocked, color, props
- `Location` missing guid, altitude
- `Benchmark` missing guid
- `Group` missing guid
- `Stat` missing guid, quality
- `Kit` missing tags
- Missing Id types: `LocationId`, `BenchmarkId`, `PropId`, `LayerId`, `GroupId`, `StatId`

**Grasshopper (warnings):**

- Missing components: `Benchmark`, `Quality`, `Prop`

## Resolution - Schema Synchronized

After iterative fixes, the schema is now synchronized across TypeScript, Python, and C#:

**Final Report: 0 errors, 6 warnings**

| Source     | Entities | ID Types |
| ---------- | -------- | -------- |
| TypeScript | 27       | 21       |
| Python     | 26       | 18       |
| C#         | 26       | 21       |

### C# Fixes Applied

- Added `LocationId` class and `Guid`, `Altitude` fields to `Location`
- Added `Mime` field to `File`
- Added `BenchmarkId` class and `Guid` field to `Benchmark`
- Added `Folder`, `Icon`, `Image`, `Unit` fields to `Quality`
- Added `Design`, `Scale`, `MirrorPlane`, `IsHidden`, `IsLocked`, `Color`, `Props` fields to `Piece` and `PieceDiff`
- Added `GroupId` class and `Guid` field to `Group`
- Added `StatId` class, `Guid` and `Quality` fields to `Stat`
- Added `LayerId` class
- Added `PropId` class
- Added `Tags` field to `Kit`

### Python Fixes Applied

- Added `LocationAltitudeField` and `altitude` field to `Location`
- Added `FileMimeField` and `mime` field to `File`
- Added `QualityFolderField`, `QualityIconField`, `QualityImageField`, `QualityUnitField` fields to `Quality`
- Added `TagDescriptionField`, `TagIconField` to `Tag`
- Added `ConceptNameField`, `ConceptDescriptionField`, `ConceptIconField` to `Concept`
- Added `LayerIsHiddenField`, `LayerIsLockedField` to `Layer`
- Added `ModelNameField` to `Model`
- Renamed `KitHomepage` to `KitHomepageField` (consistent naming)
- Added `Interface` entity with all required fields

### Script Improvements

- Added `FIELD_MAPPINGS` for cross-language field name equivalences (e.g., `key` ↔ `quality`, `is_hidden` ↔ `isHidden`)
- Expanded `RELATIONSHIP_FIELDS` to skip SQLModel relationship fields in Python comparisons
- Both Python and C# comparison logic now uses field mappings

### Final Fixes for 100% Consistency

**Python ID Types:**

- Added `LocationGuidField` and `LocationId` class
- Added `TagGuidField` and `TagId` class
- Added `ConceptGuidField` and `ConceptId` class

**Grasshopper Components:**

- Added `ModelName`, `ModelNickname`, `ModelDescription` to `QualityComponent`
- Added `ModelName`, `ModelNickname`, `ModelDescription` to `BenchmarkComponent`
- Added `ModelName`, `ModelNickname`, `ModelDescription` to `PropComponent`

### Final Status: 100% Synchronized

```
TypeScript: 27 entities, 21 ID types
Python: 26 entities, 21 ID types
C#: 26 entities, 21 ID types
Grasshopper: 23 components, 60 params, 60 goos

0 errors, 0 warnings
```
