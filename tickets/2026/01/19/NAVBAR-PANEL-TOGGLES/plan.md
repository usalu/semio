# Plan

1.  Investigate `js/semio/sketchpad/Sketchpad.tsx` to understand how `Navbar` and `PanelToggles` work.
2.  Identify the root cause of missing toggles (empty `SidePanelTabContext`).
3.  Implement a mechanism in `LayoutWrapper` to populate `SidePanelTabContext` from `AppConfig.getPanels()`.
4.  Update `LayoutWrapper` panel rendering logic to respect `leftSidePanel`, `rightSidePanel`, and `hudPanel` visibility flags and ensure default content is shown when the container is toggled on.
