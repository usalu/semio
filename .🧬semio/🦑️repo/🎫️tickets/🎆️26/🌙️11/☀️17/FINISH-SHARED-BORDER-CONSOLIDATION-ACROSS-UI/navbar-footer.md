# Navbar / Footer shared border

## Root cause
1. Band geometry: `--navbar-height` / `--footer-height` is 28.8px but `NavbarBandBody` wrapped every item (and centered item) in `h-medium` (22.4px), vertically centered. Controls therefore floated 3.2px inside the band; their own bottom/top edges could never coincide with the band's edge, so the compositor (which dedupes only coincident segments) painted two lines: the band's bottom hairline and the control's own, offset by 3.2px.
2. Product-level: the course `NavigationControls` and `Segments` (language switch) were plain `div.border.border-normal` groups, not shared-border frames/cells, so they drew an unmanaged border and had no compositor ownership.
3. Even once flush, a framed control touching the band's open edge (navbar top, footer bottom) would paint its own outer edge there, as the band draws none.

## Fix (at the source)
- `🧱️elements/🔝️Navbar/🟦️.tsx`: item wrappers and centered items are `h-full` (band height), carry `data-slot="navbar-item"`, centered overlay row is `items-stretch`. Footer reuses `NavbarBandBody`, so it is mirrored automatically.
- `🎨️styling/🖌️ui/🎨️.css`: in navbar/footer, framed controls (and their direct parents) `align-self: stretch`, framed controls `height: auto; max-height: none`, overriding `h-medium` so top/bottom coincide with the band edges.
- `🧱️elements/🗂️WindowChrome/shared-border.ts`: new `flushWithUndrawnParentEdge`; a frame side lying exactly on a side its nearest ancestor frame does not draw (navbar top, footer bottom) paints nothing. Coincident bottom/top segments with the band are deduped by the existing resolver (one owner); adjacent groups' facing edges were already deduped by it.
- Course: `🚏️navigation/🟦️.tsx` and `🪟️chrome/🟦️.tsx` (`Segments`) groups are now `data-shared-border-frame` with `data-shared-border-cell` buttons (internal single hairline dividers via existing CSS). No product-level CSS re-adds borders (`chrome-and-home` css has none for navbar). Einführung chip and status chip are borderless.

## Tests
- `🗂️WindowChrome/shared-border.test.ts`: added "band controls flush with the navbar or footer ..." (top open edge suppressed, bottom kept, inset and footer mirror) and "navbar and footer items span the band height ..." (source + CSS assertions).
- Ran `npx vitest run` over WindowChrome, ToggleGroup, Toggle, styling, border-presentation: 27 passed, 2 failed, both environmental and not caused by this change: `shared-border.test.ts` StrictMode test (`window is not defined`, needs the repo's DOM environment) and `segments.test.ts` browser case (Playwright chromium not installed).
- `repo test` for the course product produced no output within the time-box; course tests not verified. No visual (browser) verification was possible.
- Note: `panel-anchor` assumes a centered `h-medium` row; it is relative to band center so unaffected, but the Navbar docstring still mentions it.
