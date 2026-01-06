---
slug: NAVBAR-PANEL-ORDER
summary: Enforce navbar panel order Details Chat Settings
prompt: Enforce navbar panel order Details Chat Settings
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.909Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Previously

- Navbar panel toggles in Sketchpad apps showed Settings before Details and Chat.

# Plan

- Find every navbar panel definition and verify current ordering.
- Reorder all panel definitions to Details, Chat, Settings.
- Update shared docs to describe the fixed panel order and record changes.

# Changes

- Reordered navbar panel definitions in Home, Kit, Design, and Type apps to Details → Chat → Settings.
- Documented the navbar panel ordering policy in README.md and AGENTS.md.
