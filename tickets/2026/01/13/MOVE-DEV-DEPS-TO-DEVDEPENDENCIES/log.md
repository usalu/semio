# Log

## 2026-01-13

### Task: Move dev dependencies to devDependencies

Analyzed `js/semio/package.json` and identified 4 packages in `dependencies` that should be in `devDependencies`:

1. **Type definitions** (only needed for TypeScript compilation):
   - `@types/d3-force`: "^3.0.10"
   - `@types/dagre`: "^0.7.53"

2. **PostCSS plugins** (only needed during build):
   - `postcss-import`: "^16.1.1"
   - `postcss-nesting`: "^13.0.2"

### Changes Made

1. Removed 4 packages from `dependencies` section
2. Added them to `devDependencies` section in alphabetical order:
   - `@types/d3-force` and `@types/dagre` placed before `@types/eslint`
   - `postcss-import` placed after `postcss`
   - `postcss-nesting` placed after `postcss-load-config`

### Result
- File: `js/semio/package.json` updated successfully
- All 4 dev dependencies moved to their correct section
