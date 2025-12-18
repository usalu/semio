---
slug: UI-LEVEL-HIERARCHY
summary: Add window and overlay levels to UI hierarchy
prompt: >-
  Add window and overlay levels to UI hierarchy. The hierarchy is base, window,
  panel, overlay, temporary. All ui elements need to work in all 5 levels with a
  level context, provider and useLevel hook.
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-17T19:03:09.836Z'
iterations:
  - prompt: >-
      Add window and overlay levels to UI hierarchy. The hierarchy is base,
      window, panel, overlay, temporary. All ui elements need to work in all 5
      levels with a level context, provider and useLevel hook.
    date:
      started: '2025-12-17T19:03:27.340Z'
      ended: '2025-12-17T19:23:06.674Z'
    model: claude-opus-4-5
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 671d6b386b077914f2bdbab4e853923102a5903e
    files:
      updated:
        - js/js/sketchpad/elements.tsx:
            lines:
              added: 81
              removed: 15
        - js/js/globals.css:
            lines:
              added: 18
              removed: 3
        - AGENTS.md:
            lines:
              added: 13
              removed: 1
        - README.md:
            lines:
              added: 17
              removed: 0
      created: []
      removed: []
    lines:
      added: 129
      removed: 19
---
# Previously

# Plan

# Changes
