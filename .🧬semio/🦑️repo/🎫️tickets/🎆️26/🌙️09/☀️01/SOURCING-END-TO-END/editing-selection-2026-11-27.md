# Editing selection journeys: Pool modifiers and Grid marquee (en, de)

Scope: `selection: shift range, ctrl invert and alt subtract in the Pool` and `selection: Grid marquee ...`. Run with `JOURNEY=editing bun journey-editing.ts en de` from the repo root (the kit resolves modules from `process.cwd()`).

## Root causes

1. Pool ctrl invert: the journey pressed `Control`. On macOS a Control-click is the context-menu gesture, so no click reached the Table and the row never toggled. The product maps ctrl and meta alike (`interactionMergeFromModifiers`). The expectation was wrong on macOS; the journey now uses `ControlOrMeta`. No product change.
2. Grid marquee, plain drag: `world3dLeftPressOrbits` makes a plain left press orbit the camera unless the Select utility is armed (documented and unit-tested). The journey never armed it, so the drag orbited. The journey now opens the Grid utility bar and arms Select first.
3. Grid marquee, ctrl subtract (product bug): `finalizeMarqueeSelection` returned early when the merged id set was empty, so a subtractive or invertive marquee that emptied the selection never dispatched `interactionSelect` and the selection stayed. Fixed: commit when the merged set differs from the current selection (new pure helper `world3dMarqueeCommitsMerge`), for instances and components.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/👆️pointer-and-drop/🔶️active-utility/🟦️.ts` (finalize)
- `.../🌐️World3dHost/🔗️instance-selection/🟦️.ts` (`world3dMarqueeCommitsMerge`)
- `.../🌐️World3dHost/🧪️tests/🧩️component/left-press-orbit.test.ts` (unit test, 4 pass with bun test)
- `journey-editing.ts` (ControlOrMeta, arm Select), `probe-grid-selection.ts` (probe, scratch)

No Rust/wasm change, no rebuild needed (the page is served by vite with stat-guard freshness).

## Result
Final editing journey, en and de: 40 PASS, 0 FAIL (`🗑️generated/sel-run5.log`). Both scope steps pass in both languages.
