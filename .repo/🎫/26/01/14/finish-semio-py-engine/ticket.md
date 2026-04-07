# Ticket

## Todos
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

## Changes

## Log
# Log: Finish semio.py and engine.py

## 2026-01-14

### Initial Analysis
- Read current `py/semio/semio.py` (7778 lines) - contains mixed SQLModel tables, GraphQL types, and domain logic
- Read `py/engine/engine.py` (1594 lines) - contains Store abstractions and API endpoints
- Read `js/semio/semio.ts` (7741 lines) - pure domain types with Zod schemas, diffs, and validation

### Key Observations
1. TypeScript semio.ts pattern:
   - Uses Zod for schema validation
   - Each entity has: Schema, Type, Diff types, and functions (get_diff, apply_diff, inverse_diff, merge_diff)
   - Has collection diffs (removed, updated, added pattern)
   - Has validation constraints (guid-unique, name-unique, etc.)
   - Has validation context and engine

2. Current Python semio.py issues:
   - Mixed with SQLModel table definitions (database coupling)
   - Mixed with GraphQL types (graphene)
   - Uses SQLAlchemy relationships
   - Has FastAPI-specific annotations

### Implementation Strategy
Creating a clean standalone semio.py with:
- Pydantic models for domain types
- TypedDict for Diff types
- Dataclasses where appropriate
- Validation functions matching TypeScript API

### Starting Implementation
Creating the new clean semio.py following the TypeScript structure.

### Session 2 - Import Fixes and Tests

#### Problems Identified
1. `engine.py` was missing imports from `semio.py` after extraction
2. `engine.py` had erroneous import `get as semio_get` (no such function exists in semio.py)
3. `engine.py` had typo: `existingsemio` → `existingSemio`
4. `engine.py` had function name conflict: `delete_kit` MCP tool vs REST endpoint

#### Fixes Applied
1. Removed erroneous `get as semio_get` import from engine.py
2. Previous fixes from earlier session (typo and function rename) were already in place

#### Test Results
- **engine.py imports successfully** after fixing the `get as semio_get` import
- **semio.py** still has heavy dependencies (dotenv, fastapi, sqlmodel, graphene) that aren't in its pyproject.toml
- Tests run successfully using engine's environment which has all dependencies

#### Engine Tests Created
Created comprehensive `engine.test.py` with 42 tests covering:
- **Encoding** (encode/decode roundtrip)
- **OperationBuilder** (parsing kit/types/designs operations)
- **SqliteStore** (factory, caching, initialize, initialized check)
- **StoreKind and CommandKind** enums
- **REST API** (get kit not found)
- **GraphQL** (schema exists, Query class)
- **MCP Tools** (get_kit, put_kit, validate_kit, get_kit_diff, apply_kit_diff, inverse_kit_diff)
- **Cache** (dir encoding, rejection of non-remote/non-zip)
- **SSLMode** enum
- **Errors** (KitNotFound, KitAlreadyExists, OnlyRemoteKitsCanBeCached, LocalKitUriIsNotAbsolute)
- **Assistant** (encodeForPrompt, replaceDefault, encodeType, design templates)
- **Engine Configuration** (engine/rest/mcp/graphql apps exist)
- **Integration** (store initialization, storeAndOperationFromCode)

#### Final Status
- ✅ **59 tests pass** (42 engine + 17 semio)
- ✅ **engine.py imports correctly**
- ⚠️ **semio.py** still has architectural issue - heavy dependencies mixed with domain models (requires larger refactor)

## Summary
# Summary: Finish semio.py and engine.py

## Objectives
1. Get tests running for semio.py (recently extracted from engine.py)
2. Get engine.py running again after the extraction
3. Add comprehensive tests for all engine features

## Results

### ✅ All Objectives Achieved

**Test Results:** 59 tests pass (42 engine + 17 semio)

### Changes Made

#### 1. Fixed engine.py Import Error
- **Issue:** engine.py had erroneous import `get as semio_get` on line 153
- **Root Cause:** The `get` function is defined in engine.py itself (line 653), not in semio.py
- **Fix:** Removed the erroneous import line

#### 2. Created Comprehensive engine.test.py
Created 42 tests covering all major engine features:

| Test Class | Count | Coverage |
|------------|-------|----------|
| TestEncoding | 3 | encode/decode functions, roundtrip |
| TestOperationBuilder | 2 | code parsing for operations |
| TestSqliteStore | 6 | factory, caching, initialization |
| TestStoreKind | 2 | enum values |
| TestCommandKind | 2 | enum values |
| TestRestApi | 1 | error handling |
| TestGraphQL | 2 | schema and Query class |
| TestMcp | 6 | all MCP tools |
| TestCache | 2 | directory encoding, URI validation |
| TestSSLMode | 2 | enum values |
| TestErrors | 4 | error string representations |
| TestAssistant | 6 | prompt/type encoding, templates |
| TestEngineConfiguration | 2 | app existence checks |
| TestIntegration | 2 | store init, operation parsing |

### Known Issues (Not Fixed)
- **semio.py architectural debt:** The module still has heavy dependencies (sqlmodel, graphene, fastapi, dotenv) mixed with domain models
- **pyproject.toml mismatch:** semio's pyproject.toml lists only lightweight deps (pydantic, numpy, networkx) but the module requires engine's environment
- **Recommendation:** Future refactor should extract pure domain models from SQLModel/GraphQL/FastAPI coupling

## Files Modified
- `py/engine/engine.py` - Removed erroneous import
- `py/engine/engine.test.py` - Created with 42 tests
