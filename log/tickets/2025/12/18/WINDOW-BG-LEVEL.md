---
slug: WINDOW-BG-LEVEL
summary: Fix window background colors via useLevel
prompt: >-
  Fix window background colors: Window component now provides LevelProvider
  level=window and uses bg-window; GoldenLayout chrome uses var(--window)
  instead of var(--base).
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-18T15:11:24.199Z'
  finished: '2025-12-18T15:29:35.275Z'
commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
model: gpt-5.2-codex
iterations:
  - prompt: >-
      Fix window background colors: Window component now provides LevelProvider
      level=window and uses bg-window; GoldenLayout chrome uses var(--window)
      instead of var(--base).
    date:
      started: '2025-12-18T15:22:03.609Z'
      ended: '2025-12-18T15:29:19.399Z'
    model: gpt-5.2-codex
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
    files:
      updated:
        - js/js/sketchpad/elements.tsx:
            lines:
              added: 111
              removed: 121
        - js/js/globals.css:
            lines:
              added: 8
              removed: 8
        - README.md:
            lines:
              added: 11
              removed: 8
        - AGENTS.md:
            lines:
              added: 14
              removed: 4
        - log/tickets/2025/12/18/WINDOW-BG-LEVEL.md:
            lines:
              added: 48
              removed: 0
      created: []
      removed: []
    lines:
      added: 192
      removed: 141
files:
  updated:
    - AGENTS.md:
        lines:
          added: 14
          removed: 4
    - README.md:
        lines:
          added: 11
          removed: 8
    - js/js/globals.css:
        lines:
          added: 8
          removed: 8
    - js/js/sketchpad/elements.tsx:
        lines:
          added: 111
          removed: 121
    - log/tickets/2025/12/18/WINDOW-BG-LEVEL.md:
        lines:
          added: 62
          removed: 0
  created: []
  removed: []
lines:
  added: 206
  removed: 141
---

# Previously

- Window backgrounds were still using base-level styling in some window contexts, so window surfaces did not visually separate from the canvas.
- GoldenLayout window chrome (header/tab/buttons) was still forced to the base background token.

# Plan

- Make `Window` the canonical boundary for the `window` level.
- Ensure GoldenLayout window chrome uses the window background token.
- Document the background level mechanism in root docs.

# Changes

- Updated `js/js/sketchpad/elements.tsx` `Window` to scope its subtree via `LevelProvider level="window"` and apply the window background level.
- Updated `js/js/globals.css` GoldenLayout overrides so header/tabs/window buttons/stack surfaces use the window background token.
