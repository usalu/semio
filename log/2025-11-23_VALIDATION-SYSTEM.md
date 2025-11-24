# Semio Validation & Diff-Based Fix System

**Date:** 2025-11-23  
**Status:** ✅ Complete

## Overview

This document describes the implementation of a clean validation architecture for Semio:

1. **Pure domain logic in `semio.ts`** - No JSON, no editor constructs, just `Kit` and `KitDiff`
2. **Diff-based fixes** - Every suggestion is a `KitDiff` using existing tooling
3. **Minimal VS Code extension** - JSON linter that uses domain logic

## Architecture

### Layer 1: Domain Logic (`semio.ts`)

Pure functions working only with `Kit` and `KitDiff`:

- **Validation Core Types**: `SemioValidationIssue`, `SemioKitFix`, `SemioDomainLocation`
- **Validation Engine**: `validateSemioKit`, `SemioValidationRule`
- **Fix Helper**: `semioMakeFix` (generates `KitDiff` from mutations)
- **Default Rules**:
  - GUID uniqueness
  - Design sibling name uniqueness
  - Piece name uniqueness within design

### Layer 2: VS Code Extension (`js/vscode`)

JSON-aware linter:

- Parses JSON → `Kit` using `deserializeKit`
- Runs `validateSemioKit`
- Maps `SemioDomainLocation` → JSON ranges
- Applies fixes via `applyKitDiff` + `serializeKit` + full document replacement

## Implementation Status

1. ✅ Add validation core types to `semio.ts`
2. ✅ Add validation context & engine
3. ✅ Add fix helper
4. ✅ Implement GUID uniqueness rule
5. ✅ Implement type name uniqueness rule
6. ✅ Implement design name uniqueness rule
7. ✅ Implement piece name uniqueness rule
8. ✅ Implement quality name uniqueness rule
9. ✅ Implement interface name uniqueness rule
10. ✅ Implement file name uniqueness rule
11. ✅ Implement folder name uniqueness rule
12. ✅ Implement port name uniqueness rule
13. ✅ Implement model name uniqueness rule
14. ✅ Implement layer path uniqueness rule
15. ✅ Register all default rules
16. ✅ Implement VS Code extension
17. ✅ Update extension package.json
18. ✅ Update AGENTS.md with validation specs
19. ✅ Update engineering files (softwarearchitecture.pu)
20. ✅ Create central VALIDATION.md
21. ✅ Update VS Code README to reference central docs

## Validation Rules Implemented

### Name Uniqueness Rules (11 total)

All implemented rules follow the same pattern:

- Collect entities by scope (global, siblings, within parent)
- Group by name/path
- Find duplicates
- Generate fixes with `semioMakeFix` and `generateUniqueName`

| Rule                    | Scope         | Field | Status         |
| ----------------------- | ------------- | ----- | -------------- |
| `guid-unique`           | Global        | guid  | ✅ Implemented |
| `type-name-unique`      | Siblings      | name  | ✅ Implemented |
| `design-name-unique`    | Siblings      | name  | ✅ Implemented |
| `piece-name-unique`     | Within design | name  | ✅ Implemented |
| `quality-name-unique`   | Global        | name  | ✅ Implemented |
| `interface-name-unique` | Global        | name  | ✅ Implemented |
| `file-name-unique`      | Global        | name  | ✅ Implemented |
| `folder-name-unique`    | Siblings      | name  | ✅ Implemented |
| `port-name-unique`      | Within type   | name  | ✅ Implemented |
| `model-name-unique`     | Within type   | name  | ✅ Implemented |
| `layer-path-unique`     | Within design | path  | ✅ Implemented |

## Key Design Decisions

### 100% JSON-Agnostic Domain Logic

`semio.ts` contains **zero JSON logic**:

- No JSON paths
- No JSON parsing/serialization (except internal cloning for diffs)
- No editor-specific types

### Diff-Based Fixes

Every fix is a `KitDiff`:

- Created with `getKitDiff(before, after)`
- Applied with `applyKitDiff(base, diff)`
- Inverted with `inverseKitDiff(original, appliedDiff)`

This ensures:

- Reusability across all platforms (Sketchpad UI, CLI, backend, VS Code)
- Minimal changes (only what changed)
- Undo/redo support (via inverse diffs)

### Domain-Only Locations

`SemioDomainLocation` describes "where" in domain terms:

```typescript
{
  entityKind: "Piece",
  entityGuid: "01234567-89ab-cdef-0123-456789abcdef",
  field: "name"
}
```

The VS Code extension translates this to JSON ranges.

## Usage Examples

### In Sketchpad UI

```typescript
const result = validateSemioKit(currentKit);
showIssues(result.issues);

function applyFix(issue: SemioValidationIssue, fix: SemioKitFix) {
  const newKit = applyKitDiff(currentKit, fix.diff);
  setCurrentKit(newKit);
}
```

### In VS Code Extension

```typescript
const kit = deserializeKit(jsonString);
const result = validateSemioKit(kit);
const diagnostics = result.issues.map(issueToDiagnostic);
const codeActions = result.issues.flatMap(issueToCodeActions);
```

## Benefits

1. **Single source of truth** - Validation logic defined once, used everywhere
2. **Platform-agnostic** - Works in browser, Node.js, Deno, CLI, backend
3. **Testable** - Pure functions, easy to unit test
4. **Reusable fixes** - Diff-based fixes work in any context
5. **Minimal VS Code extension** - Just JSON parsing + UI glue

## Implementation Details

### Domain Logic (`js/js/semio.ts`)

Added ~550 lines of pure validation logic:

- **Core Types** (7 types): `SemioEntityKind`, `SemioValidationSeverity`, `SemioDomainLocation`, `SemioKitFix`, `SemioValidationIssue`, `SemioValidationResult`, `SemioValidationContext`
- **Engine** (5 functions): `buildSemioValidationContext`, `validateSemioKit`, `semioMakeFix`, `hasSemioErrors`, `updateGuidEverywhere`
- **Rules** (11 rules):
  - `semioGuidUniquenessRule`
  - `semioTypeNameUniquenessRule`
  - `semioDesignNameUniquenessRule`
  - `semioPieceNameUniquenessRule`
  - `semioQualityNameUniquenessRule`
  - `semioInterfaceNameUniquenessRule`
  - `semioFileNameUniquenessRule`
  - `semioFolderNameUniquenessRule`
  - `semioPortNameUniquenessRule`
  - `semioModelNameUniquenessRule`
  - `semioLayerPathUniquenessRule`

### VS Code Extension (`js/vscode`)

Added ~280 lines of JSON-aware linting:

- **Validation**: Auto-validates on open/change/save
- **Diagnostics**: Converts `SemioValidationIssue` → VS Code `Diagnostic`
- **Quick Fixes**: Applies `KitDiff` and replaces entire document
- **JSON Mapping**: Uses `jsonc-parser` to map domain locations → ranges

### Files Modified

1. `js/js/semio.ts` - Added validation system after Kit Import/Export section (~550 lines)
2. `js/vscode/src/extension.ts` - Complete rewrite with validation logic (~280 lines)
3. `js/vscode/package.json` - Updated metadata and added dependencies
4. `js/vscode/README.md` - Updated with references to central documentation
5. `VALIDATION.md` - Central validation documentation (new file)
6. `AGENTS.md` - Added validation section with complete rule documentation
7. `engineering/softwarearchitecture.pu` - Added validation class diagram
8. `agents/2025-11-23_validation-system.md` - This planning document

## Testing

To test the VS Code extension:

1. Open VS Code in `js/vscode` folder
2. Run `npm install` to install dependencies (including `jsonc-parser`)
3. Press `F5` to launch Extension Development Host
4. Open a kit JSON file (e.g., `assets/semio/kit_metabolism.json`)
5. Intentionally create validation errors:
   - Duplicate a GUID
   - Duplicate a design name among siblings
   - Duplicate a piece name within a design
6. Verify diagnostics appear
7. Click lightbulb 💡 or press `Ctrl+.` to apply Quick Fixes

## Future Extensions

- CLI validation tool
- Backend API validation endpoint
- Sketchpad validation panel
- Auto-fix on save
- Custom rule plugins
- Performance optimizations (incremental validation)

## Recent Work

- Normalized every slug in `log/` filenames by uppercasing them so the directory now follows a consistent naming convention.
