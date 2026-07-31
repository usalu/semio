# Ticket

## Todos

# Kit Import/Export Implementation

## Plan

### 1. Move Import/Export Logic to compose.ts

Extract the import/export logic from kit commands and make it reusable:

- `importKit(url: string): Promise<{ kit: Kit, files: Map<string, Blob> }>` - Fetch, unzip, parse kit.db, return kit and files
- `exportKit(kit: Kit, files: Map<string, Blob>): Promise<Blob>` - Create .compose/kit.db, zip everything, return blob

### 2. Implementation Details

#### Import

1. Fetch the URL (could be remote HTTP or local file://)
2. Unzip the archive
3. Read `.compose/kit.db` SQLite file
4. Parse SQLite schema to Kit JSON structure
5. Collect all files (paths relative to zip root)
6. Return `{ kit, files }`

#### Export

1. Convert Kit JSON to SQLite schema
2. Create `.compose/kit.db` in memory
3. Add all files to zip
4. Add `.compose/kit.db` to zip
5. Return zip as Blob

### 3. Test Strategy

Use `examples/metabolism` as test case:

1. Load metabolism kit JSON from `assets`
2. Load pure files from `examples/metabolism` folder (excluding `.compose`)
3. Export to zip blob
4. Import from zip blob
5. Verify:
   - Kit structure matches (deep equal)
   - All files present and content matches
   - Round-trip is lossless

### 4. Dependencies

- `jszip` - For zip/unzip operations ✓️ (already installed)
- `sql.js` - For SQLite in-memory operations ✓️ (already installed)
- Schema mapping between JSON and SQLite (from sqlite/schema.sql)

### 5. Implementation Steps

1. Add import/export functions to compose.ts ✓️
2. Create SQLite ↔ JSON converters ✓️
3. Implement zip handling ✓️
4. Create comprehensive test ✓️
5. Verify test passes ✓️

## Implementation Status

- [x] Plan created
- [x] Import function implemented
- [x] Export function implemented
- [x] SQLite ↔ JSON converters
- [x] Test created
- [x] Test passing

## Notes

### SQLite Schema Updates

The implementation uses a GUID-based schema instead of the legacy integer-based schema:

- Primary keys use VARCHAR(36) for GUIDs
- Both `guid` and legacy `id` columns are maintained for compatibility
- Composite unique constraints handle duplicate GUIDs with different parents (e.g., connectors shared across types)
- AUTOINCREMENT row_id is used as the actual primary key

### Test Results

The roundtrip test successfully:

- Exports the metabolism kit to a zip blob
- Imports it back from the blob
- Verifies kit metadata matches
- Verifies all types are preserved (with GUID-based matching)
- Verifies all designs are preserved
- Handles edge cases like undefined vs false for boolean properties

## Changes

## Log

## Summary
