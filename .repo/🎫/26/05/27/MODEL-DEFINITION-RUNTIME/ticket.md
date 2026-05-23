# Model Definition Runtime Rewrite

**Status:** Finished (2026-05-27).

**Goal:** Remove legacy `spatial/assets/extension` implementation; load typologies, actions, and interactions from `spatial/assets/modelDefinition`; align runtime with `spatial/AGENTS.md`.

## Done

- `@spatial/js-core`: `import.meta.glob` catalogs under `spatial/assets/modelDefinition/**`; `ModelDefinitionManifest`; removed `ExtensionViewService` and energy/structure/building extension bundles.
- Selection commands are headless `selection.*` actions (no interaction JSON in assets).
- `ActionRegistry.run` dispatches kernel commands when no declarative spec is registered; `runCommit` no longer requires a pre-registered def.
- `entity.createAnchor` interaction asset + E2E; `ModelSpace` + vertex hashing.
- Attribute/property definition catalogs; explicit `primitiveKinds` on 67 typology assets; AEC typologies via `typology/*.json` glob.
- `derivePropertyValue` for `builtin.volume` and `energy.heatedvolume`; `buildTypologyToEntityKindMap` for construct MATCH.
- `@spatial/js-query`: typology validation, anchor/object index, view-derived rejection without legacy CALL view path.
- Tests: core 264, query 34, kernel-brepjs 19, renderer-r3f 8, machine-stately 4.

## 2026-05-27 recovery

- Restored `spatial/js/core/index.ts`, `kernel-brepjs/index.ts`, `query/index.ts`, `renderer-r3f/index.tsx`, `machine-stately/index.ts` from commit `5036a2fcd` (PowerShell `>` had truncated `query`; use UTF-8 `bun -e` write).
- Fixed `machine-stately/script.ts` to lazy-import `index.ts` on `generate` only (Bun lacks `import.meta.glob` at script load).

## Declarative actions (2026-05-27)

- **47** action JSON files: `spatial.selection.apply` for selection commands; `spatial.action.capability` for geometry/command/measure/feature (no `spatial.action.execute` stubs).
- **6** transform action JSON files (`transform.move`, …) + typology `actions` aligned to interaction commit ids.
- `ActionRegistry.withBuiltins()` registers **only** `spatial.action/v1` specs (capabilities invoked via `kernel.call`, not duplicate `ActionDef.run`).
- Selection command metadata derived from model-definition typologies (`listSelectionOperationInteractionDefs`).
- `PathRoot` includes `params`; `readPathTarget` resolves action parameters in declarative steps.
- `propertyKind/*.json` included in property-definition catalog glob.
- Removed legacy `spatial.action.execute` from `executeKernelFunction` (only `spatial.action.capability` + `spatial.selection.apply`).
- Aligned measure typology: interaction id `measure.distance`; actions list only `measure.vertexDistance`.
- Machine-stately measure parity test uses `measure.distance`.
- Commit-action JSON + typology `actions` lists synced to on-disk `action/*.json` (26 typologies updated).

## Playground (`spatial/js/renderer-r3f/play`)

- Restored `core` + `machine-stately` when minified rewrite broke Vite (`executeBuiltinActionCapability`, `loadSpatialInteraction`).
- `bun ./script.ts build` and dev on http://127.0.0.1:6020/ verified; launch: `bun nx run @spatial/js-renderer-r3f:dev`.
- `play/main.tsx`: vertex count reads `geometry.vertices`; `play/vite.config.ts` aliases `@spatial/js-query`.

## Files

- `spatial/js/core/index.ts`
- `spatial/js/kernel-brepjs/index.ts`
- `spatial/js/query/index.ts`
- `spatial/js/renderer-r3f/index.tsx`
- `spatial/js/renderer-r3f/play/main.tsx`
- `spatial/js/machine-stately/index.ts`
- `spatial/js/machine-stately/script.ts`
- `spatial/assets/modelDefinition/**` (typologies, actions, interactions, transformations)
- `spatial/schema/json/*.json` (where updated)

## Final verification

| Package | Tests |
|---------|-------|
| `@spatial/js-core` | 264 |
| `@spatial/js-query` | 34 |
| `@spatial/js-kernel-brepjs` | 19 |
| `@spatial/js-machine-stately` | 4 |
| `@spatial/js-renderer-r3f` | 8 |
| Playground `bun ./script.ts build` | OK |

**Repo MCP:** unavailable (no `ticket_close`).
