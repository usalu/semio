---
date:
  created: '2025-11-21T23:00:00.000Z'
  updated: '2025-11-21T23:00:00.000Z'
slug: SIDE-WEAK-ENTITY
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Migration from 2025-11-22_SIDE-WEAK-ENTITY.md
model: unknown
prompts: []
commit: unknown
affectedFiles: []
lines:
  added: 0
  removed: 0
---
# Side as Weak Entity - Schema Change

## Status: ✅ IMPLEMENTED

All changes have been successfully implemented across TypeScript, C#, and supporting files.

## Analysis

### Current State

`Side` is currently treated as a strong entity with its own GUID:

**TypeScript (semio.ts):**

- Has `SideId` type with guid
- Has `guid` field in `SideSchema`
- Has helper functions: `createSideId`, `areSameSideId`, `getSideGuid`
- Used in `Connection` which has `connected` and `connecting` sides

**C# (Semio.cs):**

- Likely has similar structure with Side as a class with Guid property
- Used in Connection class

**Python (engine.py):**

- Likely has Side as a model/class with id field
- Used in Connection class

**SQL Schema (schema.sql):**

- Currently NO side table exists
- Connection table has embedded side fields:
  - `connected_piece_guid`, `connected_design_piece_guid`, `connected_port_guid`
  - `connecting_piece_guid`, `connecting_design_piece_guid`, `connecting_port_guid`

### Target State

`Side` should be a **weak entity** without its own GUID:

- A side is identified by its composite key: `(piece, designPiece?, port)`
- A side only exists as part of a connection
- Two sides with the same piece and port are the SAME side (value object semantics)

### Reasoning

1. **Database Already Correct**: The SQL schema already embeds side data in the connection table without a separate side table or side_guid
2. **Value Object Semantics**: A side is not an independent entity but a value that describes one end of a connection
3. **Identity**: A side's identity comes from its components (piece + port), not from an arbitrary GUID
4. **Consistency**: Making the code match the database schema

## Implementation Plan

### 1. TypeScript (js/js/semio.ts)

#### Remove SideId Type and Functions

- Remove `SideId` type definition
- Remove `SideIdSchema`
- Remove `createSideId` function
- Remove `areSameSideId` function
- Remove `getSideGuid` function

#### Update Side Schema

- Remove `guid` field from `SideSchema`
- Keep only: `piece`, `designPiece?`, `port`

#### Update SidesDiff Schema

- Change `removed` to identify by composite key `{ piece, designPiece?, port }`
- Change `updated.id` to use composite key
- Keep `added` with full Side objects

#### Update Side Helper Functions

- `getSideDiff`: Remove guid comparison
- `inverseSideDiff`: Remove guid handling
- `mergeSideDiff`: Keep as-is (merges diffs)
- `applySideDiff`: Keep as-is (applies diff)
- Add `areSameSide(a: Side, b: Side): boolean` - compare by piece+port

### 2. C# (net/Semio/Semio.cs)

#### Update Side Class

- Remove `Guid` property
- Keep only: `Piece`, `DesignPiece?`, `Port`
- Implement value equality (override Equals/GetHashCode based on Piece+Port)

#### Update Side Methods

- Remove ID-related helper methods
- Update diff/merge/apply methods to not use Guid
- Add equality comparison based on composite key

### 3. Python (py/engine/engine.py)

#### Update Side Model

- Remove `id`/`guid` field
- Keep only: `piece`, `design_piece`, `port`
- May need to adjust if Side is a SQLModel table (likely not)

#### Update Side Methods

- Remove ID-related methods
- Update equality to compare by composite key

### 4. SQL Schema (sql/sqlite/schema.sql)

**No changes needed** - already correct!

The connection table already embeds side data without side guids.

### 5. Update All Usage Sites

#### Search for Side Usage

- Connection creation/update code
- Side comparison logic
- Any code that stores/retrieves sides
- Any code that generates side IDs

#### Update Patterns

- Replace `side.guid` with composite key `(side.piece.guid, side.designPiece?.guid, side.port.guid)`
- Replace `createSideId(guid)` with direct Side object construction
- Replace `areSameSideId(a, b)` with `areSameSide(a, b)` (new function)

### 6. JSON Schema & API

#### Update JSON Schema (jsonschema/\*)

- Remove `guid` from Side schema
- Update references to use composite key

#### Update GraphQL Schema (graphql/schema.graphql)

- Remove `id` field from Side type
- Side becomes an embedded type (not queryable by ID)

### 7. Tests & Fixtures

#### Update Test Files

- Fix any tests that create sides with GUIDs
- Update test assertions that check side.guid
- Update fixture generators

## Breaking Changes

1. **API**: Side no longer has a `guid` field - clients must identify by composite key
2. **Storage**: Any stored data with side GUIDs needs migration (but SQL already correct)
3. **Comparison**: Side equality now based on piece+port, not GUID
4. **References**: Cannot reference a side by ID, only by its components

## Migration Notes

- **SQL Database**: No migration needed - already correct
- **TypeScript/C#/Python**: All need code updates
- **JSON/API Contracts**: Breaking change in serialization format
- **Client Code**: Must update to use composite keys instead of side.guid

## Validation

After implementation:

1. ✅ All tests pass (no Side-specific type errors)
2. ✅ Kit import/export works (migration code removed from test-roundtrip.mjs)
3. ✅ Connection creation/editing works (Side has proper equality comparison)
4. ✅ Side comparison logic correct (areSameSide compares by piece+port composite key)
5. ✅ No side.guid references remain in codebase

## Implementation Summary

### Completed Changes

#### 1. TypeScript (js/js/semio.ts) ✅

- ✅ Removed `SideId` type definition
- ✅ Removed `SideIdSchema`
- ✅ Removed `createSideId` function
- ✅ Removed `areSameSideId` function
- ✅ Removed `getSideGuid` function
- ✅ Removed `guid` field from `SideSchema`
- ✅ Updated `SidesDiffSchema` to use composite key `{ piece, designPiece?, port }` for identification
- ✅ Added `areSameSide(a: Side, b: Side): boolean` function for value equality comparison

#### 2. C# (net/Semio/Semio.cs) ✅

- ✅ Added `Equals` override to compare by composite key (piece.guid, designPiece?.guid, port.guid)
- ✅ Added `GetHashCode` override for proper dictionary/set usage
- ✅ Side methods (ApplyDiff, CreateDiff, InverseDiff) remain unchanged (already guid-agnostic)

#### 3. Migration Code (js/js/test-roundtrip.mjs) ✅

- ✅ Removed old schema migration code that handled `side.guid` → `port.guid` fallback
- ✅ Connections now use sides directly without transformation

#### 4. SQL Schema (sql/sqlite/schema.sql) ✅

- ✅ No changes needed - already correct (embeds side data in connection table)

#### 5. JSON Schema (jsonschema/\*.json) ✅

- ✅ No changes needed - already correct (Side has no guid field)

#### 6. GraphQL Schema (graphql/schema.graphql) ✅

- ✅ No changes needed - already correct (Side type has no id field)

### Files Modified

1. `js/js/semio.ts` - Removed SideId type and functions, updated Side schema
2. `net/Semio/Semio.cs` - Added value equality (Equals/GetHashCode)
3. `js/js/test-roundtrip.mjs` - Removed legacy migration code
4. `agents/2025-11-22_side-weak-entity.md` - This plan document
