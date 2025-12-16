---
slug: SCHEMA-CHANGES-NAMES-AND-INTERFACES
summary: Migration from 2025-11-20_SCHEMA-CHANGES-NAMES-AND-INTERFACES.md
---
# Schema Changes: Names and Interfaces

Date: 2025-11-20

## Overview

This plan covers two major schema changes:
1. Adding `name` property to `Piece`, `Port`, and `Model`
2. Converting `Interface` from a string to a full kit artifact with its own identity

## 1. Problem Description

### Current State
- `Piece`, `Port`, and `Model` lack a `name` property for user-friendly identification
- `Port.interface` is just a string, limiting extensibility
- `Port.compatibleInterfaces` is an array of strings
- No central management of interface definitions

### Desired State
- `Piece`, `Port`, and `Model` have optional `name` properties
- `Interface` is a first-class kit artifact with:
  - `guid` (InterfaceId)
  - `name`
  - `description`
  - `icon`
  - `compatibleInterfaces` (array of InterfaceId references)
- Kit contains an `interfaces` collection
- Port references interfaces by InterfaceId instead of string

## 2. Affected Files

### Schema Files
- `sqlite/schema.sql` - Database schema
- `jsonschema/kit.json` - JSON schema for kit serialization
- `engineering/dataarchitecture.pu` - PlantUML architecture diagram
- `engineering/interfacearchitecture.txt` - Interface architecture

### TypeScript/JavaScript
- `js/js/semio.ts` - Core domain logic and types
- `js/js/sketchpad/apps/*/App.tsx` - All app stores that work with these models
- `js/js/elements/**/*.tsx` - UI components that display/edit these properties

### .NET
- `net/Semio/Semio.cs` - C# domain models

### GraphQL
- `graphql/schema.graphql` - GraphQL API schema

### JSON Schema
- `jsonschema/design.json` - Design serialization
- `jsonschema/type.json` - Type serialization

## 3. Implementation Plan

### Phase 1: Schema Definition
1. Update `sqlite/schema.sql`
   - Add `name` column to `pieces`, `ports`, `models` tables
   - Create `interfaces` table with columns: guid, name, description, icon, attributes
   - Create `interface_compatibilities` junction table
   - Update `ports` table: change `interface` to `interface_id` (foreign key)
   - Remove `compatible_interfaces` column from `ports` (now in Interface definition)

2. Update `jsonschema/kit.json`
   - Add Interface definition with properties
   - Add `interfaces` array to Kit
   - Update Piece, Port, Model to include `name`
   - Update Port to reference InterfaceId instead of string

3. Update `engineering/dataarchitecture.pu`
   - Add Interface entity
   - Add relationships
   - Update Piece, Port, Model entities

### Phase 2: TypeScript Implementation
1. Update `js/js/semio.ts`
   - Add `InterfaceId` type
   - Add `Interface` model with properties
   - Add `InterfaceInput`, `InterfaceDiff`, etc.
   - Update `Piece`, `Port`, `Model` to include `name?: string`
   - Update `Port` to use `interfaceId?: InterfaceId` and remove `compatibleInterfaces`
   - Update `Kit` to include `interfaces: Interface[]`
   - Add interface-related helper functions
   - Update all getDiff, applyDiff, inverseDiff functions
   - Add getters for interface compatibility resolution

2. Update stores in `js/js/sketchpad/App.tsx` and app files
   - Update KitStore to manage interfaces
   - Add interface-related commands
   - Update DesignAppStore, TypeAppStore to handle new properties
   - Update selection and diff types

3. Update UI components
   - Add name inputs for Piece, Port, Model
   - Add Interface management UI
   - Update Port interface selection to use Interface picker
   - Show interface compatibility visually

### Phase 3: .NET Implementation
1. Update `net/Semio/Semio.cs`
   - Add `InterfaceId` struct
   - Add `Interface` class
   - Update `Piece`, `Port`, `Model` classes
   - Update serialization/deserialization

### Phase 4: API Updates
1. Update `graphql/schema.graphql`
   - Add Interface type
   - Update Piece, Port, Model types
   - Add interface queries and mutations

### Phase 5: Migration
1. Create migration scripts for existing data
   - SQLite migration to add columns and tables
   - Data migration for existing kits (set names to null, create default interfaces)

## 4. Hierarchy Updates

Update the model hierarchy order in AGENTS.md:
1. Attribute
2. Coord
3. Vec
4. Point
5. Vector
6. Plane
7. Camera
8. Location
9. Author
10. File
11. Benchmark
12. QualityKind
13. Quality
14. **Interface** (NEW - before Prop)
15. Prop
16. Model
17. Port
18. Type
19. Layer
20. Piece
21. Group
22. Side
23. Connection
24. Stat
25. Design
26. Kit

## 5. Backward Compatibility

- Old kits without names: `name` defaults to `undefined`
- Old kits with string interfaces: 
  - Create Interface artifacts from unique interface strings
  - Map Port.interface strings to new InterfaceIds
  - Build compatibleInterfaces from old Port.compatibleInterfaces arrays

## 6. Testing Considerations

- Test piece/port/model creation with and without names
- Test interface creation and compatibility resolution
- Test port connections with compatible interfaces
- Test serialization/deserialization with new schema
- Test migration of old kits

## 7. Implementation Order

1. ✅ Schema files (sqlite, jsonschema, engineering)
2. ✅ Core TypeScript types and logic (semio.ts)
3. ✅ Store updates (App.tsx, app stores)
4. ✅ UI components
5. ✅ .NET implementation
6. ✅ GraphQL schema
7. ✅ Migration scripts
8. ✅ Update AGENTS.md hierarchy
