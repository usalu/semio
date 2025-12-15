---
date:
  created: '2025-12-12T22:52:15.401Z'
  updated: '2025-12-12T22:52:15.401Z'
slug: TABLE-ROW-HEIGHT-FOOTER
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Unify table row and footer bar height
model: claude-opus-4.5
prompts: []
commit: unknown
affectedFiles: []
lines:
  added: 0
  removed: 0
---
# Previously
Table rows use the unit sizing system (`h-large` / `--size-large`) while the shared `Footer` component defaulted to a fixed `20px` height, so table rows and the footer bar did not match.

# Plan
Unify the footer bar height with the unit sizing system and ensure table rows use the same token.

# Changes
- Refactored `Footer` to use `heightKind` (unit-based) and default to `large` (`h-large` / `--size-large`).
- Table rows already use `h-large`, so they now match the footer bar height.
