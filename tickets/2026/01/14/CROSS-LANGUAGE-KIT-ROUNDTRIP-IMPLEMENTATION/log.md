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
- Implemented `KitFromZip` - extracts zip and loads Kit from .semio/kit.db
- Implemented `KitToZip` - creates zip with .semio/kit.db and files
- Fixed boolean handling (sql.NullBool instead of int)
- Test `TestKitZipRoundtrip` passes: loaded Metabolism kit with 49 types, 10 designs, 321 files

### JS/TS Verification

- `importKit` and `exportKit` functions already exist in semio.ts
- Added Zip roundtrip test to semio.test.ts
- Test "Zip -> Kit -> Zip -> Kit (roundtrip)" passes (11427ms)

### Deferred

- Rust implementation - significant effort, deferred
- .NET implementation - significant effort, deferred
- Python roundtrip - blocked by schema mismatch issues (separate ticket needed)
