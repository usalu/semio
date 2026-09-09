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
}

export interface CadStringList { values: string[]; }

export interface CadObjectsDelta { added: CadObject[]; removed: string[]; patched: { id: string; patch: Record<string, unknown> }[]; reordered?: string[]; }

export interface CadNodesDelta { added: CadNode[]; removed: string[]; patched: { id: string; patch: Record<string, unknown> }[]; reordered?: string[]; }

export interface CadObject { id: string; [key: string]: unknown }
export interface CadNode { id: string; [key: string]: unknown }
export interface CadReferenceList { values: unknown[] }
export interface CadGeometry { [key: string]: unknown }
export interface CadCamera { [key: string]: unknown }

export interface CadArtifact { [key: string]: unknown }
