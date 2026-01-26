# Plan: Move Dev Dependencies to devDependencies

## Objective
Move development-only packages from `dependencies` to `devDependencies` in `js/semio/package.json`.

## Packages to Move
The following packages are development-only and should be in `devDependencies`:

1. **Type definitions** (only needed for TypeScript compilation):
   - `@types/d3-force`: "^3.0.10"
   - `@types/dagre`: "^0.7.53"

2. **PostCSS plugins** (only needed during build):
   - `postcss-import`: "^16.1.1"
   - `postcss-nesting`: "^13.0.2"

## Steps
1. Remove the 4 packages from `dependencies` section
2. Add the 4 packages to `devDependencies` section (maintaining alphabetical order)
3. Verify the changes

## Expected Result
- `dependencies` will have 4 fewer entries
- `devDependencies` will have 4 more entries
- Package functionality unchanged (these are build-time dependencies)
