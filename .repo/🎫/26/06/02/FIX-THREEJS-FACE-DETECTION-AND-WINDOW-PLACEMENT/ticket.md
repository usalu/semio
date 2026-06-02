# Fix Three.js Face Detection and Window Placement

**Goal:** R26-02/RUNNING-SKETCHPAD/RUNNING-SKETCHPAD-APPS  
**Client:** antigravity-chat  
**LLM:** gemini-3-pro  
**Due:** 2026-06-02  
**Status:** closed  

## Summary

Fixed three bugs in `main_mcp.js` (energy calculation MCP app sketchpad UI) that prevented correct wall face detection and window placement:

1. **World Normals Fix** — The raycaster returns face normals in local geometry space. Added `THREE.Matrix3().getNormalMatrix(object.matrixWorld)` to convert to world space before rounding for N/S/E/W mapping.

2. **Click Point for Window Placement** — `addWindowToSelectedWall` hardcoded `u: 0, v: 0`. Now computes `u`/`v` from `intersects[0].point.clone()` stored on `selectedWall.clickPoint`, placing windows exactly where the user clicked.

3. **All Stories Selectable** — Removed `if (s === 0)` guard so all story meshes get `userData` and are added to `draggableObjects`, enabling click/selection on upper floor walls.

## Files

- `coda/client/bin/assistant/sketchpad/main_mcp.js` — updated

---

## Follow-up: Wall Highlight and Window Editing (2026-06-02)

Fixed two additional bugs:

**Wall Highlight Not Showing Correctly:**
- Root cause: `render3DZones()` gizmo guard (`if (translateControl.object)`) was clearing `selectedWall = null` whenever any gizmo was attached, preventing the yellow highlight mesh from being drawn on subsequent re-renders.
- Fix: Split the guard — detach gizmo + null `selectedObject`/`selectedWindowId` only; never touch `selectedWall` in this guard.
- Additional fix: Highlight plane height now spans `storyHeight * numStories` (full building height) and centers at `fullHeight / 2` so it covers multi-story buildings correctly.

**Window Editing in Selection Panel:**
- Added `window.updateWindowProperty(zoneId, winId, prop, val)` function.
- `updateSelectionPanel()` window branch now shows four editable `<input type="number">` fields: Width, Height, Horizontal offset (u), Vertical offset (v).
- Each input has an `onchange` that calls `updateWindowProperty` → modifies data → calls `rebuildAndRetainSelection` to re-render.
- Wall label shown as "North/South/East/West" for clarity.
