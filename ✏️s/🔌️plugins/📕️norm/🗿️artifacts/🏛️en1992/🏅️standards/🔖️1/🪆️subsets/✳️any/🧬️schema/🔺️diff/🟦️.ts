/** 🔺️ `En1992Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ActionKind, parseActionKind, type ActionSource, parseActionSource, type Anchor, parseAnchor, type AnnexChoice, parseAnnexChoice, type BarBend, parseBarBend, type BarLayer, parseBarLayer, type BarPoint, parseBarPoint, type BarPosition, parseBarPosition, type BondCondition, parseBondCondition, type CaseNumber, parseCaseNumber, type CaseRow, parseCaseRow, type CaseText, parseCaseText, type CementClass, parseCementClass, type ColumnPosition, parseColumnPosition, type ConcreteGrade, parseConcreteGrade, type DuctilityClass, parseDuctilityClass, type ExposureClass, parseExposureClass, type FireRating, parseFireRating, type LoadCaseActions, parseLoadCaseActions, type MemberKind, parseMemberKind, type PrestressSteel, parsePrestressSteel, type PsiCategory, parsePsiCategory, type RcMember, parseRcMember, type ReinforcementGrade, parseReinforcementGrade, type SectionPoint, parseSectionPoint, type SlabFireSystem, parseSlabFireSystem, type StudRow, parseStudRow, type SupportCondition, parseSupportCondition, type TendonPoint, parseTendonPoint, type TightnessClass, parseTightnessClass } from "../📸️snapshot/🟦️.ts";

export interface En1992Diff {
  /** @state artifact */
  annex?: AnnexChoice | null;
  /** @state artifact */
  title?: string | null;
  /** @state artifact */
  designWorkingLifeYears?: number | null;
  /** @state artifact */
  deltaCDev?: number | null;
  /** @state artifact */
  cementType?: CementClass | null;
  /** @state artifact */
  concreteGrades?: En1992ConcreteGradesRows | null;
  /** @state artifact */
  reinforcementGrades?: En1992ReinforcementGradesRows | null;
  /** @state artifact */
  prestressSteels?: En1992PrestressSteelsRows | null;
  /** @state artifact */
  members?: En1992MembersRows | null;
  /** @state artifact */
  anchors?: En1992AnchorsRows | null;
  /** @state artifact */
  placements?: En1992PlacementsRows | null;
  /** @state artifact */
  cases: CaseRowDelta;
  /** @state artifact */
  reliabilityClass?: ("RC1" | "RC2" | "RC3") | null;
}

export interface En1992AnchorsActionsInserted {
  index: number;
  row: LoadCaseActions;
}

export interface En1992AnchorsActionsModified {
  id: string;
  patch: En1992AnchorsActionsPatch;
}

export interface En1992AnchorsActionsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1992AnchorsActionsPatch {
  kind?: ActionKind | null;
  category?: PsiCategory | null;
  source?: ActionSource | null;
  gKLine?: number | null;
  qKLine?: number | null;
  pointForce?: number | null;
  mK?: number | null;
  nK?: number | null;
  vK?: number | null;
  tK?: number | null;
  vKPunch?: number | null;
}

export interface En1992AnchorsActionsRemoved {
  id: string;
  index: number;
}

export interface En1992AnchorsActionsRows {
  removed: En1992AnchorsActionsRemoved[];
  inserted: En1992AnchorsActionsInserted[];
  moved: En1992AnchorsActionsMoved[];
  modified: En1992AnchorsActionsModified[];
}

export interface En1992AnchorsInserted {
  index: number;
  row: Anchor;
}

export interface En1992AnchorsModified {
  id: string;
  patch: En1992AnchorsPatch;
}

export interface En1992AnchorsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1992AnchorsPatch {
  hEf?: number | null;
  cracked?: boolean | null;
  fUk?: number | null;
  fYk?: number | null;
  aS?: number | null;
  d?: number | null;
  c1?: number | null;
  fCk?: number | null;
  headDiameter?: number | null;
  actions?: En1992AnchorsActionsRows | null;
}

export interface En1992AnchorsRemoved {
  id: string;
  index: number;
}

export interface En1992AnchorsRows {
  removed: En1992AnchorsRemoved[];
  inserted: En1992AnchorsInserted[];
  moved: En1992AnchorsMoved[];
  modified: En1992AnchorsModified[];
}

export interface En1992ConcreteGradesInserted {
  index: number;
  row: ConcreteGrade;
}

export interface En1992ConcreteGradesModified {
  id: string;
  patch: En1992ConcreteGradesPatch;
}

export interface En1992ConcreteGradesMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1992ConcreteGradesPatch {
  name?: string | null;
  fCk?: number | null;
}

export interface En1992ConcreteGradesRemoved {
  id: string;
  index: number;
}

export interface En1992ConcreteGradesRows {
  removed: En1992ConcreteGradesRemoved[];
  inserted: En1992ConcreteGradesInserted[];
  moved: En1992ConcreteGradesMoved[];
  modified: En1992ConcreteGradesModified[];
}

export interface En1992MembersActionsInserted {
  index: number;
  row: LoadCaseActions;
}

export interface En1992MembersActionsModified {
  id: string;
  patch: En1992MembersActionsPatch;
}

export interface En1992MembersActionsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1992MembersActionsPatch {
  kind?: ActionKind | null;
  category?: PsiCategory | null;
  source?: ActionSource | null;
  gKLine?: number | null;
  qKLine?: number | null;
  pointForce?: number | null;
  mK?: number | null;
  nK?: number | null;
  vK?: number | null;
  tK?: number | null;
  vKPunch?: number | null;
}

export interface En1992MembersActionsRemoved {
  id: string;
  index: number;
}

export interface En1992MembersActionsRows {
  removed: En1992MembersActionsRemoved[];
  inserted: En1992MembersActionsInserted[];
  moved: En1992MembersActionsMoved[];
  modified: En1992MembersActionsModified[];
}

export interface En1992MembersBarsInserted {
  index: number;
  row: BarPoint;
}

export interface En1992MembersBarsModified {
  id: string;
  patch: En1992MembersBarsPatch;
}

export interface En1992MembersBarsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1992MembersBarsPatch {
  layerId?: string | null;
  y?: number | null;
  z?: number | null;
  diameter?: number | null;
  length?: number | null;
  bend?: BarBend | null;
}

export interface En1992MembersBarsRemoved {
  id: string;
  index: number;
}

export interface En1992MembersBarsRows {
  removed: En1992MembersBarsRemoved[];
  inserted: En1992MembersBarsInserted[];
  moved: En1992MembersBarsMoved[];
  modified: En1992MembersBarsModified[];
}

export interface En1992MembersInserted {
  index: number;
  row: RcMember;
}

export interface En1992MembersLongitudinalInserted {
  index: number;
  row: BarLayer;
}

export interface En1992MembersLongitudinalModified {
  id: string;
  patch: En1992MembersLongitudinalPatch;
}

export interface En1992MembersLongitudinalMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1992MembersLongitudinalPatch {
  diameter?: number | null;
  count?: number | null;
  position?: BarPosition | null;
  anchorageLength?: number | null;
  lapLength?: number | null;
  bondCondition?: BondCondition | null;
  aggregateSize?: number | null;
  lapShare?: number | null;
}

export interface En1992MembersLongitudinalRemoved {
  id: string;
  index: number;
}

export interface En1992MembersLongitudinalRows {
  removed: En1992MembersLongitudinalRemoved[];
  inserted: En1992MembersLongitudinalInserted[];
  moved: En1992MembersLongitudinalMoved[];
  modified: En1992MembersLongitudinalModified[];
}

export interface En1992MembersModified {
  id: string;
  patch: En1992MembersPatch;
}

export interface En1992MembersMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1992MembersPatch {
  labelEn?: string | null;
  labelDe?: string | null;
  kind?: MemberKind | null;
  concreteGradeId?: string | null;
  reinforcementGradeId?: string | null;
  prestressSteelId?: string | null;
  exposure?: ExposureClass | null;
  width?: number | null;
  height?: number | null;
  effectiveDepth?: number | null;
  cover?: number | null;
  span?: number | null;
  support?: SupportCondition | null;
  bucklingLength?: number | null;
  stirrupsDiameter?: number | null;
  stirrupsSpacing?: number | null;
  stirrupsLegs?: number | null;
  punchingColumnWidth?: number | null;
  punchingColumnDepth?: number | null;
  punchingColumnPosition?: ColumnPosition | null;
  punchingAsw?: number | null;
  prestressForce?: number | null;
  prestressArea?: number | null;
  prestressEccentricity?: number | null;
  prestressLossRatio?: number | null;
  fireRating?: FireRating | null;
  fireAxisDistance?: number | null;
  fireSlabSystem?: SlabFireSystem | null;
  useFem?: boolean | null;
  udl?: number | null;
  deflectionSensitive?: boolean | null;
  hdOverH?: number | null;
  liquidSigmaS?: number | null;
  liquidRhoPEff?: number | null;
  liquidFCtEff?: number | null;
  liquidSRMax?: number | null;
  bridgeSigmaC?: number | null;
  bridgeDeltaSigmaS?: number | null;
  relativeHumidity?: number | null;
  bearingWidth?: number | null;
  tightness?: (TightnessClass | null) | null;
  clearTightness?: boolean | null;
  longitudinal?: En1992MembersLongitudinalRows | null;
  actions?: En1992MembersActionsRows | null;
  sectionPoints?: En1992MembersSectionPointsRows | null;
  bars?: En1992MembersBarsRows | null;
  tendonPoints?: En1992MembersTendonPointsRows | null;
  studRows?: En1992MembersStudRowsRows | null;
}

export interface En1992MembersRemoved {
  id: string;
  index: number;
}

export interface En1992MembersRows {
  removed: En1992MembersRemoved[];
  inserted: En1992MembersInserted[];
  moved: En1992MembersMoved[];
  modified: En1992MembersModified[];
}

export interface En1992MembersSectionPointsInserted {
  index: number;
  row: SectionPoint;
}

export interface En1992MembersSectionPointsModified {
  id: string;
  patch: En1992MembersSectionPointsPatch;
}

export interface En1992MembersSectionPointsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1992MembersSectionPointsPatch {
  y?: number | null;
  z?: number | null;
}

export interface En1992MembersSectionPointsRemoved {
  id: string;
  index: number;
}

export interface En1992MembersSectionPointsRows {
  removed: En1992MembersSectionPointsRemoved[];
  inserted: En1992MembersSectionPointsInserted[];
  moved: En1992MembersSectionPointsMoved[];
  modified: En1992MembersSectionPointsModified[];
}

export interface En1992MembersStudRowsInserted {
  index: number;
  row: StudRow;
}

export interface En1992MembersStudRowsModified {
  id: string;
  patch: En1992MembersStudRowsPatch;
}

export interface En1992MembersStudRowsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1992MembersStudRowsPatch {
  angle?: number | null;
  count?: number | null;
  spacingFirst?: number | null;
  spacingRadial?: number | null;
  diameter?: number | null;
}

export interface En1992MembersStudRowsRemoved {
  id: string;
  index: number;
}

export interface En1992MembersStudRowsRows {
  removed: En1992MembersStudRowsRemoved[];
  inserted: En1992MembersStudRowsInserted[];
  moved: En1992MembersStudRowsMoved[];
  modified: En1992MembersStudRowsModified[];
}

export interface En1992MembersTendonPointsInserted {
  index: number;
  row: TendonPoint;
}

export interface En1992MembersTendonPointsModified {
  id: string;
  patch: En1992MembersTendonPointsPatch;
}

export interface En1992MembersTendonPointsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1992MembersTendonPointsPatch {
  x?: number | null;
  y?: number | null;
  z?: number | null;
}

export interface En1992MembersTendonPointsRemoved {
  id: string;
  index: number;
}

export interface En1992MembersTendonPointsRows {
  removed: En1992MembersTendonPointsRemoved[];
  inserted: En1992MembersTendonPointsInserted[];
  moved: En1992MembersTendonPointsMoved[];
  modified: En1992MembersTendonPointsModified[];
}

export interface En1992PlacementsInserted {
  index: number;
  row: LoadCaseActions;
}

export interface En1992PlacementsModified {
  id: string;
  patch: En1992PlacementsPatch;
}

export interface En1992PlacementsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1992PlacementsPatch {
  x?: number | null;
  y?: number | null;
  z?: number | null;
  yaw?: number | null;
}

export interface En1992PlacementsRemoved {
  id: string;
  index: number;
}

export interface En1992PlacementsRows {
  removed: En1992PlacementsRemoved[];
  inserted: En1992PlacementsInserted[];
  moved: En1992PlacementsMoved[];
  modified: En1992PlacementsModified[];
}

export interface En1992PrestressSteelsInserted {
  index: number;
  row: PrestressSteel;
}

export interface En1992PrestressSteelsModified {
  id: string;
  patch: En1992PrestressSteelsPatch;
}

export interface En1992PrestressSteelsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1992PrestressSteelsPatch {
  name?: string | null;
  fPk?: number | null;
  fP01k?: number | null;
}

export interface En1992PrestressSteelsRemoved {
  id: string;
  index: number;
}

export interface En1992PrestressSteelsRows {
  removed: En1992PrestressSteelsRemoved[];
  inserted: En1992PrestressSteelsInserted[];
  moved: En1992PrestressSteelsMoved[];
  modified: En1992PrestressSteelsModified[];
}

export interface En1992ReinforcementGradesInserted {
  index: number;
  row: ReinforcementGrade;
}

export interface En1992ReinforcementGradesModified {
  id: string;
  patch: En1992ReinforcementGradesPatch;
}

export interface En1992ReinforcementGradesMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1992ReinforcementGradesPatch {
  name?: string | null;
  fYk?: number | null;
  eS?: number | null;
  ductility?: DuctilityClass | null;
  k?: number | null;
  epsUk?: number | null;
}

export interface En1992ReinforcementGradesRemoved {
  id: string;
  index: number;
}

export interface En1992ReinforcementGradesRows {
  removed: En1992ReinforcementGradesRemoved[];
  inserted: En1992ReinforcementGradesInserted[];
  moved: En1992ReinforcementGradesMoved[];
  modified: En1992ReinforcementGradesModified[];
}

export interface CaseNumberPatch {
  value?: number | null;
}

export interface CaseNumberRemoval {
  id: string;
  index: number;
}

export interface CaseNumberInsertion {
  index: number;
  row: CaseNumber;
}

export interface CaseNumberRelocation {
  id: string;
  from: number;
  to: number;
}

export interface CaseNumberModification {
  id: string;
  patch: CaseNumberPatch;
}

export interface CaseNumberDelta {
  removed: CaseNumberRemoval[];
  inserted: CaseNumberInsertion[];
  moved: CaseNumberRelocation[];
  modified: CaseNumberModification[];
}

export interface CaseTextPatch {
  value?: string | null;
}

export interface CaseTextRemoval {
  id: string;
  index: number;
}

export interface CaseTextInsertion {
  index: number;
  row: CaseText;
}

export interface CaseTextRelocation {
  id: string;
  from: number;
  to: number;
}

export interface CaseTextModification {
  id: string;
  patch: CaseTextPatch;
}

export interface CaseTextDelta {
  removed: CaseTextRemoval[];
  inserted: CaseTextInsertion[];
  moved: CaseTextRelocation[];
  modified: CaseTextModification[];
}

export interface CaseRowPatch {
  labelEn?: string | null;
  labelDe?: string | null;
  entryId?: string | null;
  numbers: CaseNumberDelta;
  texts: CaseTextDelta;
}

export interface CaseRowRemoval {
  id: string;
  index: number;
}

export interface CaseRowInsertion {
  index: number;
  row: CaseRow;
}

export interface CaseRowRelocation {
  id: string;
  from: number;
  to: number;
}

export interface CaseRowModification {
  id: string;
  patch: CaseRowPatch;
}

export interface CaseRowDelta {
  removed: CaseRowRemoval[];
  inserted: CaseRowInsertion[];
  moved: CaseRowRelocation[];
  modified: CaseRowModification[];
}

export interface En1992MembersOpeningsInserted {
  index: number;
  row: LoadCaseActions;
}

export interface En1992MembersOpeningsModified {
  id: string;
  patch: En1992MembersOpeningsPatch;
}

export interface En1992MembersOpeningsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1992MembersOpeningsPatch {
  x?: number | null;
  y?: number | null;
  width?: number | null;
  depth?: number | null;
}

export interface En1992MembersOpeningsRemoved {
  id: string;
  index: number;
}

export interface En1992MembersOpeningsRows {
  removed: En1992MembersOpeningsRemoved[];
  inserted: En1992MembersOpeningsInserted[];
  moved: En1992MembersOpeningsMoved[];
  modified: En1992MembersOpeningsModified[];
}

export const parseEn1992Diff: NormWireReader<En1992Diff> = normWireObject<En1992Diff>({ annex: normWireOptional(normWireNullable(parseAnnexChoice)), title: normWireOptional(normWireNullable(normWireString)), designWorkingLifeYears: normWireOptional(normWireNullable(normWireNumber)), deltaCDev: normWireOptional(normWireNullable(normWireNumber)), cementType: normWireOptional(normWireNullable(parseCementClass)), concreteGrades: normWireOptional(normWireNullable(normWireRef(() => parseEn1992ConcreteGradesRows))), reinforcementGrades: normWireOptional(normWireNullable(normWireRef(() => parseEn1992ReinforcementGradesRows))), prestressSteels: normWireOptional(normWireNullable(normWireRef(() => parseEn1992PrestressSteelsRows))), members: normWireOptional(normWireNullable(normWireRef(() => parseEn1992MembersRows))), anchors: normWireOptional(normWireNullable(normWireRef(() => parseEn1992AnchorsRows))), placements: normWireOptional(normWireNullable(normWireRef(() => parseEn1992PlacementsRows))), cases: normWireRequired(normWireRef(() => parseCaseRowDelta)), reliabilityClass: normWireOptional(normWireNullable(normWireLiteral("RC1", "RC2", "RC3"))) });
export const parseEn1992AnchorsActionsInserted: NormWireReader<En1992AnchorsActionsInserted> = normWireObject<En1992AnchorsActionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseLoadCaseActions) });
export const parseEn1992AnchorsActionsModified: NormWireReader<En1992AnchorsActionsModified> = normWireObject<En1992AnchorsActionsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992AnchorsActionsPatch)) });
export const parseEn1992AnchorsActionsMoved: NormWireReader<En1992AnchorsActionsMoved> = normWireObject<En1992AnchorsActionsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992AnchorsActionsPatch: NormWireReader<En1992AnchorsActionsPatch> = normWireObject<En1992AnchorsActionsPatch>({ kind: normWireOptional(normWireNullable(parseActionKind)), category: normWireOptional(normWireNullable(parsePsiCategory)), source: normWireOptional(normWireNullable(parseActionSource)), gKLine: normWireOptional(normWireNullable(normWireNumber)), qKLine: normWireOptional(normWireNullable(normWireNumber)), pointForce: normWireOptional(normWireNullable(normWireNumber)), mK: normWireOptional(normWireNullable(normWireNumber)), nK: normWireOptional(normWireNullable(normWireNumber)), vK: normWireOptional(normWireNullable(normWireNumber)), tK: normWireOptional(normWireNullable(normWireNumber)), vKPunch: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1992AnchorsActionsRemoved: NormWireReader<En1992AnchorsActionsRemoved> = normWireObject<En1992AnchorsActionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992AnchorsActionsRows: NormWireReader<En1992AnchorsActionsRows> = normWireObject<En1992AnchorsActionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992AnchorsActionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992AnchorsActionsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992AnchorsActionsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992AnchorsActionsModified))) });
export const parseEn1992AnchorsInserted: NormWireReader<En1992AnchorsInserted> = normWireObject<En1992AnchorsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseAnchor) });
export const parseEn1992AnchorsModified: NormWireReader<En1992AnchorsModified> = normWireObject<En1992AnchorsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992AnchorsPatch)) });
export const parseEn1992AnchorsMoved: NormWireReader<En1992AnchorsMoved> = normWireObject<En1992AnchorsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992AnchorsPatch: NormWireReader<En1992AnchorsPatch> = normWireObject<En1992AnchorsPatch>({ hEf: normWireOptional(normWireNullable(normWireNumber)), cracked: normWireOptional(normWireNullable(normWireBoolean)), fUk: normWireOptional(normWireNullable(normWireNumber)), fYk: normWireOptional(normWireNullable(normWireNumber)), aS: normWireOptional(normWireNullable(normWireNumber)), d: normWireOptional(normWireNullable(normWireNumber)), c1: normWireOptional(normWireNullable(normWireNumber)), fCk: normWireOptional(normWireNullable(normWireNumber)), headDiameter: normWireOptional(normWireNullable(normWireNumber)), actions: normWireOptional(normWireNullable(normWireRef(() => parseEn1992AnchorsActionsRows))) });
export const parseEn1992AnchorsRemoved: NormWireReader<En1992AnchorsRemoved> = normWireObject<En1992AnchorsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992AnchorsRows: NormWireReader<En1992AnchorsRows> = normWireObject<En1992AnchorsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992AnchorsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992AnchorsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992AnchorsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992AnchorsModified))) });
export const parseEn1992ConcreteGradesInserted: NormWireReader<En1992ConcreteGradesInserted> = normWireObject<En1992ConcreteGradesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseConcreteGrade) });
export const parseEn1992ConcreteGradesModified: NormWireReader<En1992ConcreteGradesModified> = normWireObject<En1992ConcreteGradesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992ConcreteGradesPatch)) });
export const parseEn1992ConcreteGradesMoved: NormWireReader<En1992ConcreteGradesMoved> = normWireObject<En1992ConcreteGradesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992ConcreteGradesPatch: NormWireReader<En1992ConcreteGradesPatch> = normWireObject<En1992ConcreteGradesPatch>({ name: normWireOptional(normWireNullable(normWireString)), fCk: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1992ConcreteGradesRemoved: NormWireReader<En1992ConcreteGradesRemoved> = normWireObject<En1992ConcreteGradesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992ConcreteGradesRows: NormWireReader<En1992ConcreteGradesRows> = normWireObject<En1992ConcreteGradesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992ConcreteGradesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992ConcreteGradesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992ConcreteGradesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992ConcreteGradesModified))) });
export const parseEn1992MembersActionsInserted: NormWireReader<En1992MembersActionsInserted> = normWireObject<En1992MembersActionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseLoadCaseActions) });
export const parseEn1992MembersActionsModified: NormWireReader<En1992MembersActionsModified> = normWireObject<En1992MembersActionsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992MembersActionsPatch)) });
export const parseEn1992MembersActionsMoved: NormWireReader<En1992MembersActionsMoved> = normWireObject<En1992MembersActionsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersActionsPatch: NormWireReader<En1992MembersActionsPatch> = normWireObject<En1992MembersActionsPatch>({ kind: normWireOptional(normWireNullable(parseActionKind)), category: normWireOptional(normWireNullable(parsePsiCategory)), source: normWireOptional(normWireNullable(parseActionSource)), gKLine: normWireOptional(normWireNullable(normWireNumber)), qKLine: normWireOptional(normWireNullable(normWireNumber)), pointForce: normWireOptional(normWireNullable(normWireNumber)), mK: normWireOptional(normWireNullable(normWireNumber)), nK: normWireOptional(normWireNullable(normWireNumber)), vK: normWireOptional(normWireNullable(normWireNumber)), tK: normWireOptional(normWireNullable(normWireNumber)), vKPunch: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1992MembersActionsRemoved: NormWireReader<En1992MembersActionsRemoved> = normWireObject<En1992MembersActionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersActionsRows: NormWireReader<En1992MembersActionsRows> = normWireObject<En1992MembersActionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersActionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersActionsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersActionsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersActionsModified))) });
export const parseEn1992MembersBarsInserted: NormWireReader<En1992MembersBarsInserted> = normWireObject<En1992MembersBarsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseBarPoint) });
export const parseEn1992MembersBarsModified: NormWireReader<En1992MembersBarsModified> = normWireObject<En1992MembersBarsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992MembersBarsPatch)) });
export const parseEn1992MembersBarsMoved: NormWireReader<En1992MembersBarsMoved> = normWireObject<En1992MembersBarsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersBarsPatch: NormWireReader<En1992MembersBarsPatch> = normWireObject<En1992MembersBarsPatch>({ layerId: normWireOptional(normWireNullable(normWireString)), y: normWireOptional(normWireNullable(normWireNumber)), z: normWireOptional(normWireNullable(normWireNumber)), diameter: normWireOptional(normWireNullable(normWireNumber)), length: normWireOptional(normWireNullable(normWireNumber)), bend: normWireOptional(normWireNullable(parseBarBend)) });
export const parseEn1992MembersBarsRemoved: NormWireReader<En1992MembersBarsRemoved> = normWireObject<En1992MembersBarsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersBarsRows: NormWireReader<En1992MembersBarsRows> = normWireObject<En1992MembersBarsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersBarsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersBarsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersBarsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersBarsModified))) });
export const parseEn1992MembersInserted: NormWireReader<En1992MembersInserted> = normWireObject<En1992MembersInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseRcMember) });
export const parseEn1992MembersLongitudinalInserted: NormWireReader<En1992MembersLongitudinalInserted> = normWireObject<En1992MembersLongitudinalInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseBarLayer) });
export const parseEn1992MembersLongitudinalModified: NormWireReader<En1992MembersLongitudinalModified> = normWireObject<En1992MembersLongitudinalModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992MembersLongitudinalPatch)) });
export const parseEn1992MembersLongitudinalMoved: NormWireReader<En1992MembersLongitudinalMoved> = normWireObject<En1992MembersLongitudinalMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersLongitudinalPatch: NormWireReader<En1992MembersLongitudinalPatch> = normWireObject<En1992MembersLongitudinalPatch>({ diameter: normWireOptional(normWireNullable(normWireNumber)), count: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), position: normWireOptional(normWireNullable(parseBarPosition)), anchorageLength: normWireOptional(normWireNullable(normWireNumber)), lapLength: normWireOptional(normWireNullable(normWireNumber)), bondCondition: normWireOptional(normWireNullable(parseBondCondition)), aggregateSize: normWireOptional(normWireNullable(normWireNumber)), lapShare: normWireOptional(normWireNullable(normWireRange(normWireNumber, {"minimum":0,"maximum":1}))) });
export const parseEn1992MembersLongitudinalRemoved: NormWireReader<En1992MembersLongitudinalRemoved> = normWireObject<En1992MembersLongitudinalRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersLongitudinalRows: NormWireReader<En1992MembersLongitudinalRows> = normWireObject<En1992MembersLongitudinalRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersLongitudinalRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersLongitudinalInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersLongitudinalMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersLongitudinalModified))) });
export const parseEn1992MembersModified: NormWireReader<En1992MembersModified> = normWireObject<En1992MembersModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992MembersPatch)) });
export const parseEn1992MembersMoved: NormWireReader<En1992MembersMoved> = normWireObject<En1992MembersMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersPatch: NormWireReader<En1992MembersPatch> = normWireObject<En1992MembersPatch>({ labelEn: normWireOptional(normWireNullable(normWireString)), labelDe: normWireOptional(normWireNullable(normWireString)), kind: normWireOptional(normWireNullable(parseMemberKind)), concreteGradeId: normWireOptional(normWireNullable(normWireString)), reinforcementGradeId: normWireOptional(normWireNullable(normWireString)), prestressSteelId: normWireOptional(normWireNullable(normWireString)), exposure: normWireOptional(normWireNullable(parseExposureClass)), width: normWireOptional(normWireNullable(normWireNumber)), height: normWireOptional(normWireNullable(normWireNumber)), effectiveDepth: normWireOptional(normWireNullable(normWireNumber)), cover: normWireOptional(normWireNullable(normWireNumber)), span: normWireOptional(normWireNullable(normWireNumber)), support: normWireOptional(normWireNullable(parseSupportCondition)), bucklingLength: normWireOptional(normWireNullable(normWireNumber)), stirrupsDiameter: normWireOptional(normWireNullable(normWireNumber)), stirrupsSpacing: normWireOptional(normWireNullable(normWireNumber)), stirrupsLegs: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), punchingColumnWidth: normWireOptional(normWireNullable(normWireNumber)), punchingColumnDepth: normWireOptional(normWireNullable(normWireNumber)), punchingColumnPosition: normWireOptional(normWireNullable(parseColumnPosition)), punchingAsw: normWireOptional(normWireNullable(normWireNumber)), prestressForce: normWireOptional(normWireNullable(normWireNumber)), prestressArea: normWireOptional(normWireNullable(normWireNumber)), prestressEccentricity: normWireOptional(normWireNullable(normWireNumber)), prestressLossRatio: normWireOptional(normWireNullable(normWireNumber)), fireRating: normWireOptional(normWireNullable(parseFireRating)), fireAxisDistance: normWireOptional(normWireNullable(normWireNumber)), fireSlabSystem: normWireOptional(normWireNullable(parseSlabFireSystem)), useFem: normWireOptional(normWireNullable(normWireBoolean)), udl: normWireOptional(normWireNullable(normWireNumber)), deflectionSensitive: normWireOptional(normWireNullable(normWireBoolean)), hdOverH: normWireOptional(normWireNullable(normWireNumber)), liquidSigmaS: normWireOptional(normWireNullable(normWireNumber)), liquidRhoPEff: normWireOptional(normWireNullable(normWireNumber)), liquidFCtEff: normWireOptional(normWireNullable(normWireNumber)), liquidSRMax: normWireOptional(normWireNullable(normWireNumber)), bridgeSigmaC: normWireOptional(normWireNullable(normWireNumber)), bridgeDeltaSigmaS: normWireOptional(normWireNullable(normWireNumber)), relativeHumidity: normWireOptional(normWireNullable(normWireNumber)), bearingWidth: normWireOptional(normWireNullable(normWireNumber)), tightness: normWireOptional(normWireNullable(normWireNullable(parseTightnessClass))), clearTightness: normWireOptional(normWireNullable(normWireBoolean)), longitudinal: normWireOptional(normWireNullable(normWireRef(() => parseEn1992MembersLongitudinalRows))), actions: normWireOptional(normWireNullable(normWireRef(() => parseEn1992MembersActionsRows))), sectionPoints: normWireOptional(normWireNullable(normWireRef(() => parseEn1992MembersSectionPointsRows))), bars: normWireOptional(normWireNullable(normWireRef(() => parseEn1992MembersBarsRows))), tendonPoints: normWireOptional(normWireNullable(normWireRef(() => parseEn1992MembersTendonPointsRows))), studRows: normWireOptional(normWireNullable(normWireRef(() => parseEn1992MembersStudRowsRows))) });
export const parseEn1992MembersRemoved: NormWireReader<En1992MembersRemoved> = normWireObject<En1992MembersRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersRows: NormWireReader<En1992MembersRows> = normWireObject<En1992MembersRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersModified))) });
export const parseEn1992MembersSectionPointsInserted: NormWireReader<En1992MembersSectionPointsInserted> = normWireObject<En1992MembersSectionPointsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseSectionPoint) });
export const parseEn1992MembersSectionPointsModified: NormWireReader<En1992MembersSectionPointsModified> = normWireObject<En1992MembersSectionPointsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992MembersSectionPointsPatch)) });
export const parseEn1992MembersSectionPointsMoved: NormWireReader<En1992MembersSectionPointsMoved> = normWireObject<En1992MembersSectionPointsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersSectionPointsPatch: NormWireReader<En1992MembersSectionPointsPatch> = normWireObject<En1992MembersSectionPointsPatch>({ y: normWireOptional(normWireNullable(normWireNumber)), z: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1992MembersSectionPointsRemoved: NormWireReader<En1992MembersSectionPointsRemoved> = normWireObject<En1992MembersSectionPointsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersSectionPointsRows: NormWireReader<En1992MembersSectionPointsRows> = normWireObject<En1992MembersSectionPointsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersSectionPointsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersSectionPointsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersSectionPointsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersSectionPointsModified))) });
export const parseEn1992MembersStudRowsInserted: NormWireReader<En1992MembersStudRowsInserted> = normWireObject<En1992MembersStudRowsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseStudRow) });
export const parseEn1992MembersStudRowsModified: NormWireReader<En1992MembersStudRowsModified> = normWireObject<En1992MembersStudRowsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992MembersStudRowsPatch)) });
export const parseEn1992MembersStudRowsMoved: NormWireReader<En1992MembersStudRowsMoved> = normWireObject<En1992MembersStudRowsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersStudRowsPatch: NormWireReader<En1992MembersStudRowsPatch> = normWireObject<En1992MembersStudRowsPatch>({ angle: normWireOptional(normWireNullable(normWireNumber)), count: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), spacingFirst: normWireOptional(normWireNullable(normWireNumber)), spacingRadial: normWireOptional(normWireNullable(normWireNumber)), diameter: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1992MembersStudRowsRemoved: NormWireReader<En1992MembersStudRowsRemoved> = normWireObject<En1992MembersStudRowsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersStudRowsRows: NormWireReader<En1992MembersStudRowsRows> = normWireObject<En1992MembersStudRowsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersStudRowsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersStudRowsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersStudRowsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersStudRowsModified))) });
export const parseEn1992MembersTendonPointsInserted: NormWireReader<En1992MembersTendonPointsInserted> = normWireObject<En1992MembersTendonPointsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseTendonPoint) });
export const parseEn1992MembersTendonPointsModified: NormWireReader<En1992MembersTendonPointsModified> = normWireObject<En1992MembersTendonPointsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992MembersTendonPointsPatch)) });
export const parseEn1992MembersTendonPointsMoved: NormWireReader<En1992MembersTendonPointsMoved> = normWireObject<En1992MembersTendonPointsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersTendonPointsPatch: NormWireReader<En1992MembersTendonPointsPatch> = normWireObject<En1992MembersTendonPointsPatch>({ x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), z: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1992MembersTendonPointsRemoved: NormWireReader<En1992MembersTendonPointsRemoved> = normWireObject<En1992MembersTendonPointsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersTendonPointsRows: NormWireReader<En1992MembersTendonPointsRows> = normWireObject<En1992MembersTendonPointsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersTendonPointsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersTendonPointsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersTendonPointsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersTendonPointsModified))) });
export const parseEn1992PlacementsInserted: NormWireReader<En1992PlacementsInserted> = normWireObject<En1992PlacementsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseLoadCaseActions) });
export const parseEn1992PlacementsModified: NormWireReader<En1992PlacementsModified> = normWireObject<En1992PlacementsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992PlacementsPatch)) });
export const parseEn1992PlacementsMoved: NormWireReader<En1992PlacementsMoved> = normWireObject<En1992PlacementsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992PlacementsPatch: NormWireReader<En1992PlacementsPatch> = normWireObject<En1992PlacementsPatch>({ x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), z: normWireOptional(normWireNullable(normWireNumber)), yaw: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1992PlacementsRemoved: NormWireReader<En1992PlacementsRemoved> = normWireObject<En1992PlacementsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992PlacementsRows: NormWireReader<En1992PlacementsRows> = normWireObject<En1992PlacementsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992PlacementsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992PlacementsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992PlacementsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992PlacementsModified))) });
export const parseEn1992PrestressSteelsInserted: NormWireReader<En1992PrestressSteelsInserted> = normWireObject<En1992PrestressSteelsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parsePrestressSteel) });
export const parseEn1992PrestressSteelsModified: NormWireReader<En1992PrestressSteelsModified> = normWireObject<En1992PrestressSteelsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992PrestressSteelsPatch)) });
export const parseEn1992PrestressSteelsMoved: NormWireReader<En1992PrestressSteelsMoved> = normWireObject<En1992PrestressSteelsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992PrestressSteelsPatch: NormWireReader<En1992PrestressSteelsPatch> = normWireObject<En1992PrestressSteelsPatch>({ name: normWireOptional(normWireNullable(normWireString)), fPk: normWireOptional(normWireNullable(normWireNumber)), fP01k: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1992PrestressSteelsRemoved: NormWireReader<En1992PrestressSteelsRemoved> = normWireObject<En1992PrestressSteelsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992PrestressSteelsRows: NormWireReader<En1992PrestressSteelsRows> = normWireObject<En1992PrestressSteelsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992PrestressSteelsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992PrestressSteelsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992PrestressSteelsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992PrestressSteelsModified))) });
export const parseEn1992ReinforcementGradesInserted: NormWireReader<En1992ReinforcementGradesInserted> = normWireObject<En1992ReinforcementGradesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseReinforcementGrade) });
export const parseEn1992ReinforcementGradesModified: NormWireReader<En1992ReinforcementGradesModified> = normWireObject<En1992ReinforcementGradesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992ReinforcementGradesPatch)) });
export const parseEn1992ReinforcementGradesMoved: NormWireReader<En1992ReinforcementGradesMoved> = normWireObject<En1992ReinforcementGradesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992ReinforcementGradesPatch: NormWireReader<En1992ReinforcementGradesPatch> = normWireObject<En1992ReinforcementGradesPatch>({ name: normWireOptional(normWireNullable(normWireString)), fYk: normWireOptional(normWireNullable(normWireNumber)), eS: normWireOptional(normWireNullable(normWireNumber)), ductility: normWireOptional(normWireNullable(parseDuctilityClass)), k: normWireOptional(normWireNullable(normWireNumber)), epsUk: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1992ReinforcementGradesRemoved: NormWireReader<En1992ReinforcementGradesRemoved> = normWireObject<En1992ReinforcementGradesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992ReinforcementGradesRows: NormWireReader<En1992ReinforcementGradesRows> = normWireObject<En1992ReinforcementGradesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992ReinforcementGradesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992ReinforcementGradesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992ReinforcementGradesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992ReinforcementGradesModified))) });
export const parseCaseNumberPatch: NormWireReader<CaseNumberPatch> = normWireObject<CaseNumberPatch>({ value: normWireOptional(normWireNullable(normWireNumber)) });
export const parseCaseNumberRemoval: NormWireReader<CaseNumberRemoval> = normWireObject<CaseNumberRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseNumberInsertion: NormWireReader<CaseNumberInsertion> = normWireObject<CaseNumberInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCaseNumber) });
export const parseCaseNumberRelocation: NormWireReader<CaseNumberRelocation> = normWireObject<CaseNumberRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseNumberModification: NormWireReader<CaseNumberModification> = normWireObject<CaseNumberModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseCaseNumberPatch)) });
export const parseCaseNumberDelta: NormWireReader<CaseNumberDelta> = normWireObject<CaseNumberDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberModification))) });
export const parseCaseTextPatch: NormWireReader<CaseTextPatch> = normWireObject<CaseTextPatch>({ value: normWireOptional(normWireNullable(normWireString)) });
export const parseCaseTextRemoval: NormWireReader<CaseTextRemoval> = normWireObject<CaseTextRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseTextInsertion: NormWireReader<CaseTextInsertion> = normWireObject<CaseTextInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCaseText) });
export const parseCaseTextRelocation: NormWireReader<CaseTextRelocation> = normWireObject<CaseTextRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseTextModification: NormWireReader<CaseTextModification> = normWireObject<CaseTextModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseCaseTextPatch)) });
export const parseCaseTextDelta: NormWireReader<CaseTextDelta> = normWireObject<CaseTextDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseCaseTextRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseCaseTextInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseCaseTextRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseCaseTextModification))) });
export const parseCaseRowPatch: NormWireReader<CaseRowPatch> = normWireObject<CaseRowPatch>({ labelEn: normWireOptional(normWireNullable(normWireString)), labelDe: normWireOptional(normWireNullable(normWireString)), entryId: normWireOptional(normWireNullable(normWireString)), numbers: normWireRequired(normWireRef(() => parseCaseNumberDelta)), texts: normWireRequired(normWireRef(() => parseCaseTextDelta)) });
export const parseCaseRowRemoval: NormWireReader<CaseRowRemoval> = normWireObject<CaseRowRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseRowInsertion: NormWireReader<CaseRowInsertion> = normWireObject<CaseRowInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCaseRow) });
export const parseCaseRowRelocation: NormWireReader<CaseRowRelocation> = normWireObject<CaseRowRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseRowModification: NormWireReader<CaseRowModification> = normWireObject<CaseRowModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseCaseRowPatch)) });
export const parseCaseRowDelta: NormWireReader<CaseRowDelta> = normWireObject<CaseRowDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseCaseRowRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseCaseRowInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseCaseRowRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseCaseRowModification))) });
export const parseEn1992MembersOpeningsInserted: NormWireReader<En1992MembersOpeningsInserted> = normWireObject<En1992MembersOpeningsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseLoadCaseActions) });
export const parseEn1992MembersOpeningsModified: NormWireReader<En1992MembersOpeningsModified> = normWireObject<En1992MembersOpeningsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992MembersOpeningsPatch)) });
export const parseEn1992MembersOpeningsMoved: NormWireReader<En1992MembersOpeningsMoved> = normWireObject<En1992MembersOpeningsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersOpeningsPatch: NormWireReader<En1992MembersOpeningsPatch> = normWireObject<En1992MembersOpeningsPatch>({ x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), width: normWireOptional(normWireNullable(normWireNumber)), depth: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1992MembersOpeningsRemoved: NormWireReader<En1992MembersOpeningsRemoved> = normWireObject<En1992MembersOpeningsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersOpeningsRows: NormWireReader<En1992MembersOpeningsRows> = normWireObject<En1992MembersOpeningsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersOpeningsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersOpeningsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersOpeningsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersOpeningsModified))) });
