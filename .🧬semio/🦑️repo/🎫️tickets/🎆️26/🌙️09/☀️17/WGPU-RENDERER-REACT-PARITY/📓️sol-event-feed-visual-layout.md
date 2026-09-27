# EventFeed Visual Layout Parity

## Contract and implementation

- Added a neutral EventFeed visual schema/fixture and an actual mounted React plus production-CSS Chromium oracle.
- WGPU now uses one card layout for paint, pointer hit testing, context-menu hits, accepted accessibility rects, follow-scroll content height and clipping.
- The layout carries React's host/card padding and inter-card gap, 22.4 px plain cards, 38.4 px detail cards, 16 px icons, 10 px accepted time labels, and clipped title/detail bands.
- Tone mapping now uses foreground, success, warning and error theme tokens. Fatal shares the error hue and uses semibold weight.
- Added `TextWeight::Medium` as a narrower two-strike synthetic weight for React's `font-medium`; the existing face-keyed Sans/Mono atlas path remains unchanged.

## Verification

The schema validation passed on the first focused React run. The first Chromium attempt stopped at test-only DOM discovery before geometry assertions; the production component rendered five cards and the selector now follows the component's structural card/content/title nodes.

The repaired actual React plus production-CSS Chromium oracle passed with this exact command:

```text
PLAYWRIGHT_BROWSERS_PATH=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long TMPDIR=<ticket>/🗑️generated/sol-event-feed/tmp bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --excludeTaskDependencies -- --run ../../../../🧱️elements/📡️EventFeedHost/🧪️tests/🎨️layout/🟦️.tsx --silent=false --reporter=verbose
```

Receipt: one file and two tests passed; Vitest took 34.85 seconds and Nx took 39.1 seconds. Chromium measured 3.2 px host/card padding, 3.1875 px gap after device-pixel rounding, 38.375/22.375 px detail/plain cards, 16 px icon, title weights 500/600, 10 px time text, 11.2 px detail text, truncation and foreground/success/warning/error/fatal colors.

The focused WGPU source law compiled for 17 minutes 4 seconds but never entered the requested assertion. Shared native compilation stopped first in `semio-s-artifact-stdio-contract` on unresolved `dsl`, followed by eight unresolved `semio_framework_ui_viewport` imports in `semio-framework-os-infinite`. This is compile-not-assertion evidence. Root's full native/UI/WASM gates own the integrated receipt.

## Limits

The browser oracle covers one production theme and viewport. It verifies actual computed card geometry, truncation rules, weights and relative semantic tone identities; theme-token parity across every appearance remains covered by the shared theme-token suites.
