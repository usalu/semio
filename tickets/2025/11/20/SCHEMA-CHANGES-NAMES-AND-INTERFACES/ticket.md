---
slug: SCHEMA-CHANGES-NAMES-AND-INTERFACES
summary: Migration from 2025-11-20_SCHEMA-CHANGES-NAMES-AND-INTERFACES.md
prompt: Migration from 2025-11-20_SCHEMA-CHANGES-NAMES-AND-INTERFACES.md
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.679Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Schema Changes: Names and Interfaces

Date: 2025-11-20

## Overview

This plan covers two major schema changes:

1. Adding `name` property to `Piece`, `Connector`, and `Model`
2. Converting `Interface` from a string to a full kit artifact with its own identity

## 1. Problem Description

### Current State

- `Piece`, `Connector`, and `Model` lack a `name` property for user-friendly identification
- `Connector.port` is just a string, limiting extensibility
- `Connector.compatibleInterfaces` is an array of strings
- No central management of port definitions

### Desired State

- `Piece`, `Connector`, and `Model` have optional `name` properties
- `Interface` is a first-class kit artifact with:
  - `guid` (InterfaceId)
  - `name`
  - `description`
  - `icon`
  - `compatibleInterfaces` (array of InterfaceId references)
- Kit contains an `ports` collection
- Connector references ports by InterfaceId instead of string

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
   - Add `name` column to `pieces`, `connectors`, `models` tables
   - Create `ports` table with columns: guid, name, description, icon, attributes
   - Create `port_compatibilities` junction table
   - Update `connectors` table: change `port` to `port_id` (foreign key)
   - Remove `compatible_ports` column from `connectors` (now in Interface definition)

2. Update `jsonschema/kit.json`
   - Add Interface definition with properties
   - Add `ports` array to Kit
   - Update Piece, Connector, Model to include `name`
   - Update Connector to reference InterfaceId instead of string

3. Update `engineering/dataarchitecture.pu`
   - Add Interface entity
   - Add relationships
   - Update Piece, Connector, Model entities

### Phase 2: TypeScript Implementation

1. Update `js/js/semio.ts`
   - Add `InterfaceId` type
   - Add `Interface` model with properties
   - Add `InterfaceInput`, `InterfaceDiff`, etc.
   - Update `Piece`, `Connector`, `Model` to include `name?: string`
   - Update `Connector` to use `portId?: InterfaceId` and remove `compatibleInterfaces`
   - Update `Kit` to include `ports: Interface[]`
   - Add port-related helper functions
   - Update all getDiff, applyDiff, inverseDiff functions
   - Add getters for port compatibility resolution

2. Update stores in `js/js/sketchpad/App.tsx` and app files
   - Update KitStore to manage ports
   - Add port-related commands
   - Update DesignAppStore, TypeAppStore to handle new properties
   - Update selection and diff types

3. Update UI components
   - Add name inputs for Piece, Connector, Model
   - Add Interface management UI
   - Update Connector port selection to use Interface picker
   - Show port compatibility visually

### Phase 3: .NET Implementation

1. Update `net/Semio/Semio.cs`
   - Add `InterfaceId` struct
   - Add `Interface` class
   - Update `Piece`, `Connector`, `Model` classes
   - Update serialization/deserialization

### Phase 4: API Updates

1. Update `graphql/schema.graphql`
   - Add Interface type
   - Update Piece, Connector, Model types
   - Add port queries and mutations

### Phase 5: Migration

1. Create migration scripts for existing data
   - SQLite migration to add columns and tables
   - Data migration for existing kits (set names to null, create default ports)

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
17. Connector
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
- Old kits with string ports:
  - Create Interface artifacts from unique port strings
  - Map Connector.port strings to new InterfaceIds
  - Build compatibleInterfaces from old Connector.compatibleInterfaces arrays

## 6. Testing Considerations

- Test piece/connector/model creation with and without names
- Test port creation and compatibility resolution
- Test connector connections with compatible ports
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
