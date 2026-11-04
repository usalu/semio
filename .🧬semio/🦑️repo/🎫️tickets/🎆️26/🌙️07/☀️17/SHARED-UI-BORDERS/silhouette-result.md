# Silhouette Compositor Result

## Changes

Owned UI paths only, relative to `🧰️framework/🔨️modules/🖱️ui/🧱️elements/`:

- `🗂️WindowChrome/🟦️.tsx`: connects existing SVG to root-local shared-edge resolution, retains full original continuous path when all edges win, preserves celebration mask/foreignObject and loading/introduction/waiting paint. Hover can supersede effects. Captured content scroll remeasures fusion. Measured submenu rectangles intersect their visible scroll viewport before producing outline geometry.
- `🗂️WindowChrome/shared-border.ts`: shell-root registry (closest shell/body fallback), frame/edges attribute discovery, zero-inset logical registration, resolver splits, projection to original inset stroke, owner-local SVG paint, pointer/focus/active state, coalesced resize/scroll/mutation/geometry-transition work, restoration and cancellation. Rectangular opt-in frames retain only requested physical sides. Static frame hosts gain temporary relative positioning and restore it. Existing partial ShellScope test fixtures now tolerate missing rootRef through `scope?.rootRef?.current`.
- `🔲️WindowSilhouette/🟦️.tsx`: preserves existing contextMenuFusion metadata through normalization (it was previously discarded before the specialized outline builder could consume it). Existing rectangular/chip geometry formulas otherwise unchanged.
- `🔲️WindowSilhouette/shared-border.ts`: pure snapped coordinate registration/projection helpers.
- Tests: `🗂️WindowChrome/shared-border.test.ts`; `🔲️WindowSilhouette/shared-border.test.ts`, `shared-border.cases.json`, `shared-border.schema.json`.

DOM integration contract remains in `silhouette-api.md`; navbar/footer edges narrow the full-frame marker. Menu/controls agent adoption needs no import. Layout-only group markers never create outlines. Cell dividers remain the canonical CSS owner's single-border implementation; runtime owns outer-frame coincidence.

## Validation

Test-first missing-helper test failed, then passed after implementation. Fusion normalization regression also failed before its fix.

Final combined result: **16 tests pass, 175 assertions** (7 owned tests plus 9 border-core tests). Owned tests cover schema-validated language-neutral fixtures, comparison with installed Three Vector2 interpolation, scaled/reversed partial projection, exact unchanged chip path, metadata preservation, viewport-clipped fusion, reversed registration order, sibling active/hover seam ownership, partial nested hierarchy, navbar bottom-only ownership, and cleanup/restoration.

```text
/home/clstb/.local/share/semio/launcher/bin/repo tools exec --light bun -- test './🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️WindowSilhouette/shared-border.test.ts' './🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.test.ts' './🧰️framework/🔨️modules/🖱️ui/🔨️modules/📏️border-presentation/segments.test.ts'
```

Focused TypeScript source/test check passed after canonical assets became available:

```text
/home/clstb/.local/share/semio/launcher/bin/repo tools exec --light bun -- ./node_modules/typescript/bin/tsc --noEmit --skipLibCheck --target ES2022 --module preserve --moduleResolution bundler --resolveJsonModule --allowImportingTsExtensions --jsx react-jsx --types bun './🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/🟦️.tsx' './🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔲️WindowSilhouette/shared-border.test.ts' './🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.test.ts'
```

Scoped diff inspected; scoped diff whitespace check passed. No generated artifacts retained, no modifying git operations.

## Remaining Integration / Risks

- Lead owns real-browser paint, forced-colors, reduced-motion, clipping, layering, and whole-menu/control suite verification. No browser claims made by this assignment.
- Runtime activation currently requires at least one mounted WindowChrome in a root. Marker-only standalone groups retain their original CSS fallback (no speculative suppression), rather than starting a global observer. This was explicitly allowed by the owner for current integration.
- Shared seams paint once using the winning owner's original inset-one stroke. Thus changing the winning incident owner can move a seam between the incident inset stroke positions; logical coincidence is stable and no parallel duplicate is painted. Browser review should assess this against exact shape preservation versus canonical centerline attachment.
- Degenerate silhouettes whose zero-inset and inset outlines have different edge counts conservatively retain their original outline instead of discarding unrelated paint.
- Arbitrary rotated/skewed hosts are not supported by the approved axis-aligned resolver. Static axis scaling is handled; geometry transitions inside the root are remeasured while active.
- Ticket lifecycle remains with lead/setup.
