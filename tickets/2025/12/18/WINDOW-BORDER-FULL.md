---
slug: WINDOW-BORDER-FULL
prompt: >-
  Every window should have a full border around it. Currently the bottom and
  right border are missing.
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-18T15:40:32.088Z"
  finished: "2025-12-18T16:01:54.858Z"
summary: Fix missing bottom/right window borders
commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
model: gpt-5.2-codex
iterations:
  - prompt: >-
      Every window should have a full border around it. Currently the bottom and
      right border are missing.
    date:
      started: "2025-12-18T15:40:50.516Z"
      ended: "2025-12-18T15:44:21.803Z"
    model: gpt-5.2-codex
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
    files:
      updated:
        - js/js/globals.css:
            lines:
              added: 13
              removed: 11
        - README.md:
            lines:
              added: 15
              removed: 9
        - AGENTS.md:
            lines:
              added: 26
              removed: 16
        - log/tickets/2025/12/18/WINDOW-BORDER-FULL.md:
            lines:
              added: 57
              removed: 0
      created: []
      removed: []
    lines:
      added: 111
      removed: 36
  - prompt: The windows have no border now.
    date:
      started: "2025-12-18T15:51:19.817Z"
      ended: "2025-12-18T15:52:19.552Z"
    model: gpt-5.2-codex
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
    files:
      updated:
        - js/js/globals.css:
            lines:
              added: 22
              removed: 12
        - README.md:
            lines:
              added: 16
              removed: 10
        - AGENTS.md:
            lines:
              added: 28
              removed: 18
        - log/tickets/2025/12/18/WINDOW-BORDER-FULL.md:
            lines:
              added: 88
              removed: 0
      created: []
      removed: []
    lines:
      added: 154
      removed: 40
  - prompt: The windows have no border now.
    date:
      started: "2025-12-18T15:53:28.711Z"
      ended: "2025-12-18T15:57:38.721Z"
    model: gpt-5.2-codex
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
    files:
      updated:
        - js/js/globals.css:
            lines:
              added: 25
              removed: 13
        - log/tickets/2025/12/18/WINDOW-BORDER-FULL.md:
            lines:
              added: 110
              removed: 0
      created: []
      removed: []
    lines:
      added: 135
      removed: 13
  - prompt: >-
      Fix GoldenLayout borders disappearing due to invalid CSS and record final
      ticket updates.
    date:
      started: "2025-12-18T15:58:55.376Z"
      ended: "2025-12-18T16:01:06.676Z"
    model: gpt-5.2-codex
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
    files:
      updated:
        - js/js/globals.css:
            lines:
              added: 23
              removed: 11
        - log/tickets/2025/12/18/WINDOW-BORDER-FULL.md:
            lines:
              added: 136
              removed: 0
      created: []
      removed: []
    lines:
      added: 159
      removed: 11
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
          added: 23
          removed: 11
    - log/tickets/2025/12/18/WINDOW-BORDER-FULL.md:
        lines:
          added: 138
          removed: 0
  created: []
  removed: []
lines:
  added: 208
  removed: 42
---

# Previously

- GoldenLayout window stacks were framed using an outer border while the GoldenLayout root uses clipped layout containers.
- Bottom and right frame edges could disappear due to clipping/rounding at the container boundary.

# Plan

- Keep semantic window borders but ensure the frame is rendered inside the window surface so it cannot be clipped.
- Update GoldenLayout styling to use an inset 1px stroke for the stack container.
- Document the window border mechanism in root dev docs.

# Changes

- Updated `js/js/globals.css` so GoldenLayout stack windows render the frame as an inset stroke (via inset shadow) instead of an outer border.
- Adjusted `js/js/globals.css` to render the inset stroke via a `::after` overlay frame after the shadow-based frame was not visible.
- Fixed a broken `.lm_item.lm_stack` CSS block (missing closing brace) and consolidated the GoldenLayout stack frame to a single `::after` inset stroke implementation.
- Restored GoldenLayout chrome and content backgrounds to use the window/base background tokens (instead of `transparent`) while keeping the inset stack frame.
