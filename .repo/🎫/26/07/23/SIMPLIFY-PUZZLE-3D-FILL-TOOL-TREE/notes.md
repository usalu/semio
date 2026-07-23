# Simplify Puzzle 3D Fill Tool Tree

## Desired UX
Pressing the Fill tool toggle should immediately show:
1. Count slider
2. Objects distribution (kind weights)
3. Vortices distribution (kind weights)

No nested "Fill" group (the toggle already owns that row). No fill/edit-volumes mode Select cluttering the primary tree.

## Change
- `puzzle3d_fill_tool_measures` returns a flat measure list: count → distributions → edit-volumes toggle → (voxel dims when editing).
- Shared `puzzle3d_distribution_children` used by fill (open by default) and brush (still under a Distribution group).
- `setFillEditTargetVolumes` accepts Toggle `pressed` as well as legacy select values.
- Concurrent `waiting` field on UI nodes required `waiting: None` after existing `loading: None` in puzzle/framework plugin constructors so the crate compiles.

## Test
`cargo test -p puzzle-plugin fill_and_brush_params_are_tagged` → 2 passed (d3 + d5 filter match).
