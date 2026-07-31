# Verification

## Automated

- `@semio-tech/framework-renderer-react:test` — 164 passed (includes `parsePuzzle3dCatalogueDragPayload` + `snapWorldPointToGrid`)
- `cargo test -p puzzle-plugin catalogue_drag` — blocked on shared `target/.cargo-lock` during session; tests added:
  - `kinds_tree_object_drag_data_carries_object_kind_and_mesh_url`
  - `add_object_kind_honors_drop_origin`

## Manual (Puzzle 3D play)

1. Drag object kind from catalogue onto viewport — ghost mesh follows cursor, grid-snaps when enabled
2. Drop commits object at cursor via `addObjectKind` + `origin`, selects new object
3. Pointer-palette drag (panel dims) also previews and commits on pointer-up over viewport
4. Click catalogue row without drag still places at origin
