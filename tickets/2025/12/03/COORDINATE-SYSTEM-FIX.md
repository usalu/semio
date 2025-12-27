---
slug: COORDINATE-SYSTEM-FIX
summary: Fix coordinate system transformation for connectors and geometry
prompt: Fix coordinate system transformation for connectors and geometry
status: finished
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: "2025-12-16T17:06:07.828Z"
commit: "0000000000000000000000000000000000000000"
iterations: []
---

# Coordinate System Transformation Fix

**Date:** 2025-12-03
**Problem:** semio and Three.js coordinate systems mismatch causing incorrect rendering of connectors and geometry

## Problem

The application uses two different coordinate systems:

- **Semio**: X-right, Y-forward, Z-up (right-handed, Y-forward)
- **Three.js**: X-right, Y-up, Z-backward (right-handed, Z-backward)

This mismatch was causing:

1. Connectors in the Type app to be displayed at incorrect positions
2. Connector directions to point incorrectly
3. Potential issues with plane-based geometry rendering

## Transformation

The coordinate transformation between the two systems is:

- **semio (x, y, z) → Three.js (x, -z, y)**
- **Three.js (x, y, z) → semio (x, z, -y)**

This transformation is implemented in [semio.ts](../../js/js/semio.ts#L139-L151):

- `toThreeRotation()` - matrix for semio → Three.js
- `toSemioRotation()` - matrix for Three.js → Semio

## Changes Made

### 1. Connector Rendering ([Type.tsx](../../js/js/sketchpad/Type.tsx))

**Import coordinate transformation functions:**

```typescript
import { toThreeRotation, toSemioRotation } from "../semio";
```

**ConnectorVisual component (lines 1169-1181):**

- Added coordinate transformation when rendering connectors
- Connector positions are now transformed from semio to Three.js coordinates
- Connector directions are now transformed and normalized correctly

```typescript
// Transform connector position from semio coordinate system to Three.js coordinate system
const position = useMemo(() => {
  const semioPos = new THREE.Vector3(connector.point.x, connector.point.y, connector.point.z);
  const threePos = semioPos.applyMatrix4(toThreeRotation());
  return [threePos.x, threePos.y, threePos.z] as [number, number, number];
}, [connector.point]);

// Transform connector direction from semio coordinate system to Three.js coordinate system
const direction = useMemo(() => {
  const semioDir = new THREE.Vector3(connector.direction.x, connector.direction.y, connector.direction.z);
  const threeDir = semioDir.applyMatrix4(toThreeRotation()).normalize();
  return [threeDir.x, threeDir.y, threeDir.z] as [number, number, number];
}, [connector.direction]);
```

**Connector creation (lines 1600-1632):**

- When users click on the mesh to create a connector, the position and normal from Three.js raycasting are now converted back to semio coordinates before being stored

```typescript
// Convert position and normal from Three.js coordinate system back to semio coordinate system
const semioPosition = position.clone().applyMatrix4(toSemioRotation());
const semioNormal = normal.clone().applyMatrix4(toSemioRotation()).normalize();
```

### 2. Pieces (Already Correct)

Pieces in the Design app were already correctly using `toThreeRotation()` at [Design.tsx:6966](../../js/js/sketchpad/Design.tsx#L6966) and [Design.tsx:6973](../../js/js/sketchpad/Design.tsx#L6973):

```typescript
const threeMatrix = new THREE.Matrix4().multiplyMatrices(toThreeRotation(), planeMatrix);
```

### 3. Gizmo Labels (Already Correct)

The coordinate gizmo labels at [elements.tsx:4854](../../js/js/sketchpad/elements.tsx#L4854) correctly display the semio coordinate system axes:

```typescript
const labels = ["X", "Z", "-Y"];
```

This shows users the semio coordinate system even though the rendering is in Three.js.

## Testing

The changes ensure that:

- ✅ Connectors are displayed at correct positions in the 3D scene
- ✅ Connector direction arrows point correctly
- ✅ Newly created connectors are stored with correct semio coordinates
- ✅ Pieces continue to render correctly with their plane transformations
- ✅ The gizmo shows the correct coordinate system labels

## Impact

- **Type App**: Connector rendering and creation now work correctly
- **Design App**: No changes needed (already working correctly)
- **Coordinate System**: Clear separation between internal (Semio) and rendering (Three.js) coordinate systems

## Related Files

- [js/js/semio.ts](../../js/js/semio.ts) - Coordinate transformation definitions
- [js/js/sketchpad/Type.tsx](../../js/js/sketchpad/Type.tsx) - Connector rendering and creation (MODIFIED)
- [js/js/sketchpad/Design.tsx](../../js/js/sketchpad/Design.tsx) - Piece rendering (already correct)
- [js/js/sketchpad/elements.tsx](../../js/js/sketchpad/elements.tsx) - Gizmo configuration
