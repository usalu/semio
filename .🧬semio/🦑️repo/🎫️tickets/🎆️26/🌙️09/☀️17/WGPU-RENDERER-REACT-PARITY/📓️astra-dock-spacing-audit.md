# Dock Spacing Audit

This is a read-only source audit against the sealed checkpoint10 browser evidence. No new browser was run and no production source was changed.

## Measured Scene Rectangles

At 1600 × 1000, the dismissed-tour checkpoint publishes:

| Renderer | Top viewport | Perspective viewport |
| --- | --- | --- |
| React | x 6.375, y 60.75, width 523.765625, height 907.265625 | x 539.703125, y 60.75, width 1053.921875, height 907.265625 |
| WGPU | x 3.2, y 60.8, width 531.19995, height 907.2 | x 534.39996, y 60.8, width 1062.3999, height 907.2 |

React therefore has a 9.5625 px gap between the two live scene viewports. WGPU's two scene viewports touch, within floating-point rounding. This is independent of the camera-fit pose defect: even a correct camera receives a different aspect ratio and orthographic projection width.

## Source Findings

React `ui/🧱️elements/🎨️Canvas/🟦️.tsx` inserts a real `ResizableHandle` between every axis child. `ui/🧱️elements/↔️Resizable/🟦️.tsx` makes its fixed width or height `var(--spacing-single)`. The group distributes the remaining extent to its panels. Canvas mode also uses `MODE_CANVAS_INSET_CLASS = "p-single"`, and each `WindowChrome` body uses `p-single`.

WGPU `engine/🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs` independently repeats contiguous axis arithmetic in `render_axis`, `walk_resize_axis`, `collect_stack_frames`, `collect_stack_bodies`, and tab/drop walks: each width is `bounds.w * weight / total`, and the next child starts at the previous edge. No separator extent is deducted. A painted or widened resize hit around that edge does not reserve layout space.

Shell `plan_dock_windows` publishes those contiguous body rectangles into its visible scene plan, drop targets, and world viewport. `render_main_window_step` applies only its outer `theme.panel_inset` before the dock plan. The additional React body padding explains the measured horizontal edge offsets, but a repair must compare the complete cap/silhouette/content route before adding a second blanket inset: the current vertical viewport starts and ends are already nearly equal, so changing all four edges by guesswork would regress them.

## Bounded Next Packet

Use one themed axis geometry authority for bodies, caps, separators, drag/drop zones, and split resize. Reserve `(child_count - 1) * spacing_single` before distributing positive weights. Audit the external resize delta denominator against actual React ResizablePanelGroup semantics; keep minimum fractions and pair conservation. Preserve the intentionally raw theme-free `stack_frame_rects` contract only if it remains explicitly separate from actual rendered geometry; do not silently treat it as the browser pixel oracle.

Use a neutral row/column/nested fixture with explicit extent, gap, weights, and content insets. The reference must mount the actual installed `ResizablePanelGroup`, `ResizablePanel`, and `ResizableHandle` (or the actual Canvas mode), measure DOM rectangles, and validate both window body and separator geometry. The WGPU consumer must assert the actual body/drop/hit registry, not only a helper duplicating the formula.

Also resolve WindowChrome body padding with its silhouette safe-body geometry so the delivered World3d viewport matches the React host in both axes. Test maximized and empty stacks, where separator and body-padding rules differ from a normal split.

The active Sol Chrome packet remains initial body publication and retirement; this geometry packet is queued after that source is stable. Fresh canonical browser activation and camera/viewport comparison are required before acceptance.
