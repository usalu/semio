/** 🧬️ FlowPresence */
export interface FlowPresence {
  /** @state presence */
  selectedNodeIds: string[];
  /** @state presence */
  selectedEdgeIds: string[];
  /** @state presence */
  selectedHandleIds: string[];
  /** @state presence */
  previewOffNodeIds: string[];
  /** @state presence */
  camera: CameraJson;
}

export interface CameraJson {
  x: number;
  y: number;
  zoom: number;
}
