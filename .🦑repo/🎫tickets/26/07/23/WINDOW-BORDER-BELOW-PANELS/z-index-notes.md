# Window Border Below Panels

## Problem
Window silhouette SVG uses `z-[40]` inside `[data-slot="mode-dock-stack"]`, which is only `relative` (no z-index). Layout deliberately left the canvas column without a stacking context so `[data-introduction-elevated]` can rise above the veil. Consequence: silhouette `z-[40]` participates in the same stacking context as floating `Panel` (`zIndex: 20`) and paints above panels.

## Fix
Give `[data-slot="mode-dock-stack"]` `z-window` so it forms a stacking context at `--z-window` (5), below `--z-panel` / Panel's default 20. Internal silhouette `z-[40]` stays above dock chrome but cannot escape above panels. `[data-introduction-elevated]` still overrides the stack to `z-tutorial + 1` via `!important`.
