# Ticket

## Todos

# Plan: Configure Vitest Properly Across the Monorepo

## Problem

The Vitest VSCode extension detects all `vite.config.*` files as potential vitest projects, causing the warning:

> "Vitest found multiple projects. The extension will use only the first 5 due to performance concerns."

## Current State

- Root `vitest.config.ts` - tests `repo.tests.ts`
- `js/compose/vite.config.ts` - has test config for `compose.test.ts`
- `js/vscode/vite.config.ts` - no test config (uses plain `vite`)
- `js/play/vite.config.ts` - no test config (uses plain `vite`)
- `js/sketchpad/vite.config.ts` - no test config (uses plain `vite`)
- `js/temp/vite.config.ts` - no test config (uses plain `vite`)

## Solution

Create a `vitest.workspace.ts` file at the root that explicitly defines which projects have vitest tests. This tells the extension exactly which configs to use.

## Steps

1. Create `vitest.workspace.ts` at root with explicit project definitions
2. Verify the configuration works with `vitest --reporter=verbose`

## Files to Create/Modify

- Create: `vitest.workspace.ts`

## Changes

## Log

# Log

## Investigation

Explored the monorepo structure:

- Found 5 vite.config files in js/ folder: compose, vscode, play, sketchpad, temp
- Found 1 vitest.config.ts at root
- Only 2 have actual test configurations:
  - Root: `vitest.config.ts` → tests `repo.tests.ts`
  - `js/compose/vite.config.ts` → tests `compose.test.ts`

The Vitest extension is detecting all vite.config files as potential projects, hence the warning.

## Solution

In Vitest v4, the workspace configuration changed. The `test.workspace` option was removed and replaced with `test.projects`.

Updated `vitest.config.ts` to use the new `test.projects` configuration:

```typescript
export default defineConfig({
 test: {
  projects: [
   {
    test: {
     name: "repo",
     include: ["repo.tests.ts"],
     testTimeout: 60000,
    },
   },
   "./js/compose/vite.config.ts",
  ],
 },
});
```

This explicitly defines the projects with vitest tests, preventing the extension from detecting all vite.config files as separate projects.

Also added `name: "compose"` to `js/compose/vite.config.ts` test configuration for proper project identification.

### Verification

Both projects are now properly recognized:

- `npx vitest run --project repo --passWithNoTests` → works
- `npx vitest run --project compose --passWithNoTests` → works
- `npx vitest run --passWithNoTests` → runs tests from both projects

## Summary

Bulk close

## Changes

1. **vitest.config.ts** - Updated to use the new Vitest v4 `test.projects` configuration that explicitly defines which configs contain tests:
   - `repo` project (inline) for root-level repo.tests.ts
   - Reference to `js/compose/vite.config.ts` for the compose package tests

2. **js/compose/vite.config.ts** - Added `name: "compose"` to the test configuration for proper project identification

The VSCode Vitest extension will now only detect the 2 configured test projects instead of all vite.config files in the monorepo.
