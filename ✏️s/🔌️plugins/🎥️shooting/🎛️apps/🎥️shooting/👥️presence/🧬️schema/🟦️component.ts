/** 🧬️ ShootingPresence */
export interface ShootingPresence {
  /** @state shared-ui */
  selectedShotIds: string[];
  /** @state shared-ui */
  selectedAssetIds: string[];
  /** @state shared-ui */
  hoveredAssetId?: string;
  /** @state shared-ui */
  camera: ShootingCamera;
  /** @state shared-ui */
  activeUtilityId: string;
}

export interface ShootingCamera {
  position: [number, number, number];
  target: [number, number, number];
  zoom: number;
  fov: number;
  up?: [number, number, number];
  projection?: string;
}
