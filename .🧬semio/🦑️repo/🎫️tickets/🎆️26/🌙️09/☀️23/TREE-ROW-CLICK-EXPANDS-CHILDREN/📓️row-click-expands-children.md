# Tree Row Click Expands Children

## Finding

Window Options groups such as Layer Weights are tree rows with children. Two renderers required the chevron:

- Retained wgpu `EventRouter::pointer_toggle_disclosure` folded an authored tree item only when the press landed inside the chevron rect.
- React `TreeItem` group rows called `onClick` from the row shell and left folding to the chevron button. Property rows folded from the label only when the row had no activation.

## Change

A press on an expandable row folds it. The chevron still folds without activating. A declared row activation still fires. A header value control stops propagation so a slider keeps its own gesture.

## Checks

- `bun ./📜️script.ts test-wgpu-engine a_nested_tree_item_opens_from_its_row_and_keeps_its_action -- --exact` in the ui rust package: 1 passed.
- `bun ./📜️script.ts test` on the Tree component file in the ui react package: 35 passed.
