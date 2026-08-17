/** 🧱️ Fem2d viewer — Model window: typed twin of `🦀️component.rs`'s view-model. Read-only mirror of
 * the canvas-2d scene payload `render()` produces — no mutation-shaped fields (no selection, no
 * gumball), matching the viewer's `ViewEmit`-only contract. */

/** 👁️ The Model window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (a bare `Fem2dSnapshot`, no runtime/config/utility state: a viewer has none of those). */
export interface Fem2dViewModelViewModel {
  windowKindId: "fem2d-view-model";
  bodyKey: "fem2d.view.model";
  controllerId: "fem2d-view";
}

export const FEM2D_VIEW_MODEL_WINDOW_KIND_ID = "fem2d-view-model" as const;
export const FEM2D_VIEW_MODEL_BODY_KEY = "fem2d.view.model" as const;
export const FEM2D_VIEW_CONTROLLER_ID = "fem2d-view" as const;
