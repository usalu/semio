---
slug: DOC-RESTRUCTURE-FOUR-LAYER
summary: Restructure documentation to four-layer system
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.955Z"
commit: "0000000000000000000000000000000000000000"
iterations:
  - prompt: >-
      Migrate all existing docs and code to the new structure. Update outdated
      docs.
    date:
      started: "2025-12-15T17:11:13.450Z"
    model: claude-opus-4-5
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 2fb81ef29354981c1b9625769dba4a06360a4aef
    files:
      updated:
        - path: AGENTS.md
          lines:
            added: 372
            removed: 539
      created: []
      removed: []
    lines:
      added: 372
      removed: 539
---

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
