/** 💡 s.wfc.grid3d.solve — the routed solve inference's wire twin. The commit is one
 * `{x, y, z, tileId}` row per solved, unmasked cell; masked cells never appear. */

import type { Grid3dSnapshot } from "../📸️snapshot/🟦️.ts";

export const GRID3D_INFERENCE_TOOL_ID = "s.wfc.grid3d.solve";
export const GRID3D_INFERENCE_PAYLOAD_SCHEMA = "s.wfc.grid3d.inference.request.v1";

export interface Grid3dAssignment {
  x: number;
  y: number;
  z: number;
  tileId: string;
}

export interface Grid3dInferenceRequest {
  snapshot: Grid3dSnapshot;
  checkpoint?: number[] | null;
}

export interface Grid3dInferenceCommit {
  satisfiable: boolean;
  assignments: Grid3dAssignment[];
}

/** 🩺 Whether a commit covers exactly the cells the grid has to fill. An unsatisfiable answer
 * carries no rows at all, so it is reported as not covering rather than as a failure. */
export function coversEveryUnmaskedCell(snapshot: Grid3dSnapshot, commit: Grid3dInferenceCommit): boolean {
  const cells = snapshot.width * snapshot.height * snapshot.depth - snapshot.masked.length;
  return commit.satisfiable && commit.assignments.length === cells;
}
