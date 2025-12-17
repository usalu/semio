---
slug: TABLE-ROW-HEIGHT-FOOTER
summary: Unify table row and footer bar height
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.914Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

Table rows use the unit sizing system (`h-large` / `--size-large`) while the shared `Footer` component defaulted to a fixed `20px` height, so table rows and the footer bar did not match.

# Plan

Unify the footer bar height with the unit sizing system and ensure table rows use the same token.

# Changes

- Refactored `Footer` to use `heightKind` (unit-based) and default to `large` (`h-large` / `--size-large`).
- Table rows already use `h-large`, so they now match the footer bar height.
