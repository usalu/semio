# Ticket: Toolbar Redesign

**Prompt:** Create a redesigned global bottom toolbar for all applications. The toolbar is fixed at the bottom center of the screen, not floating, and rendered as one single flat horizontal band. The band is divided into two contiguous sections with no gap between them: Left: Tool Bar, Right: Tool Settings Panel. Both sections originate from the exact horizontal center of the screen.

## Status
- [x] Analysis
- [x] Implementation
- [x] Verification

## Summary
Refactored `Kit.tsx` and Verified `Type.tsx` for Split Toolbar Architecture.

1.  **Refactored `js/compose/sketchpad/Kit.tsx`**:
    *   Renamed the existing `KitFilters` component (which contained toggles for designs/types/etc.) to `KitKindToggles` to resolve naming conflicts.
    *   Created a *new* `KitFilters` component that combines the Search Input and `KitKindToggles`.
    *   Implemented `KitCreateActions` component.
    *   Updated `MultiWindowApp` to register `toolbar` sections (`selection`, `filter`, `create`) using the `toolbarGroup` property. This ensures they render correctly in the `Sketchpad.tsx` split toolbar layout.
    *   Wrapped section content in `KitScopeProvider` to ensure context availability.

2.  **Localization**:
    *   Added `compose.sketchpad.app.kit.search.placeholder` to `js/compose/sketchpad/locales/en.json`.

3.  **Verification**:
    *   Verified `js/compose/sketchpad/Type.tsx` already conforms to the split toolbar architecture (uses `addSection` with `toolbarGroup`).
    *   Confirmed `js/compose/sketchpad/Sketchpad.tsx` rendering logic: The "Tool Setting Bar" (right side) is initially empty until a toolbar group (like "Filter") is toggled active. This explains the "not visible" behavior if no group is active by default.

**Files:**
- `js/compose/sketchpad/Kit.tsx`
- `js/compose/sketchpad/locales/en.json`
