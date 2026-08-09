/** 🧬️ CadConfig */
export interface CadHoverTarget {
  objectId?: string;
  mode?: string;
  id?: number;
}
export interface CadSelectionTargets {
  mesh: boolean;
  vertex: boolean;
  edge: boolean;
  face: boolean;
}
export interface CadComponentSelection {
  targets: CadSelectionTargets;
  mode: string;
  ids: number[];
}
export interface CadSunConfig {
  enabled: boolean;
  azimuth: number;
  elevation: number;
  intensity: number;
  color: string;
}
export interface CadProjectionDsl {
  kind: string;
  [key: string]: unknown;
}
export interface CadCamera {
  position: number[];
  target: number[];
  zoom: number;
  fov: number;
  projection: CadProjectionDsl;
}
export interface CadDislocateOptions {
  moveEnabled: boolean;
  rotateEnabled: boolean;
}


export interface CadConfig {
  /** @state local-ui */
  selectedObjectIds: string[];
  /** @state local-ui */
  selectedNodeIds: string[];
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  hoveredObjectId?: string;
  /** @state local-ui */
  hoveredTarget?: CadHoverTarget;
  /** @state local-ui */
  activeObjectId?: string;
  /** @state local-ui */
  componentSelection: CadComponentSelection;
  /** @state local-ui */
  engagementInput: string;
  /** @state local-ui */
  engagementStep: string;
  /** @state local-ui */
  activeExampleId?: string;
  /** @state local-ui */
  selectedReferenceModelDefinitionId?: string;
  /** @state local-ui */
  selectedReferenceId?: string;
  /** @state local-ui */
  selectedPrimitiveId?: string;
  /** @state local-ui */
  selectedPrimitiveKind?: string;
  /** @state local-ui */
  engagementPane?: string;
  /** @state local-ui */
  engagementSessionJson?: string;
  /** @state local-ui */
  lastFinalizedInteractionId?: string;
  /** @state local-ui */
  sun: CadSunConfig;
  /** @state local-ui */
  camera: CadCamera;
  /** @state local-ui */
  cameraBuilding: CadCamera;
  /** @state local-ui */
  cameraEnergy: CadCamera;
  /** @state local-ui */
  cameraStructureClassic: CadCamera;
  /** @state local-ui */
  dislocateShape: CadDislocateOptions;
  /** @state local-ui */
  dislocateBuilding: CadDislocateOptions;
  /** @state local-ui */
  dislocateEnergy: CadDislocateOptions;
  /** @state local-ui */
  dislocateStructureClassic: CadDislocateOptions;
  /** @state local-ui */
  activeUtilityId: string;
  /** @state local-ui */
  locale: string;
  /** @state local-ui */
  terminology: string;
  /** @state local-ui */
  contributionsJson: string;
}
