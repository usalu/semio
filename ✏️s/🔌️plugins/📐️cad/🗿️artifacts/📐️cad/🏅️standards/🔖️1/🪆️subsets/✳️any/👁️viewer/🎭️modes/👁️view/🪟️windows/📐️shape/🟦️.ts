/** 📐️ CAD viewer — Shape window: typed twin of `🦀️.rs`'s view-model. Read-only mirror of
 * the world-3d scene payload `render()` produces — no mutation-shaped fields (no gumball/dislocate,
 * no engagement session), matching the viewer's `ViewEmit`-only contract. */

/** 👁️ One background reference-image overlay, read straight off `CadSnapshot.referencesByModelDefinitionId`. */
export interface CadViewShapeReference {
  id: string;
  url: string;
  origin: [number, number, number];
  widthWorld: number;
  locked: boolean;
  hidden: boolean;
  opacity: number;
}

/** 👁️ The Shape window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (a bare `CadSnapshot`, no runtime/config/utility state: a viewer has none of those). */
export interface CadViewShapeViewModel {
  windowKindId: "cad-view-shape";
  bodyKey: "cad.view.shape";
  surfaceId: "cad.view.scene3d/shape";
  pane: "shape";
  references: CadViewShapeReference[];
}

export const CAD_VIEW_SHAPE_WINDOW_KIND_ID = "cad-view-shape" as const;
export const CAD_VIEW_SHAPE_BODY_KEY = "cad.view.shape" as const;
export const CAD_VIEW_SHAPE_SURFACE_ID = "cad.view.scene3d/shape" as const;
