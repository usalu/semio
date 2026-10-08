/** 🔺️ `En1992Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireInteger, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type Anchor, type AnnexChoice, type ConcreteGrade, type ExposureClass, type FireRating, parseAnchor, parseAnnexChoice, parseConcreteGrade, parseExposureClass, parseFireRating, parsePrestressSteel, parseRcMember, parseReinforcementGrade, type PrestressSteel, type RcMember, type ReinforcementGrade } from "../📸️snapshot/🟦️.ts";

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

export interface En1992AnchorsPatch {
  id: string;
  hEf: number | null;
  aS: number | null;
}

export interface En1992AnchorsRows {
  added: Anchor[];
  removed: string[];
  modified: En1992AnchorsPatch[];
  order: string[] | null;
}

export interface En1992ConcreteGradesPatch {
  id: string;
  fCk: number | null;
}

export interface En1992ConcreteGradesRows {
  added: ConcreteGrade[];
  removed: string[];
  modified: En1992ConcreteGradesPatch[];
  order: string[] | null;
}

export interface En1992MembersActionsPatch {
  id: string;
  mK: number | null;
  nK: number | null;
  vK: number | null;
}

export interface En1992MembersActionsRows {
  modified: En1992MembersActionsPatch[];
}

export interface En1992MembersLongitudinalPatch {
  id: string;
  diameter: number | null;
  count: number | null;
}

export interface En1992MembersLongitudinalRows {
  modified: En1992MembersLongitudinalPatch[];
}

export interface En1992MembersPatch {
  id: string;
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

export interface En1992MembersRows {
  added: RcMember[];
  removed: string[];
  modified: En1992MembersPatch[];
  order: string[] | null;
}

export interface En1992PrestressSteelsRows {
  added: PrestressSteel[];
  removed: string[];
  order: string[] | null;
}

export interface En1992ReinforcementGradesPatch {
  id: string;
  fYk: number | null;
}

export interface En1992ReinforcementGradesRows {
  added: ReinforcementGrade[];
  removed: string[];
  modified: En1992ReinforcementGradesPatch[];
  order: string[] | null;
}

export const parseEn1992Diff: NormWireReader<En1992Diff> = normWireObject<En1992Diff>({ annex: normWireDefault(normWireNullable(parseAnnexChoice), () => null), title: normWireDefault(normWireNullable(normWireString), () => null), designWorkingLifeYears: normWireDefault(normWireNullable(normWireNumber), () => null), deltaCDev: normWireDefault(normWireNullable(normWireNumber), () => null), cementType: normWireDefault(normWireNullable(normWireString), () => null), concreteGrades: normWireDefault(normWireNullable(normWireRef(() => parseEn1992ConcreteGradesRows)), () => null), reinforcementGrades: normWireDefault(normWireNullable(normWireRef(() => parseEn1992ReinforcementGradesRows)), () => null), prestressSteels: normWireDefault(normWireNullable(normWireRef(() => parseEn1992PrestressSteelsRows)), () => null), members: normWireDefault(normWireNullable(normWireRef(() => parseEn1992MembersRows)), () => null), anchors: normWireDefault(normWireNullable(normWireRef(() => parseEn1992AnchorsRows)), () => null) });
export const parseEn1992AnchorsPatch: NormWireReader<En1992AnchorsPatch> = normWireObject<En1992AnchorsPatch>({ id: normWireRequired(normWireString), hEf: normWireRequired(normWireNullable(normWireNumber)), aS: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1992AnchorsRows: NormWireReader<En1992AnchorsRows> = normWireObject<En1992AnchorsRows>({ added: normWireRequired(normWireArray(parseAnchor)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992AnchorsPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseEn1992ConcreteGradesPatch: NormWireReader<En1992ConcreteGradesPatch> = normWireObject<En1992ConcreteGradesPatch>({ id: normWireRequired(normWireString), fCk: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1992ConcreteGradesRows: NormWireReader<En1992ConcreteGradesRows> = normWireObject<En1992ConcreteGradesRows>({ added: normWireRequired(normWireArray(parseConcreteGrade)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992ConcreteGradesPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseEn1992MembersActionsPatch: NormWireReader<En1992MembersActionsPatch> = normWireObject<En1992MembersActionsPatch>({ id: normWireRequired(normWireString), mK: normWireRequired(normWireNullable(normWireNumber)), nK: normWireRequired(normWireNullable(normWireNumber)), vK: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1992MembersActionsRows: NormWireReader<En1992MembersActionsRows> = normWireObject<En1992MembersActionsRows>({ modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersActionsPatch))) });
export const parseEn1992MembersLongitudinalPatch: NormWireReader<En1992MembersLongitudinalPatch> = normWireObject<En1992MembersLongitudinalPatch>({ id: normWireRequired(normWireString), diameter: normWireRequired(normWireNullable(normWireNumber)), count: normWireRequired(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))) });
export const parseEn1992MembersLongitudinalRows: NormWireReader<En1992MembersLongitudinalRows> = normWireObject<En1992MembersLongitudinalRows>({ modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersLongitudinalPatch))) });
export const parseEn1992MembersPatch: NormWireReader<En1992MembersPatch> = normWireObject<En1992MembersPatch>({ id: normWireRequired(normWireString), exposure: normWireRequired(normWireNullable(parseExposureClass)), width: normWireRequired(normWireNullable(normWireNumber)), height: normWireRequired(normWireNullable(normWireNumber)), effectiveDepth: normWireRequired(normWireNullable(normWireNumber)), cover: normWireRequired(normWireNullable(normWireNumber)), span: normWireRequired(normWireNullable(normWireNumber)), fireRating: normWireRequired(normWireNullable(parseFireRating)), fireAxisDistance: normWireRequired(normWireNullable(normWireNumber)), stirrupsSpacing: normWireRequired(normWireNullable(normWireNumber)), longitudinal: normWireRequired(normWireNullable(normWireRef(() => parseEn1992MembersLongitudinalRows))), actions: normWireRequired(normWireNullable(normWireRef(() => parseEn1992MembersActionsRows))) });
export const parseEn1992MembersRows: NormWireReader<En1992MembersRows> = normWireObject<En1992MembersRows>({ added: normWireRequired(normWireArray(parseRcMember)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992MembersPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseEn1992PrestressSteelsRows: NormWireReader<En1992PrestressSteelsRows> = normWireObject<En1992PrestressSteelsRows>({ added: normWireRequired(normWireArray(parsePrestressSteel)), removed: normWireRequired(normWireArray(normWireString)), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
export const parseEn1992ReinforcementGradesPatch: NormWireReader<En1992ReinforcementGradesPatch> = normWireObject<En1992ReinforcementGradesPatch>({ id: normWireRequired(normWireString), fYk: normWireRequired(normWireNullable(normWireNumber)) });
export const parseEn1992ReinforcementGradesRows: NormWireReader<En1992ReinforcementGradesRows> = normWireObject<En1992ReinforcementGradesRows>({ added: normWireRequired(normWireArray(parseReinforcementGrade)), removed: normWireRequired(normWireArray(normWireString)), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1992ReinforcementGradesPatch))), order: normWireRequired(normWireNullable(normWireArray(normWireString))) });
