/** 🧬️ Procedural2dConfig */
export interface Procedural2dConfig {
  /** @state local-ui */
  selectedIds: string[];
  /** @state local-ui */
  camera: CameraJson;
  /** @state local-ui */
  showMode: string;
  /** @state local-ui */
  selectedGenerationId?: string;
  /** @state local-ui */
  generationPreviewText?: string;
  /** @state local-ui */
  locale: string;
}

export type CameraJson = { x: number; y: number; zoom: number };
