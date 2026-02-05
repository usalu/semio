# Fix D3 Force Diagram Node-Edge Alignment

## Problem Analysis

### Root Causes Identified

1. **Node Size Mismatch**
   - `NODE_SCALE = 2` makes visual nodes 2x `ICON_WIDTH` (96px if ICON_WIDTH=48px)
   - D3 collision radius is `forceCollide().radius(50)` - too small for visual nodes
   - Edge endpoint calculation uses `Math.min(width, height) / 2` on node dimensions
   - Visual node appears smaller than where edges connect

2. **Coordinate System Issues**
   - D3 simulation runs nodes through collision force with radius 50
   - React Flow renders nodes at their positions with width/height properties
   - Edge calculation uses `internals.positionAbsolute` vs `position` inconsistently
   - TableAvatar renders inside the node at `size-full` but may have additional padding

3. **Node Dimension Constants**
   - `NODE_WIDTH = ICON_WIDTH * NODE_SCALE` (96px)
   - `NODE_HEIGHT = ICON_WIDTH * NODE_SCALE` (96px)
   - `ICON_WIDTH` not defined in visible context - need to find it
   - D3 collision radius (50) doesn't account for full visual node size

4. **Edge Endpoint Calculation**
   - Uses node width/height for radius calculation
   - Current formula: `radius = Math.min(width, height) / 2`
   - With NODE_WIDTH/HEIGHT of 96px, radius would be 48px
   - But D3 uses 50px collision radius - slight mismatch
   - Edge endpoints should align with actual rendered boundary

## Implementation Plan

### Phase 1: Analysis & Constants Consolidation
1. Find `ICON_WIDTH` constant and understand sizing hierarchy
2. Verify `TableAvatar` actual rendered dimensions (padding, border, etc)
3. Calculate correct collision radius for D3 based on rendered node size
4. Document coordinate system flow (D3 → React Flow → rendering)

### Phase 2: Fix D3 Collision Radius
1. Update `forceCollide().radius()` to match actual visual node radius (48px)
2. Ensure D3 simulation nodes have correct radius for spacing
3. Test that nodes don't overlap after simulation

### Phase 3: Fix Edge Endpoint Calculation
1. Ensure `getNodeIntersection()` uses consistent dimensions
2. Verify edge endpoints use correct absolute positions
3. Update endpoint calculations to use actual rendered node radius
4. Add debug helpers to visualize endpoint calculations

### Phase 4: Verify Coordinate Systems
1. Check D3 simulation position updates to React Flow positions
2. Ensure positionAbsolute is correctly derived from position
3. Test with both dragging and force simulation active

### Phase 5: Testing & Validation
1. Visual inspection: edges align with node boundaries
2. No overlapping nodes in large diagrams
3. Edges align when dragging nodes
4. Edges align during force simulation

## Success Criteria
- ✅ Node avatars and edge circles visually align
- ✅ No gaps between nodes and edge endpoints
- ✅ Consistent dimensions across simulation, rendering, and interaction
- ✅ No console errors or warnings
- ✅ Works with dragging, force simulation, and static layouts

## Files to Modify
1. `js/semio/sketchpad/Kit.tsx` - Node/edge rendering and calculations
2. `js/semio/sketchpad/elements.tsx` - D3 force configuration
