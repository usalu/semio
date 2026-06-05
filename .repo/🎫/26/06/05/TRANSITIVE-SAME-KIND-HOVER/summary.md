# Transitive Same-Kind Hover

Implemented transitive same-kind hover for puzzle 3D and puzzle 2D playgrounds.

## Behavior
- Hovering an instance derives its catalog kind and highlights all instances sharing that kind in canvas and hierarchy/kinds trees.
- Hovering a kind row in the kinds tab highlights all matching instances without a direct instance target.
- Canvas, hierarchy, and kinds tab share one hover focus per play shell.

## Files
- framework/core/index.ts — shared Puzzle2dKindHover / Puzzle3dKindHover types
- puzzle/3d/react/index.tsx — kindHover registry, transitive item hover, controlled PlayCanvas hover
- puzzle/3d/play/index.ts — hierarchy/kinds hover handlers, highlightedIds, controller hoverFocus
- puzzle/2d/rs/lib.rs — hovered_kind, transitive paint, WASM setHoveredKindSilent
- puzzle/2d/react/index.tsx — kind-aware hover payload and controlled sync
- puzzle/2d/play/index.ts — transitive hierarchy highlightedIds, kinds row hover
- framework/product/playground/renderer/react/index.tsx — shell hover wiring for 2D and 3D

## Tests
- puzzle/3d/play: 62 tests passed (including transitive highlight tests)
