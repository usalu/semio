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
