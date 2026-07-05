# Generalize Rich UI into Framework Engines

Goal: `🎯r2602🎯runningsketchpad`

## Completed

### Contract (`framework/core/rs/ui.rs`)
- Extended `NodeGraphScene` with selection/hover/lod/catalogue/controls/clusters/computing/capabilities payloads
- Extended `TextEditorScene` with occurrences/placeholders/extra-carets/selectable-spans/settings/camera
- Added `node_graph_commands` and `text_editor_commands` modules
- Mirrored in `framework/renderer/react/types.ts`

### Engines
- **`framework/editor/rs`**: `EditorHost`/`EditorSession` (relocated from writer/rs), `syncFromSceneJson`
- **`framework/graph/rs`**: `GraphHost` wrapping `DagHost`, `GraphSession` wasm, scene payload sync
- **`writer/rs`**: thin re-export + `document_vcs.rs`

### React hosts
- `node-graph-host.tsx`: WASM graph surface + label/marquee/selection overlays + context menu + Diagram SSR fallback
- `text-editor-host.tsx`: WASM editor surface + diagnostics + `textEdit`/`textSelect` commands
- `graph-canvas-overlays.tsx`: dag label/marquee overlay utilities

### WGPU
- `scenes.rs`: selection/hover chrome from scene payload fields
- Compiles on `wasm32-unknown-unknown`

### Plugins
- **s**: emits selection/hover/context_menu; handles `nodeGraphSelect`/`nodeGraphHover`
- **writer**: jack lint/semantic_tokens via `trinity_jack`; `textEdit`/`textSelect` commands
- **flow**: capabilities/lod/context menu on node-graph scene

### Cleanup
- Deleted `flow/worker.ts`, `flow/worker-client.ts`
- Added workspace entries for `framework/editor/rs`, `framework/graph/rs`

### Verify
- `cargo test` framework_graph, framework_editor, writer-plugin, s-plugin
- React renderer vitest: 11 passed
- `bun install` clean
- wgpu renderer `cargo check --target wasm32-unknown-unknown` OK

## Lowpoly Editor Parity Restoration (2026-07-05)

Restored hierarchy tree, selection/hover, semantic theme colors, gumball centroid, picking overlays, and marquee selection to match the pre-migration TypeScript editor.

### Rust (`lowpoly/plugin/rs/lib.rs`)
- `merge_selection_ids`, `toggleSelectionTarget`, `setHover`, `worldPick` merge modes
- Nested Vertices/Edges/Faces hierarchy with selected/highlighted ids and face flip action
- `gumballTarget` via `selection_transform_pivot`; fixed transform commands to preserve selection
- Removed hardcoded instance colors; `selected`/`hovered` booleans only

### Framework UI schema
- `UiTreeItemNode`: `hover_command`, `unhover_command`, `actions` (`framework/core/rs/ui.rs`, `types.ts`, `ui-interpreter.tsx`)
- Fixed struct literals in forms, note, raster, vcs, puzzle5d plugins

### React renderer (`world-3d-host.tsx`)
- Semantic colors via `resolveSemanticColorHex` with theme observer
- Scene-level gumball at selection centroid; drag-end passes mode/ids
- Per-component face/edge/vertex overlays; vertex pick id fix; marquee dispatch

### Tests
- `cargo test -p lowpoly-plugin`: 17 passed
- `cargo test -p lowpoly_core`: 13 passed
- `@semio-tech/framework-renderer-react:test`: 11 passed
- React E2E lowpoly: 1/1 passed (`verify-react-playgrounds-e2e.ts --plugin lowpoly`)
