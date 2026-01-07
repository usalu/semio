# Previously

The documentation was structured with:

- AGENTS.md Codebase section using a folder tree format with nested bullet points
- SRS section containing some framework-specific implementation details
- Several folders lacking documentation in AGENTS.md

# Plan

1. Restructure AGENTS.md Codebase from folder tree to flat `## PATH/` format
2. Clean SRS section of framework-specific implementation details
3. Move implementation details to appropriate Codebase sections
4. Add missing `## PATH/` sections for undocumented folders

# Changes

## AGENTS.md Codebase restructuring

Replaced folder tree structure (lines 1109-1399) with flat `## PATH/` headers:

- Added: `## hooks/`, `## reports/`, `## assets/`, `## engineering/`, `## examples/`, `## graphql/`, `## jsonschema/`, `## log/`, `## scripts/`, `## sql/sqlite/`
- Changed: `## js` → `## js/`, `## js/js` → `## js/js/`, `## net` → `## net/`
- Fixed file paths: `## net/Semio.cs` → `## net/Semio/Semio.cs`, `## Semio.Grasshopper.cs` → `## net/Semio.Grasshopper/Semio.Grasshopper.cs`

## Added missing folder sections

Added documentation for previously undocumented folders:

- `## py/` - Python code overview
- `## py/engine/` - Engine module details
- `## antlr/` - ANTLR grammar
- `## peg/` - PEG grammar
- `## liveblocks/` - Liveblocks schema
- `## meta/` - Metadata files
- `## openapi/` - OpenAPI schema
- `## rdf/` - RDF/SHACL schema
- `## rb/` - Ruby gem placeholder
- `## yak/` - Yak package publishing

## SRS cleanup

Moved framework-specific implementation details from SRS to Codebase:

- Model tag selection: Removed `TypeAppFooter`, `DesignAppFooter`, `selectBestModel` references from Model section
- Added implementation details to `## js/js/` section
