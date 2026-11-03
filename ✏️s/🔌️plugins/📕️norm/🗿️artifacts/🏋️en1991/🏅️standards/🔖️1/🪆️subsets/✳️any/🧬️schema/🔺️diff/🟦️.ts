/** 🔺️ `En1991Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type AccidentalCase, parseAccidentalCase, type AccidentalExplosion, parseAccidentalExplosion, type AccidentalImpact, parseAccidentalImpact, type BridgeLane, parseBridgeLane, type BulkSolid, parseBulkSolid, type CaseNumber, parseCaseNumber, type CaseRow, parseCaseRow, type CaseText, parseCaseText, type ConstructionLoad, parseConstructionLoad, type ConstructionStage, parseConstructionStage, type CraneHoist, parseCraneHoist, type DerailmentCase, parseDerailmentCase, type DynamicElement, parseDynamicElement, type En1991FireOccupancy, parseEn1991FireOccupancy, type ExecutionImperfection, parseExecutionImperfection, type FallingLoad, parseFallingLoad, type FatigueModel, parseFatigueModel, type FloorArea, parseFloorArea, type Footbridge, parseFootbridge, type HelicopterLanding, parseHelicopterLanding, type ImpactAction, parseImpactAction, type InternalExplosion, parseInternalExplosion, type LoadDispersal, parseLoadDispersal, type LocalFire, parseLocalFire, type Placement, parsePlacement, type RailDynamic, parseRailDynamic, type RoofArea, parseRoofArea, type RoofSurface, parseRoofSurface, type RotatingMachine, parseRotatingMachine, type SelfWeightElement, parseSelfWeightElement, type SnowGuard, parseSnowGuard, type SpecialVehicle, parseSpecialVehicle, type SupportCollision, parseSupportCollision, type ThermalSection, parseThermalSection, type TieRun, parseTieRun, type WindBridge, parseWindBridge, type WindFace, parseWindFace } from "../📸️snapshot/🟦️.ts";

export interface En1991Diff {
  /** @state artifact */
  annex?: ("En" | "De") | null;
  /** @state artifact */
  snowZone?: string | null;
  /** @state artifact */
  altitude?: number | null;
  /** @state artifact */
  enSk?: number | null;
  /** @state artifact */
  northGermanLowlandSnow?: boolean | null;
  /** @state artifact */
  windZone?: number | null;
  /** @state artifact */
  enVb?: number | null;
  /** @state artifact */
  terrainCategory?: number | null;
  /** @state artifact */
  mixedTerrainUpwind?: number | null;
  /** @state artifact */
  mixedTerrainDistance?: number | null;
  /** @state artifact */
  orographyFactor?: number | null;
  /** @state artifact */
  coastOrIsland?: boolean | null;
  /** @state artifact */
  airDensity?: number | null;
  /** @state artifact */
  height?: number | null;
  /** @state artifact */
  width?: number | null;
  /** @state artifact */
  depth?: number | null;
  /** @state artifact */
  assumedDeltaT?: number | null;
  /** @state artifact */
  tMax?: number | null;
  /** @state artifact */
  tMin?: number | null;
  /** @state artifact */
  t0?: number | null;
  /** @state artifact */
  thermalElementType?: string | null;
  /** @state artifact */
  thermalBridgeType?: number | null;
  /** @state artifact */
  deltaTM?: number | null;
  /** @state artifact */
  storeyCount?: number | null;
  /** @state artifact */
  fireMode?: ("none" | "nominal" | "parametric") | null;
  /** @state artifact */
  fireCurve?: ("standard" | "external" | "hydrocarbon" | "parametric") | null;
  /** @state artifact */
  fireDuration?: number | null;
  /** @state artifact */
  assumedGasTemperature?: number | null;
  /** @state artifact */
  assumedHNet?: number | null;
  /** @state artifact */
  fireCompartmentArea?: number | null;
  /** @state artifact */
  fireCompartmentHeight?: number | null;
  /** @state artifact */
  fireOpeningFactor?: number | null;
  /** @state artifact */
  fireThermalInertia?: number | null;
  /** @state artifact */
  fireOccupancy?: En1991FireOccupancy | null;
  /** @state artifact */
  fireLoadDensityQf?: number | null;
  /** @state artifact */
  assumedQfD?: number | null;
  /** @state artifact */
  constructionActivity?: string | null;
  /** @state artifact */
  assumedConstructionQk?: number | null;
  /** @state artifact */
  structureKind?: ("building" | "bridge") | null;
  /** @state artifact */
  bridgeLane?: number | null;
  /** @state artifact */
  bridgeSpan?: number | null;
  /** @state artifact */
  bridgeLaneWidth?: number | null;
  /** @state artifact */
  assumedBridgeTandem?: number | null;
  /** @state artifact */
  assumedBridgeUdl?: number | null;
  /** @state artifact */
  assumedBridgeLm2?: number | null;
  /** @state artifact */
  assumedBridgeFootway?: number | null;
  /** @state artifact */
  assumedBridgeLm3?: number | null;
  /** @state artifact */
  assumedBridgeLm4?: number | null;
  /** @state artifact */
  bridgeLoadGroup?: string | null;
  /** @state artifact */
  craneClaimed?: boolean | null;
  /** @state artifact */
  craneClass?: string | null;
  /** @state artifact */
  hoistClass?: string | null;
  /** @state artifact */
  hoistingSpeed?: number | null;
  /** @state artifact */
  assumedCraneWheel?: number | null;
  /** @state artifact */
  assumedCraneHorizontal?: number | null;
  /** @state artifact */
  siloClaimed?: boolean | null;
  /** @state artifact */
  siloKind?: string | null;
  /** @state artifact */
  siloBulkDensity?: number | null;
  /** @state artifact */
  siloHeight?: number | null;
  /** @state artifact */
  siloHydraulicRadius?: number | null;
  /** @state artifact */
  siloMu?: number | null;
  /** @state artifact */
  siloK?: number | null;
  /** @state artifact */
  assumedSiloPressure?: number | null;
  /** @state artifact */
  assumedSiloPatch?: number | null;
  /** @state artifact */
  assumedSiloWallFriction?: number | null;
  /** @state artifact */
  bridgeTandemX?: number | null;
  /** @state artifact */
  railAlpha?: number | null;
  /** @state artifact */
  assumedRailLm71?: number | null;
  /** @state artifact */
  assumedRailLm71Udl?: number | null;
  /** @state artifact */
  assumedRailSw0?: number | null;
  /** @state artifact */
  assumedRailSw2?: number | null;
  /** @state artifact */
  craneX?: number | null;
  /** @state artifact */
  craneRunwayLength?: number | null;
  /** @state artifact */
  craneWheelBase?: number | null;
  /** @state artifact */
  siloDiameter?: number | null;
  /** @state artifact */
  siloEccentricity?: number | null;
  /** @state artifact */
  siloHopperAngleDeg?: number | null;
  /** @state artifact */
  bridgeCurveRadius?: number | null;
  /** @state artifact */
  fireLoadBasis?: ("occupancyTable" | "userSpecified") | null;
  /** @state artifact */
  floors: En1991FloorDelta;
  /** @state artifact */
  selfWeightElements: En1991SelfWeightElementDelta;
  /** @state artifact */
  roofs: En1991RoofDelta;
  /** @state artifact */
  windFaces: En1991WindFaceDelta;
  /** @state artifact */
  accidentalCases: En1991AccidentalCaseDelta;
  /** @state artifact */
  bridgeLanes: En1991BridgeLaneDelta;
  /** @state artifact */
  constructionStages: En1991ConstructionStageDelta;
  /** @state artifact */
  roofSurfaces: En1991RoofSurfaceDelta;
  /** @state artifact */
  placements: En1991PlacementDelta;
  /** @state artifact */
  specialVehicles: En1991SpecialVehicleDelta;
  /** @state artifact */
  windBridges: En1991WindBridgeDelta;
  /** @state artifact */
  dynamicElements: En1991DynamicElementDelta;
  /** @state artifact */
  impactActions: En1991ImpactActionDelta;
  /** @state artifact */
  internalExplosions: En1991InternalExplosionDelta;
  /** @state artifact */
  thermalSections: En1991ThermalSectionDelta;
  /** @state artifact */
  cases: CaseRowDelta;
  /** @state artifact */
  helicopterLandings: En1991HelicopterLandingDelta;
  /** @state artifact */
  tieRuns: En1991TieRunDelta;
  /** @state artifact */
  constructionLoads: En1991ConstructionLoadDelta;
  /** @state artifact */
  fallingLoads: En1991FallingLoadDelta;
  /** @state artifact */
  executionImperfections: En1991ExecutionImperfectionDelta;
  /** @state artifact */
  derailmentCases: En1991DerailmentCaseDelta;
  /** @state artifact */
  craneHoists: En1991CraneHoistDelta;
  /** @state artifact */
  footbridges: En1991FootbridgeDelta;
  /** @state artifact */
  railDynamics: En1991RailDynamicDelta;
  /** @state artifact */
  loadDispersals: En1991LoadDispersalDelta;
  /** @state artifact */
  bulkSolids: En1991BulkSolidDelta;
  /** @state artifact */
  rotatingMachines: En1991RotatingMachineDelta;
  /** @state artifact */
  fatigueModels?: En1991FatigueModelDelta;
  /** @state artifact */
  supportCollisions?: En1991SupportCollisionDelta;
  /** @state artifact */
  localFires?: En1991LocalFireDelta;
  /** @state artifact */
  snowGuards?: En1991SnowGuardDelta;
}

export interface En1991FloorPatch {
  category?: string | null;
  area?: number | null;
  assumedQk?: number | null;
  assumedQkConcentrated?: number | null;
  assumedPartitions?: number | null;
}

export interface En1991FloorRemoval {
  id: string;
  index: number;
}

export interface En1991FloorInsertion {
  index: number;
  row: FloorArea;
}

export interface En1991FloorRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991FloorModification {
  id: string;
  patch: En1991FloorPatch;
}

export interface En1991FloorDelta {
  removed: En1991FloorRemoval[];
  inserted: En1991FloorInsertion[];
  moved: En1991FloorRelocation[];
  modified: En1991FloorModification[];
}

export interface En1991SelfWeightElementPatch {
  material?: string | null;
  thickness?: number | null;
  assumedGk?: number | null;
}

export interface En1991SelfWeightElementRemoval {
  id: string;
  index: number;
}

export interface En1991SelfWeightElementInsertion {
  index: number;
  row: SelfWeightElement;
}

export interface En1991SelfWeightElementRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991SelfWeightElementModification {
  id: string;
  patch: En1991SelfWeightElementPatch;
}

export interface En1991SelfWeightElementDelta {
  removed: En1991SelfWeightElementRemoval[];
  inserted: En1991SelfWeightElementInsertion[];
  moved: En1991SelfWeightElementRelocation[];
  modified: En1991SelfWeightElementModification[];
}

export interface En1991RoofPatch {
  roofType?: string | null;
  pitchDeg?: number | null;
  cE?: number | null;
  cT?: number | null;
  hasParapet?: boolean | null;
  parapetHeight?: number | null;
  driftObstructionHeight?: number | null;
  multiSpan?: boolean | null;
  assumedSk?: number | null;
}

export interface En1991RoofRemoval {
  id: string;
  index: number;
}

export interface En1991RoofInsertion {
  index: number;
  row: RoofArea;
}

export interface En1991RoofRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991RoofModification {
  id: string;
  patch: En1991RoofPatch;
}

export interface En1991RoofDelta {
  removed: En1991RoofRemoval[];
  inserted: En1991RoofInsertion[];
  moved: En1991RoofRelocation[];
  modified: En1991RoofModification[];
}

export interface En1991WindFacePatch {
  zone?: string | null;
  z?: number | null;
  cPe10?: number | null;
  cPe1?: number | null;
  cPi?: number | null;
  cS?: number | null;
  cD?: number | null;
  loadedArea?: number | null;
  assumedWp?: number | null;
}

export interface En1991WindFaceRemoval {
  id: string;
  index: number;
}

export interface En1991WindFaceInsertion {
  index: number;
  row: WindFace;
}

export interface En1991WindFaceRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991WindFaceModification {
  id: string;
  patch: En1991WindFacePatch;
}

export interface En1991WindFaceDelta {
  removed: En1991WindFaceRemoval[];
  inserted: En1991WindFaceInsertion[];
  moved: En1991WindFaceRelocation[];
  modified: En1991WindFaceModification[];
}

export interface En1991AccidentalCasePatch {
  impact?: AccidentalImpact[] | null;
  explosion?: AccidentalExplosion[] | null;
}

export interface En1991AccidentalCaseRemoval {
  id: string;
  index: number;
}

export interface En1991AccidentalCaseInsertion {
  index: number;
  row: AccidentalCase;
}

export interface En1991AccidentalCaseRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991AccidentalCaseModification {
  id: string;
  patch: En1991AccidentalCasePatch;
}

export interface En1991AccidentalCaseDelta {
  removed: En1991AccidentalCaseRemoval[];
  inserted: En1991AccidentalCaseInsertion[];
  moved: En1991AccidentalCaseRelocation[];
  modified: En1991AccidentalCaseModification[];
}

export interface En1991BridgeLanePatch {
  width?: number | null;
  offset?: number | null;
  lm1Lane?: number | null;
}

export interface En1991BridgeLaneRemoval {
  id: string;
  index: number;
}

export interface En1991BridgeLaneInsertion {
  index: number;
  row: BridgeLane;
}

export interface En1991BridgeLaneRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991BridgeLaneModification {
  id: string;
  patch: En1991BridgeLanePatch;
}

export interface En1991BridgeLaneDelta {
  removed: En1991BridgeLaneRemoval[];
  inserted: En1991BridgeLaneInsertion[];
  moved: En1991BridgeLaneRelocation[];
  modified: En1991BridgeLaneModification[];
}

export interface En1991ConstructionStagePatch {
  activity?: string | null;
  durationDays?: number | null;
  returnYears?: number | null;
  assumedQk?: number | null;
}

export interface En1991ConstructionStageRemoval {
  id: string;
  index: number;
}

export interface En1991ConstructionStageInsertion {
  index: number;
  row: ConstructionStage;
}

export interface En1991ConstructionStageRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991ConstructionStageModification {
  id: string;
  patch: En1991ConstructionStagePatch;
}

export interface En1991ConstructionStageDelta {
  removed: En1991ConstructionStageRemoval[];
  inserted: En1991ConstructionStageInsertion[];
  moved: En1991ConstructionStageRelocation[];
  modified: En1991ConstructionStageModification[];
}

export interface En1991RoofSurfacePatch {
  face?: string | null;
  shape?: string | null;
  pitchDeg?: number | null;
  extent?: number | null;
}

export interface En1991RoofSurfaceRemoval {
  id: string;
  index: number;
}

export interface En1991RoofSurfaceInsertion {
  index: number;
  row: RoofSurface;
}

export interface En1991RoofSurfaceRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991RoofSurfaceModification {
  id: string;
  patch: En1991RoofSurfacePatch;
}

export interface En1991RoofSurfaceDelta {
  removed: En1991RoofSurfaceRemoval[];
  inserted: En1991RoofSurfaceInsertion[];
  moved: En1991RoofSurfaceRelocation[];
  modified: En1991RoofSurfaceModification[];
}

export interface En1991PlacementPatch {
  x?: number | null;
  y?: number | null;
  z?: number | null;
  yaw?: number | null;
}

export interface En1991PlacementRemoval {
  id: string;
  index: number;
}

export interface En1991PlacementInsertion {
  index: number;
  row: Placement;
}

export interface En1991PlacementRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991PlacementModification {
  id: string;
  patch: En1991PlacementPatch;
}

export interface En1991PlacementDelta {
  removed: En1991PlacementRemoval[];
  inserted: En1991PlacementInsertion[];
  moved: En1991PlacementRelocation[];
  modified: En1991PlacementModification[];
}

export interface En1991SpecialVehiclePatch {
  designation?: string | null;
  x?: number | null;
  y?: number | null;
}

export interface En1991SpecialVehicleRemoval {
  id: string;
  index: number;
}

export interface En1991SpecialVehicleInsertion {
  index: number;
  row: SpecialVehicle;
}

export interface En1991SpecialVehicleRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991SpecialVehicleModification {
  id: string;
  patch: En1991SpecialVehiclePatch;
}

export interface En1991SpecialVehicleDelta {
  removed: En1991SpecialVehicleRemoval[];
  inserted: En1991SpecialVehicleInsertion[];
  moved: En1991SpecialVehicleRelocation[];
  modified: En1991SpecialVehicleModification[];
}

export interface En1991WindBridgePatch {
  deckWidth?: number | null;
  deckDepth?: number | null;
  parapetKind?: number | null;
  parapetHeight?: number | null;
  trafficKind?: number | null;
  heightAboveGround?: number | null;
  span?: number | null;
  assumedForce?: number | null;
}

export interface En1991WindBridgeRemoval {
  id: string;
  index: number;
}

export interface En1991WindBridgeInsertion {
  index: number;
  row: WindBridge;
}

export interface En1991WindBridgeRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991WindBridgeModification {
  id: string;
  patch: En1991WindBridgePatch;
}

export interface En1991WindBridgeDelta {
  removed: En1991WindBridgeRemoval[];
  inserted: En1991WindBridgeInsertion[];
  moved: En1991WindBridgeRelocation[];
  modified: En1991WindBridgeModification[];
}

export interface En1991DynamicElementPatch {
  height?: number | null;
  width?: number | null;
  firstFrequency?: number | null;
  logDecrement?: number | null;
  massPerLength?: number | null;
  strouhal?: number | null;
  assumedCscd?: number | null;
}

export interface En1991DynamicElementRemoval {
  id: string;
  index: number;
}

export interface En1991DynamicElementInsertion {
  index: number;
  row: DynamicElement;
}

export interface En1991DynamicElementRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991DynamicElementModification {
  id: string;
  patch: En1991DynamicElementPatch;
}

export interface En1991DynamicElementDelta {
  removed: En1991DynamicElementRemoval[];
  inserted: En1991DynamicElementInsertion[];
  moved: En1991DynamicElementRelocation[];
  modified: En1991DynamicElementModification[];
}

export interface En1991ImpactActionPatch {
  kind?: number | null;
  mass?: number | null;
  speed?: number | null;
  stiffness?: number | null;
  height?: number | null;
  x?: number | null;
  y?: number | null;
  assumedForce?: number | null;
}

export interface En1991ImpactActionRemoval {
  id: string;
  index: number;
}

export interface En1991ImpactActionInsertion {
  index: number;
  row: ImpactAction;
}

export interface En1991ImpactActionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991ImpactActionModification {
  id: string;
  patch: En1991ImpactActionPatch;
}

export interface En1991ImpactActionDelta {
  removed: En1991ImpactActionRemoval[];
  inserted: En1991ImpactActionInsertion[];
  moved: En1991ImpactActionRelocation[];
  modified: En1991ImpactActionModification[];
}

export interface En1991InternalExplosionPatch {
  volume?: number | null;
  ventArea?: number | null;
  ventPressure?: number | null;
  assumedPressure?: number | null;
}

export interface En1991InternalExplosionRemoval {
  id: string;
  index: number;
}

export interface En1991InternalExplosionInsertion {
  index: number;
  row: InternalExplosion;
}

export interface En1991InternalExplosionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991InternalExplosionModification {
  id: string;
  patch: En1991InternalExplosionPatch;
}

export interface En1991InternalExplosionDelta {
  removed: En1991InternalExplosionRemoval[];
  inserted: En1991InternalExplosionInsertion[];
  moved: En1991InternalExplosionRelocation[];
  modified: En1991InternalExplosionModification[];
}

export interface En1991ThermalSectionPatch {
  deckType?: number | null;
  surfacing?: number | null;
  length?: number | null;
  depth?: number | null;
  alphaMicro?: number | null;
  assumedRange?: number | null;
}

export interface En1991ThermalSectionRemoval {
  id: string;
  index: number;
}

export interface En1991ThermalSectionInsertion {
  index: number;
  row: ThermalSection;
}

export interface En1991ThermalSectionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991ThermalSectionModification {
  id: string;
  patch: En1991ThermalSectionPatch;
}

export interface En1991ThermalSectionDelta {
  removed: En1991ThermalSectionRemoval[];
  inserted: En1991ThermalSectionInsertion[];
  moved: En1991ThermalSectionRelocation[];
  modified: En1991ThermalSectionModification[];
}

export interface CaseNumberPatch {
  value?: number | null;
}

export interface CaseNumberRemoval {
  id: string;
  index: number;
}

export interface CaseNumberInsertion {
  index: number;
  row: CaseNumber;
}

export interface CaseNumberRelocation {
  id: string;
  from: number;
  to: number;
}

export interface CaseNumberModification {
  id: string;
  patch: CaseNumberPatch;
}

export interface CaseNumberDelta {
  removed: CaseNumberRemoval[];
  inserted: CaseNumberInsertion[];
  moved: CaseNumberRelocation[];
  modified: CaseNumberModification[];
}

export interface CaseTextPatch {
  value?: string | null;
}

export interface CaseTextRemoval {
  id: string;
  index: number;
}

export interface CaseTextInsertion {
  index: number;
  row: CaseText;
}

export interface CaseTextRelocation {
  id: string;
  from: number;
  to: number;
}

export interface CaseTextModification {
  id: string;
  patch: CaseTextPatch;
}

export interface CaseTextDelta {
  removed: CaseTextRemoval[];
  inserted: CaseTextInsertion[];
  moved: CaseTextRelocation[];
  modified: CaseTextModification[];
}

export interface CaseRowPatch {
  labelEn?: string | null;
  labelDe?: string | null;
  entryId?: string | null;
  numbers: CaseNumberDelta;
  texts: CaseTextDelta;
}

export interface CaseRowRemoval {
  id: string;
  index: number;
}

export interface CaseRowInsertion {
  index: number;
  row: CaseRow;
}

export interface CaseRowRelocation {
  id: string;
  from: number;
  to: number;
}

export interface CaseRowModification {
  id: string;
  patch: CaseRowPatch;
}

export interface CaseRowDelta {
  removed: CaseRowRemoval[];
  inserted: CaseRowInsertion[];
  moved: CaseRowRelocation[];
  modified: CaseRowModification[];
}

export interface En1991HelicopterLandingPatch {
  mass?: number | null;
  x?: number | null;
  y?: number | null;
  height?: number | null;
  assumedForce?: number | null;
}

export interface En1991HelicopterLandingRemoval {
  id: string;
  index: number;
}

export interface En1991HelicopterLandingInsertion {
  index: number;
  row: HelicopterLanding;
}

export interface En1991HelicopterLandingRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991HelicopterLandingModification {
  id: string;
  patch: En1991HelicopterLandingPatch;
}

export interface En1991HelicopterLandingDelta {
  removed: En1991HelicopterLandingRemoval[];
  inserted: En1991HelicopterLandingInsertion[];
  moved: En1991HelicopterLandingRelocation[];
  modified: En1991HelicopterLandingModification[];
}

export interface En1991TieRunPatch {
  kind?: number | null;
  spacing?: number | null;
  span?: number | null;
  deadLoad?: number | null;
  imposedLoad?: number | null;
  psi?: number | null;
  x?: number | null;
  y?: number | null;
  providedForce?: number | null;
}

export interface En1991TieRunRemoval {
  id: string;
  index: number;
}

export interface En1991TieRunInsertion {
  index: number;
  row: TieRun;
}

export interface En1991TieRunRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991TieRunModification {
  id: string;
  patch: En1991TieRunPatch;
}

export interface En1991TieRunDelta {
  removed: En1991TieRunRemoval[];
  inserted: En1991TieRunInsertion[];
  moved: En1991TieRunRelocation[];
  modified: En1991TieRunModification[];
}

export interface En1991ConstructionLoadPatch {
  stage?: string | null;
  kind?: number | null;
  magnitude?: number | null;
}

export interface En1991ConstructionLoadRemoval {
  id: string;
  index: number;
}

export interface En1991ConstructionLoadInsertion {
  index: number;
  row: ConstructionLoad;
}

export interface En1991ConstructionLoadRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991ConstructionLoadModification {
  id: string;
  patch: En1991ConstructionLoadPatch;
}

export interface En1991ConstructionLoadDelta {
  removed: En1991ConstructionLoadRemoval[];
  inserted: En1991ConstructionLoadInsertion[];
  moved: En1991ConstructionLoadRelocation[];
  modified: En1991ConstructionLoadModification[];
}

export interface En1991FallingLoadPatch {
  mass?: number | null;
  dropHeight?: number | null;
  stoppingDistance?: number | null;
  x?: number | null;
  y?: number | null;
  assumedForce?: number | null;
}

export interface En1991FallingLoadRemoval {
  id: string;
  index: number;
}

export interface En1991FallingLoadInsertion {
  index: number;
  row: FallingLoad;
}

export interface En1991FallingLoadRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991FallingLoadModification {
  id: string;
  patch: En1991FallingLoadPatch;
}

export interface En1991FallingLoadDelta {
  removed: En1991FallingLoadRemoval[];
  inserted: En1991FallingLoadInsertion[];
  moved: En1991FallingLoadRelocation[];
  modified: En1991FallingLoadModification[];
}

export interface En1991ExecutionImperfectionPatch {
  height?: number | null;
  tolerance?: number | null;
  verticalLoad?: number | null;
  x?: number | null;
  y?: number | null;
  assumedForce?: number | null;
}

export interface En1991ExecutionImperfectionRemoval {
  id: string;
  index: number;
}

export interface En1991ExecutionImperfectionInsertion {
  index: number;
  row: ExecutionImperfection;
}

export interface En1991ExecutionImperfectionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991ExecutionImperfectionModification {
  id: string;
  patch: En1991ExecutionImperfectionPatch;
}

export interface En1991ExecutionImperfectionDelta {
  removed: En1991ExecutionImperfectionRemoval[];
  inserted: En1991ExecutionImperfectionInsertion[];
  moved: En1991ExecutionImperfectionRelocation[];
  modified: En1991ExecutionImperfectionModification[];
}

export interface En1991DerailmentCasePatch {
  distance?: number | null;
  designForceX?: number | null;
  designForceY?: number | null;
  x?: number | null;
  y?: number | null;
  assumedForce?: number | null;
}

export interface En1991DerailmentCaseRemoval {
  id: string;
  index: number;
}

export interface En1991DerailmentCaseInsertion {
  index: number;
  row: DerailmentCase;
}

export interface En1991DerailmentCaseRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991DerailmentCaseModification {
  id: string;
  patch: En1991DerailmentCasePatch;
}

export interface En1991DerailmentCaseDelta {
  removed: En1991DerailmentCaseRemoval[];
  inserted: En1991DerailmentCaseInsertion[];
  moved: En1991DerailmentCaseRelocation[];
  modified: En1991DerailmentCaseModification[];
}

export interface En1991CraneHoistPatch {
  ratedLoad?: number | null;
  releaseFraction?: number | null;
  releaseDevice?: string | null;
}

export interface En1991CraneHoistRemoval {
  id: string;
  index: number;
}

export interface En1991CraneHoistInsertion {
  index: number;
  row: CraneHoist;
}

export interface En1991CraneHoistRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991CraneHoistModification {
  id: string;
  patch: En1991CraneHoistPatch;
}

export interface En1991CraneHoistDelta {
  removed: En1991CraneHoistRemoval[];
  inserted: En1991CraneHoistInsertion[];
  moved: En1991CraneHoistRelocation[];
  modified: En1991CraneHoistModification[];
}

export interface En1991FootbridgePatch {
  loadedLength?: number | null;
  deckWidth?: number | null;
  naturalFrequency?: number | null;
}

export interface En1991FootbridgeRemoval {
  id: string;
  index: number;
}

export interface En1991FootbridgeInsertion {
  index: number;
  row: Footbridge;
}

export interface En1991FootbridgeRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991FootbridgeModification {
  id: string;
  patch: En1991FootbridgePatch;
}

export interface En1991FootbridgeDelta {
  removed: En1991FootbridgeRemoval[];
  inserted: En1991FootbridgeInsertion[];
  moved: En1991FootbridgeRelocation[];
  modified: En1991FootbridgeModification[];
}

export interface En1991RailDynamicPatch {
  determinantLength?: number | null;
  maintenance?: string | null;
}

export interface En1991RailDynamicRemoval {
  id: string;
  index: number;
}

export interface En1991RailDynamicInsertion {
  index: number;
  row: RailDynamic;
}

export interface En1991RailDynamicRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991RailDynamicModification {
  id: string;
  patch: En1991RailDynamicPatch;
}

export interface En1991RailDynamicDelta {
  removed: En1991RailDynamicRemoval[];
  inserted: En1991RailDynamicInsertion[];
  moved: En1991RailDynamicRelocation[];
  modified: En1991RailDynamicModification[];
}

export interface En1991LoadDispersalPatch {
  wheelLength?: number | null;
  wheelWidth?: number | null;
  pavementThickness?: number | null;
  slabThickness?: number | null;
}

export interface En1991LoadDispersalRemoval {
  id: string;
  index: number;
}

export interface En1991LoadDispersalInsertion {
  index: number;
  row: LoadDispersal;
}

export interface En1991LoadDispersalRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991LoadDispersalModification {
  id: string;
  patch: En1991LoadDispersalPatch;
}

export interface En1991LoadDispersalDelta {
  removed: En1991LoadDispersalRemoval[];
  inserted: En1991LoadDispersalInsertion[];
  moved: En1991LoadDispersalRelocation[];
  modified: En1991LoadDispersalModification[];
}

export interface En1991BulkSolidPatch {
  internalFriction?: number | null;
}

export interface En1991BulkSolidRemoval {
  id: string;
  index: number;
}

export interface En1991BulkSolidInsertion {
  index: number;
  row: BulkSolid;
}

export interface En1991BulkSolidRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991BulkSolidModification {
  id: string;
  patch: En1991BulkSolidPatch;
}

export interface En1991BulkSolidDelta {
  removed: En1991BulkSolidRemoval[];
  inserted: En1991BulkSolidInsertion[];
  moved: En1991BulkSolidRelocation[];
  modified: En1991BulkSolidModification[];
}

export interface En1991RotatingMachinePatch {
  mass?: number | null;
  eccentricity?: number | null;
  frequency?: number | null;
}

export interface En1991RotatingMachineRemoval {
  id: string;
  index: number;
}

export interface En1991RotatingMachineInsertion {
  index: number;
  row: RotatingMachine;
}

export interface En1991RotatingMachineRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991RotatingMachineModification {
  id: string;
  patch: En1991RotatingMachinePatch;
}

export interface En1991RotatingMachineDelta {
  removed: En1991RotatingMachineRemoval[];
  inserted: En1991RotatingMachineInsertion[];
  moved: En1991RotatingMachineRelocation[];
  modified: En1991RotatingMachineModification[];
}

export interface En1991FatigueModelRemoval {
  id: string;
  index: number;
}

export interface En1991FatigueModelInsertion {
  index: number;
  row: FatigueModel;
}

export interface En1991FatigueModelRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991FatigueModelModification {
  id: string;
  patch: En1991FatigueModelPatch;
}

export interface En1991FatigueModelDelta {
  removed: En1991FatigueModelRemoval[];
  inserted: En1991FatigueModelInsertion[];
  moved: En1991FatigueModelRelocation[];
  modified: En1991FatigueModelModification[];
}

export interface En1991FatigueModelPatch {
  model?: string | null;
  lane?: number | null;
  assumedAxleLoad?: number | null;
  assumedUdl?: number | null;
}

export interface En1991SupportCollisionRemoval {
  id: string;
  index: number;
}

export interface En1991SupportCollisionInsertion {
  index: number;
  row: SupportCollision;
}

export interface En1991SupportCollisionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991SupportCollisionModification {
  id: string;
  patch: En1991SupportCollisionPatch;
}

export interface En1991SupportCollisionDelta {
  removed: En1991SupportCollisionRemoval[];
  inserted: En1991SupportCollisionInsertion[];
  moved: En1991SupportCollisionRelocation[];
  modified: En1991SupportCollisionModification[];
}

export interface En1991SupportCollisionPatch {
  location?: string | null;
  assumedForceX?: number | null;
  assumedForceY?: number | null;
  height?: number | null;
  positionX?: number | null;
}

export interface En1991LocalFireRemoval {
  id: string;
  index: number;
}

export interface En1991LocalFireInsertion {
  index: number;
  row: LocalFire;
}

export interface En1991LocalFireRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991LocalFireModification {
  id: string;
  patch: En1991LocalFirePatch;
}

export interface En1991LocalFireDelta {
  removed: En1991LocalFireRemoval[];
  inserted: En1991LocalFireInsertion[];
  moved: En1991LocalFireRelocation[];
  modified: En1991LocalFireModification[];
}

export interface En1991LocalFirePatch {
  fireDiameter?: number | null;
  heatReleaseRate?: number | null;
  ceilingHeight?: number | null;
  horizontalDistance?: number | null;
  assumedFlux?: number | null;
}

export interface En1991SnowGuardRemoval {
  id: string;
  index: number;
}

export interface En1991SnowGuardInsertion {
  index: number;
  row: SnowGuard;
}

export interface En1991SnowGuardRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1991SnowGuardModification {
  id: string;
  patch: En1991SnowGuardPatch;
}

export interface En1991SnowGuardDelta {
  removed: En1991SnowGuardRemoval[];
  inserted: En1991SnowGuardInsertion[];
  moved: En1991SnowGuardRelocation[];
  modified: En1991SnowGuardModification[];
}

export interface En1991SnowGuardPatch {
  roofId?: string | null;
  slopeLength?: number | null;
  assumedForce?: number | null;
}

export const parseEn1991Diff: NormWireReader<En1991Diff> = normWireObject<En1991Diff>({ annex: normWireOptional(normWireNullable(normWireLiteral("En", "De"))), snowZone: normWireOptional(normWireNullable(normWireString)), altitude: normWireOptional(normWireNullable(normWireNumber)), enSk: normWireOptional(normWireNullable(normWireNumber)), northGermanLowlandSnow: normWireOptional(normWireNullable(normWireBoolean)), windZone: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), enVb: normWireOptional(normWireNullable(normWireNumber)), terrainCategory: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), mixedTerrainUpwind: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), mixedTerrainDistance: normWireOptional(normWireNullable(normWireNumber)), orographyFactor: normWireOptional(normWireNullable(normWireNumber)), coastOrIsland: normWireOptional(normWireNullable(normWireBoolean)), airDensity: normWireOptional(normWireNullable(normWireNumber)), height: normWireOptional(normWireNullable(normWireNumber)), width: normWireOptional(normWireNullable(normWireNumber)), depth: normWireOptional(normWireNullable(normWireNumber)), assumedDeltaT: normWireOptional(normWireNullable(normWireNumber)), tMax: normWireOptional(normWireNullable(normWireNumber)), tMin: normWireOptional(normWireNullable(normWireNumber)), t0: normWireOptional(normWireNullable(normWireNumber)), thermalElementType: normWireOptional(normWireNullable(normWireString)), thermalBridgeType: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), deltaTM: normWireOptional(normWireNullable(normWireNumber)), storeyCount: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), fireMode: normWireOptional(normWireNullable(normWireLiteral("none", "nominal", "parametric"))), fireCurve: normWireOptional(normWireNullable(normWireLiteral("standard", "external", "hydrocarbon", "parametric"))), fireDuration: normWireOptional(normWireNullable(normWireNumber)), assumedGasTemperature: normWireOptional(normWireNullable(normWireNumber)), assumedHNet: normWireOptional(normWireNullable(normWireNumber)), fireCompartmentArea: normWireOptional(normWireNullable(normWireNumber)), fireCompartmentHeight: normWireOptional(normWireNullable(normWireNumber)), fireOpeningFactor: normWireOptional(normWireNullable(normWireNumber)), fireThermalInertia: normWireOptional(normWireNullable(normWireNumber)), fireOccupancy: normWireOptional(normWireNullable(parseEn1991FireOccupancy)), fireLoadDensityQf: normWireOptional(normWireNullable(normWireNumber)), assumedQfD: normWireOptional(normWireNullable(normWireNumber)), constructionActivity: normWireOptional(normWireNullable(normWireString)), assumedConstructionQk: normWireOptional(normWireNullable(normWireNumber)), structureKind: normWireOptional(normWireNullable(normWireLiteral("building", "bridge"))), bridgeLane: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), bridgeSpan: normWireOptional(normWireNullable(normWireNumber)), bridgeLaneWidth: normWireOptional(normWireNullable(normWireNumber)), assumedBridgeTandem: normWireOptional(normWireNullable(normWireNumber)), assumedBridgeUdl: normWireOptional(normWireNullable(normWireNumber)), assumedBridgeLm2: normWireOptional(normWireNullable(normWireNumber)), assumedBridgeFootway: normWireOptional(normWireNullable(normWireNumber)), assumedBridgeLm3: normWireOptional(normWireNullable(normWireNumber)), assumedBridgeLm4: normWireOptional(normWireNullable(normWireNumber)), bridgeLoadGroup: normWireOptional(normWireNullable(normWireString)), craneClaimed: normWireOptional(normWireNullable(normWireBoolean)), craneClass: normWireOptional(normWireNullable(normWireString)), hoistClass: normWireOptional(normWireNullable(normWireString)), hoistingSpeed: normWireOptional(normWireNullable(normWireNumber)), assumedCraneWheel: normWireOptional(normWireNullable(normWireNumber)), assumedCraneHorizontal: normWireOptional(normWireNullable(normWireNumber)), siloClaimed: normWireOptional(normWireNullable(normWireBoolean)), siloKind: normWireOptional(normWireNullable(normWireString)), siloBulkDensity: normWireOptional(normWireNullable(normWireNumber)), siloHeight: normWireOptional(normWireNullable(normWireNumber)), siloHydraulicRadius: normWireOptional(normWireNullable(normWireNumber)), siloMu: normWireOptional(normWireNullable(normWireNumber)), siloK: normWireOptional(normWireNullable(normWireNumber)), assumedSiloPressure: normWireOptional(normWireNullable(normWireNumber)), assumedSiloPatch: normWireOptional(normWireNullable(normWireNumber)), assumedSiloWallFriction: normWireOptional(normWireNullable(normWireNumber)), bridgeTandemX: normWireOptional(normWireNullable(normWireNumber)), railAlpha: normWireOptional(normWireNullable(normWireNumber)), assumedRailLm71: normWireOptional(normWireNullable(normWireNumber)), assumedRailLm71Udl: normWireOptional(normWireNullable(normWireNumber)), assumedRailSw0: normWireOptional(normWireNullable(normWireNumber)), assumedRailSw2: normWireOptional(normWireNullable(normWireNumber)), craneX: normWireOptional(normWireNullable(normWireNumber)), craneRunwayLength: normWireOptional(normWireNullable(normWireNumber)), craneWheelBase: normWireOptional(normWireNullable(normWireNumber)), siloDiameter: normWireOptional(normWireNullable(normWireNumber)), siloEccentricity: normWireOptional(normWireNullable(normWireNumber)), siloHopperAngleDeg: normWireOptional(normWireNullable(normWireNumber)), bridgeCurveRadius: normWireOptional(normWireNullable(normWireNumber)), fireLoadBasis: normWireOptional(normWireNullable(normWireLiteral("occupancyTable", "userSpecified"))), floors: normWireRequired(normWireRef(() => parseEn1991FloorDelta)), selfWeightElements: normWireRequired(normWireRef(() => parseEn1991SelfWeightElementDelta)), roofs: normWireRequired(normWireRef(() => parseEn1991RoofDelta)), windFaces: normWireRequired(normWireRef(() => parseEn1991WindFaceDelta)), accidentalCases: normWireRequired(normWireRef(() => parseEn1991AccidentalCaseDelta)), bridgeLanes: normWireRequired(normWireRef(() => parseEn1991BridgeLaneDelta)), constructionStages: normWireRequired(normWireRef(() => parseEn1991ConstructionStageDelta)), roofSurfaces: normWireRequired(normWireRef(() => parseEn1991RoofSurfaceDelta)), placements: normWireRequired(normWireRef(() => parseEn1991PlacementDelta)), specialVehicles: normWireRequired(normWireRef(() => parseEn1991SpecialVehicleDelta)), windBridges: normWireRequired(normWireRef(() => parseEn1991WindBridgeDelta)), dynamicElements: normWireRequired(normWireRef(() => parseEn1991DynamicElementDelta)), impactActions: normWireRequired(normWireRef(() => parseEn1991ImpactActionDelta)), internalExplosions: normWireRequired(normWireRef(() => parseEn1991InternalExplosionDelta)), thermalSections: normWireRequired(normWireRef(() => parseEn1991ThermalSectionDelta)), cases: normWireRequired(normWireRef(() => parseCaseRowDelta)), helicopterLandings: normWireRequired(normWireRef(() => parseEn1991HelicopterLandingDelta)), tieRuns: normWireRequired(normWireRef(() => parseEn1991TieRunDelta)), constructionLoads: normWireRequired(normWireRef(() => parseEn1991ConstructionLoadDelta)), fallingLoads: normWireRequired(normWireRef(() => parseEn1991FallingLoadDelta)), executionImperfections: normWireRequired(normWireRef(() => parseEn1991ExecutionImperfectionDelta)), derailmentCases: normWireRequired(normWireRef(() => parseEn1991DerailmentCaseDelta)), craneHoists: normWireRequired(normWireRef(() => parseEn1991CraneHoistDelta)), footbridges: normWireRequired(normWireRef(() => parseEn1991FootbridgeDelta)), railDynamics: normWireRequired(normWireRef(() => parseEn1991RailDynamicDelta)), loadDispersals: normWireRequired(normWireRef(() => parseEn1991LoadDispersalDelta)), bulkSolids: normWireRequired(normWireRef(() => parseEn1991BulkSolidDelta)), rotatingMachines: normWireRequired(normWireRef(() => parseEn1991RotatingMachineDelta)), fatigueModels: normWireOptional(normWireRef(() => parseEn1991FatigueModelDelta)), supportCollisions: normWireOptional(normWireRef(() => parseEn1991SupportCollisionDelta)), localFires: normWireOptional(normWireRef(() => parseEn1991LocalFireDelta)), snowGuards: normWireOptional(normWireRef(() => parseEn1991SnowGuardDelta)) });
export const parseEn1991FloorPatch: NormWireReader<En1991FloorPatch> = normWireObject<En1991FloorPatch>({ category: normWireOptional(normWireNullable(normWireString)), area: normWireOptional(normWireNullable(normWireNumber)), assumedQk: normWireOptional(normWireNullable(normWireNumber)), assumedQkConcentrated: normWireOptional(normWireNullable(normWireNumber)), assumedPartitions: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991FloorRemoval: NormWireReader<En1991FloorRemoval> = normWireObject<En1991FloorRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991FloorInsertion: NormWireReader<En1991FloorInsertion> = normWireObject<En1991FloorInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseFloorArea) });
export const parseEn1991FloorRelocation: NormWireReader<En1991FloorRelocation> = normWireObject<En1991FloorRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991FloorModification: NormWireReader<En1991FloorModification> = normWireObject<En1991FloorModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991FloorPatch)) });
export const parseEn1991FloorDelta: NormWireReader<En1991FloorDelta> = normWireObject<En1991FloorDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991FloorRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991FloorInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991FloorRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991FloorModification))) });
export const parseEn1991SelfWeightElementPatch: NormWireReader<En1991SelfWeightElementPatch> = normWireObject<En1991SelfWeightElementPatch>({ material: normWireOptional(normWireNullable(normWireString)), thickness: normWireOptional(normWireNullable(normWireNumber)), assumedGk: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991SelfWeightElementRemoval: NormWireReader<En1991SelfWeightElementRemoval> = normWireObject<En1991SelfWeightElementRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991SelfWeightElementInsertion: NormWireReader<En1991SelfWeightElementInsertion> = normWireObject<En1991SelfWeightElementInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseSelfWeightElement) });
export const parseEn1991SelfWeightElementRelocation: NormWireReader<En1991SelfWeightElementRelocation> = normWireObject<En1991SelfWeightElementRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991SelfWeightElementModification: NormWireReader<En1991SelfWeightElementModification> = normWireObject<En1991SelfWeightElementModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991SelfWeightElementPatch)) });
export const parseEn1991SelfWeightElementDelta: NormWireReader<En1991SelfWeightElementDelta> = normWireObject<En1991SelfWeightElementDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991SelfWeightElementRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991SelfWeightElementInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991SelfWeightElementRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991SelfWeightElementModification))) });
export const parseEn1991RoofPatch: NormWireReader<En1991RoofPatch> = normWireObject<En1991RoofPatch>({ roofType: normWireOptional(normWireNullable(normWireString)), pitchDeg: normWireOptional(normWireNullable(normWireNumber)), cE: normWireOptional(normWireNullable(normWireNumber)), cT: normWireOptional(normWireNullable(normWireNumber)), hasParapet: normWireOptional(normWireNullable(normWireBoolean)), parapetHeight: normWireOptional(normWireNullable(normWireNumber)), driftObstructionHeight: normWireOptional(normWireNullable(normWireNumber)), multiSpan: normWireOptional(normWireNullable(normWireBoolean)), assumedSk: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991RoofRemoval: NormWireReader<En1991RoofRemoval> = normWireObject<En1991RoofRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991RoofInsertion: NormWireReader<En1991RoofInsertion> = normWireObject<En1991RoofInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseRoofArea) });
export const parseEn1991RoofRelocation: NormWireReader<En1991RoofRelocation> = normWireObject<En1991RoofRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991RoofModification: NormWireReader<En1991RoofModification> = normWireObject<En1991RoofModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991RoofPatch)) });
export const parseEn1991RoofDelta: NormWireReader<En1991RoofDelta> = normWireObject<En1991RoofDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991RoofRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991RoofInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991RoofRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991RoofModification))) });
export const parseEn1991WindFacePatch: NormWireReader<En1991WindFacePatch> = normWireObject<En1991WindFacePatch>({ zone: normWireOptional(normWireNullable(normWireString)), z: normWireOptional(normWireNullable(normWireNumber)), cPe10: normWireOptional(normWireNullable(normWireNumber)), cPe1: normWireOptional(normWireNullable(normWireNumber)), cPi: normWireOptional(normWireNullable(normWireNumber)), cS: normWireOptional(normWireNullable(normWireNumber)), cD: normWireOptional(normWireNullable(normWireNumber)), loadedArea: normWireOptional(normWireNullable(normWireNumber)), assumedWp: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991WindFaceRemoval: NormWireReader<En1991WindFaceRemoval> = normWireObject<En1991WindFaceRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991WindFaceInsertion: NormWireReader<En1991WindFaceInsertion> = normWireObject<En1991WindFaceInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseWindFace) });
export const parseEn1991WindFaceRelocation: NormWireReader<En1991WindFaceRelocation> = normWireObject<En1991WindFaceRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991WindFaceModification: NormWireReader<En1991WindFaceModification> = normWireObject<En1991WindFaceModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991WindFacePatch)) });
export const parseEn1991WindFaceDelta: NormWireReader<En1991WindFaceDelta> = normWireObject<En1991WindFaceDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991WindFaceRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991WindFaceInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991WindFaceRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991WindFaceModification))) });
export const parseEn1991AccidentalCasePatch: NormWireReader<En1991AccidentalCasePatch> = normWireObject<En1991AccidentalCasePatch>({ impact: normWireOptional(normWireNullable(normWireArray(parseAccidentalImpact))), explosion: normWireOptional(normWireNullable(normWireArray(parseAccidentalExplosion))) });
export const parseEn1991AccidentalCaseRemoval: NormWireReader<En1991AccidentalCaseRemoval> = normWireObject<En1991AccidentalCaseRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991AccidentalCaseInsertion: NormWireReader<En1991AccidentalCaseInsertion> = normWireObject<En1991AccidentalCaseInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseAccidentalCase) });
export const parseEn1991AccidentalCaseRelocation: NormWireReader<En1991AccidentalCaseRelocation> = normWireObject<En1991AccidentalCaseRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991AccidentalCaseModification: NormWireReader<En1991AccidentalCaseModification> = normWireObject<En1991AccidentalCaseModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991AccidentalCasePatch)) });
export const parseEn1991AccidentalCaseDelta: NormWireReader<En1991AccidentalCaseDelta> = normWireObject<En1991AccidentalCaseDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991AccidentalCaseRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991AccidentalCaseInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991AccidentalCaseRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991AccidentalCaseModification))) });
export const parseEn1991BridgeLanePatch: NormWireReader<En1991BridgeLanePatch> = normWireObject<En1991BridgeLanePatch>({ width: normWireOptional(normWireNullable(normWireNumber)), offset: normWireOptional(normWireNullable(normWireNumber)), lm1Lane: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))) });
export const parseEn1991BridgeLaneRemoval: NormWireReader<En1991BridgeLaneRemoval> = normWireObject<En1991BridgeLaneRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991BridgeLaneInsertion: NormWireReader<En1991BridgeLaneInsertion> = normWireObject<En1991BridgeLaneInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseBridgeLane) });
export const parseEn1991BridgeLaneRelocation: NormWireReader<En1991BridgeLaneRelocation> = normWireObject<En1991BridgeLaneRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991BridgeLaneModification: NormWireReader<En1991BridgeLaneModification> = normWireObject<En1991BridgeLaneModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991BridgeLanePatch)) });
export const parseEn1991BridgeLaneDelta: NormWireReader<En1991BridgeLaneDelta> = normWireObject<En1991BridgeLaneDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991BridgeLaneRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991BridgeLaneInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991BridgeLaneRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991BridgeLaneModification))) });
export const parseEn1991ConstructionStagePatch: NormWireReader<En1991ConstructionStagePatch> = normWireObject<En1991ConstructionStagePatch>({ activity: normWireOptional(normWireNullable(normWireString)), durationDays: normWireOptional(normWireNullable(normWireNumber)), returnYears: normWireOptional(normWireNullable(normWireNumber)), assumedQk: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991ConstructionStageRemoval: NormWireReader<En1991ConstructionStageRemoval> = normWireObject<En1991ConstructionStageRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991ConstructionStageInsertion: NormWireReader<En1991ConstructionStageInsertion> = normWireObject<En1991ConstructionStageInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseConstructionStage) });
export const parseEn1991ConstructionStageRelocation: NormWireReader<En1991ConstructionStageRelocation> = normWireObject<En1991ConstructionStageRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991ConstructionStageModification: NormWireReader<En1991ConstructionStageModification> = normWireObject<En1991ConstructionStageModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991ConstructionStagePatch)) });
export const parseEn1991ConstructionStageDelta: NormWireReader<En1991ConstructionStageDelta> = normWireObject<En1991ConstructionStageDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991ConstructionStageRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991ConstructionStageInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991ConstructionStageRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991ConstructionStageModification))) });
export const parseEn1991RoofSurfacePatch: NormWireReader<En1991RoofSurfacePatch> = normWireObject<En1991RoofSurfacePatch>({ face: normWireOptional(normWireNullable(normWireString)), shape: normWireOptional(normWireNullable(normWireString)), pitchDeg: normWireOptional(normWireNullable(normWireNumber)), extent: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991RoofSurfaceRemoval: NormWireReader<En1991RoofSurfaceRemoval> = normWireObject<En1991RoofSurfaceRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991RoofSurfaceInsertion: NormWireReader<En1991RoofSurfaceInsertion> = normWireObject<En1991RoofSurfaceInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseRoofSurface) });
export const parseEn1991RoofSurfaceRelocation: NormWireReader<En1991RoofSurfaceRelocation> = normWireObject<En1991RoofSurfaceRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991RoofSurfaceModification: NormWireReader<En1991RoofSurfaceModification> = normWireObject<En1991RoofSurfaceModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991RoofSurfacePatch)) });
export const parseEn1991RoofSurfaceDelta: NormWireReader<En1991RoofSurfaceDelta> = normWireObject<En1991RoofSurfaceDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991RoofSurfaceRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991RoofSurfaceInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991RoofSurfaceRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991RoofSurfaceModification))) });
export const parseEn1991PlacementPatch: NormWireReader<En1991PlacementPatch> = normWireObject<En1991PlacementPatch>({ x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), z: normWireOptional(normWireNullable(normWireNumber)), yaw: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991PlacementRemoval: NormWireReader<En1991PlacementRemoval> = normWireObject<En1991PlacementRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991PlacementInsertion: NormWireReader<En1991PlacementInsertion> = normWireObject<En1991PlacementInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parsePlacement) });
export const parseEn1991PlacementRelocation: NormWireReader<En1991PlacementRelocation> = normWireObject<En1991PlacementRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991PlacementModification: NormWireReader<En1991PlacementModification> = normWireObject<En1991PlacementModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991PlacementPatch)) });
export const parseEn1991PlacementDelta: NormWireReader<En1991PlacementDelta> = normWireObject<En1991PlacementDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991PlacementRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991PlacementInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991PlacementRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991PlacementModification))) });
export const parseEn1991SpecialVehiclePatch: NormWireReader<En1991SpecialVehiclePatch> = normWireObject<En1991SpecialVehiclePatch>({ designation: normWireOptional(normWireNullable(normWireString)), x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991SpecialVehicleRemoval: NormWireReader<En1991SpecialVehicleRemoval> = normWireObject<En1991SpecialVehicleRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991SpecialVehicleInsertion: NormWireReader<En1991SpecialVehicleInsertion> = normWireObject<En1991SpecialVehicleInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseSpecialVehicle) });
export const parseEn1991SpecialVehicleRelocation: NormWireReader<En1991SpecialVehicleRelocation> = normWireObject<En1991SpecialVehicleRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991SpecialVehicleModification: NormWireReader<En1991SpecialVehicleModification> = normWireObject<En1991SpecialVehicleModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991SpecialVehiclePatch)) });
export const parseEn1991SpecialVehicleDelta: NormWireReader<En1991SpecialVehicleDelta> = normWireObject<En1991SpecialVehicleDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991SpecialVehicleRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991SpecialVehicleInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991SpecialVehicleRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991SpecialVehicleModification))) });
export const parseEn1991WindBridgePatch: NormWireReader<En1991WindBridgePatch> = normWireObject<En1991WindBridgePatch>({ deckWidth: normWireOptional(normWireNullable(normWireNumber)), deckDepth: normWireOptional(normWireNullable(normWireNumber)), parapetKind: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), parapetHeight: normWireOptional(normWireNullable(normWireNumber)), trafficKind: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), heightAboveGround: normWireOptional(normWireNullable(normWireNumber)), span: normWireOptional(normWireNullable(normWireNumber)), assumedForce: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991WindBridgeRemoval: NormWireReader<En1991WindBridgeRemoval> = normWireObject<En1991WindBridgeRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991WindBridgeInsertion: NormWireReader<En1991WindBridgeInsertion> = normWireObject<En1991WindBridgeInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseWindBridge) });
export const parseEn1991WindBridgeRelocation: NormWireReader<En1991WindBridgeRelocation> = normWireObject<En1991WindBridgeRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991WindBridgeModification: NormWireReader<En1991WindBridgeModification> = normWireObject<En1991WindBridgeModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991WindBridgePatch)) });
export const parseEn1991WindBridgeDelta: NormWireReader<En1991WindBridgeDelta> = normWireObject<En1991WindBridgeDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991WindBridgeRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991WindBridgeInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991WindBridgeRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991WindBridgeModification))) });
export const parseEn1991DynamicElementPatch: NormWireReader<En1991DynamicElementPatch> = normWireObject<En1991DynamicElementPatch>({ height: normWireOptional(normWireNullable(normWireNumber)), width: normWireOptional(normWireNullable(normWireNumber)), firstFrequency: normWireOptional(normWireNullable(normWireNumber)), logDecrement: normWireOptional(normWireNullable(normWireNumber)), massPerLength: normWireOptional(normWireNullable(normWireNumber)), strouhal: normWireOptional(normWireNullable(normWireNumber)), assumedCscd: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991DynamicElementRemoval: NormWireReader<En1991DynamicElementRemoval> = normWireObject<En1991DynamicElementRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991DynamicElementInsertion: NormWireReader<En1991DynamicElementInsertion> = normWireObject<En1991DynamicElementInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseDynamicElement) });
export const parseEn1991DynamicElementRelocation: NormWireReader<En1991DynamicElementRelocation> = normWireObject<En1991DynamicElementRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991DynamicElementModification: NormWireReader<En1991DynamicElementModification> = normWireObject<En1991DynamicElementModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991DynamicElementPatch)) });
export const parseEn1991DynamicElementDelta: NormWireReader<En1991DynamicElementDelta> = normWireObject<En1991DynamicElementDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991DynamicElementRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991DynamicElementInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991DynamicElementRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991DynamicElementModification))) });
export const parseEn1991ImpactActionPatch: NormWireReader<En1991ImpactActionPatch> = normWireObject<En1991ImpactActionPatch>({ kind: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), mass: normWireOptional(normWireNullable(normWireNumber)), speed: normWireOptional(normWireNullable(normWireNumber)), stiffness: normWireOptional(normWireNullable(normWireNumber)), height: normWireOptional(normWireNullable(normWireNumber)), x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), assumedForce: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991ImpactActionRemoval: NormWireReader<En1991ImpactActionRemoval> = normWireObject<En1991ImpactActionRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991ImpactActionInsertion: NormWireReader<En1991ImpactActionInsertion> = normWireObject<En1991ImpactActionInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseImpactAction) });
export const parseEn1991ImpactActionRelocation: NormWireReader<En1991ImpactActionRelocation> = normWireObject<En1991ImpactActionRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991ImpactActionModification: NormWireReader<En1991ImpactActionModification> = normWireObject<En1991ImpactActionModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991ImpactActionPatch)) });
export const parseEn1991ImpactActionDelta: NormWireReader<En1991ImpactActionDelta> = normWireObject<En1991ImpactActionDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991ImpactActionRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991ImpactActionInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991ImpactActionRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991ImpactActionModification))) });
export const parseEn1991InternalExplosionPatch: NormWireReader<En1991InternalExplosionPatch> = normWireObject<En1991InternalExplosionPatch>({ volume: normWireOptional(normWireNullable(normWireNumber)), ventArea: normWireOptional(normWireNullable(normWireNumber)), ventPressure: normWireOptional(normWireNullable(normWireNumber)), assumedPressure: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991InternalExplosionRemoval: NormWireReader<En1991InternalExplosionRemoval> = normWireObject<En1991InternalExplosionRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991InternalExplosionInsertion: NormWireReader<En1991InternalExplosionInsertion> = normWireObject<En1991InternalExplosionInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseInternalExplosion) });
export const parseEn1991InternalExplosionRelocation: NormWireReader<En1991InternalExplosionRelocation> = normWireObject<En1991InternalExplosionRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991InternalExplosionModification: NormWireReader<En1991InternalExplosionModification> = normWireObject<En1991InternalExplosionModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991InternalExplosionPatch)) });
export const parseEn1991InternalExplosionDelta: NormWireReader<En1991InternalExplosionDelta> = normWireObject<En1991InternalExplosionDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991InternalExplosionRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991InternalExplosionInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991InternalExplosionRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991InternalExplosionModification))) });
export const parseEn1991ThermalSectionPatch: NormWireReader<En1991ThermalSectionPatch> = normWireObject<En1991ThermalSectionPatch>({ deckType: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), surfacing: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), length: normWireOptional(normWireNullable(normWireNumber)), depth: normWireOptional(normWireNullable(normWireNumber)), alphaMicro: normWireOptional(normWireNullable(normWireNumber)), assumedRange: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991ThermalSectionRemoval: NormWireReader<En1991ThermalSectionRemoval> = normWireObject<En1991ThermalSectionRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991ThermalSectionInsertion: NormWireReader<En1991ThermalSectionInsertion> = normWireObject<En1991ThermalSectionInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseThermalSection) });
export const parseEn1991ThermalSectionRelocation: NormWireReader<En1991ThermalSectionRelocation> = normWireObject<En1991ThermalSectionRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991ThermalSectionModification: NormWireReader<En1991ThermalSectionModification> = normWireObject<En1991ThermalSectionModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991ThermalSectionPatch)) });
export const parseEn1991ThermalSectionDelta: NormWireReader<En1991ThermalSectionDelta> = normWireObject<En1991ThermalSectionDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991ThermalSectionRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991ThermalSectionInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991ThermalSectionRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991ThermalSectionModification))) });
export const parseCaseNumberPatch: NormWireReader<CaseNumberPatch> = normWireObject<CaseNumberPatch>({ value: normWireOptional(normWireNullable(normWireNumber)) });
export const parseCaseNumberRemoval: NormWireReader<CaseNumberRemoval> = normWireObject<CaseNumberRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseNumberInsertion: NormWireReader<CaseNumberInsertion> = normWireObject<CaseNumberInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCaseNumber) });
export const parseCaseNumberRelocation: NormWireReader<CaseNumberRelocation> = normWireObject<CaseNumberRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseNumberModification: NormWireReader<CaseNumberModification> = normWireObject<CaseNumberModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseCaseNumberPatch)) });
export const parseCaseNumberDelta: NormWireReader<CaseNumberDelta> = normWireObject<CaseNumberDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberModification))) });
export const parseCaseTextPatch: NormWireReader<CaseTextPatch> = normWireObject<CaseTextPatch>({ value: normWireOptional(normWireNullable(normWireString)) });
export const parseCaseTextRemoval: NormWireReader<CaseTextRemoval> = normWireObject<CaseTextRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseTextInsertion: NormWireReader<CaseTextInsertion> = normWireObject<CaseTextInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCaseText) });
export const parseCaseTextRelocation: NormWireReader<CaseTextRelocation> = normWireObject<CaseTextRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseTextModification: NormWireReader<CaseTextModification> = normWireObject<CaseTextModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseCaseTextPatch)) });
export const parseCaseTextDelta: NormWireReader<CaseTextDelta> = normWireObject<CaseTextDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseCaseTextRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseCaseTextInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseCaseTextRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseCaseTextModification))) });
export const parseCaseRowPatch: NormWireReader<CaseRowPatch> = normWireObject<CaseRowPatch>({ labelEn: normWireOptional(normWireNullable(normWireString)), labelDe: normWireOptional(normWireNullable(normWireString)), entryId: normWireOptional(normWireNullable(normWireString)), numbers: normWireRequired(normWireRef(() => parseCaseNumberDelta)), texts: normWireRequired(normWireRef(() => parseCaseTextDelta)) });
export const parseCaseRowRemoval: NormWireReader<CaseRowRemoval> = normWireObject<CaseRowRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseRowInsertion: NormWireReader<CaseRowInsertion> = normWireObject<CaseRowInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCaseRow) });
export const parseCaseRowRelocation: NormWireReader<CaseRowRelocation> = normWireObject<CaseRowRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseRowModification: NormWireReader<CaseRowModification> = normWireObject<CaseRowModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseCaseRowPatch)) });
export const parseCaseRowDelta: NormWireReader<CaseRowDelta> = normWireObject<CaseRowDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseCaseRowRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseCaseRowInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseCaseRowRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseCaseRowModification))) });
export const parseEn1991HelicopterLandingPatch: NormWireReader<En1991HelicopterLandingPatch> = normWireObject<En1991HelicopterLandingPatch>({ mass: normWireOptional(normWireNullable(normWireNumber)), x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), height: normWireOptional(normWireNullable(normWireNumber)), assumedForce: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991HelicopterLandingRemoval: NormWireReader<En1991HelicopterLandingRemoval> = normWireObject<En1991HelicopterLandingRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991HelicopterLandingInsertion: NormWireReader<En1991HelicopterLandingInsertion> = normWireObject<En1991HelicopterLandingInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseHelicopterLanding) });
export const parseEn1991HelicopterLandingRelocation: NormWireReader<En1991HelicopterLandingRelocation> = normWireObject<En1991HelicopterLandingRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991HelicopterLandingModification: NormWireReader<En1991HelicopterLandingModification> = normWireObject<En1991HelicopterLandingModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991HelicopterLandingPatch)) });
export const parseEn1991HelicopterLandingDelta: NormWireReader<En1991HelicopterLandingDelta> = normWireObject<En1991HelicopterLandingDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991HelicopterLandingRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991HelicopterLandingInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991HelicopterLandingRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991HelicopterLandingModification))) });
export const parseEn1991TieRunPatch: NormWireReader<En1991TieRunPatch> = normWireObject<En1991TieRunPatch>({ kind: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), spacing: normWireOptional(normWireNullable(normWireNumber)), span: normWireOptional(normWireNullable(normWireNumber)), deadLoad: normWireOptional(normWireNullable(normWireNumber)), imposedLoad: normWireOptional(normWireNullable(normWireNumber)), psi: normWireOptional(normWireNullable(normWireNumber)), x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), providedForce: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991TieRunRemoval: NormWireReader<En1991TieRunRemoval> = normWireObject<En1991TieRunRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991TieRunInsertion: NormWireReader<En1991TieRunInsertion> = normWireObject<En1991TieRunInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseTieRun) });
export const parseEn1991TieRunRelocation: NormWireReader<En1991TieRunRelocation> = normWireObject<En1991TieRunRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991TieRunModification: NormWireReader<En1991TieRunModification> = normWireObject<En1991TieRunModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991TieRunPatch)) });
export const parseEn1991TieRunDelta: NormWireReader<En1991TieRunDelta> = normWireObject<En1991TieRunDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991TieRunRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991TieRunInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991TieRunRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991TieRunModification))) });
export const parseEn1991ConstructionLoadPatch: NormWireReader<En1991ConstructionLoadPatch> = normWireObject<En1991ConstructionLoadPatch>({ stage: normWireOptional(normWireNullable(normWireString)), kind: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), magnitude: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991ConstructionLoadRemoval: NormWireReader<En1991ConstructionLoadRemoval> = normWireObject<En1991ConstructionLoadRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991ConstructionLoadInsertion: NormWireReader<En1991ConstructionLoadInsertion> = normWireObject<En1991ConstructionLoadInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseConstructionLoad) });
export const parseEn1991ConstructionLoadRelocation: NormWireReader<En1991ConstructionLoadRelocation> = normWireObject<En1991ConstructionLoadRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991ConstructionLoadModification: NormWireReader<En1991ConstructionLoadModification> = normWireObject<En1991ConstructionLoadModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991ConstructionLoadPatch)) });
export const parseEn1991ConstructionLoadDelta: NormWireReader<En1991ConstructionLoadDelta> = normWireObject<En1991ConstructionLoadDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991ConstructionLoadRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991ConstructionLoadInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991ConstructionLoadRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991ConstructionLoadModification))) });
export const parseEn1991FallingLoadPatch: NormWireReader<En1991FallingLoadPatch> = normWireObject<En1991FallingLoadPatch>({ mass: normWireOptional(normWireNullable(normWireNumber)), dropHeight: normWireOptional(normWireNullable(normWireNumber)), stoppingDistance: normWireOptional(normWireNullable(normWireNumber)), x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), assumedForce: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991FallingLoadRemoval: NormWireReader<En1991FallingLoadRemoval> = normWireObject<En1991FallingLoadRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991FallingLoadInsertion: NormWireReader<En1991FallingLoadInsertion> = normWireObject<En1991FallingLoadInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseFallingLoad) });
export const parseEn1991FallingLoadRelocation: NormWireReader<En1991FallingLoadRelocation> = normWireObject<En1991FallingLoadRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991FallingLoadModification: NormWireReader<En1991FallingLoadModification> = normWireObject<En1991FallingLoadModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991FallingLoadPatch)) });
export const parseEn1991FallingLoadDelta: NormWireReader<En1991FallingLoadDelta> = normWireObject<En1991FallingLoadDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991FallingLoadRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991FallingLoadInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991FallingLoadRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991FallingLoadModification))) });
export const parseEn1991ExecutionImperfectionPatch: NormWireReader<En1991ExecutionImperfectionPatch> = normWireObject<En1991ExecutionImperfectionPatch>({ height: normWireOptional(normWireNullable(normWireNumber)), tolerance: normWireOptional(normWireNullable(normWireNumber)), verticalLoad: normWireOptional(normWireNullable(normWireNumber)), x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), assumedForce: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991ExecutionImperfectionRemoval: NormWireReader<En1991ExecutionImperfectionRemoval> = normWireObject<En1991ExecutionImperfectionRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991ExecutionImperfectionInsertion: NormWireReader<En1991ExecutionImperfectionInsertion> = normWireObject<En1991ExecutionImperfectionInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseExecutionImperfection) });
export const parseEn1991ExecutionImperfectionRelocation: NormWireReader<En1991ExecutionImperfectionRelocation> = normWireObject<En1991ExecutionImperfectionRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991ExecutionImperfectionModification: NormWireReader<En1991ExecutionImperfectionModification> = normWireObject<En1991ExecutionImperfectionModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991ExecutionImperfectionPatch)) });
export const parseEn1991ExecutionImperfectionDelta: NormWireReader<En1991ExecutionImperfectionDelta> = normWireObject<En1991ExecutionImperfectionDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991ExecutionImperfectionRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991ExecutionImperfectionInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991ExecutionImperfectionRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991ExecutionImperfectionModification))) });
export const parseEn1991DerailmentCasePatch: NormWireReader<En1991DerailmentCasePatch> = normWireObject<En1991DerailmentCasePatch>({ distance: normWireOptional(normWireNullable(normWireNumber)), designForceX: normWireOptional(normWireNullable(normWireNumber)), designForceY: normWireOptional(normWireNullable(normWireNumber)), x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), assumedForce: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991DerailmentCaseRemoval: NormWireReader<En1991DerailmentCaseRemoval> = normWireObject<En1991DerailmentCaseRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991DerailmentCaseInsertion: NormWireReader<En1991DerailmentCaseInsertion> = normWireObject<En1991DerailmentCaseInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseDerailmentCase) });
export const parseEn1991DerailmentCaseRelocation: NormWireReader<En1991DerailmentCaseRelocation> = normWireObject<En1991DerailmentCaseRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991DerailmentCaseModification: NormWireReader<En1991DerailmentCaseModification> = normWireObject<En1991DerailmentCaseModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991DerailmentCasePatch)) });
export const parseEn1991DerailmentCaseDelta: NormWireReader<En1991DerailmentCaseDelta> = normWireObject<En1991DerailmentCaseDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991DerailmentCaseRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991DerailmentCaseInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991DerailmentCaseRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991DerailmentCaseModification))) });
export const parseEn1991CraneHoistPatch: NormWireReader<En1991CraneHoistPatch> = normWireObject<En1991CraneHoistPatch>({ ratedLoad: normWireOptional(normWireNullable(normWireNumber)), releaseFraction: normWireOptional(normWireNullable(normWireNumber)), releaseDevice: normWireOptional(normWireNullable(normWireString)) });
export const parseEn1991CraneHoistRemoval: NormWireReader<En1991CraneHoistRemoval> = normWireObject<En1991CraneHoistRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991CraneHoistInsertion: NormWireReader<En1991CraneHoistInsertion> = normWireObject<En1991CraneHoistInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCraneHoist) });
export const parseEn1991CraneHoistRelocation: NormWireReader<En1991CraneHoistRelocation> = normWireObject<En1991CraneHoistRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991CraneHoistModification: NormWireReader<En1991CraneHoistModification> = normWireObject<En1991CraneHoistModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991CraneHoistPatch)) });
export const parseEn1991CraneHoistDelta: NormWireReader<En1991CraneHoistDelta> = normWireObject<En1991CraneHoistDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991CraneHoistRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991CraneHoistInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991CraneHoistRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991CraneHoistModification))) });
export const parseEn1991FootbridgePatch: NormWireReader<En1991FootbridgePatch> = normWireObject<En1991FootbridgePatch>({ loadedLength: normWireOptional(normWireNullable(normWireNumber)), deckWidth: normWireOptional(normWireNullable(normWireNumber)), naturalFrequency: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991FootbridgeRemoval: NormWireReader<En1991FootbridgeRemoval> = normWireObject<En1991FootbridgeRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991FootbridgeInsertion: NormWireReader<En1991FootbridgeInsertion> = normWireObject<En1991FootbridgeInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseFootbridge) });
export const parseEn1991FootbridgeRelocation: NormWireReader<En1991FootbridgeRelocation> = normWireObject<En1991FootbridgeRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991FootbridgeModification: NormWireReader<En1991FootbridgeModification> = normWireObject<En1991FootbridgeModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991FootbridgePatch)) });
export const parseEn1991FootbridgeDelta: NormWireReader<En1991FootbridgeDelta> = normWireObject<En1991FootbridgeDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991FootbridgeRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991FootbridgeInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991FootbridgeRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991FootbridgeModification))) });
export const parseEn1991RailDynamicPatch: NormWireReader<En1991RailDynamicPatch> = normWireObject<En1991RailDynamicPatch>({ determinantLength: normWireOptional(normWireNullable(normWireNumber)), maintenance: normWireOptional(normWireNullable(normWireString)) });
export const parseEn1991RailDynamicRemoval: NormWireReader<En1991RailDynamicRemoval> = normWireObject<En1991RailDynamicRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991RailDynamicInsertion: NormWireReader<En1991RailDynamicInsertion> = normWireObject<En1991RailDynamicInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseRailDynamic) });
export const parseEn1991RailDynamicRelocation: NormWireReader<En1991RailDynamicRelocation> = normWireObject<En1991RailDynamicRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991RailDynamicModification: NormWireReader<En1991RailDynamicModification> = normWireObject<En1991RailDynamicModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991RailDynamicPatch)) });
export const parseEn1991RailDynamicDelta: NormWireReader<En1991RailDynamicDelta> = normWireObject<En1991RailDynamicDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991RailDynamicRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991RailDynamicInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991RailDynamicRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991RailDynamicModification))) });
export const parseEn1991LoadDispersalPatch: NormWireReader<En1991LoadDispersalPatch> = normWireObject<En1991LoadDispersalPatch>({ wheelLength: normWireOptional(normWireNullable(normWireNumber)), wheelWidth: normWireOptional(normWireNullable(normWireNumber)), pavementThickness: normWireOptional(normWireNullable(normWireNumber)), slabThickness: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991LoadDispersalRemoval: NormWireReader<En1991LoadDispersalRemoval> = normWireObject<En1991LoadDispersalRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991LoadDispersalInsertion: NormWireReader<En1991LoadDispersalInsertion> = normWireObject<En1991LoadDispersalInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseLoadDispersal) });
export const parseEn1991LoadDispersalRelocation: NormWireReader<En1991LoadDispersalRelocation> = normWireObject<En1991LoadDispersalRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991LoadDispersalModification: NormWireReader<En1991LoadDispersalModification> = normWireObject<En1991LoadDispersalModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991LoadDispersalPatch)) });
export const parseEn1991LoadDispersalDelta: NormWireReader<En1991LoadDispersalDelta> = normWireObject<En1991LoadDispersalDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991LoadDispersalRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991LoadDispersalInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991LoadDispersalRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991LoadDispersalModification))) });
export const parseEn1991BulkSolidPatch: NormWireReader<En1991BulkSolidPatch> = normWireObject<En1991BulkSolidPatch>({ internalFriction: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991BulkSolidRemoval: NormWireReader<En1991BulkSolidRemoval> = normWireObject<En1991BulkSolidRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991BulkSolidInsertion: NormWireReader<En1991BulkSolidInsertion> = normWireObject<En1991BulkSolidInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseBulkSolid) });
export const parseEn1991BulkSolidRelocation: NormWireReader<En1991BulkSolidRelocation> = normWireObject<En1991BulkSolidRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991BulkSolidModification: NormWireReader<En1991BulkSolidModification> = normWireObject<En1991BulkSolidModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991BulkSolidPatch)) });
export const parseEn1991BulkSolidDelta: NormWireReader<En1991BulkSolidDelta> = normWireObject<En1991BulkSolidDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991BulkSolidRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991BulkSolidInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991BulkSolidRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991BulkSolidModification))) });
export const parseEn1991RotatingMachinePatch: NormWireReader<En1991RotatingMachinePatch> = normWireObject<En1991RotatingMachinePatch>({ mass: normWireOptional(normWireNullable(normWireNumber)), eccentricity: normWireOptional(normWireNullable(normWireNumber)), frequency: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991RotatingMachineRemoval: NormWireReader<En1991RotatingMachineRemoval> = normWireObject<En1991RotatingMachineRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991RotatingMachineInsertion: NormWireReader<En1991RotatingMachineInsertion> = normWireObject<En1991RotatingMachineInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseRotatingMachine) });
export const parseEn1991RotatingMachineRelocation: NormWireReader<En1991RotatingMachineRelocation> = normWireObject<En1991RotatingMachineRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991RotatingMachineModification: NormWireReader<En1991RotatingMachineModification> = normWireObject<En1991RotatingMachineModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991RotatingMachinePatch)) });
export const parseEn1991RotatingMachineDelta: NormWireReader<En1991RotatingMachineDelta> = normWireObject<En1991RotatingMachineDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991RotatingMachineRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991RotatingMachineInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991RotatingMachineRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991RotatingMachineModification))) });
export const parseEn1991FatigueModelRemoval: NormWireReader<En1991FatigueModelRemoval> = normWireObject<En1991FatigueModelRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991FatigueModelInsertion: NormWireReader<En1991FatigueModelInsertion> = normWireObject<En1991FatigueModelInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseFatigueModel) });
export const parseEn1991FatigueModelRelocation: NormWireReader<En1991FatigueModelRelocation> = normWireObject<En1991FatigueModelRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991FatigueModelModification: NormWireReader<En1991FatigueModelModification> = normWireObject<En1991FatigueModelModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991FatigueModelPatch)) });
export const parseEn1991FatigueModelDelta: NormWireReader<En1991FatigueModelDelta> = normWireObject<En1991FatigueModelDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991FatigueModelRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991FatigueModelInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991FatigueModelRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991FatigueModelModification))) });
export const parseEn1991FatigueModelPatch: NormWireReader<En1991FatigueModelPatch> = normWireObject<En1991FatigueModelPatch>({ model: normWireOptional(normWireNullable(normWireString)), lane: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), assumedAxleLoad: normWireOptional(normWireNullable(normWireNumber)), assumedUdl: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991SupportCollisionRemoval: NormWireReader<En1991SupportCollisionRemoval> = normWireObject<En1991SupportCollisionRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991SupportCollisionInsertion: NormWireReader<En1991SupportCollisionInsertion> = normWireObject<En1991SupportCollisionInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseSupportCollision) });
export const parseEn1991SupportCollisionRelocation: NormWireReader<En1991SupportCollisionRelocation> = normWireObject<En1991SupportCollisionRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991SupportCollisionModification: NormWireReader<En1991SupportCollisionModification> = normWireObject<En1991SupportCollisionModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991SupportCollisionPatch)) });
export const parseEn1991SupportCollisionDelta: NormWireReader<En1991SupportCollisionDelta> = normWireObject<En1991SupportCollisionDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991SupportCollisionRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991SupportCollisionInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991SupportCollisionRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991SupportCollisionModification))) });
export const parseEn1991SupportCollisionPatch: NormWireReader<En1991SupportCollisionPatch> = normWireObject<En1991SupportCollisionPatch>({ location: normWireOptional(normWireNullable(normWireString)), assumedForceX: normWireOptional(normWireNullable(normWireNumber)), assumedForceY: normWireOptional(normWireNullable(normWireNumber)), height: normWireOptional(normWireNullable(normWireNumber)), positionX: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991LocalFireRemoval: NormWireReader<En1991LocalFireRemoval> = normWireObject<En1991LocalFireRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991LocalFireInsertion: NormWireReader<En1991LocalFireInsertion> = normWireObject<En1991LocalFireInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseLocalFire) });
export const parseEn1991LocalFireRelocation: NormWireReader<En1991LocalFireRelocation> = normWireObject<En1991LocalFireRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991LocalFireModification: NormWireReader<En1991LocalFireModification> = normWireObject<En1991LocalFireModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991LocalFirePatch)) });
export const parseEn1991LocalFireDelta: NormWireReader<En1991LocalFireDelta> = normWireObject<En1991LocalFireDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991LocalFireRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991LocalFireInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991LocalFireRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991LocalFireModification))) });
export const parseEn1991LocalFirePatch: NormWireReader<En1991LocalFirePatch> = normWireObject<En1991LocalFirePatch>({ fireDiameter: normWireOptional(normWireNullable(normWireNumber)), heatReleaseRate: normWireOptional(normWireNullable(normWireNumber)), ceilingHeight: normWireOptional(normWireNullable(normWireNumber)), horizontalDistance: normWireOptional(normWireNullable(normWireNumber)), assumedFlux: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1991SnowGuardRemoval: NormWireReader<En1991SnowGuardRemoval> = normWireObject<En1991SnowGuardRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991SnowGuardInsertion: NormWireReader<En1991SnowGuardInsertion> = normWireObject<En1991SnowGuardInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseSnowGuard) });
export const parseEn1991SnowGuardRelocation: NormWireReader<En1991SnowGuardRelocation> = normWireObject<En1991SnowGuardRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1991SnowGuardModification: NormWireReader<En1991SnowGuardModification> = normWireObject<En1991SnowGuardModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1991SnowGuardPatch)) });
export const parseEn1991SnowGuardDelta: NormWireReader<En1991SnowGuardDelta> = normWireObject<En1991SnowGuardDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1991SnowGuardRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1991SnowGuardInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1991SnowGuardRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1991SnowGuardModification))) });
export const parseEn1991SnowGuardPatch: NormWireReader<En1991SnowGuardPatch> = normWireObject<En1991SnowGuardPatch>({ roofId: normWireOptional(normWireNullable(normWireString)), slopeLength: normWireOptional(normWireNullable(normWireNumber)), assumedForce: normWireOptional(normWireNullable(normWireNumber)) });
