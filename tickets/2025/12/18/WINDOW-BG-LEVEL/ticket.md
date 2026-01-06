---
slug: WINDOW-BG-LEVEL
prompt: "Fix window background colors: Window component now provides LevelProvider level=window and uses bg-window; GoldenLayout chrome uses var(--window) instead of var(--base)."
summary: Fix window background colors via useLevel
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: 2025-12-18T15:11:24.199Z
  finished: 2025-12-18T15:29:35.275Z
commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
model: gpt-5.2-codex
iterations:
  - prompt: "Fix window background colors: Window component now provides LevelProvider level=window and uses bg-window; GoldenLayout chrome uses var(--window) instead of var(--base)."
    model: gpt-5.2-codex
    date:
      started: 2025-12-18T15:22:03.609Z
      ended: 2025-12-18T15:29:19.399Z
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
    bundles:
      "@semio":
        files:
          "":
            sections:
              "": {}
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
