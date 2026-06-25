---
goal: AI-OPTIMIZED-REPO/SINGLE-FILE-REPO/CONSISTENT-SECTIONS
---

# Ticket

## Summary

Added missing summary and spec comments for all definitions across 3 files: build.py (4 functions + 4 module vars), kit_sqlite.go (9 functions), index.ts (25+ exports/consts + 1 section wrapper). All definition-level summary comments added with RFC2119 spec comments on behavioral definitions.
## Changes

### `assets/grasshopper/build.py`
- Added summary + spec comments before `extract_param_props`, `is_numeric`, `get_pivot_y`, `parse_components_and_groups_xml`
- Added summary comments before module-level variables `definition`, `xml_file`, `extracted_data`, `json_output`

### `compose/go/kit_sqlite.go`
- Added summary + spec comments before 9 functions: `KitFromSqlite`, `loadTypes`, `loadDesigns`, `loadPieces`, `loadConnections`, `loadConnectors`, `KitToSqlite`, `KitFromZip`, `KitToZip`

### `assets/index.ts`
- Wrapped all orphan code in `//#region 🔖Exports` section with section summary
- Added summary comments before all 25+ export/const definitions
- Added spec comment for `buildLookup` function definition

## Log

## Todos

## Plan
### File 1: `assets/grasshopper/build.py`
- Add summary + spec comments before 5 module-level `def` definitions:
  - `extract_param_props`, `is_numeric`, `get_pivot_y`, `parse_components_and_groups_xml`
- The remaining module-level code (variable assignments and calls) needs summary comments
- Wrap orphan definitions in a section

### File 2: `compose/go/kit_sqlite.go`
- Add summary + spec comments before 8 exported `func` definitions:
  - `KitFromSqlite`, `KitToSqlite`, `KitFromZip`, `KitToZip`
- Add summary + spec comments before unexported helpers:
  - `loadTypes`, `loadDesigns`, `loadPieces`, `loadConnections`, `loadConnectors`

### File 3: `assets/index.ts`
- Wrap orphan `export` definitions in section regions
- Add summary comments before each `export` and `const` definition
- Add summary lines after each `#region` marker