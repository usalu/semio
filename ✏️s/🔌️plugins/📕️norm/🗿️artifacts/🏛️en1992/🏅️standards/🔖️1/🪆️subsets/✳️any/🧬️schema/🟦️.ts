/** 🧬️ En1992 hierarchical snapshot (SI). */
export type AnnexChoice = "en" | "de";
export type MemberKind = "beam" | "slab" | "column" | "wall" | "flat-slab" | "bridge" | "liquid-retaining";
export type SupportCondition = "simply-supported" | "continuous" | "cantilever" | "fixed";
export type ExposureClass = "x0"|"xc1"|"xc2"|"xc3"|"xc4"|"xd1"|"xd2"|"xd3"|"xs1"|"xs2"|"xs3"|"xf1"|"xf2"|"xf3"|"xf4"|"xa1"|"xa2"|"xa3";
export type FireRating = "r30"|"r60"|"r90"|"r120";
export type TightnessClass = "tc0"|"tc1"|"tc2";

export interface ConcreteGrade {
  id: string; name: string; fCk: number; fCkCube: number; fCm: number; fCtm: number; fCtk005: number;
  eCm: number; epsC2: number; epsCu2: number; epsC3: number; epsCu3: number; nParabola: number;
}
export interface ReinforcementGrade { id: string; name: string; fYk: number; eS: number; k: number; epsUk: number; }
export interface PrestressSteel { id: string; name: string; fPk: number; fP01k: number; }
export interface BarLayer {
  id: string; diameter: number; count: number; position: string;
  anchorageLength: number; lapLength: number; bondCondition: string; aggregateSize: number;
}
export interface Stirrups { diameter: number; spacing: number; legs: number; }
export interface PunchingSpec { columnWidth: number; columnDepth: number; columnPosition: string; asw: number; }
export interface PrestressSpec { force: number; area: number; eccentricity: number; lossRatio: number; }
export interface FireSpec { rating: FireRating; axisDistance: number; }
export interface LoadCaseActions {
  id: string; kind: string; category: string; source: string;
  gKLine: number; qKLine: number; pointForce: number;
  mK: number; nK: number; vK: number; tK: number; vKPunch: number;
}
export interface RcMember {
  id: string; labelEn: string; labelDe: string; kind: MemberKind;
  concreteGradeId: string; reinforcementGradeId: string; prestressSteelId: string;
  exposure: ExposureClass; width: number; height: number; effectiveDepth: number; cover: number;
  span: number; support: SupportCondition; bucklingLength: number;
  longitudinal: BarLayer[]; stirrups?: Stirrups|null; punching?: PunchingSpec|null;
  prestress?: PrestressSpec|null; fire?: FireSpec|null; actions: LoadCaseActions[];
  useFem: boolean; udl: number; deflectionSensitive: boolean;
  tightness?: TightnessClass|null; hdOverH: number; liquidSigmaS: number; liquidRhoPEff: number;
  liquidFCtEff: number; liquidSRMax: number; bridgeSigmaC: number; bridgeDeltaSigmaS: number;
}
export interface Anchor {
  id: string; hEf: number; cracked: boolean; fUk: number; fYk: number; aS: number; d: number;
  c1: number; nEd: number; vEd: number; fCk: number;
}
export interface En1992Snapshot {
  annex: AnnexChoice; title: string; designWorkingLifeYears: number; deltaCDev: number; cementType: string;
  concreteGrades: ConcreteGrade[]; reinforcementGrades: ReinforcementGrade[]; prestressSteels: PrestressSteel[];
  members: RcMember[]; anchors: Anchor[];
}
