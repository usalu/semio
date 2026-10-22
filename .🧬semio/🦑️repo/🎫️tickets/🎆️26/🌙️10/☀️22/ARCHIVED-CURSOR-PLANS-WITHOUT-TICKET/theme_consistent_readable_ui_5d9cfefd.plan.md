---
name: Theme Consistent Readable UI
overview: "Make sketchpad/playground UI theme-consistent: add a theme-aware contrast helper so puzzle 2d (WIRES) node/handle labels are always readable on any fill (fixing dark-fill/dark-text), and fix the remaining hardcoded-color violations across the UI."
todos: []
isProject: false
---

## Problem

In the sketchpad WIRES board, nodes are filled with their kind-catalog color (e.g. `var(--color-dark-8-9)` / `var(--color-gray-700)`), painted by the WASM/Vello renderer. But the label is drawn separately in the JS text overlay using the node's _stroke_ color (theme `emphasized`/dark) regardless of fill:

```6637:6639:puzzle/2d/react/index.tsx
      const style = this.getStyle(node.style, puzzle2dInteractionChromeStyleKey("node", node.id, chrome));
      const family = node.textFontFamily;
      ctx.fillStyle = style.stroke ?? PUZZLE_2D_STYLES_HEADLESS_FALLBACK.node.stroke ?? tokenHex("dark");
```

So a dark catalog fill + dark label = unreadable. There is no contrast/luminance helper anywhere today. The same pattern affects handle labels (line 6708).

## Approach

Auto-compute label color from the resolved node fill luminance, picking a near-black or near-white theme palette token. Resolve the _effective_ fill per node:

- selected/highlighted chrome -> the chrome tint fill (`style.fill`)
- normal + a `nodeKind` catalog color -> that catalog color (this is what WASM actually paints)
- otherwise -> theme node fill (`style.fill`)

Node scene objects have no per-instance color override (only handles do), and `puzzle2dInteractionChromeStyleKey` returns `node` / `node.selected` / `node.highlighted`, so the effective-fill logic above is complete.

## 1. Contrast helper (`ui/styling/js/resolve.ts`)

Add to the `🎨️Resolve` region:

- `relativeLuminance(hex: string): number` - WCAG sRGB relative luminance.
- `readableForegroundHex(backgroundRef: string, lightKey?, darkKey?): string` - resolves `backgroundRef` to hex, then returns `tokenHex(darkKey ?? "dark")` on light backgrounds and `tokenHex(lightKey ?? "light")` on dark backgrounds (luminance threshold ~0.5, cached like `resolveColorHex`).
- Extend the `🧪️Tests` block in the same file (no new test files) covering luminance ordering and that a dark fill yields the light token and vice versa.

## 2. Apply readable labels in puzzle 2d overlay (`puzzle/2d/react/index.tsx`)

- Add a small private helper `nodeLabelFillForOverlay(node, style, chromeKey)` (near `paintTextOverlays`) that returns the effective fill ref: catalog color from `this.kindCatalogsBundle.nodes` by `node.nodeKind` when chromeKey is the base `node`, else `style.fill`; falls back to `themeColorVar("panel")`.
- Replace the node label color line (6639) with `ctx.fillStyle = readableForegroundHex(effectiveFill, ...)` instead of `style.stroke`.
- Replace the handle label color line (6708) similarly, using the handle's effective fill (handle `color` override / `handle.style` fill).
- Import `readableForegroundHex` from `@semio-tech/ui-styling/js/resolve` alongside the existing `themeColorVar`/`tokenHex` imports (line ~27).
- Extend the existing overlay vitest blocks (the `fillText` mock tests around 8034-8546) to assert the label `fillStyle` is the readable token for a dark vs light node fill.

## 3. Fix discrete theme violations (from audit)

- `ui/react/index.tsx` `DialogOverlay` (~7005): `bg-black/50` -> semantic scrim (`bg-overlay` or `bg-foreground/50` per existing token usage) so the modal scrim follows the theme.
- `framework/product/platform/renderer/react/index.tsx` (~3103): success button `border-green-600 text-green-700` -> success semantic tokens (`border-success text-success` / `*-foreground`), matching the `destructive` variant pattern already used nearby.

## Notes / decisions

- The sketchpad kit + metabolism fixture identity-kind fills (`--color-dark-*` / `--color-light*`) are left as-is: step 2's auto-contrast makes them readable in both themes, which is the consistent fix the user asked for. (They are palette swatches by design, not semantic chrome.)
- Edge/relationship labels are not drawn in the overlay, so they are out of scope.
- Verify by running the puzzle 2d vitest project and visually checking the WIRES board in the sketchpad dev server (light + dark).
  </plan>
  <todos>[{"id": "contrast-helper", "content": "Add relativeLuminance + readableForegroundHex (with tests) to ui/styling/js/resolve.ts"}, {"id": "puzzle2d-labels", "content": "Use readable foreground for node + handle labels in puzzle 2d overlay based on effective fill; extend overlay tests"}, {"id": "discrete-fixes", "content": "Fix DialogOverlay bg-black/50 scrim and platform success button green literals to semantic tokens"}, {"id": "verify", "content": "Run puzzle 2d vitest and visually verify WIRES board readability in light + dark"}]</todos>
  </invoke>
