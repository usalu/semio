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

- `@spatial/js-core`: `every typology ships construct kit or legacy create interactions` — pass
- `@spatial/js-core`: `construct dispatch action delegates to the selected create action` — pass
- `@spatial/js-core`: `typology actions reference shipped declarative action specs` — pass (added `command.finish` action asset for shape typologies)
- `@spatial/js-renderer-r3f`: 10/10 pass
- Full `@spatial/js-core` suite: 36 failures pre-existing (interaction e2e: arc, move, copy, …)

## Regenerate assets

```bash
cd spatial/js/core && bun ./script.ts sync-typology-construct
```
