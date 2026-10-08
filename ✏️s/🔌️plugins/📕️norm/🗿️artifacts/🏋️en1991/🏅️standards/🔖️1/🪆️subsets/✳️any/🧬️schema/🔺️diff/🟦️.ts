/** 🔺️ `En1991Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireDefault, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type AccidentalCase, type AccidentalExplosion, type AccidentalImpact, type FloorArea, parseAccidentalCase, parseAccidentalExplosion, parseAccidentalImpact, parseFloorArea, parseRoofArea, parseSelfWeightElement, parseWindFace, type RoofArea, type SelfWeightElement, type WindFace } from "../📸️snapshot/🟦️.ts";

export interface En1991Diff {
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
  floors?: En1991FloorDelta;
  /** @state artifact */
  selfWeightElements?: En1991SelfWeightElementDelta;
  /** @state artifact */
  roofs?: En1991RoofDelta;
  /** @state artifact */
  windFaces?: En1991WindFaceDelta;
  /** @state artifact */
  accidentalCases?: En1991AccidentalCaseDelta;
}

export interface En1991FloorPatch {
  category: string | null;
  area: number | null;
  assumedQk: number | null;
  assumedQkConcentrated: number | null;
  assumedPartitions: number | null;
}

export interface En1991FloorAddition {
  after: string | null;
  row: FloorArea;
}

export interface En1991FloorModification {
  key: string;
  patch: En1991FloorPatch;
}

export interface En1991FloorDelta {
  removed: string[];
  added: En1991FloorAddition[];
  modified: En1991FloorModification[];
}

export interface En1991SelfWeightElementPatch {
  material: string | null;
  thickness: number | null;
  assumedGk: number | null;
}

export interface En1991SelfWeightElementAddition {
  after: string | null;
  row: SelfWeightElement;
}

export interface En1991SelfWeightElementModification {
  key: string;
  patch: En1991SelfWeightElementPatch;
}

export interface En1991SelfWeightElementDelta {
  removed: string[];
  added: En1991SelfWeightElementAddition[];
  modified: En1991SelfWeightElementModification[];
}

export interface En1991RoofPatch {
  roofType: string | null;
  pitchDeg: number | null;
  cE: number | null;
  cT: number | null;
  hasParapet: boolean | null;
  parapetHeight: number | null;
  driftObstructionHeight: number | null;
  multiSpan: boolean | null;
  assumedSk: number | null;
}

export interface En1991RoofAddition {
  after: string | null;
  row: RoofArea;
}

export interface En1991RoofModification {
  key: string;
  patch: En1991RoofPatch;
}

export interface En1991RoofDelta {
  removed: string[];
  added: En1991RoofAddition[];
  modified: En1991RoofModification[];
}

export interface En1991WindFacePatch {
  zone: string | null;
  z: number | null;
  cPe10: number | null;
  cPe1: number | null;
  cPi: number | null;
  cS: number | null;
  cD: number | null;
  loadedArea: number | null;
  assumedWp: number | null;
}

export interface En1991WindFaceAddition {
  after: string | null;
  row: WindFace;
}

export interface En1991WindFaceModification {
  key: string;
  patch: En1991WindFacePatch;
}

export interface En1991WindFaceDelta {
  removed: string[];
  added: En1991WindFaceAddition[];
  modified: En1991WindFaceModification[];
}

export interface En1991AccidentalCasePatch {
  impact: AccidentalImpact[] | null;
  explosion: AccidentalExplosion[] | null;
}

export interface En1991AccidentalCaseAddition {
  after: string | null;
  row: AccidentalCase;
}

export interface En1991AccidentalCaseModification {
  key: string;
  patch: En1991AccidentalCasePatch;
}

export interface En1991AccidentalCaseDelta {
  removed: string[];
  added: En1991AccidentalCaseAddition[];
  modified: En1991AccidentalCaseModification[];
}

export const parseEn1991Diff: NormWireReader<En1991Diff> = normWireObject<En1991Diff>({ annex: normWireDefault(normWireNullable(normWireLiteral("En", "De")), () => null), snowZone: normWireDefault(normWireNullable(normWireString), () => null), altitude: normWireDefault(normWireNullable(normWireNumber), () => null), enSk: normWireDefault(normWireNullable(normWireNumber), () => null), northGermanLowlandSnow: normWireDefault(normWireNullable(normWireBoolean), () => null), windZone: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), enVb: normWireDefault(normWireNullable(normWireNumber), () => null), terrainCategory: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), mixedTerrainUpwind: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), mixedTerrainDistance: normWireDefault(normWireNullable(normWireNumber), () => null), orographyFactor: normWireDefault(normWireNullable(normWireNumber), () => null), coastOrIsland: normWireDefault(normWireNullable(normWireBoolean), () => null), airDensity: normWireDefault(normWireNullable(normWireNumber), () => null), height: normWireDefault(normWireNullable(normWireNumber), () => null), width: normWireDefault(normWireNullable(normWireNumber), () => null), depth: normWireDefault(normWireNullable(normWireNumber), () => null), assumedDeltaT: normWireDefault(normWireNullable(normWireNumber), () => null), tMax: normWireDefault(normWireNullable(normWireNumber), () => null), tMin: normWireDefault(normWireNullable(normWireNumber), () => null), t0: normWireDefault(normWireNullable(normWireNumber), () => null), thermalElementType: normWireDefault(normWireNullable(normWireString), () => null), thermalBridgeType: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), deltaTM: normWireDefault(normWireNullable(normWireNumber), () => null), storeyCount: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), fireMode: normWireDefault(normWireNullable(normWireLiteral("none", "nominal", "parametric")), () => null), fireCurve: normWireDefault(normWireNullable(normWireLiteral("standard", "external", "hydrocarbon", "parametric")), () => null), fireDuration: normWireDefault(normWireNullable(normWireNumber), () => null), assumedGasTemperature: normWireDefault(normWireNullable(normWireNumber), () => null), assumedHNet: normWireDefault(normWireNullable(normWireNumber), () => null), fireCompartmentArea: normWireDefault(normWireNullable(normWireNumber), () => null), fireCompartmentHeight: normWireDefault(normWireNullable(normWireNumber), () => null), fireOpeningFactor: normWireDefault(normWireNullable(normWireNumber), () => null), fireThermalInertia: normWireDefault(normWireNullable(normWireNumber), () => null), fireOccupancy: normWireDefault(normWireNullable(normWireString), () => null), fireLoadDensityQf: normWireDefault(normWireNullable(normWireNumber), () => null), assumedQfD: normWireDefault(normWireNullable(normWireNumber), () => null), constructionActivity: normWireDefault(normWireNullable(normWireString), () => null), assumedConstructionQk: normWireDefault(normWireNullable(normWireNumber), () => null), structureKind: normWireDefault(normWireNullable(normWireLiteral("building", "bridge")), () => null), bridgeLane: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), bridgeSpan: normWireDefault(normWireNullable(normWireNumber), () => null), bridgeLaneWidth: normWireDefault(normWireNullable(normWireNumber), () => null), assumedBridgeTandem: normWireDefault(normWireNullable(normWireNumber), () => null), assumedBridgeUdl: normWireDefault(normWireNullable(normWireNumber), () => null), assumedBridgeLm2: normWireDefault(normWireNullable(normWireNumber), () => null), assumedBridgeFootway: normWireDefault(normWireNullable(normWireNumber), () => null), assumedBridgeLm3: normWireDefault(normWireNullable(normWireNumber), () => null), assumedBridgeLm4: normWireDefault(normWireNullable(normWireNumber), () => null), bridgeLoadGroup: normWireDefault(normWireNullable(normWireString), () => null), craneClaimed: normWireDefault(normWireNullable(normWireBoolean), () => null), craneClass: normWireDefault(normWireNullable(normWireString), () => null), hoistClass: normWireDefault(normWireNullable(normWireString), () => null), hoistingSpeed: normWireDefault(normWireNullable(normWireNumber), () => null), assumedCraneWheel: normWireDefault(normWireNullable(normWireNumber), () => null), assumedCraneHorizontal: normWireDefault(normWireNullable(normWireNumber), () => null), siloClaimed: normWireDefault(normWireNullable(normWireBoolean), () => null), siloKind: normWireDefault(normWireNullable(normWireString), () => null), siloBulkDensity: normWireDefault(normWireNullable(normWireNumber), () => null), siloHeight: normWireDefault(normWireNullable(normWireNumber), () => null), siloHydraulicRadius: normWireDefault(normWireNullable(normWireNumber), () => null), siloMu: normWireDefault(normWireNullable(normWireNumber), () => null), siloK: normWireDefault(normWireNullable(normWireNumber), () => null), assumedSiloPressure: normWireDefault(normWireNullable(normWireNumber), () => null), assumedSiloPatch: normWireDefault(normWireNullable(normWireNumber), () => null), assumedSiloWallFriction: normWireDefault(normWireNullable(normWireNumber), () => null), floors: normWireOptional(normWireRef(() => parseEn1991FloorDelta)), selfWeightElements: normWireOptional(normWireRef(() => parseEn1991SelfWeightElementDelta)), roofs: normWireOptional(normWireRef(() => parseEn1991RoofDelta)), windFaces: normWireOptional(normWireRef(() => parseEn1991WindFaceDelta)), accidentalCases: normWireOptional(normWireRef(() => parseEn1991AccidentalCaseDelta)) });
export const parseEn1991FloorPatch: NormWireReader<En1991FloorPatch> = normWireObject<En1991FloorPatch>({ category: normWireDefault(normWireNullable(normWireString), () => null), area: normWireDefault(normWireNullable(normWireNumber), () => null), assumedQk: normWireDefault(normWireNullable(normWireNumber), () => null), assumedQkConcentrated: normWireDefault(normWireNullable(normWireNumber), () => null), assumedPartitions: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1991FloorAddition: NormWireReader<En1991FloorAddition> = normWireObject<En1991FloorAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseFloorArea) });
export const parseEn1991FloorModification: NormWireReader<En1991FloorModification> = normWireObject<En1991FloorModification>({ key: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991FloorPatch)) });
export const parseEn1991FloorDelta: NormWireReader<En1991FloorDelta> = normWireObject<En1991FloorDelta>({ removed: normWireRequired(normWireArray(normWireString)), added: normWireRequired(normWireArray(normWireRef(() => parseEn1991FloorAddition))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991FloorModification))) });
export const parseEn1991SelfWeightElementPatch: NormWireReader<En1991SelfWeightElementPatch> = normWireObject<En1991SelfWeightElementPatch>({ material: normWireDefault(normWireNullable(normWireString), () => null), thickness: normWireDefault(normWireNullable(normWireNumber), () => null), assumedGk: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1991SelfWeightElementAddition: NormWireReader<En1991SelfWeightElementAddition> = normWireObject<En1991SelfWeightElementAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseSelfWeightElement) });
export const parseEn1991SelfWeightElementModification: NormWireReader<En1991SelfWeightElementModification> = normWireObject<En1991SelfWeightElementModification>({ key: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991SelfWeightElementPatch)) });
export const parseEn1991SelfWeightElementDelta: NormWireReader<En1991SelfWeightElementDelta> = normWireObject<En1991SelfWeightElementDelta>({ removed: normWireRequired(normWireArray(normWireString)), added: normWireRequired(normWireArray(normWireRef(() => parseEn1991SelfWeightElementAddition))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991SelfWeightElementModification))) });
export const parseEn1991RoofPatch: NormWireReader<En1991RoofPatch> = normWireObject<En1991RoofPatch>({ roofType: normWireDefault(normWireNullable(normWireString), () => null), pitchDeg: normWireDefault(normWireNullable(normWireNumber), () => null), cE: normWireDefault(normWireNullable(normWireNumber), () => null), cT: normWireDefault(normWireNullable(normWireNumber), () => null), hasParapet: normWireDefault(normWireNullable(normWireBoolean), () => null), parapetHeight: normWireDefault(normWireNullable(normWireNumber), () => null), driftObstructionHeight: normWireDefault(normWireNullable(normWireNumber), () => null), multiSpan: normWireDefault(normWireNullable(normWireBoolean), () => null), assumedSk: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1991RoofAddition: NormWireReader<En1991RoofAddition> = normWireObject<En1991RoofAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseRoofArea) });
export const parseEn1991RoofModification: NormWireReader<En1991RoofModification> = normWireObject<En1991RoofModification>({ key: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991RoofPatch)) });
export const parseEn1991RoofDelta: NormWireReader<En1991RoofDelta> = normWireObject<En1991RoofDelta>({ removed: normWireRequired(normWireArray(normWireString)), added: normWireRequired(normWireArray(normWireRef(() => parseEn1991RoofAddition))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991RoofModification))) });
export const parseEn1991WindFacePatch: NormWireReader<En1991WindFacePatch> = normWireObject<En1991WindFacePatch>({ zone: normWireDefault(normWireNullable(normWireString), () => null), z: normWireDefault(normWireNullable(normWireNumber), () => null), cPe10: normWireDefault(normWireNullable(normWireNumber), () => null), cPe1: normWireDefault(normWireNullable(normWireNumber), () => null), cPi: normWireDefault(normWireNullable(normWireNumber), () => null), cS: normWireDefault(normWireNullable(normWireNumber), () => null), cD: normWireDefault(normWireNullable(normWireNumber), () => null), loadedArea: normWireDefault(normWireNullable(normWireNumber), () => null), assumedWp: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1991WindFaceAddition: NormWireReader<En1991WindFaceAddition> = normWireObject<En1991WindFaceAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseWindFace) });
export const parseEn1991WindFaceModification: NormWireReader<En1991WindFaceModification> = normWireObject<En1991WindFaceModification>({ key: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991WindFacePatch)) });
export const parseEn1991WindFaceDelta: NormWireReader<En1991WindFaceDelta> = normWireObject<En1991WindFaceDelta>({ removed: normWireRequired(normWireArray(normWireString)), added: normWireRequired(normWireArray(normWireRef(() => parseEn1991WindFaceAddition))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991WindFaceModification))) });
export const parseEn1991AccidentalCasePatch: NormWireReader<En1991AccidentalCasePatch> = normWireObject<En1991AccidentalCasePatch>({ impact: normWireDefault(normWireNullable(normWireArray(parseAccidentalImpact)), () => null), explosion: normWireDefault(normWireNullable(normWireArray(parseAccidentalExplosion)), () => null) });
export const parseEn1991AccidentalCaseAddition: NormWireReader<En1991AccidentalCaseAddition> = normWireObject<En1991AccidentalCaseAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseAccidentalCase) });
export const parseEn1991AccidentalCaseModification: NormWireReader<En1991AccidentalCaseModification> = normWireObject<En1991AccidentalCaseModification>({ key: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991AccidentalCasePatch)) });
export const parseEn1991AccidentalCaseDelta: NormWireReader<En1991AccidentalCaseDelta> = normWireObject<En1991AccidentalCaseDelta>({ removed: normWireRequired(normWireArray(normWireString)), added: normWireRequired(normWireArray(normWireRef(() => parseEn1991AccidentalCaseAddition))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991AccidentalCaseModification))) });
