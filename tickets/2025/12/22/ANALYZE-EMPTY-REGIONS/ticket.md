---
slug: ANALYZE-EMPTY-REGIONS
prompt: Extend the analyze and fix code hook to check and remove empty regions.
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-22T22:16:35.062Z"
iterations: []
---

# Previously

No existing empty-region checks in the code hook.

# Plan

Add empty region detection and fix handling in hooks/code.ts, then document the behavior in README.md and AGENTS.md.

# Changes

- Added empty region detection/removal to hooks/code.ts and documented it in README.md and AGENTS.md.
