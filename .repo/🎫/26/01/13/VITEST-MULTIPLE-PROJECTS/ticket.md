# Ticket

## Todos
# Plan: Investigate Vitest Multiple Projects Warning

## Objective
Investigate why the VS Code Vitest extension reports "multiple projects" and suggest solutions.

## Steps
1. Find all vitest/vite config files in the repository (excluding node_modules)
2. Analyze which configs actually contain test configuration
3. Review current VS Code settings for any vitest configuration
4. Propose solutions to resolve the warning

## Expected Deliverables
- List of all relevant config files
- Analysis of which configs are actually used for testing
- Recommended solutions with pros/cons

## Changes

## Log
# Investigation Log: Vitest Multiple Projects Warning

## Warning Message
```
Vitest found multiple projects. The extension will use only the first 5 due to performance concerns.
Consider using a projects configuration to group your configs or increase the limit via "vitest.maximumConfigs" option.
```

## Findings

### Config Files Found (excluding node_modules)

| File | Import Source | Has `test` Block | Purpose |
|------|--------------|------------------|---------|
| `vitest.config.ts` | `vitest/config` | Yes | Root repo tests (`repo.tests.ts`) |
| `js/semio/vite.config.ts` | `vitest/config` | Yes | Semio library tests (`semio.test.ts`) |
| `js/play/vite.config.ts` | `vite` | No | Playground dev server |
| `js/vscode/vite.config.ts` | `vite` | No | VS Code extension build |
| `js/sketchpad/vite.config.ts` | `vite` | No | Sketchpad dev server |
| `js/temp/vite.config.ts` | `vite` | No | Temp project dev server |

### Analysis

The Vitest VS Code extension scans for both `vitest.config.*` and `vite.config.*` files because Vitest can be configured within Vite configs. It found **6 config files** but only **2 actually contain test configuration**:

1. **Root `vitest.config.ts`** - Tests repo-level functionality
2. **`js/semio/vite.config.ts`** - Tests the core semio library

The other 4 files (`js/play`, `js/vscode`, `js/sketchpad`, `js/temp`) are pure Vite configs for building/dev serving without any test configuration.

### Current VS Code Settings

No vitest-specific settings currently exist in `.vscode/settings.json`.

---

## Proposed Solutions

### Solution 1: Increase the Config Limit (Quick Fix)

Add to `.vscode/settings.json`:
```json
{
  "vitest.maximumConfigs": 10
}
```

**Pros:**
- Quick and simple
- No changes to project structure

**Cons:**
- Doesn't solve the root issue
- Extension still processes configs that have no tests

---

### Solution 2: Create a Vitest Workspace (Recommended)

Create `vitest.workspace.ts` at root:
```typescript
import { defineWorkspace } from 'vitest/config'

export default defineWorkspace([
  'vitest.config.ts',
  'js/semio/vite.config.ts',
])
```

**Pros:**
- Explicitly defines which configs are used for testing
- Better performance - only processes relevant configs
- Standard Vitest approach for monorepos
- Clear documentation of test locations

**Cons:**
- Adds another config file
- Need to maintain list when adding new test configs

---

### Solution 3: Use Extension Glob Patterns

Add to `.vscode/settings.json`:
```json
{
  "vitest.include": [
    "vitest.config.ts",
    "js/semio/vite.config.ts"
  ]
}
```

**Pros:**
- No new files needed
- VS Code-specific, doesn't affect CLI

**Cons:**
- Not portable to other editors
- Must maintain list in settings

---

### Solution 4: Exclude Non-Test Vite Configs

Add to `.vscode/settings.json`:
```json
{
  "vitest.exclude": [
    "js/play/**",
    "js/vscode/**",
    "js/sketchpad/**",
    "js/temp/**"
  ]
}
```

**Pros:**
- Explicit about what to skip

**Cons:**
- Must update when adding new vite projects
- Negative pattern (exclude) is less clear than positive (include)

---

## Recommendation

**Use Solution 2 (Vitest Workspace)** - This is the standard approach for monorepos and provides the cleanest separation. It makes the test structure explicit and works with both the VS Code extension and CLI.

If a quick fix is preferred, **Solution 1** (increase limit) combined with **Solution 3** (include patterns) works well.

## Summary
# Summary: Vitest Multiple Projects Warning

## Issue
The VS Code Vitest extension found 6 config files (`vitest.config.*` and `vite.config.*`) but only uses the first 5 by default.

## Root Cause
The extension scans for all Vite/Vitest configs. Only 2 actually contain test configuration:
- `vitest.config.ts` (root)
- `js/semio/vite.config.ts`

The other 4 (`js/play`, `js/vscode`, `js/sketchpad`, `js/temp`) are pure Vite build configs without tests.

## Recommended Solution
Create a `vitest.workspace.ts` file at the root to explicitly define which configs are used for testing:

```typescript
import { defineWorkspace } from 'vitest/config'

export default defineWorkspace([
  'vitest.config.ts',
  'js/semio/vite.config.ts',
])
```

This is the standard Vitest approach for monorepos and provides the best performance.

## Alternative Quick Fix
Add to `.vscode/settings.json`:
```json
{
  "vitest.maximumConfigs": 10
}
```
