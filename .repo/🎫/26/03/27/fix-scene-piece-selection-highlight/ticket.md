# Fix Scene Piece Selection Highlight

## Summary
Selected pieces in the 3D scene do not highlight because `getComputedColor` returns raw CSS variable references (e.g. `var(--color-primary)`) that `THREE.Color` cannot parse.

## Root Cause
In `ModelPiece`, `activeBaseColor` and `hoverBaseColor` are computed via `getComputedColor("--active-base")` and `getComputedColor("--hover-base")`. These CSS custom properties reference other variables (e.g. `var(--color-primary)`), so `getPropertyValue` returns the raw reference string, not the resolved color. `THREE.Color` cannot parse `var(...)` strings.

## Fix
Resolve CSS variables to actual RGB values using a temporary DOM element (same pattern as `materialColor` in `ModelPiece`).

## Files
- `compose/sketchpad/index.tsx`

## Fix Applied
In `ModelPiece` (~line 37804), replaced `getComputedColor("--active-base")` and `getComputedColor("--hover-base")` with DOM-based resolution that sets `color: var(--active-base)` on a hidden element and reads `getComputedStyle(el).color`, returning a resolved `rgb(...)` string that `THREE.Color` can parse.

## Status
Closed

## Follow-up Fix
Also fixed `plasterColor` and `plasterEdgeColor` in all 6 mesh components (`GLTFMesh`, `FBXMesh`, `OBJMesh` in both design and type scenes). These also used `getComputedColor` which returned raw `var(...)` references that `THREE.Color` couldn't parse, causing models to render black.

Added `resolveThreeColor(variable)` helper at line ~37530 that resolves CSS variables via DOM element `color` property and returns a proper `THREE.Color`.
