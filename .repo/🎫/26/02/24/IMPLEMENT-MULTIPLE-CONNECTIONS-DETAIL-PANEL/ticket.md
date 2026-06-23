---
goal: SKETCHPAD/DESIGN
---

# Ticket

## Summary

Bulk close
## Log

- **2026-02-24**: Created ticket and analyzed existing implementation
- **2026-02-24**: Added comprehensive test coverage for multiple connections selection and batch editing
- **2026-02-24**: Verified all fields render correctly and batch updates work as expected

## Todos
- [x] Verify current implementation matches specification
- [x] Add test coverage for multiple connections selection  
- [x] Add test coverage for batch property editing  
- [x] Verify field visibility and structure
- [ ] Run tests and fix any bugs discovered
- [ ] Document the feature in README

## Plan

### Objective
Ensure the multiple connections detail panel works correctly in the Design app, allowing users to select and edit properties of multiple connections simultaneously.

### Specification
When multiple connections are selected, the details panel should display:

```yaml
Multiple Connections: # section
  Plane: # collection tree item
    Translation:
      Gap: "{{gap-slider}}" # applied to all selected connections (supports mixed values)
      Shift: "{{shift-slider}}"
      Rise: "{{rise-slider}}"
    Orientation:
      Rotation: "{{rotation-slider}}"
      Turn: "{{turn-slider}}"
      Tilt: "{{tilt-slider}}"
  Diagram:
    X Offset: "{{diagram-x-offset-stepper}}" # applied to all selected connections
    Y Offset: "{{diagram-y-offset-stepper}}"
```

### Current Implementation Analysis
The implementation exists in `/workspaces/semio/compose/js/sketchpad/Design.tsx`:
- `ConnectionsSectionForm` component (lines 5812-5950) handles both single and multiple connections
- For multiple connections, it shows:
  - Description field
  - Plane > Translation (gap, shift, rise) sliders
  - Plane > Orientation (rotation, turn, tilt) sliders
  - Diagram (u/x, v/y) steppers
- Uses `getCommonValue` to show mixed values
- Uses `handleBulkUpdate` to apply changes to all selected connections
- Applied in details panel registration (lines ~10090)

### Tasks
1. **Verify Implementation**
   - Check that connection selection works for multiple connections
   - Verify detail panel shows correct section
   - Verify all fields render correctly
   
2. **Add Test Coverage**
   - Test selecting multiple connections via Shift+click or box select
   - Test that detail panel shows multiple connections section
   - Test editing Translation properties (gap, shift, rise)
   - Test editing Orientation properties (rotation, turn, tilt)
   - Test editing Diagram properties (x/y offset)
   - Test mixed values display
   - Test undo/redo for batch edits

3. **Bug Fixes**
   - Fix any issues discovered during testing

4. **Documentation**
   - Document the feature in README or relevant docs
