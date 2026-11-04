# Fractional Pane Boundary Ownership

## Root Cause

Real App windows have fractional layout heights: parent 370.234375px and Utilities chip 22.390625px. `offsetHeight` rounds those to 370 and 22. Silhouette measurement already treated subpixel differences within one pixel as rounding, not scaling; runtime registration instead unconditionally divided by integer offset size. Consequently it registered different logical bottom lines for two outlines that painted the same physical y=415 stroke. Exact coincidence resolution could not merge them, leaving normal paint on top of active/hover paint and tiny junction offsets.

## Fix

- Extracted a shared axis-scale measurement policy into WindowSilhouette's geometry helper and used it for both silhouette measurement and runtime registration.
- Rectangular frame dimensions now retain fractional CSS sizes when unscaled, rather than treating rounded offsets as transforms.
- Actual transformed hosts retain scale-aware registration. Existing unequal-scale, clipping, projection and junction regressions pass.
- Added language-neutral fractional parent/child fixture/schema and an actual runtime ownership test. It failed before the fix (duplicate child bottom=true), then passed.

## Evidence

Focused normal runner: four files, **46 tests passed** (compositor/geometry, actual controlled Mode, actual Window panes). Strict source TypeScript passes.

Real live App Storybook acceptance on port6011 passes: zero gutter, left active silhouette, right hover priority, Utilities exactly at window left/bottom, and no normal Utilities stroke over either parent bottom. SVG screen-space sampling checks actual painted paths rather than only coincident boxes. No page errors; screenshot reviewed. Internal top/right Utilities chip edges remain, while its outer left/bottom edges are shared with the containing window.

No CSS border erasure or clipping workaround was added.
