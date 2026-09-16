# Fem 2D Transform Gumball — Implementation Summary

## Plugin (fem2d editor)

- Added `transform` utility (`transformMove` remains camera pan only).
- Gumball meta layer (`meta:gumball`) with pivot, `gumballConfig`, and selection ids when `transform` is active and selection is non-empty.
- Commands: `translateSelection`, `rotateSelection`, `scaleSelection` (`Emit::amend` coalesce keys `gumball-translate|rotate|scale`), `setTransformGumballFlag`.
- Selection transform resolves nodes/regions from nodes, regions, elements, supports, and loads.
- **Window measures**: `Fem2dPlayApp::window_measures` exposes Transform utility options (Move / Rotate / Scale Axes / Uniform Scale toggles) tagged with `active_utility_id: transform` on model and results windows.
- Transform emits refresh **both** canvas bodies (model + results) on each coalesced amend.

## Framework

- `Canvas2dGumballOverlay` on `Canvas2dHost` reads meta gumball JSON and dispatches **incremental** transform actions on pointer move (`canvas2dGumballTransformStep`); pointer up ends the gesture without a duplicate commit.
- Canvas session skips plugin pointer routing when `transform` utility is active.

## Tests

### Rust (`cargo test -p semio-s-artifact-fem-2d --features component-app-assembly`)

- Gumball meta / translate / coalesce — ok
- `transform_options_group_is_tagged_for_transform_utility` — ok
- `window_measures_include_transform_utility_options` — ok

### TypeScript (renderer react vitest suite `🔬️gumball-transform-delta`)

- Incremental translate and scale step math — registered in `🎚️config/🟦️.ts` elementSuite list.

## Manual check

1. Launch fem2d dev app, select geometry, activate **Transform** in the utility rail.
2. Confirm utility options toggles appear and hide/show gumball handles.
3. Drag a handle — model and results canvases update live during drag.
