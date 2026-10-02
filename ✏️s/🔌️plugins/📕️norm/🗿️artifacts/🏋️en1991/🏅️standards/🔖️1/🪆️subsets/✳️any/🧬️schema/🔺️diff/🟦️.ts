/** 🔺️ `En1991Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { type NormJson, normWireArray, normWireBoolean, normWireInteger, normWireJson, normWireLiteral, normWireMap, normWireNumber, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1991Diff {
  /** @state artifact */
  artifact?: { [key: string]: NormJson };
  /** @state artifact */
  annex?: string;
  /** @state artifact */
  snowZone?: string;
  /** @state artifact */
  altitude?: number;
  /** @state artifact */
  enSk?: number;
  /** @state artifact */
  northGermanLowlandSnow?: boolean;
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
  tMax?: number;
  tMin?: number;
  t0?: number;
  thermalElementType?: string;
  thermalBridgeType?: number;
  deltaTM?: number;
  storeyCount?: number;
  /** @state artifact */
  fireMode?: "none" | "nominal" | "parametric";
  fireCurve?: string;
  fireDuration?: number;
  assumedGasTemperature?: number;
  assumedHNet?: number;
  fireCompartmentArea?: number;
  fireCompartmentHeight?: number;
  fireOpeningFactor?: number;
  fireThermalInertia?: number;
  fireOccupancy?: string;
  fireLoadDensityQf?: number;
  assumedQfD?: number;
  /** @state artifact */
  constructionActivity?: string;
  /** @state artifact */
  assumedConstructionQk?: number;
  /** @state artifact */
  structureKind?: "building" | "bridge";
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
  assumedBridgeLm3?: number;
  assumedBridgeLm4?: number;
  bridgeLoadGroup?: string;
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

export interface FloorArea {
  id: string;
  category: string;
  area: number;
  assumedQk: number;
  assumedQkConcentrated: number;
  assumedPartitions: number;
}

export interface SelfWeightElement {
  id: string;
  material: string;
  thickness: number;
  assumedGk: number;
}

export interface RoofArea {
  id: string;
  roofType: string;
  pitchDeg: number;
  cE: number;
  cT: number;
  hasParapet: boolean;
  parapetHeight: number;
  driftObstructionHeight: number;
  multiSpan: boolean;
  assumedSk: number;
}

export interface WindFace {
  id: string;
  zone: string;
  z: number;
  cPe10: number;
  cPe1: number;
  cPi: number;
  cS: number;
  cD: number;
  loadedArea: number;
  assumedWp: number;
}

export interface AccidentalCase {
  id: string;
  impact: AccidentalImpact[];
  explosion: AccidentalExplosion[];
}

export interface AccidentalImpact {
  vehicleMass: number;
  vehicleSpeed: number;
  assumedForce: number;
}

export interface AccidentalExplosion {
  explosionMass: number;
  standoff: number;
  assumedPressure: number;
}

export const parseEn1991Diff: NormWireReader<En1991Diff> = normWireObject<En1991Diff>({ artifact: normWireOptional(normWireMap(normWireJson)), annex: normWireOptional(normWireString), snowZone: normWireOptional(normWireString), altitude: normWireOptional(normWireNumber), enSk: normWireOptional(normWireNumber), northGermanLowlandSnow: normWireOptional(normWireBoolean), windZone: normWireOptional(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), enVb: normWireOptional(normWireNumber), terrainCategory: normWireOptional(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), mixedTerrainUpwind: normWireOptional(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), mixedTerrainDistance: normWireOptional(normWireNumber), orographyFactor: normWireOptional(normWireNumber), coastOrIsland: normWireOptional(normWireBoolean), airDensity: normWireOptional(normWireNumber), height: normWireOptional(normWireNumber), width: normWireOptional(normWireNumber), depth: normWireOptional(normWireNumber), assumedDeltaT: normWireOptional(normWireNumber), tMax: normWireOptional(normWireNumber), tMin: normWireOptional(normWireNumber), t0: normWireOptional(normWireNumber), thermalElementType: normWireOptional(normWireString), thermalBridgeType: normWireOptional(normWireInteger), deltaTM: normWireOptional(normWireNumber), storeyCount: normWireOptional(normWireInteger), fireMode: normWireOptional(normWireLiteral("none", "nominal", "parametric")), fireCurve: normWireOptional(normWireString), fireDuration: normWireOptional(normWireNumber), assumedGasTemperature: normWireOptional(normWireNumber), assumedHNet: normWireOptional(normWireNumber), fireCompartmentArea: normWireOptional(normWireNumber), fireCompartmentHeight: normWireOptional(normWireNumber), fireOpeningFactor: normWireOptional(normWireNumber), fireThermalInertia: normWireOptional(normWireNumber), fireOccupancy: normWireOptional(normWireString), fireLoadDensityQf: normWireOptional(normWireNumber), assumedQfD: normWireOptional(normWireNumber), constructionActivity: normWireOptional(normWireString), assumedConstructionQk: normWireOptional(normWireNumber), structureKind: normWireOptional(normWireLiteral("building", "bridge")), bridgeLane: normWireOptional(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), bridgeSpan: normWireOptional(normWireNumber), bridgeLaneWidth: normWireOptional(normWireNumber), assumedBridgeTandem: normWireOptional(normWireNumber), assumedBridgeUdl: normWireOptional(normWireNumber), assumedBridgeLm2: normWireOptional(normWireNumber), assumedBridgeFootway: normWireOptional(normWireNumber), assumedBridgeLm3: normWireOptional(normWireNumber), assumedBridgeLm4: normWireOptional(normWireNumber), bridgeLoadGroup: normWireOptional(normWireString), craneClaimed: normWireOptional(normWireBoolean), craneClass: normWireOptional(normWireString), hoistClass: normWireOptional(normWireString), hoistingSpeed: normWireOptional(normWireNumber), assumedCraneWheel: normWireOptional(normWireNumber), assumedCraneHorizontal: normWireOptional(normWireNumber), siloClaimed: normWireOptional(normWireBoolean), siloKind: normWireOptional(normWireString), siloBulkDensity: normWireOptional(normWireNumber), siloHeight: normWireOptional(normWireNumber), siloHydraulicRadius: normWireOptional(normWireNumber), siloMu: normWireOptional(normWireNumber), siloK: normWireOptional(normWireNumber), assumedSiloPressure: normWireOptional(normWireNumber), assumedSiloPatch: normWireOptional(normWireNumber), assumedSiloWallFriction: normWireOptional(normWireNumber), floors: normWireOptional(normWireArray(normWireRef(() => parseFloorArea))), selfWeightElements: normWireOptional(normWireArray(normWireRef(() => parseSelfWeightElement))), roofs: normWireOptional(normWireArray(normWireRef(() => parseRoofArea))), windFaces: normWireOptional(normWireArray(normWireRef(() => parseWindFace))), accidentalCases: normWireOptional(normWireArray(normWireRef(() => parseAccidentalCase))) });
export const parseFloorArea: NormWireReader<FloorArea> = normWireObject<FloorArea>({ id: normWireRequired(normWireString), category: normWireRequired(normWireString), area: normWireRequired(normWireNumber), assumedQk: normWireRequired(normWireNumber), assumedQkConcentrated: normWireRequired(normWireNumber), assumedPartitions: normWireRequired(normWireNumber) });
export const parseSelfWeightElement: NormWireReader<SelfWeightElement> = normWireObject<SelfWeightElement>({ id: normWireRequired(normWireString), material: normWireRequired(normWireString), thickness: normWireRequired(normWireNumber), assumedGk: normWireRequired(normWireNumber) });
export const parseRoofArea: NormWireReader<RoofArea> = normWireObject<RoofArea>({ id: normWireRequired(normWireString), roofType: normWireRequired(normWireString), pitchDeg: normWireRequired(normWireNumber), cE: normWireRequired(normWireNumber), cT: normWireRequired(normWireNumber), hasParapet: normWireRequired(normWireBoolean), parapetHeight: normWireRequired(normWireNumber), driftObstructionHeight: normWireRequired(normWireNumber), multiSpan: normWireRequired(normWireBoolean), assumedSk: normWireRequired(normWireNumber) });
export const parseWindFace: NormWireReader<WindFace> = normWireObject<WindFace>({ id: normWireRequired(normWireString), zone: normWireRequired(normWireString), z: normWireRequired(normWireNumber), cPe10: normWireRequired(normWireNumber), cPe1: normWireRequired(normWireNumber), cPi: normWireRequired(normWireNumber), cS: normWireRequired(normWireNumber), cD: normWireRequired(normWireNumber), loadedArea: normWireRequired(normWireNumber), assumedWp: normWireRequired(normWireNumber) });
export const parseAccidentalCase: NormWireReader<AccidentalCase> = normWireObject<AccidentalCase>({ id: normWireRequired(normWireString), impact: normWireRequired(normWireArray(normWireRef(() => parseAccidentalImpact))), explosion: normWireRequired(normWireArray(normWireRef(() => parseAccidentalExplosion))) });
export const parseAccidentalImpact: NormWireReader<AccidentalImpact> = normWireObject<AccidentalImpact>({ vehicleMass: normWireRequired(normWireNumber), vehicleSpeed: normWireRequired(normWireNumber), assumedForce: normWireRequired(normWireNumber) });
export const parseAccidentalExplosion: NormWireReader<AccidentalExplosion> = normWireObject<AccidentalExplosion>({ explosionMass: normWireRequired(normWireNumber), standoff: normWireRequired(normWireNumber), assumedPressure: normWireRequired(normWireNumber) });
