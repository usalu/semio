/** 🧬️ Generation2dPresence */
export interface Generation2dPresence {
  /** @state presence */
  camera: CameraJson;
  /** @state presence */
  showMode: string;
  /** @state presence */
  selectedGenerationId?: string;
}

export type CameraJson = { x: number; y: number; zoom: number };
