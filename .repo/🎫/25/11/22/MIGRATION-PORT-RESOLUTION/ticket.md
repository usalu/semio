# Ticket

## Todos
# Migration Script Connector Resolution Problem

## Current Status

- Assets restored from HEAD~30 (30 commits ago)
- Connection sides have NO GUIDs (correct per user requirement)
- Migration runs in 3 phases: Types -> Kit -> Designs
- Phase 4 removed (was incorrectly loading unmigrated connections)

## Critical Problem

**Pieces in migrated kit designs have NO type references**, causing flatten tests to fail.

### Evidence

```powershell
# Type map populated correctly
[DEBUG] Type map has 93 keys before migrating designs

# But pieces don't get types
[DEBUG] First piece after migration - has type: False

# Original data HAS types
git show HEAD~30:assets/compose/kit_metabolism.json
# Shows: pieces[0].type = {name: "Base", variant: ""}

# After migration: pieces[0].type = null
```

### Root Cause Analysis

1. **Kit migration flow**: `Migrate-Kit` calls `Migrate-Design` with `$typeNameToGuidMap`
2. **`Migrate-Design`** calls `Migrate-Pieces` which calls `Migrate-Piece`
3. **Problem**: `$typeNameToGuidMap` has 93 keys but pieces still don't get types
4. **No warnings**: `Migrate-Piece` doesn't trigger the type lookup warning, suggesting:
   - Either pieces don't have `type` property, OR
   - The property check isn't working as expected

### Next Steps

1. Add debug logging to `Migrate-Pieces` to see if it's being called
2. Add debug logging to `Migrate-Piece` to see if type property exists
3. Verify parameter passing chain: Kit -> Design -> Pieces -> Piece
4. Check if original kit data structure is compatible with current migration logic

### Test Status

- **flattenDesign tests**: ALL FAILING (6/6)
- **Root cause**: Pieces have no type references
- **Impact**: Without types, cannot lookup connectors for plane computation

### Key Files

- `scripts/migrate-compose-json.ps1`: Migration script
- `js/compose/compose.test.ts`: Flatten tests
- `assets/compose/kit_metabolism.json`: Main kit file

### User Requirements

1. Flatten tests must succeed (primary goal)
2. Always normalize JSON output
3. Connection sides have no GUIDs (completed)
4. Always reset assets from 30 commits ago (completed)

# Migration Script Connector Resolution - Final Status

## Completed

1. **Assets reset from HEAD~30** - All JSON files from 30 commits ago
2. **Connection sides have NO GUIDs** - Removed from `connected` and `connecting` sides
3. **Pieces have type references** - Fixed type mapping with "name|variant" keys
4. **Migration simplified** - Only migrates kit file (all data is embedded)
5. **JSON normalization** - Using `ConvertTo-Json -Compress:$false`

## Key Fixes Applied

### 1. Removed Pre-loading (CRITICAL FIX)

Pre-loading types from standalone files was loading already-migrated files with wrong structure (name=variant instead of name="Capsule", variant="\"). This interfered with proper type resolution.

```powershell
# REMOVED: Pre-loading from standalone type files
# Now: Kit migration is self-contained
```

### 2. Fixed Type Mapping

Type GUIDs are stored with multiple keys for flexible lookup:

- `name|variant|parent` (unique key)
- `name|variant` (piece lookup key) ← CRITICAL
- `name` (simple fallback)

### 3. Simplified Migration Flow

```
Before: Types → Kit → Designs → Kit (reload)
Now:    Kit only (types and designs embedded)
```

### 4. Fixed Abstract Parent Creation

Properly handle parent names as strings before GUID conversion, avoiding PowerShell hashtable stringification bugs.

## Test Results

- **Kit migration**: SUCCESS
- **Pieces have types**: YES
- **Connections have connectors**: PARTIAL (some connectors missing from original data)
- **flattenDesign tests**: STILL FAILING

## Remaining Problems

###

## Changes

## Log

## Summary
