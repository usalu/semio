# FlowReact4 Current App Authority Audit

## Verdict

The historical FlowReact4 selection and persisted-move failures do not describe the current Flow controller path. They were valid evidence for the older producer and projection boundary, but the current source has the two missing contracts in the real app s.flow.flow@1/*#editor. This audit did not run a browser or native test, so it does not claim the fresh physical journey passes.

The next decision is a fresh activation against the current seven-case Flow fixture, rather than a new renderer repair. A failure from that run would be new evidence.

## Exact current controller and receipt route

The registered controller is s.flow.flow@1/*#editor in the Flow plugin manifest. Its main window is flow-main and its surface body is flow.play.main, declared in the Flow main-window renderer.

For a WGPU drag, EngineCanvas stages one bounded action batch. It derives edits from the pointer plan, writes one nodeGraphEdit row before committing the local plan, and publishes the batch atomically. The current row writer emits move with nodeId, x, and y. Graph selection is also emitted in the same route, with every raw node id qualified through the current scene interaction domain. For Flow that domain is graph with the flow-play-document.widget. prefix.

This exactly matches React's selection helper, which prefixes node ids with the supplied interaction-domain node target prefix. React's Flow canvas commits a changed WASM graph as setHostSnapshot on pointer-up; its ordinary React Flow host instead emits the same narrow move row on node-drag stop. The Flow app must therefore accept both shapes.

It does. FlowNodeGraphEditOp now has SetHostSnapshot, DeleteSelection, Connect, Disconnect, and Move. operations_from_action uses one strict whole-array decoder: unknown or malformed rows reject the batch instead of disappearing. The retained child-group route passes FlowCommand::NodeGraphEdit to node_graph_edit_result. Move calls host.move_widget, produces a content-child mutation, and asks for full UI invalidation.

The next Flow render reads the graph InteractionView, strips Flow's scoped topology prefixes to raw widget ids, and passes that selection into the main scene. The main scene publishes that raw selection and its hostSnapshotJson. The same app's topology registers the scoped widget ids, so WGPU's qualified interactionSelect values are retained rather than pruned.

## Why FlowReact4 is stale calibration

The FlowReact4 report records empty selection and unchanged persisted coordinates. Its own producer audit identifies an old parser that accepted neither move nor disconnect and silently filtered failed rows, plus an old main renderer that always sent an empty selection. Neither condition exists in the current code:

- Flow's parser now has closed move and disconnect cases, and one malformed row refuses the whole batch.
- Flow's request-context render reads InteractionView graph selection and main::render publishes it.
- EngineCanvas emits the same scoped selection target that React emits.
- The Flow retained child route persists move through the existing content child; it is not the obsolete parent-only route.

The former five-probe result also does not represent the current physical fixture. The current ticket fixture has seven ordered cases: select-and-escape, connect, disconnect, cancelled drag, committed drag, wheel zoom, and middle pan. It names the same current controller/window/surface and uses add as the interaction specimen.

## Existing coverage and its boundary

The Flow node-graph-edit unit test is app-backed. Its node_graph_move_wire_publishes_the_requested_widget_layout law sends nodeGraphEdit move to a real Flow app, waits for a Child, Ui, Terminal typed-operation receipt, verifies the content child's add position, renders Flow main again, and checks hostSnapshotJson.layout.add. This is strong evidence for the current Flow action-to-persisted-scene route, but it was not executed during this audit.

The EngineCanvas node-graph gesture laws cover the current WGPU producer. They prove the move action shape and the scoped selection payload, but their standard fixture uses a generic FlowHost or the generation3d controller. As SolTree noted, that is not an app-backed s.flow.flow@1/*#editor receipt.

The current ticket physical adapter is the appropriate cross-implementation acceptance home. Its React side waits for Flow's actual republished host snapshot on connect/disconnect and uses the Flow probe's world coordinates for a committed drag. Its WGPU side verifies the staged action row and local visible geometry, but cannot presently read a settled Flow owner host snapshot. It must not claim WGPU persistence from the local host movement alone.

## Required fresh acceptance

1. Run the current surface-interactions script contract and a fresh React run against its declared current Flow URL. Require selection to publish add, Escape to clear it, and committed drag to change Flow's republished hostSnapshotJson layout for add.
2. Run the same WGPU cases only as renderer-action and visible-geometry evidence until the WGPU page exposes a settled Flow owner snapshot/receipt.
3. Add one native bridge law by composing the existing EngineCanvas move producer with the existing Flow app fixture: inject its captured nodeGraphEdit action into s.flow.flow@1/*#editor, settle the exact typed operation, then render Flow main and assert hostSnapshotJson.layout.add. Use the scoped selection action in the same sequence and assert the next scene selection is raw add. That closes the producer-to-app gap without treating generation3d as the Flow owner.
4. Preserve current cancellation semantics: Escape must publish no nodeGraphEdit move and the fresh release cannot mutate the persisted Flow content child.

No source or fixture was edited. No test command was run.

