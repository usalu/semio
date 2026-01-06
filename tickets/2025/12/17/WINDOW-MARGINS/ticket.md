---
slug: WINDOW-MARGINS
prompt: Every window should have a single unit margin between the window and the border of the canvas or between windows. Every window has a continuous border around it.
summary: Sketchpad windows have 1-unit margins and continuous borders
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: 2025-12-17T18:00:50.599Z
  finished: 2025-12-17T18:12:12.439Z
commit: 7c4820638369104ae259b238d9240f08e429e67e
model: gpt-5.2-codex
iterations:
  - prompt: Every window should have a single unit margin between the window and the border of the canvas or between windows. Every window has a continuous border around it.
    model: gpt-5.2-codex
    date:
      started: 2025-12-17T18:01:09.910Z
      ended: 2025-12-17T18:12:04.382Z
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 7c4820638369104ae259b238d9240f08e429e67e
    bundles:
      "@semio":
        files:
          "":
            sections:
              "": {}
---


# Previously

GoldenLayout windows touched the canvas edge and splitters used a fixed pixel width, so spacing between windows and the canvas was not aligned to the unit sizing system. Window borders were applied inconsistently between GoldenLayout and Canvas-based layouts.

# Plan

- Make Canvas add a 1-unit inner margin so window content never touches the canvas edge.
- Make multi-window containers use a 1-unit gap between windows.
- Make GoldenLayout splitters use 1-unit thickness and keep window borders continuous at the stack level.
- Document the window spacing + border mechanism in README/AGENTS.

# Changes

- Added 1-unit canvas padding and 1-unit window gaps for Canvas-based layouts.
- Updated GoldenLayout splitter sizing to use the unit system and ensured borders remain continuous around each window.
- Added a `Window.kind` switch to avoid nested borders inside GoldenLayout-rendered windows.
- Updated developer documentation to describe the window spacing/border mechanism.
