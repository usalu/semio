# Ticket

## Todos

- [x] Scale Kit diagram nodes to 2× size
- [x] Make TableAvatar fill the node box
- [x] Ensure edge intersection hits circle outline
- [x] Tune force layout defaults/minimums for larger nodes
- [ ] Update README.md and AGENTS.md

## Changes

## Log

- Reopened ticket to implement 2× node scaling and edge alignment.
- Set `NODE_SCALE = 2`, updated `NODE_WIDTH/HEIGHT` to `ICON_WIDTH * NODE_SCALE` (100px)
- Force layout clamps `linkDistance` to `NODE_WIDTH * 2.2`, `collideRadius` to `NODE_WIDTH * 0.9`
- `TableAvatar` uses `className="size-full"` to fill the node box
- Circular intersection uses `radius = NODE_WIDTH / 2` for edge endpoints
- Restarted @compose/js dev server to apply all changes
- Fixed part-of edge visibility: changed color from `--accent-secondary` to bright magenta (#ff00ff) with strokeWidth 4 for debugging
- Reference edges remain gray/dashed (strokeWidth 1) for distinction
- Fixed edge filtering logic: now automatically includes parent/child nodes needed for part-of edges even if collapsed in table view

## Summary

Refactored Kit diagram nodes to use circular TableAvatar style, matching the Design app pieces. Updated intersection math and node dimensions to ICON_WIDTH (50px).
