# Flow React/WGPU Producer Audit

## Scope and correction

This supersedes the earlier version of this report. That version incorrectly treated `generation2d` as the owner of the captured Flow run. The active target is Flow itself:

- `✏️s/🔌️plugins/🌊️flow/🔣️.json:11-16` registers `s.flow.flow@1/*#editor`.
- The same manifest maps it to `flow-main` and `flow.play.main` at `:303-315`.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️.rs:16-18` owns exactly those identifiers.

This matches the capture target (`s.flow.flow@1/*#editor`, `flow-main`, `flow.play.main`). No conclusion below relies on Generation2d.

## Observed capture

`🗑️generated/astra-runtime/flow-react-4/react/results.json` records five probes: cancellation, wheel and pan pass; selection publishes `[]`; a completed drag visibly moves the node while the next scene reports its old world coordinate. The host applies the action. Therefore the failure is after WGPU's physical gesture handling, at Flow's action/projection boundary.

## Confirmed causes

### Move is dropped at Flow's typed graph-operation bridge

The WGPU producer serializes a drag as `nodeGraphEdit` with `{ operation: "move", nodeId, x, y }` in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:3651-3697`.

The active Flow enum at `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs:16-23` accepts only `set-snapshot`, `delete-selection`, and `connect`. It has no `move` or `disconnect` variant.

`FlowPlayApp::command_from_action` parses the operation array with `filter_map(T::from_value(...).ok())` at `…/✏️editor/🦀️.rs:2389-2419`. Consequently the WGPU `move` row is silently removed and the admitted `NodeGraphEdit` reaches `node_graph_edit::apply` (`…/✏️editor/🦀️.rs:2490-2494`) with no operation. The separately implemented `moveMediaNode` command does mutate `host.move_widget` (`🎮️commands/🚚️move-media-node/🦀️.rs:18-26`), but WGPU does not dispatch that command.

**Confidence: high.** This is a direct producer/consumer vocabulary mismatch and independently explains a completed physical drag followed by unchanged persisted layout.

### Selection is explicitly omitted, and emitted ids do not match Flow topology ids

The active Flow main renderer writes `let selection: Vec<String> = Vec::new()` and inserts it into the scene at `…/🪟️windows/🌊️main/🦀️.rs:91-111`. Its request-context renderer receives `_interaction` and does not pass it down at `…/✏️editor/🦀️.rs:2572-2603`. That directly explains `published: []`.

A direct projection of the existing interaction ids would still be wrong. Flow’s `graph` topology registers node ids as `flow-play-document.widget.{widgetId}` at `…/✏️editor/🦀️.rs:151-175,2510-2517`; its command routes reverse that prefix with `flow_graph_selection_domains`. WGPU receives selected raw host node ids and emits them unchanged: it gathers `host.dag.selected_node_ids()` at EngineCanvas `:3632-3647`, then serializes every id as a node interaction target at `:3887-3892`. Thus raw `slider` is pruned against Flow’s topology rather than becoming `flow-play-document.widget.slider`.

**Confidence: high.** The public empty selection has two independent current-source causes: the render-time literal empty vector and incompatible raw WGPU target ids.

### `hostSnapshotJson`, not `NodeGraphScene.nodes`, is the active WGPU geometry authority

Flow main builds `nodes` from `with_host_from_snapshot` but builds `host_snapshot_json` from `with_live_host_snapshot`, both ultimately from `snapshot.to_host_snapshot` (`…/🪟️windows/🌊️main/🦀️.rs:85-111`; `…/✏️editor/🦀️.rs:2646-2674`). The WGPU node-graph engine selects `FlowHost` whenever `host_snapshot_json` exists and parses it as its initial source (`EngineCanvas:2110-2127`); on a new scene value it resynchronizes the live engine from precisely that JSON (`:2194-2200`).

The scene `nodes` array is therefore a mirror for Flow, not the WGPU draw geometry source. A move regression must assert the parsed `hostSnapshotJson` position (and may additionally assert the nodes mirror) after action settlement.

**Confidence: high.** This is direct control flow, not an inference from a read-back mismatch.

## Connection/delete audit

The main scene exposes only visible input/output ports from the same host snapshot (`main/🦀️.rs:55-80`). The default Flow topology includes both widget and synapse targets (`✏️editor/🧪️tests/🔬️unit/🦀️.rs:481-494`).

- `deleteSelection` correctly maps selected topology ids, syncs host selection, and deletes it at `🎮️commands/🗑️delete-selection/🦀️.rs:29-38`; its established law selects `slider`, observes a mutation, then observes slider removal at `…/🧪️tests/🔬️unit/🦀️.rs:5-12`.
- Direct `connectMediaPorts` has a four-identifier payload and calls `host.connect_ports` at `🎮️commands/🔗️connect-media-ports/🦀️.rs:13-28`.
- The WGPU batch vocabulary emits `connect`, `disconnect`, and `move` (`EngineCanvas:3660-3691`), while the active `FlowNodeGraphEditOp` accepts only `connect`. `disconnect` is therefore a second confirmed missing operation in that batch route. It has a direct standalone `disconnect` command, but that does not consume WGPU `nodeGraphEdit` rows.

The full-LOD connection law should use a schema fixture with two visible compatible ports and assert the edge endpoint ids in parsed `hostSnapshotJson`; it must not infer a route from the React TypeScript type twin, which contains only view-model constants (`…/🪟️windows/🌊️main/🟦️.ts:1-24`).

## Bounded repair assignment and fail-first laws

Owner: the active Flow editor package under `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.

1. Extend the schema-derived `FlowNodeGraphEditOp` vocabulary with Move and Disconnect, then make malformed operation input reject at the action boundary rather than disappearing in `filter_map`. Preserve the one retained `nodeGraphEdit` route and its artifact publication contract (`✏️editor/🦀️.rs:2024-2035`).
2. At the WGPU/Flow boundary, qualify host node and edge ids to Flow’s topology ids before emitting generic interaction verbs, or publish a topology whose ids are raw host ids. The former preserves Flow’s existing command/domain contract.
3. Thread the live interaction view into Flow main render and publish its selected node ids in scene `selection`, after converting topology ids back to the scene/host ids if that field’s contract demands raw ids.

The minimal production law homes are:

- operation vocabulary and persisted move/disconnect: `🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs`;
- Flow main public scene selection/layout projection: `🎭️modes/✏️edit/🪟️windows/🌊️main/🧪️tests/🔬️unit/🦀️.rs`;
- end-to-end action bridge/registered-app fixture: `🧪️tests/🔬️unit/🦀️.rs`, whose `flow_app_with_registry` and `select_graph` helpers are at `:89-114,188-200`.

Schema-first fixture laws:

1. A `nodeGraphEdit` fixture with `move { nodeId, x, y }` must decode to a nonempty typed operation, emit one artifact mutation, and change that node’s parsed `hostSnapshotJson` coordinates exactly.
2. A fixture with `disconnect { synapseId }` must remove exactly that edge; a fixture with `connect` must create exactly the requested endpoint pair from visible compatible ports.
3. A selection fixture must send WGPU/raw `slider`, assert the bridged interaction target is `flow-play-document.widget.slider`, then assert public scene selection has the expected presentation id and `deleteSelection` removes the same widget.
4. A cancelled gesture fixture must publish no move operation and leave the host snapshot layout byte-for-byte unchanged. Existing physical cancellation is green; the new law protects the Flow producer boundary.

No native build or runtime was run for this read-only audit.
