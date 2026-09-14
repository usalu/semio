# Flow window artifact tree relocation

## Problem

The procedural 3d **Flow** window rendered a side-by-side row: graph outline tree + node-graph canvas. That tree belongs on the **Artifact** panel (`framework.panel.artifact` / `procedural.play.document`), not inside the window chrome.

## Fix

1. **Flow window** (`…/🕸️flow/🦀️.rs`): body is the node-graph surface only (full-size column).
2. **Artifact panel** (`…/📌️panels/🗿️artifact/🦀️.rs`): renders `graph_outline` (nodes, nested ports, wires) with live eval status from `flow_backed_node_graph_extras`.
3. Language-agnostic law fixture for the document panel updated to match the graph-outline contract.
4. Canvas hint copy points users to the Artifact panel for the semantic tree.

## Tests

- `main_body_is_canvas_only_without_an_embedded_artifact_tree` (flow window unit)
- `document_rows_bind_both_framework_interaction_verbs` (artifact panel unit, graph-outline law)
- Existing `flow_graph_outline_*` tests still exercise `graph_outline` in isolation
