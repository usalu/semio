---
slug: SIDEPANEL-HUDPANEL-REFACTOR
prompt: >-
  Refactor panels to introduce SidePanel and HudPanel components. SidePanel is
  left/right with scrollable tabs. HudPanel is center. Replace dropdown toggles
  with simple panel toggles. Apps register tabs to panels.
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-20T16:01:35.453Z"
iterations:
  - prompt: >-
      Refactor panels to introduce SidePanel and HudPanel components. SidePanel
      is left/right with scrollable tabs. HudPanel is center. Replace dropdown
      toggles with simple panel toggles. Apps register tabs to panels.
    date:
      started: "2025-12-20T16:02:54.568Z"
      ended: "2025-12-20T16:12:57.496Z"
    model: claude-opus-4-5
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 0d022d52b13d63e4c46cdfe775245a290c4e89a2
    files:
      updated:
        - js/js/sketchpad/shared.ts:
            lines:
              added: 51
              removed: 1
        - js/js/sketchpad/elements.tsx:
            lines:
              added: 308
              removed: 12
        - js/js/sketchpad/Sketchpad.tsx:
            lines:
              added: 257
              removed: 287
        - js/js/sketchpad/locales/en.json:
            lines:
              added: 18
              removed: 0
        - js/js/sketchpad/locales/de.json:
            lines:
              added: 18
              removed: 0
      created: []
      removed: []
    lines:
      added: 652
      removed: 300
---

# Previously

# Plan

# Changes
