---
slug: TUTORIALS-CONSOLIDATION
summary: Consolidate tutorials into single file
prompt: Consolidate tutorials into single file
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.719Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

## Notes

- Consolidated tutorial modules into `js/js/sketchpad/Tutorials.tsx` with regions for types, store, commands, built-ins, and UI.
- Removed the legacy `js/js/sketchpad/tutorials` folder and pointed Sketchpad imports to the unified module.
- Updated `AGENTS.md` tutorial guidance to reflect the single-file setup.
