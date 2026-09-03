/** 🧬️ EquationCamera */
export interface EquationCamera {
  x: number;
  y: number;
  zoom: number;
}

/** 🧬️ EquationConfig */
export interface EquationConfig {
  /** @state config */
  camera: EquationCamera;
  /** @state config */
  locale: string;
}
