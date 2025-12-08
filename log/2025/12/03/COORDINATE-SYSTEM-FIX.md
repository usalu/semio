---
date: "2025-12-03T10:35:43.796Z"
slug: COORDINATE-SYSTEM-FIX
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
summary: Fix coordinate system transformation for ports and geometry
model: claude-opus-4.5
---

# Coordinate System Transformation Fix

**Date:** 2025-12-03
**Issue:** Semio and Three.js coordinate systems mismatch causing incorrect rendering of ports and geometry

## Problem

The application uses two different coordinate systems:

- **Semio**: X-right, Y-forward, Z-up (right-handed, Y-forward)
- **Three.js**: X-right, Y-up, Z-backward (right-handed, Z-backward)

This mismatch was causing:

1. Ports in the Type app to be displayed at incorrect positions
2. Port directions to point incorrectly
3. Potential issues with plane-based geometry rendering

## Transformation

The coordinate transformation between the two systems is:

- **Semio (x, y, z) → Three.js (x, -z, y)**
- **Three.js (x, y, z) → Semio (x, z, -y)**

This transformation is implemented in [semio.ts](../../js/js/semio.ts#L139-L151):

- `toThreeRotation()` - matrix for Semio → Three.js
- `toSemioRotation()` - matrix for Three.js → Semio

## Changes Made

### 1. Port Rendering ([Type.tsx](../../js/js/sketchpad/Type.tsx))

**Import coordinate transformation functions:**

```typescript
import { toThreeRotation, toSemioRotation } from "../semio";
```

**PortVisual component (lines 1169-1181):**

- Added coordinate transformation when rendering ports
- Port positions are now transformed from Semio to Three.js coordinates
- Port directions are now transformed and normalized correctly

```typescript
// Transform port position from Semio coordinate system to Three.js coordinate system
const position = useMemo(() => {
  const semioPos = new THREE.Vector3(port.point.x, port.point.y, port.point.z);
  const threePos = semioPos.applyMatrix4(toThreeRotation());
  return [threePos.x, threePos.y, threePos.z] as [number, number, number];
}, [port.point]);

// Transform port direction from Semio coordinate system to Three.js coordinate system
const direction = useMemo(() => {
  const semioDir = new THREE.Vector3(port.direction.x, port.direction.y, port.direction.z);
  const threeDir = semioDir.applyMatrix4(toThreeRotation()).normalize();
  return [threeDir.x, threeDir.y, threeDir.z] as [number, number, number];
}, [port.direction]);
```

**Port creation (lines 1600-1632):**

- When users click on the mesh to create a port, the position and normal from Three.js raycasting are now converted back to Semio coordinates before being stored

```typescript
// Convert position and normal from Three.js coordinate system back to Semio coordinate system
const semioPosition = position.clone().applyMatrix4(toSemioRotation());
const semioNormal = normal.clone().applyMatrix4(toSemioRotation()).normalize();
```

### 2. Pieces (Already Correct)

Pieces in the Design app were already correctly using `toThreeRotation()` at [Design.tsx:6966](../../js/js/sketchpad/Design.tsx#L6966) and [Design.tsx:6973](../../js/js/sketchpad/Design.tsx#L6973):

```typescript
const threeMatrix = new THREE.Matrix4().multiplyMatrices(toThreeRotation(), planeMatrix);
```

### 3. Gizmo Labels (Already Correct)

The coordinate gizmo labels at [elements.tsx:4854](../../js/js/sketchpad/elements.tsx#L4854) correctly display the Semio coordinate system axes:

```typescript
const labels = ["X", "Z", "-Y"];
```

This shows users the Semio coordinate system even though the rendering is in Three.js.

## Testing

The changes ensure that:

- ✅ Ports are displayed at correct positions in the 3D scene
- ✅ Port direction arrows point correctly
- ✅ Newly created ports are stored with correct Semio coordinates
- ✅ Pieces continue to render correctly with their plane transformations
- ✅ The gizmo shows the correct coordinate system labels

## Impact

- **Type App**: Port rendering and creation now work correctly
- **Design App**: No changes needed (already working correctly)
- **Coordinate System**: Clear separation between internal (Semio) and rendering (Three.js) coordinate systems

## Related Files

- [js/js/semio.ts](../../js/js/semio.ts) - Coordinate transformation definitions
- [js/js/sketchpad/Type.tsx](../../js/js/sketchpad/Type.tsx) - Port rendering and creation (MODIFIED)
- [js/js/sketchpad/Design.tsx](../../js/js/sketchpad/Design.tsx) - Piece rendering (already correct)
- [js/js/sketchpad/elements.tsx](../../js/js/sketchpad/elements.tsx) - Gizmo configuration
