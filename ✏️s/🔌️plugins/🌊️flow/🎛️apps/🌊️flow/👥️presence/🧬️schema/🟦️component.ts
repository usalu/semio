/** 🧬️ FlowPresence */
export interface FlowPresence {
  /** @state shared-ui */
  selectedNodeIds: string[];
  /** @state shared-ui */
  selectedEdgeIds: string[];
  /** @state shared-ui */
  selectedHandleIds: string[];
  /** @state shared-ui */
  previewOffNodeIds: string[];
  /** @state shared-ui */
  camera: CameraJson;
}

export interface CameraJson {
  x: number;
  y: number;
  zoom: number;
}
