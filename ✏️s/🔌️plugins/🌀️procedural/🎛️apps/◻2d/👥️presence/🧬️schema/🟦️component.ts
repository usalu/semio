/** 🧬️ Procedural2dPresence */
export interface Procedural2dPresence {
  /** @state presence */
  selectedIds: string[];
  /** @state presence */
  camera: CameraJson;
  /** @state presence */
  showMode: string;
  /** @state presence */
  selectedGenerationId?: string;
}

export type CameraJson = { x: number; y: number; zoom: number };
