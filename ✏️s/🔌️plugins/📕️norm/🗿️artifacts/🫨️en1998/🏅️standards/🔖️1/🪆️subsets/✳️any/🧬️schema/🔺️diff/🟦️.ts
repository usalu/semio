/** 🔺️ `En1998Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireAny, normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type AnalysisDirection, parseAnalysisDirection, type AnalysisMethodChoice, parseAnalysisMethodChoice, type BearingDevice, parseBearingDevice, type DiaphragmMaterial, parseDiaphragmMaterial, type DriftLimitClass, parseDriftLimitClass, type DuctilityClass, parseDuctilityClass, type En1998Accelerogram, parseEn1998Accelerogram, type En1998Assessment, parseEn1998Assessment, type En1998Bearing, parseEn1998Bearing, type En1998Bridge, parseEn1998Bridge, type En1998Building, parseEn1998Building, type En1998Diaphragm, parseEn1998Diaphragm, type En1998Foundation, parseEn1998Foundation, type En1998Joint, parseEn1998Joint, type En1998Member, parseEn1998Member, type En1998Pier, parseEn1998Pier, type En1998Placement, parseEn1998Placement, type En1998RetainingWall, parseEn1998RetainingWall, type En1998Silo, parseEn1998Silo, type En1998Site, parseEn1998Site, type En1998Slope, parseEn1998Slope, type En1998SoilLayer, parseEn1998SoilLayer, type En1998Storey, parseEn1998Storey, type En1998System, parseEn1998System, type En1998Tank, parseEn1998Tank, type En1998TieBeam, parseEn1998TieBeam, type En1998Tower, parseEn1998Tower, type En1998VariableAction, parseEn1998VariableAction, type ImposedCategory, parseImposedCategory, type KnowledgeLevel, parseKnowledgeLevel, type LimitState, parseLimitState, type PierStiffnessBasis, parsePierStiffnessBasis, type StructuralMaterial, parseStructuralMaterial, type SystemType, parseSystemType, type T1Method, parseT1Method } from "../📸️snapshot/🟦️.ts";

export interface En1998Diff {
  /** @state artifact */
  annex?: ("En" | "De") | null;
  /** @state artifact */
  site?: En1998Site | null;
  /** @state artifact */
  buildings: En1998BuildingDelta;
  /** @state artifact */
  bridges: En1998BridgeDelta;
  /** @state artifact */
  assessments: En1998AssessmentDelta;
  /** @state artifact */
  silos: En1998SiloDelta;
  /** @state artifact */
  tanks: En1998TankDelta;
  /** @state artifact */
  foundations: En1998FoundationDelta;
  /** @state artifact */
  retainingWalls: En1998RetainingWallDelta;
  /** @state artifact */
  towers: En1998TowerDelta;
  /** @state artifact */
  soilLayers: En1998SoilLayerDelta;
  /** @state artifact */
  diaphragms: En1998DiaphragmDelta;
  /** @state artifact */
  accelerograms: En1998AccelerogramDelta;
  /** @state artifact */
  piers: En1998PierDelta;
  /** @state artifact */
  bearings: En1998BearingDelta;
  /** @state artifact */
  joints: En1998JointDelta;
  /** @state artifact */
  tieBeams: En1998TieBeamDelta;
  /** @state artifact */
  slopes: En1998SlopeDelta;
  /** @state artifact */
  placements: En1998PlacementDelta;
}

export interface En1998SystemDelta {
  removed: En1998SystemRemoval[];
  inserted: En1998SystemInsertion[];
  moved: En1998SystemRelocation[];
  modified: En1998SystemModification[];
}

export interface En1998SystemRemoval {
  id: string;
  index: number;
}

export interface En1998SystemInsertion {
  index: number;
  row: En1998System;
}

export interface En1998SystemRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998SystemModification {
  id: string;
  patch: En1998SystemPatch;
}

export interface En1998SystemPatch {
  direction?: AnalysisDirection | null;
  systemType?: SystemType | null;
  material?: StructuralMaterial | null;
  ductilityClass?: DuctilityClass | null;
  q0?: number | null;
  alphaUOverAlpha1?: number | null;
  kW?: number | null;
  baseShearResistanceN?: number | null;
}

export interface En1998StoreyDelta {
  removed: En1998StoreyRemoval[];
  inserted: En1998StoreyInsertion[];
  moved: En1998StoreyRelocation[];
  modified: En1998StoreyModification[];
}

export interface En1998StoreyRemoval {
  id: string;
  index: number;
}

export interface En1998StoreyInsertion {
  index: number;
  row: En1998Storey;
}

export interface En1998StoreyRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998StoreyModification {
  id: string;
  patch: En1998StoreyPatch;
}

export interface En1998VariableActionDelta {
  removed: En1998VariableActionRemoval[];
  inserted: En1998VariableActionInsertion[];
  moved: En1998VariableActionRelocation[];
  modified: En1998VariableActionModification[];
}

export interface En1998VariableActionRemoval {
  id: string;
  index: number;
}

export interface En1998VariableActionInsertion {
  index: number;
  row: En1998VariableAction;
}

export interface En1998VariableActionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998VariableActionModification {
  id: string;
  patch: En1998VariableActionPatch;
}

export interface En1998VariableActionPatch {
  category?: ImposedCategory | null;
  qkN?: number | null;
}

export interface En1998StoreyPatch {
  heightM?: number | null;
  permanentGkN?: number | null;
  correlatedOccupancy?: boolean | null;
  variables: En1998VariableActionDelta;
  stiffnessX?: number | null;
  stiffnessY?: number | null;
  centreOfMassXM?: number | null;
  centreOfMassYM?: number | null;
  centreOfStiffnessXM?: number | null;
  centreOfStiffnessYM?: number | null;
  driftXM?: number | null;
  driftYM?: number | null;
  shearResistanceN?: number | null;
}

export interface En1998MemberDelta {
  removed: En1998MemberRemoval[];
  inserted: En1998MemberInsertion[];
  moved: En1998MemberRelocation[];
  modified: En1998MemberModification[];
}

export interface En1998MemberRemoval {
  id: string;
  index: number;
}

export interface En1998MemberInsertion {
  index: number;
  row: En1998Member;
}

export interface En1998MemberRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998MemberModification {
  id: string;
  patch: En1998MemberPatch;
}

export interface En1998MemberPatch {
  material?: StructuralMaterial | null;
  role?: ("beam" | "column" | "wall" | "brace") | null;
  detailingCompatibleWithQ?: boolean | null;
  minDimensionM?: number | null;
  rho?: number | null;
  rhoPrime?: number | null;
  omegaWd?: number | null;
  steelSectionClass?: number | null;
  fastenerDiameterMm?: number | null;
  timberThicknessMm?: number | null;
  wallThicknessM?: number | null;
  wallEffectiveHeightM?: number | null;
  wallLengthM?: number | null;
  wallDirectionRad?: number | null;
  wallStoreyId?: string | null;
  wallStartXM?: number | null;
  wallStartYM?: number | null;
  wallPoissonRatio?: number | null;
  effectiveStiffnessBasis?: ("simplified" | "declared") | null;
  effectiveStiffnessRatio?: number | null;
}

export interface En1998BuildingDelta {
  removed: En1998BuildingRemoval[];
  inserted: En1998BuildingInsertion[];
  moved: En1998BuildingRelocation[];
  modified: En1998BuildingModification[];
}

export interface En1998BuildingRemoval {
  id: string;
  index: number;
}

export interface En1998BuildingInsertion {
  index: number;
  row: En1998Building;
}

export interface En1998BuildingRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998BuildingModification {
  id: string;
  patch: En1998BuildingPatch;
}

export interface En1998BuildingPatch {
  name?: string | null;
  planWidthM?: number | null;
  planLengthM?: number | null;
  systems: En1998SystemDelta;
  storeys: En1998StoreyDelta;
  members: En1998MemberDelta;
  planRegular?: boolean | null;
  elevationRegular?: boolean | null;
  t1Method?: T1Method | null;
  t1GivenS?: number | null;
  ct?: number | null;
  driftLimitClass?: DriftLimitClass | null;
  nu?: number | null;
  multipleResistingSystems?: boolean | null;
  claimsSimpleMasonry?: boolean | null;
  masonryWallAreaRatio?: number | null;
  accidentalEccentricityRatio?: number | null;
  setbackRatio?: number | null;
  planEnvelopeGapRatio?: number | null;
  jointGapM?: number | null;
  nonstructuralWeightN?: number | null;
  nonstructuralHeightM?: number | null;
  nonstructuralPeriodS?: number | null;
  nonstructuralQ?: number | null;
  nonstructuralResistanceN?: number | null;
  columnMomentSumNm?: number | null;
  beamMomentSumNm?: number | null;
  overturningBaseXM?: number | null;
  overturningBaseYM?: number | null;
  tieDownMomentNm?: number | null;
  storeyStiffnessBasis?: ("simplified" | "declared") | null;
  storeyStiffnessRatio?: number | null;
}

export interface En1998BridgeDelta {
  removed: En1998BridgeRemoval[];
  inserted: En1998BridgeInsertion[];
  moved: En1998BridgeRelocation[];
  modified: En1998BridgeModification[];
}

export interface En1998BridgeRemoval {
  id: string;
  index: number;
}

export interface En1998BridgeInsertion {
  index: number;
  row: En1998Bridge;
}

export interface En1998BridgeRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998BridgeModification {
  id: string;
  patch: En1998BridgePatch;
}

export interface En1998BridgePatch {
  periodRatio?: number | null;
  fundamentalPeriodS?: number | null;
  vRdN?: number | null;
  bearingDRdM?: number | null;
  permanentGkN?: number | null;
  correlatedOccupancy?: boolean | null;
  variables: En1998VariableActionDelta;
  pierHeightM?: number | null;
  pierMomentRdNm?: number | null;
  pierShearRdN?: number | null;
  deckLengthM?: number | null;
  deckEiNm2?: number | null;
  importanceClass?: number | null;
  verticalPeriodS?: number | null;
  pierCrackedInertiaRatio?: number | null;
}

export interface En1998AssessmentDelta {
  removed: En1998AssessmentRemoval[];
  inserted: En1998AssessmentInsertion[];
  moved: En1998AssessmentRelocation[];
  modified: En1998AssessmentModification[];
}

export interface En1998AssessmentRemoval {
  id: string;
  index: number;
}

export interface En1998AssessmentInsertion {
  index: number;
  row: En1998Assessment;
}

export interface En1998AssessmentRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998AssessmentModification {
  id: string;
  patch: En1998AssessmentPatch;
}

export interface En1998AssessmentPatch {
  knowledgeLevel?: KnowledgeLevel | null;
  limitState?: LimitState | null;
  supportedBuildingId?: string | null;
  rKN?: number | null;
  gammaEl?: number | null;
  chordRotationDemandRad?: number | null;
  chordRotationYieldRad?: number | null;
  chordRotationUltimateRad?: number | null;
  sectionDepthM?: number | null;
  sectionWidthM?: number | null;
  effectiveDepthM?: number | null;
  shearSpanM?: number | null;
  axialForceN?: number | null;
  tensionSteelAreaM2?: number | null;
  compressionSteelAreaM2?: number | null;
  barDiameterMm?: number | null;
  steelYieldMpa?: number | null;
  concreteStrengthMpa?: number | null;
  stirrupLegAreaM2?: number | null;
  stirrupSpacingM?: number | null;
  stirrupYieldMpa?: number | null;
  coreWidthM?: number | null;
  coreDepthM?: number | null;
  restrainedBarCount?: number | null;
  restrainedBarSpacingM?: number | null;
  diagonalReinforcementRatio?: number | null;
  shearCrackingBeforeYield?: number | null;
  shearForceDemandN?: number | null;
  intermediateSteelAreaM2?: number | null;
  analysisMethod?: AnalysisMethodChoice | null;
}

export interface En1998SiloDelta {
  removed: En1998SiloRemoval[];
  inserted: En1998SiloInsertion[];
  moved: En1998SiloRelocation[];
  modified: En1998SiloModification[];
}

export interface En1998SiloRemoval {
  id: string;
  index: number;
}

export interface En1998SiloInsertion {
  index: number;
  row: En1998Silo;
}

export interface En1998SiloRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998SiloModification {
  id: string;
  patch: En1998SiloPatch;
}

export interface En1998SiloPatch {
  heightM?: number | null;
  radiusM?: number | null;
  permanentGkN?: number | null;
  contentQkN?: number | null;
  contentCategory?: ImposedCategory | null;
  fillingRatio?: number | null;
  nRdN?: number | null;
  vRdN?: number | null;
  qNominal?: number | null;
}

export interface En1998TankDelta {
  removed: En1998TankRemoval[];
  inserted: En1998TankInsertion[];
  moved: En1998TankRelocation[];
  modified: En1998TankModification[];
}

export interface En1998TankRemoval {
  id: string;
  index: number;
}

export interface En1998TankInsertion {
  index: number;
  row: En1998Tank;
}

export interface En1998TankRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998TankModification {
  id: string;
  patch: En1998TankPatch;
}

export interface En1998TankPatch {
  heightM?: number | null;
  radiusM?: number | null;
  permanentGkN?: number | null;
  contentQkN?: number | null;
  contentCategory?: ImposedCategory | null;
  fillingRatio?: number | null;
  vRdN?: number | null;
  shellStrengthMpa?: number | null;
  shellThicknessM?: number | null;
}

export interface En1998FoundationDelta {
  removed: En1998FoundationRemoval[];
  inserted: En1998FoundationInsertion[];
  moved: En1998FoundationRelocation[];
  modified: En1998FoundationModification[];
}

export interface En1998FoundationRemoval {
  id: string;
  index: number;
}

export interface En1998FoundationInsertion {
  index: number;
  row: En1998Foundation;
}

export interface En1998FoundationRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998FoundationModification {
  id: string;
  patch: En1998FoundationPatch;
}

export interface En1998FoundationPatch {
  supportedBuildingId?: string | null;
  areaM2?: number | null;
  pRdPa?: number | null;
  passiveResistanceN?: number | null;
  kFoundation?: number | null;
  kSoil?: number | null;
  liqN160?: number | null;
  liqFinesPercent?: number | null;
  liqDepthM?: number | null;
  liqWaterDepthM?: number | null;
  liqUnitWeightNM3?: number | null;
  liqMagnitude?: number | null;
  liqLayerThicknessM?: number | null;
  footingWidthM?: number | null;
  bearingAxialN?: number | null;
  bearingShearN?: number | null;
  bearingMomentNm?: number | null;
  bearingNMaxN?: number | null;
  soilFrictionAngleDeg?: number | null;
  pileDiameterM?: number | null;
  pileLengthM?: number | null;
  pileModulusPa?: number | null;
  pileSubgradeModulusNM3?: number | null;
  pileHeadShearN?: number | null;
  pileHeadFixed?: number | null;
  pileInterfaceDepthM?: number | null;
  pileUpperVsMS?: number | null;
  pileLowerVsMS?: number | null;
  pileMomentRdNm?: number | null;
  baseFrictionAngleDeg?: number | null;
  basePorePressurePa?: number | null;
}

export interface En1998RetainingWallDelta {
  removed: En1998RetainingWallRemoval[];
  inserted: En1998RetainingWallInsertion[];
  moved: En1998RetainingWallRelocation[];
  modified: En1998RetainingWallModification[];
}

export interface En1998RetainingWallRemoval {
  id: string;
  index: number;
}

export interface En1998RetainingWallInsertion {
  index: number;
  row: En1998RetainingWall;
}

export interface En1998RetainingWallRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998RetainingWallModification {
  id: string;
  patch: En1998RetainingWallPatch;
}

export interface En1998RetainingWallPatch {
  heightM?: number | null;
  phiDeg?: number | null;
  soilGamma?: number | null;
  r?: number | null;
  hRdNPerM?: number | null;
  waterHeightM?: number | null;
  saturatedGamma?: number | null;
  permeabilityMS?: number | null;
}

export interface En1998TowerDelta {
  removed: En1998TowerRemoval[];
  inserted: En1998TowerInsertion[];
  moved: En1998TowerRelocation[];
  modified: En1998TowerModification[];
}

export interface En1998TowerRemoval {
  id: string;
  index: number;
}

export interface En1998TowerInsertion {
  index: number;
  row: En1998Tower;
}

export interface En1998TowerRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998TowerModification {
  id: string;
  patch: En1998TowerPatch;
}

export interface En1998TowerPatch {
  heightM?: number | null;
  mRdNm?: number | null;
  isChimney?: boolean | null;
  qNominal?: number | null;
  permanentGkN?: number | null;
  correlatedOccupancy?: boolean | null;
  variables: En1998VariableActionDelta;
  eiNm2?: number | null;
  baseRotationalStiffnessNm?: number | null;
}

export interface En1998PlacementDelta {
  removed: En1998PlacementRemoval[];
  inserted: En1998PlacementInsertion[];
  moved: En1998PlacementRelocation[];
  modified: En1998PlacementModification[];
}

export interface En1998PlacementRemoval {
  id: string;
  index: number;
}

export interface En1998PlacementInsertion {
  index: number;
  row: En1998Placement;
}

export interface En1998PlacementRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998PlacementModification {
  id: string;
  patch: En1998PlacementPatch;
}

export interface En1998PlacementPatch {
  x?: number | null;
  y?: number | null;
  z?: number | null;
  yaw?: number | null;
}

export interface En1998SoilLayerDelta {
  removed: En1998SoilLayerRemoval[];
  inserted: En1998SoilLayerInsertion[];
  moved: En1998SoilLayerRelocation[];
  modified: En1998SoilLayerModification[];
}

export interface En1998SoilLayerRemoval {
  id: string;
  index: number;
}

export interface En1998SoilLayerInsertion {
  index: number;
  row: En1998SoilLayer;
}

export interface En1998SoilLayerRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998SoilLayerModification {
  id: string;
  patch: En1998SoilLayerPatch;
}

export interface En1998SoilLayerPatch {
  foundationId?: string | null;
  thicknessM?: number | null;
  unitWeightNM3?: number | null;
  shearWaveVelocityMS?: number | null;
  n160?: number | null;
  finesPercent?: number | null;
}

export interface En1998DiaphragmDelta {
  removed: En1998DiaphragmRemoval[];
  inserted: En1998DiaphragmInsertion[];
  moved: En1998DiaphragmRelocation[];
  modified: En1998DiaphragmModification[];
}

export interface En1998DiaphragmRemoval {
  id: string;
  index: number;
}

export interface En1998DiaphragmInsertion {
  index: number;
  row: En1998Diaphragm;
}

export interface En1998DiaphragmRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998DiaphragmModification {
  id: string;
  patch: En1998DiaphragmPatch;
}

export interface En1998DiaphragmPatch {
  buildingId?: string | null;
  storeyId?: string | null;
  material?: DiaphragmMaterial | null;
  thicknessM?: number | null;
  spanM?: number | null;
  depthM?: number | null;
  openingWidthM?: number | null;
  shearStrengthMpa?: number | null;
  chordAreaM2?: number | null;
  chordStrengthMpa?: number | null;
  gammaD?: number | null;
}

export interface En1998AccelerogramDelta {
  removed: En1998AccelerogramRemoval[];
  inserted: En1998AccelerogramInsertion[];
  moved: En1998AccelerogramRelocation[];
  modified: En1998AccelerogramModification[];
}

export interface En1998AccelerogramRemoval {
  id: string;
  index: number;
}

export interface En1998AccelerogramInsertion {
  index: number;
  row: En1998Accelerogram;
}

export interface En1998AccelerogramRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998AccelerogramModification {
  id: string;
  patch: En1998AccelerogramPatch;
}

export interface En1998AccelerogramPatch {
  buildingId?: string | null;
  label?: string | null;
  samples?: string | null;
  dtS?: number | null;
  scaleFactor?: number | null;
}

export interface En1998PierDelta {
  removed: En1998PierRemoval[];
  inserted: En1998PierInsertion[];
  moved: En1998PierRelocation[];
  modified: En1998PierModification[];
}

export interface En1998PierRemoval {
  id: string;
  index: number;
}

export interface En1998PierInsertion {
  index: number;
  row: En1998Pier;
}

export interface En1998PierRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998PierModification {
  id: string;
  patch: En1998PierPatch;
}

export interface En1998PierPatch {
  bridgeId?: string | null;
  xM?: number | null;
  heightM?: number | null;
  sectionBM?: number | null;
  sectionHM?: number | null;
  modulusMpa?: number | null;
  momentRdNm?: number | null;
  shearRdN?: number | null;
  reinforcementRatio?: string | null;
  coverM?: string | null;
  concreteFckMpa?: string | null;
  steelFykMpa?: string | null;
  axialLoadN?: string | null;
  stiffnessBasis?: PierStiffnessBasis | null;
}

export interface En1998BearingDelta {
  removed: En1998BearingRemoval[];
  inserted: En1998BearingInsertion[];
  moved: En1998BearingRelocation[];
  modified: En1998BearingModification[];
}

export interface En1998BearingRemoval {
  id: string;
  index: number;
}

export interface En1998BearingInsertion {
  index: number;
  row: En1998Bearing;
}

export interface En1998BearingRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998BearingModification {
  id: string;
  patch: En1998BearingPatch;
}

export interface En1998BearingPatch {
  bridgeId?: string | null;
  pierId?: string | null;
  device?: BearingDevice | null;
  xM?: number | null;
  stiffnessNPerM?: number | null;
  dRdM?: number | null;
  seatLengthM?: number | null;
  minSupportLengthM?: number | null;
}

export interface En1998JointDelta {
  removed?: En1998JointRemoval[];
  inserted?: En1998JointInsertion[];
  moved?: En1998JointRelocation[];
  modified?: En1998JointModification[];
}

export interface En1998JointRemoval {
  id: string;
  index: number;
}

export interface En1998JointInsertion {
  index: number;
  row: En1998Joint;
}

export interface En1998JointRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998JointModification {
  id: string;
  patch: En1998JointPatch;
}

export interface En1998JointPatch {
  buildingId?: string | null;
  storeyId?: string | null;
  columnId?: string | null;
  beamId?: string | null;
  xM?: number | null;
  yM?: number | null;
  columnMomentRdNm?: number | null;
  beamMomentRdNm?: number | null;
}

export interface En1998TieBeamDelta {
  removed?: En1998TieBeamRemoval[];
  inserted?: En1998TieBeamInsertion[];
  moved?: En1998TieBeamRelocation[];
  modified?: En1998TieBeamModification[];
}

export interface En1998TieBeamRemoval {
  id: string;
  index: number;
}

export interface En1998TieBeamInsertion {
  index: number;
  row: En1998TieBeam;
}

export interface En1998TieBeamRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998TieBeamModification {
  id: string;
  patch: En1998TieBeamPatch;
}

export interface En1998TieBeamPatch {
  foundationAId?: string | null;
  foundationBId?: string | null;
  span?: string | null;
  width?: string | null;
  depth?: string | null;
  steelArea?: string | null;
  steelDesignStrength?: string | null;
  relativeDisplacement?: string | null;
  displacementLimit?: string | null;
}

export interface En1998SlopeDelta {
  removed?: En1998SlopeRemoval[];
  inserted?: En1998SlopeInsertion[];
  moved?: En1998SlopeRelocation[];
  modified?: En1998SlopeModification[];
}

export interface En1998SlopeRemoval {
  id: string;
  index: number;
}

export interface En1998SlopeInsertion {
  index: number;
  row: En1998Slope;
}

export interface En1998SlopeRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1998SlopeModification {
  id: string;
  patch: En1998SlopePatch;
}

export interface En1998SlopePatch {
  governingLayerId?: string | null;
  height?: string | null;
  angle?: string | null;
  frictionAngle?: string | null;
  cohesion?: string | null;
  topographicAmplification?: string | null;
  circleCentreX?: string | null;
  circleCentreZ?: string | null;
  circleRadius?: string | null;
}

export const parseEn1998Diff: NormWireReader<En1998Diff> = normWireObject<En1998Diff>({ annex: normWireOptional(normWireNullable(normWireLiteral("En", "De"))), site: normWireOptional(normWireNullable(parseEn1998Site)), buildings: normWireRequired(normWireAny<[En1998BuildingDelta]>(normWireRef(() => parseEn1998BuildingDelta))), bridges: normWireRequired(normWireAny<[En1998BridgeDelta]>(normWireRef(() => parseEn1998BridgeDelta))), assessments: normWireRequired(normWireAny<[En1998AssessmentDelta]>(normWireRef(() => parseEn1998AssessmentDelta))), silos: normWireRequired(normWireAny<[En1998SiloDelta]>(normWireRef(() => parseEn1998SiloDelta))), tanks: normWireRequired(normWireAny<[En1998TankDelta]>(normWireRef(() => parseEn1998TankDelta))), foundations: normWireRequired(normWireAny<[En1998FoundationDelta]>(normWireRef(() => parseEn1998FoundationDelta))), retainingWalls: normWireRequired(normWireAny<[En1998RetainingWallDelta]>(normWireRef(() => parseEn1998RetainingWallDelta))), towers: normWireRequired(normWireAny<[En1998TowerDelta]>(normWireRef(() => parseEn1998TowerDelta))), soilLayers: normWireRequired(normWireAny<[En1998SoilLayerDelta]>(normWireRef(() => parseEn1998SoilLayerDelta))), diaphragms: normWireRequired(normWireAny<[En1998DiaphragmDelta]>(normWireRef(() => parseEn1998DiaphragmDelta))), accelerograms: normWireRequired(normWireAny<[En1998AccelerogramDelta]>(normWireRef(() => parseEn1998AccelerogramDelta))), piers: normWireRequired(normWireAny<[En1998PierDelta]>(normWireRef(() => parseEn1998PierDelta))), bearings: normWireRequired(normWireAny<[En1998BearingDelta]>(normWireRef(() => parseEn1998BearingDelta))), joints: normWireRequired(normWireAny<[En1998JointDelta]>(normWireRef(() => parseEn1998JointDelta))), tieBeams: normWireRequired(normWireAny<[En1998TieBeamDelta]>(normWireRef(() => parseEn1998TieBeamDelta))), slopes: normWireRequired(normWireAny<[En1998SlopeDelta]>(normWireRef(() => parseEn1998SlopeDelta))), placements: normWireRequired(normWireAny<[En1998PlacementDelta]>(normWireRef(() => parseEn1998PlacementDelta))) });
export const parseEn1998SystemDelta: NormWireReader<En1998SystemDelta> = normWireObject<En1998SystemDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998SystemRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998SystemInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998SystemRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998SystemModification))) });
export const parseEn1998SystemRemoval: NormWireReader<En1998SystemRemoval> = normWireObject<En1998SystemRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998SystemInsertion: NormWireReader<En1998SystemInsertion> = normWireObject<En1998SystemInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998System) });
export const parseEn1998SystemRelocation: NormWireReader<En1998SystemRelocation> = normWireObject<En1998SystemRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998SystemModification: NormWireReader<En1998SystemModification> = normWireObject<En1998SystemModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998SystemPatch)) });
export const parseEn1998SystemPatch: NormWireReader<En1998SystemPatch> = normWireObject<En1998SystemPatch>({ direction: normWireOptional(normWireNullable(parseAnalysisDirection)), systemType: normWireOptional(normWireNullable(parseSystemType)), material: normWireOptional(normWireNullable(parseStructuralMaterial)), ductilityClass: normWireOptional(normWireNullable(parseDuctilityClass)), q0: normWireOptional(normWireNullable(normWireNumber)), alphaUOverAlpha1: normWireOptional(normWireNullable(normWireNumber)), kW: normWireOptional(normWireNullable(normWireNumber)), baseShearResistanceN: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1998StoreyDelta: NormWireReader<En1998StoreyDelta> = normWireObject<En1998StoreyDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998StoreyRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998StoreyInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998StoreyRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998StoreyModification))) });
export const parseEn1998StoreyRemoval: NormWireReader<En1998StoreyRemoval> = normWireObject<En1998StoreyRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998StoreyInsertion: NormWireReader<En1998StoreyInsertion> = normWireObject<En1998StoreyInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998Storey) });
export const parseEn1998StoreyRelocation: NormWireReader<En1998StoreyRelocation> = normWireObject<En1998StoreyRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998StoreyModification: NormWireReader<En1998StoreyModification> = normWireObject<En1998StoreyModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998StoreyPatch)) });
export const parseEn1998VariableActionDelta: NormWireReader<En1998VariableActionDelta> = normWireObject<En1998VariableActionDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998VariableActionRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998VariableActionInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998VariableActionRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998VariableActionModification))) });
export const parseEn1998VariableActionRemoval: NormWireReader<En1998VariableActionRemoval> = normWireObject<En1998VariableActionRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998VariableActionInsertion: NormWireReader<En1998VariableActionInsertion> = normWireObject<En1998VariableActionInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998VariableAction) });
export const parseEn1998VariableActionRelocation: NormWireReader<En1998VariableActionRelocation> = normWireObject<En1998VariableActionRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998VariableActionModification: NormWireReader<En1998VariableActionModification> = normWireObject<En1998VariableActionModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998VariableActionPatch)) });
export const parseEn1998VariableActionPatch: NormWireReader<En1998VariableActionPatch> = normWireObject<En1998VariableActionPatch>({ category: normWireOptional(normWireNullable(parseImposedCategory)), qkN: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1998StoreyPatch: NormWireReader<En1998StoreyPatch> = normWireObject<En1998StoreyPatch>({ heightM: normWireOptional(normWireNullable(normWireNumber)), permanentGkN: normWireOptional(normWireNullable(normWireNumber)), correlatedOccupancy: normWireOptional(normWireNullable(normWireBoolean)), variables: normWireRequired(normWireRef(() => parseEn1998VariableActionDelta)), stiffnessX: normWireOptional(normWireNullable(normWireNumber)), stiffnessY: normWireOptional(normWireNullable(normWireNumber)), centreOfMassXM: normWireOptional(normWireNullable(normWireNumber)), centreOfMassYM: normWireOptional(normWireNullable(normWireNumber)), centreOfStiffnessXM: normWireOptional(normWireNullable(normWireNumber)), centreOfStiffnessYM: normWireOptional(normWireNullable(normWireNumber)), driftXM: normWireOptional(normWireNullable(normWireNumber)), driftYM: normWireOptional(normWireNullable(normWireNumber)), shearResistanceN: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1998MemberDelta: NormWireReader<En1998MemberDelta> = normWireObject<En1998MemberDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998MemberRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998MemberInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998MemberRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998MemberModification))) });
export const parseEn1998MemberRemoval: NormWireReader<En1998MemberRemoval> = normWireObject<En1998MemberRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998MemberInsertion: NormWireReader<En1998MemberInsertion> = normWireObject<En1998MemberInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998Member) });
export const parseEn1998MemberRelocation: NormWireReader<En1998MemberRelocation> = normWireObject<En1998MemberRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998MemberModification: NormWireReader<En1998MemberModification> = normWireObject<En1998MemberModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998MemberPatch)) });
export const parseEn1998MemberPatch: NormWireReader<En1998MemberPatch> = normWireObject<En1998MemberPatch>({ material: normWireOptional(normWireNullable(parseStructuralMaterial)), role: normWireOptional(normWireNullable(normWireLiteral("beam", "column", "wall", "brace"))), detailingCompatibleWithQ: normWireOptional(normWireNullable(normWireBoolean)), minDimensionM: normWireOptional(normWireNullable(normWireNumber)), rho: normWireOptional(normWireNullable(normWireNumber)), rhoPrime: normWireOptional(normWireNullable(normWireNumber)), omegaWd: normWireOptional(normWireNullable(normWireNumber)), steelSectionClass: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), fastenerDiameterMm: normWireOptional(normWireNullable(normWireNumber)), timberThicknessMm: normWireOptional(normWireNullable(normWireNumber)), wallThicknessM: normWireOptional(normWireNullable(normWireNumber)), wallEffectiveHeightM: normWireOptional(normWireNullable(normWireNumber)), wallLengthM: normWireOptional(normWireNullable(normWireNumber)), wallDirectionRad: normWireOptional(normWireNullable(normWireNumber)), wallStoreyId: normWireOptional(normWireNullable(normWireString)), wallStartXM: normWireOptional(normWireNullable(normWireNumber)), wallStartYM: normWireOptional(normWireNullable(normWireNumber)), wallPoissonRatio: normWireOptional(normWireNullable(normWireNumber)), effectiveStiffnessBasis: normWireOptional(normWireNullable(normWireLiteral("simplified", "declared"))), effectiveStiffnessRatio: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1998BuildingDelta: NormWireReader<En1998BuildingDelta> = normWireObject<En1998BuildingDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998BuildingRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998BuildingInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998BuildingRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998BuildingModification))) });
export const parseEn1998BuildingRemoval: NormWireReader<En1998BuildingRemoval> = normWireObject<En1998BuildingRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998BuildingInsertion: NormWireReader<En1998BuildingInsertion> = normWireObject<En1998BuildingInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998Building) });
export const parseEn1998BuildingRelocation: NormWireReader<En1998BuildingRelocation> = normWireObject<En1998BuildingRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998BuildingModification: NormWireReader<En1998BuildingModification> = normWireObject<En1998BuildingModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998BuildingPatch)) });
export const parseEn1998BuildingPatch: NormWireReader<En1998BuildingPatch> = normWireObject<En1998BuildingPatch>({ name: normWireOptional(normWireNullable(normWireString)), planWidthM: normWireOptional(normWireNullable(normWireNumber)), planLengthM: normWireOptional(normWireNullable(normWireNumber)), systems: normWireRequired(normWireRef(() => parseEn1998SystemDelta)), storeys: normWireRequired(normWireRef(() => parseEn1998StoreyDelta)), members: normWireRequired(normWireRef(() => parseEn1998MemberDelta)), planRegular: normWireOptional(normWireNullable(normWireBoolean)), elevationRegular: normWireOptional(normWireNullable(normWireBoolean)), t1Method: normWireOptional(normWireNullable(parseT1Method)), t1GivenS: normWireOptional(normWireNullable(normWireNumber)), ct: normWireOptional(normWireNullable(normWireNumber)), driftLimitClass: normWireOptional(normWireNullable(parseDriftLimitClass)), nu: normWireOptional(normWireNullable(normWireNumber)), multipleResistingSystems: normWireOptional(normWireNullable(normWireBoolean)), claimsSimpleMasonry: normWireOptional(normWireNullable(normWireBoolean)), masonryWallAreaRatio: normWireOptional(normWireNullable(normWireNumber)), accidentalEccentricityRatio: normWireOptional(normWireNullable(normWireNumber)), setbackRatio: normWireOptional(normWireNullable(normWireNumber)), planEnvelopeGapRatio: normWireOptional(normWireNullable(normWireNumber)), jointGapM: normWireOptional(normWireNullable(normWireNumber)), nonstructuralWeightN: normWireOptional(normWireNullable(normWireNumber)), nonstructuralHeightM: normWireOptional(normWireNullable(normWireNumber)), nonstructuralPeriodS: normWireOptional(normWireNullable(normWireNumber)), nonstructuralQ: normWireOptional(normWireNullable(normWireNumber)), nonstructuralResistanceN: normWireOptional(normWireNullable(normWireNumber)), columnMomentSumNm: normWireOptional(normWireNullable(normWireNumber)), beamMomentSumNm: normWireOptional(normWireNullable(normWireNumber)), overturningBaseXM: normWireOptional(normWireNullable(normWireNumber)), overturningBaseYM: normWireOptional(normWireNullable(normWireNumber)), tieDownMomentNm: normWireOptional(normWireNullable(normWireNumber)), storeyStiffnessBasis: normWireOptional(normWireNullable(normWireLiteral("simplified", "declared"))), storeyStiffnessRatio: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1998BridgeDelta: NormWireReader<En1998BridgeDelta> = normWireObject<En1998BridgeDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998BridgeRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998BridgeInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998BridgeRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998BridgeModification))) });
export const parseEn1998BridgeRemoval: NormWireReader<En1998BridgeRemoval> = normWireObject<En1998BridgeRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998BridgeInsertion: NormWireReader<En1998BridgeInsertion> = normWireObject<En1998BridgeInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998Bridge) });
export const parseEn1998BridgeRelocation: NormWireReader<En1998BridgeRelocation> = normWireObject<En1998BridgeRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998BridgeModification: NormWireReader<En1998BridgeModification> = normWireObject<En1998BridgeModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998BridgePatch)) });
export const parseEn1998BridgePatch: NormWireReader<En1998BridgePatch> = normWireObject<En1998BridgePatch>({ periodRatio: normWireOptional(normWireNullable(normWireNumber)), fundamentalPeriodS: normWireOptional(normWireNullable(normWireNumber)), vRdN: normWireOptional(normWireNullable(normWireNumber)), bearingDRdM: normWireOptional(normWireNullable(normWireNumber)), permanentGkN: normWireOptional(normWireNullable(normWireNumber)), correlatedOccupancy: normWireOptional(normWireNullable(normWireBoolean)), variables: normWireRequired(normWireRef(() => parseEn1998VariableActionDelta)), pierHeightM: normWireOptional(normWireNullable(normWireNumber)), pierMomentRdNm: normWireOptional(normWireNullable(normWireNumber)), pierShearRdN: normWireOptional(normWireNullable(normWireNumber)), deckLengthM: normWireOptional(normWireNullable(normWireNumber)), deckEiNm2: normWireOptional(normWireNullable(normWireNumber)), importanceClass: normWireOptional(normWireNullable(normWireNumber)), verticalPeriodS: normWireOptional(normWireNullable(normWireNumber)), pierCrackedInertiaRatio: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1998AssessmentDelta: NormWireReader<En1998AssessmentDelta> = normWireObject<En1998AssessmentDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998AssessmentRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998AssessmentInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998AssessmentRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998AssessmentModification))) });
export const parseEn1998AssessmentRemoval: NormWireReader<En1998AssessmentRemoval> = normWireObject<En1998AssessmentRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998AssessmentInsertion: NormWireReader<En1998AssessmentInsertion> = normWireObject<En1998AssessmentInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998Assessment) });
export const parseEn1998AssessmentRelocation: NormWireReader<En1998AssessmentRelocation> = normWireObject<En1998AssessmentRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998AssessmentModification: NormWireReader<En1998AssessmentModification> = normWireObject<En1998AssessmentModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998AssessmentPatch)) });
export const parseEn1998AssessmentPatch: NormWireReader<En1998AssessmentPatch> = normWireObject<En1998AssessmentPatch>({ knowledgeLevel: normWireOptional(normWireNullable(parseKnowledgeLevel)), limitState: normWireOptional(normWireNullable(parseLimitState)), supportedBuildingId: normWireOptional(normWireNullable(normWireString)), rKN: normWireOptional(normWireNullable(normWireNumber)), gammaEl: normWireOptional(normWireNullable(normWireNumber)), chordRotationDemandRad: normWireOptional(normWireNullable(normWireNumber)), chordRotationYieldRad: normWireOptional(normWireNullable(normWireNumber)), chordRotationUltimateRad: normWireOptional(normWireNullable(normWireNumber)), sectionDepthM: normWireOptional(normWireNullable(normWireNumber)), sectionWidthM: normWireOptional(normWireNullable(normWireNumber)), effectiveDepthM: normWireOptional(normWireNullable(normWireNumber)), shearSpanM: normWireOptional(normWireNullable(normWireNumber)), axialForceN: normWireOptional(normWireNullable(normWireNumber)), tensionSteelAreaM2: normWireOptional(normWireNullable(normWireNumber)), compressionSteelAreaM2: normWireOptional(normWireNullable(normWireNumber)), barDiameterMm: normWireOptional(normWireNullable(normWireNumber)), steelYieldMpa: normWireOptional(normWireNullable(normWireNumber)), concreteStrengthMpa: normWireOptional(normWireNullable(normWireNumber)), stirrupLegAreaM2: normWireOptional(normWireNullable(normWireNumber)), stirrupSpacingM: normWireOptional(normWireNullable(normWireNumber)), stirrupYieldMpa: normWireOptional(normWireNullable(normWireNumber)), coreWidthM: normWireOptional(normWireNullable(normWireNumber)), coreDepthM: normWireOptional(normWireNullable(normWireNumber)), restrainedBarCount: normWireOptional(normWireNullable(normWireNumber)), restrainedBarSpacingM: normWireOptional(normWireNullable(normWireNumber)), diagonalReinforcementRatio: normWireOptional(normWireNullable(normWireNumber)), shearCrackingBeforeYield: normWireOptional(normWireNullable(normWireNumber)), shearForceDemandN: normWireOptional(normWireNullable(normWireNumber)), intermediateSteelAreaM2: normWireOptional(normWireNullable(normWireNumber)), analysisMethod: normWireOptional(normWireNullable(parseAnalysisMethodChoice)) });
export const parseEn1998SiloDelta: NormWireReader<En1998SiloDelta> = normWireObject<En1998SiloDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998SiloRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998SiloInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998SiloRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998SiloModification))) });
export const parseEn1998SiloRemoval: NormWireReader<En1998SiloRemoval> = normWireObject<En1998SiloRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998SiloInsertion: NormWireReader<En1998SiloInsertion> = normWireObject<En1998SiloInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998Silo) });
export const parseEn1998SiloRelocation: NormWireReader<En1998SiloRelocation> = normWireObject<En1998SiloRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998SiloModification: NormWireReader<En1998SiloModification> = normWireObject<En1998SiloModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998SiloPatch)) });
export const parseEn1998SiloPatch: NormWireReader<En1998SiloPatch> = normWireObject<En1998SiloPatch>({ heightM: normWireOptional(normWireNullable(normWireNumber)), radiusM: normWireOptional(normWireNullable(normWireNumber)), permanentGkN: normWireOptional(normWireNullable(normWireNumber)), contentQkN: normWireOptional(normWireNullable(normWireNumber)), contentCategory: normWireOptional(normWireNullable(parseImposedCategory)), fillingRatio: normWireOptional(normWireNullable(normWireNumber)), nRdN: normWireOptional(normWireNullable(normWireNumber)), vRdN: normWireOptional(normWireNullable(normWireNumber)), qNominal: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1998TankDelta: NormWireReader<En1998TankDelta> = normWireObject<En1998TankDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998TankRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998TankInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998TankRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998TankModification))) });
export const parseEn1998TankRemoval: NormWireReader<En1998TankRemoval> = normWireObject<En1998TankRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998TankInsertion: NormWireReader<En1998TankInsertion> = normWireObject<En1998TankInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998Tank) });
export const parseEn1998TankRelocation: NormWireReader<En1998TankRelocation> = normWireObject<En1998TankRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998TankModification: NormWireReader<En1998TankModification> = normWireObject<En1998TankModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998TankPatch)) });
export const parseEn1998TankPatch: NormWireReader<En1998TankPatch> = normWireObject<En1998TankPatch>({ heightM: normWireOptional(normWireNullable(normWireNumber)), radiusM: normWireOptional(normWireNullable(normWireNumber)), permanentGkN: normWireOptional(normWireNullable(normWireNumber)), contentQkN: normWireOptional(normWireNullable(normWireNumber)), contentCategory: normWireOptional(normWireNullable(parseImposedCategory)), fillingRatio: normWireOptional(normWireNullable(normWireNumber)), vRdN: normWireOptional(normWireNullable(normWireNumber)), shellStrengthMpa: normWireOptional(normWireNullable(normWireNumber)), shellThicknessM: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1998FoundationDelta: NormWireReader<En1998FoundationDelta> = normWireObject<En1998FoundationDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998FoundationRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998FoundationInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998FoundationRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998FoundationModification))) });
export const parseEn1998FoundationRemoval: NormWireReader<En1998FoundationRemoval> = normWireObject<En1998FoundationRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998FoundationInsertion: NormWireReader<En1998FoundationInsertion> = normWireObject<En1998FoundationInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998Foundation) });
export const parseEn1998FoundationRelocation: NormWireReader<En1998FoundationRelocation> = normWireObject<En1998FoundationRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998FoundationModification: NormWireReader<En1998FoundationModification> = normWireObject<En1998FoundationModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998FoundationPatch)) });
export const parseEn1998FoundationPatch: NormWireReader<En1998FoundationPatch> = normWireObject<En1998FoundationPatch>({ supportedBuildingId: normWireOptional(normWireNullable(normWireString)), areaM2: normWireOptional(normWireNullable(normWireNumber)), pRdPa: normWireOptional(normWireNullable(normWireNumber)), passiveResistanceN: normWireOptional(normWireNullable(normWireNumber)), kFoundation: normWireOptional(normWireNullable(normWireNumber)), kSoil: normWireOptional(normWireNullable(normWireNumber)), liqN160: normWireOptional(normWireNullable(normWireNumber)), liqFinesPercent: normWireOptional(normWireNullable(normWireNumber)), liqDepthM: normWireOptional(normWireNullable(normWireNumber)), liqWaterDepthM: normWireOptional(normWireNullable(normWireNumber)), liqUnitWeightNM3: normWireOptional(normWireNullable(normWireNumber)), liqMagnitude: normWireOptional(normWireNullable(normWireNumber)), liqLayerThicknessM: normWireOptional(normWireNullable(normWireNumber)), footingWidthM: normWireOptional(normWireNullable(normWireNumber)), bearingAxialN: normWireOptional(normWireNullable(normWireNumber)), bearingShearN: normWireOptional(normWireNullable(normWireNumber)), bearingMomentNm: normWireOptional(normWireNullable(normWireNumber)), bearingNMaxN: normWireOptional(normWireNullable(normWireNumber)), soilFrictionAngleDeg: normWireOptional(normWireNullable(normWireNumber)), pileDiameterM: normWireOptional(normWireNullable(normWireNumber)), pileLengthM: normWireOptional(normWireNullable(normWireNumber)), pileModulusPa: normWireOptional(normWireNullable(normWireNumber)), pileSubgradeModulusNM3: normWireOptional(normWireNullable(normWireNumber)), pileHeadShearN: normWireOptional(normWireNullable(normWireNumber)), pileHeadFixed: normWireOptional(normWireNullable(normWireNumber)), pileInterfaceDepthM: normWireOptional(normWireNullable(normWireNumber)), pileUpperVsMS: normWireOptional(normWireNullable(normWireNumber)), pileLowerVsMS: normWireOptional(normWireNullable(normWireNumber)), pileMomentRdNm: normWireOptional(normWireNullable(normWireNumber)), baseFrictionAngleDeg: normWireOptional(normWireNullable(normWireNumber)), basePorePressurePa: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1998RetainingWallDelta: NormWireReader<En1998RetainingWallDelta> = normWireObject<En1998RetainingWallDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998RetainingWallRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998RetainingWallInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998RetainingWallRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998RetainingWallModification))) });
export const parseEn1998RetainingWallRemoval: NormWireReader<En1998RetainingWallRemoval> = normWireObject<En1998RetainingWallRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998RetainingWallInsertion: NormWireReader<En1998RetainingWallInsertion> = normWireObject<En1998RetainingWallInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998RetainingWall) });
export const parseEn1998RetainingWallRelocation: NormWireReader<En1998RetainingWallRelocation> = normWireObject<En1998RetainingWallRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998RetainingWallModification: NormWireReader<En1998RetainingWallModification> = normWireObject<En1998RetainingWallModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998RetainingWallPatch)) });
export const parseEn1998RetainingWallPatch: NormWireReader<En1998RetainingWallPatch> = normWireObject<En1998RetainingWallPatch>({ heightM: normWireOptional(normWireNullable(normWireNumber)), phiDeg: normWireOptional(normWireNullable(normWireNumber)), soilGamma: normWireOptional(normWireNullable(normWireNumber)), r: normWireOptional(normWireNullable(normWireNumber)), hRdNPerM: normWireOptional(normWireNullable(normWireNumber)), waterHeightM: normWireOptional(normWireNullable(normWireNumber)), saturatedGamma: normWireOptional(normWireNullable(normWireNumber)), permeabilityMS: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1998TowerDelta: NormWireReader<En1998TowerDelta> = normWireObject<En1998TowerDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998TowerRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998TowerInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998TowerRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998TowerModification))) });
export const parseEn1998TowerRemoval: NormWireReader<En1998TowerRemoval> = normWireObject<En1998TowerRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998TowerInsertion: NormWireReader<En1998TowerInsertion> = normWireObject<En1998TowerInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998Tower) });
export const parseEn1998TowerRelocation: NormWireReader<En1998TowerRelocation> = normWireObject<En1998TowerRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998TowerModification: NormWireReader<En1998TowerModification> = normWireObject<En1998TowerModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998TowerPatch)) });
export const parseEn1998TowerPatch: NormWireReader<En1998TowerPatch> = normWireObject<En1998TowerPatch>({ heightM: normWireOptional(normWireNullable(normWireNumber)), mRdNm: normWireOptional(normWireNullable(normWireNumber)), isChimney: normWireOptional(normWireNullable(normWireBoolean)), qNominal: normWireOptional(normWireNullable(normWireNumber)), permanentGkN: normWireOptional(normWireNullable(normWireNumber)), correlatedOccupancy: normWireOptional(normWireNullable(normWireBoolean)), variables: normWireRequired(normWireRef(() => parseEn1998VariableActionDelta)), eiNm2: normWireOptional(normWireNullable(normWireNumber)), baseRotationalStiffnessNm: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1998PlacementDelta: NormWireReader<En1998PlacementDelta> = normWireObject<En1998PlacementDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998PlacementRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998PlacementInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998PlacementRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998PlacementModification))) });
export const parseEn1998PlacementRemoval: NormWireReader<En1998PlacementRemoval> = normWireObject<En1998PlacementRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998PlacementInsertion: NormWireReader<En1998PlacementInsertion> = normWireObject<En1998PlacementInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998Placement) });
export const parseEn1998PlacementRelocation: NormWireReader<En1998PlacementRelocation> = normWireObject<En1998PlacementRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998PlacementModification: NormWireReader<En1998PlacementModification> = normWireObject<En1998PlacementModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998PlacementPatch)) });
export const parseEn1998PlacementPatch: NormWireReader<En1998PlacementPatch> = normWireObject<En1998PlacementPatch>({ x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), z: normWireOptional(normWireNullable(normWireNumber)), yaw: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1998SoilLayerDelta: NormWireReader<En1998SoilLayerDelta> = normWireObject<En1998SoilLayerDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998SoilLayerRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998SoilLayerInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998SoilLayerRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998SoilLayerModification))) });
export const parseEn1998SoilLayerRemoval: NormWireReader<En1998SoilLayerRemoval> = normWireObject<En1998SoilLayerRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998SoilLayerInsertion: NormWireReader<En1998SoilLayerInsertion> = normWireObject<En1998SoilLayerInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998SoilLayer) });
export const parseEn1998SoilLayerRelocation: NormWireReader<En1998SoilLayerRelocation> = normWireObject<En1998SoilLayerRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998SoilLayerModification: NormWireReader<En1998SoilLayerModification> = normWireObject<En1998SoilLayerModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998SoilLayerPatch)) });
export const parseEn1998SoilLayerPatch: NormWireReader<En1998SoilLayerPatch> = normWireObject<En1998SoilLayerPatch>({ foundationId: normWireOptional(normWireNullable(normWireString)), thicknessM: normWireOptional(normWireNullable(normWireNumber)), unitWeightNM3: normWireOptional(normWireNullable(normWireNumber)), shearWaveVelocityMS: normWireOptional(normWireNullable(normWireNumber)), n160: normWireOptional(normWireNullable(normWireNumber)), finesPercent: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1998DiaphragmDelta: NormWireReader<En1998DiaphragmDelta> = normWireObject<En1998DiaphragmDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998DiaphragmRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998DiaphragmInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998DiaphragmRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998DiaphragmModification))) });
export const parseEn1998DiaphragmRemoval: NormWireReader<En1998DiaphragmRemoval> = normWireObject<En1998DiaphragmRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998DiaphragmInsertion: NormWireReader<En1998DiaphragmInsertion> = normWireObject<En1998DiaphragmInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998Diaphragm) });
export const parseEn1998DiaphragmRelocation: NormWireReader<En1998DiaphragmRelocation> = normWireObject<En1998DiaphragmRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998DiaphragmModification: NormWireReader<En1998DiaphragmModification> = normWireObject<En1998DiaphragmModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998DiaphragmPatch)) });
export const parseEn1998DiaphragmPatch: NormWireReader<En1998DiaphragmPatch> = normWireObject<En1998DiaphragmPatch>({ buildingId: normWireOptional(normWireNullable(normWireString)), storeyId: normWireOptional(normWireNullable(normWireString)), material: normWireOptional(normWireNullable(parseDiaphragmMaterial)), thicknessM: normWireOptional(normWireNullable(normWireNumber)), spanM: normWireOptional(normWireNullable(normWireNumber)), depthM: normWireOptional(normWireNullable(normWireNumber)), openingWidthM: normWireOptional(normWireNullable(normWireNumber)), shearStrengthMpa: normWireOptional(normWireNullable(normWireNumber)), chordAreaM2: normWireOptional(normWireNullable(normWireNumber)), chordStrengthMpa: normWireOptional(normWireNullable(normWireNumber)), gammaD: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1998AccelerogramDelta: NormWireReader<En1998AccelerogramDelta> = normWireObject<En1998AccelerogramDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998AccelerogramRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998AccelerogramInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998AccelerogramRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998AccelerogramModification))) });
export const parseEn1998AccelerogramRemoval: NormWireReader<En1998AccelerogramRemoval> = normWireObject<En1998AccelerogramRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998AccelerogramInsertion: NormWireReader<En1998AccelerogramInsertion> = normWireObject<En1998AccelerogramInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998Accelerogram) });
export const parseEn1998AccelerogramRelocation: NormWireReader<En1998AccelerogramRelocation> = normWireObject<En1998AccelerogramRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998AccelerogramModification: NormWireReader<En1998AccelerogramModification> = normWireObject<En1998AccelerogramModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998AccelerogramPatch)) });
export const parseEn1998AccelerogramPatch: NormWireReader<En1998AccelerogramPatch> = normWireObject<En1998AccelerogramPatch>({ buildingId: normWireOptional(normWireNullable(normWireString)), label: normWireOptional(normWireNullable(normWireString)), samples: normWireOptional(normWireNullable(normWireString)), dtS: normWireOptional(normWireNullable(normWireNumber)), scaleFactor: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1998PierDelta: NormWireReader<En1998PierDelta> = normWireObject<En1998PierDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998PierRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998PierInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998PierRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998PierModification))) });
export const parseEn1998PierRemoval: NormWireReader<En1998PierRemoval> = normWireObject<En1998PierRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998PierInsertion: NormWireReader<En1998PierInsertion> = normWireObject<En1998PierInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998Pier) });
export const parseEn1998PierRelocation: NormWireReader<En1998PierRelocation> = normWireObject<En1998PierRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998PierModification: NormWireReader<En1998PierModification> = normWireObject<En1998PierModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998PierPatch)) });
export const parseEn1998PierPatch: NormWireReader<En1998PierPatch> = normWireObject<En1998PierPatch>({ bridgeId: normWireOptional(normWireNullable(normWireString)), xM: normWireOptional(normWireNullable(normWireNumber)), heightM: normWireOptional(normWireNullable(normWireNumber)), sectionBM: normWireOptional(normWireNullable(normWireNumber)), sectionHM: normWireOptional(normWireNullable(normWireNumber)), modulusMpa: normWireOptional(normWireNullable(normWireNumber)), momentRdNm: normWireOptional(normWireNullable(normWireNumber)), shearRdN: normWireOptional(normWireNullable(normWireNumber)), reinforcementRatio: normWireOptional(normWireNullable(normWireString)), coverM: normWireOptional(normWireNullable(normWireString)), concreteFckMpa: normWireOptional(normWireNullable(normWireString)), steelFykMpa: normWireOptional(normWireNullable(normWireString)), axialLoadN: normWireOptional(normWireNullable(normWireString)), stiffnessBasis: normWireOptional(normWireNullable(parsePierStiffnessBasis)) });
export const parseEn1998BearingDelta: NormWireReader<En1998BearingDelta> = normWireObject<En1998BearingDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1998BearingRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1998BearingInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1998BearingRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1998BearingModification))) });
export const parseEn1998BearingRemoval: NormWireReader<En1998BearingRemoval> = normWireObject<En1998BearingRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998BearingInsertion: NormWireReader<En1998BearingInsertion> = normWireObject<En1998BearingInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998Bearing) });
export const parseEn1998BearingRelocation: NormWireReader<En1998BearingRelocation> = normWireObject<En1998BearingRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998BearingModification: NormWireReader<En1998BearingModification> = normWireObject<En1998BearingModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998BearingPatch)) });
export const parseEn1998BearingPatch: NormWireReader<En1998BearingPatch> = normWireObject<En1998BearingPatch>({ bridgeId: normWireOptional(normWireNullable(normWireString)), pierId: normWireOptional(normWireNullable(normWireString)), device: normWireOptional(normWireNullable(parseBearingDevice)), xM: normWireOptional(normWireNullable(normWireNumber)), stiffnessNPerM: normWireOptional(normWireNullable(normWireNumber)), dRdM: normWireOptional(normWireNullable(normWireNumber)), seatLengthM: normWireOptional(normWireNullable(normWireNumber)), minSupportLengthM: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1998JointDelta: NormWireReader<En1998JointDelta> = normWireObject<En1998JointDelta>({ removed: normWireOptional(normWireArray(normWireRef(() => parseEn1998JointRemoval))), inserted: normWireOptional(normWireArray(normWireRef(() => parseEn1998JointInsertion))), moved: normWireOptional(normWireArray(normWireRef(() => parseEn1998JointRelocation))), modified: normWireOptional(normWireArray(normWireRef(() => parseEn1998JointModification))) });
export const parseEn1998JointRemoval: NormWireReader<En1998JointRemoval> = normWireObject<En1998JointRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998JointInsertion: NormWireReader<En1998JointInsertion> = normWireObject<En1998JointInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998Joint) });
export const parseEn1998JointRelocation: NormWireReader<En1998JointRelocation> = normWireObject<En1998JointRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998JointModification: NormWireReader<En1998JointModification> = normWireObject<En1998JointModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998JointPatch)) });
export const parseEn1998JointPatch: NormWireReader<En1998JointPatch> = normWireObject<En1998JointPatch>({ buildingId: normWireOptional(normWireNullable(normWireString)), storeyId: normWireOptional(normWireNullable(normWireString)), columnId: normWireOptional(normWireNullable(normWireString)), beamId: normWireOptional(normWireNullable(normWireString)), xM: normWireOptional(normWireNullable(normWireNumber)), yM: normWireOptional(normWireNullable(normWireNumber)), columnMomentRdNm: normWireOptional(normWireNullable(normWireNumber)), beamMomentRdNm: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1998TieBeamDelta: NormWireReader<En1998TieBeamDelta> = normWireObject<En1998TieBeamDelta>({ removed: normWireOptional(normWireArray(normWireRef(() => parseEn1998TieBeamRemoval))), inserted: normWireOptional(normWireArray(normWireRef(() => parseEn1998TieBeamInsertion))), moved: normWireOptional(normWireArray(normWireRef(() => parseEn1998TieBeamRelocation))), modified: normWireOptional(normWireArray(normWireRef(() => parseEn1998TieBeamModification))) });
export const parseEn1998TieBeamRemoval: NormWireReader<En1998TieBeamRemoval> = normWireObject<En1998TieBeamRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998TieBeamInsertion: NormWireReader<En1998TieBeamInsertion> = normWireObject<En1998TieBeamInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998TieBeam) });
export const parseEn1998TieBeamRelocation: NormWireReader<En1998TieBeamRelocation> = normWireObject<En1998TieBeamRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998TieBeamModification: NormWireReader<En1998TieBeamModification> = normWireObject<En1998TieBeamModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998TieBeamPatch)) });
export const parseEn1998TieBeamPatch: NormWireReader<En1998TieBeamPatch> = normWireObject<En1998TieBeamPatch>({ foundationAId: normWireOptional(normWireNullable(normWireString)), foundationBId: normWireOptional(normWireNullable(normWireString)), span: normWireOptional(normWireNullable(normWireString)), width: normWireOptional(normWireNullable(normWireString)), depth: normWireOptional(normWireNullable(normWireString)), steelArea: normWireOptional(normWireNullable(normWireString)), steelDesignStrength: normWireOptional(normWireNullable(normWireString)), relativeDisplacement: normWireOptional(normWireNullable(normWireString)), displacementLimit: normWireOptional(normWireNullable(normWireString)) });
export const parseEn1998SlopeDelta: NormWireReader<En1998SlopeDelta> = normWireObject<En1998SlopeDelta>({ removed: normWireOptional(normWireArray(normWireRef(() => parseEn1998SlopeRemoval))), inserted: normWireOptional(normWireArray(normWireRef(() => parseEn1998SlopeInsertion))), moved: normWireOptional(normWireArray(normWireRef(() => parseEn1998SlopeRelocation))), modified: normWireOptional(normWireArray(normWireRef(() => parseEn1998SlopeModification))) });
export const parseEn1998SlopeRemoval: NormWireReader<En1998SlopeRemoval> = normWireObject<En1998SlopeRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998SlopeInsertion: NormWireReader<En1998SlopeInsertion> = normWireObject<En1998SlopeInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseEn1998Slope) });
export const parseEn1998SlopeRelocation: NormWireReader<En1998SlopeRelocation> = normWireObject<En1998SlopeRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1998SlopeModification: NormWireReader<En1998SlopeModification> = normWireObject<En1998SlopeModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1998SlopePatch)) });
export const parseEn1998SlopePatch: NormWireReader<En1998SlopePatch> = normWireObject<En1998SlopePatch>({ governingLayerId: normWireOptional(normWireNullable(normWireString)), height: normWireOptional(normWireNullable(normWireString)), angle: normWireOptional(normWireNullable(normWireString)), frictionAngle: normWireOptional(normWireNullable(normWireString)), cohesion: normWireOptional(normWireNullable(normWireString)), topographicAmplification: normWireOptional(normWireNullable(normWireString)), circleCentreX: normWireOptional(normWireNullable(normWireString)), circleCentreZ: normWireOptional(normWireNullable(normWireString)), circleRadius: normWireOptional(normWireNullable(normWireString)) });
