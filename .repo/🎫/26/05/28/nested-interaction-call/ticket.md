# Nested Interaction Call

**Status:** Closed  
**Repo MCP:** unavailable in this session.

## Summary

Interactions can invoke other interactions with the `interaction.call` transition effect. The host statechart pauses (no target advance) until the child completes and commits or the user aborts (Escape). Events, selection accept, undo/redo, and cancel route to the active child. `getSnapshot()` surfaces the child with `nested: { hostInteractionId, hostState, hostContext }`.

Surface construct mode (`mode.surface`) assigns `constructMode` then calls shared `pick.face`; on success the host resumes at `committed` and runs its construct commit.

## Tests

- `@spatial/js-core`: **298/298**
- `@spatial/js-machine-stately`: **4/4**

## Files

- `spatial/js/core/index.ts`
- `spatial/js/core/typology-construct-codegen.ts`
- `spatial/js/machine-stately/index.ts`
- `spatial/assets/modelDefinition/spatial.shape/interaction/pickFace.json`
- `spatial/assets/modelDefinition/aec.building.*/interaction/construct*.json` (12 typology construct interactions regenerated)
