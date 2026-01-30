# Ticket

## Todos
# Plan: Create Standalone semio.py Package

## Objective

Create a standalone `py/semio/semio.py` package that matches the functionality of `semio.ts`, `semio.go`, `semio.rs`, and `Semio.cs`.

## Requirements

1. Pure Pydantic models (no SQLModel table=True, no graphene, no FastAPI)
2. Minimal dependencies: pydantic, numpy, networkx
3. Same model hierarchy as other implementations
4. Diff types and operations for all models
5. Validation constraints
6. Serialization/deserialization

## Models to Implement

1. Attribute
2. Coord, Vec, Point, Vector, Plane, Camera
3. Location, Author
4. File, Folder
5. Benchmark, Quality
6. Port (Port), Prop
7. Tag, Concept
8. Model, Connector
9. Type
10. Layer, Piece, Group, Side
11. Connection, Stat
12. Design
13. Kit

## Tasks

1. Create pyproject.toml with minimal dependencies
2. Create semio.py with all models
3. Update package.json with scripts
4. Create test file
5. Update AGENTS.md documentation

## Changes

## Log
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

## Summary
