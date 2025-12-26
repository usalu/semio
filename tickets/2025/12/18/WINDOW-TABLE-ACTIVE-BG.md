---
slug: WINDOW-TABLE-ACTIVE-BG
prompt: >-
  Every app has windows and there is always one active window. Make sure that
  the background of the table is set to active background color.
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-18T15:34:11.986Z"
  finished: "2025-12-18T16:32:26.549Z"
summary: Use active window background for table
commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
model: gpt-5.2-codex
iterations:
  - prompt: >-
      Every app has windows and there is always one active window. Make sure
      that the background of the table is set to active background color.
    date:
      started: "2025-12-18T16:02:05.033Z"
      ended: "2025-12-18T16:32:16.412Z"
    model: gpt-5.2-codex
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
    files:
      updated:
        - js/js/sketchpad/elements.tsx:
            lines:
              added: 145
              removed: 161
        - js/js/sketchpad/Kit.tsx:
            lines:
              added: 61
              removed: 39
        - js/js/sketchpad/Sketchpad.tsx:
            lines:
              added: 291
              removed: 626
        - README.md:
            lines:
              added: 35
              removed: 12
        - AGENTS.md:
            lines:
              added: 49
              removed: 47
        - log/tickets/2025/12/18/WINDOW-TABLE-ACTIVE-BG.md:
            lines:
              added: 63
              removed: 0
      created: []
      removed: []
    lines:
      added: 644
      removed: 885
files:
  updated:
    - AGENTS.md:
        lines:
          added: 49
          removed: 47
    - README.md:
        lines:
          added: 35
          removed: 12
    - js/js/sketchpad/Kit.tsx:
        lines:
          added: 61
          removed: 39
    - js/js/sketchpad/Sketchpad.tsx:
        lines:
          added: 291
          removed: 626
    - js/js/sketchpad/elements.tsx:
        lines:
          added: 145
          removed: 161
    - log/tickets/2025/12/18/WINDOW-TABLE-ACTIVE-BG.md:
        lines:
          added: 69
          removed: 0
  created: []
  removed: []
lines:
  added: 650
  removed: 885
---

# Previously

- The kit app renders a table inside a multi-window layout, but there was no explicit active-window state for the app.
- The table header background was not guaranteed to match the window surface background when rendered inside GoldenLayout windows.

# Plan

- Track `activeWindow` for the kit app and keep it valid across window kinds.
- Highlight the active window surface and make the table background follow the active window background.
- Document the active-window + background mechanism in root docs.

# Changes

- Updated `LayoutCanvas` to apply an active-window background tint to the active window root.
- Updated the kit app multi-window entrypoint to maintain and pass `activeWindow` into `LayoutCanvas`.
- Updated the shared `Table` component to inherit its surface background so it matches the window surface.
