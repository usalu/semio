---
date:
  created: '2025-11-26T16:43:17.475Z'
  updated: '2025-11-26T16:43:17.475Z'
slug: TUTORIALS-CONSOLIDATION
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Consolidate tutorials into single file
model: claude-sonnet-4.5
prompts: []
commit: unknown
affectedFiles: []
lines:
  added: 0
  removed: 0
---

## Notes
- Consolidated tutorial modules into `js/js/sketchpad/Tutorials.tsx` with regions for types, store, commands, built-ins, and UI.
- Removed the legacy `js/js/sketchpad/tutorials` folder and pointed Sketchpad imports to the unified module.
- Updated `AGENTS.md` tutorial guidance to reflect the single-file setup.
