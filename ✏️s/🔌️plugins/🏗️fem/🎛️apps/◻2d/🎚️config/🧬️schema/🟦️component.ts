/** 🧬️ Fem2dConfig */
export interface FemCamera {
  x: number;
  y: number;
  zoom: number;
}

export interface Fem2dConfig {
  /** @state config */
  resultSourceId?: string;
  /** @state config */
  resultMode: string;
  /** @state config */
  resultModeIndex: number;
  /** @state config */
  camera: FemCamera;
  /** @state config */
  locale: string;
}
