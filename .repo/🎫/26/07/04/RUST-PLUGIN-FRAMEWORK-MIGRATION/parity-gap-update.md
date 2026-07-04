# S Parity Gap Update (2026-07-04)

## Pass 2 — FlowCanvas + WriterCanvas

### Restored packages (from `f8376e848`, play-host regions removed)
- `flow/react` — `FlowCanvas`, `FlowExtensionHost`, WASM GPU canvas
- `writer/react` — `WriterCanvas` with wire/jack LSP
- `mathematical/graph/port/directed/dag/react` — DAG LOD overlays for flow

### Framework renderer
- New `flow-canvas` component kind + `FlowCanvasHost` (ports old `SMediaGraphCanvas` behavior)
- `os-media-graph-flow.ts` — fixture→command bridge (`moveMediaNode`, `connectMediaPorts`, etc.)
- `TextEditorHost` uses `WriterCanvas` for `language: "wire"` (compiled DAG)
- Vitest stubs in ticket folder for WASM-heavy packages

### S plugin
- Media graph window now emits `flow-canvas` scene with `os_media_graph_to_flow_fixture` JSON

### Pass 1 — Shell
- Workbench left / details right panels, 40/30/30 layout, engagement rail, catalogue drag-drop, navbar chrome

### Pass 4 — Instance document sync on drill-in

- `materialize_os_app_instance_document_json` + fixture registry in `framework/product/os/core/rs/instance.rs`
- s plugin registers `semio.draw.json` / `jack.writer.json` fixtures; `openPluginInstance` carries `documentJson`
- `os-shell` applies `setDocument` to spawned plugin WASM instance before render

### Pass 5 — Interactive graph + compiled DAG + drill-in sync

- Graph mutations: `moveMediaNode`, `connectMediaPorts`, `disconnectMediaEdge`, `removeAppInstance`, `patchAppSource` in s plugin
- `spawnApp` routes to s-play (not shell `spawnProgram` intercept)
- Compiled DAG: `media_graph_to_dag_fixture` + `dag_fixture_to_wire_literal` wire DSL in WriterCanvas
- VFS double-click → `openInstance` / `exportMedia` / studio navigation
- Bidirectional drill-in: spawned `setDocument` → `patchAppSource` on studio document
- Parameters panel: min/max/step, categorical options; `addOption`/`removeOption` patch
- Footer toolbar: undo / redo / checkpoint for studio mode

## Still open (ProductShell chrome)
- Global search (Ctrl+P) and per-window find (Ctrl+F)
- Named layout save/load
- Display and settings side panels
- Browser History API URI sync (`popstate`)

## Tests
- `cargo test -p s-plugin`: 17 passed
- `@semio-tech/framework-renderer-react:test`: 9 passed
