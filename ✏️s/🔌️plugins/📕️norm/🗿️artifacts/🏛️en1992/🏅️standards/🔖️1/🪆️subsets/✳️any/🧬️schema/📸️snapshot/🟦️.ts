/** 📸️ `En1992Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

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
  cementType: CementClass;
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
  /** @state artifact */
  placements: Placement[];
  /** @state artifact */
  cases: CaseRow[];
  /** @state artifact */
  reliabilityClass: "RC1" | "RC2" | "RC3";
}

export type CementClass = "S" | "N" | "R";

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
  ductility: DuctilityClass;
  k: number;
  epsUk: number;
}

export interface PrestressSteel {
  id: string;
  name: string;
  fPk: number;
  fP01k: number;
}

export type BarPosition = "Bottom" | "Top" | "Tension" | "Compression";

export interface BarLayer {
  id: string;
  diameter: number;
  count: number;
  position: BarPosition;
  anchorageLength: number;
  lapLength: number;
  bondCondition: BondCondition;
  aggregateSize: number;
  lapShare: number;
}

export interface SectionPoint {
  id: string;
  y: number;
  z: number;
}

export type BarBend = "Auto" | "Straight" | "Hook" | "UHook";

export interface BarPoint {
  id: string;
  layerId: string;
  y: number;
  z: number;
  diameter: number;
  length: number;
  bend: BarBend;
}

export interface TendonPoint {
  id: string;
  x: number;
  y: number;
  z: number;
}

export interface StudRow {
  id: string;
  angle: number;
  count: number;
  spacingFirst: number;
  spacingRadial: number;
  diameter: number;
}

export interface Opening {
  id: string;
  x: number;
  y: number;
  width: number;
  depth: number;
}

export interface Stirrups {
  diameter: number;
  spacing: number;
  legs: number;
}

export type ColumnPosition = "Interior" | "Edge" | "Corner" | "WallEnd";

export type PunchingBetaMethod = "Simplified" | "UnbalancedMoments";

export interface PunchingSpec {
  columnWidth: number;
  columnDepth: number;
  columnPosition: ColumnPosition;
  asw: number;
  betaMethod: PunchingBetaMethod;
  momentC1: number;
  momentC2: number;
}

export interface PrestressSpec {
  force: number;
  area: number;
  eccentricity: number;
  lossRatio: number;
}

export type SlabFireSystem = "OneWay" | "TwoWay";

export interface FireSpec {
  rating: FireRating;
  axisDistance: number;
  slabSystem: SlabFireSystem;
}

export type ActionKind = "Permanent" | "Imposed" | "Snow" | "Wind" | "Accidental" | "Prestress";

export type ActionSource = "Udl" | "Point" | "External";

export type PsiCategory = "SelfWeight" | "Office" | "Residential" | "Congregation" | "Shopping" | "Storage" | "Snow" | "SnowHigh" | "Wind" | "Temperature" | "Accidental";

export interface LoadCaseActions {
  id: string;
  kind: ActionKind;
  category: PsiCategory;
  source: ActionSource;
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
  stirrups?: Stirrups | null;
  punching?: PunchingSpec | null;
  prestress?: PrestressSpec | null;
  fire?: FireSpec | null;
  actions: LoadCaseActions[];
  useFem: boolean;
  udl: number;
  deflectionSensitive: boolean;
  tightness?: TightnessClass | null;
  hdOverH: number;
  liquidSigmaS: number;
  liquidRhoPEff: number;
  liquidFCtEff: number;
  liquidSRMax: number;
  bridgeSigmaC: number;
  bridgeDeltaSigmaS: number;
  relativeHumidity: number;
  bearingWidth: number;
  sectionPoints: SectionPoint[];
  bars: BarPoint[];
  tendonPoints: TendonPoint[];
  studRows: StudRow[];
  openings: Opening[];
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
  headDiameter: number;
}

export interface Placement {
  id: string;
  x: number;
  y: number;
  z: number;
  yaw: number;
}

export type AnnexChoice = "En" | "De";

export type DuctilityClass = "a" | "b" | "c";

export type BondCondition = "Good" | "Poor";

export type ExposureClass = "X0" | "Xc1" | "Xc2" | "Xc3" | "Xc4" | "Xd1" | "Xd2" | "Xd3" | "Xs1" | "Xs2" | "Xs3" | "Xf1" | "Xf2" | "Xf3" | "Xf4" | "Xa1" | "Xa2" | "Xa3";

export type FireRating = "R30" | "R60" | "R90" | "R120";

export type MemberKind = "Beam" | "Slab" | "Column" | "Wall" | "FlatSlab" | "RibbedSlab" | "TensionMember" | "Bridge" | "LiquidRetaining";

export type SupportCondition = "SimplySupported" | "Continuous" | "Cantilever" | "Fixed";

export type TightnessClass = "Tc0" | "Tc1" | "Tc2";

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

export const parseEn1992Snapshot: NormWireReader<En1992Snapshot> = normWireObject<En1992Snapshot>({ annex: normWireRequired(normWireRef(() => parseAnnexChoice)), title: normWireRequired(normWireString), designWorkingLifeYears: normWireRequired(normWireNumber), deltaCDev: normWireRequired(normWireNumber), cementType: normWireRequired(normWireRef(() => parseCementClass)), concreteGrades: normWireRequired(normWireArray(normWireRef(() => parseConcreteGrade))), reinforcementGrades: normWireRequired(normWireArray(normWireRef(() => parseReinforcementGrade))), prestressSteels: normWireRequired(normWireArray(normWireRef(() => parsePrestressSteel))), members: normWireRequired(normWireArray(normWireRef(() => parseRcMember))), anchors: normWireRequired(normWireArray(normWireRef(() => parseAnchor))), placements: normWireRequired(normWireArray(normWireRef(() => parsePlacement))), cases: normWireRequired(normWireArray(normWireRef(() => parseCaseRow))), reliabilityClass: normWireRequired(normWireLiteral("RC1", "RC2", "RC3")) });
export const parseCementClass: NormWireReader<CementClass> = normWireLiteral("S", "N", "R");
export const parseConcreteGrade: NormWireReader<ConcreteGrade> = normWireObject<ConcreteGrade>({ id: normWireRequired(normWireString), name: normWireRequired(normWireString), fCk: normWireRequired(normWireNumber) });
export const parseReinforcementGrade: NormWireReader<ReinforcementGrade> = normWireObject<ReinforcementGrade>({ id: normWireRequired(normWireString), name: normWireRequired(normWireString), fYk: normWireRequired(normWireNumber), eS: normWireRequired(normWireNumber), ductility: normWireRequired(normWireRef(() => parseDuctilityClass)), k: normWireRequired(normWireNumber), epsUk: normWireRequired(normWireNumber) });
export const parsePrestressSteel: NormWireReader<PrestressSteel> = normWireObject<PrestressSteel>({ id: normWireRequired(normWireString), name: normWireRequired(normWireString), fPk: normWireRequired(normWireNumber), fP01k: normWireRequired(normWireNumber) });
export const parseBarPosition: NormWireReader<BarPosition> = normWireLiteral("Bottom", "Top", "Tension", "Compression");
export const parseBarLayer: NormWireReader<BarLayer> = normWireObject<BarLayer>({ id: normWireRequired(normWireString), diameter: normWireRequired(normWireNumber), count: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), position: normWireRequired(normWireRef(() => parseBarPosition)), anchorageLength: normWireRequired(normWireNumber), lapLength: normWireRequired(normWireNumber), bondCondition: normWireRequired(normWireRef(() => parseBondCondition)), aggregateSize: normWireRequired(normWireNumber), lapShare: normWireRequired(normWireRange(normWireNumber, {"minimum":0,"maximum":1})) });
export const parseSectionPoint: NormWireReader<SectionPoint> = normWireObject<SectionPoint>({ id: normWireRequired(normWireString), y: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber) });
export const parseBarBend: NormWireReader<BarBend> = normWireLiteral("Auto", "Straight", "Hook", "UHook");
export const parseBarPoint: NormWireReader<BarPoint> = normWireObject<BarPoint>({ id: normWireRequired(normWireString), layerId: normWireRequired(normWireString), y: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber), diameter: normWireRequired(normWireNumber), length: normWireRequired(normWireNumber), bend: normWireRequired(normWireRef(() => parseBarBend)) });
export const parseTendonPoint: NormWireReader<TendonPoint> = normWireObject<TendonPoint>({ id: normWireRequired(normWireString), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber) });
export const parseStudRow: NormWireReader<StudRow> = normWireObject<StudRow>({ id: normWireRequired(normWireString), angle: normWireRequired(normWireNumber), count: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), spacingFirst: normWireRequired(normWireNumber), spacingRadial: normWireRequired(normWireNumber), diameter: normWireRequired(normWireNumber) });
export const parseOpening: NormWireReader<Opening> = normWireObject<Opening>({ id: normWireRequired(normWireString), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), width: normWireRequired(normWireNumber), depth: normWireRequired(normWireNumber) });
export const parseStirrups: NormWireReader<Stirrups> = normWireObject<Stirrups>({ diameter: normWireRequired(normWireNumber), spacing: normWireRequired(normWireNumber), legs: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})) });
export const parseColumnPosition: NormWireReader<ColumnPosition> = normWireLiteral("Interior", "Edge", "Corner", "WallEnd");
export const parsePunchingBetaMethod: NormWireReader<PunchingBetaMethod> = normWireLiteral("Simplified", "UnbalancedMoments");
export const parsePunchingSpec: NormWireReader<PunchingSpec> = normWireObject<PunchingSpec>({ columnWidth: normWireRequired(normWireNumber), columnDepth: normWireRequired(normWireNumber), columnPosition: normWireRequired(normWireRef(() => parseColumnPosition)), asw: normWireRequired(normWireNumber), betaMethod: normWireRequired(normWireRef(() => parsePunchingBetaMethod)), momentC1: normWireRequired(normWireNumber), momentC2: normWireRequired(normWireNumber) });
export const parsePrestressSpec: NormWireReader<PrestressSpec> = normWireObject<PrestressSpec>({ force: normWireRequired(normWireNumber), area: normWireRequired(normWireNumber), eccentricity: normWireRequired(normWireNumber), lossRatio: normWireRequired(normWireNumber) });
export const parseSlabFireSystem: NormWireReader<SlabFireSystem> = normWireLiteral("OneWay", "TwoWay");
export const parseFireSpec: NormWireReader<FireSpec> = normWireObject<FireSpec>({ rating: normWireRequired(normWireRef(() => parseFireRating)), axisDistance: normWireRequired(normWireNumber), slabSystem: normWireRequired(normWireRef(() => parseSlabFireSystem)) });
export const parseActionKind: NormWireReader<ActionKind> = normWireLiteral("Permanent", "Imposed", "Snow", "Wind", "Accidental", "Prestress");
export const parseActionSource: NormWireReader<ActionSource> = normWireLiteral("Udl", "Point", "External");
export const parsePsiCategory: NormWireReader<PsiCategory> = normWireLiteral("SelfWeight", "Office", "Residential", "Congregation", "Shopping", "Storage", "Snow", "SnowHigh", "Wind", "Temperature", "Accidental");
export const parseLoadCaseActions: NormWireReader<LoadCaseActions> = normWireObject<LoadCaseActions>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseActionKind)), category: normWireRequired(normWireRef(() => parsePsiCategory)), source: normWireRequired(normWireRef(() => parseActionSource)), gKLine: normWireRequired(normWireNumber), qKLine: normWireRequired(normWireNumber), pointForce: normWireRequired(normWireNumber), mK: normWireRequired(normWireNumber), nK: normWireRequired(normWireNumber), vK: normWireRequired(normWireNumber), tK: normWireRequired(normWireNumber), vKPunch: normWireRequired(normWireNumber) });
export const parseRcMember: NormWireReader<RcMember> = normWireObject<RcMember>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseMemberKind)), concreteGradeId: normWireRequired(normWireString), reinforcementGradeId: normWireRequired(normWireString), prestressSteelId: normWireRequired(normWireString), exposure: normWireRequired(normWireRef(() => parseExposureClass)), width: normWireRequired(normWireNumber), height: normWireRequired(normWireNumber), effectiveDepth: normWireRequired(normWireNumber), cover: normWireRequired(normWireNumber), span: normWireRequired(normWireNumber), support: normWireRequired(normWireRef(() => parseSupportCondition)), bucklingLength: normWireRequired(normWireNumber), longitudinal: normWireRequired(normWireArray(normWireRef(() => parseBarLayer))), stirrups: normWireOptional(normWireNullable(normWireRef(() => parseStirrups))), punching: normWireOptional(normWireNullable(normWireRef(() => parsePunchingSpec))), prestress: normWireOptional(normWireNullable(normWireRef(() => parsePrestressSpec))), fire: normWireOptional(normWireNullable(normWireRef(() => parseFireSpec))), actions: normWireRequired(normWireArray(normWireRef(() => parseLoadCaseActions))), useFem: normWireRequired(normWireBoolean), udl: normWireRequired(normWireNumber), deflectionSensitive: normWireRequired(normWireBoolean), tightness: normWireOptional(normWireNullable(normWireRef(() => parseTightnessClass))), hdOverH: normWireRequired(normWireNumber), liquidSigmaS: normWireRequired(normWireNumber), liquidRhoPEff: normWireRequired(normWireNumber), liquidFCtEff: normWireRequired(normWireNumber), liquidSRMax: normWireRequired(normWireNumber), bridgeSigmaC: normWireRequired(normWireNumber), bridgeDeltaSigmaS: normWireRequired(normWireNumber), relativeHumidity: normWireRequired(normWireNumber), bearingWidth: normWireRequired(normWireNumber), sectionPoints: normWireRequired(normWireArray(normWireRef(() => parseSectionPoint))), bars: normWireRequired(normWireArray(normWireRef(() => parseBarPoint))), tendonPoints: normWireRequired(normWireArray(normWireRef(() => parseTendonPoint))), studRows: normWireRequired(normWireArray(normWireRef(() => parseStudRow))), openings: normWireRequired(normWireArray(normWireRef(() => parseOpening))) });
export const parseAnchor: NormWireReader<Anchor> = normWireObject<Anchor>({ id: normWireRequired(normWireString), hEf: normWireRequired(normWireNumber), cracked: normWireRequired(normWireBoolean), fUk: normWireRequired(normWireNumber), fYk: normWireRequired(normWireNumber), aS: normWireRequired(normWireNumber), d: normWireRequired(normWireNumber), c1: normWireRequired(normWireNumber), fCk: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseLoadCaseActions))), headDiameter: normWireRequired(normWireNumber) });
export const parsePlacement: NormWireReader<Placement> = normWireObject<Placement>({ id: normWireRequired(normWireString), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber), yaw: normWireRequired(normWireNumber) });
export const parseAnnexChoice: NormWireReader<AnnexChoice> = normWireLiteral("En", "De");
export const parseDuctilityClass: NormWireReader<DuctilityClass> = normWireLiteral("a", "b", "c");
export const parseBondCondition: NormWireReader<BondCondition> = normWireLiteral("Good", "Poor");
export const parseExposureClass: NormWireReader<ExposureClass> = normWireLiteral("X0", "Xc1", "Xc2", "Xc3", "Xc4", "Xd1", "Xd2", "Xd3", "Xs1", "Xs2", "Xs3", "Xf1", "Xf2", "Xf3", "Xf4", "Xa1", "Xa2", "Xa3");
export const parseFireRating: NormWireReader<FireRating> = normWireLiteral("R30", "R60", "R90", "R120");
export const parseMemberKind: NormWireReader<MemberKind> = normWireLiteral("Beam", "Slab", "Column", "Wall", "FlatSlab", "RibbedSlab", "TensionMember", "Bridge", "LiquidRetaining");
export const parseSupportCondition: NormWireReader<SupportCondition> = normWireLiteral("SimplySupported", "Continuous", "Cantilever", "Fixed");
export const parseTightnessClass: NormWireReader<TightnessClass> = normWireLiteral("Tc0", "Tc1", "Tc2");
export const parseCaseNumber: NormWireReader<CaseNumber> = normWireObject<CaseNumber>({ id: normWireRequired(normWireString), value: normWireRequired(normWireNumber) });
export const parseCaseText: NormWireReader<CaseText> = normWireObject<CaseText>({ id: normWireRequired(normWireString), value: normWireRequired(normWireString) });
export const parseCaseRow: NormWireReader<CaseRow> = normWireObject<CaseRow>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), entryId: normWireRequired(normWireString), numbers: normWireRequired(normWireArray(normWireRef(() => parseCaseNumber))), texts: normWireRequired(normWireArray(normWireRef(() => parseCaseText))) });
