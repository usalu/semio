# Model Definition Runtime Rewrite

**Goal:** Remove legacy `spatial/assets/extension` implementation; load typologies, actions, and interactions from `spatial/assets/modelDefinition`; align runtime with `spatial/AGENTS.md`.

## Done

- `@spatial/js-core`: `import.meta.glob` catalogs under `spatial/assets/modelDefinition/**`; `ModelDefinitionManifest`; removed `ExtensionViewService` and energy/structure/building extension bundles.
- Selection commands are headless `selection.*` actions (no interaction JSON in assets).
- `ActionRegistry.run` dispatches kernel commands when no declarative spec is registered; `runCommit` no longer requires a pre-registered def.
- `entity.createAnchor` interaction asset + E2E; `ModelSpace` + vertex hashing.
- Attribute/property definition catalogs; explicit `primitiveKinds` on 67 typology assets; AEC typologies via `typology/*.json` glob.
- `derivePropertyValue` for `builtin.volume` and `energy.heatedvolume`; `buildTypologyToEntityKindMap` for construct MATCH.
- `@spatial/js-query`: typology validation, anchor/object index, view-derived rejection without legacy CALL view path.
- Tests: core 256+, query 33+, kernel-brepjs 19, renderer-r3f 8, machine-stately 4.

## Files

- `spatial/js/core/index.ts`
- `spatial/js/kernel-brepjs/index.ts`
- `spatial/js/query/index.ts`
- `spatial/js/renderer-r3f/index.tsx`
- `spatial/js/renderer-r3f/play/main.tsx`
- `spatial/js/machine-stately/index.ts` (comments only)

**Repo MCP:** unavailable (no `ticket_close`).
