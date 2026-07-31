# Lowpoly Select Window Options Parity

## Gap
Puzzle 3d exposes an always-visible **Select** window-options group with:
- Rectangle / Lasso method toggles
- Selective / Additive / Subtractive / Invertive merge-mode toggles
- Selectable kind toggles

Lowpoly only had mesh/face/edge/vertex toggles plus a Rectangle/Lasso dropdown, gated under the `move` utility options rail, and never emitted `selectionMergeMode`.

## Fix
- Replaced the utility-tagged selection group with an always-visible `lowpoly-select` group matching puzzle 3d's taxonomy (kinds remain mesh/face/edge/vertex).
- Added `selection_mode_default` runtime field + `setSelectionModeDefault` action.
- Emit `selectionMergeMode` in world selection JSON so the React host uses the chosen merge mode.
