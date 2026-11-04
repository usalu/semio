/** 🔺️ `En1993Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireAny, normWireArray, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionKind, parseActionKind, type AnalysisKind, parseAnalysisKind, type BridgeFatigue, parseBridgeFatigue, type BucklingLength, parseBucklingLength, type BuiltUpKind, parseBuiltUpKind, type BuiltUpMember, parseBuiltUpMember, type ChordShape, parseChordShape, type ColdFormedMember, parseColdFormedMember, type CraneRunway, parseCraneRunway, type DesignAction, parseDesignAction, type FabricationToleranceClass, parseFabricationToleranceClass, type FastenedJoint, parseFastenedJoint, type FatigueBand, parseFatigueBand, type FatigueDetail, parseFatigueDetail, type FatigueMethod, parseFatigueMethod, type FatigueStress, parseFatigueStress, type FireExposure, parseFireExposure, type FireRating, parseFireRating, type ForceAction, parseForceAction, type FoundationBlock, parseFoundationBlock, type FormingGain, parseFormingGain, type FrameAction, parseFrameAction, type FrameBar, parseFrameBar, type FrameBarRelease, parseFrameBarRelease, type FrameMemberBasis, parseFrameMemberBasis, type FrameNode, parseFrameNode, type FrameNodeSupport, parseFrameNodeSupport, type HollowJoint, parseHollowJoint, type JointCategory, parseJointCategory, type JointForceAction, parseJointForceAction, type JointKind, parseJointKind, type JointPlate, parseJointPlate, type JointRow, parseJointRow, type JointRowType, parseJointRowType, type LatticeKind, parseLatticeKind, type LoadApplication, parseLoadApplication, type LoadCase, parseLoadCase, type MaterialKind, parseMaterialKind, type MaterialDuctility, parseMaterialDuctility, type MemberAction, parseMemberAction, type MomentDiagram, parseMomentDiagram, type ParametricSection, parseParametricSection, type ParametricShape, parseParametricShape, type PatchLoadType, parsePatchLoadType, type Placement, parsePlacement, type PlatedPanel, parsePlatedPanel, type Restraint, parseRestraint, type RestraintDistribution, parseRestraintDistribution, type RestraintSupport, parseRestraintSupport, type SectionKind, parseSectionKind, type SheetPileEnvironment, parseSheetPileEnvironment, type SiloShell, parseSiloShell, type StainlessForming, parseStainlessForming, type SteelJoint, parseSteelJoint, type SteelMaterial, parseSteelMaterial, type SteelMember, parseSteelMember, type SteelMemberType, parseSteelMemberType, type SteelPile, parseSteelPile, type SteelSection, parseSteelSection, type StiffenerShape, parseStiffenerShape, type StiffnessClass, parseStiffnessClass, type Storey, parseStorey, type TensionComponent, parseTensionComponent, type TorsionLoad, parseTorsionLoad, type TowerLeg, parseTowerLeg, type WarpingSupport, parseWarpingSupport, type WeldGroup, parseWeldGroup, type WeldPattern, parseWeldPattern } from "../📸️snapshot/🟦️.ts";

export interface En1993Diff {
  /** @state artifact */
  annex?: ("En" | "De") | null;
  /** @state artifact */
  materials: En1993MaterialDelta;
  /** @state artifact */
  sections: En1993SectionDelta;
  /** @state artifact */
  members: En1993MemberDelta;
  /** @state artifact */
  loadCases: En1993LoadCaseDelta;
  /** @state artifact */
  memberActions: En1993MemberActionDelta;
  /** @state artifact */
  joints: En1993JointDelta;
  /** @state artifact */
  fatigueDetails: En1993FatigueDetailDelta;
  /** @state artifact */
  fireExposures: En1993FireExposureDelta;
  /** @state artifact */
  coldFormedMembers: En1993ColdFormedMemberDelta;
  /** @state artifact */
  platedPanels: En1993PlatedPanelDelta;
  /** @state artifact */
  siloShells: En1993SiloShellDelta;
  /** @state artifact */
  tensionComponents: En1993TensionComponentDelta;
  /** @state artifact */
  bridgeFatigue: En1993BridgeFatigueDelta;
  /** @state artifact */
  towerLegs: En1993TowerLegDelta;
  /** @state artifact */
  piles: En1993PileDelta;
  /** @state artifact */
  craneRunways: En1993CraneRunwayDelta;
  /** @state artifact */
  storeys: En1993StoreyDelta;
  /** @state artifact */
  restraints: En1993RestraintDelta;
  /** @state artifact */
  builtUpMembers: En1993BuiltUpMemberDelta;
  /** @state artifact */
  weldGroups: En1993WeldGroupDelta;
  /** @state artifact */
  parametricSections: En1993ParametricSectionDelta;
  /** @state artifact */
  hollowJoints: En1993HollowJointDelta;
  /** @state artifact */
  jointRows: En1993JointRowDelta;
  /** @state artifact */
  frameNodes: En1993FrameNodeDelta;
  /** @state artifact */
  frameBars: En1993FrameBarDelta;
  /** @state artifact */
  placements: En1993PlacementDelta;
  /** @state artifact */
  reliabilityClass?: ("RC1" | "RC2" | "RC3") | null;
  /** @state artifact */
  jointPlates: En1993JointPlateDelta;
  /** @state artifact */
  foundationBlocks: En1993FoundationBlockDelta;
  /** @state artifact */
  materialDuctility: En1993MaterialDuctilityDelta;
  /** @state artifact */
  formingGains: En1993FormingGainDelta;
  /** @state artifact */
  bucklingLengths: En1993BucklingLengthDelta;
  /** @state artifact */
  fatigueStresses: En1993FatigueStressDelta;
  /** @state artifact */
  frameActions: En1993FrameActionDelta;
  /** @state artifact */
  frameMemberBases: En1993FrameMemberBasisDelta;
  /** @state artifact */
  fastenedJoints: En1993FastenedJointDelta;
}

export interface En1993MaterialDelta {
  removed: En1993MaterialRemoval[];
  inserted: En1993MaterialInsertion[];
  moved: En1993MaterialRelocation[];
  modified: En1993MaterialModification[];
}

export interface En1993MaterialRemoval {
  id: string;
  index: number;
}

export interface En1993MaterialInsertion {
  index: number;
  row: SteelMaterial;
}

export interface En1993MaterialRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993MaterialModification {
  id: string;
  patch: En1993MaterialPatch;
}

export interface En1993MaterialPatch {
  grade?: string | null;
  fy?: number | null;
  fu?: number | null;
  eModulus?: number | null;
  gModulus?: number | null;
  subgrade?: string | null;
  kind?: MaterialKind | null;
  rambergOsgoodN?: number | null;
}

export interface En1993SectionDelta {
  removed: En1993SectionRemoval[];
  inserted: En1993SectionInsertion[];
  moved: En1993SectionRelocation[];
  modified: En1993SectionModification[];
}

export interface En1993SectionRemoval {
  id: string;
  index: number;
}

export interface En1993SectionInsertion {
  index: number;
  row: SteelSection;
}

export interface En1993SectionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993SectionModification {
  id: string;
  patch: En1993SectionPatch;
}

export interface En1993SectionPatch {
  designation?: string | null;
  kind?: SectionKind | null;
  h?: number | null;
  b?: number | null;
  tw?: number | null;
  tf?: number | null;
  r?: number | null;
  area?: number | null;
  shearAreaY?: number | null;
  shearAreaZ?: number | null;
  iy?: number | null;
  iz?: number | null;
  it?: number | null;
  iw?: number | null;
  wElY?: number | null;
  wElZ?: number | null;
  wPlY?: number | null;
  wPlZ?: number | null;
  areaNet?: number | null;
}

export interface En1993MemberDelta {
  removed: En1993MemberRemoval[];
  inserted: En1993MemberInsertion[];
  moved: En1993MemberRelocation[];
  modified: En1993MemberModification[];
}

export interface En1993MemberRemoval {
  id: string;
  index: number;
}

export interface En1993MemberInsertion {
  index: number;
  row: SteelMember;
}

export interface En1993MemberRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993MemberModification {
  id: string;
  patch: En1993MemberPatch;
}

export interface En1993MemberPatch {
  label?: string | null;
  memberType?: SteelMemberType | null;
  sectionId?: string | null;
  materialId?: string | null;
  length?: number | null;
  bucklingLengthY?: number | null;
  bucklingLengthZ?: number | null;
  ltbLength?: number | null;
  ltbRestraintSpacing?: number | null;
  loadApplication?: LoadApplication | null;
  endMomentRatioPsi?: number | null;
  momentDiagram?: MomentDiagram | null;
  deflectionLimitRatio?: number | null;
  analysis?: AnalysisKind | null;
  stainlessForming?: StainlessForming | null;
  designTemperature?: number | null;
  toughnessStressRatio?: number | null;
  radiationShift?: number | null;
  stressShift?: number | null;
  marginShift?: number | null;
  strainRateShift?: number | null;
  coldFormingRadius?: number | null;
  webMaterialId?: string | null;
  torsionLoad?: TorsionLoad | null;
  torquePosition?: number | null;
  warpingSupport?: WarpingSupport | null;
}

export interface En1993LoadCaseDelta {
  removed: En1993LoadCaseRemoval[];
  inserted: En1993LoadCaseInsertion[];
  moved: En1993LoadCaseRelocation[];
  modified: En1993LoadCaseModification[];
}

export interface En1993LoadCaseRemoval {
  id: string;
  index: number;
}

export interface En1993LoadCaseInsertion {
  index: number;
  row: LoadCase;
}

export interface En1993LoadCaseRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993LoadCaseModification {
  id: string;
  patch: En1993LoadCasePatch;
}

export interface En1993LoadCasePatch {
  name?: string | null;
  kind?: ActionKind | null;
  category?: string | null;
}

export interface En1993MemberActionDelta {
  removed: En1993MemberActionRemoval[];
  inserted: En1993MemberActionInsertion[];
  moved: En1993MemberActionRelocation[];
  modified: En1993MemberActionModification[];
}

export interface En1993MemberActionRemoval {
  id: string;
  index: number;
}

export interface En1993MemberActionInsertion {
  index: number;
  row: MemberAction;
}

export interface En1993MemberActionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993MemberActionModification {
  id: string;
  patch: En1993MemberActionPatch;
}

export interface En1993MemberActionPatch {
  memberId?: string | null;
  loadCaseId?: string | null;
  action?: DesignAction | null;
}

export interface En1993JointDelta {
  removed: En1993JointRemoval[];
  inserted: En1993JointInsertion[];
  moved: En1993JointRelocation[];
  modified: En1993JointModification[];
}

export interface En1993JointRemoval {
  id: string;
  index: number;
}

export interface En1993JointInsertion {
  index: number;
  row: SteelJoint;
}

export interface En1993JointRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993JointModification {
  id: string;
  patch: En1993JointPatch;
}

export interface En1993JointPatch {
  kind?: JointKind | null;
  memberId?: string | null;
  boltClass?: string | null;
  boltDiameter?: number | null;
  boltRows?: number | null;
  boltsPerRow?: number | null;
  pitch?: number | null;
  gauge?: number | null;
  endDistance?: number | null;
  edgeDistance?: number | null;
  shearPlanes?: number | null;
  plateThickness?: number | null;
  plateFu?: number | null;
  weldThroat?: number | null;
  weldLength?: number | null;
  weldFu?: number | null;
  weldGrade?: string | null;
  actions?: JointForceAction[] | null;
  category?: JointCategory | null;
  frictionMu?: number | null;
  preloadForce?: number | null;
  slipFactorKs?: number | null;
  frictionSurfaces?: number | null;
  plateFy?: number | null;
  plateWidth?: number | null;
  boltToWeb?: number | null;
  extensionToFlange?: number | null;
  extensionEdge?: number | null;
  alphaFactor?: number | null;
  boltStressArea?: number | null;
  boltGripLength?: number | null;
  columnFlangeThickness?: number | null;
  columnWebThickness?: number | null;
  columnWebDepth?: number | null;
  columnWebShearArea?: number | null;
  columnRootRadius?: number | null;
  columnFy?: number | null;
  columnWebStress?: number | null;
  beamFlangeThickness?: number | null;
  compressionLever?: number | null;
  momentDemand?: number | null;
  beamMomentResistance?: number | null;
  columnMomentResistance?: number | null;
  beamInertia?: number | null;
  beamSpan?: number | null;
  bracedFrame?: number | null;
  stiffnessRatio?: number | null;
  transformationParameter?: number | null;
  assumedClass?: StiffnessClass | null;
  basePlateLength?: number | null;
  concreteFcd?: number | null;
  concreteModulus?: number | null;
  concreteAreaA0?: number | null;
  concreteAreaA1?: number | null;
  jointCoefficient?: number | null;
  anchorArea?: number | null;
  anchorLength?: number | null;
  baseNormalForce?: number | null;
  baseMoment?: number | null;
  baseLeverLeft?: number | null;
  baseLeverRight?: number | null;
}

export interface En1993FatigueDetailDelta {
  removed: En1993FatigueDetailRemoval[];
  inserted: En1993FatigueDetailInsertion[];
  moved: En1993FatigueDetailRelocation[];
  modified: En1993FatigueDetailModification[];
}

export interface En1993FatigueDetailRemoval {
  id: string;
  index: number;
}

export interface En1993FatigueDetailInsertion {
  index: number;
  row: FatigueDetail;
}

export interface En1993FatigueDetailRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993FatigueDetailModification {
  id: string;
  patch: En1993FatigueDetailPatch;
}

export interface En1993FatigueDetailPatch {
  memberId?: string | null;
  category?: number | null;
  method?: FatigueMethod | null;
  spectrum?: FatigueBand[] | null;
  stressHistory?: string | null;
}

export interface En1993FireExposureDelta {
  removed: En1993FireExposureRemoval[];
  inserted: En1993FireExposureInsertion[];
  moved: En1993FireExposureRelocation[];
  modified: En1993FireExposureModification[];
}

export interface En1993FireExposureRemoval {
  id: string;
  index: number;
}

export interface En1993FireExposureInsertion {
  index: number;
  row: FireExposure;
}

export interface En1993FireExposureRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993FireExposureModification {
  id: string;
  patch: En1993FireExposurePatch;
}

export interface En1993FireExposurePatch {
  memberId?: string | null;
  rating?: FireRating | null;
  protectionThickness?: number | null;
  sectionFactor?: number | null;
  mu0?: number | null;
  designTemperature?: number | null;
  protectionConductivity?: number | null;
  protectionDensity?: number | null;
  protectionSpecificHeat?: number | null;
}

export interface En1993ColdFormedMemberDelta {
  removed: En1993ColdFormedMemberRemoval[];
  inserted: En1993ColdFormedMemberInsertion[];
  moved: En1993ColdFormedMemberRelocation[];
  modified: En1993ColdFormedMemberModification[];
}

export interface En1993ColdFormedMemberRemoval {
  id: string;
  index: number;
}

export interface En1993ColdFormedMemberInsertion {
  index: number;
  row: ColdFormedMember;
}

export interface En1993ColdFormedMemberRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993ColdFormedMemberModification {
  id: string;
  patch: En1993ColdFormedMemberPatch;
}

export interface En1993ColdFormedMemberPatch {
  bBar?: number | null;
  thickness?: number | null;
  kSigma?: number | null;
  psi?: number | null;
  fy?: number | null;
  grossResistance?: number | null;
  actions?: ForceAction[] | null;
  flangeWidth?: number | null;
  lipLength?: number | null;
  webHeight?: number | null;
  webSlope?: number | null;
  oppositeFlangeWidth?: number | null;
  springFactor?: number | null;
  cornerRadius?: number | null;
  flangeStress?: number | null;
  shearForce?: number | null;
  supportReaction?: number | null;
  supportLength?: number | null;
  cripplingCategory?: number | null;
  stiffenedSupport?: number | null;
  bendingMoment?: number | null;
  momentResistance?: number | null;
  sheetingStiffness?: number | null;
  requiredRestraintStiffness?: number | null;
  purlinSpan?: number | null;
  purlinLoad?: number | null;
  purlinLateralRatio?: number | null;
}

export interface En1993PlatedPanelDelta {
  removed: En1993PlatedPanelRemoval[];
  inserted: En1993PlatedPanelInsertion[];
  moved: En1993PlatedPanelRelocation[];
  modified: En1993PlatedPanelModification[];
}

export interface En1993PlatedPanelRemoval {
  id: string;
  index: number;
}

export interface En1993PlatedPanelInsertion {
  index: number;
  row: PlatedPanel;
}

export interface En1993PlatedPanelRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993PlatedPanelModification {
  id: string;
  patch: En1993PlatedPanelPatch;
}

export interface En1993PlatedPanelPatch {
  a?: number | null;
  b?: number | null;
  thickness?: number | null;
  fy?: number | null;
  kSigma?: number | null;
  actions?: ForceAction[] | null;
  stiffenerInertia?: number | null;
  stiffenerArea?: number | null;
  stiffenerEccentricity?: number | null;
  stiffenerAlpha?: number | null;
  compressionStress?: number | null;
  halfWidth?: number | null;
  equivalentSpan?: number | null;
  hogging?: number | null;
  patchType?: PatchLoadType | null;
  patchForce?: number | null;
  patchLength?: number | null;
  patchOffset?: number | null;
  flangeThickness?: number | null;
  flangeWidth?: number | null;
  flangeFy?: number | null;
  longitudinalOffset?: number | null;
  longitudinalInertia?: number | null;
  patchMoment?: number | null;
  patchMomentResistance?: number | null;
  bendingMoment?: number | null;
  plasticMoment?: number | null;
  flangeMoment?: number | null;
  shearForce?: number | null;
  transverseStiffenerInertia?: number | null;
  flangeInducedK?: number | null;
  flangeArea?: number | null;
  stressX?: number | null;
  stressZ?: number | null;
  shearStress?: number | null;
  alphaCr?: number | null;
  corrugationFlat?: number | null;
  corrugationProjection?: number | null;
  corrugationDepth?: number | null;
  webHeight?: number | null;
  stiffenerSpacing?: number | null;
  stiffenerShape?: StiffenerShape | null;
  stiffenerDepth?: number | null;
  stiffenerThickness?: number | null;
  stiffenerFlangeWidth?: number | null;
  stiffenerFlangeThickness?: number | null;
  lateralPressure?: number | null;
  lateralPressureCoefficient?: number | null;
}

export interface En1993SiloShellDelta {
  removed: En1993SiloShellRemoval[];
  inserted: En1993SiloShellInsertion[];
  moved: En1993SiloShellRelocation[];
  modified: En1993SiloShellModification[];
}

export interface En1993SiloShellRemoval {
  id: string;
  index: number;
}

export interface En1993SiloShellInsertion {
  index: number;
  row: SiloShell;
}

export interface En1993SiloShellRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993SiloShellModification {
  id: string;
  patch: En1993SiloShellPatch;
}

export interface En1993SiloShellPatch {
  thickness?: number | null;
  radius?: number | null;
  depth?: number | null;
  k?: number | null;
  gamma?: number | null;
  fy?: number | null;
  length?: number | null;
  fabricationClass?: FabricationToleranceClass | null;
  meridionalStress?: number | null;
  hoopStress?: number | null;
  shellShearStress?: number | null;
  boundaryFactorAxial?: number | null;
  boundaryFactorHoop?: number | null;
  boundaryFactorShear?: number | null;
  wallFriction?: number | null;
  planArea?: number | null;
  planPerimeter?: number | null;
  liquidDensity?: number | null;
  liquidHeight?: number | null;
  courseElevation?: number | null;
  internalPressure?: number | null;
  externalPressure?: number | null;
  temperatureChange?: number | null;
  ringSpacing?: number | null;
  ringArea?: number | null;
  ringInertia?: number | null;
  ringOffset?: number | null;
}

export interface En1993TensionComponentDelta {
  removed: En1993TensionComponentRemoval[];
  inserted: En1993TensionComponentInsertion[];
  moved: En1993TensionComponentRelocation[];
  modified: En1993TensionComponentModification[];
}

export interface En1993TensionComponentRemoval {
  id: string;
  index: number;
}

export interface En1993TensionComponentInsertion {
  index: number;
  row: TensionComponent;
}

export interface En1993TensionComponentRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993TensionComponentModification {
  id: string;
  patch: En1993TensionComponentPatch;
}

export interface En1993TensionComponentPatch {
  fUk?: number | null;
  fK?: number | null;
  actions?: ForceAction[] | null;
  partialFactor?: number | null;
  cableLength?: number | null;
  cableWeight?: number | null;
  cableHorizontalForce?: number | null;
  cableModulus?: number | null;
  cableStress?: number | null;
  cableUnitWeight?: number | null;
  threadFactor?: number | null;
  rodStressArea?: number | null;
  rodFu?: number | null;
  pinDiameter?: number | null;
  pinPlateThickness?: number | null;
  pinGap?: number | null;
  pinOuterThickness?: number | null;
  pinInnerThickness?: number | null;
  pinFup?: number | null;
  pinFyp?: number | null;
  pinPlateFy?: number | null;
}

export interface En1993BridgeFatigueDelta {
  removed: En1993BridgeFatigueRemoval[];
  inserted: En1993BridgeFatigueInsertion[];
  moved: En1993BridgeFatigueRelocation[];
  modified: En1993BridgeFatigueModification[];
}

export interface En1993BridgeFatigueRemoval {
  id: string;
  index: number;
}

export interface En1993BridgeFatigueInsertion {
  index: number;
  row: BridgeFatigue;
}

export interface En1993BridgeFatigueRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993BridgeFatigueModification {
  id: string;
  patch: En1993BridgeFatiguePatch;
}

export interface En1993BridgeFatiguePatch {
  memberId?: string | null;
  lambda?: number | null;
  phi2?: number | null;
  deltaSigmaP?: number | null;
  category?: number | null;
  method?: FatigueMethod | null;
  heavyVehicleWeight?: number | null;
  referenceWeight?: number | null;
  observedTraffic?: number | null;
  designLife?: number | null;
  secondLaneTraffic?: number | null;
  secondLaneWeight?: number | null;
  lambda1?: number | null;
  lambdaMax?: number | null;
  detailId?: string | null;
  plateThickness?: number | null;
  deckThickness?: number | null;
  troughSpacing?: number | null;
  troughWidth?: number | null;
  troughThickness?: number | null;
  crossbeamSpacing?: number | null;
  surfacingThickness?: number | null;
  wheelLoad?: number | null;
  wheelPatch?: number | null;
}

export interface En1993TowerLegDelta {
  removed: En1993TowerLegRemoval[];
  inserted: En1993TowerLegInsertion[];
  moved: En1993TowerLegRelocation[];
  modified: En1993TowerLegModification[];
}

export interface En1993TowerLegRemoval {
  id: string;
  index: number;
}

export interface En1993TowerLegInsertion {
  index: number;
  row: TowerLeg;
}

export interface En1993TowerLegRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993TowerLegModification {
  id: string;
  patch: En1993TowerLegPatch;
}

export interface En1993TowerLegPatch {
  memberId?: string | null;
  forceCoefficient?: number | null;
  dynamicFactor?: number | null;
  actions?: ForceAction[] | null;
  latticeKind?: LatticeKind | null;
  slenderness?: number | null;
  diameter?: number | null;
  height?: number | null;
  bendingStiffness?: number | null;
  massPerLength?: number | null;
  strouhal?: number | null;
  logDecrement?: number | null;
  airDensity?: number | null;
  meanWindSpeed?: number | null;
  crossingRate?: number | null;
  designLife?: number | null;
  kFactor?: number | null;
  kwFactor?: number | null;
  lateralForceCoefficient?: number | null;
  permittedAmplitudeRatio?: number | null;
  ringMode?: number | null;
  shellThickness?: number | null;
  guyArea?: number | null;
  guyAngle?: number | null;
  guyLength?: number | null;
}

export interface En1993PileDelta {
  removed: En1993PileRemoval[];
  inserted: En1993PileInsertion[];
  moved: En1993PileRelocation[];
  modified: En1993PileModification[];
}

export interface En1993PileRemoval {
  id: string;
  index: number;
}

export interface En1993PileInsertion {
  index: number;
  row: SteelPile;
}

export interface En1993PileRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993PileModification {
  id: string;
  patch: En1993PilePatch;
}

export interface En1993PilePatch {
  sectionId?: string | null;
  materialId?: string | null;
  drivingStress?: number | null;
  embeddedLength?: number | null;
  shaftPerimeter?: number | null;
  actions?: ForceAction[] | null;
  environment?: SheetPileEnvironment | null;
  designLife?: number | null;
  wallThickness?: number | null;
  interlockBendingFactor?: number | null;
  interlockStiffnessFactor?: number | null;
  sectionModulus?: number | null;
  shearArea?: number | null;
  designMoment?: number | null;
  designShear?: number | null;
  interlockShear?: number | null;
}

export interface En1993CraneRunwayDelta {
  removed: En1993CraneRunwayRemoval[];
  inserted: En1993CraneRunwayInsertion[];
  moved: En1993CraneRunwayRelocation[];
  modified: En1993CraneRunwayModification[];
}

export interface En1993CraneRunwayRemoval {
  id: string;
  index: number;
}

export interface En1993CraneRunwayInsertion {
  index: number;
  row: CraneRunway;
}

export interface En1993CraneRunwayRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993CraneRunwayModification {
  id: string;
  patch: En1993CraneRunwayPatch;
}

export interface En1993CraneRunwayPatch {
  memberId?: string | null;
  wheelContactLength?: number | null;
  dispersion?: number | null;
  webThickness?: number | null;
  fy?: number | null;
  phi?: number | null;
  actions?: ForceAction[] | null;
  maxWheelLoad?: number | null;
  loadFactor?: number | null;
  railInertia?: number | null;
  railFixed?: number | null;
  flangeEffectiveWidth?: number | null;
  flangeThickness?: number | null;
  wheelEccentricity?: number | null;
  torsionConstant?: number | null;
  longitudinalStress?: number | null;
  railRadius?: number | null;
  railContactWidth?: number | null;
  craneNu?: number | null;
  craneKq?: number | null;
  damageExponent?: number | null;
  stressRange?: number | null;
  detailCategoryStress?: number | null;
  fatiguePhi?: number | null;
}

export interface En1993StoreyDelta {
  removed: En1993StoreyRemoval[];
  inserted: En1993StoreyInsertion[];
  moved: En1993StoreyRelocation[];
  modified: En1993StoreyModification[];
}

export interface En1993StoreyRemoval {
  id: string;
  index: number;
}

export interface En1993StoreyInsertion {
  index: number;
  row: Storey;
}

export interface En1993StoreyRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993StoreyModification {
  id: string;
  patch: En1993StoreyPatch;
}

export interface En1993StoreyPatch {
  memberId?: string | null;
  label?: string | null;
  height?: number | null;
  verticalLoad?: number | null;
  horizontalLoad?: number | null;
  drift?: number | null;
  columns?: number | null;
  plasticAnalysis?: number | null;
  buildingHeight?: number | null;
  bracingLoad?: number | null;
  bracingLength?: number | null;
  bracingDeflection?: number | null;
}

export interface En1993RestraintDelta {
  removed: En1993RestraintRemoval[];
  inserted: En1993RestraintInsertion[];
  moved: En1993RestraintRelocation[];
  modified: En1993RestraintModification[];
}

export interface En1993RestraintRemoval {
  id: string;
  index: number;
}

export interface En1993RestraintInsertion {
  index: number;
  row: Restraint;
}

export interface En1993RestraintRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993RestraintModification {
  id: string;
  patch: En1993RestraintPatch;
}

export interface En1993RestraintPatch {
  memberId?: string | null;
  label?: string | null;
  startSupport?: RestraintSupport | null;
  endSupport?: RestraintSupport | null;
  intermediatePosition?: number | null;
  torsionalSpring?: number | null;
  lateralSpringPosition?: number | null;
  lateralSpringStiffness?: number | null;
  loadHeight?: number | null;
  loadDistribution?: RestraintDistribution | null;
  designMoment?: number | null;
  designAxial?: number | null;
  generalMethod?: number | null;
  hingeToRestraint?: number | null;
}

export interface En1993BuiltUpMemberDelta {
  removed: En1993BuiltUpMemberRemoval[];
  inserted: En1993BuiltUpMemberInsertion[];
  moved: En1993BuiltUpMemberRelocation[];
  modified: En1993BuiltUpMemberModification[];
}

export interface En1993BuiltUpMemberRemoval {
  id: string;
  index: number;
}

export interface En1993BuiltUpMemberInsertion {
  index: number;
  row: BuiltUpMember;
}

export interface En1993BuiltUpMemberRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993BuiltUpMemberModification {
  id: string;
  patch: En1993BuiltUpMemberPatch;
}

export interface En1993BuiltUpMemberPatch {
  memberId?: string | null;
  label?: string | null;
  kind?: BuiltUpKind | null;
  chordArea?: number | null;
  chordInertia?: number | null;
  chordDistance?: number | null;
  panelSpacing?: number | null;
  planes?: number | null;
  battenInertia?: number | null;
  diagonalArea?: number | null;
  diagonalLength?: number | null;
  designAxial?: number | null;
  designMoment?: number | null;
  backToBack?: number | null;
  minRadius?: number | null;
  chainSpacing?: number | null;
  chordShape?: ChordShape | null;
  chordDepth?: number | null;
  chordWidth?: number | null;
  chordFlangeThickness?: number | null;
  chordWebThickness?: number | null;
}

export interface En1993WeldGroupDelta {
  removed: En1993WeldGroupRemoval[];
  inserted: En1993WeldGroupInsertion[];
  moved: En1993WeldGroupRelocation[];
  modified: En1993WeldGroupModification[];
}

export interface En1993WeldGroupRemoval {
  id: string;
  index: number;
}

export interface En1993WeldGroupInsertion {
  index: number;
  row: WeldGroup;
}

export interface En1993WeldGroupRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993WeldGroupModification {
  id: string;
  patch: En1993WeldGroupPatch;
}

export interface En1993WeldGroupPatch {
  memberId?: string | null;
  label?: string | null;
  pattern?: WeldPattern | null;
  throat?: number | null;
  width?: number | null;
  depth?: number | null;
  shearForce?: number | null;
  eccentricity?: number | null;
  steelFu?: number | null;
  betaW?: number | null;
}

export interface En1993ParametricSectionDelta {
  removed: En1993ParametricSectionRemoval[];
  inserted: En1993ParametricSectionInsertion[];
  moved: En1993ParametricSectionRelocation[];
  modified: En1993ParametricSectionModification[];
}

export interface En1993ParametricSectionRemoval {
  id: string;
  index: number;
}

export interface En1993ParametricSectionInsertion {
  index: number;
  row: ParametricSection;
}

export interface En1993ParametricSectionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993ParametricSectionModification {
  id: string;
  patch: En1993ParametricSectionPatch;
}

export interface En1993ParametricSectionPatch {
  sectionId?: string | null;
  label?: string | null;
  shape?: ParametricShape | null;
  height?: number | null;
  width?: number | null;
  webThickness?: number | null;
  flangeThickness?: number | null;
  tolerance?: number | null;
}

export interface En1993HollowJointDelta {
  removed: En1993HollowJointRemoval[];
  inserted: En1993HollowJointInsertion[];
  moved: En1993HollowJointRelocation[];
  modified: En1993HollowJointModification[];
}

export interface En1993HollowJointRemoval {
  id: string;
  index: number;
}

export interface En1993HollowJointInsertion {
  index: number;
  row: HollowJoint;
}

export interface En1993HollowJointRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993HollowJointModification {
  id: string;
  patch: En1993HollowJointPatch;
}

export interface En1993HollowJointPatch {
  memberId?: string | null;
  label?: string | null;
  chordWidth?: number | null;
  chordHeight?: number | null;
  chordThickness?: number | null;
  chordFy?: number | null;
  chordStress?: number | null;
  brace1Width?: number | null;
  brace1Height?: number | null;
  brace1Thickness?: number | null;
  brace1Angle?: number | null;
  brace1Force?: number | null;
  brace2Width?: number | null;
  brace2Height?: number | null;
  brace2Thickness?: number | null;
  brace2Angle?: number | null;
  brace2Force?: number | null;
  gap?: number | null;
  braceFy?: number | null;
}

export interface En1993JointRowDelta {
  removed: En1993JointRowRemoval[];
  inserted: En1993JointRowInsertion[];
  moved: En1993JointRowRelocation[];
  modified: En1993JointRowModification[];
}

export interface En1993JointRowRemoval {
  id: string;
  index: number;
}

export interface En1993JointRowInsertion {
  index: number;
  row: JointRow;
}

export interface En1993JointRowRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993JointRowModification {
  id: string;
  patch: En1993JointRowPatch;
}

export interface En1993JointRowPatch {
  jointId?: string | null;
  label?: string | null;
  rowType?: JointRowType | null;
  lever?: number | null;
  boltToWeb?: number | null;
  edgeDistance?: number | null;
  extensionToFlange?: number | null;
  extensionEdge?: number | null;
  alphaFactor?: number | null;
  pitchToPrevious?: number | null;
  bolts?: number | null;
  boltDiameter?: number | null;
  boltStressArea?: number | null;
  gripLength?: number | null;
  boltClass?: string | null;
  groupWithPrevious?: number | null;
}

export interface En1993FrameNodeDelta {
  removed: En1993FrameNodeRemoval[];
  inserted: En1993FrameNodeInsertion[];
  moved: En1993FrameNodeRelocation[];
  modified: En1993FrameNodeModification[];
}

export interface En1993FrameNodeRemoval {
  id: string;
  index: number;
}

export interface En1993FrameNodeInsertion {
  index: number;
  row: FrameNode;
}

export interface En1993FrameNodeRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993FrameNodeModification {
  id: string;
  patch: En1993FrameNodePatch;
}

export interface En1993FrameNodePatch {
  label?: string | null;
  x?: number | null;
  z?: number | null;
  support?: FrameNodeSupport | null;
  loadX?: number | null;
  loadZ?: number | null;
}

export interface En1993FrameBarDelta {
  removed: En1993FrameBarRemoval[];
  inserted: En1993FrameBarInsertion[];
  moved: En1993FrameBarRelocation[];
  modified: En1993FrameBarModification[];
}

export interface En1993FrameBarRemoval {
  id: string;
  index: number;
}

export interface En1993FrameBarInsertion {
  index: number;
  row: FrameBar;
}

export interface En1993FrameBarRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993FrameBarModification {
  id: string;
  patch: En1993FrameBarPatch;
}

export interface En1993FrameBarPatch {
  memberId?: string | null;
  label?: string | null;
  startNodeId?: string | null;
  endNodeId?: string | null;
  releaseStart?: FrameBarRelease | null;
  releaseEnd?: FrameBarRelease | null;
}

export interface En1993PlacementDelta {
  removed: En1993PlacementRemoval[];
  inserted: En1993PlacementInsertion[];
  moved: En1993PlacementRelocation[];
  modified: En1993PlacementModification[];
}

export interface En1993PlacementRemoval {
  id: string;
  index: number;
}

export interface En1993PlacementInsertion {
  index: number;
  row: Placement;
}

export interface En1993PlacementRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993PlacementModification {
  id: string;
  patch: En1993PlacementPatch;
}

export interface En1993PlacementPatch {
  x?: number | null;
  y?: number | null;
  z?: number | null;
  yaw?: number | null;
}

export interface En1993JointPlateDelta {
  removed: En1993JointPlateRemoval[];
  inserted: En1993JointPlateInsertion[];
  moved: En1993JointPlateRelocation[];
  modified: En1993JointPlateModification[];
}

export interface En1993JointPlateRemoval {
  id: string;
  index: number;
}

export interface En1993JointPlateInsertion {
  index: number;
  row: JointPlate;
}

export interface En1993JointPlateRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993JointPlateModification {
  id: string;
  patch: En1993JointPlatePatch;
}

export interface En1993JointPlatePatch {
  jointId?: string | null;
  label?: string | null;
  role?: string | null;
  plateHeight?: number | null;
  plateWidth?: number | null;
  plateThickness?: number | null;
  plateCount?: number | null;
  plateFy?: number | null;
  plateFu?: number | null;
  boltClass?: string | null;
  boltDiameter?: number | null;
  holeDiameter?: number | null;
  boltRows?: number | null;
  boltsPerRow?: number | null;
  pitch?: number | null;
  gauge?: number | null;
  endDistance?: number | null;
  edgeDistance?: number | null;
  shearPlanes?: number | null;
  eccentricity?: number | null;
  weldThroat?: number | null;
  weldFu?: number | null;
  weldBetaW?: number | null;
}

export interface En1993FoundationBlockDelta {
  removed: En1993FoundationBlockRemoval[];
  inserted: En1993FoundationBlockInsertion[];
  moved: En1993FoundationBlockRelocation[];
  modified: En1993FoundationBlockModification[];
}

export interface En1993FoundationBlockRemoval {
  id: string;
  index: number;
}

export interface En1993FoundationBlockInsertion {
  index: number;
  row: FoundationBlock;
}

export interface En1993FoundationBlockRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993FoundationBlockModification {
  id: string;
  patch: En1993FoundationBlockPatch;
}

export interface En1993FoundationBlockPatch {
  jointId?: string | null;
  label?: string | null;
  blockLength?: number | null;
  blockWidth?: number | null;
  blockDepth?: number | null;
  concreteFck?: number | null;
  groutFck?: number | null;
  groutThickness?: number | null;
  anchorCount?: number | null;
  anchorClass?: string | null;
  anchorDiameter?: number | null;
  anchorStressArea?: number | null;
  anchorEmbedment?: number | null;
  anchorEdgeDistance?: number | null;
}

export interface En1993FrameActionDelta {
  removed: En1993FrameActionRemoval[];
  inserted: En1993FrameActionInsertion[];
  moved: En1993FrameActionRelocation[];
  modified: En1993FrameActionModification[];
}

export interface En1993FrameActionRemoval {
  id: string;
  index: number;
}

export interface En1993FrameActionInsertion {
  index: number;
  row: FrameAction;
}

export interface En1993FrameActionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993FrameActionModification {
  id: string;
  patch: En1993FrameActionPatch;
}

export interface En1993FrameActionPatch {
  loadCaseId?: string | null;
  targetId?: string | null;
  kind?: string | null;
  fx?: string | null;
  fz?: string | null;
  my?: string | null;
  startRatio?: string | null;
  endRatio?: string | null;
}

export interface En1993FrameMemberBasisDelta {
  removed: En1993FrameMemberBasisRemoval[];
  inserted: En1993FrameMemberBasisInsertion[];
  moved: En1993FrameMemberBasisRelocation[];
  modified: En1993FrameMemberBasisModification[];
}

export interface En1993FrameMemberBasisRemoval {
  id: string;
  index: number;
}

export interface En1993FrameMemberBasisInsertion {
  index: number;
  row: FrameMemberBasis;
}

export interface En1993FrameMemberBasisRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993FrameMemberBasisModification {
  id: string;
  patch: En1993FrameMemberBasisPatch;
}

export interface En1993FrameMemberBasisPatch {
  memberId?: string | null;
  basis?: string | null;
}

export interface En1993MaterialDuctilityDelta {
  removed: En1993MaterialDuctilityRemoval[];
  inserted: En1993MaterialDuctilityInsertion[];
  moved: En1993MaterialDuctilityRelocation[];
  modified: En1993MaterialDuctilityModification[];
}

export interface En1993MaterialDuctilityRemoval {
  id: string;
  index: number;
}

export interface En1993MaterialDuctilityInsertion {
  index: number;
  row: MaterialDuctility;
}

export interface En1993MaterialDuctilityRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993MaterialDuctilityModification {
  id: string;
  patch: En1993MaterialDuctilityPatch;
}

export interface En1993MaterialDuctilityPatch {
  materialId?: string | null;
  elongation?: string | null;
  ultimateStrain?: string | null;
}

export interface En1993FormingGainDelta {
  removed: En1993FormingGainRemoval[];
  inserted: En1993FormingGainInsertion[];
  moved: En1993FormingGainRelocation[];
  modified: En1993FormingGainModification[];
}

export interface En1993FormingGainRemoval {
  id: string;
  index: number;
}

export interface En1993FormingGainInsertion {
  index: number;
  row: FormingGain;
}

export interface En1993FormingGainRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993FormingGainModification {
  id: string;
  patch: En1993FormingGainPatch;
}

export interface En1993FormingGainPatch {
  memberId?: string | null;
  forming?: string | null;
  ultimateStrength?: string | null;
  grossArea?: string | null;
  bends?: string | null;
  usedYield?: string | null;
}

export interface En1993BucklingLengthDelta {
  removed: En1993BucklingLengthRemoval[];
  inserted: En1993BucklingLengthInsertion[];
  moved: En1993BucklingLengthRelocation[];
  modified: En1993BucklingLengthModification[];
}

export interface En1993BucklingLengthRemoval {
  id: string;
  index: number;
}

export interface En1993BucklingLengthInsertion {
  index: number;
  row: BucklingLength;
}

export interface En1993BucklingLengthRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993BucklingLengthModification {
  id: string;
  patch: En1993BucklingLengthPatch;
}

export interface En1993BucklingLengthPatch {
  memberId?: string | null;
  plane?: string | null;
  sway?: string | null;
  restraintA?: string | null;
  columnStiffnessA?: string | null;
  beamStiffnessA?: string | null;
  restraintB?: string | null;
  columnStiffnessB?: string | null;
  beamStiffnessB?: string | null;
}

export interface En1993FatigueStressDelta {
  removed: En1993FatigueStressRemoval[];
  inserted: En1993FatigueStressInsertion[];
  moved: En1993FatigueStressRelocation[];
  modified: En1993FatigueStressModification[];
}

export interface En1993FatigueStressRemoval {
  id: string;
  index: number;
}

export interface En1993FatigueStressInsertion {
  index: number;
  row: FatigueStress;
}

export interface En1993FatigueStressRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993FatigueStressModification {
  id: string;
  patch: En1993FatigueStressPatch;
}

export interface En1993FatigueStressPatch {
  detailId?: string | null;
  maxCaseId?: string | null;
  minCaseId?: string | null;
  fibreZ?: string | null;
  fibreY?: string | null;
  relief?: string | null;
  concentration?: string | null;
  thickness?: string | null;
  cycles?: string | null;
}

export interface En1993FastenedJointDelta {
  removed: En1993FastenedJointRemoval[];
  inserted: En1993FastenedJointInsertion[];
  moved: En1993FastenedJointRelocation[];
  modified: En1993FastenedJointModification[];
}

export interface En1993FastenedJointRemoval {
  id: string;
  index: number;
}

export interface En1993FastenedJointInsertion {
  index: number;
  row: FastenedJoint;
}

export interface En1993FastenedJointRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1993FastenedJointModification {
  id: string;
  patch: En1993FastenedJointPatch;
}

export interface En1993FastenedJointPatch {
  memberId?: string | null;
  kind?: string | null;
  diameter?: string | null;
  washerDiameter?: string | null;
  count?: string | null;
  perRow?: string | null;
  headThickness?: string | null;
  baseThickness?: string | null;
  headStrength?: string | null;
  baseStrength?: string | null;
  endDistance?: string | null;
  edgeDistance?: string | null;
  pitch?: string | null;
  gauge?: string | null;
  sheetWidth?: string | null;
  shearForce?: string | null;
  tensionForce?: string | null;
  shearResistance?: string | null;
}

export const parseEn1993Diff: NormWireReader<En1993Diff> = normWireObject<En1993Diff>({ annex: normWireOptional(normWireNullable(normWireLiteral("En", "De"))), materials: normWireRequired(normWireAny<[En1993MaterialDelta]>(normWireRef(() => parseEn1993MaterialDelta))), sections: normWireRequired(normWireAny<[En1993SectionDelta]>(normWireRef(() => parseEn1993SectionDelta))), members: normWireRequired(normWireAny<[En1993MemberDelta]>(normWireRef(() => parseEn1993MemberDelta))), loadCases: normWireRequired(normWireAny<[En1993LoadCaseDelta]>(normWireRef(() => parseEn1993LoadCaseDelta))), memberActions: normWireRequired(normWireAny<[En1993MemberActionDelta]>(normWireRef(() => parseEn1993MemberActionDelta))), joints: normWireRequired(normWireAny<[En1993JointDelta]>(normWireRef(() => parseEn1993JointDelta))), fatigueDetails: normWireRequired(normWireAny<[En1993FatigueDetailDelta]>(normWireRef(() => parseEn1993FatigueDetailDelta))), fireExposures: normWireRequired(normWireAny<[En1993FireExposureDelta]>(normWireRef(() => parseEn1993FireExposureDelta))), coldFormedMembers: normWireRequired(normWireAny<[En1993ColdFormedMemberDelta]>(normWireRef(() => parseEn1993ColdFormedMemberDelta))), platedPanels: normWireRequired(normWireAny<[En1993PlatedPanelDelta]>(normWireRef(() => parseEn1993PlatedPanelDelta))), siloShells: normWireRequired(normWireAny<[En1993SiloShellDelta]>(normWireRef(() => parseEn1993SiloShellDelta))), tensionComponents: normWireRequired(normWireAny<[En1993TensionComponentDelta]>(normWireRef(() => parseEn1993TensionComponentDelta))), bridgeFatigue: normWireRequired(normWireAny<[En1993BridgeFatigueDelta]>(normWireRef(() => parseEn1993BridgeFatigueDelta))), towerLegs: normWireRequired(normWireAny<[En1993TowerLegDelta]>(normWireRef(() => parseEn1993TowerLegDelta))), piles: normWireRequired(normWireAny<[En1993PileDelta]>(normWireRef(() => parseEn1993PileDelta))), craneRunways: normWireRequired(normWireAny<[En1993CraneRunwayDelta]>(normWireRef(() => parseEn1993CraneRunwayDelta))), storeys: normWireRequired(normWireAny<[En1993StoreyDelta]>(normWireRef(() => parseEn1993StoreyDelta))), restraints: normWireRequired(normWireAny<[En1993RestraintDelta]>(normWireRef(() => parseEn1993RestraintDelta))), builtUpMembers: normWireRequired(normWireAny<[En1993BuiltUpMemberDelta]>(normWireRef(() => parseEn1993BuiltUpMemberDelta))), weldGroups: normWireRequired(normWireAny<[En1993WeldGroupDelta]>(normWireRef(() => parseEn1993WeldGroupDelta))), parametricSections: normWireRequired(normWireAny<[En1993ParametricSectionDelta]>(normWireRef(() => parseEn1993ParametricSectionDelta))), hollowJoints: normWireRequired(normWireAny<[En1993HollowJointDelta]>(normWireRef(() => parseEn1993HollowJointDelta))), jointRows: normWireRequired(normWireAny<[En1993JointRowDelta]>(normWireRef(() => parseEn1993JointRowDelta))), frameNodes: normWireRequired(normWireAny<[En1993FrameNodeDelta]>(normWireRef(() => parseEn1993FrameNodeDelta))), frameBars: normWireRequired(normWireAny<[En1993FrameBarDelta]>(normWireRef(() => parseEn1993FrameBarDelta))), placements: normWireRequired(normWireAny<[En1993PlacementDelta]>(normWireRef(() => parseEn1993PlacementDelta))), reliabilityClass: normWireOptional(normWireNullable(normWireLiteral("RC1", "RC2", "RC3"))), jointPlates: normWireRequired(normWireAny<[En1993JointPlateDelta]>(normWireRef(() => parseEn1993JointPlateDelta))), foundationBlocks: normWireRequired(normWireAny<[En1993FoundationBlockDelta]>(normWireRef(() => parseEn1993FoundationBlockDelta))), materialDuctility: normWireRequired(normWireAny<[En1993MaterialDuctilityDelta]>(normWireRef(() => parseEn1993MaterialDuctilityDelta))), formingGains: normWireRequired(normWireAny<[En1993FormingGainDelta]>(normWireRef(() => parseEn1993FormingGainDelta))), bucklingLengths: normWireRequired(normWireAny<[En1993BucklingLengthDelta]>(normWireRef(() => parseEn1993BucklingLengthDelta))), fatigueStresses: normWireRequired(normWireAny<[En1993FatigueStressDelta]>(normWireRef(() => parseEn1993FatigueStressDelta))), frameActions: normWireRequired(normWireAny<[En1993FrameActionDelta]>(normWireRef(() => parseEn1993FrameActionDelta))), frameMemberBases: normWireRequired(normWireAny<[En1993FrameMemberBasisDelta]>(normWireRef(() => parseEn1993FrameMemberBasisDelta))), fastenedJoints: normWireRequired(normWireAny<[En1993FastenedJointDelta]>(normWireRef(() => parseEn1993FastenedJointDelta))) });
export const parseEn1993MaterialDelta: NormWireReader<En1993MaterialDelta> = normWireObject<En1993MaterialDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993MaterialRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993MaterialInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993MaterialRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993MaterialModification))) });
export const parseEn1993MaterialRemoval: NormWireReader<En1993MaterialRemoval> = normWireObject<En1993MaterialRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993MaterialInsertion: NormWireReader<En1993MaterialInsertion> = normWireObject<En1993MaterialInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseSteelMaterial) });
export const parseEn1993MaterialRelocation: NormWireReader<En1993MaterialRelocation> = normWireObject<En1993MaterialRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993MaterialModification: NormWireReader<En1993MaterialModification> = normWireObject<En1993MaterialModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993MaterialPatch)) });
export const parseEn1993MaterialPatch: NormWireReader<En1993MaterialPatch> = normWireObject<En1993MaterialPatch>({ grade: normWireOptional(normWireNullable(normWireString)), fy: normWireOptional(normWireNullable(normWireNumber)), fu: normWireOptional(normWireNullable(normWireNumber)), eModulus: normWireOptional(normWireNullable(normWireNumber)), gModulus: normWireOptional(normWireNullable(normWireNumber)), subgrade: normWireOptional(normWireNullable(normWireString)), kind: normWireOptional(normWireNullable(parseMaterialKind)), rambergOsgoodN: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993SectionDelta: NormWireReader<En1993SectionDelta> = normWireObject<En1993SectionDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993SectionRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993SectionInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993SectionRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993SectionModification))) });
export const parseEn1993SectionRemoval: NormWireReader<En1993SectionRemoval> = normWireObject<En1993SectionRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993SectionInsertion: NormWireReader<En1993SectionInsertion> = normWireObject<En1993SectionInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseSteelSection) });
export const parseEn1993SectionRelocation: NormWireReader<En1993SectionRelocation> = normWireObject<En1993SectionRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993SectionModification: NormWireReader<En1993SectionModification> = normWireObject<En1993SectionModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993SectionPatch)) });
export const parseEn1993SectionPatch: NormWireReader<En1993SectionPatch> = normWireObject<En1993SectionPatch>({ designation: normWireOptional(normWireNullable(normWireString)), kind: normWireOptional(normWireNullable(parseSectionKind)), h: normWireOptional(normWireNullable(normWireNumber)), b: normWireOptional(normWireNullable(normWireNumber)), tw: normWireOptional(normWireNullable(normWireNumber)), tf: normWireOptional(normWireNullable(normWireNumber)), r: normWireOptional(normWireNullable(normWireNumber)), area: normWireOptional(normWireNullable(normWireNumber)), shearAreaY: normWireOptional(normWireNullable(normWireNumber)), shearAreaZ: normWireOptional(normWireNullable(normWireNumber)), iy: normWireOptional(normWireNullable(normWireNumber)), iz: normWireOptional(normWireNullable(normWireNumber)), it: normWireOptional(normWireNullable(normWireNumber)), iw: normWireOptional(normWireNullable(normWireNumber)), wElY: normWireOptional(normWireNullable(normWireNumber)), wElZ: normWireOptional(normWireNullable(normWireNumber)), wPlY: normWireOptional(normWireNullable(normWireNumber)), wPlZ: normWireOptional(normWireNullable(normWireNumber)), areaNet: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993MemberDelta: NormWireReader<En1993MemberDelta> = normWireObject<En1993MemberDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993MemberRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993MemberInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993MemberRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993MemberModification))) });
export const parseEn1993MemberRemoval: NormWireReader<En1993MemberRemoval> = normWireObject<En1993MemberRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993MemberInsertion: NormWireReader<En1993MemberInsertion> = normWireObject<En1993MemberInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseSteelMember) });
export const parseEn1993MemberRelocation: NormWireReader<En1993MemberRelocation> = normWireObject<En1993MemberRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993MemberModification: NormWireReader<En1993MemberModification> = normWireObject<En1993MemberModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993MemberPatch)) });
export const parseEn1993MemberPatch: NormWireReader<En1993MemberPatch> = normWireObject<En1993MemberPatch>({ label: normWireOptional(normWireNullable(normWireString)), memberType: normWireOptional(normWireNullable(parseSteelMemberType)), sectionId: normWireOptional(normWireNullable(normWireString)), materialId: normWireOptional(normWireNullable(normWireString)), length: normWireOptional(normWireNullable(normWireNumber)), bucklingLengthY: normWireOptional(normWireNullable(normWireNumber)), bucklingLengthZ: normWireOptional(normWireNullable(normWireNumber)), ltbLength: normWireOptional(normWireNullable(normWireNumber)), ltbRestraintSpacing: normWireOptional(normWireNullable(normWireNumber)), loadApplication: normWireOptional(normWireNullable(parseLoadApplication)), endMomentRatioPsi: normWireOptional(normWireNullable(normWireNumber)), momentDiagram: normWireOptional(normWireNullable(parseMomentDiagram)), deflectionLimitRatio: normWireOptional(normWireNullable(normWireNumber)), analysis: normWireOptional(normWireNullable(parseAnalysisKind)), stainlessForming: normWireOptional(normWireNullable(parseStainlessForming)), designTemperature: normWireOptional(normWireNullable(normWireNumber)), toughnessStressRatio: normWireOptional(normWireNullable(normWireNumber)), radiationShift: normWireOptional(normWireNullable(normWireNumber)), stressShift: normWireOptional(normWireNullable(normWireNumber)), marginShift: normWireOptional(normWireNullable(normWireNumber)), strainRateShift: normWireOptional(normWireNullable(normWireNumber)), coldFormingRadius: normWireOptional(normWireNullable(normWireNumber)), webMaterialId: normWireOptional(normWireNullable(normWireString)), torsionLoad: normWireOptional(normWireNullable(parseTorsionLoad)), torquePosition: normWireOptional(normWireNullable(normWireNumber)), warpingSupport: normWireOptional(normWireNullable(parseWarpingSupport)) });
export const parseEn1993LoadCaseDelta: NormWireReader<En1993LoadCaseDelta> = normWireObject<En1993LoadCaseDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993LoadCaseRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993LoadCaseInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993LoadCaseRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993LoadCaseModification))) });
export const parseEn1993LoadCaseRemoval: NormWireReader<En1993LoadCaseRemoval> = normWireObject<En1993LoadCaseRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993LoadCaseInsertion: NormWireReader<En1993LoadCaseInsertion> = normWireObject<En1993LoadCaseInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseLoadCase) });
export const parseEn1993LoadCaseRelocation: NormWireReader<En1993LoadCaseRelocation> = normWireObject<En1993LoadCaseRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993LoadCaseModification: NormWireReader<En1993LoadCaseModification> = normWireObject<En1993LoadCaseModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993LoadCasePatch)) });
export const parseEn1993LoadCasePatch: NormWireReader<En1993LoadCasePatch> = normWireObject<En1993LoadCasePatch>({ name: normWireOptional(normWireNullable(normWireString)), kind: normWireOptional(normWireNullable(parseActionKind)), category: normWireOptional(normWireNullable(normWireString)) });
export const parseEn1993MemberActionDelta: NormWireReader<En1993MemberActionDelta> = normWireObject<En1993MemberActionDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993MemberActionRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993MemberActionInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993MemberActionRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993MemberActionModification))) });
export const parseEn1993MemberActionRemoval: NormWireReader<En1993MemberActionRemoval> = normWireObject<En1993MemberActionRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993MemberActionInsertion: NormWireReader<En1993MemberActionInsertion> = normWireObject<En1993MemberActionInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseMemberAction) });
export const parseEn1993MemberActionRelocation: NormWireReader<En1993MemberActionRelocation> = normWireObject<En1993MemberActionRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993MemberActionModification: NormWireReader<En1993MemberActionModification> = normWireObject<En1993MemberActionModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993MemberActionPatch)) });
export const parseEn1993MemberActionPatch: NormWireReader<En1993MemberActionPatch> = normWireObject<En1993MemberActionPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), loadCaseId: normWireOptional(normWireNullable(normWireString)), action: normWireOptional(normWireNullable(parseDesignAction)) });
export const parseEn1993JointDelta: NormWireReader<En1993JointDelta> = normWireObject<En1993JointDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993JointRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993JointInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993JointRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993JointModification))) });
export const parseEn1993JointRemoval: NormWireReader<En1993JointRemoval> = normWireObject<En1993JointRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993JointInsertion: NormWireReader<En1993JointInsertion> = normWireObject<En1993JointInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseSteelJoint) });
export const parseEn1993JointRelocation: NormWireReader<En1993JointRelocation> = normWireObject<En1993JointRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993JointModification: NormWireReader<En1993JointModification> = normWireObject<En1993JointModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993JointPatch)) });
export const parseEn1993JointPatch: NormWireReader<En1993JointPatch> = normWireObject<En1993JointPatch>({ kind: normWireOptional(normWireNullable(parseJointKind)), memberId: normWireOptional(normWireNullable(normWireString)), boltClass: normWireOptional(normWireNullable(normWireString)), boltDiameter: normWireOptional(normWireNullable(normWireNumber)), boltRows: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), boltsPerRow: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), pitch: normWireOptional(normWireNullable(normWireNumber)), gauge: normWireOptional(normWireNullable(normWireNumber)), endDistance: normWireOptional(normWireNullable(normWireNumber)), edgeDistance: normWireOptional(normWireNullable(normWireNumber)), shearPlanes: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), plateThickness: normWireOptional(normWireNullable(normWireNumber)), plateFu: normWireOptional(normWireNullable(normWireNumber)), weldThroat: normWireOptional(normWireNullable(normWireNumber)), weldLength: normWireOptional(normWireNullable(normWireNumber)), weldFu: normWireOptional(normWireNullable(normWireNumber)), weldGrade: normWireOptional(normWireNullable(normWireString)), actions: normWireOptional(normWireNullable(normWireArray(parseJointForceAction))), category: normWireOptional(normWireNullable(parseJointCategory)), frictionMu: normWireOptional(normWireNullable(normWireNumber)), preloadForce: normWireOptional(normWireNullable(normWireNumber)), slipFactorKs: normWireOptional(normWireNullable(normWireNumber)), frictionSurfaces: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), plateFy: normWireOptional(normWireNullable(normWireNumber)), plateWidth: normWireOptional(normWireNullable(normWireNumber)), boltToWeb: normWireOptional(normWireNullable(normWireNumber)), extensionToFlange: normWireOptional(normWireNullable(normWireNumber)), extensionEdge: normWireOptional(normWireNullable(normWireNumber)), alphaFactor: normWireOptional(normWireNullable(normWireNumber)), boltStressArea: normWireOptional(normWireNullable(normWireNumber)), boltGripLength: normWireOptional(normWireNullable(normWireNumber)), columnFlangeThickness: normWireOptional(normWireNullable(normWireNumber)), columnWebThickness: normWireOptional(normWireNullable(normWireNumber)), columnWebDepth: normWireOptional(normWireNullable(normWireNumber)), columnWebShearArea: normWireOptional(normWireNullable(normWireNumber)), columnRootRadius: normWireOptional(normWireNullable(normWireNumber)), columnFy: normWireOptional(normWireNullable(normWireNumber)), columnWebStress: normWireOptional(normWireNullable(normWireNumber)), beamFlangeThickness: normWireOptional(normWireNullable(normWireNumber)), compressionLever: normWireOptional(normWireNullable(normWireNumber)), momentDemand: normWireOptional(normWireNullable(normWireNumber)), beamMomentResistance: normWireOptional(normWireNullable(normWireNumber)), columnMomentResistance: normWireOptional(normWireNullable(normWireNumber)), beamInertia: normWireOptional(normWireNullable(normWireNumber)), beamSpan: normWireOptional(normWireNullable(normWireNumber)), bracedFrame: normWireOptional(normWireNullable(normWireInteger)), stiffnessRatio: normWireOptional(normWireNullable(normWireNumber)), transformationParameter: normWireOptional(normWireNullable(normWireNumber)), assumedClass: normWireOptional(normWireNullable(parseStiffnessClass)), basePlateLength: normWireOptional(normWireNullable(normWireNumber)), concreteFcd: normWireOptional(normWireNullable(normWireNumber)), concreteModulus: normWireOptional(normWireNullable(normWireNumber)), concreteAreaA0: normWireOptional(normWireNullable(normWireNumber)), concreteAreaA1: normWireOptional(normWireNullable(normWireNumber)), jointCoefficient: normWireOptional(normWireNullable(normWireNumber)), anchorArea: normWireOptional(normWireNullable(normWireNumber)), anchorLength: normWireOptional(normWireNullable(normWireNumber)), baseNormalForce: normWireOptional(normWireNullable(normWireNumber)), baseMoment: normWireOptional(normWireNullable(normWireNumber)), baseLeverLeft: normWireOptional(normWireNullable(normWireNumber)), baseLeverRight: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993FatigueDetailDelta: NormWireReader<En1993FatigueDetailDelta> = normWireObject<En1993FatigueDetailDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993FatigueDetailRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993FatigueDetailInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993FatigueDetailRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993FatigueDetailModification))) });
export const parseEn1993FatigueDetailRemoval: NormWireReader<En1993FatigueDetailRemoval> = normWireObject<En1993FatigueDetailRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FatigueDetailInsertion: NormWireReader<En1993FatigueDetailInsertion> = normWireObject<En1993FatigueDetailInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseFatigueDetail) });
export const parseEn1993FatigueDetailRelocation: NormWireReader<En1993FatigueDetailRelocation> = normWireObject<En1993FatigueDetailRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FatigueDetailModification: NormWireReader<En1993FatigueDetailModification> = normWireObject<En1993FatigueDetailModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993FatigueDetailPatch)) });
export const parseEn1993FatigueDetailPatch: NormWireReader<En1993FatigueDetailPatch> = normWireObject<En1993FatigueDetailPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), category: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), method: normWireOptional(normWireNullable(parseFatigueMethod)), spectrum: normWireOptional(normWireNullable(normWireArray(parseFatigueBand))), stressHistory: normWireOptional(normWireNullable(normWireString)) });
export const parseEn1993FireExposureDelta: NormWireReader<En1993FireExposureDelta> = normWireObject<En1993FireExposureDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993FireExposureRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993FireExposureInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993FireExposureRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993FireExposureModification))) });
export const parseEn1993FireExposureRemoval: NormWireReader<En1993FireExposureRemoval> = normWireObject<En1993FireExposureRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FireExposureInsertion: NormWireReader<En1993FireExposureInsertion> = normWireObject<En1993FireExposureInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseFireExposure) });
export const parseEn1993FireExposureRelocation: NormWireReader<En1993FireExposureRelocation> = normWireObject<En1993FireExposureRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FireExposureModification: NormWireReader<En1993FireExposureModification> = normWireObject<En1993FireExposureModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993FireExposurePatch)) });
export const parseEn1993FireExposurePatch: NormWireReader<En1993FireExposurePatch> = normWireObject<En1993FireExposurePatch>({ memberId: normWireOptional(normWireNullable(normWireString)), rating: normWireOptional(normWireNullable(parseFireRating)), protectionThickness: normWireOptional(normWireNullable(normWireNumber)), sectionFactor: normWireOptional(normWireNullable(normWireNumber)), mu0: normWireOptional(normWireNullable(normWireNumber)), designTemperature: normWireOptional(normWireNullable(normWireNumber)), protectionConductivity: normWireOptional(normWireNullable(normWireNumber)), protectionDensity: normWireOptional(normWireNullable(normWireNumber)), protectionSpecificHeat: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993ColdFormedMemberDelta: NormWireReader<En1993ColdFormedMemberDelta> = normWireObject<En1993ColdFormedMemberDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993ColdFormedMemberRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993ColdFormedMemberInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993ColdFormedMemberRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993ColdFormedMemberModification))) });
export const parseEn1993ColdFormedMemberRemoval: NormWireReader<En1993ColdFormedMemberRemoval> = normWireObject<En1993ColdFormedMemberRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993ColdFormedMemberInsertion: NormWireReader<En1993ColdFormedMemberInsertion> = normWireObject<En1993ColdFormedMemberInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseColdFormedMember) });
export const parseEn1993ColdFormedMemberRelocation: NormWireReader<En1993ColdFormedMemberRelocation> = normWireObject<En1993ColdFormedMemberRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993ColdFormedMemberModification: NormWireReader<En1993ColdFormedMemberModification> = normWireObject<En1993ColdFormedMemberModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993ColdFormedMemberPatch)) });
export const parseEn1993ColdFormedMemberPatch: NormWireReader<En1993ColdFormedMemberPatch> = normWireObject<En1993ColdFormedMemberPatch>({ bBar: normWireOptional(normWireNullable(normWireNumber)), thickness: normWireOptional(normWireNullable(normWireNumber)), kSigma: normWireOptional(normWireNullable(normWireNumber)), psi: normWireOptional(normWireNullable(normWireNumber)), fy: normWireOptional(normWireNullable(normWireNumber)), grossResistance: normWireOptional(normWireNullable(normWireNumber)), actions: normWireOptional(normWireNullable(normWireArray(parseForceAction))), flangeWidth: normWireOptional(normWireNullable(normWireNumber)), lipLength: normWireOptional(normWireNullable(normWireNumber)), webHeight: normWireOptional(normWireNullable(normWireNumber)), webSlope: normWireOptional(normWireNullable(normWireNumber)), oppositeFlangeWidth: normWireOptional(normWireNullable(normWireNumber)), springFactor: normWireOptional(normWireNullable(normWireNumber)), cornerRadius: normWireOptional(normWireNullable(normWireNumber)), flangeStress: normWireOptional(normWireNullable(normWireNumber)), shearForce: normWireOptional(normWireNullable(normWireNumber)), supportReaction: normWireOptional(normWireNullable(normWireNumber)), supportLength: normWireOptional(normWireNullable(normWireNumber)), cripplingCategory: normWireOptional(normWireNullable(normWireInteger)), stiffenedSupport: normWireOptional(normWireNullable(normWireInteger)), bendingMoment: normWireOptional(normWireNullable(normWireNumber)), momentResistance: normWireOptional(normWireNullable(normWireNumber)), sheetingStiffness: normWireOptional(normWireNullable(normWireNumber)), requiredRestraintStiffness: normWireOptional(normWireNullable(normWireNumber)), purlinSpan: normWireOptional(normWireNullable(normWireNumber)), purlinLoad: normWireOptional(normWireNullable(normWireNumber)), purlinLateralRatio: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993PlatedPanelDelta: NormWireReader<En1993PlatedPanelDelta> = normWireObject<En1993PlatedPanelDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993PlatedPanelRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993PlatedPanelInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993PlatedPanelRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993PlatedPanelModification))) });
export const parseEn1993PlatedPanelRemoval: NormWireReader<En1993PlatedPanelRemoval> = normWireObject<En1993PlatedPanelRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993PlatedPanelInsertion: NormWireReader<En1993PlatedPanelInsertion> = normWireObject<En1993PlatedPanelInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parsePlatedPanel) });
export const parseEn1993PlatedPanelRelocation: NormWireReader<En1993PlatedPanelRelocation> = normWireObject<En1993PlatedPanelRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993PlatedPanelModification: NormWireReader<En1993PlatedPanelModification> = normWireObject<En1993PlatedPanelModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993PlatedPanelPatch)) });
export const parseEn1993PlatedPanelPatch: NormWireReader<En1993PlatedPanelPatch> = normWireObject<En1993PlatedPanelPatch>({ a: normWireOptional(normWireNullable(normWireNumber)), b: normWireOptional(normWireNullable(normWireNumber)), thickness: normWireOptional(normWireNullable(normWireNumber)), fy: normWireOptional(normWireNullable(normWireNumber)), kSigma: normWireOptional(normWireNullable(normWireNumber)), actions: normWireOptional(normWireNullable(normWireArray(parseForceAction))), stiffenerInertia: normWireOptional(normWireNullable(normWireNumber)), stiffenerArea: normWireOptional(normWireNullable(normWireNumber)), stiffenerEccentricity: normWireOptional(normWireNullable(normWireNumber)), stiffenerAlpha: normWireOptional(normWireNullable(normWireNumber)), compressionStress: normWireOptional(normWireNullable(normWireNumber)), halfWidth: normWireOptional(normWireNullable(normWireNumber)), equivalentSpan: normWireOptional(normWireNullable(normWireNumber)), hogging: normWireOptional(normWireNullable(normWireInteger)), patchType: normWireOptional(normWireNullable(parsePatchLoadType)), patchForce: normWireOptional(normWireNullable(normWireNumber)), patchLength: normWireOptional(normWireNullable(normWireNumber)), patchOffset: normWireOptional(normWireNullable(normWireNumber)), flangeThickness: normWireOptional(normWireNullable(normWireNumber)), flangeWidth: normWireOptional(normWireNullable(normWireNumber)), flangeFy: normWireOptional(normWireNullable(normWireNumber)), longitudinalOffset: normWireOptional(normWireNullable(normWireNumber)), longitudinalInertia: normWireOptional(normWireNullable(normWireNumber)), patchMoment: normWireOptional(normWireNullable(normWireNumber)), patchMomentResistance: normWireOptional(normWireNullable(normWireNumber)), bendingMoment: normWireOptional(normWireNullable(normWireNumber)), plasticMoment: normWireOptional(normWireNullable(normWireNumber)), flangeMoment: normWireOptional(normWireNullable(normWireNumber)), shearForce: normWireOptional(normWireNullable(normWireNumber)), transverseStiffenerInertia: normWireOptional(normWireNullable(normWireNumber)), flangeInducedK: normWireOptional(normWireNullable(normWireNumber)), flangeArea: normWireOptional(normWireNullable(normWireNumber)), stressX: normWireOptional(normWireNullable(normWireNumber)), stressZ: normWireOptional(normWireNullable(normWireNumber)), shearStress: normWireOptional(normWireNullable(normWireNumber)), alphaCr: normWireOptional(normWireNullable(normWireNumber)), corrugationFlat: normWireOptional(normWireNullable(normWireNumber)), corrugationProjection: normWireOptional(normWireNullable(normWireNumber)), corrugationDepth: normWireOptional(normWireNullable(normWireNumber)), webHeight: normWireOptional(normWireNullable(normWireNumber)), stiffenerSpacing: normWireOptional(normWireNullable(normWireNumber)), stiffenerShape: normWireOptional(normWireNullable(parseStiffenerShape)), stiffenerDepth: normWireOptional(normWireNullable(normWireNumber)), stiffenerThickness: normWireOptional(normWireNullable(normWireNumber)), stiffenerFlangeWidth: normWireOptional(normWireNullable(normWireNumber)), stiffenerFlangeThickness: normWireOptional(normWireNullable(normWireNumber)), lateralPressure: normWireOptional(normWireNullable(normWireNumber)), lateralPressureCoefficient: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993SiloShellDelta: NormWireReader<En1993SiloShellDelta> = normWireObject<En1993SiloShellDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993SiloShellRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993SiloShellInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993SiloShellRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993SiloShellModification))) });
export const parseEn1993SiloShellRemoval: NormWireReader<En1993SiloShellRemoval> = normWireObject<En1993SiloShellRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993SiloShellInsertion: NormWireReader<En1993SiloShellInsertion> = normWireObject<En1993SiloShellInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseSiloShell) });
export const parseEn1993SiloShellRelocation: NormWireReader<En1993SiloShellRelocation> = normWireObject<En1993SiloShellRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993SiloShellModification: NormWireReader<En1993SiloShellModification> = normWireObject<En1993SiloShellModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993SiloShellPatch)) });
export const parseEn1993SiloShellPatch: NormWireReader<En1993SiloShellPatch> = normWireObject<En1993SiloShellPatch>({ thickness: normWireOptional(normWireNullable(normWireNumber)), radius: normWireOptional(normWireNullable(normWireNumber)), depth: normWireOptional(normWireNullable(normWireNumber)), k: normWireOptional(normWireNullable(normWireNumber)), gamma: normWireOptional(normWireNullable(normWireNumber)), fy: normWireOptional(normWireNullable(normWireNumber)), length: normWireOptional(normWireNullable(normWireNumber)), fabricationClass: normWireOptional(normWireNullable(parseFabricationToleranceClass)), meridionalStress: normWireOptional(normWireNullable(normWireNumber)), hoopStress: normWireOptional(normWireNullable(normWireNumber)), shellShearStress: normWireOptional(normWireNullable(normWireNumber)), boundaryFactorAxial: normWireOptional(normWireNullable(normWireNumber)), boundaryFactorHoop: normWireOptional(normWireNullable(normWireNumber)), boundaryFactorShear: normWireOptional(normWireNullable(normWireNumber)), wallFriction: normWireOptional(normWireNullable(normWireNumber)), planArea: normWireOptional(normWireNullable(normWireNumber)), planPerimeter: normWireOptional(normWireNullable(normWireNumber)), liquidDensity: normWireOptional(normWireNullable(normWireNumber)), liquidHeight: normWireOptional(normWireNullable(normWireNumber)), courseElevation: normWireOptional(normWireNullable(normWireNumber)), internalPressure: normWireOptional(normWireNullable(normWireNumber)), externalPressure: normWireOptional(normWireNullable(normWireNumber)), temperatureChange: normWireOptional(normWireNullable(normWireNumber)), ringSpacing: normWireOptional(normWireNullable(normWireNumber)), ringArea: normWireOptional(normWireNullable(normWireNumber)), ringInertia: normWireOptional(normWireNullable(normWireNumber)), ringOffset: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993TensionComponentDelta: NormWireReader<En1993TensionComponentDelta> = normWireObject<En1993TensionComponentDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993TensionComponentRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993TensionComponentInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993TensionComponentRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993TensionComponentModification))) });
export const parseEn1993TensionComponentRemoval: NormWireReader<En1993TensionComponentRemoval> = normWireObject<En1993TensionComponentRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993TensionComponentInsertion: NormWireReader<En1993TensionComponentInsertion> = normWireObject<En1993TensionComponentInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseTensionComponent) });
export const parseEn1993TensionComponentRelocation: NormWireReader<En1993TensionComponentRelocation> = normWireObject<En1993TensionComponentRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993TensionComponentModification: NormWireReader<En1993TensionComponentModification> = normWireObject<En1993TensionComponentModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993TensionComponentPatch)) });
export const parseEn1993TensionComponentPatch: NormWireReader<En1993TensionComponentPatch> = normWireObject<En1993TensionComponentPatch>({ fUk: normWireOptional(normWireNullable(normWireNumber)), fK: normWireOptional(normWireNullable(normWireNumber)), actions: normWireOptional(normWireNullable(normWireArray(parseForceAction))), partialFactor: normWireOptional(normWireNullable(normWireNumber)), cableLength: normWireOptional(normWireNullable(normWireNumber)), cableWeight: normWireOptional(normWireNullable(normWireNumber)), cableHorizontalForce: normWireOptional(normWireNullable(normWireNumber)), cableModulus: normWireOptional(normWireNullable(normWireNumber)), cableStress: normWireOptional(normWireNullable(normWireNumber)), cableUnitWeight: normWireOptional(normWireNullable(normWireNumber)), threadFactor: normWireOptional(normWireNullable(normWireNumber)), rodStressArea: normWireOptional(normWireNullable(normWireNumber)), rodFu: normWireOptional(normWireNullable(normWireNumber)), pinDiameter: normWireOptional(normWireNullable(normWireNumber)), pinPlateThickness: normWireOptional(normWireNullable(normWireNumber)), pinGap: normWireOptional(normWireNullable(normWireNumber)), pinOuterThickness: normWireOptional(normWireNullable(normWireNumber)), pinInnerThickness: normWireOptional(normWireNullable(normWireNumber)), pinFup: normWireOptional(normWireNullable(normWireNumber)), pinFyp: normWireOptional(normWireNullable(normWireNumber)), pinPlateFy: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993BridgeFatigueDelta: NormWireReader<En1993BridgeFatigueDelta> = normWireObject<En1993BridgeFatigueDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993BridgeFatigueRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993BridgeFatigueInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993BridgeFatigueRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993BridgeFatigueModification))) });
export const parseEn1993BridgeFatigueRemoval: NormWireReader<En1993BridgeFatigueRemoval> = normWireObject<En1993BridgeFatigueRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993BridgeFatigueInsertion: NormWireReader<En1993BridgeFatigueInsertion> = normWireObject<En1993BridgeFatigueInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseBridgeFatigue) });
export const parseEn1993BridgeFatigueRelocation: NormWireReader<En1993BridgeFatigueRelocation> = normWireObject<En1993BridgeFatigueRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993BridgeFatigueModification: NormWireReader<En1993BridgeFatigueModification> = normWireObject<En1993BridgeFatigueModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993BridgeFatiguePatch)) });
export const parseEn1993BridgeFatiguePatch: NormWireReader<En1993BridgeFatiguePatch> = normWireObject<En1993BridgeFatiguePatch>({ memberId: normWireOptional(normWireNullable(normWireString)), lambda: normWireOptional(normWireNullable(normWireNumber)), phi2: normWireOptional(normWireNullable(normWireNumber)), deltaSigmaP: normWireOptional(normWireNullable(normWireNumber)), category: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), method: normWireOptional(normWireNullable(parseFatigueMethod)), heavyVehicleWeight: normWireOptional(normWireNullable(normWireNumber)), referenceWeight: normWireOptional(normWireNullable(normWireNumber)), observedTraffic: normWireOptional(normWireNullable(normWireNumber)), designLife: normWireOptional(normWireNullable(normWireNumber)), secondLaneTraffic: normWireOptional(normWireNullable(normWireNumber)), secondLaneWeight: normWireOptional(normWireNullable(normWireNumber)), lambda1: normWireOptional(normWireNullable(normWireNumber)), lambdaMax: normWireOptional(normWireNullable(normWireNumber)), detailId: normWireOptional(normWireNullable(normWireString)), plateThickness: normWireOptional(normWireNullable(normWireNumber)), deckThickness: normWireOptional(normWireNullable(normWireNumber)), troughSpacing: normWireOptional(normWireNullable(normWireNumber)), troughWidth: normWireOptional(normWireNullable(normWireNumber)), troughThickness: normWireOptional(normWireNullable(normWireNumber)), crossbeamSpacing: normWireOptional(normWireNullable(normWireNumber)), surfacingThickness: normWireOptional(normWireNullable(normWireNumber)), wheelLoad: normWireOptional(normWireNullable(normWireNumber)), wheelPatch: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993TowerLegDelta: NormWireReader<En1993TowerLegDelta> = normWireObject<En1993TowerLegDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993TowerLegRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993TowerLegInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993TowerLegRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993TowerLegModification))) });
export const parseEn1993TowerLegRemoval: NormWireReader<En1993TowerLegRemoval> = normWireObject<En1993TowerLegRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993TowerLegInsertion: NormWireReader<En1993TowerLegInsertion> = normWireObject<En1993TowerLegInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseTowerLeg) });
export const parseEn1993TowerLegRelocation: NormWireReader<En1993TowerLegRelocation> = normWireObject<En1993TowerLegRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993TowerLegModification: NormWireReader<En1993TowerLegModification> = normWireObject<En1993TowerLegModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993TowerLegPatch)) });
export const parseEn1993TowerLegPatch: NormWireReader<En1993TowerLegPatch> = normWireObject<En1993TowerLegPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), forceCoefficient: normWireOptional(normWireNullable(normWireNumber)), dynamicFactor: normWireOptional(normWireNullable(normWireNumber)), actions: normWireOptional(normWireNullable(normWireArray(parseForceAction))), latticeKind: normWireOptional(normWireNullable(parseLatticeKind)), slenderness: normWireOptional(normWireNullable(normWireNumber)), diameter: normWireOptional(normWireNullable(normWireNumber)), height: normWireOptional(normWireNullable(normWireNumber)), bendingStiffness: normWireOptional(normWireNullable(normWireNumber)), massPerLength: normWireOptional(normWireNullable(normWireNumber)), strouhal: normWireOptional(normWireNullable(normWireNumber)), logDecrement: normWireOptional(normWireNullable(normWireNumber)), airDensity: normWireOptional(normWireNullable(normWireNumber)), meanWindSpeed: normWireOptional(normWireNullable(normWireNumber)), crossingRate: normWireOptional(normWireNullable(normWireNumber)), designLife: normWireOptional(normWireNullable(normWireNumber)), kFactor: normWireOptional(normWireNullable(normWireNumber)), kwFactor: normWireOptional(normWireNullable(normWireNumber)), lateralForceCoefficient: normWireOptional(normWireNullable(normWireNumber)), permittedAmplitudeRatio: normWireOptional(normWireNullable(normWireNumber)), ringMode: normWireOptional(normWireNullable(normWireNumber)), shellThickness: normWireOptional(normWireNullable(normWireNumber)), guyArea: normWireOptional(normWireNullable(normWireNumber)), guyAngle: normWireOptional(normWireNullable(normWireNumber)), guyLength: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993PileDelta: NormWireReader<En1993PileDelta> = normWireObject<En1993PileDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993PileRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993PileInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993PileRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993PileModification))) });
export const parseEn1993PileRemoval: NormWireReader<En1993PileRemoval> = normWireObject<En1993PileRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993PileInsertion: NormWireReader<En1993PileInsertion> = normWireObject<En1993PileInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseSteelPile) });
export const parseEn1993PileRelocation: NormWireReader<En1993PileRelocation> = normWireObject<En1993PileRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993PileModification: NormWireReader<En1993PileModification> = normWireObject<En1993PileModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993PilePatch)) });
export const parseEn1993PilePatch: NormWireReader<En1993PilePatch> = normWireObject<En1993PilePatch>({ sectionId: normWireOptional(normWireNullable(normWireString)), materialId: normWireOptional(normWireNullable(normWireString)), drivingStress: normWireOptional(normWireNullable(normWireNumber)), embeddedLength: normWireOptional(normWireNullable(normWireNumber)), shaftPerimeter: normWireOptional(normWireNullable(normWireNumber)), actions: normWireOptional(normWireNullable(normWireArray(parseForceAction))), environment: normWireOptional(normWireNullable(parseSheetPileEnvironment)), designLife: normWireOptional(normWireNullable(normWireNumber)), wallThickness: normWireOptional(normWireNullable(normWireNumber)), interlockBendingFactor: normWireOptional(normWireNullable(normWireNumber)), interlockStiffnessFactor: normWireOptional(normWireNullable(normWireNumber)), sectionModulus: normWireOptional(normWireNullable(normWireNumber)), shearArea: normWireOptional(normWireNullable(normWireNumber)), designMoment: normWireOptional(normWireNullable(normWireNumber)), designShear: normWireOptional(normWireNullable(normWireNumber)), interlockShear: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993CraneRunwayDelta: NormWireReader<En1993CraneRunwayDelta> = normWireObject<En1993CraneRunwayDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993CraneRunwayRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993CraneRunwayInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993CraneRunwayRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993CraneRunwayModification))) });
export const parseEn1993CraneRunwayRemoval: NormWireReader<En1993CraneRunwayRemoval> = normWireObject<En1993CraneRunwayRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993CraneRunwayInsertion: NormWireReader<En1993CraneRunwayInsertion> = normWireObject<En1993CraneRunwayInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseCraneRunway) });
export const parseEn1993CraneRunwayRelocation: NormWireReader<En1993CraneRunwayRelocation> = normWireObject<En1993CraneRunwayRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993CraneRunwayModification: NormWireReader<En1993CraneRunwayModification> = normWireObject<En1993CraneRunwayModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993CraneRunwayPatch)) });
export const parseEn1993CraneRunwayPatch: NormWireReader<En1993CraneRunwayPatch> = normWireObject<En1993CraneRunwayPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), wheelContactLength: normWireOptional(normWireNullable(normWireNumber)), dispersion: normWireOptional(normWireNullable(normWireNumber)), webThickness: normWireOptional(normWireNullable(normWireNumber)), fy: normWireOptional(normWireNullable(normWireNumber)), phi: normWireOptional(normWireNullable(normWireNumber)), actions: normWireOptional(normWireNullable(normWireArray(parseForceAction))), maxWheelLoad: normWireOptional(normWireNullable(normWireNumber)), loadFactor: normWireOptional(normWireNullable(normWireNumber)), railInertia: normWireOptional(normWireNullable(normWireNumber)), railFixed: normWireOptional(normWireNullable(normWireInteger)), flangeEffectiveWidth: normWireOptional(normWireNullable(normWireNumber)), flangeThickness: normWireOptional(normWireNullable(normWireNumber)), wheelEccentricity: normWireOptional(normWireNullable(normWireNumber)), torsionConstant: normWireOptional(normWireNullable(normWireNumber)), longitudinalStress: normWireOptional(normWireNullable(normWireNumber)), railRadius: normWireOptional(normWireNullable(normWireNumber)), railContactWidth: normWireOptional(normWireNullable(normWireNumber)), craneNu: normWireOptional(normWireNullable(normWireNumber)), craneKq: normWireOptional(normWireNullable(normWireNumber)), damageExponent: normWireOptional(normWireNullable(normWireNumber)), stressRange: normWireOptional(normWireNullable(normWireNumber)), detailCategoryStress: normWireOptional(normWireNullable(normWireNumber)), fatiguePhi: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993StoreyDelta: NormWireReader<En1993StoreyDelta> = normWireObject<En1993StoreyDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993StoreyRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993StoreyInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993StoreyRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993StoreyModification))) });
export const parseEn1993StoreyRemoval: NormWireReader<En1993StoreyRemoval> = normWireObject<En1993StoreyRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993StoreyInsertion: NormWireReader<En1993StoreyInsertion> = normWireObject<En1993StoreyInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseStorey) });
export const parseEn1993StoreyRelocation: NormWireReader<En1993StoreyRelocation> = normWireObject<En1993StoreyRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993StoreyModification: NormWireReader<En1993StoreyModification> = normWireObject<En1993StoreyModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993StoreyPatch)) });
export const parseEn1993StoreyPatch: NormWireReader<En1993StoreyPatch> = normWireObject<En1993StoreyPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), label: normWireOptional(normWireNullable(normWireString)), height: normWireOptional(normWireNullable(normWireNumber)), verticalLoad: normWireOptional(normWireNullable(normWireNumber)), horizontalLoad: normWireOptional(normWireNullable(normWireNumber)), drift: normWireOptional(normWireNullable(normWireNumber)), columns: normWireOptional(normWireNullable(normWireInteger)), plasticAnalysis: normWireOptional(normWireNullable(normWireInteger)), buildingHeight: normWireOptional(normWireNullable(normWireNumber)), bracingLoad: normWireOptional(normWireNullable(normWireNumber)), bracingLength: normWireOptional(normWireNullable(normWireNumber)), bracingDeflection: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993RestraintDelta: NormWireReader<En1993RestraintDelta> = normWireObject<En1993RestraintDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993RestraintRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993RestraintInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993RestraintRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993RestraintModification))) });
export const parseEn1993RestraintRemoval: NormWireReader<En1993RestraintRemoval> = normWireObject<En1993RestraintRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993RestraintInsertion: NormWireReader<En1993RestraintInsertion> = normWireObject<En1993RestraintInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseRestraint) });
export const parseEn1993RestraintRelocation: NormWireReader<En1993RestraintRelocation> = normWireObject<En1993RestraintRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993RestraintModification: NormWireReader<En1993RestraintModification> = normWireObject<En1993RestraintModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993RestraintPatch)) });
export const parseEn1993RestraintPatch: NormWireReader<En1993RestraintPatch> = normWireObject<En1993RestraintPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), label: normWireOptional(normWireNullable(normWireString)), startSupport: normWireOptional(normWireNullable(parseRestraintSupport)), endSupport: normWireOptional(normWireNullable(parseRestraintSupport)), intermediatePosition: normWireOptional(normWireNullable(normWireNumber)), torsionalSpring: normWireOptional(normWireNullable(normWireNumber)), lateralSpringPosition: normWireOptional(normWireNullable(normWireNumber)), lateralSpringStiffness: normWireOptional(normWireNullable(normWireNumber)), loadHeight: normWireOptional(normWireNullable(normWireNumber)), loadDistribution: normWireOptional(normWireNullable(parseRestraintDistribution)), designMoment: normWireOptional(normWireNullable(normWireNumber)), designAxial: normWireOptional(normWireNullable(normWireNumber)), generalMethod: normWireOptional(normWireNullable(normWireInteger)), hingeToRestraint: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993BuiltUpMemberDelta: NormWireReader<En1993BuiltUpMemberDelta> = normWireObject<En1993BuiltUpMemberDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993BuiltUpMemberRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993BuiltUpMemberInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993BuiltUpMemberRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993BuiltUpMemberModification))) });
export const parseEn1993BuiltUpMemberRemoval: NormWireReader<En1993BuiltUpMemberRemoval> = normWireObject<En1993BuiltUpMemberRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993BuiltUpMemberInsertion: NormWireReader<En1993BuiltUpMemberInsertion> = normWireObject<En1993BuiltUpMemberInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseBuiltUpMember) });
export const parseEn1993BuiltUpMemberRelocation: NormWireReader<En1993BuiltUpMemberRelocation> = normWireObject<En1993BuiltUpMemberRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993BuiltUpMemberModification: NormWireReader<En1993BuiltUpMemberModification> = normWireObject<En1993BuiltUpMemberModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993BuiltUpMemberPatch)) });
export const parseEn1993BuiltUpMemberPatch: NormWireReader<En1993BuiltUpMemberPatch> = normWireObject<En1993BuiltUpMemberPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), label: normWireOptional(normWireNullable(normWireString)), kind: normWireOptional(normWireNullable(parseBuiltUpKind)), chordArea: normWireOptional(normWireNullable(normWireNumber)), chordInertia: normWireOptional(normWireNullable(normWireNumber)), chordDistance: normWireOptional(normWireNullable(normWireNumber)), panelSpacing: normWireOptional(normWireNullable(normWireNumber)), planes: normWireOptional(normWireNullable(normWireInteger)), battenInertia: normWireOptional(normWireNullable(normWireNumber)), diagonalArea: normWireOptional(normWireNullable(normWireNumber)), diagonalLength: normWireOptional(normWireNullable(normWireNumber)), designAxial: normWireOptional(normWireNullable(normWireNumber)), designMoment: normWireOptional(normWireNullable(normWireNumber)), backToBack: normWireOptional(normWireNullable(normWireInteger)), minRadius: normWireOptional(normWireNullable(normWireNumber)), chainSpacing: normWireOptional(normWireNullable(normWireNumber)), chordShape: normWireOptional(normWireNullable(parseChordShape)), chordDepth: normWireOptional(normWireNullable(normWireNumber)), chordWidth: normWireOptional(normWireNullable(normWireNumber)), chordFlangeThickness: normWireOptional(normWireNullable(normWireNumber)), chordWebThickness: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993WeldGroupDelta: NormWireReader<En1993WeldGroupDelta> = normWireObject<En1993WeldGroupDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993WeldGroupRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993WeldGroupInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993WeldGroupRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993WeldGroupModification))) });
export const parseEn1993WeldGroupRemoval: NormWireReader<En1993WeldGroupRemoval> = normWireObject<En1993WeldGroupRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993WeldGroupInsertion: NormWireReader<En1993WeldGroupInsertion> = normWireObject<En1993WeldGroupInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseWeldGroup) });
export const parseEn1993WeldGroupRelocation: NormWireReader<En1993WeldGroupRelocation> = normWireObject<En1993WeldGroupRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993WeldGroupModification: NormWireReader<En1993WeldGroupModification> = normWireObject<En1993WeldGroupModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993WeldGroupPatch)) });
export const parseEn1993WeldGroupPatch: NormWireReader<En1993WeldGroupPatch> = normWireObject<En1993WeldGroupPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), label: normWireOptional(normWireNullable(normWireString)), pattern: normWireOptional(normWireNullable(parseWeldPattern)), throat: normWireOptional(normWireNullable(normWireNumber)), width: normWireOptional(normWireNullable(normWireNumber)), depth: normWireOptional(normWireNullable(normWireNumber)), shearForce: normWireOptional(normWireNullable(normWireNumber)), eccentricity: normWireOptional(normWireNullable(normWireNumber)), steelFu: normWireOptional(normWireNullable(normWireNumber)), betaW: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993ParametricSectionDelta: NormWireReader<En1993ParametricSectionDelta> = normWireObject<En1993ParametricSectionDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993ParametricSectionRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993ParametricSectionInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993ParametricSectionRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993ParametricSectionModification))) });
export const parseEn1993ParametricSectionRemoval: NormWireReader<En1993ParametricSectionRemoval> = normWireObject<En1993ParametricSectionRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993ParametricSectionInsertion: NormWireReader<En1993ParametricSectionInsertion> = normWireObject<En1993ParametricSectionInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseParametricSection) });
export const parseEn1993ParametricSectionRelocation: NormWireReader<En1993ParametricSectionRelocation> = normWireObject<En1993ParametricSectionRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993ParametricSectionModification: NormWireReader<En1993ParametricSectionModification> = normWireObject<En1993ParametricSectionModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993ParametricSectionPatch)) });
export const parseEn1993ParametricSectionPatch: NormWireReader<En1993ParametricSectionPatch> = normWireObject<En1993ParametricSectionPatch>({ sectionId: normWireOptional(normWireNullable(normWireString)), label: normWireOptional(normWireNullable(normWireString)), shape: normWireOptional(normWireNullable(parseParametricShape)), height: normWireOptional(normWireNullable(normWireNumber)), width: normWireOptional(normWireNullable(normWireNumber)), webThickness: normWireOptional(normWireNullable(normWireNumber)), flangeThickness: normWireOptional(normWireNullable(normWireNumber)), tolerance: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993HollowJointDelta: NormWireReader<En1993HollowJointDelta> = normWireObject<En1993HollowJointDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993HollowJointRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993HollowJointInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993HollowJointRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993HollowJointModification))) });
export const parseEn1993HollowJointRemoval: NormWireReader<En1993HollowJointRemoval> = normWireObject<En1993HollowJointRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993HollowJointInsertion: NormWireReader<En1993HollowJointInsertion> = normWireObject<En1993HollowJointInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseHollowJoint) });
export const parseEn1993HollowJointRelocation: NormWireReader<En1993HollowJointRelocation> = normWireObject<En1993HollowJointRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993HollowJointModification: NormWireReader<En1993HollowJointModification> = normWireObject<En1993HollowJointModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993HollowJointPatch)) });
export const parseEn1993HollowJointPatch: NormWireReader<En1993HollowJointPatch> = normWireObject<En1993HollowJointPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), label: normWireOptional(normWireNullable(normWireString)), chordWidth: normWireOptional(normWireNullable(normWireNumber)), chordHeight: normWireOptional(normWireNullable(normWireNumber)), chordThickness: normWireOptional(normWireNullable(normWireNumber)), chordFy: normWireOptional(normWireNullable(normWireNumber)), chordStress: normWireOptional(normWireNullable(normWireNumber)), brace1Width: normWireOptional(normWireNullable(normWireNumber)), brace1Height: normWireOptional(normWireNullable(normWireNumber)), brace1Thickness: normWireOptional(normWireNullable(normWireNumber)), brace1Angle: normWireOptional(normWireNullable(normWireNumber)), brace1Force: normWireOptional(normWireNullable(normWireNumber)), brace2Width: normWireOptional(normWireNullable(normWireNumber)), brace2Height: normWireOptional(normWireNullable(normWireNumber)), brace2Thickness: normWireOptional(normWireNullable(normWireNumber)), brace2Angle: normWireOptional(normWireNullable(normWireNumber)), brace2Force: normWireOptional(normWireNullable(normWireNumber)), gap: normWireOptional(normWireNullable(normWireNumber)), braceFy: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993JointRowDelta: NormWireReader<En1993JointRowDelta> = normWireObject<En1993JointRowDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993JointRowRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993JointRowInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993JointRowRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993JointRowModification))) });
export const parseEn1993JointRowRemoval: NormWireReader<En1993JointRowRemoval> = normWireObject<En1993JointRowRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993JointRowInsertion: NormWireReader<En1993JointRowInsertion> = normWireObject<En1993JointRowInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseJointRow) });
export const parseEn1993JointRowRelocation: NormWireReader<En1993JointRowRelocation> = normWireObject<En1993JointRowRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993JointRowModification: NormWireReader<En1993JointRowModification> = normWireObject<En1993JointRowModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993JointRowPatch)) });
export const parseEn1993JointRowPatch: NormWireReader<En1993JointRowPatch> = normWireObject<En1993JointRowPatch>({ jointId: normWireOptional(normWireNullable(normWireString)), label: normWireOptional(normWireNullable(normWireString)), rowType: normWireOptional(normWireNullable(parseJointRowType)), lever: normWireOptional(normWireNullable(normWireNumber)), boltToWeb: normWireOptional(normWireNullable(normWireNumber)), edgeDistance: normWireOptional(normWireNullable(normWireNumber)), extensionToFlange: normWireOptional(normWireNullable(normWireNumber)), extensionEdge: normWireOptional(normWireNullable(normWireNumber)), alphaFactor: normWireOptional(normWireNullable(normWireNumber)), pitchToPrevious: normWireOptional(normWireNullable(normWireNumber)), bolts: normWireOptional(normWireNullable(normWireInteger)), boltDiameter: normWireOptional(normWireNullable(normWireNumber)), boltStressArea: normWireOptional(normWireNullable(normWireNumber)), gripLength: normWireOptional(normWireNullable(normWireNumber)), boltClass: normWireOptional(normWireNullable(normWireString)), groupWithPrevious: normWireOptional(normWireNullable(normWireInteger)) });
export const parseEn1993FrameNodeDelta: NormWireReader<En1993FrameNodeDelta> = normWireObject<En1993FrameNodeDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993FrameNodeRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993FrameNodeInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993FrameNodeRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993FrameNodeModification))) });
export const parseEn1993FrameNodeRemoval: NormWireReader<En1993FrameNodeRemoval> = normWireObject<En1993FrameNodeRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FrameNodeInsertion: NormWireReader<En1993FrameNodeInsertion> = normWireObject<En1993FrameNodeInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseFrameNode) });
export const parseEn1993FrameNodeRelocation: NormWireReader<En1993FrameNodeRelocation> = normWireObject<En1993FrameNodeRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FrameNodeModification: NormWireReader<En1993FrameNodeModification> = normWireObject<En1993FrameNodeModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993FrameNodePatch)) });
export const parseEn1993FrameNodePatch: NormWireReader<En1993FrameNodePatch> = normWireObject<En1993FrameNodePatch>({ label: normWireOptional(normWireNullable(normWireString)), x: normWireOptional(normWireNullable(normWireNumber)), z: normWireOptional(normWireNullable(normWireNumber)), support: normWireOptional(normWireNullable(parseFrameNodeSupport)), loadX: normWireOptional(normWireNullable(normWireNumber)), loadZ: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993FrameBarDelta: NormWireReader<En1993FrameBarDelta> = normWireObject<En1993FrameBarDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993FrameBarRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993FrameBarInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993FrameBarRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993FrameBarModification))) });
export const parseEn1993FrameBarRemoval: NormWireReader<En1993FrameBarRemoval> = normWireObject<En1993FrameBarRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FrameBarInsertion: NormWireReader<En1993FrameBarInsertion> = normWireObject<En1993FrameBarInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseFrameBar) });
export const parseEn1993FrameBarRelocation: NormWireReader<En1993FrameBarRelocation> = normWireObject<En1993FrameBarRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FrameBarModification: NormWireReader<En1993FrameBarModification> = normWireObject<En1993FrameBarModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993FrameBarPatch)) });
export const parseEn1993FrameBarPatch: NormWireReader<En1993FrameBarPatch> = normWireObject<En1993FrameBarPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), label: normWireOptional(normWireNullable(normWireString)), startNodeId: normWireOptional(normWireNullable(normWireString)), endNodeId: normWireOptional(normWireNullable(normWireString)), releaseStart: normWireOptional(normWireNullable(parseFrameBarRelease)), releaseEnd: normWireOptional(normWireNullable(parseFrameBarRelease)) });
export const parseEn1993PlacementDelta: NormWireReader<En1993PlacementDelta> = normWireObject<En1993PlacementDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993PlacementRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993PlacementInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993PlacementRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993PlacementModification))) });
export const parseEn1993PlacementRemoval: NormWireReader<En1993PlacementRemoval> = normWireObject<En1993PlacementRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993PlacementInsertion: NormWireReader<En1993PlacementInsertion> = normWireObject<En1993PlacementInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parsePlacement) });
export const parseEn1993PlacementRelocation: NormWireReader<En1993PlacementRelocation> = normWireObject<En1993PlacementRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993PlacementModification: NormWireReader<En1993PlacementModification> = normWireObject<En1993PlacementModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993PlacementPatch)) });
export const parseEn1993PlacementPatch: NormWireReader<En1993PlacementPatch> = normWireObject<En1993PlacementPatch>({ x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), z: normWireOptional(normWireNullable(normWireNumber)), yaw: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993JointPlateDelta: NormWireReader<En1993JointPlateDelta> = normWireObject<En1993JointPlateDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993JointPlateRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993JointPlateInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993JointPlateRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993JointPlateModification))) });
export const parseEn1993JointPlateRemoval: NormWireReader<En1993JointPlateRemoval> = normWireObject<En1993JointPlateRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993JointPlateInsertion: NormWireReader<En1993JointPlateInsertion> = normWireObject<En1993JointPlateInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseJointPlate) });
export const parseEn1993JointPlateRelocation: NormWireReader<En1993JointPlateRelocation> = normWireObject<En1993JointPlateRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993JointPlateModification: NormWireReader<En1993JointPlateModification> = normWireObject<En1993JointPlateModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993JointPlatePatch)) });
export const parseEn1993JointPlatePatch: NormWireReader<En1993JointPlatePatch> = normWireObject<En1993JointPlatePatch>({ jointId: normWireOptional(normWireNullable(normWireString)), label: normWireOptional(normWireNullable(normWireString)), role: normWireOptional(normWireNullable(normWireString)), plateHeight: normWireOptional(normWireNullable(normWireNumber)), plateWidth: normWireOptional(normWireNullable(normWireNumber)), plateThickness: normWireOptional(normWireNullable(normWireNumber)), plateCount: normWireOptional(normWireNullable(normWireInteger)), plateFy: normWireOptional(normWireNullable(normWireNumber)), plateFu: normWireOptional(normWireNullable(normWireNumber)), boltClass: normWireOptional(normWireNullable(normWireString)), boltDiameter: normWireOptional(normWireNullable(normWireNumber)), holeDiameter: normWireOptional(normWireNullable(normWireNumber)), boltRows: normWireOptional(normWireNullable(normWireInteger)), boltsPerRow: normWireOptional(normWireNullable(normWireInteger)), pitch: normWireOptional(normWireNullable(normWireNumber)), gauge: normWireOptional(normWireNullable(normWireNumber)), endDistance: normWireOptional(normWireNullable(normWireNumber)), edgeDistance: normWireOptional(normWireNullable(normWireNumber)), shearPlanes: normWireOptional(normWireNullable(normWireInteger)), eccentricity: normWireOptional(normWireNullable(normWireNumber)), weldThroat: normWireOptional(normWireNullable(normWireNumber)), weldFu: normWireOptional(normWireNullable(normWireNumber)), weldBetaW: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993FoundationBlockDelta: NormWireReader<En1993FoundationBlockDelta> = normWireObject<En1993FoundationBlockDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993FoundationBlockRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993FoundationBlockInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993FoundationBlockRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993FoundationBlockModification))) });
export const parseEn1993FoundationBlockRemoval: NormWireReader<En1993FoundationBlockRemoval> = normWireObject<En1993FoundationBlockRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FoundationBlockInsertion: NormWireReader<En1993FoundationBlockInsertion> = normWireObject<En1993FoundationBlockInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseFoundationBlock) });
export const parseEn1993FoundationBlockRelocation: NormWireReader<En1993FoundationBlockRelocation> = normWireObject<En1993FoundationBlockRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FoundationBlockModification: NormWireReader<En1993FoundationBlockModification> = normWireObject<En1993FoundationBlockModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993FoundationBlockPatch)) });
export const parseEn1993FoundationBlockPatch: NormWireReader<En1993FoundationBlockPatch> = normWireObject<En1993FoundationBlockPatch>({ jointId: normWireOptional(normWireNullable(normWireString)), label: normWireOptional(normWireNullable(normWireString)), blockLength: normWireOptional(normWireNullable(normWireNumber)), blockWidth: normWireOptional(normWireNullable(normWireNumber)), blockDepth: normWireOptional(normWireNullable(normWireNumber)), concreteFck: normWireOptional(normWireNullable(normWireNumber)), groutFck: normWireOptional(normWireNullable(normWireNumber)), groutThickness: normWireOptional(normWireNullable(normWireNumber)), anchorCount: normWireOptional(normWireNullable(normWireInteger)), anchorClass: normWireOptional(normWireNullable(normWireString)), anchorDiameter: normWireOptional(normWireNullable(normWireNumber)), anchorStressArea: normWireOptional(normWireNullable(normWireNumber)), anchorEmbedment: normWireOptional(normWireNullable(normWireNumber)), anchorEdgeDistance: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1993FrameActionDelta: NormWireReader<En1993FrameActionDelta> = normWireObject<En1993FrameActionDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993FrameActionRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993FrameActionInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993FrameActionRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993FrameActionModification))) });
export const parseEn1993FrameActionRemoval: NormWireReader<En1993FrameActionRemoval> = normWireObject<En1993FrameActionRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FrameActionInsertion: NormWireReader<En1993FrameActionInsertion> = normWireObject<En1993FrameActionInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseFrameAction) });
export const parseEn1993FrameActionRelocation: NormWireReader<En1993FrameActionRelocation> = normWireObject<En1993FrameActionRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FrameActionModification: NormWireReader<En1993FrameActionModification> = normWireObject<En1993FrameActionModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993FrameActionPatch)) });
export const parseEn1993FrameActionPatch: NormWireReader<En1993FrameActionPatch> = normWireObject<En1993FrameActionPatch>({ loadCaseId: normWireOptional(normWireNullable(normWireString)), targetId: normWireOptional(normWireNullable(normWireString)), kind: normWireOptional(normWireNullable(normWireString)), fx: normWireOptional(normWireNullable(normWireString)), fz: normWireOptional(normWireNullable(normWireString)), my: normWireOptional(normWireNullable(normWireString)), startRatio: normWireOptional(normWireNullable(normWireString)), endRatio: normWireOptional(normWireNullable(normWireString)) });
export const parseEn1993FrameMemberBasisDelta: NormWireReader<En1993FrameMemberBasisDelta> = normWireObject<En1993FrameMemberBasisDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993FrameMemberBasisRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993FrameMemberBasisInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993FrameMemberBasisRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993FrameMemberBasisModification))) });
export const parseEn1993FrameMemberBasisRemoval: NormWireReader<En1993FrameMemberBasisRemoval> = normWireObject<En1993FrameMemberBasisRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FrameMemberBasisInsertion: NormWireReader<En1993FrameMemberBasisInsertion> = normWireObject<En1993FrameMemberBasisInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseFrameMemberBasis) });
export const parseEn1993FrameMemberBasisRelocation: NormWireReader<En1993FrameMemberBasisRelocation> = normWireObject<En1993FrameMemberBasisRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FrameMemberBasisModification: NormWireReader<En1993FrameMemberBasisModification> = normWireObject<En1993FrameMemberBasisModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993FrameMemberBasisPatch)) });
export const parseEn1993FrameMemberBasisPatch: NormWireReader<En1993FrameMemberBasisPatch> = normWireObject<En1993FrameMemberBasisPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), basis: normWireOptional(normWireNullable(normWireString)) });
export const parseEn1993MaterialDuctilityDelta: NormWireReader<En1993MaterialDuctilityDelta> = normWireObject<En1993MaterialDuctilityDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993MaterialDuctilityRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993MaterialDuctilityInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993MaterialDuctilityRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993MaterialDuctilityModification))) });
export const parseEn1993MaterialDuctilityRemoval: NormWireReader<En1993MaterialDuctilityRemoval> = normWireObject<En1993MaterialDuctilityRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993MaterialDuctilityInsertion: NormWireReader<En1993MaterialDuctilityInsertion> = normWireObject<En1993MaterialDuctilityInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseMaterialDuctility) });
export const parseEn1993MaterialDuctilityRelocation: NormWireReader<En1993MaterialDuctilityRelocation> = normWireObject<En1993MaterialDuctilityRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993MaterialDuctilityModification: NormWireReader<En1993MaterialDuctilityModification> = normWireObject<En1993MaterialDuctilityModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993MaterialDuctilityPatch)) });
export const parseEn1993MaterialDuctilityPatch: NormWireReader<En1993MaterialDuctilityPatch> = normWireObject<En1993MaterialDuctilityPatch>({ materialId: normWireOptional(normWireNullable(normWireString)), elongation: normWireOptional(normWireNullable(normWireString)), ultimateStrain: normWireOptional(normWireNullable(normWireString)) });
export const parseEn1993FormingGainDelta: NormWireReader<En1993FormingGainDelta> = normWireObject<En1993FormingGainDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993FormingGainRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993FormingGainInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993FormingGainRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993FormingGainModification))) });
export const parseEn1993FormingGainRemoval: NormWireReader<En1993FormingGainRemoval> = normWireObject<En1993FormingGainRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FormingGainInsertion: NormWireReader<En1993FormingGainInsertion> = normWireObject<En1993FormingGainInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseFormingGain) });
export const parseEn1993FormingGainRelocation: NormWireReader<En1993FormingGainRelocation> = normWireObject<En1993FormingGainRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FormingGainModification: NormWireReader<En1993FormingGainModification> = normWireObject<En1993FormingGainModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993FormingGainPatch)) });
export const parseEn1993FormingGainPatch: NormWireReader<En1993FormingGainPatch> = normWireObject<En1993FormingGainPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), forming: normWireOptional(normWireNullable(normWireString)), ultimateStrength: normWireOptional(normWireNullable(normWireString)), grossArea: normWireOptional(normWireNullable(normWireString)), bends: normWireOptional(normWireNullable(normWireString)), usedYield: normWireOptional(normWireNullable(normWireString)) });
export const parseEn1993BucklingLengthDelta: NormWireReader<En1993BucklingLengthDelta> = normWireObject<En1993BucklingLengthDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993BucklingLengthRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993BucklingLengthInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993BucklingLengthRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993BucklingLengthModification))) });
export const parseEn1993BucklingLengthRemoval: NormWireReader<En1993BucklingLengthRemoval> = normWireObject<En1993BucklingLengthRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993BucklingLengthInsertion: NormWireReader<En1993BucklingLengthInsertion> = normWireObject<En1993BucklingLengthInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseBucklingLength) });
export const parseEn1993BucklingLengthRelocation: NormWireReader<En1993BucklingLengthRelocation> = normWireObject<En1993BucklingLengthRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993BucklingLengthModification: NormWireReader<En1993BucklingLengthModification> = normWireObject<En1993BucklingLengthModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993BucklingLengthPatch)) });
export const parseEn1993BucklingLengthPatch: NormWireReader<En1993BucklingLengthPatch> = normWireObject<En1993BucklingLengthPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), plane: normWireOptional(normWireNullable(normWireString)), sway: normWireOptional(normWireNullable(normWireString)), restraintA: normWireOptional(normWireNullable(normWireString)), columnStiffnessA: normWireOptional(normWireNullable(normWireString)), beamStiffnessA: normWireOptional(normWireNullable(normWireString)), restraintB: normWireOptional(normWireNullable(normWireString)), columnStiffnessB: normWireOptional(normWireNullable(normWireString)), beamStiffnessB: normWireOptional(normWireNullable(normWireString)) });
export const parseEn1993FatigueStressDelta: NormWireReader<En1993FatigueStressDelta> = normWireObject<En1993FatigueStressDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993FatigueStressRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993FatigueStressInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993FatigueStressRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993FatigueStressModification))) });
export const parseEn1993FatigueStressRemoval: NormWireReader<En1993FatigueStressRemoval> = normWireObject<En1993FatigueStressRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FatigueStressInsertion: NormWireReader<En1993FatigueStressInsertion> = normWireObject<En1993FatigueStressInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseFatigueStress) });
export const parseEn1993FatigueStressRelocation: NormWireReader<En1993FatigueStressRelocation> = normWireObject<En1993FatigueStressRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FatigueStressModification: NormWireReader<En1993FatigueStressModification> = normWireObject<En1993FatigueStressModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993FatigueStressPatch)) });
export const parseEn1993FatigueStressPatch: NormWireReader<En1993FatigueStressPatch> = normWireObject<En1993FatigueStressPatch>({ detailId: normWireOptional(normWireNullable(normWireString)), maxCaseId: normWireOptional(normWireNullable(normWireString)), minCaseId: normWireOptional(normWireNullable(normWireString)), fibreZ: normWireOptional(normWireNullable(normWireString)), fibreY: normWireOptional(normWireNullable(normWireString)), relief: normWireOptional(normWireNullable(normWireString)), concentration: normWireOptional(normWireNullable(normWireString)), thickness: normWireOptional(normWireNullable(normWireString)), cycles: normWireOptional(normWireNullable(normWireString)) });
export const parseEn1993FastenedJointDelta: NormWireReader<En1993FastenedJointDelta> = normWireObject<En1993FastenedJointDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1993FastenedJointRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1993FastenedJointInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1993FastenedJointRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1993FastenedJointModification))) });
export const parseEn1993FastenedJointRemoval: NormWireReader<En1993FastenedJointRemoval> = normWireObject<En1993FastenedJointRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FastenedJointInsertion: NormWireReader<En1993FastenedJointInsertion> = normWireObject<En1993FastenedJointInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), row: normWireRequired(parseFastenedJoint) });
export const parseEn1993FastenedJointRelocation: NormWireReader<En1993FastenedJointRelocation> = normWireObject<En1993FastenedJointRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseEn1993FastenedJointModification: NormWireReader<En1993FastenedJointModification> = normWireObject<En1993FastenedJointModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1993FastenedJointPatch)) });
export const parseEn1993FastenedJointPatch: NormWireReader<En1993FastenedJointPatch> = normWireObject<En1993FastenedJointPatch>({ memberId: normWireOptional(normWireNullable(normWireString)), kind: normWireOptional(normWireNullable(normWireString)), diameter: normWireOptional(normWireNullable(normWireString)), washerDiameter: normWireOptional(normWireNullable(normWireString)), count: normWireOptional(normWireNullable(normWireString)), perRow: normWireOptional(normWireNullable(normWireString)), headThickness: normWireOptional(normWireNullable(normWireString)), baseThickness: normWireOptional(normWireNullable(normWireString)), headStrength: normWireOptional(normWireNullable(normWireString)), baseStrength: normWireOptional(normWireNullable(normWireString)), endDistance: normWireOptional(normWireNullable(normWireString)), edgeDistance: normWireOptional(normWireNullable(normWireString)), pitch: normWireOptional(normWireNullable(normWireString)), gauge: normWireOptional(normWireNullable(normWireString)), sheetWidth: normWireOptional(normWireNullable(normWireString)), shearForce: normWireOptional(normWireNullable(normWireString)), tensionForce: normWireOptional(normWireNullable(normWireString)), shearResistance: normWireOptional(normWireNullable(normWireString)) });
