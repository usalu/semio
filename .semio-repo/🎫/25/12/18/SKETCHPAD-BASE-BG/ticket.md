# Ticket

## Todos
# Previously

- Sketchpad root/canvas containers could render without a guaranteed base surface fill.
- Documentation described a window-only background approach, which conflicted with the desired base canvas background.

# Plan

- Restore `bg-base` on the Sketchpad layout root so the canvas is always filled behind windows.
- Align documentation to describe background levels (base/window/panel/temporary) rather than a window-only surface.

# Changes

- Restored `bg-base` on the Sketchpad `Layout` root container.
- Updated `README.md` and `AGENTS.md` to describe background levels and the base canvas surface.

## Changes

## Log

## Summary
# Summary

Restore base background for Sketchpad canvas
