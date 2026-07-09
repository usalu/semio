---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Filtered .chat and .settings tabs from the rightSidePanel detail panel in Sketchpad.tsx. The hasRightTabs check, both rightSidePanel auto-select effects, and the tab list in the render all exclude tabs whose id ends with .chat or .settings. Navbar-triggered chat/settings panels continue to source their content from rightSidePanelTabs as before — no app-file changes needed.

## Changes

- `compose/js/sketchpad/Sketchpad.tsx`: filtered `.chat` and `.settings` tabs out of the detail panel (rightSidePanel), the `hasRightTabs` navbar toggle check, and the two rightSidePanel auto-select effects. Navbar-triggered chat/settings panels still source their content from `rightSidePanelTabs` as before.

## Log

- Gathered repo metadata, traced utility-tab registration to `Home.tsx`, `Kit.tsx`, `Quality.tsx`, `Design.tsx`, `Docs.tsx`, and `Type.tsx`.
- Apps (Home, Docs, Type, Kit, Quality) each call `addSidePanelTab("right", { id: "...chat", ... })` and `addSidePanelTab("right", { id: "...settings", ... })`, which were included in the regular `rightSidePanelTabs` array shown as detail panel tabs.
- The navbar-triggered chat/settings panels sourced content from `rightSidePanelTabs.find(t => t.id.includes("chat/settings"))` — keeping them fully functional.
- Fix: in `Sketchpad.tsx`, filter tabs ending with `.chat` or `.settings` from: (a) `hasRightTabs` (navbar toggle), (b) the rightSidePanel auto-select effects, (c) the rightSidePanel tab list in the render. No changes needed in app files.

## Todos

- [x] Filter chat/settings tabs from detail panel rendering in Sketchpad.tsx
- [x] Filter chat/settings from hasRightTabs check
- [x] Filter chat/settings from rightSidePanel effects

## Plan

Filter `.chat` and `.settings` tabs out of the rightSidePanel detail panel while keeping them available for the navbar-triggered panels. Single-file change in `Sketchpad.tsx`.
