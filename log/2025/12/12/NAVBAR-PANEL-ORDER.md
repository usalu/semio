---
date:
  created: '2025-12-12T18:20:44.600Z'
  updated: '2025-12-12T18:20:44.600Z'
slug: NAVBAR-PANEL-ORDER
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Enforce navbar panel order Details Chat Settings
model: claude-opus-4.5
prompts: []
commit: unknown
affectedFiles: []
lines:
  added: 0
  removed: 0
---
# Previously

- Navbar panel toggles in Sketchpad apps showed Settings before Details and Chat.

# Plan

- Find every navbar panel definition and verify current ordering.
- Reorder all panel definitions to Details, Chat, Settings.
- Update shared docs to describe the fixed panel order and record changes.

# Changes

- Reordered navbar panel definitions in Home, Kit, Design, and Type apps to Details → Chat → Settings.
- Documented the navbar panel ordering rule in README.md and AGENTS.md.
