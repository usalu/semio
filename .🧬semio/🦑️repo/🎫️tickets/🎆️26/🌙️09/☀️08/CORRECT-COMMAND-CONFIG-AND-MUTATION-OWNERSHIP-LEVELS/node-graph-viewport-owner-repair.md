# Node Graph Viewport Owner Repair

## Scope And Result

The NodeGraph scene, React and native renderer sessions, WGPU action emitter, Surface host and domain commands now carry the shared `Viewport2d` record directly. The action boundary is the nested record `{ viewport: { x, y, zoom } }`; `viewportJson` and the NodeGraph use of `cameraJson` are removed rather than retained behind an adapter.

This repair does not change `World3dScene.cameraJson`, renderer lens values, projection modes, authored cameras, tutorial cameras or icon-render cameras. Those remain separate owners and are covered by the world-scene audit and the later projection work.

The implementation preserves the NodeGraph attach/live boundary. A scene viewport hydrates a newly attached session once. The retained Surface content signature excludes the scene viewport, and later content echoes preserve the live session camera. Pointer, wheel, fit and interaction publications read the live session viewport and publish the same typed action. A missing scene viewport uses the shared `Viewport2d` default `(0, 0, 1)`; a present invalid viewport is rejected by the shared validation contract.

## Owner Boundaries

| Boundary | Final representation | Behavior |
| --- | --- | --- |
| Shared scene | `NodeGraphScene.viewport?: Viewport2d` / `Option<Viewport2d>` | Uses the one UI viewport schema and codec; the duplicate native scene viewport record is removed. |
| Surface host | `NodeGraphScenePayload.viewport: Option<Viewport2d>` and `GraphHost::viewport() -> Viewport2d` | Strictly decodes present records, defaults only when the field is absent, hydrates on first attach and retains the live camera across lagging scene echoes. |
| React session | `GraphSession.viewport(): unknown` followed by `parseViewport2d` | Accepts the WASM/host object directly. String JSON is no longer accepted on the NodeGraph session path. |
| React action | `{ viewport: Viewport2d }` | Pointer, wheel, startup fit, graph-change refit and explicit fit all publish the typed nested record. |
| WGPU action | `GraphInteractionSnapshot.viewport: Viewport2d` | Validates finite coordinates and positive zoom before reserving an action, then writes a nested `viewport` object through the bounded action builder. |
| Flow ABI | `viewport` | The ABI name, host opcode ledger and browser declaration agree; source and published declaration/ledger files are byte-equal. |
| Domain commands | `viewport: Viewport2d` | Procedural 2D/3D, Architect, Space, DAG, Flow, Equation, Wires and Trinity consume the shared record while preserving their existing config/no-op destinations and authored defaults. |

Trinity Jack and Rewriting now expose the renderer action name `nodeGraphViewport` and validate the shared record before projecting it into their window camera config. Architect still flattens the record into its existing `graph_camera_x`, `graph_camera_y` and `graph_camera_zoom` config fields. Space still writes its workflow-window camera entry. Flow and Wires remain non-persisting view commands where their current handlers intentionally emit no document mutation.

## Schema And Interoperability Evidence

The shared viewport route already passed its complete current matrix and was not rerun after this repair because the shared viewport package was not changed:

| Check | Result |
| --- | --- |
| Neutral fixture | 20 2D/3D valid and hostile cases passed strict TypeScript, native Serde and first-party `FromValue` agreement. |
| Shared native UI viewport | 2 selected viewport ownership laws passed. |
| Kernel DSL/Pack | 2 selected viewport ownership laws passed. |
| UI scene quick suite | 117/117 passed after `NodeGraphScene` switched to the shared record. |

The NodeGraph-specific source route `node-graph-viewport-ownership` passed before the native work and again after the final ten-consumer route correction. The latest isolated Nx run completed in 46.7 seconds. It regenerated and compared the Flow declaration surface (105 methods plus parity and five hostile cases) and passed the two focused React laws with 1,053 unrelated tests skipped: live session viewport publication and agreement with the shared neutral schema. The Flow browser declaration and host opcode ledger in `🫀️core/🕸️bindings` are byte-equal to their published copies under `🕸️wasm/📦️packages/🟨️javascript`.

Static searches over the NodeGraph renderer and domain roots find no `viewportJson` or `viewport_json`. Remaining `cameraJson` occurrences belong to World3d, Paint2d, Board2d, TiledMap, TextEditor, label-overlay paint state or Trinity snapshot text serialization and are outside this repair.

## Native And Runtime Evidence

| Layer | Result | Runtime evidence |
| --- | --- | --- |
| Surface WASM | Passed | The canonical Surface WASM build regenerated a 28.41 MiB module. Generated bindings expose `GraphSession.viewport(): object` and `graphsession_viewport`. |
| UI scene | 1/1 passed | `typed_scene_neutral_catalog_matches_native_serde_contracts` exercised the committed typed-scene fixture. |
| Surface | 44/44 passed | The NodeGraph unit filter includes eight shared neutral 2D cases, invalid zoom rejection, typed custom/default cameras and the attach-versus-live echo law. |
| WGPU renderer | 4/4 passed | `node_graph_attach_tests` covers a non-empty attached draw packet, nested typed wheel action output, invalid viewport rejection and graph interaction action behavior. |
| Four WGPU host filters | Pending | Queued after the coordinated Pack/FEM native slices. |
| Ten artifact consumers | Pending | Queued through the registered `consumers` phase after the four host filters. |

The first Surface compile exposed two tests that still treated the now-fallible typed scene decoder as infallible. Both call sites now assert successful decode with `.expect(...)`.

The first WGPU run passed three of four laws. The attach law raced with its siblings because the test helper drained every globally staged scene. The helper now removes only the named surface from the fixed staging table. The rerun passed 4/4. The attach law keeps its original non-empty-scene and packet-size assertions before cleanup, then drives `EngineCanvasPacket::close_step` until the transferred opaque scene owner is terminal-empty, verifies the drained build context, drops both owners, and finally closes the host surface. This prevents a nonterminal packet `Drop` from concealing the assertion or retirement behavior.

A subsequent host compile stopped before the first host law while a concurrent required Store capability change temporarily lacked two `next_close_byte_demand` implementations. That failure was outside the viewport source and is owned by the retained Pack work. The Cargo lane was released immediately rather than holding it through the foreign repair.

## Registered Verification

The canonical root route has no dependency on ticket files:

- `bun nx run workspace:node-graph-viewport-ownership`
- `bun nx run workspace:node-graph-viewport-ownership-native`
- `bun nx run workspace:node-graph-viewport-ownership-renderer-host-native`
- `bun nx run workspace:node-graph-viewport-ownership-consumers-native`

The ticket validation facade forwards the same arguments for isolated execution. Root and ticket Nx targets and both launch catalogs contain the source, aggregate native, renderer/host native and consumer native entries in orders `311.215` through `311.218`.

## Implementation And Verification File Ledger

### Shared Scene And Typed Fixture

- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🎬️scene/🧾️typed/📇️catalog.json`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/🧫️fixtures/🧾️typed-scene/🔣️.json`

### Surface And Generated WASM

- `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs`
- `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface.js`
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface.d.ts`
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface_bg.wasm.d.ts`
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/🕸️bindings/framework_surface_bg.wasm`

### React And WGPU Renderer

- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/📖️stories/🧪️.story.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs`

### Flow ABI

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧬️schema/📡️abi.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/flow_core.js`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/📝️flow-browser.d.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/🕸️bindings/🖥️flow-host.js`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/📝️flow-browser.d.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/📦️packages/🟨️javascript/🖥️flow-host.js`

### Domain Consumers And Proofs

- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️node-graph-viewport/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️reorganize/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕸️graph/🦀️.rs`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🦀️.rs`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🟦️.ts`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎮️commands/🖱️node-graph-viewport/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️.rs`
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️main/🟦️.ts`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🕸️main/🟦️.ts`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️node-graph-viewport/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🧪️tests/🔬️window-config-ownership/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖥️set-viewport/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧵️job/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖥️set-viewport/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️graph/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🛜️wire-runtime/🦀️.rs`

### Verification And Launch Surfaces

- `📜️script.ts`
- `📋️project.json`
- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/📜️script.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/project.json`

## Remaining Acceptance Work

Root owns the remaining native sequence after the coordinated Pack, FEM28 and window-identity batches. The first phase target is `abstraction-ownership-validation:node-graph-viewport-ownership-renderer-host-native`. Because Surface WASM, UI scene, Surface and WGPU attach are already green and their source has not changed, the remaining four renderer-package filters are:

1. `concrete_window_instances_round_trip_without_kind_collapse`
2. `context_menu_point_resolves_the_exact_concrete_window_instance`
3. `canonical_ui_preference_fixture_replays_to_the_same_projection_as_typescript`
4. `build_os_commands_covers_every_wired_setting`

The second phase target is `abstraction-ownership-validation:node-graph-viewport-ownership-consumers-native`. Its ten package/filter tuples are:

1. `semio-s-artifact-procedural-generation2d` — `node_graph_viewport_sets_camera` — feature `component-app-assembly`
2. `semio-s-artifact-procedural-generation3d` — `every_command_round_trips_through_text_and_binary` — feature `component-app-assembly`
3. `semio-s-artifact-architect-program` — `every_command_round_trips_text_and_binary_under_its_declared_wire_keyword`
4. `semio-s-artifact-reasoning-wires` — `every_command_round_trips_through_text_and_binary`
5. `semio-s-plugin-space` — `node_graph_viewport_writes_typed_workflow_camera_config`
6. `semio-s-artifact-dag-dag` — `every_command_round_trips_through_text_and_binary`
7. `semio-s-artifact-flow-flow` — `node_graph_viewport_moves_the_camera`
8. `semio-s-artifact-mathematical-equation` — `node_graph_viewport_writes_config_not_mutations`
9. `semio-s-artifact-trinity-rewriting` — `trinity_rewriting_command_text_and_binary_round_trip` — feature `component-app-assembly`
10. `semio-s-artifact-trinity-jack` — `trinity_jack_command_text_and_binary_round_trip` — feature `component-app-assembly`

Existing task log paths are:

- `🗑️generated/node-graph-flow-source.log` — Flow declaration generation/check evidence.
- `🗑️generated/node-graph-source.log` — completed earlier source target, including 2/2 React and 1053 skipped.
- `🗑️generated/node-graph-native.log` — first native attempt, including successful WASM/UI evidence and the stale fallible Surface test diagnostics subsequently fixed.

The successful Surface 44/44 and WGPU 4/4 reruns were observed in the coordinated live sessions and are recorded above; no durable compiler-output log was created for those reruns. The final source target after the Wires tuple was added completed successfully in 46.7 seconds through the isolated ticket Nx facade.

After both remaining phases, update the pending result rows and the central ownership checklist with exact counts and failure/fix evidence. The task-specific Nx workspace-data directories may be deleted; shared ticket output and other owners' logs must remain under their respective owners until the ticket is closed.

Root handoff refinement: the already registered `abstraction-ownership-validation:host-ownership-native-test` executes exactly the four pending WGPU host filters through `host-ownership native`. Root can use that route to resume the pending host checks without rebuilding and rerunning the completed Surface/WASM/attach phases. The corresponding root verify route is `host-ownership`. Keep all current generated logs until whole-ticket completion; source handoff is not ticket cleanup.
