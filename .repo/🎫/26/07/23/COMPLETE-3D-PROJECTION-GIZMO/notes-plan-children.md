# Plan owns orthographic views

## Change
Top / Bottom / Front / Back / Left / Right are children of Plan, not siblings under Orthographic.

## Tree
`Parallel > Orthographic > Plan > (Top, Bottom, Front, Back, Left, Right)`

## Defaults / switcher
- Orthographic default is Plan (locked drafting view).
- Mode switcher lists only Plan's children; re-clicking Ortho from a child returns to Plan.
- Plan branch stays draggable (tree nodes carry dragData even when they have children).

## Verification
- `infinite/world/r3f` projection filters: passed
- `framework/renderer/react` tree/drag filters: passed
