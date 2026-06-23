# Ticket

## Todos
# Previously

- Window backgrounds were still using base-level styling in some window contexts, so window surfaces did not visually separate from the canvas.
- GoldenLayout window chrome (header/tab/buttons) was still forced to the base background token.

# Plan

- Make `Window` the canonical boundary for the `window` level.
- Ensure GoldenLayout window chrome uses the window background token.
- Document the background level mechanism in root docs.

# Changes

- Updated `js/compose/sketchpad/elements.tsx` `Window` to scope its subtree via `LevelProvider level="window"` and apply the window background level.
- Updated `js/compose/globals.css` GoldenLayout overrides so header/tabs/window buttons/stack surfaces use the window background token.

## Changes

## Log

## Summary
# Summary

Fix window background colors via useLevel
