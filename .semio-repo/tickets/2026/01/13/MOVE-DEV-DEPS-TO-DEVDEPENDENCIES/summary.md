# Summary

Moved 4 development-only packages from `dependencies` to `devDependencies` in `js/semio/package.json`:

- `@types/d3-force` - TypeScript type definitions
- `@types/dagre` - TypeScript type definitions
- `postcss-import` - PostCSS build plugin
- `postcss-nesting` - PostCSS build plugin

These packages are only needed during development/build time, not at runtime.
