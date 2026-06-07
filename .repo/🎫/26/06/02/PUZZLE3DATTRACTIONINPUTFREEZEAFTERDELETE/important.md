# Root cause

Orbit and selection are gated while `attractionDragActive` or `attractionIndirectPickAwait` is set (`OrbitGated`, `MarqueeBridge`, object/vortex click handlers).

Starting an attraction drag sets `attractionSessionRef` and blocks the external `attractionSession` effect from clearing state. Deleting the involved object/vortex does not call `cancelAttractionDrag`, so the session stays active: frozen camera, no selection, hover still works.

Brush mode can still start a vortex drag (small movement on vortex), which makes the stuck state likely after brush + delete.

# Fix

- `AttractionStaleSessionGuard`: cancel when structure epoch changes and vortices are gone
- Escape always cancels attraction session
- No attraction drag while brush tool active; entering brush cancels any drag
