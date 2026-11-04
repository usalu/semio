# Compositor Review

Static independent review; all paths relative to UI elements.

1. P1: `WindowChrome/shared-border.ts` overlay SVG width/height attributes lose to descendant icon-sizing selectors in Select/Tabs/ActionGroup. Protect actual CSS dimensions and exclude overlays from icon-only styling.
2. P1: Unclipped owner bounding rectangles can win spans outside an overflow ancestor; winner overlay is clipped while visible incident owner suppresses its span. Clip eligible sources to actual ancestor overflow viewport before resolution.
3. P2: Loading/waiting Action original pseudo-element effect rings remain after frame SVG replacement, including discarded seams/hover. Suppress replaced effect paint while attached and retain SVG effects.
4. P2: Per-owner inset projection creates 2px gaps at sibling/partial-overlap junctions and moves seam on state changes. Canonical shared junction projection/endpoint connection must preserve complete silhouette outline and continuity.

Lead already fixed independent earlier findings: ContextMenu keyboard row reveal, DOM seam system Highlight in forced colors, and last-relevant-cell CSS selectors robust to appended overlay SVG.

Browser probe additionally found the first compositor path attached but not considered visible by Playwright; horizontal path bounding-box height can be zero, so probe must use attachment/state and inspect actual computed paint rather than relying on path visibility alone.
