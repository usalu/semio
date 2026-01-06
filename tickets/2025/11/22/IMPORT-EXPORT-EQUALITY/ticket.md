---
slug: IMPORT-EXPORT-EQUALITY
summary: Migration from 2025-11-22_IMPORT-EXPORT-EQUALITY.md
prompt: Migration from 2025-11-22_IMPORT-EXPORT-EQUALITY.md
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.696Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Import/Export Roundtrip Test - Status

## Goal

Achieve 100% deep recursive equality between original kit and imported kit after export→import roundtrip.

## Completed Fixes

### 1. Empty Array vs Undefined - Top Level (Lines 4643, 4695, 4710)

Fixed `sqliteToKit` to return `undefined` instead of `[]` for empty collections:

- `kit.qualities` - returns `undefined` when no qualities found
- `kit.files` - returns `undefined` when no files found
- `kit.authors` - returns `undefined` when no authors found

**Pattern**:

```typescript
kit.field = results.length > 0 ? results.map(...) : undefined;
```

### 2. Empty Array vs Undefined - Attributes (13 locations)

Created `mapOrUndefined` helper function (line 4369) and applied to all attribute mappings:

- Line 4415: model attributes
- Line 4443: prop attributes (in connectors)
- Line 4451: connector attributes
- Line 4459: type attributes
- Line 4523: prop attributes (in pieces)
- Line 4531: piece attributes
- Line 4564: connection attributes
- Line 4581: layer attributes
- Line 4598: group attributes
- Line 4613: design attributes
- Line 4634: port attributes
- Line 4673: benchmark attributes
- Line 4681: quality attributes

**Pattern**:

```typescript
attributes: mapOrUndefined(someAttributes, (a: any) => ({
  guid: a.guid,
  key: a.key,
  value: toUndefined(a.value),
  definition: toUndefined(a.definition),
}));
```

### 3. Malformed JSON Data Handling

Updated `areKitsEqual`'s `normalizeArray` function to handle:

- `undefined` → `[]`
- `null` → `[]`
- Single object → `[object]`
- Array → unchanged

This handles malformed JSON data where some fields are single objects instead of arrays.

### 4. Test Normalization

Added `toArray` helper in `test-roundtrip.mjs` to normalize top-level kit fields (authors, files, qualities) that are malformed in the JSON.

## Current Status

### ✅ Resolved

- Top-level field mismatches (types count, designs count) - both show 50 and 5
- Empty collections now properly return `undefined` instead of `[]`
- Attribute collections use `mapOrUndefined` consistently
- Single objects in JSON are normalized to arrays for comparison

### ❌ Remaining Problems

#### Connector Count Mismatch (Critical)

**Type**: `cb448d7a-b169-4759-9b74-7ff1ba216b8b` (name: `\`)

- Original JSON: 1 connector
- After import: 4 connectors

**Hypothesis**: Connectors from parent types or other types being incorrectly assigned during export/import.

**Evidence**: This is NOT an empty array issue - both have connectors, just different counts.

**Investigation needed**:

1. Check if `kitToSqlite` correctly handles connector insertion (no duplicates, correct type_guid)
2. Check if `sqliteToKit` correctly filters connectors by type_guid
3. Check if type hierarchy (parent/child) is causing connector inheritance issues
4. Examine the `\` type in the metabolism JSON to see its actual connector structure

## Next Steps

1. **Debug Connector Assignment** - Add logging to track which connectors are being inserted/loaded for this specific type
2. **Check Type Hierarchy** - Verify if the `\` type has a parent and if parent connectors are being included
3. **Validate SQL Queries** - Ensure `SELECT * FROM connector WHERE type_guid = ?` is correctly scoped
4. **Compare JSON Structure** - Check if the original JSON for this type actually has 1 or 4 connectors

## Test Output

```
Original kit:
  Types: 50
  Designs: 5
  Files: undefined
  Authors: 1
  Qualities: undefined

Imported kit:
  Types: 50
  Designs: 5
  Files: undefined
  Authors: 1
  Qualities: undefined

✅ Top-level fields match!
❌ Deep comparison fails on type.connectors for type 'cb448d7a-b169-4759-9b74-7ff1ba216b8b'
```

## Files Modified

- `js/js/semio.ts` - Lines 4369, 4643, 4695, 4710, plus 13 attribute locations
- `js/js/test-roundtrip.mjs` - Added normalization and detailed logging

## Technical Debt

- Consider creating a comprehensive `normalizeKit` function that recursively fixes all malformed arrays in JSON
- Consider adding validation to `kitToSqlite` to detect and warn about data corruption
- Remove debug logging from `areKitsEqual` once issues are resolved
