/** 🧬️ CadSnapshot schema. */

export interface CadSnapshot {
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
}

export interface CadObject { id: string; [key: string]: unknown }
export interface CadNode { id: string; [key: string]: unknown }
export interface CadReferenceList { values: unknown[] }
export interface CadGeometry { [key: string]: unknown }
export interface CadCamera { [key: string]: unknown }
export interface CadComponentSelection { [key: string]: unknown }
export interface CadDislocateOptions { moveEnabled: boolean; rotateEnabled: boolean }

