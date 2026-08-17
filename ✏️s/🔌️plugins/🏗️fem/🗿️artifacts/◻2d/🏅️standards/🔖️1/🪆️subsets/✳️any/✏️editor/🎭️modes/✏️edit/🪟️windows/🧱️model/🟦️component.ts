/** 🧱️ Fem2d editor — Model window: typed twin of `🦀️component.rs`'s `render(doc, camera)` boundary —
 * the editable 2D structural canvas (nodes/members/supports, mesh-edge preview overlay). */

export interface Fem2dModelViewModel {
  windowKindId: "fem2d-model";
  bodyKey: "fem2d.play.model";
  camera: { x: number; y: number; zoom: number };
}

export const FEM2D_MODEL_WINDOW_KIND_ID = "fem2d-model" as const;
export const FEM2D_MODEL_BODY_KEY = "fem2d.play.model" as const;
