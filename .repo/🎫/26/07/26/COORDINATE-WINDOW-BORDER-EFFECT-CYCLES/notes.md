# Coordinate Window Border Effect Cycles

## Problem
Each stamped element started its own CSS animation, so loading/introduced/celebrate borders on different windows pulsed out of phase.

## Fix
- Registered inheritable phase tokens (`--loading-border-angle`, pulse opacities, dash offsets, `--introduced-border-width`, `--celebrate-border-padding`).
- Moved all border phase animations to unlayered `:root` (one document clock per effect family).
- Consumers paint inherited vars only (element rings, silhouettes, CelebrateContent).
- 3D `CelebratingConicMaterial` uses `document.timeline.currentTime` for phase lock with CSS.

## MCP
`ticket_open` / `ticket_close` failed with "Cannot call tool before MCP process client is registered" after `mcp_auth`. Local ticket folder created manually.
