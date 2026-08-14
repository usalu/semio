/** 🧬️ Shooting diff schema — sparse field delta over the artifact. */

export interface ShootingDiff {
  /** @state artifact */
  artifact?: ShootingArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  assets?: ShootingAssetsDelta;
  /** @state artifact */
  savedCameras?: ShootingSavedCamerasDelta;
  /** @state artifact */
  scene?: ShootingSceneLighting;
  /** @state artifact */
  shots?: ShootingShotsDelta;
  /** @state artifact */
  activeShotId?: string;
  /** @state artifact */
  activeAssetId?: string;
  /** @state presence */
  selectedShotIds?: ShootingStringList;
  /** @state presence */
  activeUtilityId?: string;
  /** @state config */
  defaultShotFormat?: string;
  /** @state config */
  defaultShotShape?: string;
  /** @state config */
  defaultAssetFormat?: string;
  /** @state config */
  centerModel?: boolean;
  /** @state config */
  fitRevision?: number;
  /** @state config */
  cameraDraftLabel?: string;
  /** @state config */
  camera?: ShootingCamera;
  /** @state config */
  locale?: string;
}

export interface ShootingStringList {
  values: string[];
}

export interface ShootingAssetsDelta {
  added: ShootingAsset[];
  removed: string[];
  patched: ShootingAssetPatchEntry[];
  reordered?: string[];
}

export interface ShootingShotsDelta {
  added: ShootingShot[];
  removed: string[];
  patched: ShootingShotPatchEntry[];
  reordered?: string[];
}

export interface ShootingSavedCamerasDelta {
  added: ShootingSavedCamera[];
  removed: string[];
  patched: ShootingSavedCameraPatchEntry[];
  reordered?: string[];
}

export interface ShootingAssetPatchEntry {
  id: string;
  patch: ShootingAssetPatch;
}

export interface ShootingShotPatchEntry {
  id: string;
  patch: ShootingShotPatch;
}

export interface ShootingSavedCameraPatchEntry {
  id: string;
  patch: ShootingSavedCameraPatch;
}

export interface ShootingAssetPatch {
  name?: string;
  url?: string;
  origin?: [number, number, number];
  orientation?: [number, number, number, number];
  scale?: [number, number, number];
}

export interface ShootingShotPatch {
  label?: string;
  width?: number;
  height?: number;
  format?: string;
  shape?: string;
}

export interface ShootingSavedCameraPatch {
  label?: string;
  camera?: ShootingCamera;
}

export interface ShootingArtifact {
  schema: string;
  assets: ShootingAsset[];
  savedCameras: ShootingSavedCamera[];
  scene: ShootingSceneLighting;
  shots: ShootingShot[];
  activeShotId: string;
  activeAssetId: string;
  selectedShotIds: string[];
  activeUtilityId: string;
  defaultShotFormat: string;
  defaultShotShape: string;
  defaultAssetFormat: string;
  centerModel: boolean;
  fitRevision: number;
  cameraDraftLabel: string;
  camera: ShootingCamera;
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
