# Ticket Summary: Pane Toggles and Panel Borders On Interaction

## Objective
Ensure pane toggles (and folded panel/pane chrome toggles) disappear during window interactions (e.g. group rectangle selection) and ensure panel silhouette borders hide completely during ghost sessions.

## Key Changes
1. **WindowChrome & Silhouette Border**:
   - Added `data-dim` to `WindowChromeSilhouetteBorder` root SVG and fallback `<div>`.
   - Added `data-dim` to `WindowChrome` `chipOnly` cap container (`data-slot="window-chrome-chip-cap"`).
2. **Pane & PanelTabBar**:
   - Updated `Pane` component so `data-dim` is stamped when `dimWhenOpen` is true regardless of `effectiveFolded` state.
   - Updated `PanelTabBar` row markup so tab rows always include `data-dim`.
3. **CSS**:
   - Updated `[data-ghost="true"]` rules in `🎨️globals-ui.css` to target `[data-window-silhouette-border]` alongside `[data-dim]`.
4. **Unit Tests**:
   - Updated `GhostProvider` test cases in `📦 index.tsx` to verify that folded pane toggles, folded panel toggles, and panel silhouette borders dim during window interaction ghost sessions.

## Verification
- Unit test suite passed for `GhostProvider` interaction ghosting.
