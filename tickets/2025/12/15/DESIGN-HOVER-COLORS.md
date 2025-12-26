---
slug: DESIGN-HOVER-COLORS
summary: Fix design piece hover/select colors
prompt: >-
  design app: - The piece nodes dont show hover color when hovering over the
  piece in diagram. - The piece geometry material is not showing hover or select
  color.
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.954Z"
commit: "0000000000000000000000000000000000000000"
iterations:
  - prompt: >-
      design app: - The piece nodes dont show hover color when hovering over the
      piece in diagram. - The piece geometry material is not showing hover or
      select color.
    date:
      started: "2025-12-15T09:58:28.842Z"
    model: gpt-5-2
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 67126961d64c89450f396abedd5d477670f1ad4d
    files:
      updated:
        - path: AGENTS.md
          lines:
            added: 54
            removed: 20
        - path: README.md
          lines:
            added: 54
            removed: 20
        - path: js/js/sketchpad/Design.tsx
          lines:
            added: 54
            removed: 20
        - path: js/js/sketchpad/elements.tsx
          lines:
            added: 54
            removed: 20
      created:
        - log/tickets/2025/12/15/DESIGN-HOVER-COLORS.md
      removed: []
    lines:
      added: 216
      removed: 80
---

# Previously

- Diagram hover state was being set, but the `Avatar` node used `ring-inset` so the hover/selection ring was visually hidden by the full-size `AvatarFallback` background.
- Scene selection/hover colors were not visible when a base `color` was provided to `Geometry`.
- Loaded type meshes in the Design scene used fixed plaster materials and did not react to selection/hover.

# Plan

- Make diagram node rings visible on hover/selection.
- Make `Geometry` apply hover/selection colors even when `color` is passed.
- Add hover/selection highlighting for loaded piece meshes (GLTF/FBX/OBJ).
- Document the mechanisms in `README.md` and `AGENTS.md`.

# Changes

- Updated avatar ring classes to use non-inset rings so hover/selection rings render correctly.
- Updated `Geometry` to prioritize `selected`/`hovered` theme colors over base colors and to default edges/emissive to the interactive color.
- Extended Design scene mesh loaders to accept a `highlightColor` and apply it to cloned materials on selection/hover.
- Updated `README.md` and `AGENTS.md` to describe the selection/hover rendering behavior.
