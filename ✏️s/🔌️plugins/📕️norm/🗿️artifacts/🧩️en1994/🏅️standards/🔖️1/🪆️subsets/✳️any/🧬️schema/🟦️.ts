/** 🧬️ `En1994Artifact` wire twin: the artifact document across its state lanes, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface En1994Artifact {
  /** @state artifact */
  annex: AnnexChoice;
  /** @state artifact */
  structureKind: StructureKind;
  /** @state artifact */
  steelFYPa: number;
  /** @state artifact */
  beams: CompositeBeam[];
  /** @state artifact */
  columns: CompositeColumn[];
  /** @state artifact */
  slabs: CompositeSlab[];
  /** @state artifact */
  placements: Placement[];
  /** @state artifact */
  fireRating: FireRating;
  /** @state artifact */
  insulationThicknessM: number;
  /** @state artifact */
  fatigueDetail: FatigueDetail;
  /** @state artifact */
  reliabilityClass: "RC1" | "RC2" | "RC3";
  /** @state artifact */
  studs: Stud[];
  /** @state artifact */
  cases: CaseRow[];
}

export type FatigueDetail = "studWelded" | "shearConnector" | "reinforcement" | "flangeButtWeld";

export type FireRating = "unrated" | "r30" | "r60" | "r90" | "r120";

export type StructureKind = "building" | "bridge";

export type ActionStage = "construction" | "composite" | "any";

export type ActionKind = "permanent" | "imposed" | "snow" | "wind" | "construction" | "fatigue";

export interface CharacteristicAction {
  id: string;
  kind: ActionKind;
  category: string;
  stage: ActionStage;
  qAreaPa: number;
  fKN: number;
  deltaSigmaKPa: number;
  deltaTauKPa: number;
  xM: number;
}

export interface SlabAction {
  id: string;
  kind: ActionKind;
  category: string;
  stage: ActionStage;
  qAreaPa: number;
  fKN: number;
  deltaSigmaKPa: number;
  deltaTauKPa: number;
  xM: number;
  yM: number;
}

export interface SteelSection {
  designation: string;
  heightM: number;
  widthM: number;
  twM: number;
  tfM: number;
  aM2: number;
  wPlYM3: number;
  iYM4: number;
  aVM2: number;
  rM: number;
}

export type SheetingProfile = "trapezoidal" | "reEntrant";

export interface ProfiledSheeting {
  profile: SheetingProfile;
  heightM: number;
  ribWidthM: number;
  thicknessM: number;
  ribsParallelToBeam: boolean;
}

export type ConstructionMethod = "propped" | "unpropped";

export type SupportCondition = "simplySupported" | "continuous2Span" | "endSpan" | "interiorSpan" | "cantilever";

export type ReinforcementProduct = "hotRolled" | "coldWorked";

export interface CompositeBeam {
  id: string;
  spanM: number;
  spacingM: number;
  support: SupportCondition;
  construction: ConstructionMethod;
  steel: SteelSection;
  slabThicknessM: number;
  concreteFCkPa: number;
  concreteECmPa: number;
  sheeting: ProfiledSheeting;
  transverseAsM2PerM: number;
  ltbLengthM: number;
  asHoggingM2PerM: number;
  barSpacingM: number;
  wkLimitM: number;
  nCycles: number;
  adjacentSpanM: number;
  patchBearingLengthM: number;
  bridgeLanes: number;
  crackedLengthFraction: number;
  studTransverseSpacingM: number;
  studEdgeDistanceM: number;
  transverseAsTopM2PerM: number;
  endPlateWidthM: number;
  endPlateHeightM: number;
  endZoneLengthM: number;
  reinforcementProduct: ReinforcementProduct;
  actions: CharacteristicAction[];
}

export type BucklingCurve = "a" | "b" | "c" | "d";

export type ColumnKind = "encased" | "concreteFilled" | "partiallyEncased";

export interface CompositeColumn {
  id: string;
  kind: ColumnKind;
  lengthM: number;
  outerSizeM: number;
  wallThicknessM: number;
  steelAM2: number;
  steelFYPa: number;
  concreteAM2: number;
  concreteFCkPa: number;
  reinforcementAsM2: number;
  reinforcementFYkPa: number;
  iM4: number;
  bucklingCurve: BucklingCurve;
  introducedForceN: number;
  sectionHeightM: number;
  sectionWidthM: number;
  sectionTwM: number;
  sectionTfM: number;
  rebarCoverM: number;
  reinforcementProduct: ReinforcementProduct;
  actions: ColumnAction[];
}

export interface CompositeSlab {
  id: string;
  spanM: number;
  support: SupportCondition;
  sheeting: ProfiledSheeting;
  concreteThicknessM: number;
  fCkPa: number;
  mFactor: number;
  kFactor: number;
  asM2PerM: number;
  actions: SlabAction[];
}

export interface Placement {
  id: string;
  x: number;
  y: number;
  z: number;
  yaw: number;
}

export interface ColumnAction {
  id: string;
  kind: ActionKind;
  category: string;
  stage: ActionStage;
  nKN: number;
  mKNm: number;
}

export type AnnexChoice = "En" | "De";

export interface Stud {
  id: string;
  beamId: string;
  xM: number;
  diameterM: number;
  heightM: number;
  fUPa: number;
}

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

export const parseEn1994Artifact: NormWireReader<En1994Artifact> = normWireObject<En1994Artifact>({ annex: normWireRequired(normWireRef(() => parseAnnexChoice)), structureKind: normWireRequired(normWireRef(() => parseStructureKind)), steelFYPa: normWireRequired(normWireNumber), beams: normWireRequired(normWireArray(normWireRef(() => parseCompositeBeam))), columns: normWireRequired(normWireArray(normWireRef(() => parseCompositeColumn))), slabs: normWireRequired(normWireArray(normWireRef(() => parseCompositeSlab))), placements: normWireRequired(normWireArray(normWireRef(() => parsePlacement))), fireRating: normWireRequired(normWireRef(() => parseFireRating)), insulationThicknessM: normWireRequired(normWireNumber), fatigueDetail: normWireRequired(normWireRef(() => parseFatigueDetail)), reliabilityClass: normWireRequired(normWireLiteral("RC1", "RC2", "RC3")), studs: normWireRequired(normWireArray(normWireRef(() => parseStud))), cases: normWireRequired(normWireArray(normWireRef(() => parseCaseRow))) });
export const parseFatigueDetail: NormWireReader<FatigueDetail> = normWireLiteral("studWelded", "shearConnector", "reinforcement", "flangeButtWeld");
export const parseFireRating: NormWireReader<FireRating> = normWireLiteral("unrated", "r30", "r60", "r90", "r120");
export const parseStructureKind: NormWireReader<StructureKind> = normWireLiteral("building", "bridge");
export const parseActionStage: NormWireReader<ActionStage> = normWireLiteral("construction", "composite", "any");
export const parseActionKind: NormWireReader<ActionKind> = normWireLiteral("permanent", "imposed", "snow", "wind", "construction", "fatigue");
export const parseCharacteristicAction: NormWireReader<CharacteristicAction> = normWireObject<CharacteristicAction>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseActionKind)), category: normWireRequired(normWireString), stage: normWireRequired(normWireRef(() => parseActionStage)), qAreaPa: normWireRequired(normWireNumber), fKN: normWireRequired(normWireNumber), deltaSigmaKPa: normWireRequired(normWireNumber), deltaTauKPa: normWireRequired(normWireNumber), xM: normWireRequired(normWireNumber) });
export const parseSlabAction: NormWireReader<SlabAction> = normWireObject<SlabAction>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseActionKind)), category: normWireRequired(normWireString), stage: normWireRequired(normWireRef(() => parseActionStage)), qAreaPa: normWireRequired(normWireNumber), fKN: normWireRequired(normWireNumber), deltaSigmaKPa: normWireRequired(normWireNumber), deltaTauKPa: normWireRequired(normWireNumber), xM: normWireRequired(normWireNumber), yM: normWireRequired(normWireNumber) });
export const parseSteelSection: NormWireReader<SteelSection> = normWireObject<SteelSection>({ designation: normWireRequired(normWireString), heightM: normWireRequired(normWireNumber), widthM: normWireRequired(normWireNumber), twM: normWireRequired(normWireNumber), tfM: normWireRequired(normWireNumber), aM2: normWireRequired(normWireNumber), wPlYM3: normWireRequired(normWireNumber), iYM4: normWireRequired(normWireNumber), aVM2: normWireRequired(normWireNumber), rM: normWireRequired(normWireNumber) });
export const parseSheetingProfile: NormWireReader<SheetingProfile> = normWireLiteral("trapezoidal", "reEntrant");
export const parseProfiledSheeting: NormWireReader<ProfiledSheeting> = normWireObject<ProfiledSheeting>({ profile: normWireRequired(normWireRef(() => parseSheetingProfile)), heightM: normWireRequired(normWireNumber), ribWidthM: normWireRequired(normWireNumber), thicknessM: normWireRequired(normWireNumber), ribsParallelToBeam: normWireRequired(normWireBoolean) });
export const parseConstructionMethod: NormWireReader<ConstructionMethod> = normWireLiteral("propped", "unpropped");
export const parseSupportCondition: NormWireReader<SupportCondition> = normWireLiteral("simplySupported", "continuous2Span", "endSpan", "interiorSpan", "cantilever");
export const parseReinforcementProduct: NormWireReader<ReinforcementProduct> = normWireLiteral("hotRolled", "coldWorked");
export const parseCompositeBeam: NormWireReader<CompositeBeam> = normWireObject<CompositeBeam>({ id: normWireRequired(normWireString), spanM: normWireRequired(normWireNumber), spacingM: normWireRequired(normWireNumber), support: normWireRequired(normWireRef(() => parseSupportCondition)), construction: normWireRequired(normWireRef(() => parseConstructionMethod)), steel: normWireRequired(normWireRef(() => parseSteelSection)), slabThicknessM: normWireRequired(normWireNumber), concreteFCkPa: normWireRequired(normWireNumber), concreteECmPa: normWireRequired(normWireNumber), sheeting: normWireRequired(normWireRef(() => parseProfiledSheeting)), transverseAsM2PerM: normWireRequired(normWireNumber), ltbLengthM: normWireRequired(normWireNumber), asHoggingM2PerM: normWireRequired(normWireNumber), barSpacingM: normWireRequired(normWireNumber), wkLimitM: normWireRequired(normWireNumber), nCycles: normWireRequired(normWireNumber), adjacentSpanM: normWireRequired(normWireNumber), patchBearingLengthM: normWireRequired(normWireNumber), bridgeLanes: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), crackedLengthFraction: normWireRequired(normWireNumber), studTransverseSpacingM: normWireRequired(normWireNumber), studEdgeDistanceM: normWireRequired(normWireNumber), transverseAsTopM2PerM: normWireRequired(normWireNumber), endPlateWidthM: normWireRequired(normWireNumber), endPlateHeightM: normWireRequired(normWireNumber), endZoneLengthM: normWireRequired(normWireNumber), reinforcementProduct: normWireRequired(normWireRef(() => parseReinforcementProduct)), actions: normWireRequired(normWireArray(normWireRef(() => parseCharacteristicAction))) });
export const parseBucklingCurve: NormWireReader<BucklingCurve> = normWireLiteral("a", "b", "c", "d");
export const parseColumnKind: NormWireReader<ColumnKind> = normWireLiteral("encased", "concreteFilled", "partiallyEncased");
export const parseCompositeColumn: NormWireReader<CompositeColumn> = normWireObject<CompositeColumn>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseColumnKind)), lengthM: normWireRequired(normWireNumber), outerSizeM: normWireRequired(normWireNumber), wallThicknessM: normWireRequired(normWireNumber), steelAM2: normWireRequired(normWireNumber), steelFYPa: normWireRequired(normWireNumber), concreteAM2: normWireRequired(normWireNumber), concreteFCkPa: normWireRequired(normWireNumber), reinforcementAsM2: normWireRequired(normWireNumber), reinforcementFYkPa: normWireRequired(normWireNumber), iM4: normWireRequired(normWireNumber), bucklingCurve: normWireRequired(normWireRef(() => parseBucklingCurve)), introducedForceN: normWireRequired(normWireNumber), sectionHeightM: normWireRequired(normWireNumber), sectionWidthM: normWireRequired(normWireNumber), sectionTwM: normWireRequired(normWireNumber), sectionTfM: normWireRequired(normWireNumber), rebarCoverM: normWireRequired(normWireNumber), reinforcementProduct: normWireRequired(normWireRef(() => parseReinforcementProduct)), actions: normWireRequired(normWireArray(normWireRef(() => parseColumnAction))) });
export const parseCompositeSlab: NormWireReader<CompositeSlab> = normWireObject<CompositeSlab>({ id: normWireRequired(normWireString), spanM: normWireRequired(normWireNumber), support: normWireRequired(normWireRef(() => parseSupportCondition)), sheeting: normWireRequired(normWireRef(() => parseProfiledSheeting)), concreteThicknessM: normWireRequired(normWireNumber), fCkPa: normWireRequired(normWireNumber), mFactor: normWireRequired(normWireNumber), kFactor: normWireRequired(normWireNumber), asM2PerM: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseSlabAction))) });
export const parsePlacement: NormWireReader<Placement> = normWireObject<Placement>({ id: normWireRequired(normWireString), x: normWireRequired(normWireNumber), y: normWireRequired(normWireNumber), z: normWireRequired(normWireNumber), yaw: normWireRequired(normWireNumber) });
export const parseColumnAction: NormWireReader<ColumnAction> = normWireObject<ColumnAction>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireRef(() => parseActionKind)), category: normWireRequired(normWireString), stage: normWireRequired(normWireRef(() => parseActionStage)), nKN: normWireRequired(normWireNumber), mKNm: normWireRequired(normWireNumber) });
export const parseAnnexChoice: NormWireReader<AnnexChoice> = normWireLiteral("En", "De");
export const parseStud: NormWireReader<Stud> = normWireObject<Stud>({ id: normWireRequired(normWireString), beamId: normWireRequired(normWireString), xM: normWireRequired(normWireNumber), diameterM: normWireRequired(normWireNumber), heightM: normWireRequired(normWireNumber), fUPa: normWireRequired(normWireNumber) });
export const parseCaseNumber: NormWireReader<CaseNumber> = normWireObject<CaseNumber>({ id: normWireRequired(normWireString), value: normWireRequired(normWireNumber) });
export const parseCaseText: NormWireReader<CaseText> = normWireObject<CaseText>({ id: normWireRequired(normWireString), value: normWireRequired(normWireString) });
export const parseCaseRow: NormWireReader<CaseRow> = normWireObject<CaseRow>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), entryId: normWireRequired(normWireString), numbers: normWireRequired(normWireArray(normWireRef(() => parseCaseNumber))), texts: normWireRequired(normWireArray(normWireRef(() => parseCaseText))) });
