/** 🧬️ Shooting artifact schema — every field with its state class. */

export interface ShootingArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  assets: ShootingAsset[];
  /** @state artifact */
  savedCameras: ShootingSavedCamera[];
  /** @state artifact */
  scene: ShootingSceneLighting;
  /** @state artifact */
  shots: ShootingShot[];
  /** @state artifact */
  activeShotId: string;
  /** @state artifact */
  activeAssetId: string;
  /** @state presence */
  selectedShotIds: string[];
  /** @state presence */
  activeUtilityId: string;
  /** @state config */
  defaultShotFormat: string;
  /** @state config */
  defaultShotShape: string;
  /** @state config */
  defaultAssetFormat: string;
  /** @state config */
  centerModel: boolean;
  /** @state config */
  fitRevision: number;
  /** @state config */
  cameraDraftLabel: string;
  /** @state config */
  camera: ShootingCamera;
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

export interface ShootingSavedCamera {
  id: string;
  label: string;
  camera: ShootingCamera;
}

export interface ShootingAsset {
  id: string;
  name: string;
  url: string;
  format: string;
  origin: [number, number, number];
  orientation?: [number, number, number, number];
  scale?: [number, number, number];
}

export interface ShootingShot {
  id: string;
  label: string;
  width: number;
  height: number;
  format: string;
  shape: string;
  background?: string;
  cameraId?: string;
}

export interface ShootingSun {
  enabled: boolean;
  azimuth: number;
  elevation: number;
  intensity: number;
  color: string;
}

export interface ShootingAmbient {
  intensity: number;
  color: string;
}

export interface ShootingShadow {
  enabled: boolean;
  opacity: number;
  softness: number;
}

export interface ShootingMaterial {
  color: string;
  metalness: number;
  roughness: number;
  emissive: string;
  emissiveIntensity: number;
}

export interface ShootingSceneLighting {
  background: string;
  sun: ShootingSun;
  ambient: ShootingAmbient;
  shadow: ShootingShadow;
  material: ShootingMaterial;
  emblemBase64?: string;
}
