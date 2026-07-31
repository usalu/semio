# Show Puzzle 3D Suggested Objects as Preview

## Root causes

1. **Plugin gate (sibling ticket `PUZZLE-3D-CONTEXT-MENU-SUGGEST-STAYS-OFF-BRUSH`)**: one-shot context-menu suggestions no longer switch into brush mode. Preview must therefore be emitted whenever `suggestion_menu` is open, not only when `active_utility == "brush"`. Already fixed in `world_brush_preview_json`.

2. **React ghost mesh URL**: `BrushPreviewGhost` only rendered a GLB when the preview `meshUrl` was already present in the scene's `meshes` list. Catalogue-drop ghosts load `meshUrl` directly; suggestion ghosts did not — unplaced kinds fell back to a unit box (and with `frameloop="demand"` often never painted).

3. **Demand frameloop**: mounting the ghost without an invalidate left it invisible until the next orbit tick.

4. **Collision mesh registration**: brush mesh registrars only covered placed-object URLs. Preview kinds not yet in the scene stayed `unknownPending`, so the suggestion list never left "Checking placement…" and never emitted a preview.

5. **HostEffect utility ref sync** (already fixed in parallel): `applyHostEffects` must call `setActiveUtilityForWindow` so `refreshUi` sees the map before the next React render.

## Fix

- `brushPreviewGhostMeshUrl` + `BrushPreviewGhost` load `meshUrl` directly (catalogue-drop parity), translucent opacity, demand invalidate.
- Include `brushPreview.meshUrl` in `brushMeshUrls` for collision registration.
- Vitest coverage for ghost mesh URL resolution.
