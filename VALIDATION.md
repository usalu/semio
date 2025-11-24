# Semio Validation System

**Domain-pure validation with diff-based fixes**

## Overview

Semio includes a validation system built entirely in `semio.ts` with **zero JSON dependencies**. All validation logic works with `Kit` objects and produces `KitDiff`-based fixes that can be applied, inverted, and merged.

## Architecture

### Layer 1: Domain Logic (`js/js/semio.ts`)

Pure functions working only with `Kit` and `KitDiff`:

- **100% JSON-agnostic** - No JSON paths, parsing, or serialization
- **Pure functions** - Deterministic and side-effect free
- **Diff-based fixes** - Every fix is a `KitDiff`
- **Reusable everywhere** - Browser, Node.js, Deno, CLI, backend, VS Code

### Layer 2: Platform Integrations

Each platform provides its own thin wrapper:

- **VS Code Extension** (`js/vscode`) - JSON linter with Quick Fixes
- **Sketchpad UI** - In-app validation panel
- **CLI** - Command-line validation tool
- **Backend** - API validation endpoint

## Validation Rules

### 1. GUID Uniqueness (`guid-unique`)

**Severity:** Error  
**Scope:** Global

All GUIDs must be unique across the entire kit for:

- Kit, Types, Designs, Pieces, Connections, Stats
- Qualities, Interfaces, Files, Folders

**Fix:** Regenerates GUID and updates all references.

### 2. Type Name Uniqueness (`type-name-unique`)

**Severity:** Error  
**Scope:** Siblings (same parent)

Types with the same parent must have unique names.

**Fix:** Renames with suffix (e.g., "Wall 2", "Wall 3").

### 3. Design Name Uniqueness (`design-name-unique`)

**Severity:** Error  
**Scope:** Siblings (same parent)

Designs with the same parent must have unique names.

**Fix:** Renames with suffix.

### 4. Piece Name Uniqueness (`piece-name-unique`)

**Severity:** Error  
**Scope:** Within design

Pieces within a design must have unique names.

**Fix:** Renames with suffix.

### 5. Quality Name Uniqueness (`quality-name-unique`)

**Severity:** Error  
**Scope:** Global

All qualities must have unique names.

**Fix:** Renames with suffix.

### 6. Interface Name Uniqueness (`interface-name-unique`)

**Severity:** Error  
**Scope:** Global

All interfaces must have unique names.

**Fix:** Renames with suffix.

### 7. File Name Uniqueness (`file-name-unique`)

**Severity:** Error  
**Scope:** Global

All files must have unique names.

**Fix:** Renames with suffix.

### 8. Folder Name Uniqueness (`folder-name-unique`)

**Severity:** Error  
**Scope:** Siblings (same parent)

Folders with the same parent must have unique names.

**Fix:** Renames with suffix.

### 9. Port Name Uniqueness (`port-name-unique`)

**Severity:** Error  
**Scope:** Within type

Ports within a type must have unique names.

**Fix:** Renames with suffix.

### 10. Model Name Uniqueness (`model-name-unique`)

**Severity:** Error  
**Scope:** Within type

Models within a type must have unique names.

**Fix:** Renames with suffix.

### 11. Layer Path Uniqueness (`layer-path-unique`)

**Severity:** Error  
**Scope:** Within design

Layer paths within a design must be unique.

**Fix:** Renames with suffix.

## Uniqueness Summary

| Entity     | Scope         | Field | Rule ID               |
| ---------- | ------------- | ----- | --------------------- |
| Kit        | Global        | guid  | guid-unique           |
| Type       | Siblings      | name  | type-name-unique      |
| Type       | Global        | guid  | guid-unique           |
| Design     | Siblings      | name  | design-name-unique    |
| Design     | Global        | guid  | guid-unique           |
| Piece      | Within design | name  | piece-name-unique     |
| Piece      | Global        | guid  | guid-unique           |
| Connection | Global        | guid  | guid-unique           |
| Port       | Within type   | name  | port-name-unique      |
| Model      | Within type   | name  | model-name-unique     |
| Quality    | Global        | name  | quality-name-unique   |
| Quality    | Global        | guid  | guid-unique           |
| Interface  | Global        | name  | interface-name-unique |
| Interface  | Global        | guid  | guid-unique           |
| File       | Global        | name  | file-name-unique      |
| File       | Global        | guid  | guid-unique           |
| Folder     | Siblings      | name  | folder-name-unique    |
| Folder     | Global        | guid  | guid-unique           |
| Layer      | Within design | path  | layer-path-unique     |
| Stat       | Global        | guid  | guid-unique           |

## Usage

### In TypeScript/JavaScript

```typescript
import { validateSemioKit, hasSemioErrors, applyKitDiff } from "@semio/js/semio";

// Validate a kit
const result = validateSemioKit(kit);

// Check for errors
if (hasSemioErrors(result)) {
  console.error("Validation errors:", result.issues);
}

// Apply a fix
const issue = result.issues[0];
const fix = issue.fixes[0];
const fixedKit = applyKitDiff(kit, fix.diff);
```

### Custom Rules

```typescript
import { SemioValidationRule, defaultSemioValidationRules } from "@semio/js/semio";

const myRule: SemioValidationRule = (ctx) => {
  const issues = [];
  // Custom validation logic
  return issues;
};

const result = validateSemioKit(kit, {
  rules: [...defaultSemioValidationRules, myRule],
});
```

### In VS Code

1. Install the Semio VS Code extension
2. Open a kit JSON file (`kit_*.json`, `*_kit.json`, or `kit.json`)
3. Validation happens automatically
4. Click lightbulb 💡 or press `Ctrl+.` for Quick Fixes

## Creating New Rules

1. **Define the rule** following `SemioValidationRule` signature:

```typescript
export const myRule: SemioValidationRule = (ctx) => {
  const issues: SemioValidationIssue[] = [];
  // Validation logic here
  return issues;
};
```

2. **Use `semioMakeFix`** to generate diff-based fixes:

```typescript
const fix = semioMakeFix(ctx, "Fix description", (clone) => {
  // Mutate the clone
  clone.someProperty = newValue;
});
```

3. **Register the rule**:

```typescript
defaultSemioValidationRules.push(myRule);
```

4. **Document** in `AGENTS.md` and this file.

## Core Types

### SemioValidationIssue

```typescript
interface SemioValidationIssue {
  ruleId: string; // Unique identifier
  severity: "error" | "warning"; // Severity level
  message: string; // Human-readable message
  location: SemioDomainLocation; // Where the issue is
  relatedGuids?: Guid[]; // Other involved entities
  fixes: SemioKitFix[]; // Suggested fixes
}
```

### SemioKitFix

```typescript
interface SemioKitFix {
  title: string; // Display text
  diff: KitDiff; // Minimal diff to apply
}
```

### SemioDomainLocation

```typescript
interface SemioDomainLocation {
  entityKind: SemioEntityKind; // Type of entity
  entityGuid?: Guid; // Entity identifier
  field?: string; // Specific field
}
```

## Benefits

1. **Single source of truth** - One validation system for all platforms
2. **Platform-agnostic** - Pure domain logic, no platform dependencies
3. **Testable** - Pure functions with predictable outputs
4. **Reusable fixes** - Diffs work everywhere (UI, CLI, backend, VS Code)
5. **Undo/redo support** - Diffs can be inverted
6. **Minimal integrations** - Platforms need only thin wrappers

## Platform Implementations

- **VS Code** - See `js/vscode/README.md`
- **Sketchpad** - See `js/js/sketchpad/README.md` (planned)
- **CLI** - See `cli/README.md` (planned)
- **Backend** - See `backend/README.md` (planned)

## Testing

```typescript
import { describe, it, expect } from "vitest";
import { validateSemioKit, createKit } from "@semio/js/semio";

describe("Validation", () => {
  it("detects duplicate GUIDs", () => {
    const kit = createKit({
      /* ... */
    });
    const result = validateSemioKit(kit);
    expect(result.issues).toHaveLength(1);
    expect(result.issues[0].ruleId).toBe("guid-unique");
  });
});
```

## Performance

- **Incremental validation** - Planned for large kits
- **Caching** - Validation context is built once per kit
- **Parallel rules** - Rules can be run in parallel (future)

## Future Enhancements

- Reference validation (type references, port interfaces)
- Geometric validation (plane validity, intersection checks)
- Semantic validation (mandatory ports, connection compatibility)
- Performance profiling and optimization
- Rule configuration (enable/disable rules)
- Custom rule plugins
- Batch fixes (fix all issues at once)
- Incremental validation (validate only changed parts)
