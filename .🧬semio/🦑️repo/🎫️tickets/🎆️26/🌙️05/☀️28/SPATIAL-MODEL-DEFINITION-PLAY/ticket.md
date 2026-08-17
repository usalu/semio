# Spatial Model Definition Play

**Status:** Closed  
**Repo MCP:** unavailable in this session (`ticket_open` / `ticket_close` / `repo://goals` not registered).

## Summary

When a non-builtin model definition is active, the spatial stack scopes selection/filter kinds, interactions, actions, selection commands, attributes, properties, construct MATCH/CALL, and runtime action execution to that definition's asset catalog.

## Scope layer (core)

- `resolveModelDefinitionScope`, `list*ForModelDefinition`, `modelDefinitionSelectionEntityKinds`, `modelDefinitionUsesGeometryPicking`
- `buildTypologyToEntityKindMapForModelDefinition` — AEC typologies map to `object` for construct
- `listApplicablePropertyDefinitionsForModelDefinition`
- `assertActionAvailableInModelDefinition` — single guard used by `ActionRegistry.run`, `applyEffectAsync`, `runSelectionOperationInteraction`
- Selection operations built from action assets per definition folder

## Runtime enforcement

- **ActionRegistry.run** — rejects out-of-scope actions (geometry actions under energy, etc.)
- **applyEffectAsync** — guards transition effect actions + passes `activeModelDefinitionId`
- **runSelectionOperationInteraction** — scoped lookup + guard
- **Construct query** — parse-time typology validation per MD; MATCH seeds object rows by `typologyId`; CALL guarded

## Renderer / Play

- Pick targets: topology vs typology objects filtered by MD entity kinds + typology ownership
- REPL: scoped interactions, actions, selection commands; selection command buttons in aside
- Selection accept defaults to MD entity kinds; pick-kind toggle dimming uses mapped accept set
- MD switch resets toggles, selection, interaction, `lastFinalizedInteractionId`
- `SelectionAttributesPanel` + `SelectionPropertiesPanel` in play aside (scoped defs)
- Play: `scopedInteractions`, guarded interaction load, construct panel seeds from active MD typologies

## Kernel

- STEP export derives properties via `listApplicablePropertyDefinitionsForModelDefinition(modelId, …)`

- `interactions` prop removed from `InteractionRepl` / `PlaySession` (scoped list resolved inside REPL from `activeModelDefinitionId`)

## Machine-stately

- `buildSpatialStatelyMachineCatalogView` requires `{ modelDefinitionId }`
- `script.ts generate` passes `--model-definition` (default `builtin`)

## Tests (verified)

- `@spatial/js-query`: **42/42**
- `@spatial/js-renderer-r3f`: **10/10**
- `@spatial/js-machine-stately`: **4/4**
- `@spatial/js-kernel-brepjs`: **22/22**
- `@spatial/js-core`: **286/286**

## Cleanup (final pass)

- `listActionsForModelDefinition` includes action ids referenced by owned interaction specs (transition effects + commit)
- Shipped `command.*` action assets (`addPoint`, `addSelection`, `selectionBboxCenter`, `constrainMoveCursor`, `undoPick`) under `spatial.shape`
- `command.undoPick` capability implementation
- `SpatialInteraction` interface hoisted into `ModelDefinitionScope` region
- Catalog + transform interaction tests aligned with current assets and object-first move/copy workflow

## Legacy removed

- Unscoped: `listSpatialInteractions`, `resolveSpatialInteractionKey`, `buildTypologyToEntityKindMap`, `listApplicablePropertyDefinitions`, `listSelectionOperationInteractionDefs`, `runViewDerivedCall`, view-derived construct CALL
- `InteractionRepl` no longer accepts host `interactions` list

## Files

- `spatial/js/core/index.ts`
- `spatial/js/query/index.ts`
- `spatial/js/renderer-r3f/index.tsx`
- `spatial/js/renderer-r3f/play/main.tsx`
- `spatial/js/kernel-brepjs/index.ts`
- `spatial/js/machine-stately/index.ts`
- `spatial/js/machine-stately/script.ts`
