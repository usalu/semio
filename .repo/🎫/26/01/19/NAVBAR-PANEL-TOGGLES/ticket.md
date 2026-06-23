# Ticket

## Todos
# Plan

1.  Investigate `js/compose/sketchpad/Sketchpad.tsx` to understand how `Navbar` and `PanelToggles` work.
2.  Identify the root cause of missing toggles (empty `SidePanelTabContext`).
3.  Implement a mechanism in `LayoutWrapper` to populate `SidePanelTabContext` from `AppConfig.getPanels()`.
4.  Update `LayoutWrapper` panel rendering logic to respect `leftSidePanel`, `rightSidePanel`, and `hudPanel` visibility flags and ensure default content is shown when the container is toggled on.

## Changes

## Log
# Log

## Investigation

The user reported missing Navbar panel toggles.
Investigation of `js/compose/sketchpad/Sketchpad.tsx` revealed that `PanelToggles` component relies on `SidePanelTabContext` being populated.
However, `SidePanelTabContext` was empty because there was no logic connecting `AppConfig.getPanels()` (which defines the panels) to the `addSidePanelTab` context method. The `usePanelConfigs` hook existed but was unused.

## Fix

1.  Modified `LayoutWrapper` in `js/compose/sketchpad/Sketchpad.tsx` to automatically register side panels based on the current app's configuration using `usePanelConfigs` and `addSidePanelTab`.
2.  Updated `LayoutWrapper`'s panel rendering logic to correctly handle the container visibility flags (`leftSidePanel`, `rightSidePanel`, `hudPanel`). This ensures that when a toggle is clicked (activating the container flag), a valid panel section (e.g., `details` or `workbench`) is shown as a fallback if none was specifically active.

This restores the functionality of the Left, Middle (HUD), and Right panel toggles in the Navbar.

## Summary
