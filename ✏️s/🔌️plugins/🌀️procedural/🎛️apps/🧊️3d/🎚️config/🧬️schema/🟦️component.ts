/** 🧬️ Procedural3dConfig */
export interface Procedural3dConfig {
  /** @state config */
  lodMode: string;
  /** @state config */
  showMode: string;
  /** @state config */
  camera: CameraJson;
  /** @state config */
  previewCamera: Procedural3dPreviewCamera;
  /** @state config */
  sunJson: string;
  /** @state config */
  selectedGenerationId?: string;
  /** @state config */
  generationPreviewText?: string;
  /** @state config */
  activeUtilityId: string;
  /** @state config */
  locale: string;
}

export type CameraJson = { x: number; y: number; zoom: number };
export type Procedural3dPreviewCamera = {
  position: number[];
  target: number[];
  fov: number;
};
