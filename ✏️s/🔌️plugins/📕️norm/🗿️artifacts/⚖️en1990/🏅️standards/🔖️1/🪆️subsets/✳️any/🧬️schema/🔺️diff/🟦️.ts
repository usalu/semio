/** 🔺️ `En1990Diff` wire twin: the sparse delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1990AccidentalAction, type En1990BridgeSls, type En1990ImportanceClass, type En1990Member, type En1990MemberEffect, type En1990PermanentAction, type En1990SeismicAction, type En1990VariableAction, parseEn1990AccidentalAction, parseEn1990BridgeSls, parseEn1990ImportanceClass, parseEn1990Member, parseEn1990MemberEffect, parseEn1990PermanentAction, parseEn1990SeismicAction, parseEn1990VariableAction } from "../📸️snapshot/🟦️.ts";

export interface En1990PermanentPatch { kind: (string) | null; gk: (number) | null; }
export interface En1990PermanentAddition { after: string | null; row: En1990PermanentAction; }
export interface En1990PermanentModification { key: string; patch: En1990PermanentPatch; }
export interface En1990PermanentDelta { removed: string[]; added: En1990PermanentAddition[]; modified: En1990PermanentModification[]; }
export interface En1990VariablePatch { category: (string) | null; qk: (number) | null; }
export interface En1990VariableAddition { after: string | null; row: En1990VariableAction; }
export interface En1990VariableModification { key: string; patch: En1990VariablePatch; }
export interface En1990VariableDelta { removed: string[]; added: En1990VariableAddition[]; modified: En1990VariableModification[]; }
export interface En1990AccidentalPatch { ad: (number) | null; }
export interface En1990AccidentalAddition { after: string | null; row: En1990AccidentalAction; }
export interface En1990AccidentalModification { key: string; patch: En1990AccidentalPatch; }
export interface En1990AccidentalDelta { removed: string[]; added: En1990AccidentalAddition[]; modified: En1990AccidentalModification[]; }
export interface En1990SeismicPatch { aEk: (number) | null; importanceClass: (En1990ImportanceClass) | null; }
export interface En1990SeismicAddition { after: string | null; row: En1990SeismicAction; }
export interface En1990SeismicModification { key: string; patch: En1990SeismicPatch; }
export interface En1990SeismicDelta { removed: string[]; added: En1990SeismicAddition[]; modified: En1990SeismicModification[]; }
export interface En1990MemberPatch { labelEn: (string) | null; labelDe: (string) | null; rdStr: (number) | null; rdGeo: (number) | null; rdEquStab: (number) | null; rdEquDestab: (number) | null; rdFat: (number) | null; span: (number) | null; deflectionW: (number) | null; deflectionLimitRatio: (number) | null; vibrationFrequency: (number) | null; vibrationFrequencyMin: (number) | null; }
export interface En1990MemberAddition { after: string | null; row: En1990Member; }
export interface En1990MemberModification { key: string; patch: En1990MemberPatch; }
export interface En1990MemberDelta { removed: string[]; added: En1990MemberAddition[]; modified: En1990MemberModification[]; }
export interface En1990BridgeSlsPatch { memberId: (string) | null; deckAcceleration: (number) | null; deckAccelerationLimit: (number) | null; deckTwist: (number) | null; deckTwistLimit: (number) | null; bridgeDeflection: (number) | null; bridgeDeflectionLimit: (number) | null; }
export interface En1990BridgeSlsAddition { after: string | null; row: En1990BridgeSls; }
export interface En1990BridgeSlsModification { key: string; patch: En1990BridgeSlsPatch; }
export interface En1990BridgeSlsDelta { removed: string[]; added: En1990BridgeSlsAddition[]; modified: En1990BridgeSlsModification[]; }
export type En1990RowOp = "Insert" | "Remove" | "Replace";
export interface En1990EffectEdit { op: En1990RowOp; index: number; id: string; value: En1990MemberEffect | null; }

export interface En1990Diff {
  /** @state artifact */
  annex: ("En" | "De") | null;
  /** @state artifact */
  projectId: string | null;
  /** @state artifact */
  structureKind: string | null;
  /** @state artifact */
  altitudeM: number | null;
  /** @state artifact */
  consequenceClass: number | null;
  /** @state artifact */
  reliabilityClass: number | null;
  /** @state artifact */
  designWorkingLifeCategory: number | null;
  /** @state artifact */
  designWorkingLifeYears: number | null;
  /** @state artifact */
  referencePeriodYears: number | null;
  /** @state artifact */
  supervisionLevel: string | null;
  /** @state artifact */
  inspectionLevel: string | null;
  /** @state artifact */
  kFiDeclared: number | null;
  /** @state artifact */
  betaComputed: number | null;
  /** @state artifact */
  permanents: En1990PermanentDelta;
  /** @state artifact */
  variables: En1990VariableDelta;
  /** @state artifact */
  accidentals: En1990AccidentalDelta;
  /** @state artifact */
  seismics: En1990SeismicDelta;
  /** @state artifact */
  members: En1990MemberDelta;
  /** @state artifact */
  bridgeSls: En1990BridgeSlsDelta;
  /** @state artifact */
  effects: En1990EffectEdit[];
}

export const parseEn1990PermanentPatch: NormWireReader<En1990PermanentPatch> = normWireObject<En1990PermanentPatch>({ kind: normWireDefault(normWireNullable(normWireString), () => null), gk: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1990PermanentAddition: NormWireReader<En1990PermanentAddition> = normWireObject<En1990PermanentAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseEn1990PermanentAction) });
export const parseEn1990PermanentModification: NormWireReader<En1990PermanentModification> = normWireObject<En1990PermanentModification>({ key: normWireRequired(normWireString), patch: normWireRequired(parseEn1990PermanentPatch) });
export const parseEn1990PermanentDelta: NormWireReader<En1990PermanentDelta> = normWireObject<En1990PermanentDelta>({ removed: normWireDefault(normWireArray(normWireString), () => []), added: normWireDefault(normWireArray(parseEn1990PermanentAddition), () => []), modified: normWireDefault(normWireArray(parseEn1990PermanentModification), () => []) });
export const parseEn1990VariablePatch: NormWireReader<En1990VariablePatch> = normWireObject<En1990VariablePatch>({ category: normWireDefault(normWireNullable(normWireString), () => null), qk: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1990VariableAddition: NormWireReader<En1990VariableAddition> = normWireObject<En1990VariableAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseEn1990VariableAction) });
export const parseEn1990VariableModification: NormWireReader<En1990VariableModification> = normWireObject<En1990VariableModification>({ key: normWireRequired(normWireString), patch: normWireRequired(parseEn1990VariablePatch) });
export const parseEn1990VariableDelta: NormWireReader<En1990VariableDelta> = normWireObject<En1990VariableDelta>({ removed: normWireDefault(normWireArray(normWireString), () => []), added: normWireDefault(normWireArray(parseEn1990VariableAddition), () => []), modified: normWireDefault(normWireArray(parseEn1990VariableModification), () => []) });
export const parseEn1990AccidentalPatch: NormWireReader<En1990AccidentalPatch> = normWireObject<En1990AccidentalPatch>({ ad: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1990AccidentalAddition: NormWireReader<En1990AccidentalAddition> = normWireObject<En1990AccidentalAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseEn1990AccidentalAction) });
export const parseEn1990AccidentalModification: NormWireReader<En1990AccidentalModification> = normWireObject<En1990AccidentalModification>({ key: normWireRequired(normWireString), patch: normWireRequired(parseEn1990AccidentalPatch) });
export const parseEn1990AccidentalDelta: NormWireReader<En1990AccidentalDelta> = normWireObject<En1990AccidentalDelta>({ removed: normWireDefault(normWireArray(normWireString), () => []), added: normWireDefault(normWireArray(parseEn1990AccidentalAddition), () => []), modified: normWireDefault(normWireArray(parseEn1990AccidentalModification), () => []) });
export const parseEn1990SeismicPatch: NormWireReader<En1990SeismicPatch> = normWireObject<En1990SeismicPatch>({ aEk: normWireDefault(normWireNullable(normWireNumber), () => null), importanceClass: normWireDefault(normWireNullable(parseEn1990ImportanceClass), () => null) });
export const parseEn1990SeismicAddition: NormWireReader<En1990SeismicAddition> = normWireObject<En1990SeismicAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseEn1990SeismicAction) });
export const parseEn1990SeismicModification: NormWireReader<En1990SeismicModification> = normWireObject<En1990SeismicModification>({ key: normWireRequired(normWireString), patch: normWireRequired(parseEn1990SeismicPatch) });
export const parseEn1990SeismicDelta: NormWireReader<En1990SeismicDelta> = normWireObject<En1990SeismicDelta>({ removed: normWireDefault(normWireArray(normWireString), () => []), added: normWireDefault(normWireArray(parseEn1990SeismicAddition), () => []), modified: normWireDefault(normWireArray(parseEn1990SeismicModification), () => []) });
export const parseEn1990MemberPatch: NormWireReader<En1990MemberPatch> = normWireObject<En1990MemberPatch>({ labelEn: normWireDefault(normWireNullable(normWireString), () => null), labelDe: normWireDefault(normWireNullable(normWireString), () => null), rdStr: normWireDefault(normWireNullable(normWireNumber), () => null), rdGeo: normWireDefault(normWireNullable(normWireNumber), () => null), rdEquStab: normWireDefault(normWireNullable(normWireNumber), () => null), rdEquDestab: normWireDefault(normWireNullable(normWireNumber), () => null), rdFat: normWireDefault(normWireNullable(normWireNumber), () => null), span: normWireDefault(normWireNullable(normWireNumber), () => null), deflectionW: normWireDefault(normWireNullable(normWireNumber), () => null), deflectionLimitRatio: normWireDefault(normWireNullable(normWireNumber), () => null), vibrationFrequency: normWireDefault(normWireNullable(normWireNumber), () => null), vibrationFrequencyMin: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1990MemberAddition: NormWireReader<En1990MemberAddition> = normWireObject<En1990MemberAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseEn1990Member) });
export const parseEn1990MemberModification: NormWireReader<En1990MemberModification> = normWireObject<En1990MemberModification>({ key: normWireRequired(normWireString), patch: normWireRequired(parseEn1990MemberPatch) });
export const parseEn1990MemberDelta: NormWireReader<En1990MemberDelta> = normWireObject<En1990MemberDelta>({ removed: normWireDefault(normWireArray(normWireString), () => []), added: normWireDefault(normWireArray(parseEn1990MemberAddition), () => []), modified: normWireDefault(normWireArray(parseEn1990MemberModification), () => []) });
export const parseEn1990BridgeSlsPatch: NormWireReader<En1990BridgeSlsPatch> = normWireObject<En1990BridgeSlsPatch>({ memberId: normWireDefault(normWireNullable(normWireString), () => null), deckAcceleration: normWireDefault(normWireNullable(normWireNumber), () => null), deckAccelerationLimit: normWireDefault(normWireNullable(normWireNumber), () => null), deckTwist: normWireDefault(normWireNullable(normWireNumber), () => null), deckTwistLimit: normWireDefault(normWireNullable(normWireNumber), () => null), bridgeDeflection: normWireDefault(normWireNullable(normWireNumber), () => null), bridgeDeflectionLimit: normWireDefault(normWireNullable(normWireNumber), () => null) });
export const parseEn1990BridgeSlsAddition: NormWireReader<En1990BridgeSlsAddition> = normWireObject<En1990BridgeSlsAddition>({ after: normWireRequired(normWireNullable(normWireString)), row: normWireRequired(parseEn1990BridgeSls) });
export const parseEn1990BridgeSlsModification: NormWireReader<En1990BridgeSlsModification> = normWireObject<En1990BridgeSlsModification>({ key: normWireRequired(normWireString), patch: normWireRequired(parseEn1990BridgeSlsPatch) });
export const parseEn1990BridgeSlsDelta: NormWireReader<En1990BridgeSlsDelta> = normWireObject<En1990BridgeSlsDelta>({ removed: normWireDefault(normWireArray(normWireString), () => []), added: normWireDefault(normWireArray(parseEn1990BridgeSlsAddition), () => []), modified: normWireDefault(normWireArray(parseEn1990BridgeSlsModification), () => []) });
export const parseEn1990RowOp: NormWireReader<En1990RowOp> = normWireLiteral("Insert", "Remove", "Replace");
export const parseEn1990EffectEdit: NormWireReader<En1990EffectEdit> = normWireObject<En1990EffectEdit>({ op: normWireRequired(parseEn1990RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1990MemberEffect)) });
export const parseEn1990Diff: NormWireReader<En1990Diff> = normWireObject<En1990Diff>({ annex: normWireDefault(normWireNullable(normWireLiteral("En", "De")), () => null), projectId: normWireDefault(normWireNullable(normWireString), () => null), structureKind: normWireDefault(normWireNullable(normWireString), () => null), altitudeM: normWireDefault(normWireNullable(normWireNumber), () => null), consequenceClass: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), reliabilityClass: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), designWorkingLifeCategory: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), designWorkingLifeYears: normWireDefault(normWireNullable(normWireNumber), () => null), referencePeriodYears: normWireDefault(normWireNullable(normWireNumber), () => null), supervisionLevel: normWireDefault(normWireNullable(normWireString), () => null), inspectionLevel: normWireDefault(normWireNullable(normWireString), () => null), kFiDeclared: normWireDefault(normWireNullable(normWireNumber), () => null), betaComputed: normWireDefault(normWireNullable(normWireNumber), () => null), permanents: normWireDefault(normWireRef(() => parseEn1990PermanentDelta), () => ({ removed: [], added: [], modified: [] })), variables: normWireDefault(normWireRef(() => parseEn1990VariableDelta), () => ({ removed: [], added: [], modified: [] })), accidentals: normWireDefault(normWireRef(() => parseEn1990AccidentalDelta), () => ({ removed: [], added: [], modified: [] })), seismics: normWireDefault(normWireRef(() => parseEn1990SeismicDelta), () => ({ removed: [], added: [], modified: [] })), members: normWireDefault(normWireRef(() => parseEn1990MemberDelta), () => ({ removed: [], added: [], modified: [] })), bridgeSls: normWireDefault(normWireRef(() => parseEn1990BridgeSlsDelta), () => ({ removed: [], added: [], modified: [] })), effects: normWireDefault(normWireArray(parseEn1990EffectEdit), () => []) });
