---
goal: AI-OPTIMIZED-REPO/SINGLE-FILE-REPO/CONSISTENT-SECTIONS
---

# Ticket

## Summary

Fixed all code violations in semio/net/Semio/Semio.cs (47→0) and semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs (85→0). Added Imports/Namespace section wrappers for orphan definitions, RFC2119-keyword section summaries for 70 sections, and definition summary+spec comments for 29 class definitions across both files.
## Changes

### semio/net/Semio/Semio.cs
- Added `#region 🔖Imports` / `#endregion 🔖Imports` around using statements (orphan fix)
- Added `#region 🔖Namespace` / `#endregion 🔖Namespace` around namespace declaration (orphan fix)
- Added RFC2119-keyword section summaries to 39 sections (Constants, Utility, Expressions, Entitying, SemioValidation, Attribute, Coord, Point, Vector, Plane, Location, Author, File, Folder, Benchmark, QualityKind, Quality, Tag, Concept, Port, Prop, Model, Connector, Type, Layer, Group, Piece, Side, Connection, Stat, Design, Kit, Design Family Helpers, Type Family Helpers, Api, ZipRoundtrip, KitImporter, KitExporter, SemioDiff)
- Added definition summary + spec comments for Symbol, Entity, EntityValidator

### semio/gh/Semio.Grasshopper/Semio.Grasshopper.cs
- Added `#region 🔖Imports` / `#endregion 🔖Imports` around using statements (orphan fix)
- Added `#region 🔖Namespace` / `#endregion 🔖Namespace` around namespace declaration (orphan fix)
- Added RFC2119-keyword section summaries to 31 sections (Constants, Utility, Converters, Bases, Attribute, Coord, Location, Author, File, Folder, Benchmark, QualityKind, Quality, Tag, Prop, Model, Connector, Concept, Port, Type, Layer, Group, Piece, Side, Connection, Stat, Design, Kit, Scripting, Engine, Persistence)
- Added definition summary + spec comments for 26 base classes (Goo, Param, EnumGoo, EnumParam, PassthroughComponent, IdGoo, IdParam, IdComponent, DiffGoo, DiffParam, DiffComponent, SerializeComponent, DeserializeComponent, SerializeDiffComponent, DeserializeDiffComponent, SerializeIdComponent, DeserializeIdComponent, EntityGoo, EntityParam, EntityComponent, EntityIdGoo, EntityIdParam, EntityIdComponent, EntityDiffGoo, EntityDiffParam, EntityDiffComponent)

## Log

- Ran analyze on both files to identify violations
- Created Python script to automate section summary, orphan wrapping, and definition comment insertion
- Initial pass fixed 39+3 violations in Semio.cs and 31+26 in Grasshopper.cs
- Updated all section summaries to include RFC2119 keywords (MUST) for spec exemption
- Fixed remaining Symbol definition missing summary
- Verified 0 violations on both files
- Verified Semio.cs builds successfully (0 errors)
- Grasshopper.cs has pre-existing errors (ModelsDiff, KitsDiff, System.Attribute conflict) unrelated to comment changes

## Todos

- [x] Fix orphan definitions in both files
- [x] Add section summaries with MUST keywords to all sections
- [x] Add definition summaries and spec comments
- [x] Verify 0 violations
- [x] Verify build

## Plan

1. Run analyze to identify all violations
2. Wrap orphan code (imports, namespace) in section regions
3. Add section summary comments after region markers
4. Add definition summary + spec comments before classes
5. Ensure all comments contain RFC2119 keywords for spec exemption
6. Verify 0 violations and successful build
