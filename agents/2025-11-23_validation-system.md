# Semio Validation & Diff-Based Fix System

**Date:** 2025-11-23  
**Status:** Planning → Implementation

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

## Implementation Plan

1. ✓ Add validation core types to `semio.ts`
2. ✓ Add validation context & engine
3. ✓ Add fix helper
4. ✓ Implement GUID uniqueness rule
5. ✓ Implement design sibling name rule
6. ✓ Implement piece name uniqueness rule
7. ✓ Register default rules
8. ✓ Implement VS Code extension
9. ✓ Update extension package.json

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

## Future Extensions

- CLI validation tool
- Backend API validation endpoint
- Sketchpad validation panel
- Auto-fix on save
- Custom rule plugins
- Performance optimizations (incremental validation)
