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
