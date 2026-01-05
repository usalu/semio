---
slug: SKETCHPAD-ALL-PANELS-BG
prompt: All panels should have panel background. Currently I only see the border of e.g. details or settings panel in home. Toolbar panel is transparent but should have the panel background.
summary: Ensure all panels paint panel background
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: 2025-12-18T18:16:24.765Z
  finished: 2025-12-18T18:18:43.801Z
commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
model: gpt-5.2-codex
iterations:
  - prompt: Ensure all panels paint bg-panel, including toolbar; enforce panel level context globally for Panel and toolbar surfaces.
    model: gpt-5.2-codex
    date:
      started: 2025-12-18T18:16:49.866Z
      ended: 2025-12-18T18:18:33.484Z
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

- The shared `Panel` component did not enforce the panel level background, so panels could appear as border-only.
- The toolbar surface was rendered without a panel background, so it remained transparent.

# Plan

- Make the shared `Panel` component scope its subtree with `LevelProvider level="panel"` and paint `bg-panel`.
- Treat the toolbar surface as a panel: render it under `LevelProvider level="panel"` and paint `bg-panel`.
- Align root docs to state that all panel surfaces use panel level background.

# Changes

- Restored `LevelProvider level="panel"` and `bg-panel` painting for the shared `Panel` component.
- Rendered the Sketchpad toolbar under `LevelProvider level="panel"` and applied `bg-panel` to its container.
- Updated `AGENTS.md` to document panel background enforcement.
