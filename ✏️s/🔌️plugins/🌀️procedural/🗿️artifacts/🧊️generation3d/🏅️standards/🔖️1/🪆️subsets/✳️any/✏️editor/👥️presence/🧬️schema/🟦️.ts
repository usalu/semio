/** 🧬️ Generation3dPresence */
export interface Generation3dPresence {
  /** @state presence */
  camera: CameraJson;
  /** @state presence */
  previewCamera: Generation3dPreviewCamera;
  /** @state presence */
  activeUtilityId: string;
  /** @state presence */
  showMode: string;
}

export type CameraJson = { x: number; y: number; zoom: number };
export type Generation3dPreviewCamera = {
  position: number[];
  target: number[];
  fov: number;
};
