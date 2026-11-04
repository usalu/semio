/** 🧬️ `En1998Artifact` wire twin: the artifact document across its state lanes, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface En1998Artifact {
  /** @state artifact */
  annex: string;
  /** @state artifact */
  site: En1998Site;
  /** @state artifact */
  buildings: En1998Building[];
  /** @state artifact */
  bridges: En1998Bridge[];
  /** @state artifact */
  assessments: En1998Assessment[];
  /** @state artifact */
  silos: En1998Silo[];
  /** @state artifact */
  tanks: En1998Tank[];
  /** @state artifact */
  foundations: En1998Foundation[];
  /** @state artifact */
  retainingWalls: En1998RetainingWall[];
  /** @state artifact */
  towers: En1998Tower[];
  /** @state artifact */
  soilLayers: En1998SoilLayer[];
  /** @state artifact */
  diaphragms: En1998Diaphragm[];
  /** @state artifact */
  accelerograms: En1998Accelerogram[];
  /** @state artifact */
  piers: En1998Pier[];
  /** @state artifact */
  bearings: En1998Bearing[];
  /** @state artifact */
  joints: En1998Joint[];
  /** @state artifact */
  placements: En1998Placement[];
  /** @state artifact */
  tieBeams: En1998TieBeam[];
  /** @state artifact */
  slopes: En1998Slope[];
}

export type EnGroundType = "a" | "b" | "c" | "d" | "e";

export type EnSpectrumType = "type1" | "type2";

export type ImportanceClass = "i" | "ii" | "iii" | "iv";

export interface En1998Site {
  sApR: number;
  aGr: number;
  deGroundCombo: DeGroundCombo;
  enGroundType: EnGroundType;
  enSpectrumType: EnSpectrumType;
  importanceClass: ImportanceClass;
  vS30MS: number;
  nSpt: number;
  cUKpa: number;
  softClayThicknessM: number;
  softClayPi: number;
  liquefiableLayer: boolean;
  alluviumThicknessM: number;
  alluviumVSMS: number;
  belowAlluviumVSMS: number;
}

export type AnalysisDirection = "x" | "y";

export type SystemType = "frame" | "wall" | "dual" | "masonry";

export type StructuralMaterial = "rc" | "steel" | "composite" | "timber" | "masonry";

export type DuctilityClass = "dcl" | "dcm" | "dch";

export interface En1998System {
  id: string;
  direction: AnalysisDirection;
  systemType: SystemType;
  material: StructuralMaterial;
  ductilityClass: DuctilityClass;
  q0: number;
  alphaUOverAlpha1: number;
  kW: number;
  baseShearResistanceN: number;
}

export interface En1998Storey {
  id: string;
  heightM: number;
  permanentGkN: number;
  correlatedOccupancy: boolean;
  variables: En1998VariableAction[];
  stiffnessX: number;
  stiffnessY: number;
  centreOfMassXM: number;
  centreOfMassYM: number;
  centreOfStiffnessXM: number;
  centreOfStiffnessYM: number;
  driftXM: number;
  driftYM: number;
  shearResistanceN: number;
}

export interface En1998Member {
  id: string;
  material: StructuralMaterial;
  role: MemberKind;
  detailingCompatibleWithQ: boolean;
  minDimensionM: number;
  rho: number;
  rhoPrime: number;
  omegaWd: number;
  steelSectionClass: number;
  fastenerDiameterMm: number;
  timberThicknessMm: number;
  wallThicknessM: number;
  wallEffectiveHeightM: number;
  wallLengthM: number;
  wallDirectionRad: number;
  wallStoreyId: string;
  wallStartXM: number;
  wallStartYM: number;
  wallPoissonRatio: number;
  effectiveStiffnessBasis: StiffnessBasis;
  effectiveStiffnessRatio: number;
}

export type T1Method = "ct" | "rayleigh" | "given";

export type DriftLimitClass = "brittle" | "ductile" | "isolated";

export interface En1998Building {
  id: string;
  name: string;
  planWidthM: number;
  planLengthM: number;
  systems: En1998System[];
  storeys: En1998Storey[];
  members: En1998Member[];
  planRegular: boolean;
  elevationRegular: boolean;
  t1Method: T1Method;
  t1GivenS: number;
  ct: number;
  driftLimitClass: DriftLimitClass;
  nu: number;
  multipleResistingSystems: boolean;
  claimsSimpleMasonry: boolean;
  masonryWallAreaRatio: number;
  accidentalEccentricityRatio: number;
  setbackRatio: number;
  planEnvelopeGapRatio: number;
  jointGapM: number;
  nonstructuralWeightN: number;
  nonstructuralHeightM: number;
  nonstructuralPeriodS: number;
  nonstructuralQ: number;
  nonstructuralResistanceN: number;
  columnMomentSumNm: number;
  beamMomentSumNm: number;
  overturningBaseXM: number;
  overturningBaseYM: number;
  tieDownMomentNm: number;
  storeyStiffnessBasis: StiffnessBasis;
  storeyStiffnessRatio: number;
}

export interface En1998Bridge {
  id: string;
  periodRatio: number;
  fundamentalPeriodS: number;
  vRdN: number;
  bearingDRdM: number;
  permanentGkN: number;
  correlatedOccupancy: boolean;
  variables: En1998VariableAction[];
  pierHeightM: number;
  pierMomentRdNm: number;
  pierShearRdN: number;
  deckLengthM: number;
  deckEiNm2: number;
  importanceClass: number;
  verticalPeriodS: number;
  pierCrackedInertiaRatio: number;
}

export type LimitState = "nc" | "sd" | "dl";

export type KnowledgeLevel = "kl1" | "kl2" | "kl3";

export interface En1998Assessment {
  id: string;
  knowledgeLevel: KnowledgeLevel;
  limitState: LimitState;
  supportedBuildingId: string;
  rKN: number;
  gammaEl: number;
  chordRotationDemandRad: number;
  chordRotationYieldRad: number;
  chordRotationUltimateRad: number;
  sectionDepthM: number;
  sectionWidthM: number;
  effectiveDepthM: number;
  shearSpanM: number;
  axialForceN: number;
  tensionSteelAreaM2: number;
  compressionSteelAreaM2: number;
  barDiameterMm: number;
  steelYieldMpa: number;
  concreteStrengthMpa: number;
  stirrupLegAreaM2: number;
  stirrupSpacingM: number;
  stirrupYieldMpa: number;
  coreWidthM: number;
  coreDepthM: number;
  restrainedBarCount: number;
  restrainedBarSpacingM: number;
  diagonalReinforcementRatio: number;
  shearCrackingBeforeYield: number;
  shearForceDemandN: number;
  intermediateSteelAreaM2: number;
  analysisMethod: AnalysisMethodChoice;
}

export interface En1998Silo {
  id: string;
  heightM: number;
  radiusM: number;
  permanentGkN: number;
  contentQkN: number;
  contentCategory: ImposedCategory;
  fillingRatio: number;
  nRdN: number;
  vRdN: number;
  qNominal: number;
}

export interface En1998Tank {
  id: string;
  heightM: number;
  radiusM: number;
  permanentGkN: number;
  contentQkN: number;
  contentCategory: ImposedCategory;
  fillingRatio: number;
  vRdN: number;
  shellStrengthMpa: number;
  shellThicknessM: number;
}

export interface En1998Foundation {
  id: string;
  supportedBuildingId: string;
  areaM2: number;
  pRdPa: number;
  passiveResistanceN: number;
  kFoundation: number;
  kSoil: number;
  liqN160: number;
  liqFinesPercent: number;
  liqDepthM: number;
  liqWaterDepthM: number;
  liqUnitWeightNM3: number;
  liqMagnitude: number;
  liqLayerThicknessM: number;
  footingWidthM: number;
  bearingAxialN: number;
  bearingShearN: number;
  bearingMomentNm: number;
  bearingNMaxN: number;
  soilFrictionAngleDeg: number;
  pileDiameterM: number;
  pileLengthM: number;
  pileModulusPa: number;
  pileSubgradeModulusNM3: number;
  pileHeadShearN: number;
  pileHeadFixed: number;
  pileInterfaceDepthM: number;
  pileUpperVsMS: number;
  pileLowerVsMS: number;
  pileMomentRdNm: number;
  baseFrictionAngleDeg: number;
  basePorePressurePa: number;
}

export interface En1998RetainingWall {
  id: string;
  heightM: number;
  phiDeg: number;
  soilGamma: number;
  r: number;
  hRdNPerM: number;
  waterHeightM: number;
  saturatedGamma: number;
  permeabilityMS: number;
}

export interface En1998SoilLayer {
  id: string;
  foundationId: string;
  thicknessM: number;
  unitWeightNM3: number;
  shearWaveVelocityMS: number;
  n160: number;
  finesPercent: number;
}

export type DiaphragmMaterial = "concrete" | "timber" | "steelDeck";

export interface En1998Diaphragm {
  id: string;
  buildingId: string;
  storeyId: string;
  material: DiaphragmMaterial;
  thicknessM: number;
  spanM: number;
  depthM: number;
  openingWidthM: number;
  shearStrengthMpa: number;
  chordAreaM2: number;
  chordStrengthMpa: number;
  gammaD: number;
}

export interface En1998Accelerogram {
  id: string;
  buildingId: string;
  label: string;
  samples: string;
  dtS: number;
  scaleFactor: number;
}

export type PierStiffnessBasis = "declared" | "computed";

export interface En1998Pier {
  id: string;
  bridgeId: string;
  xM: number;
  heightM: number;
  sectionBM: number;
  sectionHM: number;
  modulusMpa: number;
  momentRdNm: number;
  shearRdN: number;
  reinforcementRatio: number;
  coverM: number;
  concreteFckMpa: number;
  steelFykMpa: number;
  axialLoadN: number;
  stiffnessBasis: PierStiffnessBasis;
}

export type BearingDevice = "fixed" | "elastomeric" | "sliding" | "isolator";

export interface En1998Bearing {
  id: string;
  bridgeId: string;
  pierId: string;
  device: BearingDevice;
  xM: number;
  stiffnessNPerM: number;
  dRdM: number;
  seatLengthM: number;
  minSupportLengthM: number;
}

export interface En1998Joint {
  id: string;
  buildingId: string;
  storeyId: string;
  columnId: string;
  beamId: string;
  xM: number;
  yM: number;
  columnMomentRdNm: number;
  beamMomentRdNm: number;
}

export interface En1998Tower {
  id: string;
  heightM: number;
  mRdNm: number;
  isChimney: boolean;
  qNominal: number;
  permanentGkN: number;
  correlatedOccupancy: boolean;
  variables: En1998VariableAction[];
  eiNm2: number;
  baseRotationalStiffnessNm: number;
}

export interface En1998Placement {
  id: string;
  x: number;
  y: number;
  z: number;
  yaw: number;
}

export type ImposedCategory = "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "snow" | "wind";

export interface En1998VariableAction {
  id: string;
  category: ImposedCategory;
  qkN: number;
}

export type StiffnessBasis = "simplified" | "declared";

export type MemberKind = "beam" | "column" | "wall" | "brace";

export type DeGroundCombo = "A-R" | "B-R" | "C-R" | "B-T" | "C-T" | "C-S";

export interface En1998TieBeam {
  id: string;
  foundationAId: string;
  foundationBId: string;
  span: number;
  width: number;
  depth: number;
  steelArea: number;
  steelDesignStrength: number;
  relativeDisplacement: number;
  displacementLimit: number;
}

export interface En1998Slope {
  id: string;
  governingLayerId: string;
  height: number;
  angle: number;
  frictionAngle: number;
  cohesion: number;
  topographicAmplification: number;
  circleCentreX: number;
  circleCentreZ: number;
  circleRadius: number;
}

export type AnalysisMethodChoice = "derived" | "lf" | "mrs" | "pushover" | "nlth";

export const parseEn1998Artifact: NormWireReader<En1998Artifact> = normWireObject<En1998Artifact>({ annex: normWireRequired(normWireString), site: normWireRequired(normWireRef(() => parseEn1998Site)), buildings: normWireRequired(normWireArray(normWireRef(() => parseEn1998Building))), bridges: normWireRequired(normWireArray(normWireRef(() => parseEn1998Bridge))), assessments: normWireRequired(normWireArray(normWireRef(() => parseEn1998Assessment))), silos: normWireRequired(normWireArray(normWireRef(() => parseEn1998Silo))), tanks: normWireRequired(normWireArray(normWireRef(() => parseEn1998Tank))), foundations: normWireRequired(normWireArray(normWireRef(() => parseEn1998Foundation))), retainingWalls: normWireRequired(normWireArray(normWireRef(() => parseEn1998RetainingWall))), towers: normWireRequired(normWireArray(normWireRef(() => parseEn1998Tower))), soilLayers: normWireRequired(normWireArray(normWireRef(() => parseEn1998SoilLayer))), diaphragms: normWireRequired(normWireArray(normWireRef(() => parseEn1998Diaphragm))), accelerograms: normWireRequired(normWireArray(normWireRef(() => parseEn1998Accelerogram))), piers: normWireRequired(normWireArray(normWireRef(() => parseEn1998Pier))), bearings: normWireRequired(normWireArray(normWireRef(() => parseEn1998Bearing))), joints: normWireRequired(normWireArray(normWireRef(() => parseEn1998Joint))), placements: normWireRequired(normWireArray(normWireRef(() => parseEn1998Placement))), tieBeams: normWireRequired(normWireArray(normWireRef(() => parseEn1998TieBeam))), slopes: normWireRequired(normWireArray(normWireRef(() => parseEn1998Slope))) });
export const parseEnGroundType: NormWireReader<EnGroundType> = normWireLiteral("a", "b", "c", "d", "e");
export const parseEnSpectrumType: NormWireReader<EnSpectrumType> = normWireLiteral("type1", "type2");
export const parseImportanceClass: NormWireReader<ImportanceClass> = normWireLiteral("i", "ii", "iii", "iv");
export const parseEn1998Site: NormWireReader<En1998Site> = normWireObject<En1998Site>({ sApR: normWireRequired(normWireNumber), aGr: normWireRequired(normWireNumber), deGroundCombo: normWireRequired(normWireRef(() => parseDeGroundCombo)), enGroundType: normWireRequired(normWireRef(() => parseEnGroundType)), enSpectrumType: normWireRequired(normWireRef(() => parseEnSpectrumType)), importanceClass: normWireRequired(normWireRef(() => parseImportanceClass)), vS30MS: normWireRequired(normWireNumber), nSpt: normWireRequired(normWireNumber), cUKpa: normWireRequired(normWireNumber), softClayThicknessM: normWireRequired(normWireNumber), softClayPi: normWireRequired(normWireNumber), liquefiableLayer: normWireRequired(normWireBoolean), alluviumThicknessM: normWireRequired(normWireNumber), alluviumVSMS: normWireRequired(normWireNumber), belowAlluviumVSMS: normWireRequired(normWireNumber) });
export const parseAnalysisDirection: NormWireReader<AnalysisDirection> = normWireLiteral("x", "y");
export const parseSystemType: NormWireReader<SystemType> = normWireLiteral("frame", "wall", "dual", "masonry");
export const parseStructuralMaterial: NormWireReader<StructuralMaterial> = normWireLiteral("rc", "steel", "composite", "timber", "masonry");
export const parseDuctilityClass: NormWireReader<DuctilityClass> = normWireLiteral("dcl", "dcm", "dch");
export const parseEn1998System: NormWireReader<En1998System> = normWireObject<En1998System>({ id: normWireRequired(normWireString), direction: normWireRequired(normWireRef(() => parseAnalysisDirection)), systemType: normWireRequired(normWireRef(() => parseSystemType)), material: normWireRequired(normWireRef(() => parseStructuralMaterial)), ductilityClass: normWireRequired(normWireRef(() => parseDuctilityClass)), q0: normWireRequired(normWireNumber), alphaUOverAlpha1: normWireRequired(normWireNumber), kW: normWireRequired(normWireNumber), baseShearResistanceN: normWireRequired(normWireNumber) });
export const parseEn1998Storey: NormWireReader<En1998Storey> = normWireObject<En1998Storey>({ id: normWireRequired(normWireString), heightM: normWireRequired(normWireNumber), permanentGkN: normWireRequired(normWireNumber), correlatedOccupancy: normWireRequired(normWireBoolean), variables: normWireRequired(normWireArray(normWireRef(() => parseEn1998VariableAction))), stiffnessX: normWireRequired(normWireNumber), stiffnessY: normWireRequired(normWireNumber), centreOfMassXM: normWireRequired(normWireNumber), centreOfMassYM: normWireRequired(normWireNumber), centreOfStiffnessXM: normWireRequired(normWireNumber), centreOfStiffnessYM: normWireRequired(normWireNumber), driftXM: normWireRequired(normWireNumber), driftYM: normWireRequired(normWireNumber), shearResistanceN: normWireRequired(normWireNumber) });
export const parseEn1998Member: NormWireReader<En1998Member> = normWireObject<En1998Member>({ id: normWireRequired(normWireString), material: normWireRequired(normWireRef(() => parseStructuralMaterial)), role: normWireRequired(normWireRef(() => parseMemberKind)), detailingCompatibleWithQ: normWireRequired(normWireBoolean), minDimensionM: normWireRequired(normWireNumber), rho: normWireRequired(normWireNumber), rhoPrime: normWireRequired(normWireNumber), omegaWd: normWireRequired(normWireNumber), steelSectionClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), fastenerDiameterMm: normWireRequired(normWireNumber), timberThicknessMm: normWireRequired(normWireNumber), wallThicknessM: normWireRequired(normWireNumber), wallEffectiveHeightM: normWireRequired(normWireNumber), wallLengthM: normWireRequired(normWireNumber), wallDirectionRad: normWireRequired(normWireNumber), wallStoreyId: normWireRequired(normWireString), wallStartXM: normWireRequired(normWireNumber), wallStartYM: normWireRequired(normWireNumber), wallPoissonRatio: normWireRequired(normWireNumber), effectiveStiffnessBasis: normWireRequired(normWireRef(() => parseStiffnessBasis)), effectiveStiffnessRatio: normWireRequired(normWireNumber) });
export const parseT1Method: NormWireReader<T1Method> = normWireLiteral("ct", "rayleigh", "given");
export const parseDriftLimitClass: NormWireReader<DriftLimitClass> = normWireLiteral("brittle", "ductile", "isolated");
export const parseEn1998Building: NormWireReader<En1998Building> = normWireObject<En1998Building>({ id: normWireRequired(normWireString), name: normWireRequired(normWireString), planWidthM: normWireRequired(normWireNumber), planLengthM: normWireRequired(normWireNumber), systems: normWireRequired(normWireArray(normWireRef(() => parseEn1998System))), storeys: normWireRequired(normWireArray(normWireRef(() => parseEn1998Storey))), members: normWireRequired(normWireArray(normWireRef(() => parseEn1998Member))), planRegular: normWireRequired(normWireBoolean), elevationRegular: normWireRequired(normWireBoolean), t1Method: normWireRequired(normWireRef(() => parseT1Method)), t1GivenS: normWireRequired(normWireNumber), ct: normWireRequired(normWireNumber), driftLimitClass: normWireRequired(normWireRef(() => parseDriftLimitClass)), nu: normWireRequired(normWireNumber), multipleResistingSystems: normWireRequired(normWireBoolean), claimsSimpleMasonry: normWireRequired(normWireBoolean), masonryWallAreaRatio: normWireRequired(normWireNumber), accidentalEccentricityRatio: normWireRequired(normWireNumber), setbackRatio: normWireRequired(normWireNumber), planEnvelopeGapRatio: normWireRequired(normWireNumber), jointGapM: normWireRequired(normWireNumber), nonstructuralWeightN: normWireRequired(normWireNumber), nonstructuralHeightM: normWireRequired(normWireNumber), nonstructuralPeriodS: normWireRequired(normWireNumber), nonstructuralQ: normWireRequired(normWireNumber), nonstructuralResistanceN: normWireRequired(normWireNumber), columnMomentSumNm: normWireRequired(normWireNumber), beamMomentSumNm: normWireRequired(normWireNumber), overturningBaseXM: normWireRequired(normWireNumber), overturningBaseYM: normWireRequired(normWireNumber), tieDownMomentNm: normWireRequired(normWireNumber), storeyStiffnessBasis: normWireRequired(normWireRef(() => parseStiffnessBasis)), storeyStiffnessRatio: normWireRequired(normWireNumber) });
export const parseEn1998Bridge: NormWireReader<En1998Bridge> = normWireObject<En1998Bridge>({ id: normWireRequired(normWireString), periodRatio: normWireRequired(normWireNumber), fundamentalPeriodS: normWireRequired(normWireNumber), vRdN: normWireRequired(normWireNumber), bearingDRdM: normWireRequired(normWireNumber), permanentGkN: normWireRequired(normWireNumber), correlatedOccupancy: normWireRequired(normWireBoolean), variables: normWireRequired(normWireArray(normWireRef(() => parseEn1998VariableAction))), pierHeightM: normWireRequired(normWireNumber), pierMomentRdNm: normWireRequired(normWireNumber), pierShearRdN: normWireRequired(normWireNumber), deckLengthM: normWireRequired(normWireNumber), deckEiNm2: normWireRequired(normWireNumber), importanceClass: normWireRequired(normWireNumber), verticalPeriodS: normWireRequired(normWireNumber), pierCrackedInertiaRatio: normWireRequired(normWireNumber) });
export const parseLimitState: NormWireReader<LimitState> = normWireLiteral("nc", "sd", "dl");
export const parseKnowledgeLevel: NormWireReader<KnowledgeLevel> = normWireLiteral("kl1", "kl2", "kl3");
export const parseEn1998Assessment: NormWireReader<En1998Assessment> = normWireObject<En1998Assessment>({ id: normWireRequired(normWireString), knowledgeLevel: normWireRequired(normWireRef(() => parseKnowledgeLevel)), limitState: normWireRequired(normWireRef(() => parseLimitState)), supportedBuildingId: normWireRequired(normWireString), rKN: normWireRequired(normWireNumber), gammaEl: normWireRequired(normWireNumber), chordRotationDemandRad: normWireRequired(normWireNumber), chordRotationYieldRad: normWireRequired(normWireNumber), chordRotationUltimateRad: normWireRequired(normWireNumber), sectionDepthM: normWireRequired(normWireNumber), sectionWidthM: normWireRequired(normWireNumber), effectiveDepthM: normWireRequired(normWireNumber), shearSpanM: normWireRequired(normWireNumber), axialForceN: normWireRequired(normWireNumber), tensionSteelAreaM2: normWireRequired(normWireNumber), compressionSteelAreaM2: normWireRequired(normWireNumber), barDiameterMm: normWireRequired(normWireNumber), steelYieldMpa: normWireRequired(normWireNumber), concreteStrengthMpa: normWireRequired(normWireNumber), stirrupLegAreaM2: normWireRequired(normWireNumber), stirrupSpacingM: normWireRequired(normWireNumber), stirrupYieldMpa: normWireRequired(normWireNumber), coreWidthM: normWireRequired(normWireNumber), coreDepthM: normWireRequired(normWireNumber), restrainedBarCount: normWireRequired(normWireNumber), restrainedBarSpacingM: normWireRequired(normWireNumber), diagonalReinforcementRatio: normWireRequired(normWireNumber), shearCrackingBeforeYield: normWireRequired(normWireNumber), shearForceDemandN: normWireRequired(normWireNumber), intermediateSteelAreaM2: normWireRequired(normWireNumber), analysisMethod: normWireRequired(normWireRef(() => parseAnalysisMethodChoice)) });
export const parseEn1998Silo: NormWireReader<En1998Silo> = normWireObject<En1998Silo>({ id: normWireRequired(normWireString), heightM: normWireRequired(normWireNumber), radiusM: normWireRequired(normWireNumber), permanentGkN: normWireRequired(normWireNumber), contentQkN: normWireRequired(normWireNumber), contentCategory: normWireRequired(normWireRef(() => parseImposedCategory)), fillingRatio: normWireRequired(normWireNumber), nRdN: normWireRequired(normWireNumber), vRdN: normWireRequired(normWireNumber), qNominal: normWireRequired(normWireNumber) });
export const parseEn1998Tank: NormWireReader<En1998Tank> = normWireObject<En1998Tank>({ id: normWireRequired(normWireString), heightM: normWireRequired(normWireNumber), radiusM: normWireRequired(normWireNumber), permanentGkN: normWireRequired(normWireNumber), contentQkN: normWireRequired(normWireNumber), contentCategory: normWireRequired(normWireRef(() => parseImposedCategory)), fillingRatio: normWireRequired(normWireNumber), vRdN: normWireRequired(normWireNumber), shellStrengthMpa: normWireRequired(normWireNumber), shellThicknessM: normWireRequired(normWireNumber) });
export const parseEn1998Foundation: NormWireReader<En1998Foundation> = normWireObject<En1998Foundation>({ id: normWireRequired(normWireString), supportedBuildingId: normWireRequired(normWireString), areaM2: normWireRequired(normWireNumber), pRdPa: normWireRequired(normWireNumber), passiveResistanceN: normWireRequired(normWireNumber), kFoundation: normWireRequired(normWireNumber), kSoil: normWireRequired(normWireNumber), liqN160: normWireRequired(normWireNumber), liqFinesPercent: normWireRequired(normWireNumber), liqDepthM: normWireRequired(normWireNumber), liqWaterDepthM: normWireRequired(normWireNumber), liqUnitWeightNM3: normWireRequired(normWireNumber), liqMagnitude: normWireRequired(normWireNumber), liqLayerThicknessM: normWireRequired(normWireNumber), footingWidthM: normWireRequired(normWireNumber), bearingAxialN: normWireRequired(normWireNumber), bearingShearN: normWireRequired(normWireNumber), bearingMomentNm: normWireRequired(normWireNumber), bearingNMaxN: normWireRequired(normWireNumber), soilFrictionAngleDeg: normWireRequired(normWireNumber), pileDiameterM: normWireRequired(normWireNumber), pileLengthM: normWireRequired(normWireNumber), pileModulusPa: normWireRequired(normWireNumber), pileSubgradeModulusNM3: normWireRequired(normWireNumber), pileHeadShearN: normWireRequired(normWireNumber), pileHeadFixed: normWireRequired(normWireNumber), pileInterfaceDepthM: normWireRequired(normWireNumber), pileUpperVsMS: normWireRequired(normWireNumber), pileLowerVsMS: normWireRequired(normWireNumber), pileMomentRdNm: normWireRequired(normWireNumber), baseFrictionAngleDeg: normWireRequired(normWireNumber), basePorePressurePa: normWireRequired(normWireNumber) });
export const parseEn1998RetainingWall: NormWireReader<En1998RetainingWall> = normWireObject<En1998RetainingWall>({ id: normWireRequired(normWireString), heightM: normWireRequired(normWireNumber), phiDeg: normWireRequired(normWireNumber), soilGamma: normWireRequired(normWireNumber), r: normWireRequired(normWireNumber), hRdNPerM: normWireRequired(normWireNumber), waterHeightM: normWireRequired(normWireNumber), saturatedGamma: normWireRequired(normWireNumber), permeabilityMS: normWireRequired(normWireNumber) });
export const parseEn1998SoilLayer: NormWireReader<En1998SoilLayer> = normWireObject<En1998SoilLayer>({ id: normWireRequired(normWireString), foundationId: normWireRequired(normWireString), thicknessM: normWireRequired(normWireNumber), unitWeightNM3: normWireRequired(normWireNumber), shearWaveVelocityMS: normWireRequired(normWireNumber), n160: normWireRequired(normWireNumber), finesPercent: normWireRequired(normWireNumber) });
export const parseDiaphragmMaterial: NormWireReader<DiaphragmMaterial> = normWireLiteral("concrete", "timber", "steelDeck");
export const parseEn1998Diaphragm: NormWireReader<En1998Diaphragm> = normWireObject<En1998Diaphragm>({ id: normWireRequired(normWireString), buildingId: normWireRequired(normWireString), storeyId: normWireRequired(normWireString), material: normWireRequired(normWireRef(() => parseDiaphragmMaterial)), thicknessM: normWireRequired(normWireNumber), spanM: normWireRequired(normWireNumber), depthM: normWireRequired(normWireNumber), openingWidthM: normWireRequired(normWireNumber), shearStrengthMpa: normWireRequired(normWireNumber), chordAreaM2: normWireRequired(normWireNumber), chordStrengthMpa: normWireRequired(normWireNumber), gammaD: normWireRequired(normWireNumber) });
export const parseEn1998Accelerogram: NormWireReader<En1998Accelerogram> = normWireObject<En1998Accelerogram>({ id: normWireRequired(normWireString), buildingId: normWireRequired(normWireString), label: normWireRequired(normWireString), samples: normWireRequired(normWireString), dtS: normWireRequired(normWireNumber), scaleFactor: normWireRequired(normWireNumber) });
export const parsePierStiffnessBasis: NormWireReader<PierStiffnessBasis> = normWireLiteral("declared", "computed");
export const parseEn1998Pier: NormWireReader<En1998Pier> = normWireObject<En1998Pier>({ id: normWireRequired(normWireString), bridgeId: normWireRequired(normWireString), xM: normWireRequired(normWireNumber), heightM: normWireRequired(normWireNumber), sectionBM: normWireRequired(normWireNumber), sectionHM: normWireRequired(normWireNumber), modulusMpa: normWireRequired(normWireNumber), momentRdNm: normWireRequired(normWireNumber), shearRdN: normWireRequired(normWireNumber), reinforcementRatio: normWireRequired(normWireNumber), coverM: normWireRequired(normWireNumber), concreteFckMpa: normWireRequired(normWireNumber), steelFykMpa: normWireRequired(normWireNumber), axialLoadN: normWireRequired(normWireNumber), stiffnessBasis: normWireRequired(normWireRef(() => parsePierStiffnessBasis)) });
export const parseBearingDevice: NormWireReader<BearingDevice> = normWireLiteral("fixed", "elastomeric", "sliding", "isolator");
export const parseEn1998Bearing: NormWireReader<En1998Bearing> = normWireObject<En1998Bearing>({ id: normWireRequired(normWireString), bridgeId: normWireRequired(normWireString), pierId: normWireRequired(normWireString), device: normWireRequired(normWireRef(() => parseBearingDevice)), xM: normWireRequired(normWireNumber), stiffnessNPerM: normWireRequired(normWireNumber), dRdM: normWireRequired(normWireNumber), seatLengthM: normWireRequired(normWireNumber), minSupportLengthM: normWireRequired(normWireNumber) });
export const parseEn1998Joint: NormWireReader<En1998Joint> = normWireObject<En1998Joint>({ id: normWireRequired(normWireString), buildingId: normWireRequired(normWireString), storeyId: normWireRequired(normWireString), columnId: normWireRequired(normWireString), beamId: normWireRequired(normWireString), xM: normWireRequired(normWireNumber), yM: normWireRequired(normWireNumber), columnMomentRdNm: normWireRequired(normWireNumber), beamMomentRdNm: normWireRequired(normWireNumber) });
export const parseEn1998Tower: NormWireReader<En1998Tower> = normWireObject<En1998Tower>({ id: normWireRequired(normWireString), heightM: normWireRequired(normWireNumber), mRdNm: normWireRequired(normWireNumber), isChimney: normWireRequired(normWireBoolean), qNominal: normWireRequired(normWireNumber), permanentGkN: normWireRequired(normWireNumber), correlatedOccupancy: normWireRequired(normWireBoolean), variables: normWireRequired(normWireArray(normWireRef(() => parseEn1998VariableAction))), eiNm2: normWireRequired(normWireNumber), baseRotationalStiffnessNm: normWireRequired(normWireNumber) });
export const parseEn1998Placement: NormWireReader<En1998Placement> = normWireObject<En1998Placement>({ id: normWireRequired(normWireString), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber), yaw: normWireRequired(normWireNumber) });
export const parseImposedCategory: NormWireReader<ImposedCategory> = normWireLiteral("a", "b", "c", "d", "e", "f", "g", "h", "snow", "wind");
export const parseEn1998VariableAction: NormWireReader<En1998VariableAction> = normWireObject<En1998VariableAction>({ id: normWireRequired(normWireString), category: normWireRequired(normWireRef(() => parseImposedCategory)), qkN: normWireRequired(normWireNumber) });
export const parseStiffnessBasis: NormWireReader<StiffnessBasis> = normWireLiteral("simplified", "declared");
export const parseMemberKind: NormWireReader<MemberKind> = normWireLiteral("beam", "column", "wall", "brace");
export const parseDeGroundCombo: NormWireReader<DeGroundCombo> = normWireLiteral("A-R", "B-R", "C-R", "B-T", "C-T", "C-S");
export const parseEn1998TieBeam: NormWireReader<En1998TieBeam> = normWireObject<En1998TieBeam>({ id: normWireRequired(normWireString), foundationAId: normWireRequired(normWireString), foundationBId: normWireRequired(normWireString), span: normWireRequired(normWireNumber), width: normWireRequired(normWireNumber), depth: normWireRequired(normWireNumber), steelArea: normWireRequired(normWireNumber), steelDesignStrength: normWireRequired(normWireNumber), relativeDisplacement: normWireRequired(normWireNumber), displacementLimit: normWireRequired(normWireNumber) });
export const parseEn1998Slope: NormWireReader<En1998Slope> = normWireObject<En1998Slope>({ id: normWireRequired(normWireString), governingLayerId: normWireRequired(normWireString), height: normWireRequired(normWireNumber), angle: normWireRequired(normWireNumber), frictionAngle: normWireRequired(normWireNumber), cohesion: normWireRequired(normWireNumber), topographicAmplification: normWireRequired(normWireNumber), circleCentreX: normWireRequired(normWireNumber), circleCentreZ: normWireRequired(normWireNumber), circleRadius: normWireRequired(normWireNumber) });
export const parseAnalysisMethodChoice: NormWireReader<AnalysisMethodChoice> = normWireLiteral("derived", "lf", "mrs", "pushover", "nlth");
