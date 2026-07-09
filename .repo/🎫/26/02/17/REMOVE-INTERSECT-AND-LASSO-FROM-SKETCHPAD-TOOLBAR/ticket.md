---
goal: CLEANUP
---

# Ticket

## Summary

Successfully removed intersect and lasso toolbar elements from the Type sketchpad. The intersect mode toggle and lasso toolbar section have been removed from the UI along with their corresponding tool definitions and components. All unit tests pass, confirming that the underlying selection composition logic remains intact and can still be accessed through keyboard modifiers (Shift+Ctrl for intersect).

## Changes

- Removed intersect toggle from TypeSelectSettings component
- Removed lasso toolbar section from Type app initialization
- Removed SelectionIntersectTool, LassoRectangularTool, and LassoFreeformTool definitions
- Removed TypeLassoSettings component
- Removed IntersectIcon and DiagramIcon from imports
- Updated TypeAppTools array to exclude removed tools
- Updated cleanup function to remove reference to lasso section

## Log

## Todos

## Plan

Remove intersect and lasso from the Type sketchpad toolbar:

1. Remove the intersect toggle from TypeSelectSettings component
2. Remove the lasso toolbar section from the Type app initialization
3. Clean up any related imports and tool definitions
4. Update tests that reference these removed elements

## Todos

- [x] Remove intersect toggle from TypeSelectSettings
- [x] Remove lasso toolbar section
- [x] Remove SelectionIntersectTool definition
- [x] Clean up imports
- [x] Verify changes
