# Terra Graph Native Audit

## Scope and evidence

This audit is read-only. It traces failures 5 and 6 in [📓️astra-renderer11-failures.md](./📓️astra-renderer11-failures.md) against the committed WGPU graph fixture, the native scene producer, and the React `NodeGraphHost` contract. No build was run for this audit.

The prior native run reported 1,119 passing and 32 failing tests. Its geometry trace is retained in [renderer-native-tests-11-plain.txt](./🗑️generated/astra-runtime/renderer-native-tests-11-plain.txt).

## 5. Caption-count failure

`node_graph_paint_publishes_its_captions_over_the_engine_raster` currently builds a `NodeGraphScene` in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs:23` with the fixture's `hostSnapshot`, but leaves `NodeGraphScene::operators` empty. The fixture at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧫️fixtures/🕸️wgpu-node-graph/🔣️.json` contains seven widgets and six synapses only.

That is not the shape of the production Flow scene. `document_operator_records` in `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs:227` creates one operator record per neuron kind, and the producer installs those records in `NodeGraphScene.operators` at `:295` and `:304`. EngineCanvas consumes them during Flow synchronization in `🎯️targets/🧊️wgpu/🦀️.rs:2180`; its host attachment and synchronization are therefore present and connected.

Without those records, `FlowHost` has no kind information after `from_host_snapshot`. `neuron_output_ports` consequently returns no outputs for the three non-variadic neuron widgets in `🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs:567`. The native trace confirms the resulting state: the three slider captions and outputs, profile plus two inputs, extrusion-axis plus three inputs, and extrude plus two inputs produce 16 labels. Label publishing retains the height slider even though its live entity geometry is off screen; the preview intentionally has no label.

The present `>= 20` premise is also not the React contract. `DagHost::node_label_text` intentionally returns no caption for `Preview` and `Note` at `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:6158`; `label_overlay_rows_for_node` correspondingly suppresses their port rows at `:5802`. With production operator records, the fixture's three neuron outputs (`profile@wire`, `extrusion-axis@vectorOut`, and `extrude@solid`) add three visible port captions. The precise expected detail-LOD producer state is six node labels and thirteen port labels, for 19 rows. This is a contract-derived expectation and should be confirmed by the native test after the fixture is corrected.

There is no WGPU caption-layer production gap. `paint_node_graph_labels` obtains the public Flow label state and paints every row in `🎯️targets/🧊️wgpu/🦀️.rs:4104`. The WGPU scene layer invokes captions after the engine raster in `🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:2024`. React makes the same producer call in `🧱️elements/🕸️NodeGraph/🟦️.tsx:708` and paints each returned row at `:1896`; it does not synthesize labels for omitted widget kinds.

### Bounded fix

1. Extend the committed WGPU fixture with the same `operators` records supplied by the production Flow-window producer, and have `flow_window_scene` deserialize and install them in `NodeGraphScene.operators`. Keep the fixture independent of the procedural plugin at runtime.
2. Replace the numerical lower bound with exact producer assertions: six labelled non-preview/non-note nodes, thirteen labelled ports, and no preview caption. Assert that the three output handle identifiers above resolve as output geometry and that the six fixture synapses retain their endpoints. Retain the existing caption-after-raster and glyph-coverage checks.
3. Do not alter `FlowHost` delegation. Its public `entity_screen_json`, `slider_overlay_state_json`, and `label_overlay_paint_state_json` methods already delegate to live `DagHost` at `🌊️flow/🖥️host/🦀️.rs:1795`, `:1979`, and `:2154`.

The React `paintOverlays` producer/consumer path is the parity oracle for this change. A second oracle is the production `document_operator_records` scene assembly described above.

## 6. Tutorial semantic-point failure

`tutorial_semantic_points_resolve_through_live_graph_geometry` at `🧪️tests/🕸️wgpu-node-graph/🦀️.rs:182` requires a Canvas origin, `height` entity point, `e1` curve point, and `height` slider-domain point to resolve inside the 966 by 836 local surface.

The stored fixture camera makes the `height` node intentionally off screen. `DagHost::entity_screen_json` only marks node and handle geometry visible when its centre is on the surface, as implemented in `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:3969` and `:4072`. With the fixture's node x coordinate `-197.1913555`, camera x `94.7558157`, zoom `1.78443256`, and local centre `483`, the `height` centre projects to approximately `-37`; it is outside the left edge. The entity request correctly returns invisible and `resolve_tutorial_surface_point` correctly returns `None` for that point.

The test therefore assumes a visibility condition the committed camera does not provide. The height-domain target shares that off-screen region; `e1` may still resolve because its midpoint lies between height and extrusion-axis, but the test never reaches it after the height entity fails. React does not provide hidden-target reframing: `GraphSliderOverlays` maps each published slider from world to screen in `🧱️elements/🕸️NodeGraph/🟦️.tsx:2072`, and the containing surface clips it. It does not turn an off-screen semantic target into an on-screen target.

There is nevertheless a bounded native defect worth fixing. The slider branch of `tutorial_graph_geometry` in `🎯️targets/🧊️wgpu/🦀️.rs:1769` converts slider state to a point without rejecting an out-of-surface result. The entity and edge branches use `DagHost` visibility information; the domain branch can therefore return a local point outside the attached surface after the first test assertion is corrected. A semantic tutorial point advertised as resolved must be targetable on that surface.

### Bounded fix

1. Make `tutorial_graph_geometry` reject a slider-domain result outside the current `EngineSurface` local bounds. Apply the final local-surface validation consistently to every graph semantic result so `Some(point)` always denotes an on-surface, targetable point. Do not add automatic camera reframing; neither the Flow/Dag nor React contract supplies it.
2. Keep the committed camera and preserve a strong visibility law. Change the positive entity and slider-domain targets to `radius`; retain `e1` as the visible midpoint curve case or use `e2` as the radius-connected alternative. Add explicit negative assertions that the stored-camera `height` entity and height slider domain resolve to `None`.
3. Preserve the existing bounds assertions for every positive result. They validate the renderer's public tutorial contract rather than a raster implementation detail.

`DagHost::entity_screen_json` is the live geometry oracle for entity and curve cases. The React `NodeGraph` overlay mapping above is the oracle for slider visibility and for the absence of implicit framing.

## Recommended verification after implementation

Run the two native graph tests first, then the renderer native suite that produced the cited log. The updated diagnostic messages should show the exact missing semantic item if any remains. Confirm the corrected fixture publishes the three neuron output handles and exactly nineteen detail-LOD caption rows before accepting raster ordering evidence.
