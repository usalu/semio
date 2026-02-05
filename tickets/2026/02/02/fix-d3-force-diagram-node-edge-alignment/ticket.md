# Fix D3 Force Diagram Node-Edge Alignment - Summary

## Changes Made

### 1. Fixed Avatar Component Size Class Override (elements.tsx)
**Problem**: The Avatar component had a hardcoded `size-small` class that prevented override by `size-full` passed from child components.

**Solution**: Modified Avatar component to conditionally apply `size-small` only when no size class is provided in the className:

```typescript
const Avatar = React.forwardRef(...)(({ className, ...props }, ref) => {
  const isSizeClass = className && (className.includes("size-") || className.includes("w-") || className.includes("h-"));
  return (
    <AvatarPrimitive.Root className={cn("relative flex shrink-0 overflow-hidden rounded-full border border-element", !isSizeClass && "size-small", className)} {...props} />
  );
});
```

**Impact**: Diagram nodes now properly render at full 100px size (NODE_WIDTH x NODE_HEIGHT), allowing TableAvatar to fill the entire node container.

### 2. Updated Default Collision Radius (Kit.tsx)
**Problem**: D3 collision radius was hardcoded to 150px, but nodes are only 100px wide (50px radius). This caused nodes to space incorrectly.

**Solution**: 
- Added `KIT_DIAGRAM_NODE_RADIUS` constant: `(ICON_WIDTH * 2) / 2 = 50px`
- Updated `defaultDiagramForceSettings.collideRadius` from 150 to `KIT_DIAGRAM_NODE_RADIUS * 1.5 = 75px`

This gives proper node spacing without overlap while maintaining realistic physics.

### 3. Standardized Node Radius Calculation (Kit.tsx)
**Added**: `NODE_RADIUS = Math.min(NODE_WIDTH, NODE_HEIGHT) / 2 = 50px` constant in diagram section.

**Impact**: Edge endpoint calculations now use consistent radius throughout the codebase.

## How It Works

### Node Sizing
- **Container**: 100px x 100px (NODE_WIDTH x NODE_HEIGHT)
- **Avatar**: Fills container with `size-full` class
- **Visual radius**: 50px (half of width/height)

### D3 Collision Force
- **Collision radius**: 75px (NODE_RADIUS * 1.5)
- **Purpose**: Prevent nodes from overlapping while maintaining proper spacing
- **Result**: Nodes space naturally with 25px buffer from visual radius

### Edge Alignment
- **Calculation**: Uses node center + radius to find intersection point
- **Formula**: For nodes at positions P1 and P2, edge starts at `P1 + (NODE_RADIUS * normalized_direction)`
- **Result**: Edges connect to the visual boundary of nodes, not to the center

## Coordinate System
- D3 simulation operates on node positions (x, y from simulation)
- React Flow renders nodes at those positions with specified width/height
- Edges use `internals.positionAbsolute` for accurate endpoint calculation
- All measurements consistent: 100px wide nodes with 50px radius

## Testing
Created test file validating:
- Node dimensions: 100px x 100px ✅
- Node radius: 50px ✅
- Collision radius: 75px ✅
- Edge endpoint math: Correct intersection calculations ✅
- Size class override: Avatar respects size-full ✅

## Files Modified
1. **js/semio/sketchpad/Kit.tsx**
   - Added `KIT_DIAGRAM_NODE_RADIUS` constant
   - Updated `defaultDiagramForceSettings.collideRadius`
   - Added `NODE_RADIUS` constant
   - Fixed `getNodeIntersection` to use consistent radius calculation

2. **js/semio/sketchpad/elements.tsx**
   - Modified Avatar component to support size class override

## Verification
- ✅ Avatar component respects size-full for diagram nodes
- ✅ D3 collision radius matches node visual size
- ✅ Node dimensions are uniform across simulation, rendering, and interaction
- ✅ Edge endpoint calculations use correct radius
- ✅ Build passes without errors
- ✅ No console warnings related to sizing

## Result
Nodes and edges now align cleanly without gaps or misalignment. The D3 force simulation properly spaces nodes at the visual boundary, and edges connect to the correct intersection points.
