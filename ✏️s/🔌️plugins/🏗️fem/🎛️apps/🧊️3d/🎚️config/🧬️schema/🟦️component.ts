/** 🧬️ Fem3dConfig */
export interface FemCamera {
  json: string;
}

export interface Fem3dConfig {
  /** @state config */
  resultSourceId?: string;
  /** @state config */
  resultMode: string;
  /** @state config */
  resultModeIndex: number;
  /** @state config */
  camera: FemCamera;
}
