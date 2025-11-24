---
date: '2025-11-21T23:00:00.000Z'
slug: IMPORT-EXPORT-FIXES
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Migration from 2025-11-22_IMPORT-EXPORT-FIXES.md
model: unknown
---
# Import/Export Fixes - 2025-01-22

## Issues Identified from Test

### 1. License Field ✓ FIXED

- **Error**: `kit.license: type string vs undefined`
- **Status**: Import code exists at line 4631, issue is likely with `toUndefined()` converting empty strings to undefined
- **Root Cause**: `toUndefined` converts both null AND empty strings to undefined, but export uses `|| null` which converts empty strings to null
- **Fix Required**: Handle empty strings consistently

### 2. Connections Not Imported

- **Error**: `kit.designs[0].connections: array length 179 vs 0`
- **Status**: Import code exists at lines 4723, 4805-4835, but returns 0 connections
- **Root Cause**: Either connections not exported OR import issue
- **Investigation Needed**: Check if connections are being exported to the database

### 3. Attribute Definitions

- **Error**: Multiple attributes showing `definition: type string vs undefined`
- **Status**: `toUndefined()` converts empty strings to undefined
- **Root Cause**: Original data has empty strings `""`, export converts to `null`, import converts to `undefined`
- **Fix Required**: Preserve empty strings vs undefined distinction

### 4. Piece Design Reference

- **Error**: `kit.designs[0].pieces[0].design: missing in a`
- **Status**: Import reads `design_guid_ref` field at line 4761
- **Root Cause**: Unclear, need to verify export

## Code Changes Made

1. Fixed `design_prop` schema PRIMARY KEY (guid) instead of composite key
2. Added `key` field to prop INSERT statements
3. Added piece_prop junction table population
4. Added `props` field to Piece schema
5. Fixed boolean field handling for NOT NULL DEFAULT 0 fields:
   - type.virtual: use 0 instead of null
   - type.isAbstract: use 0 instead of null
   - port.mandatory: use 0 instead of null
6. Fixed Quality import to use `defaultValue` instead of `default`
7. Fixed boolean import to return undefined instead of false for nullable booleans

## Next Steps

1. Debug connection export - check if validation is too strict
2. Fix string/undefined handling for optional text fields
3. Test piece design reference export/import
