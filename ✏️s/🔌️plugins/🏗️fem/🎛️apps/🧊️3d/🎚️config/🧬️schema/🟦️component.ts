/** 🧬️ Fem3dConfig */
export interface FemCamera {
  json: string;
}

export interface Fem3dConfig {
  /** @state local-ui */
  resultSourceId?: string;
  /** @state local-ui */
  resultMode: string;
  /** @state local-ui */
  resultModeIndex: number;
  /** @state local-ui */
  camera: FemCamera;
}
