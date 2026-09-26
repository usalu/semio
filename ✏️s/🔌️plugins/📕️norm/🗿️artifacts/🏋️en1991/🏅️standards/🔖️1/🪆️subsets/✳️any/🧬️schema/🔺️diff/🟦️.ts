/** 🧬️ En1991Diff schema — artifact-lane fields only. */

export interface En1991Diff {
  /** @state artifact */
  annex?: string;
  /** @state artifact */
  snowZone?: string;
  /** @state artifact */
  altitude?: number;
  /** @state artifact */
  enSk?: number;
  /** @state artifact */
  exceptionalSnowNorthGermanLowlands?: boolean;
  /** @state artifact */
  windZone?: number;
  /** @state artifact */
  enVb?: number;
  /** @state artifact */
  terrainCategory?: number;
  /** @state artifact */
  mixedTerrainUpwind?: number;
  /** @state artifact */
  mixedTerrainDistance?: number;
  /** @state artifact */
  orographyFactor?: number;
  /** @state artifact */
  coastOrIsland?: boolean;
  /** @state artifact */
  airDensity?: number;
  /** @state artifact */
  height?: number;
  /** @state artifact */
  width?: number;
  /** @state artifact */
  depth?: number;
  /** @state artifact */
  assumedDeltaT?: number;
  /** @state artifact */
  requiredDeltaT?: number;
  /** @state artifact */
  constructionActivity?: string;
  /** @state artifact */
  assumedConstructionQk?: number;
  /** @state artifact */
  structureKind?: boolean;
  /** @state artifact */
  bridgeLane?: number;
  /** @state artifact */
  bridgeSpan?: number;
  /** @state artifact */
  bridgeLaneWidth?: number;
  /** @state artifact */
  assumedBridgeTandem?: number;
  /** @state artifact */
  assumedBridgeUdl?: number;
  /** @state artifact */
  assumedBridgeLm2?: number;
  /** @state artifact */
  assumedBridgeFootway?: number;
  /** @state artifact */
  craneClaimed?: boolean;
  /** @state artifact */
  craneClass?: string;
  /** @state artifact */
  hoistClass?: string;
  /** @state artifact */
  hoistingSpeed?: number;
  /** @state artifact */
  assumedCraneWheel?: number;
  /** @state artifact */
  assumedCraneHorizontal?: number;
  /** @state artifact */
  siloClaimed?: boolean;
  /** @state artifact */
  siloKind?: string;
  /** @state artifact */
  siloBulkDensity?: number;
  /** @state artifact */
  siloHeight?: number;
  /** @state artifact */
  siloHydraulicRadius?: number;
  /** @state artifact */
  siloMu?: number;
  /** @state artifact */
  siloK?: number;
  /** @state artifact */
  assumedSiloPressure?: number;
  /** @state artifact */
  assumedSiloPatch?: number;
  /** @state artifact */
  assumedSiloWallFriction?: number;
  /** @state artifact */
  floors?: FloorArea[];
  /** @state artifact */
  selfWeightElements?: SelfWeightElement[];
  /** @state artifact */
  roofs?: RoofArea[];
  /** @state artifact */
  windFaces?: WindFace[];
  /** @state artifact */
  accidentalCases?: AccidentalCase[];
}

export interface FloorArea { id: string; category: string; area: number; assumedQk: number; assumedQkConcentrated: number; assumedPartitions: number; }
export interface SelfWeightElement { id: string; material: string; thickness: number; assumedGk: number; }
export interface RoofArea { id: string; roofType: string; pitchDeg: number; cE: number; cT: number; hasParapet: boolean; parapetHeight: number; driftObstructionHeight: number; multiSpan: boolean; assumedSk: number; }
export interface WindFace { id: string; zone: string; z: number; cPe10: number; cPe1: number; cPi: number; cS: number; cD: number; assumedWp: number; }
export interface AccidentalCase { id: string; kind: string; vehicleMass: number; vehicleSpeed: number; explosionMass: number; standoff: number; assumedForce: number; assumedPressure: number; }
