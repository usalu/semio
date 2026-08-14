/** 🧬️ FlowPresence */
export interface FlowPresence {
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
