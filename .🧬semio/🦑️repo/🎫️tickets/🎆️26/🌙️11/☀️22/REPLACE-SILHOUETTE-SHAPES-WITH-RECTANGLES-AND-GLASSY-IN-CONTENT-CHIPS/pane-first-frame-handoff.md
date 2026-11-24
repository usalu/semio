# Pane first-frame layout: decided from the window kind

## Cause
`pane_document_rect` was fed `engine_host_attached = surfaces.engine_host_id_for_window(..).is_some()`. The host registers only after the document has painted (`sync_engine_surface_states`), so the first frame of a new engine-host window (world3d, node graph, tiled map, board2d) was laid out with chip clearance and became full-bleed a frame later: a visible one-frame jump.

## Change
- Dock crate (`🛰️Dock/🎯️targets/🧊️wgpu/🔲️window-frame-and-stack-geometry/🦀️.rs`): new `surface_kind_is_engine_host(Option<&SurfaceKind>)` (World3d, NodeGraph, TiledMap, Board2d) and `kind_pane_document_rect(window_rect, frame, kind)`, re-exported from the dock crate root (`🎯️targets/🧊️wgpu/🦀️.rs`).
- Shell phase 4 (`🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🪀️view-panels/🖼️chrome-frame-and-main-window/🦀️.rs`): the late attached flag is replaced by the manifest kind, `docs.window_document_session(..)` then `dock.window_kind_of(..).surface_kind`, passed to `kind_pane_document_rect`. An undeclared kind is document-like.
- `🪀️shell-view-panels/🦀️.rs`: import now `kind_pane_document_rect`.
- Test (next to the clearance test, `🛰️Dock/🧪️tests/🔬️wgpu-unit/🪟️window-instances-geometry-and-frame/🦀️.rs`): `first_laid_out_pane_rect_follows_the_declared_window_kind_not_the_late_host_registration`. Engine-host kinds yield the full rect, document kinds and unknown the clearance rect, no frame the full rect.

## Verified (repo cargo, prototype, shared store)
- `semio-renderer-wgpu-dock`: 2 passed.
- `semio-renderer-wgpu-dock-laws`: 77 passed (76 + new test).
- `semio-renderer-wgpu-shell-view-chrome`: 6 passed; `shell-view-panels` compiles (0 tests). The chrome lib-test needs `--features semio-renderer-wgpu-shell-view-chrome/testkit` when run without the shell-tests crates. Earlier runs failed on other agents' in-flight edits (plugin-document-media, plugin-instance-app), not on this change.
- Not run: shell-tests crates (fixtures owned by another agent).
