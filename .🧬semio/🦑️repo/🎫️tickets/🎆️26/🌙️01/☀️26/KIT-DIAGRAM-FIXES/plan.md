# Plan - Kit Diagram Fixes

## Problem

- Kit diagram pan/zoom interactions are blocked by node dragging.
- Relationship lines are misaligned with node icons (pointing to centers instead of boundaries).
- Selection highlight and relationship line styles do not match the Design app reference.

## Proposed Changes

1. **Fix Interactions**: Update `Kit.tsx` diagram settings to allow right-click panning (`panOnDrag={[1, 2]}`) and disable `selectionOnDrag`.
2. **Geometric Alignment**: Implement rectangular intersection logic in `getNodeIntersection` to account for `220x140` node dimensions.
3. **Styling Parity**:
   - Update `TableAvatar` in `elements.tsx` to support `isSelected` and `isHovered` props.
   - Refactor `KitArtifactNode` to use `TableAvatar` with selection states and update Tailwind rings to `ring-active-base` (red).
   - Update `FloatingEdge` and `FloatingConnectionLine` stroke colors (`accent-secondary`) and widths to match Design app.
   - Remove "triangle" markers from relationship lines.

## Acceptance Criteria

- [x] Panning and zooming work via right-click or scroll wheel.
- [x] Relationship lines terminate precisely at rectangular node boundaries.
- [x] Selection highlight is red (`var(--active-base)`).
- [x] Relationship lines use `var(--accent-secondary)` and correct stroke widths.
