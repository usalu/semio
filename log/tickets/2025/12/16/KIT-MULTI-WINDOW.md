---
slug: KIT-MULTI-WINDOW
summary: Extend kit app to multi-window with table and diagram views
prompt: >-
  The kit app should be extended to a multi-window app like the design app. It
  should have two window kinds: table and diagram. The table window is the
  current canvas. The diagram window should show a forced layout graph of all
  the artifacts of the kits and their relationships. There are two different
  kind of relationships: part of (children of parents, artifacts inside folders)
  and references (such as between a type and a design if there is a piece inside
  of the design with that type). Hover and selection of artefacts are again
  shared among the windows.
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-16T17:06:07.971Z'
commit: '0000000000000000000000000000000000000000'
iterations:
  - prompt: >-
      The kit app should be extended to a multi-window app like the design app.
      It should have two window kinds: table and diagram. The table window is
      the current canvas. The diagram window should show a forced layout graph
      of all the artifacts of the kits and their relationships. There are two
      different kind of relationships: part of (children of parents, artifacts
      inside folders) and references (such as between a type and a design if
      there is a piece inside of the design with that type). Hover and selection
      of artefacts are again shared among the windows.
    date:
      started: '2025-12-15T23:29:43.849Z'
    model: claude-opus-4-5
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
  - prompt: >-
      Fix issues: Table window is empty. All nodes should be circles like Design
      diagram nodes with artifact icons. Layout is not a draggable force layout
      - add force slider to diagram settings. Migrate existing kit app tests and
      ensure they pass.
    date:
      started: '2025-12-16T00:01:48.177Z'
    model: claude-opus-4-5
    commit: 06f516fab841773c499e5e7b9dbcdcfa63ad94cb
    files:
      updated:
        - path: AGENTS.md
          lines:
            added: 131
            removed: 5
        - path: js/js/sketchpad/Kit.tsx
          lines:
            added: 131
            removed: 5
        - path: js/js/sketchpad/locales/de.json
          lines:
            added: 131
            removed: 5
        - path: js/js/sketchpad/locales/en.json
          lines:
            added: 131
            removed: 5
      created: []
      removed: []
    lines:
      added: 524
      removed: 20
---

# Previously

# Plan

# Changes
