# Nested Interaction Call

**Status:** In progress (assets restored 2026-05-28)  
**Repo MCP:** ticket_reopen failed (missing ticket.json).

## Summary

General nested interaction mechanism:

- **`interaction.call` effect** — `interaction`, optional `inputs` (Expr map → child context), optional `outputs` (`PathTarget` + `Expr` bindings evaluated in child context), host pauses until child settles.
- **Rollback** — `rollback` snapshot captured before transition effects; abort restores it (cancels partial branch).
- **Resume** — on success, `mergeInteractionCallOutputs` + advance to `resumeTarget`, then normal host commit flow.
- **`invocation: "callable"`** — excluded from standalone host catalogs; `listSpatialInteractionsForModelDefinition` filters by default.
- **`InteractionRuntimeOptions.interactions`** + **`resolveInteractionSpecForCall`** — registry override for tests/custom hosts.
- **Deep nesting** — each runtime delegates `send`/`cancel`/`undo`/`redo`/`query`/`commit` to active child; snapshots delegate with `nested` chain (`outer`).
- **Shape hubs** — `surface.construct` / `curve.construct` callable interactions (sync via `bun ./📜️script.ts sync-shape-construct`); typology construct calls `surface.construct` with full output map.
- **Pick face** — inlined in `surface.construct` (`pick_face` state), no `pick.face` asset.
- **Output binding** — typology `surfaceConstructMode` maps from child `surfaceConstructMode` (not `constructMode`).

## Tests

- `@spatial/js-core`: **304/304**
- `@spatial/js-machine-stately`: **4/4**

## Files

- `spatial/js/core/index.ts`
- `spatial/js/core/shape-construct-codegen.ts`
- `spatial/js/core/typology-construct-codegen.ts`
- `spatial/js/machine-stately/index.ts`
- `spatial/assets/modelDefinition/spatial.shape/interaction/constructSurface.json`
- `spatial/assets/modelDefinition/spatial.shape/interaction/constructCurve.json`
