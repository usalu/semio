# Ticket

## Todos
# Plan - REPO-TREE-REFACTOR

## Backend (Go/GraphQL)
- [ ] Revert `Range` type to use `Int` for `start`/`end` instead of `Position`.
- [ ] Verify/Add `sections` field to `File` type.
- [ ] Verify/Add `definitions` field to `Section` type.
- [ ] Update `repo.go` resolvers for `Range`, `File.sections`, `Section.definitions`.

## Frontend (VS Code)
- [ ] Regenerate GraphQL types.
- [ ] Update `CodebaseProvider` (Tree layout):
    - [ ] Root should be bundles (`repo.bundles`), not the repo itself.
    - [ ] `File` items should represent children as sections.
    - [ ] `Section` items should represent children as definitions.
- [ ] Ensure `TreeItem.collapsibleState` is set to `Collapsed` for expandable items (Bundles, Folders, Files, Sections).
- [ ] Implement `getChildren` logic to fetch deeper levels lazily if not already done.

## Verification
- [ ] Check `Range` type in schema.
- [ ] Check Tree View structure in VS Code (simulated via looking at code).

## Changes

## Log
# Node ID Refactoring

## Analysis

Current ID system is inconsistent:
- Repo: "repo" or hardcoded
- Bundle: `"bundle:" + name`
- Folder: `"folder:" + path` 
- File: `"file:" + path`
- Section: `"section:" + name` (not unique!)
- Definition: `"definition:" + name` (not unique!)
- Contributor: `"contributor:" + github`
- Ticket: `"ticket:YYYY/MM/DD/SLUG"` 
- Policy: `"policy:" + name`
- ViolationKind: `"violationKind:" + kind` (not linked to policy!)
- Violation: timestamp-based (not deterministic!)

## New ID System

Following semio's `@` naming convention:

```
@semio                                          - repo
@semio/BUNDLE                                   - bundle
@semio/repo/FOLDER/...                          - folder outside bundle
@semio/BUNDLE/FOLDER/...                        - folder in bundle
@semio/repo/PATH/FILE                           - file outside bundle
@semio/BUNDLE/PATH/FILE                         - file in bundle
@semio/BUNDLE/PATH/FILE#SECTION#SUBSECTION      - section
@semio/BUNDLE/PATH/FILE#SECTION§DEFINITION      - definition
@semio/contributors/GITHUB                      - contributor
@semio/tickets/YYYY/MM/DD/SLUG                  - ticket
@semio/commits/SHA                              - commit
@semio/policies/POLICY                          - policy
@semio/policies/POLICY/violations/KIND          - violationKind
@semio/violations/SCOPE#LINE:COL                - violation
```

Benefits:
- Globally unique
- Human-readable
- Path-based (natural hierarchy)
- Deterministic (except violations which are ephemeral)
- Consistent with semio naming

## Implementation Progress

### Phase 1: Core GetID() Methods ✅

Updated GetID() methods for:
- `Repo` → `"@semio"`
- `Bundle` → `"@semio/" + name`
- `Section` → `filePath + "#" + sectionPath` (with fallback)
- `Definition` → `filePath + "#" + sectionPath + "§" + name` (with fallback)
- `Contributor` → `"@semio/contributors/" + github`
- `Ticket` → `"@semio/tickets/YYYY/MM/DD/SLUG"`
- `Commit` → `"@semio/commits/" + SHA`
- `Policy` → `"@semio/policies/" + name`
- `ViolationKindMeta` → `"@semio/policies/" + policyID + "/violations/" + kind`

Added fields to support proper IDs:
- `Definition.FilePath` and `Definition.SectionPath` for context
- `ViolationKindMeta.PolicyID` to link violations to policies

### Phase 2: Helper Functions ✅

Created helper functions for ID construction:
- `buildFolderID(path, bundleID)` - folder IDs with bundle context
- `buildFileID(path, bundleID)` - file IDs with bundle context
- `buildSectionID(fileID, sectionPath)` - section IDs
- `buildDefinitionID(fileID, sectionPath, name)` - definition IDs
- `buildViolationID(scope, line, col)` - violation IDs

### Phase 3: Update ID Generation Sites ✅

Updated functions to use new helper functions:
- `FolderCreate()` - uses `buildFolderID()`
- `FolderMove()` - uses `buildFolderID()`
- `FileCreate()` - uses `buildFileID()` and `buildFolderID()`
- `FileMove()` - uses `buildFileID()` and `buildFolderID()`
- `CreateViolation()` - uses `buildViolationID()`
- `Policy()` resolver - uses new format directly

Added bundle resolution to file/folder operations using `ResolveBundleForPath()`.

### Phase 4: Fix Compilation Issues ✅

- Added `GetDefinitions()` method to `defaultContext`
- Added `GetSections()` method to `defaultContext`
- Updated `GetDefinitions()` in `repoContext` to set `FilePath` field

## Next Steps

### Remaining ID Generation Sites:

1. **Section GraphQL field resolver** (line ~8527): Section's file field still uses old format
2. **Section listing functions**: Need to update where sections are created/listed
3. **Folder/file tree building functions**: May have additional ID generation
4. **Tests**: All tests need updating to expect new ID format

### Compilation Status: ✅ No errors

### Testing Plan:

1. Run Go tests: `cd go/repo && go test -v`
2. Run CLI tests: `cd go/cli && go test -v`
3. Test GraphQL queries manually
4. Test VS Code extension integration
5. Test MCP server integration

### Breaking Changes:

This is a breaking change. All existing code that relies on node IDs will need updates:
- VS Code extension queries
- MCP server tool responses
- Any external systems using the GraphQL API

All IDs are now deterministic based on file paths and bundle membership, making them reproducible across different instances.

## Summary
# Summary - Node ID Refactoring

## Completed

Successfully refactored the Node ID system to use a consistent, globally unique format based on semio's `@` naming convention.

### Changes Made:

1. **Updated all GetID() methods** to return new ID format:
   - Repo: `@semio`
   - Bundle: `@semio/BUNDLE`
   - Contributor: `@semio/contributors/GITHUB`
   - Ticket: `@semio/tickets/YYYY/MM/DD/SLUG`
   - Commit: `@semio/commits/SHA`
   - Policy: `@semio/policies/NAME`
   - ViolationKindMeta: `@semio/policies/POLICY/violations/KIND`

2. **Added new fields** for proper ID construction:
   - `Definition.FilePath` and `Definition.SectionPath`
   - `ViolationKindMeta.PolicyID`

3. **Created helper functions**:
   - `buildFolderID(path, bundleID)` 
   - `buildFileID(path, bundleID)`
   - `buildSectionID(fileID, sectionPath)`
   - `buildDefinitionID(fileID, sectionPath, name)`
   - `buildViolationID(scope, line, col)`

4. **Updated ID generation sites**:
   - Folder/File Create, Move operations
   - Violation creation
   - Policy initialization
   - Query resolvers for Folder and File

5. **Set PolicyID** on all ViolationKindMeta instances in GetPolicies() and GetViolationKinds()

## Compilation Status

✅ No compilation errors - code compiles successfully

## Remaining Work

1. One GraphQL field resolver for Section's file field (line ~8527)
2. Update tests to expect new ID format
3. Integration testing with VS Code extension and MCP server

## Benefits

- **Globally unique IDs**: No more collisions
- **Human-readable**: IDs are clear and descriptive
- **Deterministic**: Same codebase always generates same IDs (except ephemeral violations)
- **Path-based hierarchy**: Natural navigation structure
- **Consistent**: All IDs follow the same `@semio/...` pattern

## Breaking Changes

This is a breaking change for any code that relies on node IDs. All external systems will need to update their ID expectations.

Files modified: `go/repo/repo.go`
