/** 🧬️ s.wfc.grid2d artifact schema — the persisted WFC problem for a regular 2D grid. */

export {
  type Grid2dSnapshot,
  type WfcTile2d,
  type WfcTileMedia2d,
  type WfcAdjacencyRule2d,
  type WfcPinnedCell2d,
  type WfcCell2d,
  type WfcColor,
  type WfcPoint2,
  type WfcPathSegment,
  type WfcVectorPath,
  type WfcDirection2d,
} from "./📸️snapshot/🟦️.ts";

export interface Grid2dArtifact {
  /** @state artifact */
  snapshot: import("./📸️snapshot/🟦️.ts").Grid2dSnapshot;
}
