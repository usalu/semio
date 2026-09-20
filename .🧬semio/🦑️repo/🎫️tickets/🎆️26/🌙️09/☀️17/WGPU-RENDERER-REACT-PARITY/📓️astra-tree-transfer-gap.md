# Retained Tree Drag Initiation Gap

The fresh React empty-dock journey requires pressing the visible transfer handle. Dragging the label/row does not create a window with the Default driver; selecting the Surface driver removes handles and permits row drags. The current ticket probe records which physical source it used, so a whole-row WGPU fallback is not accepted as identical interaction.

## Current Consumers

- React Tree `deriveTreeDragRoles` and `renderTreeDragHandles` expose sort (`grip-vertical`) and transfer (`move`) handles; `useUiDriverDragSurface` controls their presence. The window template uses a transfer payload.
- WGPU retained `📥️input/🦀️.rs::retained_hit_registration` maps a draggable item's entire row band to `drag_axis=Both` and its payload. It receives only TreeRowMetrics, with no driver policy.
- WGPU `⚡️events/🦀️.rs::EventRouter::dispatch` automatically adds the payload on any row pointer-down and promotes it by movement. No driver handle/surface test is present in that path.
- WGPU production `🖌️paint/🦀️.rs::retained_tree_node_step` paints chevrons, icons, labels and row actions but no transfer/sort handle. Do not repair only `paint_tree_item`, which is a test/reference painter.

## Bounded Required Packet

Carry the canonical driver drag mode into retained tree geometry, paint, hit publication and event initiation through one owned policy. Reserve the same trailing handle slot and action spacing as React, publish a semantic handle hit, and only arm row drags in Surface mode. Preserve row selection, chevrons and inline controls. Exercise both engine routing and host hit registry; a painter-only fix or registry-only narrowing leaves the other dispatch path wrong.

Use a neutral fixture with Default/Handle and Surface drivers, a transfer template, a sort-only row, a non-draggable row, and pointer starts on label versus handle. Verify actual PointerDown→Move→Up with cancellation and driver change. Use React Tree as the independent component oracle, then rerun empty-dock template creation with the same physical source in both renderers.
