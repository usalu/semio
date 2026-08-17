/** 🧬️ ShootingPresence */
export interface ShootingPresence {
  /** @state presence */
  selectedShotIds: string[];
  /** @state presence */
  camera: ShootingCamera;
  /** @state presence */
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
