# PanelTabBar seam (audit finding b)

Base: `🧰️framework/🔨️modules/🖱️ui`.

## Analysis
- Expanded tab rows carried a native `border-b`/`border-t` (`panelAnchorTabBarClass(direction, expanded)`, `panelTabBarClass`, `mobilePanelTabBarClass`). The WindowChrome silhouette outline wraps chips + body and strokes NO line under a chip, so the row border was the only seam under the tabs, while the silhouette inner edge draws the seam beside the chips (gap). Two different owners (native border vs compositor stroke, both at different sub-pixel offsets) for what must be one line; on mobile the row border sat under the panel frame's own outline.
- A native border on the scroller does not scroll, but a compositor overlay (absolute child) inside an `overflow-x-auto` element does.
- Design kept: tab row separated from the body by one line, line also under the active tab (documented behaviour of expanded panel strips; mode-dock chips differ only because they are fused with the body by the silhouette). Adjacent tabs share one `shared-border-cell` divider each (unchanged).

## Fix
- `PanelTabBar` row is now two elements: `data-slot="panel|mobile-panel-tab-frame"` (non-scrolling, `relative flex`, sizing/height, `useSharedBorderFrameRef`, `data-shared-border-frame` + `data-shared-border-edges="bottom|top"`) containing the unchanged `data-slot="…-tabs"` scroller (`overflow-x-auto`, z-40, `flex-auto`, no border, `data-shared-border-group` for cell dividers, drop target, scrollIntoView). The compositor overlay is appended to the frame, so it never scrolls with overflowing tabs.
- Edge rule `panelTabBarEdge(variant, direction, expanded)` (chrome-class-names): collapsed strips: no edge (silhouette owns all sides); expanded: only the content-facing edge (bottom, top for upward/bottom-docked); mobile: bottom. Outer left/right/top are never declared, so they coincide with and are owned by the panel/silhouette/mobile panel frame.
- Class constants: `panelTabBarClass`/`panelAnchorTabBarClass` are frame classes (no border); `panelAnchorTabBarClass` is now a constant not a function; new exports `panelTabBarScrollRowClass`, `panelTabBarEdge` (framework-exports and package source updated).
- Behaviour note: the seam now follows compositor hover state like every other frame (emphasised while the pointer is in the strip).

## Tests
- New `🧱️elements/🧭️PanelTabBar/shared-border.test.tsx` (8 pass, mocked rects as in `shared-border.test.ts`): edge rule table; frame hosts edge, scroller border-free, one `shared-border-cell` per tab and no native edge classes; collapsed has no edge; bottom-dock top edge; exactly one owner paints the strip/body seam and it spans the full strip width; no vertical or top segment from the tab frame while the host frame owns the outer left edge; overflow-scrolled case (scrollWidth 600 vs 200, scrollLeft moved, scroll event): overlay is a child of the frame not the scroller, paint unchanged and still full width; mobile edge.
- Regression: `PanelTabBar`, `WindowChrome` (15), `Tabs` (7), `Canvas` (17+14+3, shared-border-dock now fully passing), `Panel` (7), `ToggleGroup` (22): all green.
- Updated expectations in legacy `🧪️tests/🧪️owned-locale-detector-retirement/🎨️shell-components-1/🟦️.tsx` (border-b/t classes on `panel-tabs` -> frame edge attributes on `panel-tab-frame`). That harness-driven suite (also control-chrome-1, tree-helpers-1) cannot be run directly (needs the ui-react config), so those edits are unrun. No visual render check; sub-pixel coincidence relies on the compositor (frames and silhouette share the same 1px inset paint model).
