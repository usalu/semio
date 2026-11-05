/** 📸️ `En1993Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface En1993Snapshot {
  /** @state artifact */
  annex: "En" | "De";
  /** @state artifact */
  materials: SteelMaterial[];
  /** @state artifact */
  sections: SteelSection[];
  /** @state artifact */
  members: SteelMember[];
  /** @state artifact */
  loadCases: LoadCase[];
  /** @state artifact */
  memberActions: MemberAction[];
  /** @state artifact */
  joints: SteelJoint[];
  /** @state artifact */
  fatigueDetails: FatigueDetail[];
  /** @state artifact */
  fireExposures: FireExposure[];
  /** @state artifact */
  coldFormedMembers: ColdFormedMember[];
  /** @state artifact */
  platedPanels: PlatedPanel[];
  /** @state artifact */
  siloShells: SiloShell[];
  /** @state artifact */
  tensionComponents: TensionComponent[];
  /** @state artifact */
  bridgeFatigue: BridgeFatigue[];
  /** @state artifact */
  towerLegs: TowerLeg[];
  /** @state artifact */
  piles: SteelPile[];
  /** @state artifact */
  craneRunways: CraneRunway[];
  /** @state artifact */
  storeys: Storey[];
  /** @state artifact */
  restraints: Restraint[];
  /** @state artifact */
  builtUpMembers: BuiltUpMember[];
  /** @state artifact */
  weldGroups: WeldGroup[];
  /** @state artifact */
  parametricSections: ParametricSection[];
  /** @state artifact */
  hollowJoints: HollowJoint[];
  /** @state artifact */
  jointRows: JointRow[];
  /** @state artifact */
  frameNodes: FrameNode[];
  /** @state artifact */
  frameBars: FrameBar[];
  /** @state artifact */
  placements: Placement[];
  /** @state artifact */
  jointPlates: JointPlate[];
  /** @state artifact */
  foundationBlocks: FoundationBlock[];
  /** @state artifact */
  reliabilityClass: "RC1" | "RC2" | "RC3";
  /** @state artifact */
  frameActions: FrameAction[];
  /** @state artifact */
  frameMemberBases: FrameMemberBasis[];
  /** @state artifact */
  fatigueStresses: FatigueStress[];
  /** @state artifact */
  bucklingLengths: BucklingLength[];
  /** @state artifact */
  formingGains: FormingGain[];
  /** @state artifact */
  materialDuctility: MaterialDuctility[];
  /** @state artifact */
  fastenedJoints: FastenedJoint[];
}

export type MaterialKind = "Carbon" | "Stainless";

export interface SteelMaterial {
  id: string;
  grade: string;
  fy: number;
  fu: number;
  eModulus: number;
  gModulus: number;
  subgrade: string;
  kind: MaterialKind;
  rambergOsgoodN: number;
}

export type SectionKind = "RolledI" | "WeldedI" | "Hss" | "Chs" | "Angle";

export interface SteelSection {
  id: string;
  designation: string;
  kind: SectionKind;
  h: number;
  b: number;
  tw: number;
  tf: number;
  r: number;
  area: number;
  shearAreaY: number;
  shearAreaZ: number;
  iy: number;
  iz: number;
  it: number;
  iw: number;
  wElY: number;
  wElZ: number;
  wPlY: number;
  wPlZ: number;
  areaNet: number;
  fabrication: HollowFabrication;
}

export type WarpingSupport = "Fork" | "Fixed" | "FixedFork" | "Cantilever";

export type SteelMemberType = "Beam" | "Column" | "BeamColumn" | "Brace" | "Tie";

export type LoadApplication = "ShearCenter" | "TopFlange" | "BottomFlange";

export type MomentDiagram = "Linear" | "Uniform" | "Parabolic";

export type AnalysisKind = "Elastic" | "Plastic";

export type StainlessForming = "Unset" | "ColdFormedOpen" | "WeldedOpen" | "Hollow";

export type TorsionLoad = "Point" | "Udl";

export interface SteelMember {
  id: string;
  label: string;
  memberType: SteelMemberType;
  sectionId: string;
  materialId: string;
  length: number;
  bucklingLengthY: number;
  bucklingLengthZ: number;
  ltbLength: number;
  ltbRestraintSpacing: number;
  loadApplication: LoadApplication;
  endMomentRatioPsi: number;
  momentDiagram: MomentDiagram;
  deflectionLimitRatio: number;
  analysis: AnalysisKind;
  stainlessForming: StainlessForming;
  designTemperature: number;
  toughnessStressRatio: number;
  radiationShift: number;
  stressShift: number;
  marginShift: number;
  strainRateShift: number;
  coldFormingRadius: number;
  webMaterialId: string;
  torsionLoad: TorsionLoad;
  torquePosition: number;
  warpingSupport: WarpingSupport;
}

export type ActionKind = "Permanent" | "Imposed" | "Wind" | "Snow" | "Accidental";

export interface LoadCase {
  id: string;
  name: string;
  kind: ActionKind;
  category: string;
}

export interface DesignAction {
  n: number;
  vy: number;
  vz: number;
  my: number;
  mz: number;
  t: number;
}

export interface MemberAction {
  id: string;
  memberId: string;
  loadCaseId: string;
  action: DesignAction;
}

export type StiffnessClass = "Unset" | "Rigid" | "SemiRigid" | "Pinned";

export type JointKind = "Bolted" | "Welded" | "EndPlate" | "BasePlate";

export type JointCategory = "A" | "B" | "C";

export interface SteelJoint {
  id: string;
  kind: JointKind;
  memberId: string;
  boltClass: string;
  boltDiameter: number;
  boltRows: number;
  boltsPerRow: number;
  pitch: number;
  gauge: number;
  endDistance: number;
  edgeDistance: number;
  shearPlanes: number;
  plateThickness: number;
  plateFu: number;
  weldThroat: number;
  weldLength: number;
  weldFu: number;
  weldGrade: string;
  actions: JointForceAction[];
  category: JointCategory;
  frictionMu: number;
  preloadForce: number;
  slipFactorKs: number;
  frictionSurfaces: number;
  plateFy: number;
  plateWidth: number;
  boltToWeb: number;
  extensionToFlange: number;
  extensionEdge: number;
  alphaFactor: number;
  boltStressArea: number;
  boltGripLength: number;
  columnFlangeThickness: number;
  columnWebThickness: number;
  columnWebDepth: number;
  columnWebShearArea: number;
  columnRootRadius: number;
  columnFy: number;
  columnWebStress: number;
  beamFlangeThickness: number;
  compressionLever: number;
  momentDemand: number;
  beamMomentResistance: number;
  columnMomentResistance: number;
  beamInertia: number;
  beamSpan: number;
  bracedFrame: number;
  stiffnessRatio: number;
  transformationParameter: number;
  assumedClass: StiffnessClass;
  basePlateLength: number;
  concreteFcd: number;
  concreteModulus: number;
  concreteAreaA0: number;
  concreteAreaA1: number;
  jointCoefficient: number;
  anchorArea: number;
  anchorLength: number;
  baseNormalForce: number;
  baseMoment: number;
  baseLeverLeft: number;
  baseLeverRight: number;
  beamWebThickness: number;
  beamWebFy: number;
  weldForceLongitudinal: number;
  weldForceTransverse: number;
}

export type FatigueMethod = "DamageTolerant" | "SafeLife";

export interface FatigueDetail {
  id: string;
  memberId: string;
  category: number;
  method: FatigueMethod;
  spectrum: FatigueBand[];
  stressHistory: string;
  consequence: FatigueConsequence;
}

export type FireRating = "R30" | "R60" | "R90" | "R120";

export interface FireExposure {
  id: string;
  memberId: string;
  rating: FireRating;
  protectionThickness: number;
  sectionFactor: number;
  mu0: number;
  designTemperature: number;
  protectionConductivity: number;
  protectionDensity: number;
  protectionSpecificHeat: number;
}

export interface ColdFormedMember {
  id: string;
  bBar: number;
  thickness: number;
  kSigma: number;
  psi: number;
  fy: number;
  grossResistance: number;
  actions: ForceAction[];
  flangeWidth: number;
  lipLength: number;
  webHeight: number;
  webSlope: number;
  oppositeFlangeWidth: number;
  springFactor: number;
  cornerRadius: number;
  flangeStress: number;
  shearForce: number;
  supportReaction: number;
  supportLength: number;
  cripplingCategory: number;
  stiffenedSupport: number;
  bendingMoment: number;
  momentResistance: number;
  sheetingStiffness: number;
  requiredRestraintStiffness: number;
  purlinSpan: number;
  purlinLoad: number;
  purlinLateralRatio: number;
}

export type StiffenerShape = "None" | "Flat" | "Tee" | "Angle";

export type PatchLoadType = "Unset" | "Through" | "OneFlangeNearEnd" | "EndFlange";

export interface PlatedPanel {
  id: string;
  a: number;
  b: number;
  thickness: number;
  fy: number;
  kSigma: number;
  actions: ForceAction[];
  stiffenerInertia: number;
  stiffenerArea: number;
  stiffenerEccentricity: number;
  stiffenerAlpha: number;
  compressionStress: number;
  halfWidth: number;
  equivalentSpan: number;
  hogging: number;
  patchType: PatchLoadType;
  patchForce: number;
  patchLength: number;
  patchOffset: number;
  flangeThickness: number;
  flangeWidth: number;
  flangeFy: number;
  longitudinalOffset: number;
  longitudinalInertia: number;
  patchMoment: number;
  patchMomentResistance: number;
  bendingMoment: number;
  plasticMoment: number;
  flangeMoment: number;
  shearForce: number;
  transverseStiffenerInertia: number;
  flangeInducedK: number;
  flangeArea: number;
  stressX: number;
  stressZ: number;
  shearStress: number;
  alphaCr: number;
  corrugationFlat: number;
  corrugationProjection: number;
  corrugationDepth: number;
  webHeight: number;
  stiffenerSpacing: number;
  stiffenerShape: StiffenerShape;
  stiffenerDepth: number;
  stiffenerThickness: number;
  stiffenerFlangeWidth: number;
  stiffenerFlangeThickness: number;
  lateralPressure: number;
  lateralPressureCoefficient: number;
  steelGrade: string;
  steelSubgrade: string;
  designTemperature: number;
}

export type FabricationToleranceClass = "Unset" | "A" | "B" | "C";

export interface SiloShell {
  id: string;
  thickness: number;
  radius: number;
  depth: number;
  k: number;
  gamma: number;
  fy: number;
  length: number;
  fabricationClass: FabricationToleranceClass;
  meridionalStress: number;
  hoopStress: number;
  shellShearStress: number;
  boundaryFactorAxial: number;
  boundaryFactorHoop: number;
  boundaryFactorShear: number;
  wallFriction: number;
  planArea: number;
  planPerimeter: number;
  liquidDensity: number;
  liquidHeight: number;
  courseElevation: number;
  internalPressure: number;
  externalPressure: number;
  temperatureChange: number;
  ringSpacing: number;
  ringArea: number;
  ringInertia: number;
  ringOffset: number;
}

export interface TensionComponent {
  id: string;
  fUk: number;
  fK: number;
  actions: ForceAction[];
  partialFactor: number;
  cableLength: number;
  cableWeight: number;
  cableHorizontalForce: number;
  cableModulus: number;
  cableStress: number;
  cableUnitWeight: number;
  threadFactor: number;
  rodStressArea: number;
  rodFu: number;
  pinDiameter: number;
  pinPlateThickness: number;
  pinGap: number;
  pinOuterThickness: number;
  pinInnerThickness: number;
  pinFup: number;
  pinFyp: number;
  pinPlateFy: number;
}

export interface BridgeFatigue {
  id: string;
  memberId: string;
  lambda: number;
  phi2: number;
  deltaSigmaP: number;
  category: number;
  method: FatigueMethod;
  heavyVehicleWeight: number;
  referenceWeight: number;
  observedTraffic: number;
  designLife: number;
  secondLaneTraffic: number;
  secondLaneWeight: number;
  lambda1: number;
  lambdaMax: number;
  detailId: string;
  plateThickness: number;
  deckThickness: number;
  troughSpacing: number;
  troughWidth: number;
  troughThickness: number;
  crossbeamSpacing: number;
  surfacingThickness: number;
  wheelLoad: number;
  wheelPatch: number;
  consequence: FatigueConsequence;
}

export type LatticeKind = "Unset" | "Leg" | "Diagonal" | "Redundant";

export interface TowerLeg {
  id: string;
  memberId: string;
  forceCoefficient: number;
  dynamicFactor: number;
  actions: ForceAction[];
  latticeKind: LatticeKind;
  slenderness: number;
  diameter: number;
  height: number;
  bendingStiffness: number;
  massPerLength: number;
  strouhal: number;
  logDecrement: number;
  airDensity: number;
  meanWindSpeed: number;
  crossingRate: number;
  designLife: number;
  kFactor: number;
  kwFactor: number;
  lateralForceCoefficient: number;
  permittedAmplitudeRatio: number;
  ringMode: number;
  shellThickness: number;
  guyArea: number;
  guyAngle: number;
  guyLength: number;
}

export type SheetPileEnvironment = "Unset" | "UndisturbedSoil" | "PollutedSoil" | "CompactedFill" | "NonCompactedFill";

export interface SteelPile {
  id: string;
  sectionId: string;
  materialId: string;
  drivingStress: number;
  embeddedLength: number;
  shaftPerimeter: number;
  actions: ForceAction[];
  environment: SheetPileEnvironment;
  designLife: number;
  wallThickness: number;
  interlockBendingFactor: number;
  interlockStiffnessFactor: number;
  sectionModulus: number;
  shearArea: number;
  designMoment: number;
  designShear: number;
  interlockShear: number;
}

export interface CraneRunway {
  id: string;
  memberId: string;
  wheelContactLength: number;
  dispersion: number;
  webThickness: number;
  fy: number;
  phi: number;
  actions: ForceAction[];
  maxWheelLoad: number;
  loadFactor: number;
  railInertia: number;
  railFixed: number;
  flangeEffectiveWidth: number;
  flangeThickness: number;
  wheelEccentricity: number;
  torsionConstant: number;
  longitudinalStress: number;
  railRadius: number;
  railContactWidth: number;
  craneNu: number;
  craneKq: number;
  damageExponent: number;
  stressRange: number;
  detailCategoryStress: number;
  fatiguePhi: number;
}

export interface Storey {
  id: string;
  memberId: string;
  label: string;
  height: number;
  verticalLoad: number;
  horizontalLoad: number;
  drift: number;
  columns: number;
  plasticAnalysis: number;
  buildingHeight: number;
  bracingLoad: number;
  bracingLength: number;
  bracingDeflection: number;
}

export type RestraintSupport = "Fork" | "Fixed";

export type RestraintDistribution = "Uniform" | "Udl" | "Linear";

export interface Restraint {
  id: string;
  memberId: string;
  label: string;
  startSupport: RestraintSupport;
  endSupport: RestraintSupport;
  intermediatePosition: number;
  torsionalSpring: number;
  lateralSpringPosition: number;
  lateralSpringStiffness: number;
  loadHeight: number;
  loadDistribution: RestraintDistribution;
  designMoment: number;
  designAxial: number;
  generalMethod: number;
  hingeToRestraint: number;
}

export type ChordShape = "Unset" | "Channel" | "I" | "Rhs" | "Chs";

export type BuiltUpKind = "Laced" | "Battened" | "Closely";

export interface BuiltUpMember {
  id: string;
  memberId: string;
  label: string;
  kind: BuiltUpKind;
  chordArea: number;
  chordInertia: number;
  chordDistance: number;
  panelSpacing: number;
  planes: number;
  battenInertia: number;
  diagonalArea: number;
  diagonalLength: number;
  designAxial: number;
  designMoment: number;
  backToBack: number;
  minRadius: number;
  chainSpacing: number;
  chordShape: ChordShape;
  chordDepth: number;
  chordWidth: number;
  chordFlangeThickness: number;
  chordWebThickness: number;
}

export type WeldPattern = "Line" | "TwoLines" | "Box";

export interface WeldGroup {
  id: string;
  memberId: string;
  label: string;
  pattern: WeldPattern;
  throat: number;
  width: number;
  depth: number;
  shearForce: number;
  eccentricity: number;
  steelFu: number;
  betaW: number;
}

export type ParametricShape = "WeldedI" | "Rhs" | "Chs" | "Angle";

export interface ParametricSection {
  id: string;
  sectionId: string;
  label: string;
  shape: ParametricShape;
  height: number;
  width: number;
  webThickness: number;
  flangeThickness: number;
  tolerance: number;
}

export interface HollowJoint {
  id: string;
  memberId: string;
  label: string;
  chordWidth: number;
  chordHeight: number;
  chordThickness: number;
  chordFy: number;
  chordStress: number;
  brace1Width: number;
  brace1Height: number;
  brace1Thickness: number;
  brace1Angle: number;
  brace1Force: number;
  brace2Width: number;
  brace2Height: number;
  brace2Thickness: number;
  brace2Angle: number;
  brace2Force: number;
  gap: number;
  braceFy: number;
}

export type JointRowType = "Extended" | "FirstBelow" | "Inner";

export interface JointRow {
  id: string;
  jointId: string;
  label: string;
  rowType: JointRowType;
  lever: number;
  boltToWeb: number;
  edgeDistance: number;
  extensionToFlange: number;
  extensionEdge: number;
  alphaFactor: number;
  pitchToPrevious: number;
  bolts: number;
  boltDiameter: number;
  boltStressArea: number;
  gripLength: number;
  boltClass: string;
  groupWithPrevious: number;
}

export type FrameNodeSupport = "Free" | "Pinned" | "Fixed" | "Roller";

export interface FrameNode {
  id: string;
  label: string;
  x: number;
  z: number;
  support: FrameNodeSupport;
  loadX: number;
  loadZ: number;
}

export type FrameBarRelease = "Rigid" | "Pinned" | "SemiRigid";

export interface FrameBar {
  id: string;
  memberId: string;
  label: string;
  startNodeId: string;
  endNodeId: string;
  releaseStart: FrameBarRelease;
  releaseEnd: FrameBarRelease;
}

export interface Placement {
  id: string;
  x: number;
  y: number;
  z: number;
  yaw: number;
}

export interface JointPlate {
  id: string;
  jointId: string;
  label: string;
  role: string;
  plateHeight: number;
  plateWidth: number;
  plateThickness: number;
  plateCount: number;
  plateFy: number;
  plateFu: number;
  boltClass: string;
  boltDiameter: number;
  holeDiameter: number;
  boltRows: number;
  boltsPerRow: number;
  pitch: number;
  gauge: number;
  endDistance: number;
  edgeDistance: number;
  shearPlanes: number;
  eccentricity: number;
  weldThroat: number;
  weldFu: number;
  weldBetaW: number;
  groupMethod: BoltGroupMethod;
}

export interface FoundationBlock {
  id: string;
  jointId: string;
  label: string;
  blockLength: number;
  blockWidth: number;
  blockDepth: number;
  concreteFck: number;
  groutFck: number;
  groutThickness: number;
  anchorCount: number;
  anchorClass: string;
  anchorDiameter: number;
  anchorStressArea: number;
  anchorEmbedment: number;
  anchorEdgeDistance: number;
}

export interface ForceAction {
  id: string;
  loadCaseId: string;
  force: number;
}

export interface JointForceAction {
  id: string;
  loadCaseId: string;
  shear: number;
  tension: number;
}

export interface FatigueBand {
  id: string;
  deltaSigma: number;
  cycles: number;
}

export type En1993AnnexChoice = "En" | "De";

export type FrameActionKind = "NodalForce" | "NodalMoment" | "BarUniform";

export interface FrameAction {
  id: string;
  loadCaseId: string;
  targetId: string;
  kind: FrameActionKind;
  fx: number;
  fz: number;
  my: number;
  startRatio: number;
  endRatio: number;
}

export type MemberForceBasis = "FrameDerived" | "Explicit";

export interface FrameMemberBasis {
  id: string;
  memberId: string;
  basis: MemberForceBasis;
}

export type HollowFabrication = "Unset" | "HotFinished" | "ColdFormed";

export type BoltGroupMethod = "Elastic" | "InstantaneousCentre";

export type StressRelief = "AsWelded" | "StressRelieved" | "NonWelded";

export interface FatigueStress {
  id: string;
  detailId: string;
  maxCaseId: string;
  minCaseId: string;
  fibreZ: number;
  fibreY: number;
  relief: StressRelief;
  concentration: number;
  thickness: number;
  cycles: number;
}

export type BucklingPlane = "MajorAxis" | "MinorAxis";

export type FrameSway = "NonSway" | "Sway";

export type NodeARestraint = "Framed" | "Fixed" | "Pinned";

export type NodeBRestraint = "Framed" | "Fixed" | "Pinned";

export interface BucklingLength {
  id: string;
  memberId: string;
  plane: BucklingPlane;
  sway: FrameSway;
  restraintA: NodeARestraint;
  columnStiffnessA: number;
  beamStiffnessA: number;
  restraintB: NodeBRestraint;
  columnStiffnessB: number;
  beamStiffnessB: number;
}

export type FormingProcess = "RollForming" | "OtherForming";

export interface FormingGain {
  id: string;
  memberId: string;
  forming: FormingProcess;
  ultimateStrength: number;
  grossArea: number;
  bends: number;
  usedYield: number;
}

export interface MaterialDuctility {
  id: string;
  materialId: string;
  elongation: number;
  ultimateStrain: number;
}

export type FastenerKind = "SelfTappingScrew" | "BlindRivet";

export interface FastenedJoint {
  id: string;
  memberId: string;
  kind: FastenerKind;
  diameter: number;
  washerDiameter: number;
  count: number;
  perRow: number;
  headThickness: number;
  baseThickness: number;
  headStrength: number;
  baseStrength: number;
  endDistance: number;
  edgeDistance: number;
  pitch: number;
  gauge: number;
  sheetWidth: number;
  shearForce: number;
  tensionForce: number;
  shearResistance: number;
}

export type FatigueConsequence = "Low" | "High";

export const parseEn1993Snapshot: NormWireReader<En1993Snapshot> = normWireObject<En1993Snapshot>({ annex: normWireRequired(normWireLiteral("En", "De")), materials: normWireRequired(normWireArray(normWireRef(() => parseSteelMaterial))), sections: normWireRequired(normWireArray(normWireRef(() => parseSteelSection))), members: normWireRequired(normWireArray(normWireRef(() => parseSteelMember))), loadCases: normWireRequired(normWireArray(normWireRef(() => parseLoadCase))), memberActions: normWireRequired(normWireArray(normWireRef(() => parseMemberAction))), joints: normWireRequired(normWireArray(normWireRef(() => parseSteelJoint))), fatigueDetails: normWireRequired(normWireArray(normWireRef(() => parseFatigueDetail))), fireExposures: normWireRequired(normWireArray(normWireRef(() => parseFireExposure))), coldFormedMembers: normWireRequired(normWireArray(normWireRef(() => parseColdFormedMember))), platedPanels: normWireRequired(normWireArray(normWireRef(() => parsePlatedPanel))), siloShells: normWireRequired(normWireArray(normWireRef(() => parseSiloShell))), tensionComponents: normWireRequired(normWireArray(normWireRef(() => parseTensionComponent))), bridgeFatigue: normWireRequired(normWireArray(normWireRef(() => parseBridgeFatigue))), towerLegs: normWireRequired(normWireArray(normWireRef(() => parseTowerLeg))), piles: normWireRequired(normWireArray(normWireRef(() => parseSteelPile))), craneRunways: normWireRequired(normWireArray(normWireRef(() => parseCraneRunway))), storeys: normWireRequired(normWireArray(normWireRef(() => parseStorey))), restraints: normWireRequired(normWireArray(normWireRef(() => parseRestraint))), builtUpMembers: normWireRequired(normWireArray(normWireRef(() => parseBuiltUpMember))), weldGroups: normWireRequired(normWireArray(normWireRef(() => parseWeldGroup))), parametricSections: normWireRequired(normWireArray(normWireRef(() => parseParametricSection))), hollowJoints: normWireRequired(normWireArray(normWireRef(() => parseHollowJoint))), jointRows: normWireRequired(normWireArray(normWireRef(() => parseJointRow))), frameNodes: normWireRequired(normWireArray(normWireRef(() => parseFrameNode))), frameBars: normWireRequired(normWireArray(normWireRef(() => parseFrameBar))), placements: normWireRequired(normWireArray(normWireRef(() => parsePlacement))), jointPlates: normWireRequired(normWireArray(normWireRef(() => parseJointPlate))), foundationBlocks: normWireRequired(normWireArray(normWireRef(() => parseFoundationBlock))), reliabilityClass: normWireRequired(normWireLiteral("RC1", "RC2", "RC3")), frameActions: normWireRequired(normWireArray(normWireRef(() => parseFrameAction))), frameMemberBases: normWireRequired(normWireArray(normWireRef(() => parseFrameMemberBasis))), fatigueStresses: normWireRequired(normWireArray(normWireRef(() => parseFatigueStress))), bucklingLengths: normWireRequired(normWireArray(normWireRef(() => parseBucklingLength))), formingGains: normWireRequired(normWireArray(normWireRef(() => parseFormingGain))), materialDuctility: normWireRequired(normWireArray(normWireRef(() => parseMaterialDuctility))), fastenedJoints: normWireRequired(normWireArray(normWireRef(() => parseFastenedJoint))) });
export const parseMaterialKind: NormWireReader<MaterialKind> = normWireLiteral("Carbon", "Stainless");
export const parseSteelMaterial: NormWireReader<SteelMaterial> = normWireObject<SteelMaterial>({ id: normWireRequired(normWireString), grade: normWireRequired(normWireString), fy: normWireRequired(normWireNumber), fu: normWireRequired(normWireNumber), eModulus: normWireRequired(normWireNumber), gModulus: normWireRequired(normWireNumber), subgrade: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseMaterialKind)), rambergOsgoodN: normWireRequired(normWireNumber) });
export const parseSectionKind: NormWireReader<SectionKind> = normWireLiteral("RolledI", "WeldedI", "Hss", "Chs", "Angle");
export const parseSteelSection: NormWireReader<SteelSection> = normWireObject<SteelSection>({ id: normWireRequired(normWireString), designation: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseSectionKind)), h: normWireRequired(normWireNumber), b: normWireRequired(normWireNumber), tw: normWireRequired(normWireNumber), tf: normWireRequired(normWireNumber), r: normWireRequired(normWireNumber), area: normWireRequired(normWireNumber), shearAreaY: normWireRequired(normWireNumber), shearAreaZ: normWireRequired(normWireNumber), iy: normWireRequired(normWireNumber), iz: normWireRequired(normWireNumber), it: normWireRequired(normWireNumber), iw: normWireRequired(normWireNumber), wElY: normWireRequired(normWireNumber), wElZ: normWireRequired(normWireNumber), wPlY: normWireRequired(normWireNumber), wPlZ: normWireRequired(normWireNumber), areaNet: normWireRequired(normWireNumber), fabrication: normWireRequired(normWireRef(() => parseHollowFabrication)) });
export const parseWarpingSupport: NormWireReader<WarpingSupport> = normWireLiteral("Fork", "Fixed", "FixedFork", "Cantilever");
export const parseSteelMemberType: NormWireReader<SteelMemberType> = normWireLiteral("Beam", "Column", "BeamColumn", "Brace", "Tie");
export const parseLoadApplication: NormWireReader<LoadApplication> = normWireLiteral("ShearCenter", "TopFlange", "BottomFlange");
export const parseMomentDiagram: NormWireReader<MomentDiagram> = normWireLiteral("Linear", "Uniform", "Parabolic");
export const parseAnalysisKind: NormWireReader<AnalysisKind> = normWireLiteral("Elastic", "Plastic");
export const parseStainlessForming: NormWireReader<StainlessForming> = normWireLiteral("Unset", "ColdFormedOpen", "WeldedOpen", "Hollow");
export const parseTorsionLoad: NormWireReader<TorsionLoad> = normWireLiteral("Point", "Udl");
export const parseSteelMember: NormWireReader<SteelMember> = normWireObject<SteelMember>({ id: normWireRequired(normWireString), label: normWireRequired(normWireString), memberType: normWireRequired(normWireRef(() => parseSteelMemberType)), sectionId: normWireRequired(normWireString), materialId: normWireRequired(normWireString), length: normWireRequired(normWireNumber), bucklingLengthY: normWireRequired(normWireNumber), bucklingLengthZ: normWireRequired(normWireNumber), ltbLength: normWireRequired(normWireNumber), ltbRestraintSpacing: normWireRequired(normWireNumber), loadApplication: normWireRequired(normWireRef(() => parseLoadApplication)), endMomentRatioPsi: normWireRequired(normWireNumber), momentDiagram: normWireRequired(normWireRef(() => parseMomentDiagram)), deflectionLimitRatio: normWireRequired(normWireNumber), analysis: normWireRequired(normWireRef(() => parseAnalysisKind)), stainlessForming: normWireRequired(normWireRef(() => parseStainlessForming)), designTemperature: normWireRequired(normWireNumber), toughnessStressRatio: normWireRequired(normWireNumber), radiationShift: normWireRequired(normWireNumber), stressShift: normWireRequired(normWireNumber), marginShift: normWireRequired(normWireNumber), strainRateShift: normWireRequired(normWireNumber), coldFormingRadius: normWireRequired(normWireNumber), webMaterialId: normWireRequired(normWireString), torsionLoad: normWireRequired(normWireRef(() => parseTorsionLoad)), torquePosition: normWireRequired(normWireNumber), warpingSupport: normWireRequired(normWireRef(() => parseWarpingSupport)) });
export const parseActionKind: NormWireReader<ActionKind> = normWireLiteral("Permanent", "Imposed", "Wind", "Snow", "Accidental");
export const parseLoadCase: NormWireReader<LoadCase> = normWireObject<LoadCase>({ id: normWireRequired(normWireString), name: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseActionKind)), category: normWireRequired(normWireString) });
export const parseDesignAction: NormWireReader<DesignAction> = normWireObject<DesignAction>({ n: normWireRequired(normWireNumber), vy: normWireRequired(normWireNumber), vz: normWireRequired(normWireNumber), my: normWireRequired(normWireNumber), mz: normWireRequired(normWireNumber), t: normWireRequired(normWireNumber) });
export const parseMemberAction: NormWireReader<MemberAction> = normWireObject<MemberAction>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), loadCaseId: normWireRequired(normWireString), action: normWireRequired(normWireRef(() => parseDesignAction)) });
export const parseStiffnessClass: NormWireReader<StiffnessClass> = normWireLiteral("Unset", "Rigid", "SemiRigid", "Pinned");
export const parseJointKind: NormWireReader<JointKind> = normWireLiteral("Bolted", "Welded", "EndPlate", "BasePlate");
export const parseJointCategory: NormWireReader<JointCategory> = normWireLiteral("A", "B", "C");
export const parseSteelJoint: NormWireReader<SteelJoint> = normWireObject<SteelJoint>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseJointKind)), memberId: normWireRequired(normWireString), boltClass: normWireRequired(normWireString), boltDiameter: normWireRequired(normWireNumber), boltRows: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), boltsPerRow: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), pitch: normWireRequired(normWireNumber), gauge: normWireRequired(normWireNumber), endDistance: normWireRequired(normWireNumber), edgeDistance: normWireRequired(normWireNumber), shearPlanes: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), plateThickness: normWireRequired(normWireNumber), plateFu: normWireRequired(normWireNumber), weldThroat: normWireRequired(normWireNumber), weldLength: normWireRequired(normWireNumber), weldFu: normWireRequired(normWireNumber), weldGrade: normWireRequired(normWireString), actions: normWireRequired(normWireArray(normWireRef(() => parseJointForceAction))), category: normWireRequired(normWireRef(() => parseJointCategory)), frictionMu: normWireRequired(normWireNumber), preloadForce: normWireRequired(normWireNumber), slipFactorKs: normWireRequired(normWireNumber), frictionSurfaces: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), plateFy: normWireRequired(normWireNumber), plateWidth: normWireRequired(normWireNumber), boltToWeb: normWireRequired(normWireNumber), extensionToFlange: normWireRequired(normWireNumber), extensionEdge: normWireRequired(normWireNumber), alphaFactor: normWireRequired(normWireNumber), boltStressArea: normWireRequired(normWireNumber), boltGripLength: normWireRequired(normWireNumber), columnFlangeThickness: normWireRequired(normWireNumber), columnWebThickness: normWireRequired(normWireNumber), columnWebDepth: normWireRequired(normWireNumber), columnWebShearArea: normWireRequired(normWireNumber), columnRootRadius: normWireRequired(normWireNumber), columnFy: normWireRequired(normWireNumber), columnWebStress: normWireRequired(normWireNumber), beamFlangeThickness: normWireRequired(normWireNumber), compressionLever: normWireRequired(normWireNumber), momentDemand: normWireRequired(normWireNumber), beamMomentResistance: normWireRequired(normWireNumber), columnMomentResistance: normWireRequired(normWireNumber), beamInertia: normWireRequired(normWireNumber), beamSpan: normWireRequired(normWireNumber), bracedFrame: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), stiffnessRatio: normWireRequired(normWireNumber), transformationParameter: normWireRequired(normWireNumber), assumedClass: normWireRequired(normWireRef(() => parseStiffnessClass)), basePlateLength: normWireRequired(normWireNumber), concreteFcd: normWireRequired(normWireNumber), concreteModulus: normWireRequired(normWireNumber), concreteAreaA0: normWireRequired(normWireNumber), concreteAreaA1: normWireRequired(normWireNumber), jointCoefficient: normWireRequired(normWireNumber), anchorArea: normWireRequired(normWireNumber), anchorLength: normWireRequired(normWireNumber), baseNormalForce: normWireRequired(normWireNumber), baseMoment: normWireRequired(normWireNumber), baseLeverLeft: normWireRequired(normWireNumber), baseLeverRight: normWireRequired(normWireNumber), beamWebThickness: normWireRequired(normWireNumber), beamWebFy: normWireRequired(normWireNumber), weldForceLongitudinal: normWireRequired(normWireNumber), weldForceTransverse: normWireRequired(normWireNumber) });
export const parseFatigueMethod: NormWireReader<FatigueMethod> = normWireLiteral("DamageTolerant", "SafeLife");
export const parseFatigueDetail: NormWireReader<FatigueDetail> = normWireObject<FatigueDetail>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), category: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), method: normWireRequired(normWireRef(() => parseFatigueMethod)), spectrum: normWireRequired(normWireArray(normWireRef(() => parseFatigueBand))), stressHistory: normWireRequired(normWireString), consequence: normWireRequired(normWireRef(() => parseFatigueConsequence)) });
export const parseFireRating: NormWireReader<FireRating> = normWireLiteral("R30", "R60", "R90", "R120");
export const parseFireExposure: NormWireReader<FireExposure> = normWireObject<FireExposure>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), rating: normWireRequired(normWireRef(() => parseFireRating)), protectionThickness: normWireRequired(normWireNumber), sectionFactor: normWireRequired(normWireNumber), mu0: normWireRequired(normWireNumber), designTemperature: normWireRequired(normWireNumber), protectionConductivity: normWireRequired(normWireNumber), protectionDensity: normWireRequired(normWireNumber), protectionSpecificHeat: normWireRequired(normWireNumber) });
export const parseColdFormedMember: NormWireReader<ColdFormedMember> = normWireObject<ColdFormedMember>({ id: normWireRequired(normWireString), bBar: normWireRequired(normWireNumber), thickness: normWireRequired(normWireNumber), kSigma: normWireRequired(normWireNumber), psi: normWireRequired(normWireNumber), fy: normWireRequired(normWireNumber), grossResistance: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseForceAction))), flangeWidth: normWireRequired(normWireNumber), lipLength: normWireRequired(normWireNumber), webHeight: normWireRequired(normWireNumber), webSlope: normWireRequired(normWireNumber), oppositeFlangeWidth: normWireRequired(normWireNumber), springFactor: normWireRequired(normWireNumber), cornerRadius: normWireRequired(normWireNumber), flangeStress: normWireRequired(normWireNumber), shearForce: normWireRequired(normWireNumber), supportReaction: normWireRequired(normWireNumber), supportLength: normWireRequired(normWireNumber), cripplingCategory: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), stiffenedSupport: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), bendingMoment: normWireRequired(normWireNumber), momentResistance: normWireRequired(normWireNumber), sheetingStiffness: normWireRequired(normWireNumber), requiredRestraintStiffness: normWireRequired(normWireNumber), purlinSpan: normWireRequired(normWireNumber), purlinLoad: normWireRequired(normWireNumber), purlinLateralRatio: normWireRequired(normWireNumber) });
export const parseStiffenerShape: NormWireReader<StiffenerShape> = normWireLiteral("None", "Flat", "Tee", "Angle");
export const parsePatchLoadType: NormWireReader<PatchLoadType> = normWireLiteral("Unset", "Through", "OneFlangeNearEnd", "EndFlange");
export const parsePlatedPanel: NormWireReader<PlatedPanel> = normWireObject<PlatedPanel>({ id: normWireRequired(normWireString), a: normWireRequired(normWireNumber), b: normWireRequired(normWireNumber), thickness: normWireRequired(normWireNumber), fy: normWireRequired(normWireNumber), kSigma: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseForceAction))), stiffenerInertia: normWireRequired(normWireNumber), stiffenerArea: normWireRequired(normWireNumber), stiffenerEccentricity: normWireRequired(normWireNumber), stiffenerAlpha: normWireRequired(normWireNumber), compressionStress: normWireRequired(normWireNumber), halfWidth: normWireRequired(normWireNumber), equivalentSpan: normWireRequired(normWireNumber), hogging: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), patchType: normWireRequired(normWireRef(() => parsePatchLoadType)), patchForce: normWireRequired(normWireNumber), patchLength: normWireRequired(normWireNumber), patchOffset: normWireRequired(normWireNumber), flangeThickness: normWireRequired(normWireNumber), flangeWidth: normWireRequired(normWireNumber), flangeFy: normWireRequired(normWireNumber), longitudinalOffset: normWireRequired(normWireNumber), longitudinalInertia: normWireRequired(normWireNumber), patchMoment: normWireRequired(normWireNumber), patchMomentResistance: normWireRequired(normWireNumber), bendingMoment: normWireRequired(normWireNumber), plasticMoment: normWireRequired(normWireNumber), flangeMoment: normWireRequired(normWireNumber), shearForce: normWireRequired(normWireNumber), transverseStiffenerInertia: normWireRequired(normWireNumber), flangeInducedK: normWireRequired(normWireNumber), flangeArea: normWireRequired(normWireNumber), stressX: normWireRequired(normWireNumber), stressZ: normWireRequired(normWireNumber), shearStress: normWireRequired(normWireNumber), alphaCr: normWireRequired(normWireNumber), corrugationFlat: normWireRequired(normWireNumber), corrugationProjection: normWireRequired(normWireNumber), corrugationDepth: normWireRequired(normWireNumber), webHeight: normWireRequired(normWireNumber), stiffenerSpacing: normWireRequired(normWireNumber), stiffenerShape: normWireRequired(normWireRef(() => parseStiffenerShape)), stiffenerDepth: normWireRequired(normWireNumber), stiffenerThickness: normWireRequired(normWireNumber), stiffenerFlangeWidth: normWireRequired(normWireNumber), stiffenerFlangeThickness: normWireRequired(normWireNumber), lateralPressure: normWireRequired(normWireNumber), lateralPressureCoefficient: normWireRequired(normWireNumber), steelGrade: normWireRequired(normWireString), steelSubgrade: normWireRequired(normWireString), designTemperature: normWireRequired(normWireNumber) });
export const parseFabricationToleranceClass: NormWireReader<FabricationToleranceClass> = normWireLiteral("Unset", "A", "B", "C");
export const parseSiloShell: NormWireReader<SiloShell> = normWireObject<SiloShell>({ id: normWireRequired(normWireString), thickness: normWireRequired(normWireNumber), radius: normWireRequired(normWireNumber), depth: normWireRequired(normWireNumber), k: normWireRequired(normWireNumber), gamma: normWireRequired(normWireNumber), fy: normWireRequired(normWireNumber), length: normWireRequired(normWireNumber), fabricationClass: normWireRequired(normWireRef(() => parseFabricationToleranceClass)), meridionalStress: normWireRequired(normWireNumber), hoopStress: normWireRequired(normWireNumber), shellShearStress: normWireRequired(normWireNumber), boundaryFactorAxial: normWireRequired(normWireNumber), boundaryFactorHoop: normWireRequired(normWireNumber), boundaryFactorShear: normWireRequired(normWireNumber), wallFriction: normWireRequired(normWireNumber), planArea: normWireRequired(normWireNumber), planPerimeter: normWireRequired(normWireNumber), liquidDensity: normWireRequired(normWireNumber), liquidHeight: normWireRequired(normWireNumber), courseElevation: normWireRequired(normWireNumber), internalPressure: normWireRequired(normWireNumber), externalPressure: normWireRequired(normWireNumber), temperatureChange: normWireRequired(normWireNumber), ringSpacing: normWireRequired(normWireNumber), ringArea: normWireRequired(normWireNumber), ringInertia: normWireRequired(normWireNumber), ringOffset: normWireRequired(normWireNumber) });
export const parseTensionComponent: NormWireReader<TensionComponent> = normWireObject<TensionComponent>({ id: normWireRequired(normWireString), fUk: normWireRequired(normWireNumber), fK: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseForceAction))), partialFactor: normWireRequired(normWireNumber), cableLength: normWireRequired(normWireNumber), cableWeight: normWireRequired(normWireNumber), cableHorizontalForce: normWireRequired(normWireNumber), cableModulus: normWireRequired(normWireNumber), cableStress: normWireRequired(normWireNumber), cableUnitWeight: normWireRequired(normWireNumber), threadFactor: normWireRequired(normWireNumber), rodStressArea: normWireRequired(normWireNumber), rodFu: normWireRequired(normWireNumber), pinDiameter: normWireRequired(normWireNumber), pinPlateThickness: normWireRequired(normWireNumber), pinGap: normWireRequired(normWireNumber), pinOuterThickness: normWireRequired(normWireNumber), pinInnerThickness: normWireRequired(normWireNumber), pinFup: normWireRequired(normWireNumber), pinFyp: normWireRequired(normWireNumber), pinPlateFy: normWireRequired(normWireNumber) });
export const parseBridgeFatigue: NormWireReader<BridgeFatigue> = normWireObject<BridgeFatigue>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), lambda: normWireRequired(normWireNumber), phi2: normWireRequired(normWireNumber), deltaSigmaP: normWireRequired(normWireNumber), category: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), method: normWireRequired(normWireRef(() => parseFatigueMethod)), heavyVehicleWeight: normWireRequired(normWireNumber), referenceWeight: normWireRequired(normWireNumber), observedTraffic: normWireRequired(normWireNumber), designLife: normWireRequired(normWireNumber), secondLaneTraffic: normWireRequired(normWireNumber), secondLaneWeight: normWireRequired(normWireNumber), lambda1: normWireRequired(normWireNumber), lambdaMax: normWireRequired(normWireNumber), detailId: normWireRequired(normWireString), plateThickness: normWireRequired(normWireNumber), deckThickness: normWireRequired(normWireNumber), troughSpacing: normWireRequired(normWireNumber), troughWidth: normWireRequired(normWireNumber), troughThickness: normWireRequired(normWireNumber), crossbeamSpacing: normWireRequired(normWireNumber), surfacingThickness: normWireRequired(normWireNumber), wheelLoad: normWireRequired(normWireNumber), wheelPatch: normWireRequired(normWireNumber), consequence: normWireRequired(normWireRef(() => parseFatigueConsequence)) });
export const parseLatticeKind: NormWireReader<LatticeKind> = normWireLiteral("Unset", "Leg", "Diagonal", "Redundant");
export const parseTowerLeg: NormWireReader<TowerLeg> = normWireObject<TowerLeg>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), forceCoefficient: normWireRequired(normWireNumber), dynamicFactor: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseForceAction))), latticeKind: normWireRequired(normWireRef(() => parseLatticeKind)), slenderness: normWireRequired(normWireNumber), diameter: normWireRequired(normWireNumber), height: normWireRequired(normWireNumber), bendingStiffness: normWireRequired(normWireNumber), massPerLength: normWireRequired(normWireNumber), strouhal: normWireRequired(normWireNumber), logDecrement: normWireRequired(normWireNumber), airDensity: normWireRequired(normWireNumber), meanWindSpeed: normWireRequired(normWireNumber), crossingRate: normWireRequired(normWireNumber), designLife: normWireRequired(normWireNumber), kFactor: normWireRequired(normWireNumber), kwFactor: normWireRequired(normWireNumber), lateralForceCoefficient: normWireRequired(normWireNumber), permittedAmplitudeRatio: normWireRequired(normWireNumber), ringMode: normWireRequired(normWireNumber), shellThickness: normWireRequired(normWireNumber), guyArea: normWireRequired(normWireNumber), guyAngle: normWireRequired(normWireNumber), guyLength: normWireRequired(normWireNumber) });
export const parseSheetPileEnvironment: NormWireReader<SheetPileEnvironment> = normWireLiteral("Unset", "UndisturbedSoil", "PollutedSoil", "CompactedFill", "NonCompactedFill");
export const parseSteelPile: NormWireReader<SteelPile> = normWireObject<SteelPile>({ id: normWireRequired(normWireString), sectionId: normWireRequired(normWireString), materialId: normWireRequired(normWireString), drivingStress: normWireRequired(normWireNumber), embeddedLength: normWireRequired(normWireNumber), shaftPerimeter: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseForceAction))), environment: normWireRequired(normWireRef(() => parseSheetPileEnvironment)), designLife: normWireRequired(normWireNumber), wallThickness: normWireRequired(normWireNumber), interlockBendingFactor: normWireRequired(normWireNumber), interlockStiffnessFactor: normWireRequired(normWireNumber), sectionModulus: normWireRequired(normWireNumber), shearArea: normWireRequired(normWireNumber), designMoment: normWireRequired(normWireNumber), designShear: normWireRequired(normWireNumber), interlockShear: normWireRequired(normWireNumber) });
export const parseCraneRunway: NormWireReader<CraneRunway> = normWireObject<CraneRunway>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), wheelContactLength: normWireRequired(normWireNumber), dispersion: normWireRequired(normWireNumber), webThickness: normWireRequired(normWireNumber), fy: normWireRequired(normWireNumber), phi: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseForceAction))), maxWheelLoad: normWireRequired(normWireNumber), loadFactor: normWireRequired(normWireNumber), railInertia: normWireRequired(normWireNumber), railFixed: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), flangeEffectiveWidth: normWireRequired(normWireNumber), flangeThickness: normWireRequired(normWireNumber), wheelEccentricity: normWireRequired(normWireNumber), torsionConstant: normWireRequired(normWireNumber), longitudinalStress: normWireRequired(normWireNumber), railRadius: normWireRequired(normWireNumber), railContactWidth: normWireRequired(normWireNumber), craneNu: normWireRequired(normWireNumber), craneKq: normWireRequired(normWireNumber), damageExponent: normWireRequired(normWireNumber), stressRange: normWireRequired(normWireNumber), detailCategoryStress: normWireRequired(normWireNumber), fatiguePhi: normWireRequired(normWireNumber) });
export const parseStorey: NormWireReader<Storey> = normWireObject<Storey>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), label: normWireRequired(normWireString), height: normWireRequired(normWireNumber), verticalLoad: normWireRequired(normWireNumber), horizontalLoad: normWireRequired(normWireNumber), drift: normWireRequired(normWireNumber), columns: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), plasticAnalysis: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), buildingHeight: normWireRequired(normWireNumber), bracingLoad: normWireRequired(normWireNumber), bracingLength: normWireRequired(normWireNumber), bracingDeflection: normWireRequired(normWireNumber) });
export const parseRestraintSupport: NormWireReader<RestraintSupport> = normWireLiteral("Fork", "Fixed");
export const parseRestraintDistribution: NormWireReader<RestraintDistribution> = normWireLiteral("Uniform", "Udl", "Linear");
export const parseRestraint: NormWireReader<Restraint> = normWireObject<Restraint>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), label: normWireRequired(normWireString), startSupport: normWireRequired(normWireRef(() => parseRestraintSupport)), endSupport: normWireRequired(normWireRef(() => parseRestraintSupport)), intermediatePosition: normWireRequired(normWireNumber), torsionalSpring: normWireRequired(normWireNumber), lateralSpringPosition: normWireRequired(normWireNumber), lateralSpringStiffness: normWireRequired(normWireNumber), loadHeight: normWireRequired(normWireNumber), loadDistribution: normWireRequired(normWireRef(() => parseRestraintDistribution)), designMoment: normWireRequired(normWireNumber), designAxial: normWireRequired(normWireNumber), generalMethod: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), hingeToRestraint: normWireRequired(normWireNumber) });
export const parseChordShape: NormWireReader<ChordShape> = normWireLiteral("Unset", "Channel", "I", "Rhs", "Chs");
export const parseBuiltUpKind: NormWireReader<BuiltUpKind> = normWireLiteral("Laced", "Battened", "Closely");
export const parseBuiltUpMember: NormWireReader<BuiltUpMember> = normWireObject<BuiltUpMember>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), label: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseBuiltUpKind)), chordArea: normWireRequired(normWireNumber), chordInertia: normWireRequired(normWireNumber), chordDistance: normWireRequired(normWireNumber), panelSpacing: normWireRequired(normWireNumber), planes: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), battenInertia: normWireRequired(normWireNumber), diagonalArea: normWireRequired(normWireNumber), diagonalLength: normWireRequired(normWireNumber), designAxial: normWireRequired(normWireNumber), designMoment: normWireRequired(normWireNumber), backToBack: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), minRadius: normWireRequired(normWireNumber), chainSpacing: normWireRequired(normWireNumber), chordShape: normWireRequired(normWireRef(() => parseChordShape)), chordDepth: normWireRequired(normWireNumber), chordWidth: normWireRequired(normWireNumber), chordFlangeThickness: normWireRequired(normWireNumber), chordWebThickness: normWireRequired(normWireNumber) });
export const parseWeldPattern: NormWireReader<WeldPattern> = normWireLiteral("Line", "TwoLines", "Box");
export const parseWeldGroup: NormWireReader<WeldGroup> = normWireObject<WeldGroup>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), label: normWireRequired(normWireString), pattern: normWireRequired(normWireRef(() => parseWeldPattern)), throat: normWireRequired(normWireNumber), width: normWireRequired(normWireNumber), depth: normWireRequired(normWireNumber), shearForce: normWireRequired(normWireNumber), eccentricity: normWireRequired(normWireNumber), steelFu: normWireRequired(normWireNumber), betaW: normWireRequired(normWireNumber) });
export const parseParametricShape: NormWireReader<ParametricShape> = normWireLiteral("WeldedI", "Rhs", "Chs", "Angle");
export const parseParametricSection: NormWireReader<ParametricSection> = normWireObject<ParametricSection>({ id: normWireRequired(normWireString), sectionId: normWireRequired(normWireString), label: normWireRequired(normWireString), shape: normWireRequired(normWireRef(() => parseParametricShape)), height: normWireRequired(normWireNumber), width: normWireRequired(normWireNumber), webThickness: normWireRequired(normWireNumber), flangeThickness: normWireRequired(normWireNumber), tolerance: normWireRequired(normWireNumber) });
export const parseHollowJoint: NormWireReader<HollowJoint> = normWireObject<HollowJoint>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), label: normWireRequired(normWireString), chordWidth: normWireRequired(normWireNumber), chordHeight: normWireRequired(normWireNumber), chordThickness: normWireRequired(normWireNumber), chordFy: normWireRequired(normWireNumber), chordStress: normWireRequired(normWireNumber), brace1Width: normWireRequired(normWireNumber), brace1Height: normWireRequired(normWireNumber), brace1Thickness: normWireRequired(normWireNumber), brace1Angle: normWireRequired(normWireNumber), brace1Force: normWireRequired(normWireNumber), brace2Width: normWireRequired(normWireNumber), brace2Height: normWireRequired(normWireNumber), brace2Thickness: normWireRequired(normWireNumber), brace2Angle: normWireRequired(normWireNumber), brace2Force: normWireRequired(normWireNumber), gap: normWireRequired(normWireNumber), braceFy: normWireRequired(normWireNumber) });
export const parseJointRowType: NormWireReader<JointRowType> = normWireLiteral("Extended", "FirstBelow", "Inner");
export const parseJointRow: NormWireReader<JointRow> = normWireObject<JointRow>({ id: normWireRequired(normWireString), jointId: normWireRequired(normWireString), label: normWireRequired(normWireString), rowType: normWireRequired(normWireRef(() => parseJointRowType)), lever: normWireRequired(normWireNumber), boltToWeb: normWireRequired(normWireNumber), edgeDistance: normWireRequired(normWireNumber), extensionToFlange: normWireRequired(normWireNumber), extensionEdge: normWireRequired(normWireNumber), alphaFactor: normWireRequired(normWireNumber), pitchToPrevious: normWireRequired(normWireNumber), bolts: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), boltDiameter: normWireRequired(normWireNumber), boltStressArea: normWireRequired(normWireNumber), gripLength: normWireRequired(normWireNumber), boltClass: normWireRequired(normWireString), groupWithPrevious: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseFrameNodeSupport: NormWireReader<FrameNodeSupport> = normWireLiteral("Free", "Pinned", "Fixed", "Roller");
export const parseFrameNode: NormWireReader<FrameNode> = normWireObject<FrameNode>({ id: normWireRequired(normWireString), label: normWireRequired(normWireString), x: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber), support: normWireRequired(normWireRef(() => parseFrameNodeSupport)), loadX: normWireRequired(normWireNumber), loadZ: normWireRequired(normWireNumber) });
export const parseFrameBarRelease: NormWireReader<FrameBarRelease> = normWireLiteral("Rigid", "Pinned", "SemiRigid");
export const parseFrameBar: NormWireReader<FrameBar> = normWireObject<FrameBar>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), label: normWireRequired(normWireString), startNodeId: normWireRequired(normWireString), endNodeId: normWireRequired(normWireString), releaseStart: normWireRequired(normWireRef(() => parseFrameBarRelease)), releaseEnd: normWireRequired(normWireRef(() => parseFrameBarRelease)) });
export const parsePlacement: NormWireReader<Placement> = normWireObject<Placement>({ id: normWireRequired(normWireString), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber), yaw: normWireRequired(normWireNumber) });
export const parseJointPlate: NormWireReader<JointPlate> = normWireObject<JointPlate>({ id: normWireRequired(normWireString), jointId: normWireRequired(normWireString), label: normWireRequired(normWireString), role: normWireRequired(normWireString), plateHeight: normWireRequired(normWireNumber), plateWidth: normWireRequired(normWireNumber), plateThickness: normWireRequired(normWireNumber), plateCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), plateFy: normWireRequired(normWireNumber), plateFu: normWireRequired(normWireNumber), boltClass: normWireRequired(normWireString), boltDiameter: normWireRequired(normWireNumber), holeDiameter: normWireRequired(normWireNumber), boltRows: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), boltsPerRow: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), pitch: normWireRequired(normWireNumber), gauge: normWireRequired(normWireNumber), endDistance: normWireRequired(normWireNumber), edgeDistance: normWireRequired(normWireNumber), shearPlanes: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), eccentricity: normWireRequired(normWireNumber), weldThroat: normWireRequired(normWireNumber), weldFu: normWireRequired(normWireNumber), weldBetaW: normWireRequired(normWireNumber), groupMethod: normWireRequired(normWireRef(() => parseBoltGroupMethod)) });
export const parseFoundationBlock: NormWireReader<FoundationBlock> = normWireObject<FoundationBlock>({ id: normWireRequired(normWireString), jointId: normWireRequired(normWireString), label: normWireRequired(normWireString), blockLength: normWireRequired(normWireNumber), blockWidth: normWireRequired(normWireNumber), blockDepth: normWireRequired(normWireNumber), concreteFck: normWireRequired(normWireNumber), groutFck: normWireRequired(normWireNumber), groutThickness: normWireRequired(normWireNumber), anchorCount: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), anchorClass: normWireRequired(normWireString), anchorDiameter: normWireRequired(normWireNumber), anchorStressArea: normWireRequired(normWireNumber), anchorEmbedment: normWireRequired(normWireNumber), anchorEdgeDistance: normWireRequired(normWireNumber) });
export const parseForceAction: NormWireReader<ForceAction> = normWireObject<ForceAction>({ id: normWireRequired(normWireString), loadCaseId: normWireRequired(normWireString), force: normWireRequired(normWireNumber) });
export const parseJointForceAction: NormWireReader<JointForceAction> = normWireObject<JointForceAction>({ id: normWireRequired(normWireString), loadCaseId: normWireRequired(normWireString), shear: normWireRequired(normWireNumber), tension: normWireRequired(normWireNumber) });
export const parseFatigueBand: NormWireReader<FatigueBand> = normWireObject<FatigueBand>({ id: normWireRequired(normWireString), deltaSigma: normWireRequired(normWireNumber), cycles: normWireRequired(normWireNumber) });
export const parseEn1993AnnexChoice: NormWireReader<En1993AnnexChoice> = normWireLiteral("En", "De");
export const parseFrameActionKind: NormWireReader<FrameActionKind> = normWireLiteral("NodalForce", "NodalMoment", "BarUniform");
export const parseFrameAction: NormWireReader<FrameAction> = normWireObject<FrameAction>({ id: normWireRequired(normWireString), loadCaseId: normWireRequired(normWireString), targetId: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseFrameActionKind)), fx: normWireRequired(normWireNumber), fz: normWireRequired(normWireNumber), my: normWireRequired(normWireNumber), startRatio: normWireRequired(normWireNumber), endRatio: normWireRequired(normWireNumber) });
export const parseMemberForceBasis: NormWireReader<MemberForceBasis> = normWireLiteral("FrameDerived", "Explicit");
export const parseFrameMemberBasis: NormWireReader<FrameMemberBasis> = normWireObject<FrameMemberBasis>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), basis: normWireRequired(normWireRef(() => parseMemberForceBasis)) });
export const parseHollowFabrication: NormWireReader<HollowFabrication> = normWireLiteral("Unset", "HotFinished", "ColdFormed");
export const parseBoltGroupMethod: NormWireReader<BoltGroupMethod> = normWireLiteral("Elastic", "InstantaneousCentre");
export const parseStressRelief: NormWireReader<StressRelief> = normWireLiteral("AsWelded", "StressRelieved", "NonWelded");
export const parseFatigueStress: NormWireReader<FatigueStress> = normWireObject<FatigueStress>({ id: normWireRequired(normWireString), detailId: normWireRequired(normWireString), maxCaseId: normWireRequired(normWireString), minCaseId: normWireRequired(normWireString), fibreZ: normWireRequired(normWireNumber), fibreY: normWireRequired(normWireNumber), relief: normWireRequired(normWireRef(() => parseStressRelief)), concentration: normWireRequired(normWireNumber), thickness: normWireRequired(normWireNumber), cycles: normWireRequired(normWireNumber) });
export const parseBucklingPlane: NormWireReader<BucklingPlane> = normWireLiteral("MajorAxis", "MinorAxis");
export const parseFrameSway: NormWireReader<FrameSway> = normWireLiteral("NonSway", "Sway");
export const parseNodeARestraint: NormWireReader<NodeARestraint> = normWireLiteral("Framed", "Fixed", "Pinned");
export const parseNodeBRestraint: NormWireReader<NodeBRestraint> = normWireLiteral("Framed", "Fixed", "Pinned");
export const parseBucklingLength: NormWireReader<BucklingLength> = normWireObject<BucklingLength>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), plane: normWireRequired(normWireRef(() => parseBucklingPlane)), sway: normWireRequired(normWireRef(() => parseFrameSway)), restraintA: normWireRequired(normWireRef(() => parseNodeARestraint)), columnStiffnessA: normWireRequired(normWireNumber), beamStiffnessA: normWireRequired(normWireNumber), restraintB: normWireRequired(normWireRef(() => parseNodeBRestraint)), columnStiffnessB: normWireRequired(normWireNumber), beamStiffnessB: normWireRequired(normWireNumber) });
export const parseFormingProcess: NormWireReader<FormingProcess> = normWireLiteral("RollForming", "OtherForming");
export const parseFormingGain: NormWireReader<FormingGain> = normWireObject<FormingGain>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), forming: normWireRequired(normWireRef(() => parseFormingProcess)), ultimateStrength: normWireRequired(normWireNumber), grossArea: normWireRequired(normWireNumber), bends: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), usedYield: normWireRequired(normWireNumber) });
export const parseMaterialDuctility: NormWireReader<MaterialDuctility> = normWireObject<MaterialDuctility>({ id: normWireRequired(normWireString), materialId: normWireRequired(normWireString), elongation: normWireRequired(normWireNumber), ultimateStrain: normWireRequired(normWireNumber) });
export const parseFastenerKind: NormWireReader<FastenerKind> = normWireLiteral("SelfTappingScrew", "BlindRivet");
export const parseFastenedJoint: NormWireReader<FastenedJoint> = normWireObject<FastenedJoint>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseFastenerKind)), diameter: normWireRequired(normWireNumber), washerDiameter: normWireRequired(normWireNumber), count: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), perRow: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), headThickness: normWireRequired(normWireNumber), baseThickness: normWireRequired(normWireNumber), headStrength: normWireRequired(normWireNumber), baseStrength: normWireRequired(normWireNumber), endDistance: normWireRequired(normWireNumber), edgeDistance: normWireRequired(normWireNumber), pitch: normWireRequired(normWireNumber), gauge: normWireRequired(normWireNumber), sheetWidth: normWireRequired(normWireNumber), shearForce: normWireRequired(normWireNumber), tensionForce: normWireRequired(normWireNumber), shearResistance: normWireRequired(normWireNumber) });
export const parseFatigueConsequence: NormWireReader<FatigueConsequence> = normWireLiteral("Low", "High");
