# Log

## Investigation

The user reported missing Navbar panel toggles.
Investigation of `js/semio/sketchpad/Sketchpad.tsx` revealed that `PanelToggles` component relies on `SidePanelTabContext` being populated.
However, `SidePanelTabContext` was empty because there was no logic connecting `AppConfig.getPanels()` (which defines the panels) to the `addSidePanelTab` context method. The `usePanelConfigs` hook existed but was unused.

## Fix

1.  Modified `LayoutWrapper` in `js/semio/sketchpad/Sketchpad.tsx` to automatically register side panels based on the current app's configuration using `usePanelConfigs` and `addSidePanelTab`.
2.  Updated `LayoutWrapper`'s panel rendering logic to correctly handle the container visibility flags (`leftSidePanel`, `rightSidePanel`, `hudPanel`). This ensures that when a toggle is clicked (activating the container flag), a valid panel section (e.g., `details` or `workbench`) is shown as a fallback if none was specifically active.

This restores the functionality of the Left, Middle (HUD), and Right panel toggles in the Navbar.
