# Puzzle 3D Transform Gumball Fix

## Symptoms

1. Sibling World3d panes did not show live gumball drag preview.
2. Vortex markers (and attraction endpoints) stayed at committed world positions while the mesh preview moved.
3. Rotate handles appeared to do nothing while translate still committed — rotate drags were dispatched as `translateSelection`.

## Root cause (rotate)

`mergeWorldSelectionWithLeftoverV1` let the host leftover overlay overwrite the guest `transformMode: "transform"` with `leftoverWorldGumballPoseV1`'s legacy `"move"`. With `transformMode === "move"`, `gumballKindForTransformMode` ignored rotate handle kinds and always built a translate delta.

## Fix

- Prefer guest `transformMode` over leftover in merge; stamp `"transform"` from leftover gumball pose.
- Treat `"move"` like `"transform"` when resolving handle kind → commit action.
- Add `WorldGumballTransformPreviewStore` (same controller / sourceId ownership model as marquee selection preview).
- Publish preview on gumball drag; sibling panes apply instance root poses from committed instance JSON + delta.
- Map vortex markers and attraction endpoints through `gumballPreviewWorldPoint` / direction helper while preview is active.

## Verification

- Engine contract: `shares live gumball transform previews…`, merge transformMode law, rotate-under-move handle law, existing gumball delta tests.
- Manual: split-pane puzzle3d, transform utility, drag move and rotate — all panes + vortices track; release commits rotate to document orientation.
