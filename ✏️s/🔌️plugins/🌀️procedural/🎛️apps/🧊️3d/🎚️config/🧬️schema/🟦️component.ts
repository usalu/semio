/** 🧬️ Procedural3dConfig */
export interface Procedural3dConfig {
  /** @state local-ui */
  selectedNodeIds: string[];
  /** @state local-ui */
  lodMode: string;
  /** @state local-ui */
  showMode: string;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  hoveredNodeId?: string;
  /** @state local-ui */
  camera: CameraJson;
  /** @state local-ui */
  previewCamera: Procedural3dPreviewCamera;
  /** @state local-ui */
  sunJson: string;
  /** @state local-ui */
  selectedGenerationId?: string;
  /** @state local-ui */
  generationPreviewText?: string;
  /** @state local-ui */
  activeUtilityId: string;
  /** @state local-ui */
  locale: string;
  /** @state local-ui */
  contributionsJson: string;
}

export type CameraJson = { x: number; y: number; zoom: number };
export type Procedural3dPreviewCamera = {
  position: number[];
  target: number[];
  fov: number;
};
