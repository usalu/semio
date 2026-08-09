/** 🧬️ Procedural3dPresence */
export interface Procedural3dPresence {
  /** @state shared-ui */
  selectedNodeIds: string[];
  /** @state shared-ui */
  hoveredNodeId?: string;
  /** @state shared-ui */
  camera: CameraJson;
  /** @state shared-ui */
  previewCamera: Procedural3dPreviewCamera;
  /** @state shared-ui */
  selectionMethod: string;
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state shared-ui */
  showMode: string;
}

export type CameraJson = { x: number; y: number; zoom: number };
export type Procedural3dPreviewCamera = {
  position: number[];
  target: number[];
  fov: number;
};
