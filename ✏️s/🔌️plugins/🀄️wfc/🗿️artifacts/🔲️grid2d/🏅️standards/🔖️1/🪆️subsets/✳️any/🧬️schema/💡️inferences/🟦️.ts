/** 💡️ s.wfc.grid2d solve inference — the derived answer over the persisted problem. Rows are
 * positional tuples on the wire (`[x, y, tileId]`), matching the Rust `Vec<(u32, u32, String)>`. */
export type Grid2dAssignment = [number, number, string];
export type Grid2dCellEntropy = [number, number, number];

export interface Grid2dInferenceCommit {
  assignments: Grid2dAssignment[];
  contradiction: boolean;
  entropy: Grid2dCellEntropy[];
}

export const GRID2D_INFERENCE_TOOL_ID = "s.wfc.grid2d.solve";
export const GRID2D_INFERENCE_PAYLOAD_SCHEMA = "s.wfc.grid2d.inference.request.v1";
