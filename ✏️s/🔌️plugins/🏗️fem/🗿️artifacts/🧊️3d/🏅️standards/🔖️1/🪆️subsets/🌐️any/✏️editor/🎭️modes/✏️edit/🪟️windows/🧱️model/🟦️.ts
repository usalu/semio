/** 🧱️ FEM 3D editor — Model window: typed twin of `🦀️.rs`'s
 * `render(doc: &Fem3dSnapshot, camera: &FemCamera)` boundary — the undeformed structure (nodes,
 * bar/frame members, meshed solids) as a World3d scene, no displacement offset, no stress coloring. */

export interface Fem3dModelViewModel {
  windowKindId: "fem3d-model";
  bodyKey: "fem3d.play.model";
  surfaceId: "fem3d.play.model";
}

export const FEM3D_WINDOW_MODEL = "fem3d-model" as const;
export const FEM3D_BODY_MODEL = "fem3d.play.model" as const;
