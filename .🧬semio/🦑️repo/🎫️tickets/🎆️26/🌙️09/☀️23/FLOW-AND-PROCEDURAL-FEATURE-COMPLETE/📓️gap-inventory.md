# Flow and Procedural Gap Inventory

Source: gap inventory on 2026-09-23. The three known bugs (synapses, one-sided widgets, procedural 2D catalogue) are tracked in the other audit notes.

1. Generation2d `nodeGraphEdit` ignores `move`, `disconnect`, and `setSlider`. File: `generation2d/.../🎮️commands/✏️node-graph-edit/🦀️.rs`.
2. Generation2d flow window omits `NodeGraphInteractionDomain`, operator records, and live selection. File: `generation2d/.../windows/🕸️flow/🦀️.rs`.
3. `previewEval` refuses to run while operator kinds are unserved. File: `generation2d/.../preview-eval/🦀️.rs` (same pattern in generation3d).
4. Generation3d catalogue rows are click-only and emit no `application/x-flow-widget`. File: `generation3d/.../🛍️catalogue/🦀️.rs`.
5. Generation2d context menu passes an empty selection. File: `generation2d/.../✏️editor/🦀️.rs`.
6. Flow context menu reads selection only from the surface snapshot. File: `flow/.../✏️editor/🦀️.rs`.
7. Generation2d has no `patchFlowWidgets` command. Generation3d does.
8. Generation2d has no first-class `deleteSelection` or `disconnect`. Delete only exists inside `nodeGraphEdit`, and that route sees an empty selection.
9. Generation2d `render` builds the graph with a throwaway `FlowEvalSession`.
10. Flow `nodeGraphEdit` has no `setSlider` arm.
11. Txt import and export for flow and both procedural artifacts return not-implemented.
12. Generation2d omits generation3d keyboard graph navigation commands.

`connectMediaPorts` discards `host.connect_ports` errors.
