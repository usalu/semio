/** 🧬️ Procedural3dPresence */
export interface Procedural3dPresence {
  /** @state presence */
  camera: CameraJson;
  /** @state presence */
  previewCamera: Procedural3dPreviewCamera;
  /** @state presence */
  activeUtilityId: string;
  /** @state presence */
  showMode: string;
}

export type CameraJson = { x: number; y: number; zoom: number };
export type Procedural3dPreviewCamera = {
  position: number[];
  target: number[];
  fov: number;
};
