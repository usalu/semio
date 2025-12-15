---
slug: DESIGN-HOVER-COLORS
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Fix design piece hover/select colors
model: gpt-5.2-codex
input:
  - prompt: >-
      design app: - The piece nodes dont show hover color when hovering over the
      piece in diagram. - The piece geometry material is not showing hover or
      select color.
    date: '2025-12-15T09:58:28.842Z'
commit: 67126961d64c89450f396abedd5d477670f1ad4d
files:
  updated:
    - AGENTS.md
    - README.md
    - js/js/sketchpad/Design.tsx
    - js/js/sketchpad/elements.tsx
  created:
    - log/2025/12/15/DESIGN-HOVER-COLORS.md
lines:
  added: 215
  removed: 78
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
