/** 💡️ wfc3d solve inference — the assignment derived from the persisted problem spec, never stored. */
export interface Wfc3dInferenceCommit {
  assignments: Record<string, string>;
}

export const WFC3D_INFERENCE_TOOL_ID = "s.wfc.wfc3d.solve";
export const WFC3D_INFERENCE_PAYLOAD_SCHEMA = "s.wfc.wfc3d.inference.request.v1";
