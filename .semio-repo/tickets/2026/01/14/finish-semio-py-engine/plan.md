# Plan: Finish semio.py and engine.py

## Goal

Create a standalone `py/semio/semio.py` module with pure domain types, diffs, and validation - matching the pattern of `semio.ts`, `semio.go`, `semio.rs`, and `Semio.cs`.

## Current State Analysis

- `py/semio/semio.py` (7778 lines) contains domain models BUT mixed with:
  - SQLModel table definitions (database-specific)
  - SQLAlchemy relationships
  - GraphQL types (graphene)
  - FastAPI path annotations
- `py/engine/engine.py` (1594 lines) contains:
  - Store abstractions (Database, REST, GraphQL)
  - API endpoints
  - MCP tools
  - Engine startup logic

## Target State

- `py/semio/semio.py` should contain ONLY:
  - Pure Pydantic models (no SQLModel tables)
  - Type definitions (Attribute, Point, Vector, Plane, etc.)
  - Diff types and functions (get_diff, apply_diff, inverse_diff, merge_diff)
  - Validation constraints and validation engine
  - Serialization/deserialization helpers
  - ID types and helpers
  - Utility functions

- `py/engine/engine.py` should:
  - Import from `semio.py` for domain types
  - Define SQLModel tables that map to domain types
  - Handle API, GraphQL, MCP server

## Tasks

1. Extract pure domain models from semio.py - remove SQLModel table definitions
2. Add Diff types for all entities (following TypeScript pattern)
3. Add get_diff, apply_diff, inverse_diff, merge_diff functions for each type
4. Add validation constraints (GUID uniqueness, name uniqueness, etc.)
5. Add validation engine (ValidationContext, validateKit)
6. Update engine.py to import from semio and define SQLModel tables separately

## Entities to Define (following hierarchy from AGENTS.md)

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
14. Port
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

## Implementation Approach

Since the current semio.py is 7778 lines with mixed concerns, we will:

1. Create a new clean semio.py with pure domain types
2. Move SQLModel table definitions to engine.py
3. Keep the same class names but remove table=True and SQLAlchemy specifics
