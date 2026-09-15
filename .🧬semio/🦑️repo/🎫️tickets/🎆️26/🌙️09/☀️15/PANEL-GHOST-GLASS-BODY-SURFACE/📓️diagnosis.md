# Panel Ghost Glass Body Surface

## Symptom

During window interaction ghost sessions (canvas selection/drag), panel and window chrome marked with `data-dim` fades out, but a frosted glass rectangle remains visible behind the panel body.

## Root cause

`WindowChrome` renders the clipped body fill on a dedicated sibling layer `[data-slot="window-chrome-body-surface"]` (`.ui-glass`, absolutely positioned). Ghost dimming in `🎨️.css` only targets `[data-dim]` and `[data-window-silhouette-border]`. The body content plane (`panel-content` / `pane-body`) had `data-dim`, but the glass fill layer did not, so it stayed at full opacity while siblings were hidden.

## Fix

Stamp `data-dim` on `window-chrome-body-surface` so it participates in the same ghost contract as cap, body, and silhouette border.

## Goal association

`r2602` / first-class hover and selection — interaction ghost chrome must fully clear non-active UI.

## MCP

Repo MCP was unavailable in this session (`ticket_open` / `repo://goals` not in catalog). Ticket folder created manually.
