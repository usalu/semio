# Generation2d Canvas Edits Apply

Gap inventory items **1, 2, 5, 8** for making generation2d node-graph canvas edits actually apply (mirroring generation3d / flow main).

## What changed

1. **`node-graph-edit`** — handles `move`, `disconnect`, and `setSlider` (plus existing connect / deleteSelection). Document-level `disconnect_synapse` fallback when host kinds are unregistered.
2. **`windows/flow`** — stamps `NodeGraphInteractionDomain`, catalogue operator records, and live selection onto the node-graph scene the same way flow’s main window does.
3. **`editor` context menu** — `context_menu_with_request_context` builds delete-selection from `GraphInteractionMarks` / surface selection instead of an empty `Vec`.
4. **Delete route** — `nodeGraphEdit` `apply` reads `interaction.selection("graph")` so `deleteSelection` from the menu sees the live selection.

## Test helper fix (this pass)

`select_graph` in the editor unit tests was settling with `settle_registered_typed_operation`. `interactionSelect` is a reserved framework job; a bare admit leaves `InteractionState` empty, so the context-menu selection law still saw no delete row. Switched to `settle_framework_reserved_admission` (generation3d twin).

## Unrelated compile unblock

Catalogue panel unit test `component_rows_drag_the_flow_widget_descriptor` shadowed `app()` with a live binding, then called `app()` again — scoped the first fixture so the crate tests compile. Catalogue feature ownership stays with the other agent.

## Commands and results

```text
cargo test --manifest-path Cargo.toml -p semio-s-artifact-procedural-generation2d \
  --features component-app-assembly --lib node_graph_edit -- --nocapture
→ ok. 4 passed (move, disconnect/connect, setSlider ×2)

cargo test --manifest-path Cargo.toml -p semio-s-artifact-procedural-generation2d \
  --features component-app-assembly --lib 'windows::flow::tests' -- --nocapture
→ ok. 5 passed (definition, operators, selection paint, render, fields)

cargo test --manifest-path Cargo.toml -p semio-s-artifact-procedural-generation2d \
  --features component-app-assembly --lib context_menu_reads_the_framework_owned_graph_selection -- --nocapture
→ ok. 1 passed
```

## Out of scope (this pass)

- txt import/export
- keyboard navigation
- catalogue panel feature work, add-widget command, `FlowHost::build_dag_host_snapshot_v1`, DAG board column layout, WFC
