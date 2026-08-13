/** 🧬️ CadSnapshot schema. */

export interface CadSnapshot {
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
}

export interface CadObject { id: string; [key: string]: unknown }
export interface CadNode { id: string; [key: string]: unknown }
export interface CadReferenceList { values: unknown[] }
export interface CadGeometry { [key: string]: unknown }
export interface CadCamera { [key: string]: unknown }
export interface CadComponentSelection { [key: string]: unknown }
export interface CadDislocateOptions { moveEnabled: boolean; rotateEnabled: boolean }

