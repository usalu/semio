---
slug: SQL-TYPESCRIPT-COMPLIANCE
summary: Migration from 2025-11-22_SQL-TYPESCRIPT-COMPLIANCE.md
prompt: Migration from 2025-11-22_SQL-TYPESCRIPT-COMPLIANCE.md
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-16T17:06:07.704Z'
commit: '0000000000000000000000000000000000000000'
iterations: []
---

# SQL Schema Compliance with TypeScript (2025-11-22)

## Goal

Make SQL schema and import/export code 100% compliant with TypeScript schemas as the single source of truth.

## TypeScript Schema Analysis

### Design Schema (from DesignSchema)

```typescript
{
  guid: string,
  name: string,
  parent?: DesignId,
  isAbstract?: boolean,           // ❌ MISSING in SQL
  folder?: string,                // ❌ MISSING in SQL
  pieces?: Piece[],
  connections?: Connection[],
  stats?: Stat[],
  props?: Prop[],                 // ❌ MISSING junction table in SQL
  layers?: Layer[],
  activeLayer?: LayerId,          // ✅ EXISTS as active_layer_guid
  groups?: Group[],
  canScale?: boolean,             // ❌ MISSING in SQL
  canMirror?: boolean,            // ❌ MISSING in SQL
  unit?: string,                  // ✅ EXISTS
  location?: LocationId,          // ✅ EXISTS as location_guid
  authors?: AuthorId[],           // ⚠️ EXISTS but no junction table
  concepts?: string[],            // ✅ EXISTS via design_concept table
  icon?: string,                  // ✅ EXISTS
  image?: string,                 // ✅ EXISTS
  description?: string,           // ✅ EXISTS
  attributes?: Attribute[],       // ✅ EXISTS via attribute table
  createdAt: Date,                // ✅ EXISTS as created
  updatedAt: Date,                // ✅ EXISTS as updated
}
```

### Side Schema (from SideSchema)

```typescript
{
  piece: PieceId,                 // ✅ piece.guid reference
  designPiece?: PieceId,          // ✅ designPiece.guid reference
  port: PortId,                   // ✅ port.guid reference
}
```

**IMPORTANT**: Side does NOT have a `guid` property!

### Stat Schema (from StatSchema)

```typescript
{
  guid: string,
  quality: QualityId,
  unit?: string,
  min?: number,
  minExcluded?: boolean,          // ❌ MISSING in SQL
  max?: number,
  maxExcluded?: boolean,          // ❌ MISSING in SQL
}
```

## Required SQL Schema Changes

### 1. Update `design` table

```sql
ALTER TABLE design ADD COLUMN is_abstract BOOLEAN NOT NULL DEFAULT 0;
ALTER TABLE design ADD COLUMN folder VARCHAR(256);
ALTER TABLE design ADD COLUMN can_scale BOOLEAN NOT NULL DEFAULT 0;
ALTER TABLE design ADD COLUMN can_mirror BOOLEAN NOT NULL DEFAULT 0;
```

### 2. Create `design_prop` junction table

```sql
CREATE TABLE design_prop (
	design_guid VARCHAR(36) NOT NULL,
	prop_guid VARCHAR(36) NOT NULL,
	PRIMARY KEY (design_guid, prop_guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid),
	FOREIGN KEY(prop_guid) REFERENCES prop (guid)
);
```

### 3. Create `design_author` junction table

```sql
CREATE TABLE design_author (
	design_guid VARCHAR(36) NOT NULL,
	author_guid VARCHAR(36) NOT NULL,
	rank INTEGER NOT NULL DEFAULT 0,
	PRIMARY KEY (design_guid, author_guid),
	FOREIGN KEY(design_guid) REFERENCES design (guid),
	FOREIGN KEY(author_guid) REFERENCES author (guid)
);
```

### 4. Update `stat` table

```sql
ALTER TABLE stat ADD COLUMN min_excluded BOOLEAN NOT NULL DEFAULT 0;
ALTER TABLE stat ADD COLUMN max_excluded BOOLEAN NOT NULL DEFAULT 0;
```

### 5. Remove `design_guid` from `author` table

The author table has a direct design_guid FK which is wrong - authors should be linked via junction table:

```sql
-- This column should be removed as it conflicts with the many-to-many relationship
-- ALTER TABLE author DROP COLUMN design_guid;
-- However, for backwards compatibility, keep it but use design_author junction table instead
```

## Required Code Changes in semio.ts

### 1. Fix `sqliteToKit` - Remove Side GUIDs

**Location**: Line ~4560

**Current Code** (WRONG):

```typescript
connected: {
  guid: guid(),  // ❌ WRONG - Side doesn't have guid
  piece: { guid: c.connected_piece_guid },
  designPiece: c.connected_design_piece_guid ? { guid: c.connected_design_piece_guid } : undefined,
  port: { guid: c.connected_port_guid },
},
```

**Fixed Code**:

```typescript
connected: {
  piece: { guid: c.connected_piece_guid },
  designPiece: c.connected_design_piece_guid ? { guid: c.connected_design_piece_guid } : undefined,
  port: { guid: c.connected_port_guid },
},
```

### 2. Fix `sqliteToKit` - Load Design Props

**Location**: After loading designs (~line 4520)

**Add**:

```typescript
const designProps = execResult("SELECT prop.* FROM prop JOIN design_prop ON prop.guid = design_prop.prop_guid WHERE design_prop.design_guid = ?", [designGuid]);
```

And in the design object:

```typescript
props: mapOrUndefined(designProps, (pr: any) => {
  const propAttributes = execResult("SELECT * FROM attribute WHERE prop_guid = ?", [pr.guid]);
  return {
    guid: pr.guid,
    key: pr.key,
    value: pr.value,
    unit: toUndefined(pr.unit),
    quality: pr.quality_guid ? { guid: pr.quality_guid } : undefined,
    attributes: mapOrUndefined(propAttributes, (a: any) => ({
      guid: a.guid,
      key: a.key,
      value: toUndefined(a.value),
      definition: toUndefined(a.definition),
    })),
  };
}),
```

### 3. Fix `sqliteToKit` - Load Design Authors

**Location**: After loading designs

**Add**:

```typescript
const designAuthors = execResult("SELECT author_guid FROM design_author WHERE design_guid = ? ORDER BY rank", [designGuid]);
```

And in the design object:

```typescript
authors: designAuthors.length > 0 ? designAuthors.map((a: any) => ({ guid: a.author_guid })) : undefined,
```

### 4. Fix `sqliteToKit` - Add Design Properties

**Location**: In design object construction

**Add**:

```typescript
isAbstract: Boolean(row.is_abstract),
folder: toUndefined(row.folder),
canScale: Boolean(row.can_scale),
canMirror: Boolean(row.can_mirror),
```

### 5. Fix `sqliteToKit` - Add Stat Properties

**Location**: In stats mapping

**Current**:

```typescript
stats: stats.map((s: any) => ({
  guid: s.guid,
  quality: { guid: s.quality_guid },
  min: s.min_value,
  max: s.max_value,
  unit: toUndefined(s.unit),
})),
```

**Fixed**:

```typescript
stats: stats.map((s: any) => ({
  guid: s.guid,
  quality: { guid: s.quality_guid },
  min: s.min_value,
  minExcluded: Boolean(s.min_excluded),
  max: s.max_value,
  maxExcluded: Boolean(s.max_excluded),
  unit: toUndefined(s.unit),
})),
```

### 6. Fix `kitToSqlite` - Export Design Props

**Location**: After exporting design (~line 5430)

**Add**:

```typescript
// Export design props
if (design.props) {
  for (const prop of design.props) {
    db.run("INSERT INTO prop (guid, key, value, unit, quality_guid) VALUES (?, ?, ?, ?, ?)", [prop.guid, prop.key, prop.value, prop.unit ?? null, prop.quality?.guid ?? null]);
    db.run("INSERT INTO design_prop (design_guid, prop_guid) VALUES (?, ?)", [design.guid, prop.guid]);
    if (prop.attributes) {
      for (const attr of prop.attributes) {
        db.run("INSERT INTO attribute (guid, key, value, definition, prop_guid) VALUES (?, ?, ?, ?, ?)", [attr.guid, attr.key, attr.value ?? null, attr.definition ?? null, prop.guid]);
      }
    }
  }
}
```

### 7. Fix `kitToSqlite` - Export Design Authors

**Location**: After exporting design

**Add**:

```typescript
// Export design authors
if (design.authors) {
  for (let i = 0; i < design.authors.length; i++) {
    const authorId = design.authors[i];
    db.run("INSERT INTO design_author (design_guid, author_guid, rank) VALUES (?, ?, ?)", [design.guid, authorId.guid, i]);
  }
}
```

### 8. Fix `kitToSqlite` - Update Design INSERT

**Location**: Line ~5410

**Current**:

```typescript
"INSERT INTO design (guid, name, parent_guid, unit, description, icon, image, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
```

**Fixed**:

```typescript
("INSERT INTO design (guid, name, parent_guid, is_abstract, folder, unit, location_guid, active_layer_guid, can_scale, can_mirror, description, icon, image, created, updated, kit_guid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
  [
    design.guid,
    design.name,
    design.parent?.guid ?? null,
    design.isAbstract ? 1 : 0,
    design.folder ?? null,
    design.unit ?? null,
    design.location?.guid ?? null,
    design.activeLayer?.guid ?? null,
    design.canScale ? 1 : 0,
    design.canMirror ? 1 : 0,
    design.description ?? null,
    design.icon ?? null,
    design.image ?? null,
    design.createdAt.toISOString(),
    design.updatedAt.toISOString(),
    kit.guid,
  ]);
```

### 9. Fix `kitToSqlite` - Update Stat INSERT

**Location**: Where stats are inserted

**Add** min_excluded and max_excluded columns and values.

## Implementation Order

1. ✅ Update SQL schema file (schema.sql)
2. ✅ Update kitToSqlite to export new properties
3. ✅ Update sqliteToKit to import new properties
4. ✅ Remove Side guid generation
5. ✅ Test roundtrip with full deep equality

## Testing

After implementation, the test should pass with full `areKitsEqual` check, not just critical data preservation.

---

## ✅ IMPLEMENTATION COMPLETED (2025-11-22)

All changes have been successfully implemented and the test now passes with full deep equality!

### Changes Made

#### 1. SQL Schema Updates (schema.sql)

- ✅ Added `is_abstract`, `folder`, `can_scale`, `can_mirror` columns to design table
- ✅ Created `design_prop` junction table for design ↔ quality many-to-many
- ✅ Created `design_author` junction table for design ↔ author many-to-many
- ✅ Added `min_excluded`, `max_excluded` columns to stat table

#### 2. Embedded Schema Updates (semio.ts embedded schema string)

- ✅ Updated design table definition with new columns
- ✅ Added design_prop table definition
- ✅ Added design_author table definition
- ✅ Updated stat table definition with excluded flags

#### 3. Export Code Updates (kitToSqlite in semio.ts)

- ✅ Updated design INSERT to include is_abstract, folder, can_scale, can_mirror
- ✅ Added design_prop export loop
- ✅ Added design_author export loop with rank
- ✅ Updated stat INSERT to include min_excluded, max_excluded

#### 4. Import Code Updates (sqliteToKit in semio.ts)

- ✅ **CRITICAL**: Removed `guid: guid()` from connection sides (lines 4595, 4601)
- ✅ Added design_prop query and mapping
- ✅ Added design_author query and mapping (ordered by rank)
- ✅ Added isAbstract, folder, canScale, canMirror to design object
- ✅ Added minExcluded, maxExcluded to stat mapping

#### 5. Test Updates (test-roundtrip.mjs)

- ✅ Removed Side GUID workaround from cleanKitAttributes
- ✅ Removed design property workarounds (activeLayer, concepts, parent, groups, layers, stats, unit)
- ✅ Removed piece property workarounds (design, mirrorPlane, scale)
- ✅ Changed test to use full `areKitsEqual()` check instead of critical data preservation

### Test Results

```
Loading metabolism kit...
Kit statistics:
  Types: 50
  Designs: 5
  Authors: 1
  Files: 0
  Qualities: 0

1. Exporting kit to zip...
   Exported to: C:\git\semio.tech\semio\assets\metabolism.zip
   File size: 2568.21 KB

2. Importing kit from zip...
   Imported kit: Metabolism
   Types: 50
   Designs: 5
   Files: 0

3. Comparing original and imported kits...
✅ SUCCESS: Full deep equality achieved!

Roundtrip test passed:
  - 50 types preserved
  - 5 designs preserved
  - 3580 connections preserved

  All kit data matches exactly - SQL schema is 100% TypeScript compliant!
```

### Files Modified

1. `sql/sqlite/schema.sql` - Added columns and tables
2. `js/js/semio.ts` - Updated embedded schema, kitToSqlite, and sqliteToKit
3. `js/js/test-roundtrip.mjs` - Removed workarounds, enabled full equality check

### Verification

- ✅ No TypeScript errors
- ✅ No SQL errors
- ✅ Test passes with exit code 0
- ✅ Full deep equality using `areKitsEqual()`
- ✅ All 3580 connections preserved
- ✅ File size 2568 KB (consistent with connection preservation)

## Conclusion

The SQL schema is now 100% compliant with TypeScript schemas. All TypeScript properties are exported to SQL and imported back correctly. No workarounds or normalizations needed in tests. The roundtrip test verifies complete data fidelity.
