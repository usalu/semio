/** 🧬️ CadDiff schema. */

export interface CadDiff {
  /** @state persistent */
  artifact?: CadArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  id?: string;
  /** @state persistent */
  objects?: CadObjectsDelta;
  /** @state persistent */
  buildingObjects?: CadObjectsDelta;
  /** @state persistent */
  energyObjects?: CadObjectsDelta;
  /** @state persistent */
  structureClassicObjects?: CadObjectsDelta;
  /** @state persistent */
  referencesByModelDefinitionId?: Record<string, CadReferenceList>;
  /** @state persistent */
  nodes?: CadNodesDelta;
  /** @state persistent */
  shapeGeometry?: CadGeometry | null;
  /** @state persistent */
  buildingGeometry?: CadGeometry | null;
  /** @state persistent */
  energyGeometry?: CadGeometry | null;
  /** @state persistent */
  structureClassicGeometry?: CadGeometry | null;
  /** @state persistent */
  activeModelDefinitionId?: string;
  /** @state shared-ui */
  selectedObjectIds?: CadStringList;
  /** @state shared-ui */
  selectedNodeIds?: CadStringList;
  /** @state shared-ui */
  activeObjectId?: string | null;
  /** @state shared-ui */
  componentSelection?: CadComponentSelection;
  /** @state shared-ui */
  selectedReferenceModelDefinitionId?: string | null;
  /** @state shared-ui */
  selectedReferenceId?: string | null;
  /** @state shared-ui */
  selectedPrimitiveId?: string | null;
  /** @state shared-ui */
  selectedPrimitiveKind?: string | null;
  /** @state shared-ui */
  activeUtilityId?: string;
  /** @state shared-ui */
  activeExampleId?: string | null;
  /** @state local-ui */
  selectionMethod?: string;
  /** @state local-ui */
  engagementInput?: string;
  /** @state local-ui */
  engagementStep?: string;
  /** @state local-ui */
  engagementPane?: string | null;
  /** @state local-ui */
  engagementSessionJson?: string | null;
  /** @state local-ui */
  lastFinalizedInteractionId?: string | null;
  /** @state local-ui */
  sunEnabled?: boolean;
  /** @state local-ui */
  sunAzimuth?: number;
  /** @state local-ui */
  sunElevation?: number;
  /** @state local-ui */
  sunIntensity?: number;
  /** @state local-ui */
  sunColor?: string;
  /** @state local-ui */
  camera?: CadCamera;
  /** @state local-ui */
  cameraBuilding?: CadCamera;
  /** @state local-ui */
  cameraEnergy?: CadCamera;
  /** @state local-ui */
  cameraStructureClassic?: CadCamera;
  /** @state local-ui */
  dislocateShape?: CadDislocateOptions;
  /** @state local-ui */
  dislocateBuilding?: CadDislocateOptions;
  /** @state local-ui */
  dislocateEnergy?: CadDislocateOptions;
  /** @state local-ui */
  dislocateStructureClassic?: CadDislocateOptions;
  /** @state local-ui */
  locale?: string;
  /** @state local-ui */
  terminology?: string;
  /** @state local-ui */
  contributionsJson?: string;
  /** @state preview */
  hoveredObjectId?: string | null;
  /** @state preview */
  hoveredTargetObjectId?: string | null;
  /** @state preview */
  hoveredTargetMode?: string | null;
  /** @state preview */
  hoveredTargetId?: number | null;
}

export interface CadStringList { values: string[]; }

export interface CadObjectsDelta { added: CadObject[]; removed: string[]; patched: { id: string; patch: Record<string, unknown> }[]; reordered?: string[]; }

export interface CadNodesDelta { added: CadNode[]; removed: string[]; patched: { id: string; patch: Record<string, unknown> }[]; reordered?: string[]; }

export interface CadObject { id: string; [key: string]: unknown }
export interface CadNode { id: string; [key: string]: unknown }
export interface CadReferenceList { values: unknown[] }
export interface CadGeometry { [key: string]: unknown }
export interface CadCamera { [key: string]: unknown }
export interface CadComponentSelection { [key: string]: unknown }
export interface CadDislocateOptions { moveEnabled: boolean; rotateEnabled: boolean }

export interface CadArtifact { [key: string]: unknown }
