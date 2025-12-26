---
slug: KIT-IMPORT-EXPORT-DIAGNOSIS
summary: Migration from 2025-11-22_KIT-IMPORT-EXPORT-DIAGNOSIS.md
prompt: Migration from 2025-11-22_KIT-IMPORT-EXPORT-DIAGNOSIS.md
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.699Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Kit Import/Export Diagnosis

## Issues Found

### 1. Layer and Group GUID Generation ✅ FIXED

- **Problem**: In `kitToSqlite`, we're not preserving layer and group GUIDs
- **Current**: Creating new GUIDs with `const layerGuid = guid()` and `const groupGuid = guid()`
- **Expected**: Use the existing `layer.guid` and `group.guid`
- **Status**: Fixed - now using existing GUIDs

### 2. Interface Compatible Property Name Mismatch ✅ FIXED

- **Problem**: Schema uses `compatibleInterfaces` but SQL uses `compatible`
- **Current**: Reading as `compatible` in `sqliteToKit`
- **Expected**: Should be `compatibleInterfaces`
- **Status**: Fixed - now using `compatibleInterfaces`

### 3. Missing Prop.key in Connector Props ✅ FIXED

- **Problem**: Connector props don't have a `key` field in the SQL schema
- **Current**: Prop table has `key` but connector props don't use it
- **Expected**: Connector props should not require `key` field
- **Status**: Fixed - removed key requirement for connector props

### 4. Design Props Missing GUID ✅ FIXED

- **Problem**: Design props are being created without preserving GUID
- **Current**: Creating new GUID in `sqliteToKit`
- **Expected**: Need to store and retrieve design prop GUIDs
- **Status**: Fixed - added guid column to design_prop table

### 5. Date Serialization Issues

- **Problem**: Dates are stored as ISO strings but returned as Date objects
- **Current**: `new Date(row.created)` creates Date objects
- **Expected**: Should match the original kit format (strings or objects?)
- **Need to verify**: Are dates strings or Date objects in the original kit?

### 6. Empty String vs Undefined Normalization

- **Problem**: `toUndefined` converts empty strings to undefined, but original kit has empty strings
- **Current**: `toUndefined(value)` converts `""` and `null` to `undefined`
- **Expected**: Need to preserve empty strings as-is or always normalize
- **Decision**: The schema should define what's allowed

### 7. Optional Properties Appearing as Undefined

- **Problem**: Properties like `interface`, `props`, `attributes`, `models`, `concepts`, `authors`, `parent` are undefined in original but being read from SQL
- **Current**: Using `mapOrUndefined` which returns undefined for empty arrays
- **Expected**: These should NOT be set at all if they're empty/null in SQL
- **Impact**: Comparison fails because original doesn't have the property, but imported has it as undefined

### 8. Missing Top-Level Kit Properties

- **Problem**: `preview`, `interfaces`, `qualities`, `files`, `folders`, `concepts`, `attributes` missing from original
- **Likely**: These are optional and the original kit doesn't have them set
- **Expected**: Import should also leave them undefined if not in SQL

## Root Causes

The main issues are:

1. **Inconsistent null/undefined handling**: SQL NULL → JavaScript needs consistent conversion
2. **Empty collection handling**: Empty arrays in SQL should become undefined, not empty arrays
3. **Date type consistency**: Need to decide if dates are strings or Date objects

## Implementation Plan

1. ✅ Fix layer GUID preservation in `kitToSqlite`
2. ✅ Fix group GUID preservation in `kitToSqlite`
3. ✅ Fix interface compatible interfaces naming
4. ✅ Fix prop key handling for connector props
5. ✅ Add design_prop table GUID column
6. ⏳ Fix date serialization (keep as strings in SQL, return as strings or Date objects consistently)
7. ⏳ Fix empty string vs undefined (preserve original format or normalize consistently)
8. ⏳ Fix optional properties (don't add undefined properties that weren't in original)
