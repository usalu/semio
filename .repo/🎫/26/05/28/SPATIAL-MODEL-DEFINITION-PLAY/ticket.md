# Spatial Model Definition Play

**Repo MCP:** unavailable in this session (`ticket_open` / `ticket_close` / `repo://goals` not registered).

## Summary

Replaced the legacy view chrome with model-definition selection and transformation dropdowns so play matches the new spatial spec (model definitions + `from_*` / `to_*` transformations, no views).

## Changes

- **Core:** `GEOMETRY_MODEL_DEFINITION_ID`, `isGeometryModelDefinition`, manifest parsing for `spatial.modelDefinition/v1`, `listTransformationsIntoModelDefinition` / `listTransformationsFromModelDefinition`, removed `views` from runtime/query/effect paths, renamed `activeViewId` → `activeModelDefinitionId`, `selectionOperationUsesModelObjects`.
- **Renderer:** InteractionRepl aside now lists model definitions, transform-from (incoming) and transform-to (outgoing) selectors; picking/scene visibility keyed off model definition instead of view id; `onApplyTransformation` callback.
- **Play:** Wired controlled model definition state, transformation apply overwrites model + switches target definition; renamed Save (View) → Save (Model).
- **Query / machine-stately / kernel-brepjs:** Thread `activeModelDefinitionId` through state engine send and construct query env (no views).

## Tests

- `@spatial/js-renderer-r3f` index vitest: **8/8 passed**
- `@spatial/js-query` index vitest: **34/34 passed**
- `@spatial/js-core`: transformation + manifest tests pass; other failures pre-exist in ongoing spatial refactor (typology catalog counts, transform.move interaction ids, etc.)

## Files

- `spatial/js/core/index.ts`
- `spatial/js/renderer-r3f/index.tsx`
- `spatial/js/renderer-r3f/play/main.tsx`
- `spatial/js/query/index.ts`
- `spatial/js/machine-stately/index.ts`
- `spatial/js/kernel-brepjs/index.ts`
