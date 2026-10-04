/** 🔺️ `En1990Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireInteger, normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type En1990AccidentalAction, type En1990BridgeSls, type En1990Member, type En1990MemberEffect, type En1990PermanentAction, type En1990SeismicAction, type En1990VariableAction, parseEn1990AccidentalAction, parseEn1990BridgeSls, parseEn1990Member, parseEn1990MemberEffect, parseEn1990PermanentAction, parseEn1990SeismicAction, parseEn1990VariableAction } from "../📸️snapshot/🟦️.ts";

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
  permanents: En1990PermanentAction[] | null;
  /** @state artifact */
  variables: En1990VariableAction[] | null;
  /** @state artifact */
  accidentals: En1990AccidentalAction[] | null;
  /** @state artifact */
  seismics: En1990SeismicAction[] | null;
  /** @state artifact */
  members: En1990Member[] | null;
  /** @state artifact */
  bridgeSls: En1990BridgeSls[] | null;
  /** @state artifact */
  effects: En1990MemberEffect[] | null;
}

export const parseEn1990Diff: NormWireReader<En1990Diff> = normWireObject<En1990Diff>({ annex: normWireDefault(normWireNullable(normWireLiteral("En", "De")), () => null), projectId: normWireDefault(normWireNullable(normWireString), () => null), structureKind: normWireDefault(normWireNullable(normWireString), () => null), altitudeM: normWireDefault(normWireNullable(normWireNumber), () => null), consequenceClass: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), reliabilityClass: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), designWorkingLifeCategory: normWireDefault(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), () => null), designWorkingLifeYears: normWireDefault(normWireNullable(normWireNumber), () => null), referencePeriodYears: normWireDefault(normWireNullable(normWireNumber), () => null), supervisionLevel: normWireDefault(normWireNullable(normWireString), () => null), inspectionLevel: normWireDefault(normWireNullable(normWireString), () => null), kFiDeclared: normWireDefault(normWireNullable(normWireNumber), () => null), betaComputed: normWireDefault(normWireNullable(normWireNumber), () => null), permanents: normWireDefault(normWireNullable(normWireArray(parseEn1990PermanentAction)), () => null), variables: normWireDefault(normWireNullable(normWireArray(parseEn1990VariableAction)), () => null), accidentals: normWireDefault(normWireNullable(normWireArray(parseEn1990AccidentalAction)), () => null), seismics: normWireDefault(normWireNullable(normWireArray(parseEn1990SeismicAction)), () => null), members: normWireDefault(normWireNullable(normWireArray(parseEn1990Member)), () => null), bridgeSls: normWireDefault(normWireNullable(normWireArray(parseEn1990BridgeSls)), () => null), effects: normWireDefault(normWireNullable(normWireArray(parseEn1990MemberEffect)), () => null) });
