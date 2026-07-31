# Typology Construct Kit

**Repo MCP:** unavailable (`ticket_close` not registered in session).

## Summary

Every AEC/FEM typology that lacked create flows now ships three declarative create actions (2 points + height, curve + height, surface) plus one `construct*` interaction that dispatches to the correct create action via `constructMode`. Wall/column typologies also reference existing `building.wall.*` / `building.mushroomcolumn.*` actions. `spatial.shape` typologies keep legacy interactions (`primitive.box`, `curve.line`, …).

## Pattern (example: External Wall)

- `energy.energy.createExternalWallFrom2PointsAndHeight`
- `energy.energy.createExternalWallFromCurveAndHeight`
- `energy.energy.createExternalWallFromSurface`
- `energy.energy.constructExternalWall` (interaction + dispatch action)

## Code

- `spatial/js/core/typology-construct-codegen.ts` — naming + interaction template
- `spatial/js/core/index.ts` — `typologyConstructRoutes`, dispatch in `executeBuiltinActionCapability`, catalog tests
- `spatial/js/core/sync-typology-construct.ts` — asset generator
- `spatial/js/core/script.ts` — `sync-typology-construct` command

## Assets (12 typologies × 5 files)

- `aec.building.energy` (5 typologies)
- `aec.building.structure.classic` (4)
- `aec.building.structure.fem.line|surface|solid` (3)

## Tests

- `@spatial/js-core`: **286/286 pass** (interaction e2e, transform move/copy, typology construct kit)
- `@spatial/js-kernel-brepjs`: typology `createFrom2PointsAndHeight` handler — pass
- `@spatial/js-renderer-r3f`: **10/10 pass**

## Follow-up fixes (continue)

- `actionAvailableInModelDefinition`: allow `command.*` infrastructure actions during interactions
- `InteractionRuntime.send(start)`: run `start` transition before `consumeStartSelection` so pre-selected targets work
- `ActionRegistry.withBuiltins`: register capability-only `command.*` actions with declarative specs
- `executeBuiltinActionCapability`: pass `model` into `executeCommandDiff` for wire/face create modes
- `BrepjsKernel.executeCommandDiff`: `*From2PointsAndHeight`, `*FromCurveAndHeight`, `*FromSurface` handlers
- `spatial.shape/action/finish.json` for `command.finish` typology references

## Regenerate assets

```bash
cd spatial/js/core && bun ./📜️script.ts sync-typology-construct
```
