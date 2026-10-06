/** 📸️ `En1991Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1991Snapshot {
  /** @state artifact */
  annex: "En" | "De";
  /** @state artifact */
  snowZone: string;
  /** @state artifact */
  altitude: number;
  /** @state artifact */
  enSk: number;
  /** @state artifact */
  northGermanLowlandSnow: boolean;
  /** @state artifact */
  windZone: number;
  /** @state artifact */
  enVb: number;
  /** @state artifact */
  terrainCategory: number;
  /** @state artifact */
  mixedTerrainUpwind: number;
  /** @state artifact */
  mixedTerrainDistance: number;
  /** @state artifact */
  orographyFactor: number;
  /** @state artifact */
  coastOrIsland: boolean;
  /** @state artifact */
  airDensity: number;
  /** @state artifact */
  height: number;
  /** @state artifact */
  width: number;
  /** @state artifact */
  depth: number;
  /** @state artifact */
  assumedDeltaT: number;
  /** @state artifact */
  constructionActivity: string;
  /** @state artifact */
  assumedConstructionQk: number;
  /** @state artifact */
  structureKind: "building" | "bridge";
  /** @state artifact */
  bridgeLane: number;
  /** @state artifact */
  bridgeSpan: number;
  /** @state artifact */
  bridgeLaneWidth: number;
  /** @state artifact */
  assumedBridgeTandem: number;
  /** @state artifact */
  assumedBridgeUdl: number;
  /** @state artifact */
  assumedBridgeLm2: number;
  /** @state artifact */
  assumedBridgeFootway: number;
  /** @state artifact */
  craneClaimed: boolean;
  /** @state artifact */
  craneClass: string;
  /** @state artifact */
  hoistClass: string;
  /** @state artifact */
  hoistingSpeed: number;
  /** @state artifact */
  assumedCraneWheel: number;
  /** @state artifact */
  assumedCraneHorizontal: number;
  /** @state artifact */
  siloClaimed: boolean;
  /** @state artifact */
  siloKind: string;
  /** @state artifact */
  siloBulkDensity: number;
  /** @state artifact */
  siloHeight: number;
  /** @state artifact */
  siloHydraulicRadius: number;
  /** @state artifact */
  siloMu: number;
  /** @state artifact */
  siloK: number;
  /** @state artifact */
  assumedSiloPressure: number;
  /** @state artifact */
  assumedSiloPatch: number;
  /** @state artifact */
  assumedSiloWallFriction: number;
  /** @state artifact */
  floors: FloorArea[];
  /** @state artifact */
  selfWeightElements: SelfWeightElement[];
  /** @state artifact */
  roofs: RoofArea[];
  /** @state artifact */
  windFaces: WindFace[];
  /** @state artifact */
  accidentalCases: AccidentalCase[];
  tMax: number;
  tMin: number;
  t0: number;
  thermalElementType: string;
  thermalBridgeType: number;
  deltaTM: number;
  storeyCount: number;
  /** @state artifact */
  fireMode: "none" | "nominal" | "parametric";
  fireCurve: "standard" | "external" | "hydrocarbon" | "parametric";
  fireDuration: number;
  assumedGasTemperature: number;
  assumedHNet: number;
  fireCompartmentArea: number;
  fireCompartmentHeight: number;
  fireOpeningFactor: number;
  fireThermalInertia: number;
  fireOccupancy: string;
  fireLoadDensityQf: number;
  assumedQfD: number;
  assumedBridgeLm3: number;
  assumedBridgeLm4: number;
  bridgeLoadGroup: string;
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

export type En1991StructureKind = "building" | "bridge";

export type En1991AnnexChoice = "En" | "De";

export type En1991FireCurve = "standard" | "external" | "hydrocarbon" | "parametric";

export type En1991FireMode = "none" | "nominal" | "parametric";

export const parseEn1991Snapshot: NormWireReader<En1991Snapshot> = normWireObject<En1991Snapshot>({ annex: normWireRequired(normWireLiteral("En", "De")), snowZone: normWireRequired(normWireString), altitude: normWireRequired(normWireNumber), enSk: normWireRequired(normWireNumber), northGermanLowlandSnow: normWireRequired(normWireBoolean), windZone: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), enVb: normWireRequired(normWireNumber), terrainCategory: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), mixedTerrainUpwind: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), mixedTerrainDistance: normWireRequired(normWireNumber), orographyFactor: normWireRequired(normWireNumber), coastOrIsland: normWireRequired(normWireBoolean), airDensity: normWireRequired(normWireNumber), height: normWireRequired(normWireNumber), width: normWireRequired(normWireNumber), depth: normWireRequired(normWireNumber), assumedDeltaT: normWireRequired(normWireNumber), constructionActivity: normWireRequired(normWireString), assumedConstructionQk: normWireRequired(normWireNumber), structureKind: normWireRequired(normWireLiteral("building", "bridge")), bridgeLane: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), bridgeSpan: normWireRequired(normWireNumber), bridgeLaneWidth: normWireRequired(normWireNumber), assumedBridgeTandem: normWireRequired(normWireNumber), assumedBridgeUdl: normWireRequired(normWireNumber), assumedBridgeLm2: normWireRequired(normWireNumber), assumedBridgeFootway: normWireRequired(normWireNumber), craneClaimed: normWireRequired(normWireBoolean), craneClass: normWireRequired(normWireString), hoistClass: normWireRequired(normWireString), hoistingSpeed: normWireRequired(normWireNumber), assumedCraneWheel: normWireRequired(normWireNumber), assumedCraneHorizontal: normWireRequired(normWireNumber), siloClaimed: normWireRequired(normWireBoolean), siloKind: normWireRequired(normWireString), siloBulkDensity: normWireRequired(normWireNumber), siloHeight: normWireRequired(normWireNumber), siloHydraulicRadius: normWireRequired(normWireNumber), siloMu: normWireRequired(normWireNumber), siloK: normWireRequired(normWireNumber), assumedSiloPressure: normWireRequired(normWireNumber), assumedSiloPatch: normWireRequired(normWireNumber), assumedSiloWallFriction: normWireRequired(normWireNumber), floors: normWireRequired(normWireArray(normWireRef(() => parseFloorArea))), selfWeightElements: normWireRequired(normWireArray(normWireRef(() => parseSelfWeightElement))), roofs: normWireRequired(normWireArray(normWireRef(() => parseRoofArea))), windFaces: normWireRequired(normWireArray(normWireRef(() => parseWindFace))), accidentalCases: normWireRequired(normWireArray(normWireRef(() => parseAccidentalCase))), tMax: normWireRequired(normWireNumber), tMin: normWireRequired(normWireNumber), t0: normWireRequired(normWireNumber), thermalElementType: normWireRequired(normWireString), thermalBridgeType: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), deltaTM: normWireRequired(normWireNumber), storeyCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), fireMode: normWireRequired(normWireLiteral("none", "nominal", "parametric")), fireCurve: normWireRequired(normWireLiteral("standard", "external", "hydrocarbon", "parametric")), fireDuration: normWireRequired(normWireNumber), assumedGasTemperature: normWireRequired(normWireNumber), assumedHNet: normWireRequired(normWireNumber), fireCompartmentArea: normWireRequired(normWireNumber), fireCompartmentHeight: normWireRequired(normWireNumber), fireOpeningFactor: normWireRequired(normWireNumber), fireThermalInertia: normWireRequired(normWireNumber), fireOccupancy: normWireRequired(normWireString), fireLoadDensityQf: normWireRequired(normWireNumber), assumedQfD: normWireRequired(normWireNumber), assumedBridgeLm3: normWireRequired(normWireNumber), assumedBridgeLm4: normWireRequired(normWireNumber), bridgeLoadGroup: normWireRequired(normWireString) });
export const parseFloorArea: NormWireReader<FloorArea> = normWireObject<FloorArea>({ id: normWireRequired(normWireString), category: normWireRequired(normWireString), area: normWireRequired(normWireNumber), assumedQk: normWireRequired(normWireNumber), assumedQkConcentrated: normWireRequired(normWireNumber), assumedPartitions: normWireRequired(normWireNumber) });
export const parseSelfWeightElement: NormWireReader<SelfWeightElement> = normWireObject<SelfWeightElement>({ id: normWireRequired(normWireString), material: normWireRequired(normWireString), thickness: normWireRequired(normWireNumber), assumedGk: normWireRequired(normWireNumber) });
export const parseRoofArea: NormWireReader<RoofArea> = normWireObject<RoofArea>({ id: normWireRequired(normWireString), roofType: normWireRequired(normWireString), pitchDeg: normWireRequired(normWireNumber), cE: normWireRequired(normWireNumber), cT: normWireRequired(normWireNumber), hasParapet: normWireRequired(normWireBoolean), parapetHeight: normWireRequired(normWireNumber), driftObstructionHeight: normWireRequired(normWireNumber), multiSpan: normWireRequired(normWireBoolean), assumedSk: normWireRequired(normWireNumber) });
export const parseWindFace: NormWireReader<WindFace> = normWireObject<WindFace>({ id: normWireRequired(normWireString), zone: normWireRequired(normWireString), z: normWireRequired(normWireNumber), cPe10: normWireRequired(normWireNumber), cPe1: normWireRequired(normWireNumber), cPi: normWireRequired(normWireNumber), cS: normWireRequired(normWireNumber), cD: normWireRequired(normWireNumber), loadedArea: normWireRequired(normWireNumber), assumedWp: normWireRequired(normWireNumber) });
export const parseAccidentalCase: NormWireReader<AccidentalCase> = normWireObject<AccidentalCase>({ id: normWireRequired(normWireString), impact: normWireRequired(normWireArray(normWireRef(() => parseAccidentalImpact))), explosion: normWireRequired(normWireArray(normWireRef(() => parseAccidentalExplosion))) });
export const parseAccidentalImpact: NormWireReader<AccidentalImpact> = normWireObject<AccidentalImpact>({ vehicleMass: normWireRequired(normWireNumber), vehicleSpeed: normWireRequired(normWireNumber), assumedForce: normWireRequired(normWireNumber) });
export const parseAccidentalExplosion: NormWireReader<AccidentalExplosion> = normWireObject<AccidentalExplosion>({ explosionMass: normWireRequired(normWireNumber), standoff: normWireRequired(normWireNumber), assumedPressure: normWireRequired(normWireNumber) });
export const parseEn1991StructureKind: NormWireReader<En1991StructureKind> = normWireLiteral("building", "bridge");
export const parseEn1991AnnexChoice: NormWireReader<En1991AnnexChoice> = normWireLiteral("En", "De");
export const parseEn1991FireCurve: NormWireReader<En1991FireCurve> = normWireLiteral("standard", "external", "hydrocarbon", "parametric");
export const parseEn1991FireMode: NormWireReader<En1991FireMode> = normWireLiteral("none", "nominal", "parametric");
