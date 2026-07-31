# Ticket

## Todos

- [x] Analyze node avatar dimensions vs edge calculation radius
- [x] Identify dimension mismatch between Avatar size and NODE_RADIUS
- [x] Implement fix to ensure Avatar explicitly matches NODE_WIDTH/NODE_HEIGHT
- [x] Add style prop to TableAvatar for explicit dimension control
- [x] Update Kit diagram nodes to use explicit dimensions
- [x] Build and verify changes compile

## Changes

### Modified Files

1. **js/compose/sketchpad/Kit.tsx**
   - Updated `KitArtifactNode` to use explicit px dimensions for node container (`width: ${NODE_WIDTH}px`, `height: ${NODE_HEIGHT}px`)
   - Changed TableAvatar className from `size-full` to `!w-full !h-full` for better CSS specificity
   - Added inline style prop to TableAvatar with explicit dimensions matching NODE_WIDTH/NODE_HEIGHT (100px)

2. **js/compose/sketchpad/elements.tsx**
   - Added `style?: React.CSSProperties` to `TableAvatarProps` interface
   - Updated `TableAvatar` component to accept and forward style prop to Avatar component

## Log

### Analysis Phase

- **Problem Identified**: Node avatars rendered at default `size-small` (~16px at compact spacing) while edge calculations used NODE_RADIUS (50px)
- **Root Cause**: `size-full` class on Avatar was not forcing dimensions to match the 100px x 100px node container
- **Expected Dimensions**:
  - ICON_WIDTH: 50px
  - NODE_SCALE: 2
  - NODE_WIDTH/HEIGHT: 100px
  - NODE_RADIUS: 50px (used for edge intersection calculations)

### Implementation Phase

- Ensured node container uses explicit `${NODE_WIDTH}px` string dimensions instead of numeric values
- Added style prop passthrough to TableAvatar and Avatar components
- Applied explicit dimensions (100px x 100px) via inline style to override any CSS specificity issues
- Used `!w-full !h-full` Tailwind classes with important flag for guaranteed application

## Summary

Fixed D3 Force Diagram Node-Edge Alignment by addressing avatar sizing mismatch. Implemented absolute positioning with explicit 100px dimensions, modified Avatar component to respect style prop, and ensured avatars fill node containers for clean edge connections at 50px radius.
