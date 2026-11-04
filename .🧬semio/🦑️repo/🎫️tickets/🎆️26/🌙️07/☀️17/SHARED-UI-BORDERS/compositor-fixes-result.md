# Compositor Review Fixes

## Changed Scope

Only WindowChrome and WindowSilhouette source/test directories, plus this ticket report. No CSS or control edits; no modifying git operations. Existing ticket lifecycle remains with lead.

Relative to `🧰️framework/🔨️modules/🖱️ui/🧱️elements/`:

- `🗂️WindowChrome/shared-border.ts`: protected inline-important SVG width/height/min/max/flex sizing; ancestor overflow client-viewport clipping (axis-specific and scaled); pre-resolution canonical-centerline visibility eligibility; state-independent canonical seam centerlines and junction connectors; runtime effect stamp/class precedence; external celebration SVG mask/foreignObject paint.
- `🗂️WindowChrome/🟦️.tsx`: applies protected sizing to both ordinary and celebrated silhouette overlays. Existing silhouette celebration masks/foreignObjects remain intact.
- `🔲️WindowSilhouette/shared-border.ts`: transform-bearing source geometry, pure clipping/visibility eligibility, indexed/cached junction projection. Original projection helper remains tested as a primitive, but independent inset interpolation no longer determines compositor seam positions.
- Both `shared-border.test.ts` files: converted from Bun to Vitest; expanded focused regressions.
- New neutral fixtures/schema: `🗂️WindowChrome/compositor.cases.json`, `compositor.schema.json`; `🔲️WindowSilhouette/junctions.cases.json`, `junctions.schema.json`.

## Geometry Decisions

Canonical line paint is the midpoint of incident inset paint extremes, independent of state and source order. Opposite sibling inset strokes therefore meet at the logical shared centerline, while same-side nested outlines retain their common original inset. Junctions intersect canonical incident lines. Unequally scaled partial overlaps receive orthogonal endpoint connectors rather than diagonal distortion. Hover can transfer ownership without moving the seam.

Isolated complete silhouettes retain their original full path, including stepped chips. Complete winning owners do not bypass canonical projection when a junction or span changes their paint geometry. Single-side frame endpoints retain original inset coordinates when no incident junction requires connection.

Overflow clipping excludes invisible original inset strokes before resolution. A second geometry-only eligibility sweep also excludes a candidate if canonical centerline movement would put its paint outside its own clipping viewport. This prevents an originally visible-but-near-clipped active source from winning an invisible relocated seam. The sweep has a no-finite-clip fast path and removes candidates monotonically.

## Browser/API Contract

- Registration remains `[data-shared-border-frame]` / `[data-shared-border-edges="top right bottom left"]` and is activated by an existing WindowChrome in the shell.
- Attached frame: `data-shared-border-attached` contains the physical sides.
- Overlay: `svg[data-shared-border-overlay]` has inline `width`, `height`, `min-width`, `min-height`, `max-width`, `max-height`, `flex-shrink` marked `!important`. Window silhouette SVGs use the same protection.
- Paint paths: `[data-shared-border-segment][data-shared-border-state]`; class remains `window-silhouette-border window-silhouette-border-STATE`.
- State combines fallback, valid explicit `data-shared-border-state`, `data-active`, focus, `border-waiting`, `border-loading`, `data-introduced`, and `data-celebrated` according to existing resolver precedence. Pointer hover wins. False-valued effect stamps do not activate effects. Runtime stamp mutations are observed.
- External celebrated paths are inside SVG masks and paint via `foreignObject > .window-silhouette-border-celebrated-fill`; inspect attachment/computed paint, not Playwright path visibility, because horizontal/vertical paths may have zero-area bounding boxes.
- For adjacent 100px unscaled windows at viewport x=0 and x=100: top endpoints meet at `(100,1)` and shared vertical centerline is `x=100`, before and after hover ownership transfer.

## Validation

Test-first helper import failed before implementation. Final focused Vitest run: **2 files passed, 16 tests passed**. Both files now import `vitest`, not `bun:test`.

Lead confirms both tests are now included in the permanent ui-react config. Exact paths:

- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.test.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️WindowSilhouette/shared-border.test.ts`

Focused command:

```sh
/home/clstb/.local/share/semio/launcher/bin/repo tools exec --light bun -- ./node_modules/vitest/vitest.mjs run --config './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️17/SHARED-UI-BORDERS/vitest.config.ts' './🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.test.ts' './🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️WindowSilhouette/shared-border.test.ts'
```

Coverage includes schema validation, installed Three Vector2 reference comparisons, order-independent sibling/nested/scaled junction fixtures, DOM sibling top continuity, hover-stable seam ownership, partial nested continuity, unequal-scale orthogonal connections, overflow-clipped active owners, canonical relocation clipping, important overlay sizing, effect stamps/classes, cleanup/restoration, exact chip path preservation, and prior fusion normalization/viewport measurement.

Focused TypeScript source/test check passed using `--types vitest/globals`; scoped git diff whitespace check passed. Final owned source was inspected, including untracked helper files (which ordinary git diff does not display). No generated output files retained.

## Remaining Integration

Lead owns browser paint verification and CSS original-effect suppression. Both tests are already included in permanent ui-react config per lead confirmation. **Additional CSS action required:** external `[data-introduced="true"]` paints an inset `box-shadow` (styling CSS around line 1397), while `[data-celebrated="true"]::after` paints a conic ring (around line 1355), with additional celebrated leaf-control rules around line 7442 onward. Suppress those replaced original paints for full-frame compositor attachments as well as loading/waiting rings; preserve their source stamps so registry state remains available. Introduced SVG paths retain the pulse class, and external celebrated mask paths now explicitly include `window-silhouette-border-celebrated-mask` plus the conic-fill foreignObject. Hover replaces the SVG effect with hover paint. No stylesheet was edited by this assignment. No browser run was duplicated. Preserve existing axis-aligned-only scope (rotation/skew remain unsupported). In browser verification, inspect celebrated seams at owner SVG/mask viewport boundaries as well as ordinary stroke junctions, since canonical seams can now lie exactly on the owner viewport edge. Existing root-local activation and degenerate-outline fallback policy remains unchanged.
