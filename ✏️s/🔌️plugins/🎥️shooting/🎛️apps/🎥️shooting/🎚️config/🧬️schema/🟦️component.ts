/** 🧬️ ShootingConfig */
export interface ShootingConfig {
  /** @state local-ui */
  defaultShotFormat: string;
  /** @state local-ui */
  defaultShotShape: string;
  /** @state local-ui */
  defaultAssetFormat: string;
  /** @state local-ui */
  selectedShotIds: string[];
  /** @state local-ui */
  selectedAssetIds: string[];
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  hoveredAssetId?: string;
  /** @state local-ui */
  centerModel: boolean;
  /** @state local-ui */
  fitRevision: number;
  /** @state local-ui */
  cameraDraftLabel: string;
  /** @state local-ui */
  camera: ShootingCamera;
  /** @state local-ui */
  activeUtilityId: string;
  /** @state local-ui */
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
