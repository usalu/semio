# Model Definition Runtime Rewrite

**Goal:** Remove legacy `spatial/assets/extension` implementation; load typologies, actions, and interactions from `spatial/assets/modelDefinition`; align runtime with `spatial/AGENTS.md`.

## Done

- `@spatial/js-core`: `import.meta.glob` catalogs under `spatial/assets/modelDefinition/**`; `ModelDefinitionManifest`; removed `ExtensionViewService` and energy/structure/building extension bundles.
- Selection commands are headless `selection.*` actions (no interaction JSON in assets).
- `ActionRegistry.run` dispatches kernel commands when no declarative spec is registered; `runCommit` no longer requires a pre-registered def.
- Tests: core 240, kernel-brepjs 19, query 32, renderer-r3f 8, machine-stately 4.

## Files

- `spatial/js/core/index.ts`
- `spatial/js/kernel-brepjs/index.ts`
- `spatial/js/query/index.ts`
- `spatial/js/renderer-r3f/index.tsx`
- `spatial/js/renderer-r3f/play/main.tsx`
- `spatial/js/machine-stately/index.ts` (comments only)

**Repo MCP:** unavailable (no `ticket_close`).
