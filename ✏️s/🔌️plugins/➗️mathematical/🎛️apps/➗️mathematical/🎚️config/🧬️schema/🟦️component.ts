/** 🧬️ MathematicalCamera */
export interface MathematicalCamera {
  x: number;
  y: number;
  zoom: number;
}

/** 🧬️ MathematicalConfig */
export interface MathematicalConfig {
  /** @state local-ui */
  camera: MathematicalCamera;
  /** @state local-ui */
  locale: string;
}
