# Ticket

## Todos

# Schema Changes: Names and Ports

Date: 2025-11-20

## Overview

This plan covers two major schema changes:

1. Adding `name` property to `Piece`, `Connector`, and `Model`
2. Converting `Port` from a string to a full kit artifact with its own identity

## 1. Problem Description

### Current State

- `Piece`, `Connector`, and `Model` lack a `name` property for user-friendly identification
- `Connector.port` is just a string, limiting extensibility
- `Connector.compatiblePorts` is an array of strings
- No central management of port definitions

### Desired State

- `Piece`, `Connector`, and `Model` have optional `name` properties
- `Port` is a first-class kit artifact with:
  - `guid` (PortId)
  - `name`
  - `description`
  - `icon`
  - `compatiblePorts` (array of PortId references)
- Kit contains an `ports` collection
- Connector references ports by PortId instead of string

## 2. Affected Files

### Schema Files

- `sqlite/schema.sql` - Database schema
- `jsonschema/kit.json` - JSON schema for kit serialization
- `engineering/dataarchitecture.pu` - PlantUML architecture diagram
- `engineering/interfacearchitecture.txt` - Port architecture

### TypeScript/JavaScript

- `js/compose/compose.ts` - Core domain logic and types
- `js/compose/sketchpad/apps/*/App.tsx` - All app stores that work with these models
- `js/elements/**/*.tsx` - UI components that display/edit these properties

### .NET

- `net/Compose/Compose.cs` - C# domain models

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
   - Remove `compatible_ports` column from `connectors` (now in Port definition)

2. Update `jsonschema/kit.json`
   - Add Port definition with properties
   - Add `ports` array to Kit
   - Update Piece, Connector, Model to include `name`
   - Update Connector to reference PortId instead of string

3. Update `engineering/dataarchitecture.pu`
   - Add Port entity
   - Add relationships
   - Update Piece, Connector, Model entities

### Phase 2: TypeScript Implementation

1. Update `js/compose/compose.ts`
   - Add `PortId` type
   - Add `Port` model with properties
   - Add `PortInput`, `PortDiff`, etc.
   - Update `Piece`, `Connector`, `Model` to include `name?: string`
   - Update `Connector` to use `portId?: PortId` and remove `compatiblePorts`
   - Update `Kit` to include `ports: Port[]`
   - Add port-related helper functions
   - Update all getDiff, applyDiff, inverseDiff functions
   - Add getters for port compatibility resolution

2. Update stores in `js/compose/sketchpad/App.tsx` and app files
   - Update KitStore to manage ports
   - Add port-related commands
   - Update DesignAppStore, TypeAppStore to handle new properties
   - Update selection and diff types

3. Update UI components
   - Add name inputs for Piece, Connector, Model
   - Add Port management UI
   - Update Connector port selection to use Port picker
   - Show port compatibility visually

### Phase 3: .NET Implementation

1. Update `net/Compose/Compose.cs`
   - Add `PortId` struct
   - Add `Port` class
   - Update `Piece`, `Connector`, `Model` classes
   - Update serialization/deserialization

### Phase 4: API Updates

1. Update `graphql/schema.graphql`
   - Add Port type
   - Update Piece, Connector, Model types
   - Add port queries and mutations

### Phase 5: Migration

1. Create migration scripts for existing data
   - SQLite migration to add columns and tables
   - Data migration for existing kits (set names to null, create default ports)

## 4. Document Updates

Update the model document order in AGENTS.md:

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
14. **Port** (NEW - before Prop)
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
  - Create Port artifacts from unique port strings
  - Map Connector.port strings to new PortIds
  - Build compatiblePorts from old Connector.compatiblePorts arrays

## 6. Testing Considerations

- Test piece/connector/model creation with and without names
- Test port creation and compatibility resolution
- Test connector connections with compatible ports
- Test serialization/deserialization with new schema
- Test migration of old kits

## 7. Implementation Order

1. ✅️ Schema files (sqlite, jsonschema, engineering)
2. ✅️ Core TypeScript types and logic (compose.ts)
3. ✅️ Store updates (App.tsx, app stores)
4. ✅️ UI components
5. ✅️ .NET implementation
6. ✅️ GraphQL schema
7. ✅️ Migration scripts
8. ✅️ Update AGENTS.md document

## Changes

## Log

## Summary
