/** 🧬️ Shooting artifact schema — every field with its state class. */

export interface ShootingArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  assets: ShootingAsset[];
  /** @state persistent */
  savedCameras: ShootingSavedCamera[];
  /** @state persistent */
  scene: ShootingSceneLighting;
  /** @state persistent */
  shots: ShootingShot[];
  /** @state persistent */
  activeShotId: string;
  /** @state persistent */
  activeAssetId: string;
  /** @state shared-ui */
  selectedShotIds: string[];
  /** @state shared-ui */
  selectedAssetIds: string[];
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state local-ui */
  defaultShotFormat: string;
  /** @state local-ui */
  defaultShotShape: string;
  /** @state local-ui */
  defaultAssetFormat: string;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  centerModel: boolean;
  /** @state local-ui */
  fitRevision: number;
  /** @state local-ui */
  cameraDraftLabel: string;
  /** @state local-ui */
  camera: ShootingCamera;
  /** @state local-ui */
  locale: string;
  /** @state preview */
  hoveredAssetId?: string;
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
