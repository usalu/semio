# Log: Create Standalone semio.py Package

## 2026-01-14

### Analysis
- Examined existing `py/semio/semio.py` - contains engine code with SQLModel/graphene dependencies
- Analyzed `semio.ts` structure - pure Zod schemas with diff operations
- Analyzed `semio.go` - pure Go structs with JSON tags
- Analyzed `semio.rs` - pure Rust structs with serde
- Analyzed `Semio.cs` - pure C# classes

### Implementation
- Creating `pyproject.toml` for the standalone package
- Rewriting `semio.py` with pure Pydantic models
- Adding diff operations for all models
- Adding validation constraints

### Progress
- Reviewed full codebase structure
- Identified 8655 lines in current semio.py that mix domain models with ORM/GraphQL
- semio.py pyproject.toml already has minimal dependencies (pydantic, numpy, networkx)
- Need to rewrite semio.py to remove: sqlmodel, graphene, graphene_pydantic, graphene_sqlalchemy, fastapi, loguru, sqlalchemy, dotenv
- Keep: pydantic, numpy, networkx, pytransform3d

### Starting Rewrite
Creating pure Pydantic-based semio.py following the pattern from semio.ts and semio.go
