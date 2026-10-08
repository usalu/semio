/** 🔺️ `En1995Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireDefault, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type CharacteristicAction, type ConnectionAction, type MemberRole, parseCharacteristicAction, parseConnectionAction, parseMemberRole, parseSupportType, parseTimberConnection, parseTimberMember, type SupportType, type TimberConnection, type TimberMember } from "../📸️snapshot/🟦️.ts";

export interface En1995Diff {
  /** @state artifact */
  annex: ("En" | "De") | null;
  /** @state artifact */
  members?: En1995MemberDelta;
  /** @state artifact */
  connections?: En1995ConnectionDelta;
}

export interface En1995MemberActionPatch {
  kind: string | null;
  category: string | null;
  loadDuration: string | null;
  qLineNPerM: number | null;
  fPointN: number | null;
  mKNm: number | null;
  vKN: number | null;
  nKN: number | null;
  nTKN: number | null;
  fC90KN: number | null;
}

export interface En1995MemberActionRemoval {
  id: string;
  index: number;
}

export interface En1995MemberActionInsertion {
  index: number;
  row: CharacteristicAction;
}

export interface En1995MemberActionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995MemberActionModification {
  id: string;
  patch: En1995MemberActionPatch;
}

export interface En1995MemberActionDelta {
  removed: En1995MemberActionRemoval[];
  inserted: En1995MemberActionInsertion[];
  moved: En1995MemberActionRelocation[];
  modified: En1995MemberActionModification[];
}

export interface En1995ConnectionActionPatch {
  kind: string | null;
  loadDuration: string | null;
  fKN: number | null;
}

export interface En1995ConnectionActionRemoval {
  id: string;
  index: number;
}

export interface En1995ConnectionActionInsertion {
  index: number;
  row: ConnectionAction;
}

export interface En1995ConnectionActionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995ConnectionActionModification {
  id: string;
  patch: En1995ConnectionActionPatch;
}

export interface En1995ConnectionActionDelta {
  removed: En1995ConnectionActionRemoval[];
  inserted: En1995ConnectionActionInsertion[];
  moved: En1995ConnectionActionRelocation[];
  modified: En1995ConnectionActionModification[];
}

export interface En1995MemberPatch {
  labelEn: string | null;
  labelDe: string | null;
  role: MemberRole | null;
  strengthClass: string | null;
  serviceClass: number | null;
  support: SupportType | null;
  bM: number | null;
  hM: number | null;
  spanM: number | null;
  supportLengthM: number | null;
  bearingLengthM: number | null;
  bucklingLengthYM: number | null;
  bucklingLengthZM: number | null;
  lateralRestraintSpacingM: number | null;
  notchDepthM: number | null;
  notchDistanceM: number | null;
  mCritNm: number | null;
  massKgPerM: number | null;
  massKgPerM2: number | null;
  dampingXi: number | null;
  fireDurationS: number | null;
  bridgeNObs: number | null;
  bridgeTLYears: number | null;
  bridgeBeta: number | null;
  bridgeA: number | null;
  bridgeB: number | null;
  bridgeCrowdPerM2: number | null;
  actions?: En1995MemberActionDelta;
}

export interface En1995MemberRemoval {
  id: string;
  index: number;
}

export interface En1995MemberInsertion {
  index: number;
  row: TimberMember;
}

export interface En1995MemberRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995MemberModification {
  id: string;
  patch: En1995MemberPatch;
}

export interface En1995MemberDelta {
  removed: En1995MemberRemoval[];
  inserted: En1995MemberInsertion[];
  moved: En1995MemberRelocation[];
  modified: En1995MemberModification[];
}

export interface En1995ConnectionPatch {
  labelEn: string | null;
  labelDe: string | null;
  fastenerType: string | null;
  strengthClass: string | null;
  serviceClass: number | null;
  diameterM: number | null;
  number: number | null;
  rows: number | null;
  spacingM: number | null;
  edgeDistanceM: number | null;
  endDistanceM: number | null;
  t1M: number | null;
  t2M: number | null;
  steelPlate: boolean | null;
  steelPlateThicknessM: number | null;
  shearPlanes: number | null;
  fUK: number | null;
  actions?: En1995ConnectionActionDelta;
}

export interface En1995ConnectionRemoval {
  id: string;
  index: number;
}

export interface En1995ConnectionInsertion {
  index: number;
  row: TimberConnection;
}

export interface En1995ConnectionRelocation {
  id: string;
  from: number;
  to: number;
}

export interface En1995ConnectionModification {
  id: string;
  patch: En1995ConnectionPatch;
}

export interface En1995ConnectionDelta {
  removed: En1995ConnectionRemoval[];
  inserted: En1995ConnectionInsertion[];
  moved: En1995ConnectionRelocation[];
  modified: En1995ConnectionModification[];
}

export const parseEn1995Diff: NormWireReader<En1995Diff> = normWireObject<En1995Diff>({ annex: normWireDefault(normWireNullable(normWireLiteral("En", "De")), () => null), members: normWireOptional(normWireRef(() => parseEn1995MemberDelta)), connections: normWireOptional(normWireRef(() => parseEn1995ConnectionDelta)) });
export const parseEn1995MemberActionPatch: NormWireReader<En1995MemberActionPatch> = normWireObject<En1995MemberActionPatch>({ kind: normWireDefault(normWireNullable(normWireString), () => null), category: normWireDefault(normWireNullable(normWireString), () => null), loadDuration: normWireDefault(normWireNullable(normWireString), () => null), qLineNPerM: normWireDefault(normWireNullable(normWireNumber), () => null), fPointN: normWireDefault(normWireNullable(normWireNumber), () => null), mKNm: normWireDefault(normWireNullable(normWireNumber), () => null), vKN: normWireDefault(normWireNullable(normWireNumber), () => null), nKN: normWireDefault(normWireNullable(normWireNumber), () => null), nTKN: normWireDefault(normWireNullable(normWireNumber), () => null), fC90KN: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1995MemberActionRemoval: NormWireReader<En1995MemberActionRemoval> = normWireObject<En1995MemberActionRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995MemberActionInsertion: NormWireReader<En1995MemberActionInsertion> = normWireObject<En1995MemberActionInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseCharacteristicAction) });
export const parseEn1995MemberActionRelocation: NormWireReader<En1995MemberActionRelocation> = normWireObject<En1995MemberActionRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995MemberActionModification: NormWireReader<En1995MemberActionModification> = normWireObject<En1995MemberActionModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995MemberActionPatch)) });
export const parseEn1995MemberActionDelta: NormWireReader<En1995MemberActionDelta> = normWireObject<En1995MemberActionDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995MemberActionRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995MemberActionInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995MemberActionRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995MemberActionModification))) });
export const parseEn1995ConnectionActionPatch: NormWireReader<En1995ConnectionActionPatch> = normWireObject<En1995ConnectionActionPatch>({ kind: normWireDefault(normWireNullable(normWireString), () => null), loadDuration: normWireDefault(normWireNullable(normWireString), () => null), fKN: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1995ConnectionActionRemoval: NormWireReader<En1995ConnectionActionRemoval> = normWireObject<En1995ConnectionActionRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995ConnectionActionInsertion: NormWireReader<En1995ConnectionActionInsertion> = normWireObject<En1995ConnectionActionInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseConnectionAction) });
export const parseEn1995ConnectionActionRelocation: NormWireReader<En1995ConnectionActionRelocation> = normWireObject<En1995ConnectionActionRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995ConnectionActionModification: NormWireReader<En1995ConnectionActionModification> = normWireObject<En1995ConnectionActionModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995ConnectionActionPatch)) });
export const parseEn1995ConnectionActionDelta: NormWireReader<En1995ConnectionActionDelta> = normWireObject<En1995ConnectionActionDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995ConnectionActionRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995ConnectionActionInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995ConnectionActionRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995ConnectionActionModification))) });
export const parseEn1995MemberPatch: NormWireReader<En1995MemberPatch> = normWireObject<En1995MemberPatch>({ labelEn: normWireDefault(normWireNullable(normWireString), () => null), labelDe: normWireDefault(normWireNullable(normWireString), () => null), role: normWireDefault(normWireNullable(parseMemberRole), () => null), strengthClass: normWireDefault(normWireNullable(normWireString), () => null), serviceClass: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), support: normWireDefault(normWireNullable(parseSupportType), () => null), bM: normWireDefault(normWireNullable(normWireNumber), () => null), hM: normWireDefault(normWireNullable(normWireNumber), () => null), spanM: normWireDefault(normWireNullable(normWireNumber), () => null), supportLengthM: normWireDefault(normWireNullable(normWireNumber), () => null), bearingLengthM: normWireDefault(normWireNullable(normWireNumber), () => null), bucklingLengthYM: normWireDefault(normWireNullable(normWireNumber), () => null), bucklingLengthZM: normWireDefault(normWireNullable(normWireNumber), () => null), lateralRestraintSpacingM: normWireDefault(normWireNullable(normWireNumber), () => null), notchDepthM: normWireDefault(normWireNullable(normWireNumber), () => null), notchDistanceM: normWireDefault(normWireNullable(normWireNumber), () => null), mCritNm: normWireDefault(normWireNullable(normWireNumber), () => null), massKgPerM: normWireDefault(normWireNullable(normWireNumber), () => null), massKgPerM2: normWireDefault(normWireNullable(normWireNumber), () => null), dampingXi: normWireDefault(normWireNullable(normWireNumber), () => null), fireDurationS: normWireDefault(normWireNullable(normWireNumber), () => null), bridgeNObs: normWireDefault(normWireNullable(normWireNumber), () => null), bridgeTLYears: normWireDefault(normWireNullable(normWireNumber), () => null), bridgeBeta: normWireDefault(normWireNullable(normWireNumber), () => null), bridgeA: normWireDefault(normWireNullable(normWireNumber), () => null), bridgeB: normWireDefault(normWireNullable(normWireNumber), () => null), bridgeCrowdPerM2: normWireDefault(normWireNullable(normWireNumber), () => null), actions: normWireOptional(normWireRef(() => parseEn1995MemberActionDelta)) });
export const parseEn1995MemberRemoval: NormWireReader<En1995MemberRemoval> = normWireObject<En1995MemberRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995MemberInsertion: NormWireReader<En1995MemberInsertion> = normWireObject<En1995MemberInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseTimberMember) });
export const parseEn1995MemberRelocation: NormWireReader<En1995MemberRelocation> = normWireObject<En1995MemberRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995MemberModification: NormWireReader<En1995MemberModification> = normWireObject<En1995MemberModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995MemberPatch)) });
export const parseEn1995MemberDelta: NormWireReader<En1995MemberDelta> = normWireObject<En1995MemberDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995MemberRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995MemberInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995MemberRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995MemberModification))) });
export const parseEn1995ConnectionPatch: NormWireReader<En1995ConnectionPatch> = normWireObject<En1995ConnectionPatch>({ labelEn: normWireDefault(normWireNullable(normWireString), () => null), labelDe: normWireDefault(normWireNullable(normWireString), () => null), fastenerType: normWireDefault(normWireNullable(normWireString), () => null), strengthClass: normWireDefault(normWireNullable(normWireString), () => null), serviceClass: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), diameterM: normWireDefault(normWireNullable(normWireNumber), () => null), number: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), () => null), rows: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), () => null), spacingM: normWireDefault(normWireNullable(normWireNumber), () => null), edgeDistanceM: normWireDefault(normWireNullable(normWireNumber), () => null), endDistanceM: normWireDefault(normWireNullable(normWireNumber), () => null), t1M: normWireDefault(normWireNullable(normWireNumber), () => null), t2M: normWireDefault(normWireNullable(normWireNumber), () => null), steelPlate: normWireDefault(normWireNullable(normWireBoolean), () => null), steelPlateThicknessM: normWireDefault(normWireNullable(normWireNumber), () => null), shearPlanes: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), () => null), fUK: normWireDefault(normWireNullable(normWireNumber), () => null), actions: normWireOptional(normWireRef(() => parseEn1995ConnectionActionDelta)) });
export const parseEn1995ConnectionRemoval: NormWireReader<En1995ConnectionRemoval> = normWireObject<En1995ConnectionRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995ConnectionInsertion: NormWireReader<En1995ConnectionInsertion> = normWireObject<En1995ConnectionInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseTimberConnection) });
export const parseEn1995ConnectionRelocation: NormWireReader<En1995ConnectionRelocation> = normWireObject<En1995ConnectionRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1995ConnectionModification: NormWireReader<En1995ConnectionModification> = normWireObject<En1995ConnectionModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1995ConnectionPatch)) });
export const parseEn1995ConnectionDelta: NormWireReader<En1995ConnectionDelta> = normWireObject<En1995ConnectionDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1995ConnectionRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1995ConnectionInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1995ConnectionRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1995ConnectionModification))) });
