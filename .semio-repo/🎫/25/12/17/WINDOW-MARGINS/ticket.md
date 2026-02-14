# Ticket

## Todos
# Previously

GoldenLayout windows touched the canvas edge and splitters used a fixed pixel width, so spacing between windows and the canvas was not aligned to the unit sizing system. Window borders were applied inconsistently between GoldenLayout and Canvas-based layouts.

# Plan

- Make Canvas add a 1-unit inner margin so window content never touches the canvas edge.
- Make multi-window containers use a 1-unit gap between windows.
- Make GoldenLayout splitters use 1-unit thickness and keep window borders continuous at the stack level.
- Document the window spacing + border mechanism in README/AGENTS.

# Changes

- Added 1-unit canvas padding and 1-unit window gaps for Canvas-based layouts.
- Updated GoldenLayout splitter sizing to use the unit system and ensured borders remain continuous around each window.
- Added a `Window.kind` switch to avoid nested borders inside GoldenLayout-rendered windows.
- Updated developer documentation to describe the window spacing/border mechanism.

## Changes

## Log

## Summary
# Summary

Sketchpad windows have 1-unit margins and continuous borders
