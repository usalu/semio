/** 📊️ Fem2d editor — Results window: typed twin of `🦀️component.rs`'s `render(doc, display, camera)`
 * boundary — static/modal/buckling analysis views, nodal-averaged von-Mises stress contours, reaction
 * labels and moment diagrams. */

export type Fem2dResultDisplayMode = { kind: "static" } | { kind: "modal"; modeIndex: number } | { kind: "buckling"; modeIndex: number };

export interface Fem2dResultsViewModel {
  windowKindId: "fem2d-results";
  bodyKey: "fem2d.play.results";
  display: { sourceId?: string; mode: Fem2dResultDisplayMode };
  camera: { x: number; y: number; zoom: number };
}

export const FEM2D_RESULTS_WINDOW_KIND_ID = "fem2d-results" as const;
export const FEM2D_RESULTS_BODY_KEY = "fem2d.play.results" as const;
