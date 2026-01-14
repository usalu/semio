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
