---
slug: WINDOW-ONLY-BG
prompt: Remove per-element backgrounds so only Window surfaces have a background.
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-18T15:34:29.349Z'
  finished: '2025-12-18T15:55:58.196Z'
summary: Make UI elements transparent; only windows paint background
commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
model: gpt-5.2-codex
iterations:
  - prompt: >-
      Remove per-element level backgrounds so only Window surfaces have a
      background; make panels and layout chrome transparent; keep temporary
      surfaces readable.
    date:
      started: '2025-12-18T15:39:08.907Z'
      ended: '2025-12-18T15:55:14.385Z'
    model: gpt-5.2-codex
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
    files:
      updated:
        - js/js/sketchpad/elements.tsx:
            lines:
              added: 127
              removed: 146
        - js/js/sketchpad/Sketchpad.tsx:
            lines:
              added: 145
              removed: 233
        - js/js/sketchpad/Tutorials.tsx:
            lines:
              added: 1
              removed: 19
        - js/js/sketchpad/Home.tsx:
            lines:
              added: 269
              removed: 265
        - js/js/sketchpad/Kit.tsx:
            lines:
              added: 24
              removed: 18
        - js/js/sketchpad/Quality.tsx:
            lines:
              added: 9
              removed: 11
        - js/js/globals.css:
            lines:
              added: 24
              removed: 12
        - README.md:
            lines:
              added: 19
              removed: 13
        - AGENTS.md:
            lines:
              added: 28
              removed: 18
        - log/tickets/2025/12/18/WINDOW-ONLY-BG.md:
            lines:
              added: 51
              removed: 0
      created: []
      removed: []
    lines:
      added: 697
      removed: 735
files:
  updated:
    - AGENTS.md:
        lines:
          added: 28
          removed: 18
    - README.md:
        lines:
          added: 19
          removed: 13
    - js/js/globals.css:
        lines:
          added: 24
          removed: 12
    - js/js/sketchpad/Home.tsx:
        lines:
          added: 269
          removed: 265
    - js/js/sketchpad/Kit.tsx:
        lines:
          added: 24
          removed: 18
    - js/js/sketchpad/Quality.tsx:
        lines:
          added: 9
          removed: 11
    - js/js/sketchpad/Sketchpad.tsx:
        lines:
          added: 145
          removed: 233
    - js/js/sketchpad/Tutorials.tsx:
        lines:
          added: 1
          removed: 19
    - js/js/sketchpad/elements.tsx:
        lines:
          added: 127
          removed: 146
    - log/tickets/2025/12/18/WINDOW-ONLY-BG.md:
        lines:
          added: 85
          removed: 0
  created: []
  removed: []
lines:
  added: 731
  removed: 735
---

# Previously

- Multiple UI elements and overlays painted their own backgrounds (panel/temporary/popover/window tokens), resulting in layered filled surfaces.
- GoldenLayout chrome and Sketchpad shell containers also applied background fills.

# Plan

- Keep `Window` as the only component that paints a filled background surface.
- Make all other elements and overlays backgroundless (transparent), relying on borders/blur for separation.
- Update GoldenLayout overrides so chrome is transparent.
- Update root docs to describe the window-only background mechanism.

# Changes

- Made level backgrounds transparent via `getLevelBgClass` and removed per-element background fills from core elements.
- Updated Sketchpad shell and app overlays to remove `bg-base`/`bg-panel`/`bg-temporary` usage.
- Updated GoldenLayout CSS overrides to remove window chrome background fills.
