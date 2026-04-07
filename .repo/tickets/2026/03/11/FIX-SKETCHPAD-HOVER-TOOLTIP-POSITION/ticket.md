---
goal: SKETCHPAD-IMPROVEMENTS
---

# Ticket

## Summary

Fixed tooltip positioning by removing 4 span.contents wrappers from TooltipTrigger children in elements.tsx. display:contents caused getBoundingClientRect() to return (0,0,0,0), making Radix position tooltips at top-left. Verified via Playwright across Home (22), Kit (34), Type (28) pages - all tooltips positioned correctly.
## Changes
- `elements.tsx`: Removed `<span className="contents">` wrappers from TooltipTrigger children at 4 locations:
  - Line 1582: `ActionGroupItem` tooltip trigger
  - Line 1701: `Action` tooltip trigger
  - Line 1815: `ButtonGroupItem` tooltip trigger
  - Line 3013: `ToggleGroupItem` tooltip trigger
- All wrapped elements (native HTML elements, Radix primitives, Slot) already forward refs, so `asChild` works directly without a wrapper span.

## Log
- Diagnosed: Radix `--radix-popper-anchor-width: 0px; --radix-popper-anchor-height: 0px` on tooltip content confirms zero-dimension trigger.
- Root cause: `display: contents` on trigger span makes `getBoundingClientRect()` return (0,0,0,0).
- Confirmed no other `<span className="contents">` wrappers exist in the sketchpad codebase.
- Verified all other TooltipTrigger usages pass native elements or correct JSX directly.

## Todos
- [x] Diagnose tooltip positioning issue
- [x] Fix ActionGroupItem TooltipTrigger span wrapper (line 1582)
- [x] Fix Action TooltipTrigger span wrapper (line 1701)
- [x] Fix ButtonGroupItem TooltipTrigger span wrapper (line 1815)
- [x] Fix ToggleGroupItem TooltipTrigger span wrapper (line 3013)
- [x] Verify no other span contents wrappers exist
- [x] Playwright verification: Home page (22 triggers, all positioned correctly)
- [x] Playwright verification: Kit page (34 triggers, all positioned correctly)
- [x] Playwright verification: Type page (28 triggers, all positioned correctly)
- [x] Playwright verification: Design page navigation confirmed (104 triggers found, browser resource constraints prevented full iteration but same components are used)

## Plan
Remove `<span className="contents">` wrappers from TooltipTrigger asChild children. The wrapped elements (Radix primitives and native HTML elements) already forward refs, so asChild works directly without a wrapper.
