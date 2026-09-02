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
  /** @state config */
  selectedObjectIds: string[];
  /** @state config */
  selectedNodeIds: string[];
  /** @state config */
  selectionMethod: string;
  /** @state config */
  hoveredObjectId?: string;
  /** @state config */
  hoveredTarget?: CadHoverTarget;
  /** @state config */
  activeObjectId?: string;
  /** @state config */
  componentSelection: CadComponentSelection;
  /** @state config */
  engagementInput: string;
  /** @state config */
  engagementStep: string;
  /** @state config */
  activeExampleId?: string;
  /** @state config */
  selectedReferenceModelDefinitionId?: string;
  /** @state config */
  selectedReferenceId?: string;
  /** @state config */
  selectedPrimitiveId?: string;
  /** @state config */
  selectedPrimitiveKind?: string;
  /** @state config */
  engagementPane?: string;
  /** @state config */
  engagementSessionJson?: string;
  /** @state config */
  engagementPreviewOperationJson?: string;
  /** @state config @minimum 0 @maximum 2147483647 Exact signed-32 generation. */
  engagementPreviewGeneration: number;
  /** @state config */
  lastFinalizedInteractionId?: string;
  /** @state config */
  sun: CadSunConfig;
  /** @state config */
  camera: CadCamera;
  /** @state config */
  cameraBuilding: CadCamera;
  /** @state config */
  cameraEnergy: CadCamera;
  /** @state config */
  cameraStructureClassic: CadCamera;
  /** @state config */
  dislocateShape: CadDislocateOptions;
  /** @state config */
  dislocateBuilding: CadDislocateOptions;
  /** @state config */
  dislocateEnergy: CadDislocateOptions;
  /** @state config */
  dislocateStructureClassic: CadDislocateOptions;
  /** @state config */
  activeUtilityId: string;
  /** @state config */
  locale: string;
  /** @state config */
  terminology: string;
  /** @state config */
  contributionsJson: string;
}
