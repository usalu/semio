# Mode separator reconciliation

## Decision
Abutting frames share one border line (design.md points 2, 7, 8), so the dock has no separator and no canvas inset. React already encodes this (commit 3d62b2ef22c: `ResizableHandle` is 0 wide, `MODE_CANVAS_INSET_CLASS = "p-0"`, stack body `p-0`, asserted by `shared-border-dock.test.tsx`). The shared fixture was the stale party. Document content still starts below the chip row by the frame clearance (design point 7); the old silhouette-era negative margin that cancelled that padding is gone, so the oracle now expects margin 0.

## Files changed
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/📐️dock-axis-geometry/🔣️.json`: `separatorToken`/`canvasInsetToken`/`windowBodyInlineInsetToken` replaced by `separatorPixels: 0`, `canvasInsetPixels: 0`, `windowBodyInlineInsetPixels: 0`; `oracleSample.tokenPixels` became `separatorPixels: 0`; stacks are flush (left 0,0,280x500; right-top 280,0,520x350; right-bottom 280,350,520x150).
- `…/🧪️tests/📐️dock-axis-geometry/🟦️.ts`: solver reads the new fields.
- `…/🧪️tests/📚️storybook-new-stories/🟦️.ts`: Mode test renamed "Mode abuts frames on one shared border line before distributing nested React panel extents"; tokens come from the fixture, handle thickness 0, body margin 0.
- Rust: `…/🛰️Dock/🧪️tests/🔬️wgpu-unit/🧭️drag-merge-mobile-and-focus/🦀️.rs`: test renamed `abutting_axis_geometry_matches_the_shared_react_fixture`, reads new fields, asserts 0/0, dropped the `theme.gap_standard == token` assertion, added live-frames-equal-fixture check. No Rust production change was needed (live frames already abut).

## Verified by running
- Story test "Mode abuts frames..." : 1 passed (dev Storybook, ui scope, port 6090; I started and stopped it).
- Vitest (compositor-vitest.sh): `shared-border-dock` + `mode` filter 17 passed, 1 skipped; `Canvas Resizable dock` filter 3 files, 36 passed, 1 skipped.
- bun test of dock-axis-geometry: 2 pass.
- Rust: `semio-renderer-wgpu-dock-laws` 76 passed; `semio-renderer-wgpu-dock` 2 passed.

## Not run
Full Playwright suite (only the Mode test); the TiledMapHost image-render failure is unchanged and out of scope (needs wasm). The Shell parity test reads only `panelChrome` from the fixture (unchanged), not compiled/run here.
