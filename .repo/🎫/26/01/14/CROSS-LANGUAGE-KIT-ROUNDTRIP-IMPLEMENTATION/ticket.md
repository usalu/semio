# Ticket

## Todos

# Plan: Cross-Language Kit Roundtrip Implementation

## Steps

1. [x] Python: Implement import_kit/export_kit functions
2. [ ] Python: Fix test import issues and verify roundtrip
3. [ ] Go: Complete SQLite loading/saving implementation
4. [ ] Rust: Implement Zip/SQLite roundtrip
5. [ ] .NET: Implement Zip/SQLite roundtrip
6. [ ] JS/TS: Verify existing implementation
7. [ ] Run all tests and verify functional equivalence
8. [ ] Update documentation

## Changes

## Log

# Log: Cross-Language Kit Roundtrip Implementation

## 2026-01-14

### Session Start

- Continuing from previous session
- Python import_kit/export_kit implemented
- Python test has import issues (ModuleNotFoundError)
- Go SQLite implementation started (kit_sqlite.go created with stubs)

### Python Issues Found

1. **Missing dependencies**: Added python-dotenv, fastapi, graphene, etc. to pyproject.toml
2. **Schema mismatch**: Python SQLModel uses plural table names (`kits`) but reference SQLite uses singular (`kit`)
3. **JSON schema mismatch**: kit_metabolism.json doesn't match current Python model schema (missing fields, different structures)
4. **SQLModel relationship issues**: Direct list assignment to relationships fails outside of database session

### Decision

- Python roundtrip implementation blocked by schema evolution issues
- Move forward with Go implementation which can use reference SQLite schema directly
- Python issues need separate ticket for schema reconciliation

### Current Focus

- Continue Go SQLite implementation

### Go Implementation

- Added imports for archive/zip, io, os, path/filepath, strings
- Implemented `loadTypes`, `loadDesigns`, `loadPieces`, `loadConnections`, `loadConnectors`, `loadModels`
- Implemented `KitFromSqlite` - loads Kit from SQLite database
- Implemented `KitToSqlite` - saves Kit to SQLite database with schema
- Implemented `KitFromZip` - extracts zip and loads Kit from .compose/kit.db
- Implemented `KitToZip` - creates zip with .compose/kit.db and files
- Fixed boolean handling (sql.NullBool instead of int)
- Test `TestKitZipRoundtrip` passes: loaded Metabolism kit with 49 types, 10 designs, 321 files

### JS/TS Verification

- `importKit` and `exportKit` functions already exist in compose.ts
- Added Zip roundtrip test to compose.test.ts
- Test "Zip -> Kit -> Zip -> Kit (roundtrip)" passes (11427ms)

### Deferred

- Rust implementation - significant effort, deferred
- .NET implementation - significant effort, deferred
- Python roundtrip - blocked by schema mismatch issues (separate ticket needed)

## Summary

# Summary: Cross-Language Kit Roundtrip Implementation

## Completed

### Go Implementation

- Created `go/compose/kit_sqlite.go` with full SQLite/Zip roundtrip support
- `KitFromSqlite` - loads Kit from SQLite database
- `KitToSqlite` - saves Kit to SQLite database
- `KitFromZip` - extracts zip and loads Kit from .compose/kit.db
- `KitToZip` - creates zip with .compose/kit.db and files
- Helper functions: `loadTypes`, `loadDesigns`, `loadPieces`, `loadConnections`, `loadConnectors`, `loadModels`
- Test `TestKitZipRoundtrip` passes: loaded Metabolism kit with 49 types, 10 designs, 321 files

### JS/TS Verification

- Verified existing `importKit` and `exportKit` functions in compose.ts
- Added Zip roundtrip test to compose.test.ts
- Test "Zip -> Kit -> Zip -> Kit (roundtrip)" passes (11427ms)

### Python Dependencies

- Added missing dependencies to py/compose/pyproject.toml (python-dotenv, fastapi, graphene, etc.)

## Blocked

### Python Roundtrip

- Schema mismatch: Python SQLModel uses plural table names (`kits`) but reference SQLite uses singular (`kit`)
- JSON schema mismatch: kit_metabolism.json doesn't match current Python model schema
- SQLModel relationship issues: Direct list assignment fails outside of database session
- **Needs separate ticket for schema reconciliation**

## Deferred

- Rust implementation - significant effort required
- .NET implementation - significant effort required

## Files Modified

- `go/compose/kit_sqlite.go` - created
- `go/compose/compose_test.go` - updated
- `js/compose/compose.test.ts` - updated
- `py/compose/pyproject.toml` - updated
- `py/compose/compose.py` - updated (Session fixes)
- `py/compose/compose.test.py` - updated
