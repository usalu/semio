# Summary

Configured Vitest properly for the monorepo to fix the VSCode extension warning about multiple projects.

## Changes

1. **vitest.config.ts** - Updated to use the new Vitest v4 `test.projects` configuration that explicitly defines which configs contain tests:
   - `repo` project (inline) for root-level repo.tests.ts
   - Reference to `js/semio/vite.config.ts` for the semio package tests

2. **js/semio/vite.config.ts** - Added `name: "semio"` to the test configuration for proper project identification

The VSCode Vitest extension will now only detect the 2 configured test projects instead of all vite.config files in the monorepo.
