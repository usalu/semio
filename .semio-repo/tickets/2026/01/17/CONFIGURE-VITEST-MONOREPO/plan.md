# Plan: Configure Vitest Properly Across the Monorepo

## Problem
The Vitest VSCode extension detects all `vite.config.*` files as potential vitest projects, causing the warning:
> "Vitest found multiple projects. The extension will use only the first 5 due to performance concerns."

## Current State
- Root `vitest.config.ts` - tests `repo.tests.ts`
- `js/semio/vite.config.ts` - has test config for `semio.test.ts`
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
