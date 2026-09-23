# Flow and Procedural Feature Complete

## Goal

`🎯r2603🎯runningnet🎯runninggrasshopper` — Running Grasshopper.

Repo MCP `repo://goals` was unavailable (`Server "repo" not found`). Goals were read from `.🧬semio/🦑️repo/🎯️goals`.

## Required end state

1. A flow that contains synapses paints those synapses on the node-graph canvas.
2. Every widget shows both sides: inputs on the left, outputs on the right. Sources show their output side. Sinks show their input side. Neurons show both.
3. The procedural 2D catalogue lists every component from `flow_palette_catalogue_sections`, each row has a drag-and-drop handle (`application/x-flow-widget`), and dragging over the flow canvas shows a live ghost preview.

## Known starting facts

- Procedural 2D catalogue (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/.../🛍️catalogue/🦀️.rs`) hardcodes seven items and uses `tree_item_with_action` with no drag payload.
- Procedural 3D catalogue already lists `flow_palette_catalogue_sections` but also has no drag payload.
- Flow catalogue already emits `application/x-flow-widget` via `tree_item_with_action_draggable`.
- `EngineCanvas` ghost preview (`node_graph_drag_ghost_descriptor`) accepts that MIME and calls `set_ghost_widget`.
- `widget_io_ports` returns no ports for preview, action, and export sinks, while `widget_to_dag_node` still stores one input on the node kind.
- Synapse edges are built in `FlowHost::build_dag_host_snapshot_v1` and dropped again in the DAG board when source or target handles are missing.

## Out of scope

Wave function collapse and other agents' tasks. Do not revert their edits. Do not use modifying git commands.
