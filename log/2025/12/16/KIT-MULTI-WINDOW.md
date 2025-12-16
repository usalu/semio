---
slug: KIT-MULTI-WINDOW
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Extend kit app to multi-window with table and diagram views
model: claude-opus-4.5
input:
  - prompt: >-
      The kit app should be extended to a multi-window app like the design app.
      It should have two window kinds: table and diagram. The table window is
      the current canvas. The diagram window should show a forced layout graph
      of all the artifacts of the kits and their relationships. There are two
      different kind of relationships: part of (children of parents, artifacts
      inside folders) and references (such as between a type and a design if
      there is a piece inside of the design with that type). Hover and selection
      of artefacts are again shared among the windows.
    date: '2025-12-15T23:29:43.849Z'
  - prompt: >-
      Fix issues: Table window is empty. All nodes should be circles like Design
      diagram nodes with artifact icons. Layout is not a draggable force layout
      - add force slider to diagram settings. Migrate existing kit app tests and
      ensure they pass.
    date: '2025-12-16T00:01:48.177Z'
commit: 06f516fab841773c499e5e7b9dbcdcfa63ad94cb
files:
  updated:
    - AGENTS.md
    - js/js/sketchpad/Kit.tsx
    - js/js/sketchpad/locales/de.json
    - js/js/sketchpad/locales/en.json
lines:
  added: 522
  removed: 18
---
# Previously

# Plan

# Changes
