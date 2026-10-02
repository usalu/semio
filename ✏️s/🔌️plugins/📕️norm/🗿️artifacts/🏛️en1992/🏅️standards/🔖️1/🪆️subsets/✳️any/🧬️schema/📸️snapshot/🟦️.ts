/** 📸️ `En1992Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1992Snapshot {
  /** @state artifact */
  annex: AnnexChoice;
  /** @state artifact */
  title: string;
  /** @state artifact */
  designWorkingLifeYears: number;
  /** @state artifact */
  deltaCDev: number;
  /** @state artifact */
  cementType: string;
  /** @state artifact */
  concreteGrades: ConcreteGrade[];
  /** @state artifact */
  reinforcementGrades: ReinforcementGrade[];
  /** @state artifact */
  prestressSteels: PrestressSteel[];
  /** @state artifact */
  members: RcMember[];
  /** @state artifact */
  anchors: Anchor[];
}

export interface ConcreteGrade {
  id: string;
  name: string;
  fCk: number;
}

export interface ReinforcementGrade {
  id: string;
  name: string;
  fYk: number;
  eS: number;
  ductility: "a" | "b" | "c";
  k: number;
  epsUk: number;
}

export interface PrestressSteel {
  id: string;
  name: string;
  fPk: number;
  fP01k: number;
}

export interface BarLayer {
  id: string;
  diameter: number;
  count: number;
  position: string;
  anchorageLength: number;
  lapLength: number;
  bondCondition: string;
  aggregateSize: number;
}

export interface Stirrups {
  diameter: number;
  spacing: number;
  legs: number;
}

export interface PunchingSpec {
  columnWidth: number;
  columnDepth: number;
  columnPosition: string;
  asw: number;
}

export interface PrestressSpec {
  force: number;
  area: number;
  eccentricity: number;
  lossRatio: number;
}

export interface FireSpec {
  rating: FireRating;
  axisDistance: number;
  columnMethod: string;
  slabSystem: string;
}

export interface LoadCaseActions {
  id: string;
  kind: string;
  category: string;
  source: string;
  gKLine: number;
  qKLine: number;
  pointForce: number;
  mK: number;
  nK: number;
  vK: number;
  tK: number;
  vKPunch: number;
}

export interface RcMember {
  id: string;
  labelEn: string;
  labelDe: string;
  kind: MemberKind;
  concreteGradeId: string;
  reinforcementGradeId: string;
  prestressSteelId: string;
  exposure: ExposureClass;
  width: number;
  height: number;
  effectiveDepth: number;
  cover: number;
  span: number;
  support: SupportCondition;
  bucklingLength: number;
  longitudinal: BarLayer[];
  stirrups: Stirrups | null;
  punching: PunchingSpec | null;
  prestress: PrestressSpec | null;
  fire: FireSpec | null;
  actions: LoadCaseActions[];
  useFem: boolean;
  udl: number;
  deflectionSensitive: boolean;
  tightness: TightnessClass | null;
  hdOverH: number;
  liquidSigmaS: number;
  liquidRhoPEff: number;
  liquidFCtEff: number;
  liquidSRMax: number;
  bridgeSigmaC: number;
  bridgeDeltaSigmaS: number;
}

export interface Anchor {
  id: string;
  hEf: number;
  cracked: boolean;
  fUk: number;
  fYk: number;
  aS: number;
  d: number;
  c1: number;
  fCk: number;
  actions: LoadCaseActions[];
}

export type AnnexChoice = "En" | "De";

export type ExposureClass = "X0" | "Xc1" | "Xc2" | "Xc3" | "Xc4" | "Xd1" | "Xd2" | "Xd3" | "Xs1" | "Xs2" | "Xs3" | "Xf1" | "Xf2" | "Xf3" | "Xf4" | "Xa1" | "Xa2" | "Xa3";

export type FireRating = "R30" | "R60" | "R90" | "R120";

export type MemberKind = "Beam" | "Slab" | "Column" | "Wall" | "FlatSlab" | "RibbedSlab" | "TensionMember" | "Bridge" | "LiquidRetaining";

export type SupportCondition = "SimplySupported" | "Continuous" | "Cantilever" | "Fixed";

export type TightnessClass = "Tc0" | "Tc1" | "Tc2";

export type DuctilityClass = "a" | "b" | "c";

export const parseEn1992Snapshot: NormWireReader<En1992Snapshot> = normWireObject<En1992Snapshot>({ annex: normWireRequired(normWireRef(() => parseAnnexChoice)), title: normWireRequired(normWireString), designWorkingLifeYears: normWireRequired(normWireNumber), deltaCDev: normWireRequired(normWireNumber), cementType: normWireRequired(normWireString), concreteGrades: normWireRequired(normWireArray(normWireRef(() => parseConcreteGrade))), reinforcementGrades: normWireRequired(normWireArray(normWireRef(() => parseReinforcementGrade))), prestressSteels: normWireRequired(normWireArray(normWireRef(() => parsePrestressSteel))), members: normWireRequired(normWireArray(normWireRef(() => parseRcMember))), anchors: normWireRequired(normWireArray(normWireRef(() => parseAnchor))) });
export const parseConcreteGrade: NormWireReader<ConcreteGrade> = normWireObject<ConcreteGrade>({ id: normWireRequired(normWireString), name: normWireRequired(normWireString), fCk: normWireRequired(normWireNumber) });
export const parseReinforcementGrade: NormWireReader<ReinforcementGrade> = normWireObject<ReinforcementGrade>({ id: normWireRequired(normWireString), name: normWireRequired(normWireString), fYk: normWireRequired(normWireNumber), eS: normWireRequired(normWireNumber), ductility: normWireRequired(normWireLiteral("a", "b", "c")), k: normWireRequired(normWireNumber), epsUk: normWireRequired(normWireNumber) });
export const parsePrestressSteel: NormWireReader<PrestressSteel> = normWireObject<PrestressSteel>({ id: normWireRequired(normWireString), name: normWireRequired(normWireString), fPk: normWireRequired(normWireNumber), fP01k: normWireRequired(normWireNumber) });
export const parseBarLayer: NormWireReader<BarLayer> = normWireObject<BarLayer>({ id: normWireRequired(normWireString), diameter: normWireRequired(normWireNumber), count: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), position: normWireRequired(normWireString), anchorageLength: normWireRequired(normWireNumber), lapLength: normWireRequired(normWireNumber), bondCondition: normWireRequired(normWireString), aggregateSize: normWireRequired(normWireNumber) });
export const parseStirrups: NormWireReader<Stirrups> = normWireObject<Stirrups>({ diameter: normWireRequired(normWireNumber), spacing: normWireRequired(normWireNumber), legs: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parsePunchingSpec: NormWireReader<PunchingSpec> = normWireObject<PunchingSpec>({ columnWidth: normWireRequired(normWireNumber), columnDepth: normWireRequired(normWireNumber), columnPosition: normWireRequired(normWireString), asw: normWireRequired(normWireNumber) });
export const parsePrestressSpec: NormWireReader<PrestressSpec> = normWireObject<PrestressSpec>({ force: normWireRequired(normWireNumber), area: normWireRequired(normWireNumber), eccentricity: normWireRequired(normWireNumber), lossRatio: normWireRequired(normWireNumber) });
export const parseFireSpec: NormWireReader<FireSpec> = normWireObject<FireSpec>({ rating: normWireRequired(normWireRef(() => parseFireRating)), axisDistance: normWireRequired(normWireNumber), columnMethod: normWireRequired(normWireString), slabSystem: normWireRequired(normWireString) });
export const parseLoadCaseActions: NormWireReader<LoadCaseActions> = normWireObject<LoadCaseActions>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireString), category: normWireRequired(normWireString), source: normWireRequired(normWireString), gKLine: normWireRequired(normWireNumber), qKLine: normWireRequired(normWireNumber), pointForce: normWireRequired(normWireNumber), mK: normWireRequired(normWireNumber), nK: normWireRequired(normWireNumber), vK: normWireRequired(normWireNumber), tK: normWireRequired(normWireNumber), vKPunch: normWireRequired(normWireNumber) });
export const parseRcMember: NormWireReader<RcMember> = normWireObject<RcMember>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseMemberKind)), concreteGradeId: normWireRequired(normWireString), reinforcementGradeId: normWireRequired(normWireString), prestressSteelId: normWireRequired(normWireString), exposure: normWireRequired(normWireRef(() => parseExposureClass)), width: normWireRequired(normWireNumber), height: normWireRequired(normWireNumber), effectiveDepth: normWireRequired(normWireNumber), cover: normWireRequired(normWireNumber), span: normWireRequired(normWireNumber), support: normWireRequired(normWireRef(() => parseSupportCondition)), bucklingLength: normWireRequired(normWireNumber), longitudinal: normWireRequired(normWireArray(normWireRef(() => parseBarLayer))), stirrups: normWireRequired(normWireNullable(normWireRef(() => parseStirrups))), punching: normWireRequired(normWireNullable(normWireRef(() => parsePunchingSpec))), prestress: normWireRequired(normWireNullable(normWireRef(() => parsePrestressSpec))), fire: normWireRequired(normWireNullable(normWireRef(() => parseFireSpec))), actions: normWireRequired(normWireArray(normWireRef(() => parseLoadCaseActions))), useFem: normWireRequired(normWireBoolean), udl: normWireRequired(normWireNumber), deflectionSensitive: normWireRequired(normWireBoolean), tightness: normWireRequired(normWireNullable(normWireRef(() => parseTightnessClass))), hdOverH: normWireRequired(normWireNumber), liquidSigmaS: normWireRequired(normWireNumber), liquidRhoPEff: normWireRequired(normWireNumber), liquidFCtEff: normWireRequired(normWireNumber), liquidSRMax: normWireRequired(normWireNumber), bridgeSigmaC: normWireRequired(normWireNumber), bridgeDeltaSigmaS: normWireRequired(normWireNumber) });
export const parseAnchor: NormWireReader<Anchor> = normWireObject<Anchor>({ id: normWireRequired(normWireString), hEf: normWireRequired(normWireNumber), cracked: normWireRequired(normWireBoolean), fUk: normWireRequired(normWireNumber), fYk: normWireRequired(normWireNumber), aS: normWireRequired(normWireNumber), d: normWireRequired(normWireNumber), c1: normWireRequired(normWireNumber), fCk: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseLoadCaseActions))) });
export const parseAnnexChoice: NormWireReader<AnnexChoice> = normWireLiteral("En", "De");
export const parseExposureClass: NormWireReader<ExposureClass> = normWireLiteral("X0", "Xc1", "Xc2", "Xc3", "Xc4", "Xd1", "Xd2", "Xd3", "Xs1", "Xs2", "Xs3", "Xf1", "Xf2", "Xf3", "Xf4", "Xa1", "Xa2", "Xa3");
export const parseFireRating: NormWireReader<FireRating> = normWireLiteral("R30", "R60", "R90", "R120");
export const parseMemberKind: NormWireReader<MemberKind> = normWireLiteral("Beam", "Slab", "Column", "Wall", "FlatSlab", "RibbedSlab", "TensionMember", "Bridge", "LiquidRetaining");
export const parseSupportCondition: NormWireReader<SupportCondition> = normWireLiteral("SimplySupported", "Continuous", "Cantilever", "Fixed");
export const parseTightnessClass: NormWireReader<TightnessClass> = normWireLiteral("Tc0", "Tc1", "Tc2");
export const parseDuctilityClass: NormWireReader<DuctilityClass> = normWireLiteral("a", "b", "c");

export * from "./🪶️sqlite/🟦️.ts";
