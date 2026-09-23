# Procedural 2D Catalogue Drag Audit

Source: catalogue audit on 2026-09-23.

## Roster

`semio_framework_os_flow::flow_palette_catalogue_sections()` is the full list. It is `flow_catalogue_sections()` (one section per installed flow extension, each item `kind: neuron`) plus three static sections: `inputs` (4), `outputs` (6), `contract` (2).

Generation2d already publishes that roster through `app_catalogue_json()`. The panel still hardcodes seven rows and uses `tree_item_with_action` with no drag payload.

## Drag payload

MIME `application/x-flow-widget`. The tree drag object is `{ "application/x-flow-widget": "<WidgetDescriptor JSON string>" }`.

Examples:

- `{"kind":"inputSlider"}`
- `{"kind":"neuron","neuronKind":"math.add"}`
- `{"kind":"outputExport","format":"svg"}`
- `{"kind":"outputAction","action":"log"}`

`node_graph_sync_flow_widget_ghost` passes that string to `FlowHost::set_ghost_widget` when the surface engine is `NodeGraphEngine::Flow`.

## Flow window

`generation2d` `windows/🕸️flow` `render` sets `host_snapshot_json` via `flow_backed_node_graph_extras`, so the wgpu canvas uses `FlowHost`. Ghost preview does not need a `setGhostWidget` app command.

## Drop gap

`node_graph_flow_widget_drop_action` emits `addWidget` with only `kind`, `neuronKind`, `x`, and `y`. It drops `format` and `action`.

`Generation2dCommand::AddWidget` has the same four fields. The handler builds a descriptor from `kind` and optional `neuronKind` only, so an export or action drop cannot select the catalogue variant.

## Tests

Keep the closed and opened show-mode tests. Replace the hardcoded sources=2 / components=3 / sinks=2 totals with palette group totals. Assert `application/x-flow-widget` and `"draggable":true` on component rows.

Click args stay `{ kind, neuronKind? }`, not generation3d's `neuron|math.add` pipe.
