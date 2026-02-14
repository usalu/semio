# Fix D3 Force Diagram Node-Edge Alignment

## Problem Analysis

Node avatars in the D3 Force diagram appear smaller than their edge connection circles, creating visual gaps. Edges connect to an invisible larger circle while the rendered node is smaller.

## Root Cause Investigation

1. **Node Avatar Dimensions**
   - Avatar component size vs rendered size
   - Border, padding, and content box calculations
   - Small avatar size definition (h-small, w-small = 5 units)

2. **Edge Endpoint Calculations**
   - Edge connection point radius calculations
   - positionAbsolute coordinates from React Flow
   - Node dimension measurements for intersection math

3. **Coordinate System**
   - D3 simulation positions (x, y)
   - React Flow rendering positions (positionAbsolute)
   - Transformation pipeline consistency

4. **Dimension Propagation**
   - How node size flows from simulation → rendering → interaction
   - Where radius values are defined and used

## Implementation Plan

### Phase 1: Diagnostic Analysis
1. Search for Avatar/node sizing in elements.tsx
2. Search for edge endpoint calculation logic
3. Identify all radius constants and calculations
4. Map dimension flow through the system

### Phase 2: Playwright Test Creation
1. Create visual regression test for node-edge alignment
2. Measure actual rendered node dimensions
3. Measure edge connection point positions
4. Verify gaps/overlaps programmatically

### Phase 3: Fix Implementation
1. Align Avatar size constant with edge calculation radius
2. Ensure consistent radius usage across:
   - D3 force simulation node radius
   - Avatar component dimensions
   - Edge intersection calculations
   - Interaction hit testing
3. Verify DIAGRAM_UNIT coordinate system usage

### Phase 4: Validation
1. Run Playwright tests to verify alignment
2. Test across all diagram views (Kit, Design, Type)
3. Verify no visual gaps or overlaps
4. Ensure drag, hover, and selection still work

## Success Criteria

- [ ] Node avatars and edge endpoints align perfectly
- [ ] No visual gaps between nodes and edges
- [ ] Consistent radius values across all code paths
- [ ] Playwright tests pass with visual verification
- [ ] All existing tests still pass

## Files to Modify

- `js/semio/sketchpad/elements.tsx` - Diagram component and Avatar sizing
- `js/semio/sketchpad/Kit.tsx` - Kit diagram edge calculations (if needed)
- `js/semio/sketchpad/Design.tsx` - Design diagram (if needed)
- `js/semio/globals.css` - Size unit definitions (if needed)
- New: Playwright test file for alignment verification
