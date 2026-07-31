# Ticket

## Todos

# Coordinate System Transformation Fix

**Date:** 2025-12-03
**Problem:** compose and Three.js coordinate systems mismatch causing incorrect rendering of connectors and geometry

## Problem

The application uses two different coordinate systems:

- **Compose**: X-right, Y-forward, Z-up (right-handed, Y-forward)
- **Three.js**: X-right, Y-up, Z-backward (right-handed, Z-backward)

This mismatch was causing:

1. Connectors in the Type app to be displayed at incorrect positions
2. Connector directions to point incorrectly
3. Potential issues with plane-based geometry rendering

## Transformation

The coordinate transformation between the two systems is:

- **compose (x, y, z) → Three.js (x, -z, y)**
- **Three.js (x, y, z) → compose (x, z, -y)**

This transformation is implemented in [compose.ts](../../compose/compose/compose.ts#L139-L151):

- `toThreeRotation()` - matrix for compose → Three.js
- `toComposeRotation()` - matrix for Three.js → Compose

## Changes Made

### 1. Connector Rendering ([Type.tsx](../../compose/compose/sketchpad/Type.tsx))

**Import coordinate transformation functions:**

```typescript
import { toThreeRotation, toComposeRotation } from "../compose";
```

**ConnectorVisual component (lines 1169-1181):**

- Added coordinate transformation when rendering connectors
- Connector positions are now transformed from compose to Three.js coordinates
- Connector directions are now transformed and normalized correctly

```typescript
// Transform connector position from compose coordinate system to Three.js coordinate system
const position = useMemo(() => {
 const composePos = new THREE.Vector3(connector.point.x, connector.point.y, connector.point.z);
 const threePos = composePos.applyMatrix4(toThreeRotation());
 return [threePos.x, threePos.y, threePos.z] as [number, number, number];
}, [connector.point]);

// Transform connector direction from compose coordinate system to Three.js coordinate system
const direction = useMemo(() => {
 const composeDir = new THREE.Vector3(connector.direction.x, connector.direction.y, connector.direction.z);
 const threeDir = composeDir.applyMatrix4(toThreeRotation()).normalize();
 return [threeDir.x, threeDir.y, threeDir.z] as [number, number, number];
}, [connector.direction]);
```

**Connector creation (lines 1600-1632):**

- When users click on the mesh to create a connector, the position and normal from Three.js raycasting are now converted back to compose coordinates before being stored

```typescript
// Convert position and normal from Three.js coordinate system back to compose coordinate system
const composePosition = position.clone().applyMatrix4(toComposeRotation());
const composeNormal = normal.clone().applyMatrix4(toComposeRotation()).normalize();
```

### 2. Pieces (Already Correct)

Pieces in the Design app were already correctly using `toThreeRotation()` at [Design.tsx:6966](../../compose/compose/sketchpad/Design.tsx#L6966) and [Design.tsx:6973](../../compose/compose/sketchpad/Design.tsx#L6973):

```typescript
const threeMatrix = new THREE.Matrix4().multiplyMatrices(toThreeRotation(), planeMatrix);
```

### 3. Gizmo Labels (Already Correct)

The coordinate gizmo labels at [elements.tsx:4854](../../compose/compose/sketchpad/elements.tsx#L4854) correctly display the compose coordinate system axes:

```typescript
const labels = ["X", "Z", "-Y"];
```

This shows users the compose coordinate system even though the rendering is in Three.js.

## Testing

The changes ensure that:

- ✅ Connectors are displayed at correct positions in the 3D scene
- ✅ Connector direction arrows point correctly
- ✅ Newly created connectors are stored with correct compose coordinates
- ✅ Pieces continue to render correctly with their plane transformations
- ✅ The gizmo shows the correct coordinate system labels

## Impact

- **Type App**: Connector rendering and creation now work correctly
- **Design App**: No changes needed (already working correctly)
- **Coordinate System**: Clear separation between internal (Compose) and rendering (Three.js) coordinate systems

## Related Files

- [js/compose/compose.ts](../../compose/compose/compose.ts) - Coordinate transformation definitions
- [js/compose/sketchpad/Type.tsx](../../compose/compose/sketchpad/Type.tsx) - Connector rendering and creation (MODIFIED)
- [js/compose/sketchpad/Design.tsx](../../compose/compose/sketchpad/Design.tsx) - Piece rendering (already correct)
- [js/compose/sketchpad/elements.tsx](../../compose/compose/sketchpad/elements.tsx) - Gizmo configuration

## Changes

## Log

## Summary

# Summary

Fix coordinate system transformation for connectors and geometry
