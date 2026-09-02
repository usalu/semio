/** 🧬️ CadDiff schema. */

export interface CadDiff {
  /** @state artifact */
  artifact?: CadArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  id?: string;
  /** @state artifact */
  objects?: CadObjectsDelta;
  /** @state artifact */
  buildingObjects?: CadObjectsDelta;
  /** @state artifact */
  energyObjects?: CadObjectsDelta;
  /** @state artifact */
  structureClassicObjects?: CadObjectsDelta;
  /** @state artifact */
  referencesByModelDefinitionId?: Record<string, CadReferenceList>;
  /** @state artifact */
  nodes?: CadNodesDelta;
  /** @state artifact */
  shapeGeometry?: CadGeometry | null;
  /** @state artifact */
  buildingGeometry?: CadGeometry | null;
  /** @state artifact */
  energyGeometry?: CadGeometry | null;
  /** @state artifact */
  structureClassicGeometry?: CadGeometry | null;
  /** @state artifact */
  activeModelDefinitionId?: string;
  /** @state presence */
  selectedObjectIds?: CadStringList;
  /** @state presence */
  selectedNodeIds?: CadStringList;
  /** @state presence */
  activeObjectId?: string | null;
  /** @state presence */
  componentSelection?: CadComponentSelection;
  /** @state presence */
  selectedReferenceModelDefinitionId?: string | null;
  /** @state presence */
  selectedReferenceId?: string | null;
  /** @state presence */
  selectedPrimitiveId?: string | null;
  /** @state presence */
  selectedPrimitiveKind?: string | null;
  /** @state presence */
  activeUtilityId?: string;
  /** @state presence */
  activeExampleId?: string | null;
  /** @state config */
  selectionMethod?: string;
  /** @state config */
  engagementInput?: string;
  /** @state config */
  engagementStep?: string;
  /** @state config */
  engagementPane?: string | null;
  /** @state config */
  engagementSessionJson?: string | null;
  /** @state config */
  lastFinalizedInteractionId?: string | null;
  /** @state config */
  sunEnabled?: boolean;
  /** @state config */
  sunAzimuth?: number;
  /** @state config */
  sunElevation?: number;
  /** @state config */
  sunIntensity?: number;
  /** @state config */
  sunColor?: string;
  /** @state config */
  camera?: CadCamera;
  /** @state config */
  cameraBuilding?: CadCamera;
  /** @state config */
  cameraEnergy?: CadCamera;
  /** @state config */
  cameraStructureClassic?: CadCamera;
  /** @state config */
  dislocateShape?: CadDislocateOptions;
  /** @state config */
  dislocateBuilding?: CadDislocateOptions;
  /** @state config */
  dislocateEnergy?: CadDislocateOptions;
  /** @state config */
  dislocateStructureClassic?: CadDislocateOptions;
  /** @state config */
  locale?: string;
  /** @state config */
  terminology?: string;
  /** @state config */
  contributionsJson?: string;
  /** @state artifact */
  hoveredObjectId?: string | null;
  /** @state artifact */
  hoveredTargetObjectId?: string | null;
  /** @state artifact */
  hoveredTargetMode?: string | null;
  /** @state artifact */
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
