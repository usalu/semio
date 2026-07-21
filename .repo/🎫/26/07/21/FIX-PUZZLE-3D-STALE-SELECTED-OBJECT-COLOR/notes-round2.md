# Stale selected color — round 2

## Root causes found
1. **React GLB path**: style paint was applied with imperative `material.color.set` after clone. Deselect often left the previous selected tint until a later hover forced another update. Fix: bake color/emissive into the clone `useMemo` (style deps recreate the tree) and remount with `key={styleKind}`.
2. **WGPU/infinite world path**: `apply_runtime_draw_flags` did `instance.selected = instance.selected || local_selected`, so a stale `instancesJson` selected bit kept the selected shader flag after `selectionJson` cleared. Fix: assign from the live selection snapshot only.
