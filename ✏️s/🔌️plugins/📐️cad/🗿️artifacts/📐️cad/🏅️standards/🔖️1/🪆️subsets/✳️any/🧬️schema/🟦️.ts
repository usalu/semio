/** 🧬️ CadArtifact schema. */

export interface CadArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  objects: CadObject[];
  /** @state artifact */
  buildingObjects: CadObject[];
  /** @state artifact */
  energyObjects: CadObject[];
  /** @state artifact */
  structureClassicObjects: CadObject[];
  /** @state artifact */
  referencesByModelDefinitionId: Record<string, CadReferenceList>;
  /** @state artifact */
  nodes: CadNode[];
  /** @state artifact */
  shapeGeometry?: CadGeometry;
  /** @state artifact */
  buildingGeometry?: CadGeometry;
  /** @state artifact */
  energyGeometry?: CadGeometry;
  /** @state artifact */
  structureClassicGeometry?: CadGeometry;
  /** @state artifact */
  activeModelDefinitionId: string;
  /** @state presence */
  selectedObjectIds: string[];
  /** @state presence */
  selectedNodeIds: string[];
  /** @state presence */
  activeObjectId?: string;
  /** @state presence */
  componentSelection: CadComponentSelection;
  /** @state presence */
  selectedReferenceModelDefinitionId?: string;
  /** @state presence */
  selectedReferenceId?: string;
  /** @state presence */
  selectedPrimitiveId?: string;
  /** @state presence */
  selectedPrimitiveKind?: string;
  /** @state presence */
  activeUtilityId: string;
  /** @state presence */
  activeExampleId?: string;
  /** @state config */
  selectionMethod: string;
  /** @state config */
  engagementInput: string;
  /** @state config */
  engagementStep: string;
  /** @state config */
  engagementPane?: string;
  /** @state config */
  engagementSessionJson?: string;
  /** @state config */
  lastFinalizedInteractionId?: string;
  /** @state config */
  sunEnabled: boolean;
  /** @state config */
  sunAzimuth: number;
  /** @state config */
  sunElevation: number;
  /** @state config */
  sunIntensity: number;
  /** @state config */
  sunColor: string;
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
  locale: string;
  /** @state config */
  terminology: string;
  /** @state config */
  contributionsJson: string;
  /** @state artifact */
  hoveredObjectId?: string;
  /** @state artifact */
  hoveredTargetObjectId?: string;
  /** @state artifact */
  hoveredTargetMode?: string;
  /** @state artifact */
  hoveredTargetId?: number;
}

export interface CadObject { id: string; [key: string]: unknown }
export interface CadNode { id: string; [key: string]: unknown }
export interface CadReferenceList { values: unknown[] }
export interface CadGeometry { [key: string]: unknown }
export interface CadCamera { [key: string]: unknown }
export interface CadComponentSelection { [key: string]: unknown }
export interface CadDislocateOptions { moveEnabled: boolean; rotateEnabled: boolean }

