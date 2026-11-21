# Generation3d Current UX Audit

## Unconnected spatial inspector inputs do not publish channel hover

In `✏️editor/📌️panels/🔍️inspection/🦀️.rs`, connected neuron inputs use `connected_source` (lines 134–136, 242–254), which binds the input-direction graph port. Unconnected point/vector inputs instead build `coordinate_input` directly (159–161, 274–295), and planes build two such controls (162–167). Those groups and axis inputs have no `HoverPreview` action. Scalar and collection literal input rows likewise lack that binding. Output rows explicitly bind their output port (185–200). The shared preview visits evaluated input geometry and marks it by directional graph port (Edit preview lines 384–385, 438–440), so moving the pointer over the actual unconnected spatial input inspector control cannot publish the target needed to highlight its preview or graph port. The same authored UI feeds React and WGPU.

Reproduce with a neuron with an unconnected, evaluated point/vector/plane input: select the node, hover the inspector input label or axis and observe no channel hover while hovering its graph input port highlights its preview. Bind the containing input row/group to `graph_port_id(selected_id, port.name, true)` using the existing `bind_graph_row_hover`, including coordinate and collection descendants; preserve child edit actions and clear-on-leave. Cover pointer entry over both label and control, exit, and the projected input preview in mounted React and native retained dispatch. Inspect whether other input types need the same wrapper.

No mounted run or test was performed in this read-only audit; the reproduction is a source-derived scenario, not an observed runtime result. No second distinct issue was established.
