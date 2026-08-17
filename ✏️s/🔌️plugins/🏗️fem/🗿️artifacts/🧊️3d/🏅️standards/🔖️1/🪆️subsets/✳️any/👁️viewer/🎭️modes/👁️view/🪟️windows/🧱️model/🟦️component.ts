/** 🧱️ FEM 3D viewer — Model window: typed twin of `🦀️component.rs`'s `render(doc: &Fem3dSnapshot)`
 * boundary. Read-only mirror of the world-3d scene payload `render()` produces — no mutation-shaped
 * fields (no result-mode/camera config, no engagement session), matching the viewer's `ViewEmit`-only
 * contract. Renders the exact same undeformed structure the editor's Model window renders, minus any
 * per-session camera (the viewer always uses the framework default). */

export interface Fem3dViewModelViewModel {
  windowKindId: "fem3d-view-model";
  bodyKey: "fem3d.view.model";
}

export const FEM3D_VIEW_MODEL_WINDOW_KIND_ID = "fem3d-view-model" as const;
export const FEM3D_VIEW_MODEL_BODY_KEY = "fem3d.view.model" as const;
