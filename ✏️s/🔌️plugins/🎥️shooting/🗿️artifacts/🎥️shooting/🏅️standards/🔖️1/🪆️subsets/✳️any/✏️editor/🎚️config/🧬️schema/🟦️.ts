/** 🧬️ ShootingConfig */
export interface ShootingConfig {
  /** @state config */
  defaultShotFormat: string;
  /** @state config */
  defaultShotShape: string;
  /** @state config */
  defaultAssetFormat: string;
  /** @state config */
  selectedShotIds: string[];
  /** @state config */
  centerModel: boolean;
  /** @state config */
  fitRevision: number;
  /** @state config */
  cameraDraftLabel: string;
  /** @state config */
  camera: ShootingCamera;
  /** @state config */
  activeUtilityId: string;
  /** @state config */
  locale: string;
}

export interface ShootingCamera {
  position: [number, number, number];
  target: [number, number, number];
  zoom: number;
  fov: number;
  up?: [number, number, number];
  projection?: string;
}
