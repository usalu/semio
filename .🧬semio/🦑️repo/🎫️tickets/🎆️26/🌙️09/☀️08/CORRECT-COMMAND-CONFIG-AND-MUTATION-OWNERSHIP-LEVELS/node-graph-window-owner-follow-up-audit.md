# Node Graph Window Owner Follow-up Audit

Root follow-up: Generation2d's app-global camera is also read by both edit and generate Canvas2d preview renderers, so the next migration must give those real preview windows their own viewport owner before removing the app field. The corrected full inventory and migration sequence are in `generation-camera-exact-owner-migration-plan.md`. The Space singleton source note limits claims of observed collisions; it does not exempt a window-specific setting from this goal's required window-level abstraction.

## Scope and evidence standard

This is a read-only source audit of the typed NodeGraph viewport route for Generation2d, Generation3d, Wires, Flow, and Space. It follows the repair evidence in node-graph-viewport-owner-repair.md and the checkpoint in ownership-completion-checklist.md.

The route to the native action is source-proven: NodeGraphScene.viewport is a shared Viewport2d, the Surface NodeGraphScenePayload carries viewport: Option<Viewport2d>, GraphHost returns the live camera through its WASM GraphSession, and the UI sends nodeGraphViewport with a viewport object. The native artifact actions parse that object as the same shared Viewport2d. This establishes typed transport only. It does not establish the owner which retains the value or the reader which renders a later snapshot.

No Cargo, native, browser, or runtime tests were run for this audit. Statements headed **Source-proven** are static-path findings, including assertions that appear in source tests. They are not runtime proof. “Native pending” means that the checklist/report names no completed native result for the relevant law in the material inspected.

Architect and Trinity are excluded. Generation3d World3d preview navigation is also excluded: its preview_camera and its recently migrated transient owner are a separate 3D concern, and NodeGraphViewport writes only the 2D flow camera.

## Result at the ownership boundary

| Mounted graph | Actual invocation context and declared lane | Final retained owner | Later snapshot reader | Exact-window / reopen status |
| --- | --- | --- | --- | --- |
| Generation2d NodeGraph, generation2d-main | The generated action parses viewport; NodeGraphViewport emits Generation2dConfigMutation::SetCamera on the Config lane. | The sole Generation2dConfig.camera: CameraJson record. It is app config, not addressed WindowConfig. | The Flow window renderer reads config.camera and constructs Viewport2d for NodeGraphScene::base. | No exact context, two-instance isolation law, or reopen law found. Native pending. |
| Generation3d NodeGraph, procedural-main | The generated action maps nodeGraphViewport to NodeGraphViewport; its handler emits Generation3dConfigMutation::SetCamera on the Config lane. | The sole Generation3dConfig.camera: CameraJson record. It is app config, not addressed WindowConfig. | The flow window instead reads document.fixture.camera before NodeGraphScene::base; no inspected editor renderer reads the config field. | The stored config write is disconnected from the graph reader. No exact context, two-instance isolation law, or reopen law found. Native pending. |
| Wires canvas, reasoning-wires-composite | NodeGraphViewport is in WIRES_RETAINED_TOOL_IDS on the WindowConfig lane. The retained WiresWindowDragWork requires the canvas WindowConfig owner and emits a typed WindowConfigMutation. | WiresCanvasWindowConfigOwner, addressed by the concrete view.window_id after verifying its kind. | The Canvas2d renderer receives the exact WiresCanvasWindowConfig and reads camera. | Source has concrete-owner rejection and a one-window document-reload camera preservation check. It has no two-canvas viewport/config-pack reopen law. Native pending. |
| Flow NodeGraph, flow-main | NodeGraphViewport is in FLOW_DIRECT_STORE_TOOL_IDS on the WindowConfig lane. The retained direct-store reducer creates main::config::addressed(view, next). | FlowMainWindowConfigOwner, addressed by the concrete view.window_id after verifying membership and kind. | The Flow main renderer receives FlowMainWindowConfig and converts camera to Viewport2d. | Source law creates two flow-main instances, separates viewports, serializes window config packs, recreates the app, and checks both rendered viewports. Native pending. |
| Space Workflow NodeGraph, s-workflow | The generated action dispatches NodeGraphViewport on the Config lane. The command writes SpaceConfigMutation::SetCamera with the S_PLAY_WINDOW_WORKFLOW constant. | SpaceConfig.camera: BTreeMap<String, SpaceWindowCamera>, keyed by the fixed workflow window-kind identifier in current code. | workflow_camera reads config.camera[S_PLAY_WINDOW_WORKFLOW], then converts SpaceWindowCamera to OsWorkflowCamera and Viewport2d. | Current source explicitly says split-pane window instances do not exist. The singleton command/round-trip source checks exist; no exact-instance or reopen law exists. Native pending. |

## Transport and no-op interpretation

The raw Flow and Wires command modules have NoConfig handlers which return an empty Emit. They are not the final mounted-action destination:

- Flow's registered direct-store path intercepts NodeGraphViewport, changes the copy of FlowMainWindowConfig.camera, and publishes main::config::addressed(view, next) on WindowConfig.
- Wires' retained WiresWindowDragWork intercepts NodeGraphViewport, requires the registered WiresCanvasWindowConfigOwner snapshot, and publishes a WindowConfigMutation for its concrete window id. The synchronous path also calls canvas::config::addressed.

Thus the raw empty Emit is an intentional generic-command fallback, not evidence that a renderer viewport action is currently discarded. Calling either mounted path a no-op would be a declaration-only finding and would be incorrect.

Generation2d, Generation3d, and Space do not have such a superseding exact WindowConfig publication. Their actual retained action routes end in the app-config mutations named above.

## Source-backed ownership findings

### Generation2d has a real global-camera alias

Source-proven:

- The node-graph-viewport handler in the Generation2d editor command tree converts Viewport2d to CameraJson and emits Generation2dConfigMutation::SetCamera.
- Generation2dConfig contains one camera field, and the flow window's render code reads that field without a ViewModel or window id.
- The mounted NodeGraph kind is generation2d-main.

There is therefore no state boundary capable of distinguishing two Generation2d NodeGraph instances. If the generic host supplies two same-kind view contexts, both actions write the same persisted record and both subsequent renders read it. This is an owner-level gap, not a field-name concern. The source reviewed does not itself demonstrate that the current Generation2d layout creates two such instances, so it is not evidence of an observed live cross-window incident.

The Generation2d config schema exposes that global CameraJson in both native Rust and TypeScript, each with x/y/zoom. It duplicates the neutral Viewport2d semantic record held by the action and scene. It is both a cross-language schema facet and the global retained record; moving it behind an exact schema fixes the ownership problem and gives the conversion one defined boundary.

### Generation3d has a disconnected global 2D camera and an authored graph seed

Source-proven:

- The editor action decoder maps nodeGraphViewport to the 2D NodeGraphViewport command, and that handler emits Generation3dConfigMutation::SetCamera with CameraJson.
- Generation3dConfig contains that one app-global camera field, but the procedural-main flow window reads document.fixture.camera at its NodeGraphScene construction. No inspected editor render call reads Generation3dConfig.camera.
- The mounted graph kind is procedural-main.

The NodeGraph action therefore writes a retained record that the current graph renderer does not consume. The document camera is an authored fixture value, including bundled-example camera values; it must not become a mutable app-global view record merely to make the current write appear useful. The required correction is an exact procedural-main window viewport whose initial seed policy explicitly consumes the authored value once, while later renders and viewport actions use only that exact-window record.

No exact instance id reaches the current config mutation or the future persisted view owner. This is independent of Generation3dPreviewCamera and the World3d preview windows, whose own global camera reader/writer inventory is covered by generation-camera-exact-owner-migration-plan.md.

### Flow is an exact retained route, with a source law

Source-proven in the Flow editor and Flow main config:

- FlowMainWindowConfigOwner declares the flow-main kind and is registered as a WindowConfig owner.
- main::config::addressed requires view.window_id, finds that id in view.window_instances, requires the flow-main kind, and creates WindowConfigMutation::of for that exact id.
- The renderer reads the WindowConfig snapshot through FlowMainWindowConfig; it does not read an app-global camera.
- flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows constructs left and right flow-main instances, sends different NodeGraphViewport values, checks separate scenes, captures window_config_packs, recreates an app, reloads each pack, and checks the reopened scenes.

The native and TypeScript schema facets expose CameraJson in FlowMainWindowConfig with the same x/y/zoom fields. That is a type duplication relative to neutral Viewport2d, but it is not a second active owner: the config record is one exact-window owner and the schema forms describe it. It should be converged only in an explicitly schema-first field unification slice, not treated as an ownership migration.

The existing law is the correct proof target. Its source assertion does not replace the pending native execution recorded by the checklist.

### Wires is an exact retained route, but lacks the full camera lifecycle law

Source-proven in the Wires editor and canvas config:

- WiresCanvasWindowConfigOwner declares reasoning-wires-composite.
- canvas::config::addressed rejects a missing window id and an id whose ViewWindowInstance does not have that kind, then creates a typed WindowConfigMutation for the concrete id.
- WiresWindowDragWork extent requires that exact owner for NodeGraphViewport; its step writes SetCamera for window.window_id().
- The Canvas2d scene receives the exact config and uses camera_x, camera_y, and zoom.
- wires_pointer_move_document_replacement_clears_only_successful_reload_previews dispatches NodeGraphViewport for a concrete left canvas, settles the real registered operation, reloads the document, and asserts that the same concrete camera and its config generation remain.

The Wires native and TypeScript schemas each represent WiresCanvasCamera with x/y/zoom. As for Flow, that is a schema-language/type facet at the action-to-owner boundary, not an additional runtime state owner. Do not migrate ownership solely because both forms exist.

The existing source test covers one concrete canvas through document reload while proving transient cleanup. It does not set distinct NodeGraph viewports on two same-kind canvases, capture window config packs, recreate an app, and read both reopened scenes. That specific ownership/persistence law remains absent.

### Space is currently singleton-scoped, not exact-instance scoped

Source-proven:

- NodeGraphViewport unconditionally writes the S_PLAY_WINDOW_WORKFLOW constant as the mutation key. Neither its handler nor its renderer receives an exact view-window id for camera lookup.
- SpaceConfig comments state that the camera map currently always uses that constant because split-pane window instances do not exist anywhere in the codebase.
- node_graph_viewport_writes_typed_workflow_camera_config and set_camera_round_trips_and_keys_by_window_id exercise the fixed-key mutation. They prove command/config codec behavior only.

The map key is called window_id, but its actual value is the window-kind/static workflow id. It is not a concrete WindowConfig owner. This is a real abstraction boundary difference from Flow and Wires, but it is not source proof of an existing two-Workflow-window collision because source expressly says that feature does not exist. Treat it as required schema work before enabling same-kind Workflow instances, not as proof of a current multi-window regression.

SpaceConfig is mirrored by a native schema and a TypeScript schema whose camera field is a Record<string, SpaceWindowCamera>; the latter declares x/y/zoom. SpaceWindowCamera and OsWorkflowCamera are additional field-equivalent conversion records. The current conversion is needed by the OS-facing workflow renderer; it is not another persisted owner. Defer type convergence to an exact-window migration instead of changing it alone.

## Smallest schema-first execution slices

1. **Generation2d exact camera configs.** Introduce schema-owned exact owners for generation2d-main, generation2d-preview, and generation2d-generate-preview. Each receives its own persisted Viewport2d record; the latter two kinds are both Canvas2d but are distinct concrete window kinds. Require and validate the concrete ViewModel context in the NodeGraph action, replace Generation2dConfig.camera, and render every surface from its exact snapshot. Implement every native/TS/proto/GraphQL/JSON facet together. Required laws: generation2d_flow_camera_isolates_and_reopens_exact_main_windows, generation2d_edit_preview_camera_isolates_and_reopens_exact_preview_windows, and generation2d_generate_preview_camera_isolates_and_reopens_exact_preview_windows.

2. **Generation3d exact cameras.** Make the equivalent procedural-main migration for its 2D NodeGraph viewport, with an explicit one-time authored fixture.camera seed policy. Separately migrate the actual World3d preview camera from app config into exact procedural-preview and generation3d-generate-preview owners. Do not change the separate preview-evaluation transient owner. Required laws are specified in generation-camera-exact-owner-migration-plan.md.

3. **Wires law completion, not an owner migration.** Add wires_node_graph_viewport_isolates_and_reopens_exact_canvas_windows. It must use the registered retained operation, two same-kind canvas instance ids, distinct Viewport2d actions, exact scene reads, config-pack capture/recreate/reload, and wrong/missing-kind rejection. This validates the existing WiresCanvasWindowConfigOwner rather than replacing it.

4. **Flow native closure and optional type convergence.** Run the existing flow_window_ownership_runtime_isolates_restores_and_resets_exact_windows in the intended native consumer gate. A separate follow-up may replace CameraJson with the shared Viewport2d in its schema if all generated facets and neutral oracle agree; it is not needed to repair owner selection.

5. **Space current singleton proof, then future exact migration.** Add space_workflow_node_graph_viewport_reopens_fixed_workflow_camera to prove the present fixed workflow camera survives a full config-pack/app recreation. Before a multi-instance Workflow feature can ship, replace the static map entry with SpaceWorkflowWindowConfig plus a WindowConfigOwner addressed by trusted ViewModel context and add space_node_graph_viewport_isolates_and_reopens_exact_workflow_windows. Do not claim the latter is current behavior until a two-instance layout exists.

Every new two-instance law must also assert that document Pack/SPR are unchanged by viewport actions, that stale/missing/wrong-kind contexts reject, and that a fresh app reading only the captured window config packs renders the same separate viewports.

## Existing native consumer filters and their limit

The repair report lists these consumer filters as pending:

- Generation2d: node_graph_viewport_sets_camera
- Generation3d: every_command_round_trips_through_text_and_binary
- Wires: every_command_round_trips_through_text_and_binary
- Space: node_graph_viewport_writes_typed_workflow_camera_config
- Flow: node_graph_viewport_moves_the_camera

They remain useful route/codec checks, but none except the separate Flow window-ownership law proves two exact instances and reopen persistence. The next owner-validation gate should therefore run the existing Flow law, add the Wires law above, and run the Generation2d/Generation3d laws only after their exact schema owners exist. The Space fixed-workflow law is a singleton persistence check; it must not be presented as a two-window law.

## Assignment recommendation

The next owner-repair implementation scope should be Generation2d and Generation3d 2D flow windows together only if they share an established schema/WindowConfig pattern; otherwise split them by artifact. Flow and Wires should receive validation completion, not a new ownership rewrite. Space should remain a scoped singleton follow-up until same-kind Workflow instances become an actual feature. World3d navigation remains out of scope.
