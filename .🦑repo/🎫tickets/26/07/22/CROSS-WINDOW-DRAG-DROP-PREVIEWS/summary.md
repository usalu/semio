# Cross-Window Drag Drop Previews

## Problem

Catalogue/fixture drop ghosts were local to the hovered host instance, so multi-pane layouts (e.g. split Puzzle 3D windows) only showed the preview in the pane under the pointer.

## Fix

- **Puzzle 3D**: shared world-space catalogue-drop store keyed by `controllerId`. Every `World3dHost` pane subscribes and renders the same origin; only the hovered pane publishes raycast updates.
- **Puzzle 2D**: fixture-drop preview uses world `x`/`y` (not per-pane screen coords) and is pushed to every `board2dPeerRegistry` pane of the controller.

## Verification

`bun ./script.ts test quick` in `framework/renderer/react` — 202/202 passed.
