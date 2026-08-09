/** 🧬️ Procedural2dPresence */
export interface Procedural2dPresence {
  /** @state shared-ui */
  selectedIds: string[];
  /** @state shared-ui */
  camera: CameraJson;
  /** @state shared-ui */
  showMode: string;
  /** @state shared-ui */
  selectedGenerationId?: string;
}

export type CameraJson = { x: number; y: number; zoom: number };
