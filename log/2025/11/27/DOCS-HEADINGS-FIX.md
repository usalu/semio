---
slug: DOCS-HEADINGS-FIX
summary: >-
  Fix HeadingsProvider context issue - Details panel shows no headings because
  it's outside the HeadingsProvider context
---

# Previously

The Docs app had an issue where the Details panel showed "No headings found" even when the page had headings. This was caused by the `HeadingsProvider` context being inside the App component while the Details panel was rendered through the `PanelSectionProvider` which is at the Sketchpad level - outside the HeadingsProvider's context.

Additionally, the MDX heading components were registering headings using `children.toString()` which produced broken text like `[object Object],🦗 Grasshopper (Rhino 8)` for complex React children.

# Plan

1. Replace the React Context-based headings state with a global module-level state using an event emitter pattern
2. Make the `useHeadings` hook subscribe to the global state so it works from any component
3. Update the `HeadingsProvider` to use the global state (for backwards compatibility with existing code)
4. Remove the faulty `children.toString()` heading registration from MDX components since the DOM-based extraction in `PageCanvas` is more reliable

# Changes

- `js/js/sketchpad/Docs.tsx`:
  - Added global `headingsState` object with subscribe/notify pattern for event-based updates
  - Updated `useHeadings` hook to use global state with proper subscription
  - Simplified `HeadingsProvider` to use `useHeadings` (global state) instead of local state
  - Removed `useEffect` heading registration from MDX h1-h6 components (DOM extraction handles this)
  - Fixed `children` handling in MDX heading components to not call `.toString()` on React children
