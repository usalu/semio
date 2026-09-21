# Flow Node Graph Move and Selection Parity

## Scope

This packet repairs the active `s.flow.flow@1/*#editor` application. The captured window is `flow-main`, whose body key is `flow.play.main`. Generation2d is outside this packet.

The first bounded slice covers two published defects:

- WGPU emits a `nodeGraphEdit` row shaped as `{ "operation": "move", "nodeId": "add", "x": 284, "y": 48 }`, while Flow's closed operation enum has no move variant and its action bridge silently filters failed rows.
- Flow's request-context renderer ignores the framework interaction view and always publishes an empty `NodeGraphScene.selection`.

The generic NodeGraph producer also emits raw node, edge, and handle identifiers. Flow's declared interaction topology uses the scoped prefixes `flow-play-document.widget.`, `flow-play-document.synapse.`, and `flow-play-document.handle.`. The application cannot fix producer ids after dispatch, so that part is a separate shared scene/renderer slice held until the renderer checkpoint is sealed.

## Existing admission authority

The graph-operation retained route already declares two Flow-local limits: `FLOW_STORE_MAX_MUTATION_ITEMS` admits at most 256 `nodeGraphEdit` or `spotlightCommit` rows, and `FLOW_GRAPH_OPERATION_RAW_BYTES` admits at most 16,384 encoded bytes. `flow_graph_operation_payload_admitted` applies the row limit from retained-work `extent` and `step`; `ArtifactRetainedCommandPayload::try_new` and the route factory apply the 16,384-byte wire limit. The framework's general job owner permits 256 pages of 16 KiB (4 MiB), but that is a broader owner ceiling and is not the Flow action parser's limit.

`handle_action` calls `operations_from_action` while constructing `FlowCommand`, before those retained-route checks. The strict parser therefore still walks every supplied DSL row and fully parses a `setHostSnapshot.hostSnapshotJson` string before the established 256-row and 16,384-byte authorities run. This packet does not introduce a second arbitrary cap during checkpoint capture; the pre-admission traversal remains a separately identified authority gap.

Wire refusal and host semantics are separate. Unknown operation tags, missing or extra fields, invalid identifiers, non-finite coordinates, and invalid host-snapshot encoding refuse the complete command before admission. A structurally valid `connect`, `disconnect`, `move`, or `deleteSelection` can still name an absent or incompatible domain target; the Flow host decides that semantic result, and the current batched executor treats an individual host error as no mutation rather than reclassifying it as a wire-format fault.

## Fail-first laws

The Flow artifact now has laws that:

- dispatch the exact WGPU move wire through `handle_action`, settle the registered retained operation, and require the `add` layout entry to become `(284, 48)`;
- require one unknown operation row to reject the complete action instead of producing an empty successful edit;
- render after a real framework `interactionSelect` and require the main scene to project the scoped widget target back to raw scene selection `add`;
- require the main scene to publish the complete graph interaction address with node, edge, and handle prefixes.

Rendered scene assertions use the framework's `decode_fixture_scene_with_lanes::<NodeGraphScene>` inverse on the real projected surface and then compare the typed scene as JSON. They do not depend on member order, text formatting, or an obsolete nested `nodeGraph` member. Fixtures retire their live Flow snapshots and closing app owners on success and failure paths.

## Causal repair boundary

The app-local repair will add a typed `Move` operation, make operation-array decoding strict, run move through the existing retained graph-operation owner and `host_operations`, and thread `InteractionView` selection into `main::render`. The non-request renderer supplies an empty selection because no interaction state exists on that interface.

The later shared repair will add an optional, closed `NodeGraphInteractionDomain` envelope to the typed scene contract. React and WGPU will qualify every node, edge, and handle target through that scene-owned address. A single neutral fixture and JSON schema will be consumed by the Rust scene law and the real React producer oracle; Ajv will independently validate the fixture. Flow will set its three application prefixes in its main scene. The shared producer will publish no domain action when the optional address is absent.

## Validation receipts

- `flow-native-1.log`: compile-time setup RED after 4m26s. Six `E0277` diagnostics proved the new real-app laws passed the `FlowAppFixture` wrapper to `settle_registered_typed_operation`; the harness was corrected to pass its dereferenced live app.
- `flow-native-2.log`: focused run completed in 2m58s with 0/4 selected passing and 246 outside the filter. The move law reached its intended production RED (`None` instead of `Some((284, 48))`), and the unknown-operation law reached its intended whole-command-refusal RED. The two render laws exposed a setup error by looking for an obsolete recursively nested `nodeGraph` member; they now decode the real surface document through `decode_fixture_scene_with_lanes`.
- The move law now proves that the starter graph contains widget `add` at its resolved `(0, 0)` position before dispatch. The refusal law sends one valid move followed by one unknown operation and requires no retained operation and no partial layout update.
- Production now decodes the exact current `setHostSnapshot`, `deleteSelection`, `connect`, `disconnect`, and `move` rows through one closed parser. Unknown operations, missing fields, extra fields, duplicate fields, invalid identifiers, non-finite coordinates, and invalid host snapshots refuse the whole array before retained admission. `NodeGraphEdit` and `SpotlightCommit` share that parser and typed operation enum.
- The language-neutral physical fixture retains the original five cases and adds trusted output-handle-to-input-handle connect plus wired-input-to-empty-canvas disconnect. React requires the actual Flow owner to republish the expected synapse and then retire its owner-assigned id; WGPU requires the exact closed connect/disconnect rows, obtains the owner-assigned disconnect id after the connect publication, and requires a repeated detach after retirement to publish no graph edit. Its source oracle pins the React Flow guest and WGPU producers to the same `connect`/`disconnect` row vocabulary and pins Flow's parser to all five current variants.
- A read-only Bun validation compiled the ticket TypeScript syntax and validated the seven-case fixture against its draft-2020-12 schema with Ajv: `[DEBUG] Flow physical fixture Ajv and TypeScript syntax passed`. The browser command was intentionally not run while root owns the activated servers.
- The app-local green confirmation and shared scene/React/WGPU domain confirmation remain pending the root-owned Nx/Cargo lane and renderer checkpoint 17.

## FlowNative3 ownership diagnosis

FlowNative3 completed 1 of 5 selected laws with 246 outside the filter in 2m1s. The closed operation vocabulary passed. The scene-domain law reached its intended missing-envelope RED. The other failures separated two fixture lifecycle faults from production behavior:

- `select_graph` stopped at the admission receipt from `FrameworkInteractionSelectJob`; the reserved job had not applied selection when the render read `InteractionView`, so the scene serialized no `selection` member and the typed JSON observation read `Null`. The helper now calls `settle_framework_reserved_admission`, matching the framework's and Generation3d's existing interaction laws.
- `FlowAppFixture::Drop` spun `close_step` up to one million times without the framework helper's time, yield, blocked-authority, and awaiting-input handling. It now delegates to `close_registered_fixture_app` and leaves panic cleanup to the underlying panic-aware owners. This addresses the unknown-operation law's terminal-empty timeout and makes any remaining move failure attributable to publication or retained ownership rather than the duplicate close loop.

The move law's FlowNative3 ordered-map retirement panic happened after the exact action entered this flawed fixture boundary, so no production conclusion is drawn from it yet. The next focused run must first confirm the move publication assertion and exact terminal close under the corrected fixture.

## Schema-owned NodeGraph interaction projection

The shared scene contract now carries an optional, closed `NodeGraphInteractionDomain` with exactly four non-empty fields: `id`, `nodeTargetPrefix`, `edgeTargetPrefix`, and `handleTargetPrefix`. Rust serde and DSL-value encoders use the same camel-case envelope; TypeScript mirrors it. An absent envelope means the surface declares no framework interaction address, so generic React and WGPU NodeGraph producers publish neither `interactionSelect` nor `interactionHover`. Incoming scene selection, hover, edge, and handle ids remain raw engine ids for painting.

Flow's editable main scene owns the app-specific mapping: `graph`, `flow-play-document.widget.`, `flow-play-document.synapse.`, and `flow-play-document.handle.`. React qualifies all node/edge/handle selection targets and node/handle hover targets at its shared producer boundary. WGPU stores the domain with the retained engine surface, applies the same prefixes, includes the declared domain id in publication fingerprints, and leaves domain-independent viewport publication intact. The WGPU node-graph law's live scene now opts into that explicit address and expects the qualified target text.

The neutral fixture and Draft-07 schema live at `renderer/engine/🧪️fixtures/🕸️node-graph-domain-interaction/🔣️.json` and `renderer/engine/🧬️schema/🕸️node-graph-domain-interaction/🔣️.json`. The exact React/Ajv producer oracle validates the closed non-empty envelope, mixed node/edge/handle selection, node and handle hover, empty hover clear, domain absence, and raw incoming paint ids. `node-graph-domain-react-3.log` records one file and five passing tests in 4.38 seconds of Vitest / 5.9 seconds of Nx with zero cache hits. `node-graph-domain-react-1.log` targeted the unrelated UI project and `node-graph-domain-react-2.log` used the renderer's default fundamental include; both correctly selected no tests and are not validation receipts.

## Bounded operation parsing and semantic refusal

`operations_from_action` now refuses more than the retained route's existing 256 rows before walking the batch, and refuses canonical argument JSON larger than the route's existing 16,384-byte wire authority before parsing a `setHostSnapshot` payload. No new capacity was introduced. The closed five-operation vocabulary remains shared by `nodeGraphEdit` and `spotlightCommit`.

The batched host path now has a fallible atomic host-operation helper. `setHostSnapshot`, `deleteSelection`, `connect`, `disconnect`, and `move` preserve their exact semantic error as a `Fault`; a failure retires the temporary host and returns no partial diff. This replaces the prior `.is_ok()` filters and `.ok()` snapshot decode that silently treated semantic refusal as a successful empty mutation. A native law pins the 257-row and over-16-KiB parser refusals; its root-owned execution is pending after the current native lane.

## Native 68 domain integration repair

Native 68 ran 1,257 renderer tests and exposed three packet-adjacent assertions:

- The initial cache stored `Option<Option<NodeGraphInteractionDomain>>` inline. Four `String` headers cost 96 bytes in both the live `EngineSurface` and its `EngineSurfaceRetirement`, increasing every fixed slot by 192 bytes and omitting the new owner from the close cursor. The cache now stores `Option<Arc<NodeGraphInteractionDomain>>`: one eight-byte shared owner per live/retiring representation, cheap event-time clones, and an explicit close step that drains all four strings before clearing the owner. The measured slot delta is therefore 16 bytes, from 75,704 to 75,720, and the committed fixed-slot fixture records that representation rather than accepting the 75,896-byte inline form.
- The physical Flow gesture fixture still expected raw `extrude` in `interactionSelect`. It now derives the node prefix from the mounted scene's declared domain and expects `flow-play-document.widget.extrude`; the edit operation itself continues to carry raw `nodeId: "extrude"` for the Flow command parser.
- The saturation law left one queue slot open because the old graph dispatch emitted two actions. An absent interaction domain intentionally suppresses selection and hover, so the wheel emits only its domain-independent viewport action and the old setup admitted it. The graph queue is now completely full; the board setup still leaves one slot because its wheel reserves two and its later pointer-up case deliberately fills the last slot. Both paths retain the invariant that a refused reservation leaves the camera unchanged.

No Cargo command was run locally. The exact focused filter is `test(engine_canvas_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack) | test(a_press_on_a_node_body_selects_that_node_and_a_released_drag_publishes_its_move) | test(saturated_graph_and_board_wheel_queues_preserve_cameras)`.
