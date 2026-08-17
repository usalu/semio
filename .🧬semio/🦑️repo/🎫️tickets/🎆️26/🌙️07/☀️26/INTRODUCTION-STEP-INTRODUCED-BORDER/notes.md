# Introduction Step Introduced Border

## Intent
Introduction info-box `WindowChrome` silhouette should paint as `introduced` (highlight color + pulsing stroke-width), matching stamped `data-introduced` UI targets.

## Change
- Added optional `borderKind` on `WindowChrome` / `WindowChromeSilhouetteBorder`.
- `UIIntroduction` passes `borderKind="introduced"`.
- Vitest asserts silhouette `data-kind="introduced"`, introduced stroke, and `window-silhouette-border-introduced` class.

## MCP
`ticket_open` / `ticket_close` failed with "Cannot call tool before MCP process client is registered" after successful `mcp_auth`. Local ticket folder created manually.
