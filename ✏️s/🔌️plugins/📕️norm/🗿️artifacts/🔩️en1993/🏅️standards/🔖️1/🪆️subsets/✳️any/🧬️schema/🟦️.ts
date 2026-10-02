/** 🧬️ `En1993Artifact` wire twin: the artifact document across its state lanes, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1993Artifact {
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
}

export interface SteelMaterial {
  id: string;
  grade: string;
  fy: number;
  fu: number;
  eModulus: number;
  gModulus: number;
  subgrade: string;
  kind: string;
}

export interface SteelSection {
  id: string;
  designation: string;
  kind: string;
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
}

export interface SteelMember {
  id: string;
  label: string;
  memberType: string;
  sectionId: string;
  materialId: string;
  length: number;
  bucklingLengthY: number;
  bucklingLengthZ: number;
  ltbLength: number;
  ltbRestraintSpacing: number;
  loadApplication: string;
  endMomentRatioPsi: number;
  momentDiagram: string;
  deflectionLimitRatio: number;
  analysis: string;
}

export interface LoadCase {
  id: string;
  name: string;
  kind: string;
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

export interface SteelJoint {
  id: string;
  kind: string;
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
  category: string;
  frictionMu: number;
  preloadForce: number;
  slipFactorKs: number;
  frictionSurfaces: number;
}

export interface FatigueDetail {
  id: string;
  memberId: string;
  category: number;
  method: string;
  spectrum: FatigueBand[];
}

export interface FireExposure {
  id: string;
  memberId: string;
  rating: string;
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
}

export interface PlatedPanel {
  id: string;
  a: number;
  b: number;
  thickness: number;
  fy: number;
  kSigma: number;
  actions: ForceAction[];
}

export interface SiloShell {
  id: string;
  thickness: number;
  radius: number;
  depth: number;
  k: number;
  gamma: number;
  fy: number;
}

export interface TensionComponent {
  id: string;
  fUk: number;
  fK: number;
  actions: ForceAction[];
}

export interface BridgeFatigue {
  id: string;
  memberId: string;
  lambda: number;
  phi2: number;
  deltaSigmaP: number;
  category: number;
  method: string;
}

export interface TowerLeg {
  id: string;
  memberId: string;
  forceCoefficient: number;
  dynamicFactor: number;
  actions: ForceAction[];
}

export interface SteelPile {
  id: string;
  sectionId: string;
  materialId: string;
  drivingStress: number;
  embeddedLength: number;
  shaftPerimeter: number;
  actions: ForceAction[];
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

export const parseEn1993Artifact: NormWireReader<En1993Artifact> = normWireObject<En1993Artifact>({ annex: normWireRequired(normWireLiteral("En", "De")), materials: normWireRequired(normWireArray(normWireRef(() => parseSteelMaterial))), sections: normWireRequired(normWireArray(normWireRef(() => parseSteelSection))), members: normWireRequired(normWireArray(normWireRef(() => parseSteelMember))), loadCases: normWireRequired(normWireArray(normWireRef(() => parseLoadCase))), memberActions: normWireRequired(normWireArray(normWireRef(() => parseMemberAction))), joints: normWireRequired(normWireArray(normWireRef(() => parseSteelJoint))), fatigueDetails: normWireRequired(normWireArray(normWireRef(() => parseFatigueDetail))), fireExposures: normWireRequired(normWireArray(normWireRef(() => parseFireExposure))), coldFormedMembers: normWireRequired(normWireArray(normWireRef(() => parseColdFormedMember))), platedPanels: normWireRequired(normWireArray(normWireRef(() => parsePlatedPanel))), siloShells: normWireRequired(normWireArray(normWireRef(() => parseSiloShell))), tensionComponents: normWireRequired(normWireArray(normWireRef(() => parseTensionComponent))), bridgeFatigue: normWireRequired(normWireArray(normWireRef(() => parseBridgeFatigue))), towerLegs: normWireRequired(normWireArray(normWireRef(() => parseTowerLeg))), piles: normWireRequired(normWireArray(normWireRef(() => parseSteelPile))), craneRunways: normWireRequired(normWireArray(normWireRef(() => parseCraneRunway))) });
export const parseSteelMaterial: NormWireReader<SteelMaterial> = normWireObject<SteelMaterial>({ id: normWireRequired(normWireString), grade: normWireRequired(normWireString), fy: normWireRequired(normWireNumber), fu: normWireRequired(normWireNumber), eModulus: normWireRequired(normWireNumber), gModulus: normWireRequired(normWireNumber), subgrade: normWireRequired(normWireString), kind: normWireRequired(normWireString) });
export const parseSteelSection: NormWireReader<SteelSection> = normWireObject<SteelSection>({ id: normWireRequired(normWireString), designation: normWireRequired(normWireString), kind: normWireRequired(normWireString), h: normWireRequired(normWireNumber), b: normWireRequired(normWireNumber), tw: normWireRequired(normWireNumber), tf: normWireRequired(normWireNumber), r: normWireRequired(normWireNumber), area: normWireRequired(normWireNumber), shearAreaY: normWireRequired(normWireNumber), shearAreaZ: normWireRequired(normWireNumber), iy: normWireRequired(normWireNumber), iz: normWireRequired(normWireNumber), it: normWireRequired(normWireNumber), iw: normWireRequired(normWireNumber), wElY: normWireRequired(normWireNumber), wElZ: normWireRequired(normWireNumber), wPlY: normWireRequired(normWireNumber), wPlZ: normWireRequired(normWireNumber), areaNet: normWireRequired(normWireNumber) });
export const parseSteelMember: NormWireReader<SteelMember> = normWireObject<SteelMember>({ id: normWireRequired(normWireString), label: normWireRequired(normWireString), memberType: normWireRequired(normWireString), sectionId: normWireRequired(normWireString), materialId: normWireRequired(normWireString), length: normWireRequired(normWireNumber), bucklingLengthY: normWireRequired(normWireNumber), bucklingLengthZ: normWireRequired(normWireNumber), ltbLength: normWireRequired(normWireNumber), ltbRestraintSpacing: normWireRequired(normWireNumber), loadApplication: normWireRequired(normWireString), endMomentRatioPsi: normWireRequired(normWireNumber), momentDiagram: normWireRequired(normWireString), deflectionLimitRatio: normWireRequired(normWireNumber), analysis: normWireRequired(normWireString) });
export const parseLoadCase: NormWireReader<LoadCase> = normWireObject<LoadCase>({ id: normWireRequired(normWireString), name: normWireRequired(normWireString), kind: normWireRequired(normWireString), category: normWireRequired(normWireString) });
export const parseDesignAction: NormWireReader<DesignAction> = normWireObject<DesignAction>({ n: normWireRequired(normWireNumber), vy: normWireRequired(normWireNumber), vz: normWireRequired(normWireNumber), my: normWireRequired(normWireNumber), mz: normWireRequired(normWireNumber), t: normWireRequired(normWireNumber) });
export const parseMemberAction: NormWireReader<MemberAction> = normWireObject<MemberAction>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), loadCaseId: normWireRequired(normWireString), action: normWireRequired(normWireRef(() => parseDesignAction)) });
export const parseSteelJoint: NormWireReader<SteelJoint> = normWireObject<SteelJoint>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireString), memberId: normWireRequired(normWireString), boltClass: normWireRequired(normWireString), boltDiameter: normWireRequired(normWireNumber), boltRows: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), boltsPerRow: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), pitch: normWireRequired(normWireNumber), gauge: normWireRequired(normWireNumber), endDistance: normWireRequired(normWireNumber), edgeDistance: normWireRequired(normWireNumber), shearPlanes: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), plateThickness: normWireRequired(normWireNumber), plateFu: normWireRequired(normWireNumber), weldThroat: normWireRequired(normWireNumber), weldLength: normWireRequired(normWireNumber), weldFu: normWireRequired(normWireNumber), weldGrade: normWireRequired(normWireString), actions: normWireRequired(normWireArray(normWireRef(() => parseJointForceAction))), category: normWireRequired(normWireString), frictionMu: normWireRequired(normWireNumber), preloadForce: normWireRequired(normWireNumber), slipFactorKs: normWireRequired(normWireNumber), frictionSurfaces: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseFatigueDetail: NormWireReader<FatigueDetail> = normWireObject<FatigueDetail>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), category: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), method: normWireRequired(normWireString), spectrum: normWireRequired(normWireArray(normWireRef(() => parseFatigueBand))) });
export const parseFireExposure: NormWireReader<FireExposure> = normWireObject<FireExposure>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), rating: normWireRequired(normWireString), protectionThickness: normWireRequired(normWireNumber), sectionFactor: normWireRequired(normWireNumber), mu0: normWireRequired(normWireNumber), designTemperature: normWireRequired(normWireNumber), protectionConductivity: normWireRequired(normWireNumber), protectionDensity: normWireRequired(normWireNumber), protectionSpecificHeat: normWireRequired(normWireNumber) });
export const parseColdFormedMember: NormWireReader<ColdFormedMember> = normWireObject<ColdFormedMember>({ id: normWireRequired(normWireString), bBar: normWireRequired(normWireNumber), thickness: normWireRequired(normWireNumber), kSigma: normWireRequired(normWireNumber), psi: normWireRequired(normWireNumber), fy: normWireRequired(normWireNumber), grossResistance: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseForceAction))) });
export const parsePlatedPanel: NormWireReader<PlatedPanel> = normWireObject<PlatedPanel>({ id: normWireRequired(normWireString), a: normWireRequired(normWireNumber), b: normWireRequired(normWireNumber), thickness: normWireRequired(normWireNumber), fy: normWireRequired(normWireNumber), kSigma: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseForceAction))) });
export const parseSiloShell: NormWireReader<SiloShell> = normWireObject<SiloShell>({ id: normWireRequired(normWireString), thickness: normWireRequired(normWireNumber), radius: normWireRequired(normWireNumber), depth: normWireRequired(normWireNumber), k: normWireRequired(normWireNumber), gamma: normWireRequired(normWireNumber), fy: normWireRequired(normWireNumber) });
export const parseTensionComponent: NormWireReader<TensionComponent> = normWireObject<TensionComponent>({ id: normWireRequired(normWireString), fUk: normWireRequired(normWireNumber), fK: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseForceAction))) });
export const parseBridgeFatigue: NormWireReader<BridgeFatigue> = normWireObject<BridgeFatigue>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), lambda: normWireRequired(normWireNumber), phi2: normWireRequired(normWireNumber), deltaSigmaP: normWireRequired(normWireNumber), category: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), method: normWireRequired(normWireString) });
export const parseTowerLeg: NormWireReader<TowerLeg> = normWireObject<TowerLeg>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), forceCoefficient: normWireRequired(normWireNumber), dynamicFactor: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseForceAction))) });
export const parseSteelPile: NormWireReader<SteelPile> = normWireObject<SteelPile>({ id: normWireRequired(normWireString), sectionId: normWireRequired(normWireString), materialId: normWireRequired(normWireString), drivingStress: normWireRequired(normWireNumber), embeddedLength: normWireRequired(normWireNumber), shaftPerimeter: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseForceAction))) });
export const parseCraneRunway: NormWireReader<CraneRunway> = normWireObject<CraneRunway>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), wheelContactLength: normWireRequired(normWireNumber), dispersion: normWireRequired(normWireNumber), webThickness: normWireRequired(normWireNumber), fy: normWireRequired(normWireNumber), phi: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseForceAction))) });
export const parseForceAction: NormWireReader<ForceAction> = normWireObject<ForceAction>({ id: normWireRequired(normWireString), loadCaseId: normWireRequired(normWireString), force: normWireRequired(normWireNumber) });
export const parseJointForceAction: NormWireReader<JointForceAction> = normWireObject<JointForceAction>({ id: normWireRequired(normWireString), loadCaseId: normWireRequired(normWireString), shear: normWireRequired(normWireNumber), tension: normWireRequired(normWireNumber) });
export const parseFatigueBand: NormWireReader<FatigueBand> = normWireObject<FatigueBand>({ id: normWireRequired(normWireString), deltaSigma: normWireRequired(normWireNumber), cycles: normWireRequired(normWireNumber) });
export const parseEn1993AnnexChoice: NormWireReader<En1993AnnexChoice> = normWireLiteral("En", "De");
