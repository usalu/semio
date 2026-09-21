# Flow Content Child Publication Repair

## Failure evidence

`🗑️generated/astra-runtime/flow-native24-full/run.log` established the production boundary: `NodeGraphEdit` delete and Spotlight connect went through parent `FlowMutation` recipes, while `MoveMediaNode` changed the parent content handle to a cache-only child. The command result therefore either reached an unsupported parent recipe or named a child absent from the composed store during archive.

The detailed independent source audit is `📓️terra-flow-native24-six-failures-audit.md`.

## Repair

The existing `FlowChildGroupWork` route now owns `addWidget`, `moveMediaNode`, `nodeGraphEdit`, and `spotlightCommit`. Its publication contracts declare the Child lane for all four tools. The old direct-parent and graph-parent registrations no longer own those three migrated verbs.

`node_graph_edit_result` now reads the exact typed `SemioFlowSnapshot` from `ArtifactView.children`, executes the current strict graph vocabulary against a bounded live Flow host, derives the resulting child snapshot, and publishes one typed `SemioFlowMutation::SetSnapshot` through `ChildEmit` against the existing `snapshot.content.child_id`. A semantic no-op emits nothing. Any real result is checked as exactly one content child emit, with no parent/config/draft/effect/event publication. `MoveMediaNode` preserves its exact coalescing key.

Spotlight delegates to the same typed-child function. Move delegates through the same strict move operation rather than producing a parent `MoveWidgets` mutation.

The existing actual app laws now pin unchanged parent content identity for delete, Spotlight connect, node-graph move, and MoveMediaNode archive/load. The archive law continues to require the composed child store before export and exact round-trip equality.

## Source boundary

- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️spotlight-commit/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚚️move-media-node/🦀️.rs`
- corresponding node-graph and binary archive unit laws.

## Verification state

All six touched Rust files parse through `rustfmt --edition 2021 --emit stdout`. Source inventory confirms `moveMediaNode`, `nodeGraphEdit`, and `spotlightCommit` have Child publication contracts and no retained Artifact publication contract. No Cargo or native test command was run by this agent; root owns the serialized native lane.

Recommended focused filter:

```text
test(batched_delete_selection_clears_the_node_selection_on_the_scene) | test(spotlight_commit_shares_the_node_graph_edit_vocabulary) | test(node_graph_move_wire_publishes_the_requested_widget_layout) | test(flow_document_text_round_trips_store_with_applied_operation)
```

## Flow Native 25 follow-up

The full Flow 25 receipt at `🗑️generated/astra-runtime/flow-native25-full/run.log` ran 253 tests: 244 passed and 9 failed. The new child commands compiled and reached retained settlement. The three node-graph assertions and the move/archive assertion then read `FlowSnapshot::to_host_snapshot()`, whose working scene is explicitly a parent cache and does not observe the durable child publication. Their unchanged parent child IDs already passed, so repointing the parent or expanding its recipe would have reintroduced the invalid composition that this repair removes.

The laws now decode the exact `SemioFlowSnapshot` from `PluginApp::child_store("content", parent.content.child_id)` after terminal settlement. They assert delete, connect, and move semantics on that composed child; each actual command also pins the exact `[Child, Ui, Terminal]` receipt. The archive law observes the moved child before export, then retains the existing complete-document archive/load equality check.

The three retained-route failures were synchronization seams. The neutral `interactive-job` fixture now places `moveMediaNode`, `nodeGraphEdit`, and `spotlightCommit` in the `ChildGroup` with the Child lane; the native route probes and exact factory inventory expect those same four child routes. The two remaining Flow 25 failures, instance convergence and exact-window isolation, are outside this content-child route packet and remain independently owned.

The canonical schema/source oracle completed through Nx:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/flow-plugin:test-source
exit 0; Flow interactive-job oracle: 34 tool ids over 4 factories, batchOnly=0
```

Receipt: `🗑️generated/astra-runtime/flow-child-source/run.log`. This command executes the Flow language-neutral fixture through Ajv and its independent source oracle. An additional `nx exec` attempt for the narrower action-cohort command did not execute because the current workspace project graph reports an unrelated `framework-os-kernel`/`value-derive-rs` cycle; it provides no test evidence and is recorded at `🗑️generated/astra-runtime/flow-child-source/action-cohort.log`.

The next native gate should use:

```text
test(batched_delete_selection_clears_the_node_selection_on_the_scene) | test(spotlight_commit_shares_the_node_graph_edit_vocabulary) | test(node_graph_move_wire_publishes_the_requested_widget_layout) | test(flow_document_text_round_trips_store_with_applied_operation) | test(every_graph_operation_route_is_admitted_by_its_own_retained_factory) | test(retained_route_dispositions_are_exact_and_exhaustive) | test(retained_content_child_factory_is_exact_and_legacy_closed)
```

Flow Native 26 completed the full 253-test suite with 251 passing and 2 failing. All seven content-child publication, durable-child observation, route disposition, and factory inventory laws above passed. The remaining failures are the independently owned replication convergence and same-byte window-generation transient cases. Receipt: `🗑️generated/astra-runtime/flow-native26-full/run.log`.
