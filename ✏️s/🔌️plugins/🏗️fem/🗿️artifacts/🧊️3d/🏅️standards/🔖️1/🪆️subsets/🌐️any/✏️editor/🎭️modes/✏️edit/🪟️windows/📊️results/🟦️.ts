/** 📊️ Results view projection of the concrete window's `Fem3dResultsWindowConfig`.
 * @see ./🎚️config/🧬️schema/🟦️.ts */

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
