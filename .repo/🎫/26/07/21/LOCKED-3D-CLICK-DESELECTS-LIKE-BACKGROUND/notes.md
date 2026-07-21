# Locked 3D Click Deselects Like Background

## Bug
Locked world entities (e.g. puzzle 3d references) still participate in r3f raycasts and call `stopPropagation` on pointer down. That blocks `WorldCanvas.onPointerMissed` / background deselect, so existing selection stays.

## Fix
- Canvas pick uses `worldEntitySelectable` (not inspectable): locked → no raycast → click equals background / pass-through.
- Volumes: same raycast gate when not selectable.
- Puzzle objects emit `disabled: locked`; instance nodes skip pick when disabled.
- `worldPick` on a locked/hidden object clears selection when merge is replace (defense in depth).
