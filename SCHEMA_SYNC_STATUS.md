# Schema Synchronization Status

## Overview

This document tracks the synchronization of entity schemas across TypeScript (source of truth), C#, and Python implementations.

**Last Updated**: Schema synchronization session
**TypeScript Source**: `js/js/semio.ts`
**C# Implementation**: `net/Semio/Semio.cs`
**Python Implementation**: `py/engine/engine.py`

## Critical Breaking Changes

### ID System Migration
- **Old**: Composite keys (Name+Variant for Type, Name+Variant+View for Design, Name for Kit)
- **New**: GUID-based identification for all entities
- **Impact**: All existing code using composite keys needs migration

### Temporal Fields
- Added `createdAt`, `createdBy`, `updatedAt`, `updatedBy` to multiple entities
- Enables audit trail and versioning

### Hierarchical Organization
- Added `parent` field to Type, Design, Folder for hierarchy
- Added `folder` field to Type, Design, File for organization
- Added `folders` array to Kit

## Completed Work

### ✅ File Entity (All Implementations)

#### C# (`Semio.cs`)
- ✅ **FileId**: Changed from `Url` to `Guid`
- ✅ **File** class: Added all 10 fields
  - `Guid` (string)
  - `Name` (string)
  - `Remote` (string?)
  - `Folder` (string?)
  - `Size` (int?)
  - `Hash` (string?)
  - `CreatedAt` (string)
  - `CreatedBy` (string?)
  - `UpdatedAt` (string)
  - `UpdatedBy` (string?)
- ✅ **FileDiff**: Updated all fields to nullable
- ✅ **FilesDiff**: Updated to use new FileId
- ✅ **File.ApplyDiff**: Updated method
- ✅ **FileDiff.MergeDiff**: Updated method
- ✅ **Implicit operators**: Updated conversions

#### Python (`engine.py`)
- ✅ **File model**: Complete implementation with all fields
- ✅ **Field mixins**: FileGuidField, FileNameField, FileRemoteField, FileFolderField, FileSizeField, FileHashField, FileCreatedAtField, FileCreatedByField, FileUpdatedAtField, FileUpdatedByField
- ✅ **FileId, FileProps, FileInput, FileContext, FileOutput**: All classes defined
- ✅ **Kit.files_**: Relationship added to Kit model

### ✅ Type Entity (C# Only)

#### C# (`Semio.cs`)
- ✅ **TypeId**: Changed from `Name` + `Variant` to `Guid`
  - `public string Guid { get; set; } = "";`
  - Updated `ToIdString()` to use Guid
  - Updated implicit operators
- ✅ **Type** class: Added 6 new fields
  - `Guid` (string) - PRIMARY KEY
  - `Parent` (string?) - hierarchy
  - `IsAbstract` (bool) - virtual type flag
  - `Folder` (string?) - organization
  - `CreatedAt` (string) - audit
  - `UpdatedAt` (string) - audit
  - Changed `Scalable`/`Mirrorable` to `CanScale`/`CanMirror`
- ✅ **TypeDiff**: Updated all new fields to nullable
- ✅ **TypeDiff.MergeDiff**: Updated merge logic
- ✅ **Type.ApplyDiff**: Updated application logic
- ✅ **Implicit operators**: Updated conversions

#### Python (`engine.py`)
- ❌ **NOT IMPLEMENTED** - Needs complete Type model with:
  - TypeGuidField, TypeNameField, TypeVariantField, etc.
  - TypeId, TypeProps, TypeInput, TypeContext, TypeOutput
  - Type(TableEntity) with all fields and relationships

### ✅ Design Entity (C# Only)

#### C# (`Semio.cs`)
- ✅ **DesignId**: Changed from `Name` + `Variant` + `View` to `Guid`
  - `public string Guid { get; set; } = "";`
  - Updated `ToIdString()` to use Guid
  - Updated implicit operators
- ✅ **Design** class: Added 9 new fields
  - `Guid` (string) - PRIMARY KEY
  - `Parent` (string?) - hierarchy
  - `IsAbstract` (bool) - virtual design flag
  - `Folder` (string?) - organization
  - `ActiveLayer` (string?) - UI state
  - `Props` (List<Prop>?) - design properties
  - `Layers` (List<Layer>?) - layer organization
  - `Groups` (List<Group>?) - piece groups
  - `CanScale` (bool) - scaling capability
  - `CanMirror` (bool) - mirroring capability
  - `CreatedAt` (string) - audit
  - `UpdatedAt` (string) - audit
- ✅ **DesignDiff**: Updated all fields
  - All fields now nullable with `?`
  - Added all new fields
- ✅ **DesignDiff.MergeDiff**: Updated merge logic with all new fields
- ✅ **Design.ApplyDiff**: Updated application logic
- ✅ **Implicit operators**: Updated conversions

#### Python (`engine.py`)
- ❌ **NOT IMPLEMENTED** - Needs complete Design model with:
  - DesignGuidField, DesignNameField, DesignVariantField, etc.
  - DesignId, DesignProps, DesignInput, DesignContext, DesignOutput
  - Design(TableEntity) with all fields and relationships

### ✅ Kit Entity (C# Only)

#### C# (`Semio.cs`)
- ✅ **KitId**: Changed from `Name` to `Guid`
  - `public string Guid { get; set; } = "";`
  - Updated `ToIdString()` to use Guid
  - Updated implicit operators
- ✅ **Kit** class: Added 4 new fields
  - `Guid` (string) - PRIMARY KEY
  - `Folders` (List<Folder>) - folder organization
  - `CreatedAt` (string) - audit
  - `UpdatedAt` (string) - audit
- ✅ **KitDiff**: Updated all fields
  - `Guid` (string?) - nullable
  - `Folders` (FoldersDiff?) - folder changes
  - `CreatedAt` (string?) - nullable
  - `UpdatedAt` (string?) - nullable
  - All other fields now nullable
- ✅ **KitDiff.MergeDiff**: Updated merge logic
- ✅ **Implicit operators**: Updated conversions

#### Python (`engine.py`)
- ❌ **NOT IMPLEMENTED** - Needs complete Kit model with:
  - KitGuidField, KitNameField, KitVersionField, etc.
  - KitId, KitProps, KitInput, KitContext, KitOutput
  - Kit(TableEntity) with all fields and relationships
  - Folders relationship

### ✅ Folder Entity (C# Only) - **NEWLY ADDED**

#### C# (`Semio.cs`)
- ✅ **FolderId**: Complete implementation
  - `public string Guid { get; set; } = "";`
  - Implicit operators for Folder and FolderDiff
- ✅ **Folder** class: Complete implementation (9 fields)
  - `Guid` (string) - PRIMARY KEY
  - `Name` (string) - folder name
  - `Parent` (string?) - parent folder for hierarchy
  - `Description` (string) - optional description
  - `Attributes` (List<Attribute>) - metadata
  - `CreatedAt` (string) - creation timestamp
  - `CreatedBy` (string?) - creator user
  - `UpdatedAt` (string) - last update timestamp
  - `UpdatedBy` (string?) - last updater user
- ✅ **FolderDiff**: Complete implementation with nullable fields
- ✅ **FoldersDiff**: Complete implementation (Removed, Updated, Added)
- ✅ **Folder.ApplyDiff**: Complete implementation
- ✅ **FolderDiff.MergeDiff**: Complete implementation
- ✅ **Implicit operators**: Complete conversions

#### Python (`engine.py`)
- ❌ **NOT IMPLEMENTED** - Needs complete Folder model with:
  - FolderGuidField, FolderNameField, FolderParentField, etc.
  - FolderId, FolderProps, FolderInput, FolderContext, FolderOutput
  - Folder(TableEntity) with all fields and relationships

### ✅ Port Entity (C# Only) - **FIELD NAME STANDARDIZATION**

#### C# (`Semio.cs`)
- ✅ **Port**: Changed field name from `Id` to `Guid`
  - Removed `[JsonProperty("id_")]` attribute
  - `public string Guid { get; set; } = "";`
  - Updated all references (ToIdString, implicit operators)
- ✅ **PortId**: Changed from `Id` to `Guid`
  - `public string Guid { get; set; } = "";`
  - Updated implicit operators
- ✅ **PortDiff**: Changed from `Id` to `Guid`
  - `public string? Guid { get; set; }`
  - All fields now nullable with `?`
  - Updated MergeDiff logic
- ✅ **Port.ApplyDiff**: Updated to use Guid
- ✅ **Port.CreateDiff**: Updated to use Guid

#### Python (`engine.py`)
- ⚠️ **NEEDS UPDATE** - Currently uses `id_` field, should be standardized to `guid`

## Pending Work

### ❌ Python Implementation (Major Effort Required)

The Python implementation uses a field-based mixin pattern that requires extensive boilerplate for each entity. Each entity needs:

#### Type Entity Pattern (Example)
```python
# 1. Field Mixins (one per field, ~15 total)
class TypeGuidField(RealField, abc.ABC):
    guid: str = Field(default="")

class TypeNameField(RealField, abc.ABC):
    name: str = Field(default="")

class TypeParentField(RealField, abc.ABC):
    parent: str | None = Field(default=None)

# ... repeat for all 15+ fields

# 2. Composite Classes
class TypeId(TypeGuidField, Id):
    pass

class TypeProps(TypeUpdatedAtField, TypeCreatedAtField, TypeFolderField, TypeIsAbstractField, TypeParentField, TypeVariantField, TypeNameField, TypeGuidField, Props):
    pass

class TypeInput(TypeUpdatedAtField, TypeCreatedAtField, TypeFolderField, TypeIsAbstractField, TypeParentField, TypeVariantField, TypeNameField, TypeGuidField, Input):
    pass

class TypeContext(TypeNameField, TypeVariantField, TypeGuidField, Context):
    pass

class TypeOutput(TypeUpdatedAtField, TypeCreatedAtField, TypeFolderField, TypeIsAbstractField, TypeParentField, TypeVariantField, TypeNameField, TypeGuidField, Output):
    pass

# 3. Table Entity
class Type(TypeUpdatedAtField, TypeCreatedAtField, TypeFolderField, TypeIsAbstractField, TypeParentField, TypeVariantField, TypeNameField, TypeGuidField, TableEntity, table=True):
    __tablename__ = "types"
    
    # Relationships
    kit_guid: str = Field(foreign_key="kits.guid")
    kit: "Kit" = Relationship(back_populates="types")
    ports: list["Port"] = Relationship(back_populates="type")
    representations: list["Representation"] = Relationship(back_populates="type")
```

#### Required Python Entities
1. **Type** (~15 fields)
   - guid, name, variant, parent, isAbstract, folder, description, icon, image, canScale, canMirror, unit, stock, location, createdAt, updatedAt
   - Relationships: Kit (parent), Port[], Representation[], Prop[], Attribute[], Author[]

2. **Design** (~20 fields)
   - guid, name, variant, view, parent, isAbstract, folder, description, icon, image, location, unit, activeLayer, canScale, canMirror, createdAt, updatedAt
   - Relationships: Kit (parent), Piece[], Connection[], Layer[], Group[], Prop[], Stat[], Attribute[], Author[]

3. **Kit** (~15 fields)
   - guid, name, version, description, icon, image, preview, concepts, remote, homepage, license, createdAt, updatedAt
   - Relationships: Type[], Design[], File[], Folder[], Quality[], Author[], Attribute[]

4. **Folder** (~9 fields)
   - guid, name, parent, description, attributes, createdAt, createdBy, updatedAt, updatedBy
   - Relationships: Kit (parent), Parent Folder, Child Folders, Files, Types, Designs

5. **Port** (Field Rename)
   - Change `id_` to `guid` consistently

### ❌ Database Migrations (Python)

SQLAlchemy/Alembic migrations needed for:
1. Add `types` table with all fields and relationships
2. Add `designs` table with all fields and relationships
3. Update `kits` table:
   - Add `guid` column (UUID, primary key)
   - Add `created_at` column (timestamp)
   - Add `updated_at` column (timestamp)
   - Add `folders` relationship
   - Migrate existing `name` primary key to `guid` system
4. Add `folders` table with all fields and relationships
5. Update `ports` table:
   - Rename `id_` column to `guid`
   - Update foreign key references

### ❌ ApplyTypesDiff / ApplyDesignsDiff Methods (C#)

Need to update matching logic from composite keys to GUID:

**Old Pattern**:
```csharp
var existingType = types.FirstOrDefault(t => t.Name == diff.Name && t.Variant == diff.Variant);
```

**New Pattern**:
```csharp
var existingType = types.FirstOrDefault(t => t.Guid == diff.Guid);
```

Files to update:
- `Kit.ApplyDiff` method for Types matching
- `Kit.ApplyDiff` method for Designs matching
- `Kit.ApplyDiff` method for Files matching
- `Kit.ApplyDiff` method for Folders matching (new)

### ❌ Grasshopper Components (C#)

The following Grasshopper Goo classes need updates:

#### Completed
- ✅ FileGoo - Updated to use Guid
- ✅ FileIdGoo - Updated to use Guid
- ✅ FileDiffGoo - Updated to use Guid

#### Pending
- ❌ TypeGoo - Needs Guid field, new fields (parent, isAbstract, folder, timestamps)
- ❌ TypeIdGoo - Needs Guid instead of Name+Variant
- ❌ TypeDiffGoo - Needs all new fields
- ❌ DesignGoo - Needs Guid field, new fields (parent, isAbstract, folder, activeLayer, etc.)
- ❌ DesignIdGoo - Needs Guid instead of Name+Variant+View
- ❌ DesignDiffGoo - Needs all new fields
- ❌ KitGoo - Needs Guid field, Folders, timestamps
- ❌ KitIdGoo - Needs Guid instead of Name
- ❌ KitDiffGoo - Needs all new fields
- ❌ FolderGoo - NEW - Complete implementation needed
- ❌ FolderIdGoo - NEW - Complete implementation needed
- ❌ FolderDiffGoo - NEW - Complete implementation needed
- ❌ PortGoo - Rename Id to Guid
- ❌ PortIdGoo - Rename Id to Guid
- ❌ PortDiffGoo - Rename Id to Guid

### ❌ Serialization/Deserialization

JSON serialization needs review for:
- GUID fields instead of composite keys
- New optional fields (proper null handling)
- Folder entity (completely new)
- Port guid field name change

### ❌ API/GraphQL Schema Updates

If there are API or GraphQL schemas, they need updates to match:
- New GUID-based identification
- New fields on all entities
- New Folder entity
- Changed field names (Port.Id → Port.Guid)

## Testing Requirements

### Critical Tests Needed

1. **ID Migration Tests**
   - Verify GUID generation for all entities
   - Test backward compatibility (if required)
   - Test ID collision handling

2. **Hierarchy Tests**
   - Parent-child relationships for Types
   - Parent-child relationships for Designs
   - Parent-child relationships for Folders
   - Folder organization for Files, Types, Designs

3. **Diff Application Tests**
   - Type.ApplyDiff with all new fields
   - Design.ApplyDiff with all new fields
   - Kit.ApplyDiff with all new fields
   - Folder.ApplyDiff (new entity)
   - Port.ApplyDiff with renamed field

4. **Serialization Tests**
   - JSON round-trip for all entities
   - Null handling for optional fields
   - GUID serialization/deserialization

5. **Database Tests** (Python)
   - Table creation with new schemas
   - Relationship integrity
   - Migration scripts
   - Query performance with GUID indexes

## Migration Strategy

### Phase 1: Core Models (Completed)
- ✅ Update C# entity classes
- ✅ Update C# ID classes
- ✅ Update C# Diff classes
- ✅ Add Folder entity to C#

### Phase 2: Python Implementation (Not Started)
- ⏳ Create field mixins for Type (~15 fields)
- ⏳ Create field mixins for Design (~20 fields)
- ⏳ Create field mixins for Kit (~15 fields)
- ⏳ Create field mixins for Folder (~9 fields)
- ⏳ Update Port field name (id_ → guid)
- ⏳ Create composite classes (Id, Props, Input, Context, Output)
- ⏳ Create table entities with relationships
- ⏳ Write database migration scripts

### Phase 3: Integration Updates (Not Started)
- ⏳ Update Grasshopper Goo classes
- ⏳ Update ApplyTypesDiff/ApplyDesignsDiff methods
- ⏳ Update serialization/deserialization
- ⏳ Update API/GraphQL schemas (if applicable)

### Phase 4: Testing & Validation (Not Started)
- ⏳ Unit tests for all entity operations
- ⏳ Integration tests for diff application
- ⏳ Database migration validation
- ⏳ End-to-end workflow tests

## Backward Compatibility Considerations

### Breaking Changes
1. **FileId**: Changed from URL string to GUID
   - All file references need migration
   - External integrations need updates

2. **TypeId**: Changed from {Name, Variant} to GUID
   - All type references need migration
   - Grasshopper scripts need updates

3. **DesignId**: Changed from {Name, Variant, View} to GUID
   - All design references need migration
   - UI components need updates

4. **KitId**: Changed from Name to GUID
   - All kit references need migration

5. **Port**: Field renamed from Id to Guid
   - All port references need updates
   - JSON deserialization needs compatibility layer

### Migration Path
- Consider adding a migration tool to convert old IDs to new GUIDs
- Maintain lookup tables for Name→GUID mapping during transition
- Version API to support both old and new formats temporarily

## Summary Statistics

### Entities Updated
- **File**: ✅ Complete (TS/C#/Python)
- **Folder**: ✅ C# Complete, ❌ Python Missing
- **Type**: ✅ C# Complete, ❌ Python Missing
- **Design**: ✅ C# Complete, ❌ Python Missing
- **Kit**: ✅ C# Complete, ❌ Python Missing
- **Port**: ✅ C# Complete, ⚠️ Python Needs Field Rename

### Lines of Code Changed
- **C# (Semio.cs)**: ~500+ lines modified/added
- **Python (engine.py)**: ~200 lines (File entity only)
- **TypeScript (semio.ts)**: 0 (source of truth, no changes needed)

### Estimated Remaining Work
- **Python Implementation**: 40-60 hours
  - Type entity: ~8 hours
  - Design entity: ~12 hours
  - Kit entity: ~8 hours
  - Folder entity: ~6 hours
  - Database migrations: ~10 hours
  - Testing: ~10 hours
- **Grasshopper Components**: 8-12 hours
- **Integration Updates**: 6-8 hours
- **Testing & Validation**: 10-15 hours

**Total Estimated**: 64-95 hours of additional work

## Next Steps (Priority Order)

1. **HIGH**: Complete Python Type entity implementation
2. **HIGH**: Complete Python Design entity implementation
3. **HIGH**: Complete Python Kit entity implementation
4. **HIGH**: Complete Python Folder entity implementation
5. **HIGH**: Write and test database migration scripts
6. **MEDIUM**: Update Port field name in Python (id_ → guid)
7. **MEDIUM**: Update ApplyTypesDiff/ApplyDesignsDiff methods in C#
8. **MEDIUM**: Update Grasshopper Goo classes
9. **LOW**: Update API/GraphQL schemas
10. **LOW**: Write comprehensive test suite
11. **LOW**: Create migration tool for existing data

## Notes

- TypeScript schema in `js/js/semio.ts` is the authoritative source
- All timestamps use ISO 8601 format (string representation)
- All GUIDs should be generated using UUID v4
- Nullable fields in diffs indicate "no change" vs explicit null value
- Database schema file `sqlite/schema.sql` needs updating to match new schemas
- Consider performance impact of GUID indexes vs composite key indexes

## References

- TypeScript Schema: `js/js/semio.ts`
- C# Implementation: `net/Semio/Semio.cs`
- Python Implementation: `py/engine/engine.py`
- Database Schema: `sqlite/schema.sql`
- Grasshopper Components: `net/Semio.Grasshopper/Semio.Grasshopper.cs`
- Architecture Document: `AGENTS.md`
