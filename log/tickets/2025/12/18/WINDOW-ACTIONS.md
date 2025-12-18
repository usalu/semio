---
slug: WINDOW-ACTIONS
prompt: The buttons for every window should be action ui elements.
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-18T15:27:45.865Z'
summary: Use Action UI elements for all window buttons
iterations:
  - prompt: The buttons for every window should be action ui elements.
    date:
      started: '2025-12-18T15:42:02.388Z'
      ended: '2025-12-18T16:12:16.838Z'
    model: gpt-5.2-codex
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 20a57039fec48f8c991d98769aa31e15cbd6859c
    files:
      updated:
        - js/js/sketchpad/Sketchpad.tsx:
            lines:
              added: 168
              removed: 376
        - js/js/sketchpad/elements.tsx:
            lines:
              added: 127
              removed: 137
        - js/js/globals.css:
            lines:
              added: 24
              removed: 25
        - README.md:
            lines:
              added: 21
              removed: 12
        - AGENTS.md:
            lines:
              added: 29
              removed: 18
        - log/tickets/2025/12/18/WINDOW-ACTIONS.md:
            lines:
              added: 55
              removed: 0
      created: []
      removed: []
    lines:
      added: 424
      removed: 568
---

# Previously

GoldenLayout window chrome controls and the splitter add-window overlay used non-Action UI buttons, causing inconsistent interaction styling and tooltip/id behavior.

# Plan

- Render all window-related buttons (window chrome actions and splitter add-window actions) as Action UI elements.
- Ensure GoldenLayout windows forward Action clicks to the layout controls.
- Document the window action control mechanism in README.md and AGENTS.md.

# Changes

- Implemented ActionGroup/ActionGroupItem-based splitter add-window overlay actions.
- Added Action-based window chrome controls for GoldenLayout windows by forwarding Action clicks to GoldenLayout control elements.
- Hid GoldenLayout native control buttons so window actions are presented exclusively as Action UI elements.
- Updated README.md and AGENTS.md to document window action controls.
