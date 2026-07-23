# Media Graph App Drop Preview and Create

## Root cause (WGPU / native `s`)

Catalogue rows drag with MIME `application/x-semio-catalogue-item`, but:

1. `node_graph_sync_flow_widget_ghost` only accepted `application/x-flow-widget`, so every catalogue drag cleared the ghost (no preview).
2. `finish_tree_drag` only spawned on `HitKind::World3d` / `Window`. The media graph hit is `HitKind::ScrollRegion`, so drops never created nodes. Screen coords were also used instead of graph world coords.

React `FlowGraphCanvasHost` already implemented both behaviours; WGPU did not.

## Fix

In `framework/renderer/wgpu/rs/lib.rs` `engine_canvas`:

- Accept catalogue MIME for ghost preview (descriptor `{ kind: "neuron", neuronKind: label|appId }`).
- `node_graph_catalogue_drop_action` over `node_graph_states` bounds → `spawnApp` with surface controller + world position.
- `finish_tree_drag` calls that before giving up; remove the dead World3d/Window catalogue branch.
