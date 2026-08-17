/** 📊️ FEM 3D editor — Results window: typed twin of `🦀️component.rs`'s
 * `render(doc: &Fem3dSnapshot, cfg: &Fem3dConfig)` boundary — static/modal/buckling analysis views
 * over the same node/member/solid scene the Model window renders, dispatched off `Fem3dConfig`'s
 * `resultMode`/`resultSourceId`/`resultModeIndex` fields (see `../../../../🎚️config/🟦️component.ts`). */

export type Fem3dResultDisplayMode = "static" | "modal" | "buckling";

export interface Fem3dResultsViewModel {
  windowKindId: "fem3d-results";
  bodyKey: "fem3d.play.results";
  resultSourceId: string | null;
  resultMode: Fem3dResultDisplayMode;
  resultModeIndex: number;
}

export const FEM3D_WINDOW_RESULTS = "fem3d-results" as const;
export const FEM3D_BODY_RESULTS = "fem3d.play.results" as const;
