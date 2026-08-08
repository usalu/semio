/** 🧬️ CadArtifact schema. */

export interface CadArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  id: string;
  /** @state persistent */
  objects: CadObject[];
  /** @state persistent */
  buildingObjects: CadObject[];
  /** @state persistent */
  energyObjects: CadObject[];
  /** @state persistent */
  structureClassicObjects: CadObject[];
  /** @state persistent */
  referencesByModelDefinitionId: Record<string, CadReferenceList>;
  /** @state persistent */
  nodes: CadNode[];
  /** @state persistent */
  shapeGeometry?: CadGeometry;
  /** @state persistent */
  buildingGeometry?: CadGeometry;
  /** @state persistent */
  energyGeometry?: CadGeometry;
  /** @state persistent */
  structureClassicGeometry?: CadGeometry;
  /** @state persistent */
  activeModelDefinitionId: string;
  /** @state shared-ui */
  selectedObjectIds: string[];
  /** @state shared-ui */
  selectedNodeIds: string[];
  /** @state shared-ui */
  activeObjectId?: string;
  /** @state shared-ui */
  componentSelection: CadComponentSelection;
  /** @state shared-ui */
  selectedReferenceModelDefinitionId?: string;
  /** @state shared-ui */
  selectedReferenceId?: string;
  /** @state shared-ui */
  selectedPrimitiveId?: string;
  /** @state shared-ui */
  selectedPrimitiveKind?: string;
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state shared-ui */
  activeExampleId?: string;
  /** @state local-ui */
  selectionMethod: string;
  /** @state local-ui */
  engagementInput: string;
  /** @state local-ui */
  engagementStep: string;
  /** @state local-ui */
  engagementPane?: string;
  /** @state local-ui */
  engagementSessionJson?: string;
  /** @state local-ui */
  lastFinalizedInteractionId?: string;
  /** @state local-ui */
  sunEnabled: boolean;
  /** @state local-ui */
  sunAzimuth: number;
  /** @state local-ui */
  sunElevation: number;
  /** @state local-ui */
  sunIntensity: number;
  /** @state local-ui */
  sunColor: string;
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
  locale: string;
  /** @state local-ui */
  terminology: string;
  /** @state local-ui */
  contributionsJson: string;
  /** @state preview */
  hoveredObjectId?: string;
  /** @state preview */
  hoveredTargetObjectId?: string;
  /** @state preview */
  hoveredTargetMode?: string;
  /** @state preview */
  hoveredTargetId?: number;
}

export interface CadObject { id: string; [key: string]: unknown }
export interface CadNode { id: string; [key: string]: unknown }
export interface CadReferenceList { values: unknown[] }
export interface CadGeometry { [key: string]: unknown }
export interface CadCamera { [key: string]: unknown }
export interface CadComponentSelection { [key: string]: unknown }
export interface CadDislocateOptions { moveEnabled: boolean; rotateEnabled: boolean }

