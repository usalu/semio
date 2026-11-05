/** 📸️ `En1991Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

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
  tMax: number;
  /** @state artifact */
  tMin: number;
  /** @state artifact */
  t0: number;
  /** @state artifact */
  thermalElementType: string;
  /** @state artifact */
  thermalBridgeType: number;
  /** @state artifact */
  deltaTM: number;
  /** @state artifact */
  storeyCount: number;
  /** @state artifact */
  fireMode: "none" | "nominal" | "parametric";
  /** @state artifact */
  fireCurve: "standard" | "external" | "hydrocarbon" | "parametric";
  /** @state artifact */
  fireDuration: number;
  /** @state artifact */
  assumedGasTemperature: number;
  /** @state artifact */
  assumedHNet: number;
  /** @state artifact */
  fireCompartmentArea: number;
  /** @state artifact */
  fireCompartmentHeight: number;
  /** @state artifact */
  fireOpeningFactor: number;
  /** @state artifact */
  fireThermalInertia: number;
  /** @state artifact */
  fireOccupancy: En1991FireOccupancy;
  /** @state artifact */
  fireLoadDensityQf: number;
  /** @state artifact */
  assumedQfD: number;
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
  assumedBridgeLm3: number;
  /** @state artifact */
  assumedBridgeLm4: number;
  /** @state artifact */
  bridgeLoadGroup: string;
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
  bridgeTandemX: number;
  /** @state artifact */
  railAlpha: number;
  /** @state artifact */
  assumedRailLm71: number;
  /** @state artifact */
  assumedRailLm71Udl: number;
  /** @state artifact */
  assumedRailSw0: number;
  /** @state artifact */
  assumedRailSw2: number;
  /** @state artifact */
  craneX: number;
  /** @state artifact */
  craneRunwayLength: number;
  /** @state artifact */
  craneWheelBase: number;
  /** @state artifact */
  siloDiameter: number;
  /** @state artifact */
  siloEccentricity: number;
  /** @state artifact */
  siloHopperAngleDeg: number;
  /** @state artifact */
  bridgeCurveRadius: number;
  /** @state artifact */
  fireLoadBasis: "occupancyTable" | "userSpecified";
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
  /** @state artifact */
  bridgeLanes: BridgeLane[];
  /** @state artifact */
  constructionStages: ConstructionStage[];
  /** @state artifact */
  roofSurfaces: RoofSurface[];
  /** @state artifact */
  placements: Placement[];
  /** @state artifact */
  specialVehicles: SpecialVehicle[];
  /** @state artifact */
  windBridges: WindBridge[];
  /** @state artifact */
  dynamicElements: DynamicElement[];
  /** @state artifact */
  impactActions: ImpactAction[];
  /** @state artifact */
  internalExplosions: InternalExplosion[];
  /** @state artifact */
  thermalSections: ThermalSection[];
  /** @state artifact */
  cases: CaseRow[];
  /** @state artifact */
  helicopterLandings: HelicopterLanding[];
  /** @state artifact */
  tieRuns: TieRun[];
  /** @state artifact */
  constructionLoads: ConstructionLoad[];
  /** @state artifact */
  fallingLoads: FallingLoad[];
  /** @state artifact */
  executionImperfections: ExecutionImperfection[];
  /** @state artifact */
  derailmentCases: DerailmentCase[];
  /** @state artifact */
  craneHoists: CraneHoist[];
  /** @state artifact */
  footbridges: Footbridge[];
  /** @state artifact */
  railDynamics: RailDynamic[];
  /** @state artifact */
  loadDispersals: LoadDispersal[];
  /** @state artifact */
  bulkSolids: BulkSolid[];
  /** @state artifact */
  rotatingMachines: RotatingMachine[];
  /** @state artifact */
  fatigueModels: FatigueModel[];
  /** @state artifact */
  supportCollisions: SupportCollision[];
  /** @state artifact */
  localFires: LocalFire[];
  /** @state artifact */
  snowGuards: SnowGuard[];
  /** @state artifact */
  railDerailments: RailDerailment[];
  /** @state artifact */
  railLoadGroups: RailLoadGroup[];
  /** @state artifact */
  dustExplosions: DustExplosion[];
}

export type En1991FireOccupancy = "dwelling" | "office" | "hospital" | "school" | "hotel" | "library" | "shopping" | "theatre" | "transport";

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

export type En1991FireLoadBasis = "occupancyTable" | "userSpecified";

export interface BridgeLane {
  id: string;
  width: number;
  offset: number;
  lm1Lane: number;
}

export interface ConstructionStage {
  id: string;
  activity: string;
  durationDays: number;
  returnYears: number;
  assumedQk: number;
}

export interface RoofSurface {
  id: string;
  face: string;
  shape: string;
  pitchDeg: number;
  extent: number;
}

export interface Placement {
  id: string;
  x: number;
  y: number;
  z: number;
  yaw: number;
}

export interface SpecialVehicle {
  id: string;
  designation: string;
  x: number;
  y: number;
}

export interface WindBridge {
  id: string;
  deckWidth: number;
  deckDepth: number;
  parapetKind: number;
  parapetHeight: number;
  trafficKind: number;
  heightAboveGround: number;
  span: number;
  assumedForce: number;
}

export interface DynamicElement {
  id: string;
  height: number;
  width: number;
  firstFrequency: number;
  logDecrement: number;
  massPerLength: number;
  strouhal: number;
  assumedCscd: number;
}

export interface ImpactAction {
  id: string;
  kind: number;
  mass: number;
  speed: number;
  stiffness: number;
  height: number;
  x: number;
  y: number;
  assumedForce: number;
}

export interface InternalExplosion {
  id: string;
  volume: number;
  ventArea: number;
  ventPressure: number;
  assumedPressure: number;
}

export interface ThermalSection {
  id: string;
  deckType: number;
  surfacing: number;
  length: number;
  depth: number;
  alphaMicro: number;
  assumedRange: number;
}

export interface CaseNumber {
  id: string;
  value: number;
}

export interface CaseText {
  id: string;
  value: string;
}

export interface CaseRow {
  id: string;
  labelEn: string;
  labelDe: string;
  entryId: string;
  numbers: CaseNumber[];
  texts: CaseText[];
}

export interface HelicopterLanding {
  id: string;
  mass: number;
  x: number;
  y: number;
  height: number;
  assumedForce: number;
}

export interface TieRun {
  id: string;
  kind: number;
  spacing: number;
  span: number;
  deadLoad: number;
  imposedLoad: number;
  psi: number;
  x: number;
  y: number;
  providedForce: number;
}

export interface ConstructionLoad {
  id: string;
  stage: string;
  kind: number;
  magnitude: number;
}

export interface FallingLoad {
  id: string;
  mass: number;
  dropHeight: number;
  stoppingDistance: number;
  x: number;
  y: number;
  assumedForce: number;
}

export interface ExecutionImperfection {
  id: string;
  height: number;
  tolerance: number;
  verticalLoad: number;
  x: number;
  y: number;
  assumedForce: number;
}

export interface DerailmentCase {
  id: string;
  distance: number;
  designForceX: number;
  designForceY: number;
  x: number;
  y: number;
  assumedForce: number;
}

export interface CraneHoist {
  id: string;
  ratedLoad: number;
  releaseFraction: number;
  releaseDevice: string;
}

export interface Footbridge {
  id: string;
  loadedLength: number;
  deckWidth: number;
  naturalFrequency: number;
  assumedServiceAxle: number;
}

export interface RailDynamic {
  id: string;
  determinantLength: number;
  maintenance: string;
}

export interface LoadDispersal {
  id: string;
  wheelLength: number;
  wheelWidth: number;
  pavementThickness: number;
  slabThickness: number;
}

export interface BulkSolid {
  id: string;
  internalFriction: number;
}

export interface RotatingMachine {
  id: string;
  mass: number;
  eccentricity: number;
  frequency: number;
}

export interface FatigueModel {
  id: string;
  model: string;
  lane: number;
  assumedAxleLoad: number;
  assumedUdl: number;
}

export interface SupportCollision {
  id: string;
  location: string;
  assumedForceX: number;
  assumedForceY: number;
  height: number;
  positionX: number;
}

export interface LocalFire {
  id: string;
  fireDiameter: number;
  heatReleaseRate: number;
  ceilingHeight: number;
  horizontalDistance: number;
  assumedFlux: number;
}

export interface SnowGuard {
  id: string;
  roofId: string;
  slopeLength: number;
  assumedForce: number;
}

export interface RailDerailment {
  id: string;
  situation: string;
  assumedLineLoad: number;
  loadedLength: number;
  offset: number;
}

export interface RailLoadGroup {
  id: string;
  loadGroup: string;
  factorVertical: number;
  factorLongitudinal: number;
  factorLateral: number;
  assumedAxleLoad: number;
  assumedUdl: number;
  assumedLongitudinal: number;
  assumedLateral: number;
}

export interface DustExplosion {
  id: string;
  kst: number;
  pMax: number;
  volume: number;
  ventArea: number;
  staticOpeningPressure: number;
  designPressure: number;
}

export const parseEn1991Snapshot: NormWireReader<En1991Snapshot> = normWireObject<En1991Snapshot>({ annex: normWireRequired(normWireLiteral("En", "De")), snowZone: normWireRequired(normWireString), altitude: normWireRequired(normWireNumber), enSk: normWireRequired(normWireNumber), northGermanLowlandSnow: normWireRequired(normWireBoolean), windZone: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), enVb: normWireRequired(normWireNumber), terrainCategory: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), mixedTerrainUpwind: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), mixedTerrainDistance: normWireRequired(normWireNumber), orographyFactor: normWireRequired(normWireNumber), coastOrIsland: normWireRequired(normWireBoolean), airDensity: normWireRequired(normWireNumber), height: normWireRequired(normWireNumber), width: normWireRequired(normWireNumber), depth: normWireRequired(normWireNumber), assumedDeltaT: normWireRequired(normWireNumber), tMax: normWireRequired(normWireNumber), tMin: normWireRequired(normWireNumber), t0: normWireRequired(normWireNumber), thermalElementType: normWireRequired(normWireString), thermalBridgeType: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), deltaTM: normWireRequired(normWireNumber), storeyCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), fireMode: normWireRequired(normWireLiteral("none", "nominal", "parametric")), fireCurve: normWireRequired(normWireLiteral("standard", "external", "hydrocarbon", "parametric")), fireDuration: normWireRequired(normWireNumber), assumedGasTemperature: normWireRequired(normWireNumber), assumedHNet: normWireRequired(normWireNumber), fireCompartmentArea: normWireRequired(normWireNumber), fireCompartmentHeight: normWireRequired(normWireNumber), fireOpeningFactor: normWireRequired(normWireNumber), fireThermalInertia: normWireRequired(normWireNumber), fireOccupancy: normWireRequired(normWireRef(() => parseEn1991FireOccupancy)), fireLoadDensityQf: normWireRequired(normWireNumber), assumedQfD: normWireRequired(normWireNumber), constructionActivity: normWireRequired(normWireString), assumedConstructionQk: normWireRequired(normWireNumber), structureKind: normWireRequired(normWireLiteral("building", "bridge")), bridgeLane: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), bridgeSpan: normWireRequired(normWireNumber), bridgeLaneWidth: normWireRequired(normWireNumber), assumedBridgeTandem: normWireRequired(normWireNumber), assumedBridgeUdl: normWireRequired(normWireNumber), assumedBridgeLm2: normWireRequired(normWireNumber), assumedBridgeFootway: normWireRequired(normWireNumber), assumedBridgeLm3: normWireRequired(normWireNumber), assumedBridgeLm4: normWireRequired(normWireNumber), bridgeLoadGroup: normWireRequired(normWireString), craneClaimed: normWireRequired(normWireBoolean), craneClass: normWireRequired(normWireString), hoistClass: normWireRequired(normWireString), hoistingSpeed: normWireRequired(normWireNumber), assumedCraneWheel: normWireRequired(normWireNumber), assumedCraneHorizontal: normWireRequired(normWireNumber), siloClaimed: normWireRequired(normWireBoolean), siloKind: normWireRequired(normWireString), siloBulkDensity: normWireRequired(normWireNumber), siloHeight: normWireRequired(normWireNumber), siloHydraulicRadius: normWireRequired(normWireNumber), siloMu: normWireRequired(normWireNumber), siloK: normWireRequired(normWireNumber), assumedSiloPressure: normWireRequired(normWireNumber), assumedSiloPatch: normWireRequired(normWireNumber), assumedSiloWallFriction: normWireRequired(normWireNumber), bridgeTandemX: normWireRequired(normWireNumber), railAlpha: normWireRequired(normWireNumber), assumedRailLm71: normWireRequired(normWireNumber), assumedRailLm71Udl: normWireRequired(normWireNumber), assumedRailSw0: normWireRequired(normWireNumber), assumedRailSw2: normWireRequired(normWireNumber), craneX: normWireRequired(normWireNumber), craneRunwayLength: normWireRequired(normWireNumber), craneWheelBase: normWireRequired(normWireNumber), siloDiameter: normWireRequired(normWireNumber), siloEccentricity: normWireRequired(normWireNumber), siloHopperAngleDeg: normWireRequired(normWireNumber), bridgeCurveRadius: normWireRequired(normWireNumber), fireLoadBasis: normWireRequired(normWireLiteral("occupancyTable", "userSpecified")), floors: normWireRequired(normWireArray(normWireRef(() => parseFloorArea))), selfWeightElements: normWireRequired(normWireArray(normWireRef(() => parseSelfWeightElement))), roofs: normWireRequired(normWireArray(normWireRef(() => parseRoofArea))), windFaces: normWireRequired(normWireArray(normWireRef(() => parseWindFace))), accidentalCases: normWireRequired(normWireArray(normWireRef(() => parseAccidentalCase))), bridgeLanes: normWireRequired(normWireArray(normWireRef(() => parseBridgeLane))), constructionStages: normWireRequired(normWireArray(normWireRef(() => parseConstructionStage))), roofSurfaces: normWireRequired(normWireArray(normWireRef(() => parseRoofSurface))), placements: normWireRequired(normWireArray(normWireRef(() => parsePlacement))), specialVehicles: normWireRequired(normWireArray(normWireRef(() => parseSpecialVehicle))), windBridges: normWireRequired(normWireArray(normWireRef(() => parseWindBridge))), dynamicElements: normWireRequired(normWireArray(normWireRef(() => parseDynamicElement))), impactActions: normWireRequired(normWireArray(normWireRef(() => parseImpactAction))), internalExplosions: normWireRequired(normWireArray(normWireRef(() => parseInternalExplosion))), thermalSections: normWireRequired(normWireArray(normWireRef(() => parseThermalSection))), cases: normWireRequired(normWireArray(normWireRef(() => parseCaseRow))), helicopterLandings: normWireRequired(normWireArray(normWireRef(() => parseHelicopterLanding))), tieRuns: normWireRequired(normWireArray(normWireRef(() => parseTieRun))), constructionLoads: normWireRequired(normWireArray(normWireRef(() => parseConstructionLoad))), fallingLoads: normWireRequired(normWireArray(normWireRef(() => parseFallingLoad))), executionImperfections: normWireRequired(normWireArray(normWireRef(() => parseExecutionImperfection))), derailmentCases: normWireRequired(normWireArray(normWireRef(() => parseDerailmentCase))), craneHoists: normWireRequired(normWireArray(normWireRef(() => parseCraneHoist))), footbridges: normWireRequired(normWireArray(normWireRef(() => parseFootbridge))), railDynamics: normWireRequired(normWireArray(normWireRef(() => parseRailDynamic))), loadDispersals: normWireRequired(normWireArray(normWireRef(() => parseLoadDispersal))), bulkSolids: normWireRequired(normWireArray(normWireRef(() => parseBulkSolid))), rotatingMachines: normWireRequired(normWireArray(normWireRef(() => parseRotatingMachine))), fatigueModels: normWireRequired(normWireArray(normWireRef(() => parseFatigueModel))), supportCollisions: normWireRequired(normWireArray(normWireRef(() => parseSupportCollision))), localFires: normWireRequired(normWireArray(normWireRef(() => parseLocalFire))), snowGuards: normWireRequired(normWireArray(normWireRef(() => parseSnowGuard))), railDerailments: normWireRequired(normWireArray(normWireRef(() => parseRailDerailment))), railLoadGroups: normWireRequired(normWireArray(normWireRef(() => parseRailLoadGroup))), dustExplosions: normWireRequired(normWireArray(normWireRef(() => parseDustExplosion))) });
export const parseEn1991FireOccupancy: NormWireReader<En1991FireOccupancy> = normWireLiteral("dwelling", "office", "hospital", "school", "hotel", "library", "shopping", "theatre", "transport");
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
export const parseEn1991FireLoadBasis: NormWireReader<En1991FireLoadBasis> = normWireLiteral("occupancyTable", "userSpecified");
export const parseBridgeLane: NormWireReader<BridgeLane> = normWireObject<BridgeLane>({ id: normWireRequired(normWireString), width: normWireRequired(normWireNumber), offset: normWireRequired(normWireNumber), lm1Lane: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})) });
export const parseConstructionStage: NormWireReader<ConstructionStage> = normWireObject<ConstructionStage>({ id: normWireRequired(normWireString), activity: normWireRequired(normWireString), durationDays: normWireRequired(normWireNumber), returnYears: normWireRequired(normWireNumber), assumedQk: normWireRequired(normWireNumber) });
export const parseRoofSurface: NormWireReader<RoofSurface> = normWireObject<RoofSurface>({ id: normWireRequired(normWireString), face: normWireRequired(normWireString), shape: normWireRequired(normWireString), pitchDeg: normWireRequired(normWireNumber), extent: normWireRequired(normWireNumber) });
export const parsePlacement: NormWireReader<Placement> = normWireObject<Placement>({ id: normWireRequired(normWireString), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber), yaw: normWireRequired(normWireNumber) });
export const parseSpecialVehicle: NormWireReader<SpecialVehicle> = normWireObject<SpecialVehicle>({ id: normWireRequired(normWireString), designation: normWireRequired(normWireString), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber) });
export const parseWindBridge: NormWireReader<WindBridge> = normWireObject<WindBridge>({ id: normWireRequired(normWireString), deckWidth: normWireRequired(normWireNumber), deckDepth: normWireRequired(normWireNumber), parapetKind: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), parapetHeight: normWireRequired(normWireNumber), trafficKind: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), heightAboveGround: normWireRequired(normWireNumber), span: normWireRequired(normWireNumber), assumedForce: normWireRequired(normWireNumber) });
export const parseDynamicElement: NormWireReader<DynamicElement> = normWireObject<DynamicElement>({ id: normWireRequired(normWireString), height: normWireRequired(normWireNumber), width: normWireRequired(normWireNumber), firstFrequency: normWireRequired(normWireNumber), logDecrement: normWireRequired(normWireNumber), massPerLength: normWireRequired(normWireNumber), strouhal: normWireRequired(normWireNumber), assumedCscd: normWireRequired(normWireNumber) });
export const parseImpactAction: NormWireReader<ImpactAction> = normWireObject<ImpactAction>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), mass: normWireRequired(normWireNumber), speed: normWireRequired(normWireNumber), stiffness: normWireRequired(normWireNumber), height: normWireRequired(normWireNumber), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), assumedForce: normWireRequired(normWireNumber) });
export const parseInternalExplosion: NormWireReader<InternalExplosion> = normWireObject<InternalExplosion>({ id: normWireRequired(normWireString), volume: normWireRequired(normWireNumber), ventArea: normWireRequired(normWireNumber), ventPressure: normWireRequired(normWireNumber), assumedPressure: normWireRequired(normWireNumber) });
export const parseThermalSection: NormWireReader<ThermalSection> = normWireObject<ThermalSection>({ id: normWireRequired(normWireString), deckType: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), surfacing: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), length: normWireRequired(normWireNumber), depth: normWireRequired(normWireNumber), alphaMicro: normWireRequired(normWireNumber), assumedRange: normWireRequired(normWireNumber) });
export const parseCaseNumber: NormWireReader<CaseNumber> = normWireObject<CaseNumber>({ id: normWireRequired(normWireString), value: normWireRequired(normWireNumber) });
export const parseCaseText: NormWireReader<CaseText> = normWireObject<CaseText>({ id: normWireRequired(normWireString), value: normWireRequired(normWireString) });
export const parseCaseRow: NormWireReader<CaseRow> = normWireObject<CaseRow>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), entryId: normWireRequired(normWireString), numbers: normWireRequired(normWireArray(normWireRef(() => parseCaseNumber))), texts: normWireRequired(normWireArray(normWireRef(() => parseCaseText))) });
export const parseHelicopterLanding: NormWireReader<HelicopterLanding> = normWireObject<HelicopterLanding>({ id: normWireRequired(normWireString), mass: normWireRequired(normWireNumber), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), height: normWireRequired(normWireNumber), assumedForce: normWireRequired(normWireNumber) });
export const parseTieRun: NormWireReader<TieRun> = normWireObject<TieRun>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), spacing: normWireRequired(normWireNumber), span: normWireRequired(normWireNumber), deadLoad: normWireRequired(normWireNumber), imposedLoad: normWireRequired(normWireNumber), psi: normWireRequired(normWireNumber), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), providedForce: normWireRequired(normWireNumber) });
export const parseConstructionLoad: NormWireReader<ConstructionLoad> = normWireObject<ConstructionLoad>({ id: normWireRequired(normWireString), stage: normWireRequired(normWireString), kind: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), magnitude: normWireRequired(normWireNumber) });
export const parseFallingLoad: NormWireReader<FallingLoad> = normWireObject<FallingLoad>({ id: normWireRequired(normWireString), mass: normWireRequired(normWireNumber), dropHeight: normWireRequired(normWireNumber), stoppingDistance: normWireRequired(normWireNumber), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), assumedForce: normWireRequired(normWireNumber) });
export const parseExecutionImperfection: NormWireReader<ExecutionImperfection> = normWireObject<ExecutionImperfection>({ id: normWireRequired(normWireString), height: normWireRequired(normWireNumber), tolerance: normWireRequired(normWireNumber), verticalLoad: normWireRequired(normWireNumber), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), assumedForce: normWireRequired(normWireNumber) });
export const parseDerailmentCase: NormWireReader<DerailmentCase> = normWireObject<DerailmentCase>({ id: normWireRequired(normWireString), distance: normWireRequired(normWireNumber), designForceX: normWireRequired(normWireNumber), designForceY: normWireRequired(normWireNumber), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), assumedForce: normWireRequired(normWireNumber) });
export const parseCraneHoist: NormWireReader<CraneHoist> = normWireObject<CraneHoist>({ id: normWireRequired(normWireString), ratedLoad: normWireRequired(normWireNumber), releaseFraction: normWireRequired(normWireNumber), releaseDevice: normWireRequired(normWireString) });
export const parseFootbridge: NormWireReader<Footbridge> = normWireObject<Footbridge>({ id: normWireRequired(normWireString), loadedLength: normWireRequired(normWireNumber), deckWidth: normWireRequired(normWireNumber), naturalFrequency: normWireRequired(normWireNumber), assumedServiceAxle: normWireRequired(normWireNumber) });
export const parseRailDynamic: NormWireReader<RailDynamic> = normWireObject<RailDynamic>({ id: normWireRequired(normWireString), determinantLength: normWireRequired(normWireNumber), maintenance: normWireRequired(normWireString) });
export const parseLoadDispersal: NormWireReader<LoadDispersal> = normWireObject<LoadDispersal>({ id: normWireRequired(normWireString), wheelLength: normWireRequired(normWireNumber), wheelWidth: normWireRequired(normWireNumber), pavementThickness: normWireRequired(normWireNumber), slabThickness: normWireRequired(normWireNumber) });
export const parseBulkSolid: NormWireReader<BulkSolid> = normWireObject<BulkSolid>({ id: normWireRequired(normWireString), internalFriction: normWireRequired(normWireNumber) });
export const parseRotatingMachine: NormWireReader<RotatingMachine> = normWireObject<RotatingMachine>({ id: normWireRequired(normWireString), mass: normWireRequired(normWireNumber), eccentricity: normWireRequired(normWireNumber), frequency: normWireRequired(normWireNumber) });
export const parseFatigueModel: NormWireReader<FatigueModel> = normWireObject<FatigueModel>({ id: normWireRequired(normWireString), model: normWireRequired(normWireString), lane: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), assumedAxleLoad: normWireRequired(normWireNumber), assumedUdl: normWireRequired(normWireNumber) });
export const parseSupportCollision: NormWireReader<SupportCollision> = normWireObject<SupportCollision>({ id: normWireRequired(normWireString), location: normWireRequired(normWireString), assumedForceX: normWireRequired(normWireNumber), assumedForceY: normWireRequired(normWireNumber), height: normWireRequired(normWireNumber), positionX: normWireRequired(normWireNumber) });
export const parseLocalFire: NormWireReader<LocalFire> = normWireObject<LocalFire>({ id: normWireRequired(normWireString), fireDiameter: normWireRequired(normWireNumber), heatReleaseRate: normWireRequired(normWireNumber), ceilingHeight: normWireRequired(normWireNumber), horizontalDistance: normWireRequired(normWireNumber), assumedFlux: normWireRequired(normWireNumber) });
export const parseSnowGuard: NormWireReader<SnowGuard> = normWireObject<SnowGuard>({ id: normWireRequired(normWireString), roofId: normWireRequired(normWireString), slopeLength: normWireRequired(normWireNumber), assumedForce: normWireRequired(normWireNumber) });
export const parseRailDerailment: NormWireReader<RailDerailment> = normWireObject<RailDerailment>({ id: normWireRequired(normWireString), situation: normWireRequired(normWireString), assumedLineLoad: normWireRequired(normWireNumber), loadedLength: normWireRequired(normWireNumber), offset: normWireRequired(normWireNumber) });
export const parseRailLoadGroup: NormWireReader<RailLoadGroup> = normWireObject<RailLoadGroup>({ id: normWireRequired(normWireString), loadGroup: normWireRequired(normWireString), factorVertical: normWireRequired(normWireNumber), factorLongitudinal: normWireRequired(normWireNumber), factorLateral: normWireRequired(normWireNumber), assumedAxleLoad: normWireRequired(normWireNumber), assumedUdl: normWireRequired(normWireNumber), assumedLongitudinal: normWireRequired(normWireNumber), assumedLateral: normWireRequired(normWireNumber) });
export const parseDustExplosion: NormWireReader<DustExplosion> = normWireObject<DustExplosion>({ id: normWireRequired(normWireString), kst: normWireRequired(normWireNumber), pMax: normWireRequired(normWireNumber), volume: normWireRequired(normWireNumber), ventArea: normWireRequired(normWireNumber), staticOpeningPressure: normWireRequired(normWireNumber), designPressure: normWireRequired(normWireNumber) });
