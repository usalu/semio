# Flow Wgpu Rich Rendering Parity

## Root causes

1. **Canvas theme not re-applied after host rebuild** — `sync_canvas_theme_dark` only called `set_canvas_theme_dark` when the dark/light flag changed. `sync_flow_host` can call `set_neuron_kind_infos_json` → `rebuild_dag()` on the same frame; theme must be pushed every paint frame (premigration `syncSessionCanvasTheme` ran every render).
2. **Operator sync order** — fixture was synced before operators; first-frame DAG could rebuild without neuron kind infos. Operators now sync first.
3. **`proximityDistance` dropped** — flow plugin serializes it in `lod_json` but wgpu `sync_flow_host` ignored it.
4. **Selection chrome missing** — premigration drew marquee + selection union bounds as DOM overlays (`SelectionMarquee`, `DagSelectionBoundsBox`); wgpu never painted them. Added `paint_node_graph_overlays`.

## Chrome rail audit

`render_window_content` receives the full dock body `content` rect before measures/engagement rails paint beside it (no shrink). Node-graph `inner` equals scene `bounds` — no zero-area regression from today's chrome work.

## Verification

- `flow_core::paint_scene_dark_theme_paints_edges_and_nodes`
- Rebuild wgpu wasm + boot `SEMIO_RENDERER=wgpu SEMIO_PLUGIN=flow`, screenshot default/zoom/selected/marquee states (see `wgpu-flow-parity-verify.mjs`).
