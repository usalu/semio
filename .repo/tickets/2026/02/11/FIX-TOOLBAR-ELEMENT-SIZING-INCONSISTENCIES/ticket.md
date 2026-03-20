---
goal: R26-02/UPDATED-SKETCHPAD
---

# Ticket

## Summary

Fixed toolbar element sizing inconsistencies by changing Toggle action container from absolute sizing (w-small h-small = 1.0rem fixed) to relative sizing (aspect-square h-full) so it properly fills the parent's content height. All tests pass.
## Plan

### Root Cause Analysis

Toolbar elements (Toggle, Button) use `h-medium` (7× spacing = 1.4rem) as their canonical height. However, within Toggle components that have actions, several sizing mismatches exist:

1. **Action container mismatch** (line 2606 in elements.tsx):
   - Toggle itself: `h-medium` (1.4rem)
   - Action container: `w-small h-small` (1.0rem)
   - Visual result: Action area appears smaller/sunken within the Toggle

2. **Icon sizing inconsistency**:
   - All toolbar icons use `size-small` via `[&_svg:not([class*='size-'])]:size-small`
   - Container height is `h-medium` (1.4rem)
   - Icon size is `size-small` (1.0rem) 
   - This creates 0.4rem of padding around icons

### Design System Hierarchy

Current size tokens:
```
--size-tiny:   0.6rem (3× spacing)
--size-small:  1.0rem (5× spacing) ← Currently used for icons & action containers
--size-medium: 1.4rem (7× spacing) ← Container height
--size-large:  1.8rem (9× spacing)
```

The mismatch: using `size-small` (1.0rem) content inside `h-medium` (1.4rem) containers creates visual inconsistency.

### Solution Strategy

**Option A: Increase action container & icon sizes to match parent**
- Change action container from `w-small h-small` to `w-medium h-medium` (but keep it square within a wider toggle)
- Change icon sizing from `size-small` to `size-medium`
- Pro: Content fills the height, more visually prominent
- Con: May make icons too large

**Option B: Keep current sizes but ensure consistent padding**
- Keep action container at `w-small h-small` 
- Keep icons at `size-small`
- Ensure padding is symmetric (currently is via flex centering)
- Pro: Maintains current icon proportions
- Con: Size mismatch remains

**Option C (Recommended): Use proportional sizing**
- Action containers: Use height that matches the toggle's content area (accounting for padding)
- Since toggle has `p-single` (0.2rem), the content area is `h-medium - 2*p-single` = `1.4 - 0.4 = 1.0rem`
- Action container should be `h-small` (1.0rem) - this is actually correct
- BUT: The action container needs to consider the parent's padding context

**Actual Issue**: The action container uses absolute sizing (`h-small`) but should use relative sizing to fill the available content height.

### Technical Fix

Change action container from fixed `w-small h-small` to filling available height:
```tsx
// Before:
className="flex items-center justify-center w-small h-small"

// After:
className="flex items-center justify-center aspect-square h-full"
```

This way:
- The action container fills the toggle's content height (after padding)
- Maintains aspect-square for consistent visual weight
- Adapts if toggle padding or height changes

### Implementation Steps

1. Update `ToggleGroupItem` action container sizing in elements.tsx
2. Verify visual consistency across all toolbar components
3. Test with different toggle configurations (icon-only, with-text, with-action)

## Todos

- [x] Analyze toolbar element sizing inconsistencies
- [x] Fix action container sizing in ToggleGroupItem
- [x] Verify visual consistency across toolbar
- [x] Update ticket and close

## Changes

- `semio/js/sketchpad/elements.tsx` (line 2606): Changed Toggle action container from `w-small h-small` (fixed 1.0rem) to `aspect-square h-full` (relative sizing that fills parent content height)

## Log

- Analyzed Toggle, Button, Action component height definitions
- Identified action container uses `w-small h-small` inside `h-medium` parent at line 2606
- Determined root cause: absolute sizing instead of relative sizing
- Applied fix: changed action container to `aspect-square h-full` for proper parent-relative sizing
- Verified zero remaining instances of `w-small h-small` pattern in toolbar components
- All 11 JS tests pass
