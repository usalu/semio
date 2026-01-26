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
