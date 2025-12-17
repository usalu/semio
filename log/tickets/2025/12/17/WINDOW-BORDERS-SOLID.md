---
slug: WINDOW-BORDERS-SOLID
summary: Fix window borders and revert to solid style
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: '2025-12-17T18:34:02.257Z'
  finished: '2025-12-17T18:36:14.754Z'
commit: 7c4820638369104ae259b238d9240f08e429e67e
model: gpt-5.2-codex
iterations:
  - prompt: Still no bottom and right border. Remove the dashed border.
    date:
      started: '2025-12-17T18:34:07.737Z'
      ended: '2025-12-17T18:36:06.323Z'
    model: gpt-5.2-codex
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 7c4820638369104ae259b238d9240f08e429e67e
    files:
      updated:
        - js/js/globals.css:
            lines:
              added: 26
              removed: 7
        - js/js/sketchpad/elements.tsx:
            lines:
              added: 38
              removed: 39
        - README.md:
            lines:
              added: 17
              removed: 0
        - AGENTS.md:
            lines:
              added: 16
              removed: 8
        - log/tickets/2025/12/17/WINDOW-BORDERS-SOLID.md:
            lines:
              added: 49
              removed: 0
      created: []
      removed: []
    lines:
      added: 146
      removed: 54
files:
  updated:
    - AGENTS.md:
        lines:
          added: 16
          removed: 8
    - README.md:
        lines:
          added: 17
          removed: 0
    - js/js/globals.css:
        lines:
          added: 26
          removed: 7
    - js/js/sketchpad/elements.tsx:
        lines:
          added: 38
          removed: 39
    - log/tickets/2025/12/17/WINDOW-BORDERS-SOLID.md:
        lines:
          added: 51
          removed: 0
  created: []
  removed: []
lines:
  added: 148
  removed: 54
---
# Previously
GoldenLayout window borders were still missing on the bottom/right edges and dashed borders were not desired.

# Plan
Render GoldenLayout window borders as an inset 1px stroke so all edges render reliably, revert window borders to solid style, and update dev docs to match the final mechanism.

# Changes
GoldenLayout stacks now render a continuous solid outline via inset box-shadow to avoid edge clipping on bottom/right. Canvas windows reverted from dashed to solid `border-window`. Documentation updated.
