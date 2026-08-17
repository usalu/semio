/** 🧬️ MathematicalCamera */
export interface MathematicalCamera {
  x: number;
  y: number;
  zoom: number;
}

/** 🧬️ MathematicalConfig */
export interface MathematicalConfig {
  /** @state config */
  camera: MathematicalCamera;
  /** @state config */
  locale: string;
}
