/** 🧬️ EN 1992 snapshot TypeScript facet — mirrors Rust subject. */
export type AnnexChoice = "En" | "De";
export type MemberKind = "Beam" | "Slab" | "Column" | "Wall" | "FlatSlab" | "RibbedSlab" | "TensionMember" | "Bridge" | "LiquidRetaining";
export type SupportCondition = "SimplySupported" | "Continuous" | "Cantilever" | "Fixed";
export type ExposureClass = "X0" | "Xc1" | "Xc2" | "Xc3" | "Xc4" | "Xd1" | "Xd2" | "Xd3" | "Xs1" | "Xs2" | "Xs3" | "Xf1" | "Xf2" | "Xf3" | "Xf4" | "Xa1" | "Xa2" | "Xa3";
export type FireRating = "R30" | "R60" | "R90" | "R120";
export type TightnessClass = "Tc0" | "Tc1" | "Tc2";
export type DuctilityClass = "A" | "B" | "C";

export interface ConcreteGrade { id: string; name: string; fCk: number; }
export interface ReinforcementGrade { id: string; name: string; fYk: number; eS: number; ductility: DuctilityClass; k: number; epsUk: number; }
export interface PrestressSteel { id: string; name: string; fPk: number; fP01k: number; }
export interface BarLayer { id: string; diameter: number; count: number; position: string; anchorageLength: number; lapLength: number; bondCondition: string; aggregateSize: number; }
export interface Stirrups { diameter: number; spacing: number; legs: number; }
export interface PunchingSpec { columnWidth: number; columnDepth: number; columnPosition: string; asw: number; }
export interface PrestressSpec { force: number; area: number; eccentricity: number; lossRatio: number; }
export interface FireSpec { rating: FireRating; axisDistance: number; columnMethod: string; slabSystem: string; }
export interface LoadCaseActions {
  id: string; kind: string; category: string; source: string;
  gKLine: number; qKLine: number; pointForce: number;
  mK: number; nK: number; vK: number; tK: number; vKPunch: number;
}
export interface RcMember {
  id: string; labelEn: string; labelDe: string; kind: MemberKind;
  concreteGradeId: string; reinforcementGradeId: string; prestressSteelId: string;
  exposure: ExposureClass; width: number; height: number; effectiveDepth: number; cover: number; span: number;
  support: SupportCondition; bucklingLength: number; longitudinal: BarLayer[];
  stirrups?: Stirrups|null; punching?: PunchingSpec|null; prestress?: PrestressSpec|null; fire?: FireSpec|null;
  actions: LoadCaseActions[]; useFem: boolean; udl: number; deflectionSensitive: boolean; tightness: TightnessClass;
  hdOverH: number; liquidSigmaS: number; liquidRhoPEff: number; liquidFCtEff: number; liquidSRMax: number;
  bridgeSigmaC: number; bridgeDeltaSigmaS: number;
}
export interface Anchor {
  id: string; hEf: number; cracked: boolean; fUk: number; fYk: number; aS: number; d: number; c1: number; fCk: number;
  actions: LoadCaseActions[];
}
export interface En1992Snapshot {
  annex: AnnexChoice; title: string; designWorkingLifeYears: number; deltaCDev: number; cementType: string;
  concreteGrades: ConcreteGrade[]; reinforcementGrades: ReinforcementGrade[]; prestressSteels: PrestressSteel[];
  members: RcMember[]; anchors: Anchor[];
}
