# pmndrs/drei `<Fisheye>` evaluation

## What drei ships
`@react-three/drei/core/Fisheye` (v10.x):
- Wraps **React children** in `createPortal` → `RenderCubeTexture` (6-face `WebGLCubeRenderTarget`, `HalfFloatType`, DPR-aware resolution)
- Displays the env map on a **sphere** filmed by a fixed orthographic camera
- Props: `zoom`, `segments`, `resolution`, `renderPriority` — **no FOV, strength, or panini**
- Picking: custom `compute` that intersects the sphere and reflects into the cube camera

## What we need
`WorldProjectionSpec.curvilinear`: `{ fov, strength, mapping: "fisheye" | "panini" }`
- Mounted as a **sibling** pass via `WorldProjectionRig` next to orbit controls, gizmo, LOD content
- Strength 0 = rectilinear passthrough; panini is a sibling mapping
- Docstring already rejected cubemap unwrap: one RT vs six, materials stay as-is, taxonomy does not promise ≥180°

## Why not a drop-in
| Concern | drei Fisheye | Our pass |
|---|---|---|
| Integration | Must wrap scene children | Sibling `useFrame` blit |
| Panini / strength / fov | Unsupported | First-class |
| Gizmo / HUD | Would distort if wrapped | Stays undistorted outside pass |
| Picking | Sphere-normal reflect | `worldCurvilinearUnproject` NDC remap |

Adopting drei would mean restructuring `WorldCanvas` / framework+CAD trees, dropping panini/strength, and fisheye-warping chrome unless carefully split — worse than the planar remap for this taxonomy.

## Borrowed from drei anyway
- Capture `type: HalfFloatType` (drei cube RT) for less banding under warp
- Keep prior fixes: `LinearFilter`, DPR size, `LinearSRGBColorSpace`, `colorspace_fragment`, `toneMapped: false`

## Verdict
**Do not use drei `<Fisheye>`.** Keep `WorldCurvilinearPass` as the curvilinear implementation.
