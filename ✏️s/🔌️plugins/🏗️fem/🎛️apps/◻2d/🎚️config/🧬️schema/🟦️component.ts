/** 🧬️ Fem2dConfig */
export interface FemCamera {
  x: number;
  y: number;
  zoom: number;
}

export interface Fem2dConfig {
  /** @state local-ui */
  resultSourceId?: string;
  /** @state local-ui */
  resultMode: string;
  /** @state local-ui */
  resultModeIndex: number;
  /** @state local-ui */
  camera: FemCamera;
  /** @state local-ui */
  locale: string;
}
