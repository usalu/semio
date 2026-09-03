/** 🧬️ Generation2dConfig */
export interface Generation2dConfig {
  /** @state config */
  camera: CameraJson;
  /** @state config */
  showMode: string;
  /** @state config */
  selectedGenerationId?: string;
  /** @state config */
  generationPreviewText?: string;
  /** @state config */
  locale: string;
}

export type CameraJson = { x: number; y: number; zoom: number };
