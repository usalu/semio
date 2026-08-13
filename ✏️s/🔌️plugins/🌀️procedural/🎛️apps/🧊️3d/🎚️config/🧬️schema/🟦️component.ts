/** 🧬️ Procedural3dConfig */
export interface Procedural3dConfig {
  /** @state config */
  selectedNodeIds: string[];
  /** @state config */
  lodMode: string;
  /** @state config */
  showMode: string;
  /** @state config */
  selectionMethod: string;
  /** @state config */
  hoveredNodeId?: string;
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
  /** @state config */
  contributionsJson: string;
}

export type CameraJson = { x: number; y: number; zoom: number };
export type Procedural3dPreviewCamera = {
  position: number[];
  target: number[];
  fov: number;
};
