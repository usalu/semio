---
slug: SKETCHPAD-LAYOUT-BASE-LEVEL
prompt: >-
  Make sure the global sketchpad has layout base context provider. Currently
  e.g. navbar and footer are transparent.
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-18T16:15:40.759Z'
  finished: '2025-12-18T16:18:19.098Z'
summary: Wrap global Sketchpad layout in base LevelProvider
commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
model: gpt-5.2-codex
iterations:
  - prompt: >-
      Wrap global Sketchpad Layout in LevelProvider(base) so Navbar/Footer use
      bg-base instead of inheriting overlay/window level.
    date:
      started: '2025-12-18T16:16:25.304Z'
      ended: '2025-12-18T16:18:09.794Z'
    model: gpt-5.2-codex
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
    files:
      updated:
        - js/js/sketchpad/Sketchpad.tsx:
            lines:
              added: 254
              removed: 460
        - README.md:
            lines:
              added: 23
              removed: 12
        - AGENTS.md:
            lines:
              added: 30
              removed: 18
        - log/tickets/2025/12/18/SKETCHPAD-LAYOUT-BASE-LEVEL.md:
            lines:
              added: 56
              removed: 0
      created: []
      removed: []
    lines:
      added: 363
      removed: 490
files:
  updated:
    - AGENTS.md:
        lines:
          added: 30
          removed: 18
    - README.md:
        lines:
          added: 23
          removed: 12
    - js/js/sketchpad/Sketchpad.tsx:
        lines:
          added: 254
          removed: 460
    - log/tickets/2025/12/18/SKETCHPAD-LAYOUT-BASE-LEVEL.md:
        lines:
          added: 58
          removed: 0
  created: []
  removed: []
lines:
  added: 365
  removed: 490
---

# Previously

- Global Sketchpad chrome (Navbar/Footer) derives its background from `useLevel()`.
- When Sketchpad was rendered under non-base level providers, Navbar/Footer could incorrectly inherit a non-base level and appear transparent/incorrect.

# Plan

- Wrap the global Sketchpad layout root in `LevelProvider level="base"`.
- Document the invariant in root docs so background expectations stay consistent.

# Changes

- Wrapped the top-level Sketchpad `Layout` in `LevelProvider level="base"`.
- Documented the global base level provider invariant in `README.md` and `AGENTS.md`.
