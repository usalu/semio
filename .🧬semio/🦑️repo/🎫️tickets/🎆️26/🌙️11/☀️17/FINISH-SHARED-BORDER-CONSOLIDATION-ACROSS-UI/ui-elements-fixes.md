# UI elements shared-border fixes

Base: `🧰️framework/🔨️modules/🖱️ui`. Source audit: `audit-ui-elements.md`.

## Verdicts and fixes
- (a) Layout mobile panel `chrome-frame`: REAL. The frame was a non-compositor 4-side native border directly under the navbar's compositor-owned bottom edge (2px seam). Fix: removed the `chrome-frame` overlay div, the `shellChromeFrameLayerClass` constant and its exports (framework-exports, package source, test harness, chrome-classes docstring), and the `[data-slot="chrome-frame"]` CSS. `LayoutMobilePanel` now marks its `PanelGhostRoot` as `data-shared-border-frame` with `useSharedBorderFrameRef`; the compositor overlay (all four edges, pointer-hover aware since the host receives pointer events) is deduped against the navbar bottom segment. The panel-hover CSS comment region and `control-chrome-1` test were updated (no `chrome-frame` anywhere in the CSS; two `shellChromeFrameLayerClass` assertions removed).
- (b) PanelTabBar row edges: NOT CHANGED, not a confirmed double. Tab dividers are already single `shared-border-cell` hairlines, the row edge is the only line at the strip/body junction (collapsed bars already defer all outer edges to the silhouette), and the row is an `overflow-x-auto` scroller: a compositor overlay inside a scroller would scroll away from the edge once tabs overflow. Making it a frame needs a non-scrolling wrapper plus a render check of the silhouette junction; left as follow-up rather than risk a regression.
- (c) Canvas `app-mode-nav`: REAL (mechanism inconsistency). Now a compositor frame `data-shared-border-edges="bottom"` with `borderNormalBottomClass` (normal token) and `useSharedBorderFrameRef`, same pattern as Navbar.
- (d) HistoryTable: REAL. Removed both per-cell `border-b border-[var(--border)]`; one `data-slot="history-row-divider"` per row (grid-spanning, `borderNormalBottomClass`, pointer-events-none), omitted on the last row so nothing doubles the surrounding frame.
- (e) Table last row in a frame: REAL. CSS: `:is([data-shared-border-frame], popover-content, dialog-content) table > tbody > tr:last-child { border-bottom-width: 0 }`.
- (f) Dead selectors: `side-panel-tabs`, `mobile-panel-tabs`, `accordion-item` removed (no emitter anywhere in the repo). `command-input-wrapper` (emitted by Command) and `table tbody tr` / `thead.border-b` kept.

## Tests
- `🗂️WindowChrome/shared-border.test.ts`: added "elements defer their edges to the shared mechanism instead of stacking native ones" (source/CSS assertions for a, c, d, e, f). Ran jsdom: 15 passed.
- Canvas `shared-border-dock.test.tsx`: 8 pass, 9 fail with `react-resizable-panels` `n is not a constructor` (jsdom lacks a ResizeObserver-class; unrelated to these files).
- `control-chrome-1` suite is only reachable via the ui-react vitest config, which refuses direct runs (policy error); not run. `tsc --noEmit` showed no errors in touched files. No visual render check.
