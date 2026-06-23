---
goal: SKETCHPAD-IMPROVEMENTS
---

# Fix Diagram Piece Drag In Design App

## Summary

Fixed piece dragging by closing left side panel overlay. Fixed Radix slider interaction with XPath ancestor locator for dblclick edit mode. Fixed rotation eval bug. All Design E2E tests pass.
## Changes
- Close left side panel before drag test to prevent overlay intercepting mouse events on diagram nodes
- Replace Playwright `.fill()` on hidden Radix slider `<input type="range">` with dblclick on `slider-value` → fill `input[type="number"]` → Enter
- Fix slider locator: use XPath `ancestor::div[@data-slot="slider-row"]` from `SliderPrimitive.Root` to find sibling `slider-value` span
- Fix rotation eval bug: connection lookup used `designGuid` instead of `guid`
- Remove childDebug diagnostic from test
- Remove DOM state debug evaluation from test
- Increase test timeout to 900s
- Remove unused `gapSliderInput` variable

## Log
- Investigating drag failure: nodeMovedInViewport=false, du=0, dv=0
- ReactFlow nodes have draggable:true, nodesDraggable:true passed to ReactFlow
- panOnDrag=[1,2] (middle/right click for panning), selectionOnDrag=true (left click on pane for lasso)
- Root cause: left side panel (position:absolute) overlays diagram nodes, intercepting mouse events
- Fix: close left panel via compose.sketchpad.navbar.panelToggle.leftSidePanel toggle
- Drag confirmed working: nodeMovedInViewport=true, dx=90, dy=45
- Slider `.fill()` hangs on hidden Radix range input → changed to dblclick edit mode
- Slider locator broken: slider-value is sibling of SliderPrimitive.Root, not descendant
- Fix: XPath ancestor::div[@data-slot="slider-row"] to navigate up then find sibling
- Rotation eval bug: c.guid === designGuid → c.guid === guid
- All tests pass: 1 passed (4.2m)

## Todos
- [x] Diagnose drag failure root cause (left panel overlay)
- [x] Fix diagram node drag behavior (close left panel)
- [x] Fix slider interaction (dblclick edit mode + XPath ancestor locator)
- [x] Fix rotation eval bug
- [x] Clean up test diagnostics
- [x] Run full Design test suite (PASSED)
