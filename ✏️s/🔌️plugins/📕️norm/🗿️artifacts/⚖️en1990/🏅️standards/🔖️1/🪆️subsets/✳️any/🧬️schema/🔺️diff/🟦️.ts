/** 🔺️ `En1990Diff` wire twin: the sparse delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1990AccidentalAction, type En1990BridgeSls, type En1990Member, type En1990MemberEffect, type En1990PermanentAction, type En1990SeismicAction, type En1990VariableAction, parseEn1990AccidentalAction, parseEn1990BridgeSls, parseEn1990Member, parseEn1990MemberEffect, parseEn1990PermanentAction, parseEn1990SeismicAction, parseEn1990VariableAction } from "../📸️snapshot/🟦️.ts";

export type En1990RowOp = "Insert" | "Remove" | "Replace";
export interface En1990PermanentEdit { op: En1990RowOp; index: number; id: string; value: En1990PermanentAction | null; }
export interface En1990VariableEdit { op: En1990RowOp; index: number; id: string; value: En1990VariableAction | null; }
export interface En1990AccidentalEdit { op: En1990RowOp; index: number; id: string; value: En1990AccidentalAction | null; }
export interface En1990SeismicEdit { op: En1990RowOp; index: number; id: string; value: En1990SeismicAction | null; }
export interface En1990MemberEdit { op: En1990RowOp; index: number; id: string; value: En1990Member | null; }
export interface En1990BridgeSlsEdit { op: En1990RowOp; index: number; id: string; value: En1990BridgeSls | null; }
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
  permanents: En1990PermanentEdit[];
  /** @state artifact */
  variables: En1990VariableEdit[];
  /** @state artifact */
  accidentals: En1990AccidentalEdit[];
  /** @state artifact */
  seismics: En1990SeismicEdit[];
  /** @state artifact */
  members: En1990MemberEdit[];
  /** @state artifact */
  bridgeSls: En1990BridgeSlsEdit[];
  /** @state artifact */
  effects: En1990EffectEdit[];
}

export const parseEn1990RowOp: NormWireReader<En1990RowOp> = normWireLiteral("Insert", "Remove", "Replace");
export const parseEn1990PermanentEdit: NormWireReader<En1990PermanentEdit> = normWireObject<En1990PermanentEdit>({ op: normWireRequired(parseEn1990RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1990PermanentAction)) });
export const parseEn1990VariableEdit: NormWireReader<En1990VariableEdit> = normWireObject<En1990VariableEdit>({ op: normWireRequired(parseEn1990RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1990VariableAction)) });
export const parseEn1990AccidentalEdit: NormWireReader<En1990AccidentalEdit> = normWireObject<En1990AccidentalEdit>({ op: normWireRequired(parseEn1990RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1990AccidentalAction)) });
export const parseEn1990SeismicEdit: NormWireReader<En1990SeismicEdit> = normWireObject<En1990SeismicEdit>({ op: normWireRequired(parseEn1990RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1990SeismicAction)) });
export const parseEn1990MemberEdit: NormWireReader<En1990MemberEdit> = normWireObject<En1990MemberEdit>({ op: normWireRequired(parseEn1990RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1990Member)) });
export const parseEn1990BridgeSlsEdit: NormWireReader<En1990BridgeSlsEdit> = normWireObject<En1990BridgeSlsEdit>({ op: normWireRequired(parseEn1990RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1990BridgeSls)) });
export const parseEn1990EffectEdit: NormWireReader<En1990EffectEdit> = normWireObject<En1990EffectEdit>({ op: normWireRequired(parseEn1990RowOp), index: normWireRequired(normWireInteger), id: normWireRequired(normWireString), value: normWireRequired(normWireNullable(parseEn1990MemberEffect)) });
export const parseEn1990Diff: NormWireReader<En1990Diff> = normWireObject<En1990Diff>({ annex: normWireDefault(normWireNullable(normWireLiteral("En", "De")), () => null), projectId: normWireDefault(normWireNullable(normWireString), () => null), structureKind: normWireDefault(normWireNullable(normWireString), () => null), altitudeM: normWireDefault(normWireNullable(normWireNumber), () => null), consequenceClass: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), reliabilityClass: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), designWorkingLifeCategory: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), designWorkingLifeYears: normWireDefault(normWireNullable(normWireNumber), () => null), referencePeriodYears: normWireDefault(normWireNullable(normWireNumber), () => null), supervisionLevel: normWireDefault(normWireNullable(normWireString), () => null), inspectionLevel: normWireDefault(normWireNullable(normWireString), () => null), kFiDeclared: normWireDefault(normWireNullable(normWireNumber), () => null), betaComputed: normWireDefault(normWireNullable(normWireNumber), () => null), permanents: normWireDefault(normWireArray(parseEn1990PermanentEdit), () => []), variables: normWireDefault(normWireArray(parseEn1990VariableEdit), () => []), accidentals: normWireDefault(normWireArray(parseEn1990AccidentalEdit), () => []), seismics: normWireDefault(normWireArray(parseEn1990SeismicEdit), () => []), members: normWireDefault(normWireArray(parseEn1990MemberEdit), () => []), bridgeSls: normWireDefault(normWireArray(parseEn1990BridgeSlsEdit), () => []), effects: normWireDefault(normWireArray(parseEn1990EffectEdit), () => []) });
