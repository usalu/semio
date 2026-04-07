---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fixed right side panel not showing by default in Design app. The panelVisibility was missing rightSidePanel: true in 5 locations across Design.tsx and Sketchpad.tsx. Added rightSidePanel: true to DesignStore constructor, plugin createDefaultState, useDesignAppInitialize DESIGN.INIT event, DEFAULT_PANEL_VISIBILITY constant, and createDefaultDesignAppState. Detail panel now renders with piece properties when a piece is selected. 13/13 unit tests pass, 7/7 e2e tests pass, 0 TS errors.
## Changes

## Log

## Todos

## Plan
