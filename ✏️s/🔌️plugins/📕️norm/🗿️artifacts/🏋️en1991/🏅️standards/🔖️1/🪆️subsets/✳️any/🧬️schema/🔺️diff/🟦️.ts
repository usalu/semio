/** 🔺️ `En1991Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireDefault, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type AccidentalCase, type FloorArea, parseAccidentalCase, parseFloorArea, parseRoofArea, parseSelfWeightElement, parseWindFace, type RoofArea, type SelfWeightElement, type WindFace } from "../📸️snapshot/🟦️.ts";
import { type En1991Artifact, parseEn1991Artifact } from "../🟦️.ts";

export interface En1991Diff {
  /** @state artifact */
  artifact: En1991Artifact | null;
  /** @state artifact */
  annex: ("En" | "De") | null;
  /** @state artifact */
  snowZone: string | null;
  /** @state artifact */
  altitude: number | null;
  /** @state artifact */
  enSk: number | null;
  /** @state artifact */
  northGermanLowlandSnow: boolean | null;
  /** @state artifact */
  windZone: number | null;
  /** @state artifact */
  enVb: number | null;
  /** @state artifact */
  terrainCategory: number | null;
  /** @state artifact */
  mixedTerrainUpwind: number | null;
  /** @state artifact */
  mixedTerrainDistance: number | null;
  /** @state artifact */
  orographyFactor: number | null;
  /** @state artifact */
  coastOrIsland: boolean | null;
  /** @state artifact */
  airDensity: number | null;
  /** @state artifact */
  height: number | null;
  /** @state artifact */
  width: number | null;
  /** @state artifact */
  depth: number | null;
  /** @state artifact */
  assumedDeltaT: number | null;
  /** @state artifact */
  tMax: number | null;
  /** @state artifact */
  tMin: number | null;
  /** @state artifact */
  t0: number | null;
  /** @state artifact */
  thermalElementType: string | null;
  /** @state artifact */
  thermalBridgeType: number | null;
  /** @state artifact */
  deltaTM: number | null;
  /** @state artifact */
  storeyCount: number | null;
  /** @state artifact */
  fireMode: ("none" | "nominal" | "parametric") | null;
  /** @state artifact */
  fireCurve: ("standard" | "external" | "hydrocarbon" | "parametric") | null;
  /** @state artifact */
  fireDuration: number | null;
  /** @state artifact */
  assumedGasTemperature: number | null;
  /** @state artifact */
  assumedHNet: number | null;
  /** @state artifact */
  fireCompartmentArea: number | null;
  /** @state artifact */
  fireCompartmentHeight: number | null;
  /** @state artifact */
  fireOpeningFactor: number | null;
  /** @state artifact */
  fireThermalInertia: number | null;
  /** @state artifact */
  fireOccupancy: string | null;
  /** @state artifact */
  fireLoadDensityQf: number | null;
  /** @state artifact */
  assumedQfD: number | null;
  /** @state artifact */
  constructionActivity: string | null;
  /** @state artifact */
  assumedConstructionQk: number | null;
  /** @state artifact */
  structureKind: ("building" | "bridge") | null;
  /** @state artifact */
  bridgeLane: number | null;
  /** @state artifact */
  bridgeSpan: number | null;
  /** @state artifact */
  bridgeLaneWidth: number | null;
  /** @state artifact */
  assumedBridgeTandem: number | null;
  /** @state artifact */
  assumedBridgeUdl: number | null;
  /** @state artifact */
  assumedBridgeLm2: number | null;
  /** @state artifact */
  assumedBridgeFootway: number | null;
  /** @state artifact */
  assumedBridgeLm3: number | null;
  /** @state artifact */
  assumedBridgeLm4: number | null;
  /** @state artifact */
  bridgeLoadGroup: string | null;
  /** @state artifact */
  craneClaimed: boolean | null;
  /** @state artifact */
  craneClass: string | null;
  /** @state artifact */
  hoistClass: string | null;
  /** @state artifact */
  hoistingSpeed: number | null;
  /** @state artifact */
  assumedCraneWheel: number | null;
  /** @state artifact */
  assumedCraneHorizontal: number | null;
  /** @state artifact */
  siloClaimed: boolean | null;
  /** @state artifact */
  siloKind: string | null;
  /** @state artifact */
  siloBulkDensity: number | null;
  /** @state artifact */
  siloHeight: number | null;
  /** @state artifact */
  siloHydraulicRadius: number | null;
  /** @state artifact */
  siloMu: number | null;
  /** @state artifact */
  siloK: number | null;
  /** @state artifact */
  assumedSiloPressure: number | null;
  /** @state artifact */
  assumedSiloPatch: number | null;
  /** @state artifact */
  assumedSiloWallFriction: number | null;
  /** @state artifact */
  floors: { values: FloorArea[]; } | null;
  /** @state artifact */
  selfWeightElements: { values: SelfWeightElement[]; } | null;
  /** @state artifact */
  roofs: { values: RoofArea[]; } | null;
  /** @state artifact */
  windFaces: { values: WindFace[]; } | null;
  /** @state artifact */
  accidentalCases: { values: AccidentalCase[]; } | null;
}

export const parseEn1991Diff: NormWireReader<En1991Diff> = normWireObject<En1991Diff>({ artifact: normWireDefault(normWireNullable(parseEn1991Artifact), () => null), annex: normWireDefault(normWireNullable(normWireLiteral("En", "De")), () => null), snowZone: normWireDefault(normWireNullable(normWireString), () => null), altitude: normWireDefault(normWireNullable(normWireNumber), () => null), enSk: normWireDefault(normWireNullable(normWireNumber), () => null), northGermanLowlandSnow: normWireDefault(normWireNullable(normWireBoolean), () => null), windZone: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), enVb: normWireDefault(normWireNullable(normWireNumber), () => null), terrainCategory: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), mixedTerrainUpwind: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), mixedTerrainDistance: normWireDefault(normWireNullable(normWireNumber), () => null), orographyFactor: normWireDefault(normWireNullable(normWireNumber), () => null), coastOrIsland: normWireDefault(normWireNullable(normWireBoolean), () => null), airDensity: normWireDefault(normWireNullable(normWireNumber), () => null), height: normWireDefault(normWireNullable(normWireNumber), () => null), width: normWireDefault(normWireNullable(normWireNumber), () => null), depth: normWireDefault(normWireNullable(normWireNumber), () => null), assumedDeltaT: normWireDefault(normWireNullable(normWireNumber), () => null), tMax: normWireDefault(normWireNullable(normWireNumber), () => null), tMin: normWireDefault(normWireNullable(normWireNumber), () => null), t0: normWireDefault(normWireNullable(normWireNumber), () => null), thermalElementType: normWireDefault(normWireNullable(normWireString), () => null), thermalBridgeType: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), deltaTM: normWireDefault(normWireNullable(normWireNumber), () => null), storeyCount: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), fireMode: normWireDefault(normWireNullable(normWireLiteral("none", "nominal", "parametric")), () => null), fireCurve: normWireDefault(normWireNullable(normWireLiteral("standard", "external", "hydrocarbon", "parametric")), () => null), fireDuration: normWireDefault(normWireNullable(normWireNumber), () => null), assumedGasTemperature: normWireDefault(normWireNullable(normWireNumber), () => null), assumedHNet: normWireDefault(normWireNullable(normWireNumber), () => null), fireCompartmentArea: normWireDefault(normWireNullable(normWireNumber), () => null), fireCompartmentHeight: normWireDefault(normWireNullable(normWireNumber), () => null), fireOpeningFactor: normWireDefault(normWireNullable(normWireNumber), () => null), fireThermalInertia: normWireDefault(normWireNullable(normWireNumber), () => null), fireOccupancy: normWireDefault(normWireNullable(normWireString), () => null), fireLoadDensityQf: normWireDefault(normWireNullable(normWireNumber), () => null), assumedQfD: normWireDefault(normWireNullable(normWireNumber), () => null), constructionActivity: normWireDefault(normWireNullable(normWireString), () => null), assumedConstructionQk: normWireDefault(normWireNullable(normWireNumber), () => null), structureKind: normWireDefault(normWireNullable(normWireLiteral("building", "bridge")), () => null), bridgeLane: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), bridgeSpan: normWireDefault(normWireNullable(normWireNumber), () => null), bridgeLaneWidth: normWireDefault(normWireNullable(normWireNumber), () => null), assumedBridgeTandem: normWireDefault(normWireNullable(normWireNumber), () => null), assumedBridgeUdl: normWireDefault(normWireNullable(normWireNumber), () => null), assumedBridgeLm2: normWireDefault(normWireNullable(normWireNumber), () => null), assumedBridgeFootway: normWireDefault(normWireNullable(normWireNumber), () => null), assumedBridgeLm3: normWireDefault(normWireNullable(normWireNumber), () => null), assumedBridgeLm4: normWireDefault(normWireNullable(normWireNumber), () => null), bridgeLoadGroup: normWireDefault(normWireNullable(normWireString), () => null), craneClaimed: normWireDefault(normWireNullable(normWireBoolean), () => null), craneClass: normWireDefault(normWireNullable(normWireString), () => null), hoistClass: normWireDefault(normWireNullable(normWireString), () => null), hoistingSpeed: normWireDefault(normWireNullable(normWireNumber), () => null), assumedCraneWheel: normWireDefault(normWireNullable(normWireNumber), () => null), assumedCraneHorizontal: normWireDefault(normWireNullable(normWireNumber), () => null), siloClaimed: normWireDefault(normWireNullable(normWireBoolean), () => null), siloKind: normWireDefault(normWireNullable(normWireString), () => null), siloBulkDensity: normWireDefault(normWireNullable(normWireNumber), () => null), siloHeight: normWireDefault(normWireNullable(normWireNumber), () => null), siloHydraulicRadius: normWireDefault(normWireNullable(normWireNumber), () => null), siloMu: normWireDefault(normWireNullable(normWireNumber), () => null), siloK: normWireDefault(normWireNullable(normWireNumber), () => null), assumedSiloPressure: normWireDefault(normWireNullable(normWireNumber), () => null), assumedSiloPatch: normWireDefault(normWireNullable(normWireNumber), () => null), assumedSiloWallFriction: normWireDefault(normWireNullable(normWireNumber), () => null), floors: normWireDefault(normWireNullable(normWireObject<{ values: FloorArea[]; }>({ values: normWireRequired(normWireArray(parseFloorArea)) })), () => null), selfWeightElements: normWireDefault(normWireNullable(normWireObject<{ values: SelfWeightElement[]; }>({ values: normWireRequired(normWireArray(parseSelfWeightElement)) })), () => null), roofs: normWireDefault(normWireNullable(normWireObject<{ values: RoofArea[]; }>({ values: normWireRequired(normWireArray(parseRoofArea)) })), () => null), windFaces: normWireDefault(normWireNullable(normWireObject<{ values: WindFace[]; }>({ values: normWireRequired(normWireArray(parseWindFace)) })), () => null), accidentalCases: normWireDefault(normWireNullable(normWireObject<{ values: AccidentalCase[]; }>({ values: normWireRequired(normWireArray(parseAccidentalCase)) })), () => null) });
