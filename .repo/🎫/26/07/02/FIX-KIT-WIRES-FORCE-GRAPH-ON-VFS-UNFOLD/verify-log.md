# Verify log

## Root cause

1. **Position reset on topology replace**: When VFS unfolds, `syncKitWiresTopology` rebuilds the wires fixture with fresh grid positions. `usePlatformTopologyStore` called `replaceModel` on structure changes, wiping settled force-graph positions and camera — nodes jumped off screen.

2. **Duplicate async VFS loads**: `ensureChildrenLoaded` and `ensureChildrenLoadedAsync` started separate in-flight loads for the same branch. `prepareKitWiresVfsForTopology` could read visible nodes before the toggle-triggered load finished, so wires sometimes missed newly expanded children.

## Fix

- `mergeLiveForceGraphTopologyModel` in puzzle 5d: preserve existing flat centers + camera; spawn new nodes near their containment parent.
- Platform topology hook: merge on `:kit:wires` structure changes instead of raw replace.
- Unified VFS child loading through one shared promise per branch key.
- FiveD live force RAF restarts when node ids change, not only count.

## Tests run

- `@semio-tech/framework-platform-core:test` — 16 passed (incl. shared async load test)
- `puzzle/5d/react` vitest `-t mergeLiveForceGraphTopologyModel` — passed
- `@semio-tech/compose-sketchpad` vitest `-t "typology is expanded"` — passed
