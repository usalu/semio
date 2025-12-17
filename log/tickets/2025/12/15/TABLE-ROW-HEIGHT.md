---
slug: TABLE-ROW-HEIGHT
summary: Normalize Sketchpad table row heights
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.962Z"
commit: "0000000000000000000000000000000000000000"
iterations:
  - prompt: >-
      Finish (currently table rows are still heigher than toggles): Table rows
      have a fixed height...
    date:
      started: "2025-12-15T12:26:26.622Z"
    model: gpt-5-2
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 76900221ecf5cfb30a37d69fbb66abb3e0a0e45a
    files:
      updated:
        - path: AGENTS.md
          lines:
            added: 11
            removed: 2
        - path: README.md
          lines:
            added: 11
            removed: 2
        - path: js/js/sketchpad/elements.tsx
          lines:
            added: 11
            removed: 2
        - path: log/tickets/2025/12/15/TABLE-ROW-HEIGHT.md
          lines:
            added: 11
            removed: 2
      created: []
      removed: []
    lines:
      added: 44
      removed: 8
---

# Previously

# Sketchpad tables target `h-medium` rows but table body `td` used `p-single`, so rows expanded beyond the fixed height whenever a cell contained an `h-medium` control (Toggle/Input/etc).

# Plan

- Align the `Table` primitive so row height is enforced by the row, not by additive cell padding.
- Keep horizontal padding (`px-single`) and vertically center all cell content.
- Update dev docs to capture the table sizing mechanism.

# Changes

- Updated `Table` and `TableSkeleton` body cells to use `px-single py-0` and wrap cell content in a vertically centered `h-full` flex container.
