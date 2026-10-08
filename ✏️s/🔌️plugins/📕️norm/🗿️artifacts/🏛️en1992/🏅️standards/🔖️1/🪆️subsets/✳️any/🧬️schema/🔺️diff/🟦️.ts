/** 🔺️ `En1992Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireInteger, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Anchor, type AnnexChoice, type BarLayer, type ConcreteGrade, type ExposureClass, type FireRating, type LoadCaseActions, parseAnchor, parseAnnexChoice, parseBarLayer, parseConcreteGrade, parseExposureClass, parseFireRating, parseLoadCaseActions, parsePrestressSteel, parseRcMember, parseReinforcementGrade, type PrestressSteel, type RcMember, type ReinforcementGrade } from "../📸️snapshot/🟦️.ts";

export interface En1992Diff {
  /** @state artifact */
  annex: AnnexChoice | null;
  /** @state artifact */
  title: string | null;
  /** @state artifact */
  designWorkingLifeYears: number | null;
  /** @state artifact */
  deltaCDev: number | null;
  /** @state artifact */
  cementType: string | null;
  /** @state artifact */
  concreteGrades: En1992ConcreteGradesRows | null;
  /** @state artifact */
  reinforcementGrades: En1992ReinforcementGradesRows | null;
  /** @state artifact */
  prestressSteels: En1992PrestressSteelsRows | null;
  /** @state artifact */
  members: En1992MembersRows | null;
  /** @state artifact */
  anchors: En1992AnchorsRows | null;
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
  hEf: number | null;
  aS: number | null;
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
  fCk: number | null;
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
  mK: number | null;
  nK: number | null;
  vK: number | null;
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
  diameter: number | null;
  count: number | null;
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
  exposure: ExposureClass | null;
  width: number | null;
  height: number | null;
  effectiveDepth: number | null;
  cover: number | null;
  span: number | null;
  fireRating: FireRating | null;
  fireAxisDistance: number | null;
  stirrupsSpacing: number | null;
  longitudinal: En1992MembersLongitudinalRows | null;
  actions: En1992MembersActionsRows | null;
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
  name: string | null;
  fPk: number | null;
  fP01k: number | null;
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
  fYk: number | null;
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

export const parseEn1992Diff: NormWireReader<En1992Diff> = normWireObject<En1992Diff>({ annex: normWireDefault(normWireNullable(parseAnnexChoice), () => null), title: normWireDefault(normWireNullable(normWireString), () => null), designWorkingLifeYears: normWireDefault(normWireNullable(normWireNumber), () => null), deltaCDev: normWireDefault(normWireNullable(normWireNumber), () => null), cementType: normWireDefault(normWireNullable(normWireString), () => null), concreteGrades: normWireDefault(normWireNullable(normWireRef(() => parseEn1992ConcreteGradesRows)), () => null), reinforcementGrades: normWireDefault(normWireNullable(normWireRef(() => parseEn1992ReinforcementGradesRows)), () => null), prestressSteels: normWireDefault(normWireNullable(normWireRef(() => parseEn1992PrestressSteelsRows)), () => null), members: normWireDefault(normWireNullable(normWireRef(() => parseEn1992MembersRows)), () => null), anchors: normWireDefault(normWireNullable(normWireRef(() => parseEn1992AnchorsRows)), () => null) });
export const parseEn1992AnchorsInserted: NormWireReader<En1992AnchorsInserted> = normWireObject<En1992AnchorsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseAnchor) });
export const parseEn1992AnchorsModified: NormWireReader<En1992AnchorsModified> = normWireObject<En1992AnchorsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992AnchorsPatch)) });
export const parseEn1992AnchorsMoved: NormWireReader<En1992AnchorsMoved> = normWireObject<En1992AnchorsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992AnchorsPatch: NormWireReader<En1992AnchorsPatch> = normWireObject<En1992AnchorsPatch>({ hEf: normWireRequired(normWireNullable(normWireNumber)), aS: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1992AnchorsRemoved: NormWireReader<En1992AnchorsRemoved> = normWireObject<En1992AnchorsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992AnchorsRows: NormWireReader<En1992AnchorsRows> = normWireObject<En1992AnchorsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992AnchorsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992AnchorsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992AnchorsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992AnchorsModified))) });
export const parseEn1992ConcreteGradesInserted: NormWireReader<En1992ConcreteGradesInserted> = normWireObject<En1992ConcreteGradesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseConcreteGrade) });
export const parseEn1992ConcreteGradesModified: NormWireReader<En1992ConcreteGradesModified> = normWireObject<En1992ConcreteGradesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992ConcreteGradesPatch)) });
export const parseEn1992ConcreteGradesMoved: NormWireReader<En1992ConcreteGradesMoved> = normWireObject<En1992ConcreteGradesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992ConcreteGradesPatch: NormWireReader<En1992ConcreteGradesPatch> = normWireObject<En1992ConcreteGradesPatch>({ fCk: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1992ConcreteGradesRemoved: NormWireReader<En1992ConcreteGradesRemoved> = normWireObject<En1992ConcreteGradesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992ConcreteGradesRows: NormWireReader<En1992ConcreteGradesRows> = normWireObject<En1992ConcreteGradesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992ConcreteGradesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992ConcreteGradesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992ConcreteGradesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992ConcreteGradesModified))) });
export const parseEn1992MembersActionsInserted: NormWireReader<En1992MembersActionsInserted> = normWireObject<En1992MembersActionsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseLoadCaseActions) });
export const parseEn1992MembersActionsModified: NormWireReader<En1992MembersActionsModified> = normWireObject<En1992MembersActionsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992MembersActionsPatch)) });
export const parseEn1992MembersActionsMoved: NormWireReader<En1992MembersActionsMoved> = normWireObject<En1992MembersActionsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersActionsPatch: NormWireReader<En1992MembersActionsPatch> = normWireObject<En1992MembersActionsPatch>({ mK: normWireRequired(normWireNullable(normWireNumber)), nK: normWireRequired(normWireNullable(normWireNumber)), vK: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1992MembersActionsRemoved: NormWireReader<En1992MembersActionsRemoved> = normWireObject<En1992MembersActionsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersActionsRows: NormWireReader<En1992MembersActionsRows> = normWireObject<En1992MembersActionsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersActionsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersActionsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersActionsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersActionsModified))) });
export const parseEn1992MembersInserted: NormWireReader<En1992MembersInserted> = normWireObject<En1992MembersInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseRcMember) });
export const parseEn1992MembersLongitudinalInserted: NormWireReader<En1992MembersLongitudinalInserted> = normWireObject<En1992MembersLongitudinalInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseBarLayer) });
export const parseEn1992MembersLongitudinalModified: NormWireReader<En1992MembersLongitudinalModified> = normWireObject<En1992MembersLongitudinalModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992MembersLongitudinalPatch)) });
export const parseEn1992MembersLongitudinalMoved: NormWireReader<En1992MembersLongitudinalMoved> = normWireObject<En1992MembersLongitudinalMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersLongitudinalPatch: NormWireReader<En1992MembersLongitudinalPatch> = normWireObject<En1992MembersLongitudinalPatch>({ diameter: normWireRequired(normWireNullable(normWireNumber)), count: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))) });
export const parseEn1992MembersLongitudinalRemoved: NormWireReader<En1992MembersLongitudinalRemoved> = normWireObject<En1992MembersLongitudinalRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersLongitudinalRows: NormWireReader<En1992MembersLongitudinalRows> = normWireObject<En1992MembersLongitudinalRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersLongitudinalRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersLongitudinalInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersLongitudinalMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersLongitudinalModified))) });
export const parseEn1992MembersModified: NormWireReader<En1992MembersModified> = normWireObject<En1992MembersModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992MembersPatch)) });
export const parseEn1992MembersMoved: NormWireReader<En1992MembersMoved> = normWireObject<En1992MembersMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersPatch: NormWireReader<En1992MembersPatch> = normWireObject<En1992MembersPatch>({ exposure: normWireRequired(normWireNullable(parseExposureClass)), width: normWireRequired(normWireNullable(normWireNumber)), height: normWireRequired(normWireNullable(normWireNumber)), effectiveDepth: normWireRequired(normWireNullable(normWireNumber)), cover: normWireRequired(normWireNullable(normWireNumber)), span: normWireRequired(normWireNullable(normWireNumber)), fireRating: normWireRequired(normWireNullable(parseFireRating)), fireAxisDistance: normWireRequired(normWireNullable(normWireNumber)), stirrupsSpacing: normWireRequired(normWireNullable(normWireNumber)), longitudinal: normWireRequired(normWireNullable(normWireRef(() => parseEn1992MembersLongitudinalRows))), actions: normWireRequired(normWireNullable(normWireRef(() => parseEn1992MembersActionsRows))) });
export const parseEn1992MembersRemoved: NormWireReader<En1992MembersRemoved> = normWireObject<En1992MembersRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992MembersRows: NormWireReader<En1992MembersRows> = normWireObject<En1992MembersRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersModified))) });
export const parseEn1992PrestressSteelsInserted: NormWireReader<En1992PrestressSteelsInserted> = normWireObject<En1992PrestressSteelsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parsePrestressSteel) });
export const parseEn1992PrestressSteelsModified: NormWireReader<En1992PrestressSteelsModified> = normWireObject<En1992PrestressSteelsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992PrestressSteelsPatch)) });
export const parseEn1992PrestressSteelsMoved: NormWireReader<En1992PrestressSteelsMoved> = normWireObject<En1992PrestressSteelsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992PrestressSteelsPatch: NormWireReader<En1992PrestressSteelsPatch> = normWireObject<En1992PrestressSteelsPatch>({ name: normWireRequired(normWireNullable(normWireString)), fPk: normWireRequired(normWireNullable(normWireNumber)), fP01k: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1992PrestressSteelsRemoved: NormWireReader<En1992PrestressSteelsRemoved> = normWireObject<En1992PrestressSteelsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992PrestressSteelsRows: NormWireReader<En1992PrestressSteelsRows> = normWireObject<En1992PrestressSteelsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992PrestressSteelsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992PrestressSteelsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992PrestressSteelsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992PrestressSteelsModified))) });
export const parseEn1992ReinforcementGradesInserted: NormWireReader<En1992ReinforcementGradesInserted> = normWireObject<En1992ReinforcementGradesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseReinforcementGrade) });
export const parseEn1992ReinforcementGradesModified: NormWireReader<En1992ReinforcementGradesModified> = normWireObject<En1992ReinforcementGradesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1992ReinforcementGradesPatch)) });
export const parseEn1992ReinforcementGradesMoved: NormWireReader<En1992ReinforcementGradesMoved> = normWireObject<En1992ReinforcementGradesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992ReinforcementGradesPatch: NormWireReader<En1992ReinforcementGradesPatch> = normWireObject<En1992ReinforcementGradesPatch>({ fYk: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1992ReinforcementGradesRemoved: NormWireReader<En1992ReinforcementGradesRemoved> = normWireObject<En1992ReinforcementGradesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1992ReinforcementGradesRows: NormWireReader<En1992ReinforcementGradesRows> = normWireObject<En1992ReinforcementGradesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1992ReinforcementGradesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1992ReinforcementGradesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1992ReinforcementGradesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992ReinforcementGradesModified))) });
