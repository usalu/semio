---
name: fix brush upside-down flip
overview: Fix puzzle3d brush predictions flipping upside down by handling the anti-parallel edge case of THREE.Quaternion.setFromUnitVectors with a deterministic 180-degree rotation, mirroring the existing Rhino/Grasshopper fix.
todos:
 - id: ticket
   content: Read repo://goals and open/reopen a ticket for the brush upside-down flip fix
   status: completed
 - id: helpers
   content: Add vec3Dot and vec3Cross helpers (with emoji docstrings) near normalizeVec3Cad in puzzle/3d/react/index.tsx
   status: completed
 - id: fix
   content: Add anti-parallel edge-case guard in computeBrushPlacementPose with deterministic 180-degree rotation (Z axis when horizontal, cross(Z,dir) otherwise)
   status: completed
 - id: tests
   content: Extend existing computeBrushPlacementPose tests with horizontal and vertical collinear cases proving no upside-down flip
   status: completed
 - id: validate
   content: Run puzzle3d react vitest suite and confirm green
   status: completed
 - id: close
   content: Close the ticket with summary of changed files
   status: completed
isProject: false
---

# Fix Puzzle3d Brush Prediction Upside-Down Flip

## Root cause

Brush placement orientation is computed in `computeBrushPlacementPose` ([puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) ~line 3128) via:

```ts
const qThree = new Quaternion().setFromUnitVectors(new Vector3(...localDir), new Vector3(...desiredWorldDir));
```

`THREE.Quaternion.setFromUnitVectors(vFrom, vTo)` has the same singularity as Rhino `Transform.Rotation`: when `vFrom` and `vTo` are anti-parallel (here when `localDir == targetDir`, since `desiredWorldDir = -targetDir`), there are infinitely many perpendicular axes and three.js picks one arbitrarily (`Y`/`X`). In this file's CAD frame Z is up, so that arbitrary horizontal axis flips the object upside down.

This is the exact case already solved on the C# side in `ComputeChildPlane` ([compose/client/ui/gh/Compose.Grasshopper/Compose.Grasshopper.cs](compose/client/ui/gh/Compose.Grasshopper/Compose.Grasshopper.cs) lines 305-320): when directions are parallel, flip 180 deg about `ZAxis` if the direction is flat on XY, else about `cross(ZAxis, dir)`.

## Change

In `computeBrushPlacementPose`, replace the single `setFromUnitVectors` call with an anti-parallel guard that mirrors the C# logic. The axis is chosen from the target direction (CAD Z-up frame):

- if `|dir.z| < tol` (horizontal): rotate 180 deg about Z axis `[0,0,1]` (object spins, stays upright)
- else: rotate 180 deg about `normalize(cross(Zaxis, dir))`

A 180 deg quaternion about unit axis `a` is `[a.x, a.y, a.z, 0]`.

Detection: `dot(localDir, desiredWorldDir) < -1 + tol` (use `tol = 1e-6`), so near-collinear cases are caught robustly, not just the exact three.js singularity. Non-anti-parallel cases keep using `setFromUnitVectors` unchanged.

Add two tiny local helpers near the existing vec helpers (`normalizeVec3Cad`/`negateVec3Cad`, ~line 2945): `vec3Dot` and `vec3Cross` (no general cross/dot helper exists yet). Each gets the repo-required emoji docstring.

### Sketch

```ts
const localDir = normalizeVec3Cad(args.sourceLocalDirection);
const targetDir = normalizeVec3Cad(args.targetWorldDirectionCad);
const desiredWorldDir = negateVec3Cad(targetDir);
let orientation: Quat;
if (vec3Dot(localDir, desiredWorldDir) < -1 + 1e-6) {
 const axis = Math.abs(targetDir[2]) < 1e-6 ? ([0, 0, 1] as Vec3) : normalizeVec3Cad(vec3Cross([0, 0, 1], targetDir));
 orientation = [axis[0], axis[1], axis[2], 0];
} else {
 const q = new Quaternion().setFromUnitVectors(new Vector3(...localDir), new Vector3(...desiredWorldDir));
 orientation = [q.x, q.y, q.z, q.w];
}
```

The downstream `origin = targetWorldPositionCad - quatRotateVec(orientation, scaledLocal)` is unchanged.

## Tests

Extend the existing test block in [puzzle/3d/react/index.tsx](puzzle/3d/react/index.tsx) (next to the `computeBrushPlacementPose` tests ~line 10358, do not create new test files). Add cases that previously flipped upside down:

- Horizontal collinear: `sourceLocalDirection [1,0,0]`, `targetWorldDirectionCad [1,0,0]` -> assert world direction opposes target and that a local up vector `[0,0,1]` stays up (world up Z stays `+1`, proving no upside-down flip).
- Vertical collinear: `sourceLocalDirection [0,0,1]`, `targetWorldDirectionCad [0,0,1]` -> assert directions oppose and the result is a stable 180 deg flip.

## Validation

- Run the puzzle3d react tests via the project's nx/launch config (vitest) and confirm all pass.

## Repo workflow

- Read `repo://goals`, open/reopen a ticket for this fix before editing, and close it with a summary of touched files when done (only `puzzle/3d/react/index.tsx` is expected to change).
