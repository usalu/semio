# Research Findings: Pane Toggles and Panel Borders Visibility on Interaction

## Problem Statement
When interacting with the window (e.g. group rectangle selection, canvas panning, dragging), the pane toggles and panel borders wrongly stay visible instead of hiding alongside open panel/pane chrome.

## Root Cause Analysis

1. **Folded Pane & Panel Toggles Excluded from Ghost Dimming**:
   - In `Pane` (`🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx`), line 8701: `{...(dimWhenOpen && !effectiveFolded ? { "data-dim": true } : {})}` explicitly omitted `data-dim` when `effectiveFolded` was true.
   - In `WindowChrome` (`📦 index.tsx`), when `chipOnly` was true (folded pane/panel chip), `<div data-slot="window-chrome-chip-cap"...>` omitted `data-dim`.
   - In `PanelTabBar` / `PanelChromeTabBar` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️PanelTabBar/🟦️component.tsx`), line 494: `const ghostDim = showActiveColor;` omitted `data-dim` when `showActiveColor` was false (folded panel toggles).
   - As a result, during window interaction (when `GhostProvider` sets `data-ghost="true"`), open panels/panes dimmed, but folded pane toggles and panel toggles stayed visible.

2. **Panel Silhouette Borders Lacking Ghost Dimming**:
   - `WindowChromeSilhouetteBorder` (`📦 index.tsx`) renders the SVG containing the U-cutout border for panels, panes, and windows (`<svg data-window-silhouette-border...>`).
   - `WindowChromeSilhouetteBorder` lacked the `data-dim` attribute on its root `<svg>` and fallback `<div>`.
   - CSS rule `[data-ghost="true"][data-ghost-region] { border-color: transparent !important; }` in `🎨️globals-ui.css` only affects HTML CSS `border-color`, whereas SVG silhouette borders use `<path stroke="...">`.
   - As a result, panel silhouette SVG borders remained fully visible during window interactions.

## Proposed Remediation Plan

1. **Enable Ghost Dimming on Pane & Panel Toggles**:
   - Modify `Pane` in `📦️index.tsx` so `data-dim` is always applied when `dimWhenOpen` is true, regardless of `effectiveFolded` state.
   - Modify `WindowChrome` `chipOnly` cap container to include `data-dim`.
   - Modify `PanelTabBar` so tab rows always include `data-dim`.

2. **Enable Ghost Dimming on Panel Silhouette Borders**:
   - Add `data-dim` to `WindowChromeSilhouetteBorder` SVG and pending `<div>` elements.
   - Update `🎨️globals-ui.css` to ensure `[data-window-silhouette-border]` and `[data-dim]` fade out (`opacity: 0 !important;`) when `[data-ghost="true"]` is active.

3. **Update Unit Tests**:
   - Update `GhostProvider` test suite in `📦 index.tsx` to assert that folded pane toggles, folded panel toggles, and panel silhouette borders dim when interaction ghosting is active.
