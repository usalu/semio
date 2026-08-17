# UI Surface System Retention Audit

## Snapshot

- Definition: `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌈️Surface/🟦️component.tsx`, SHA-256 `47c2b80a5d228626e10fe43ec20f011b3eee9cccad25c3041cc71d2b17e5aadb`, clean.
- No co-located production source conflict was found. Glass/Provider stories are excluded consumer evidence.
- Shared React barrel after the Chrome registrar: SHA-256 `0f8def42b5703b2ab00bd31f6e7b242e334ea9f60fdd9a5d35c1a88fdf8fa401`.

## Facets and Production Closure

- Depth/z-order context (`Level`, provider/hook, z-class resolution) is shared by Select, ActionGroup, ButtonGroup, ToggleGroup, PanelTabBar, Layout, Panel, Canvas, Window, OS ShellHost, and the infinite-world renderer.
- Surface scope/fill composition is shared by Select, Canvas, Dialog, Layout, Footer, Navbar, Popover, Window, UIDialog, and ClassNames.
- Active-surface pointer/focus coordination is shared by Canvas, Panel, PanelTabBar, and Window.
- `LEVELS` is story/test-facing only; several exported type names are implementation contracts rather than independent consumers. Export visibility alone is not a semantic responsibility.

## Disposition

Retain the coherent Surface system for this wave. Its three facets share the same surface-depth/fill/activation invariants and have broad interdependent production fanout; splitting them casually would create circular or duplicated surface state. A later public-surface tightening lease may privatize story/test-only constants and types after updating those fixtures, but no zero/one-consumer implementation can be safely removed in isolation from the current graph.
