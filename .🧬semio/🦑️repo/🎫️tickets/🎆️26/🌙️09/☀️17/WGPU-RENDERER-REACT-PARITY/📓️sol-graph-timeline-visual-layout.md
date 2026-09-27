# GraphTimeline Visual Layout Parity

## Contract and implementation

- Added the neutral `Graph Timeline Layout` schema and fixture under `GraphTimelineHost/🧬️schema/🎨️layout` and `GraphTimelineHost/🧫️fixtures/🎨️layout`.
- The actual mounted `GraphTimelineHost` is serialized into Chromium with the repository's production Tailwind CSS and embedded WOFF2 assets. The accepted measurements are 3.2 px host padding, 24 px rows, an 80.9375 px intrinsic label track, a 104 px graph column, a 20 px author origin for lane 1, 16 px avatars with 8 px overlap, and an inert description cell.
- WGPU now computes one `GraphTimelineLayout` for paint, pointer hit testing, accepted accessibility controls and the scroll viewport. It replaces the 28%-or-96 px label track and `control_height * 1.33` rows with content-derived label width and `tree_row_height`.
- Author stacks begin at `max(laneX - 12, 0)` inside the graph track. `mutationLevel` is decoded and painted as the same uppercase semantic badge family as React. The selectable labels-plus-graph boundary continues to exclude descriptions.
- The shared author fixture/schema now records the actual production-CSS 16 px avatar, 8 px overlap and 8 px advance.

## Verification

Focused command:

```text
PLAYWRIGHT_BROWSERS_PATH=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_LEVEL=long TMPDIR=<ticket>/🗑️generated/sol-graph-timeline/tmp bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --excludeTaskDependencies -- --run '../../../../🧱️elements/🌳️GraphTimelineHost/🧪️tests/🎨️layout/🟦️.tsx' --silent=false --reporter=verbose
```

Result: 1 file passed, 2/2 tests passed; Vitest 49.52 s, Nx 57.9 s.

The first browser run was intentionally fail-first: it measured `contentLeft = 3.1875` against the unrounded 3.2 px token and measured the actual avatar overlap as 8 px. The contract now tolerates Chromium's subpixel edge rounding while retaining exact computed token values.

The WGPU source law `shared_layout_contract_drives_paint_hit_accessibility_and_mutation_badge_geometry` consumes the same fixture and checks geometry, selectable/inert partitioning, accepted AX rects and semantic warning paint. Its focused native result is pending the shared Rust build queue; root's full native/UI/WASM gates own the integrated receipt.

## Limits

The Chromium receipt proves the actual component, production CSS and font assets at the fixture viewport. It does not claim exhaustive pixel identity for every label string, viewport, theme or device scale. WGPU uses a deterministic intrinsic-width estimate so paint, pointer and AT retain one boundary without depending on a mutable glyph atlas.
