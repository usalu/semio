/** 🧬️ Generation3dConfig */
export interface Generation3dConfig {
  /** @state config */
  lodMode: string;
  /** @state config */
  showMode: string;
  /** @state config */
  camera: CameraJson;
  /** @state config */
  previewCamera: Generation3dPreviewCamera;
  /** @state config */
  sunJson: string;
  /** @state config */
  selectedGenerationId?: string;
}

export type CameraJson = { x: number; y: number; zoom: number };
export type Generation3dPreviewCamera = {
  position: number[];
  target: number[];
  fov: number;
};
