---
slug: WINDOW-ONLY-BG
prompt: Remove per-element backgrounds so only Window surfaces have a background.
summary: Make UI elements transparent; only windows paint background
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: 2025-12-18T15:34:29.349Z
  finished: 2025-12-18T15:55:58.196Z
commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
model: gpt-5.2-codex
iterations:
  - prompt: Remove per-element level backgrounds so only Window surfaces have a background; make panels and layout chrome transparent; keep temporary surfaces readable.
    model: gpt-5.2-codex
    date:
      started: 2025-12-18T15:39:08.907Z
      ended: 2025-12-18T15:55:14.385Z
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
