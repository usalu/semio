/** 📸️ `En1990Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1990Snapshot {
  /** @state artifact */
  annex: "En" | "De";
  /** @state artifact */
  projectId: string;
  /** @state artifact */
  structureKind: string;
  /** @state artifact */
  altitudeM: number;
  /** @state artifact */
  consequenceClass: number;
  /** @state artifact */
  reliabilityClass: number;
  /** @state artifact */
  designWorkingLifeCategory: number;
  /** @state artifact */
  designWorkingLifeYears: number;
  /** @state artifact */
  referencePeriodYears: number;
  /** @state artifact */
  supervisionLevel: string;
  /** @state artifact */
  inspectionLevel: string;
  /** @state artifact */
  kFiDeclared: number;
  /** @state artifact */
  betaComputed: number;
  /** @state artifact */
  permanents: En1990PermanentAction[];
  /** @state artifact */
  variables: En1990VariableAction[];
  /** @state artifact */
  accidentals: En1990AccidentalAction[];
  /** @state artifact */
  seismics: En1990SeismicAction[];
  /** @state artifact */
  members: En1990Member[];
  /** @state artifact */
  bridgeSls: En1990BridgeSls[];
  /** @state artifact */
  effects: En1990MemberEffect[];
}

export interface En1990PermanentAction {
  id: string;
  kind: string;
  gk: number;
}

export interface En1990VariableAction {
  id: string;
  category: string;
  qk: number;
}

export interface En1990AccidentalAction {
  id: string;
  ad: number;
}

export interface En1990SeismicAction {
  id: string;
  aEk: number;
  importanceClass: En1990ImportanceClass;
}

export interface En1990Member {
  id: string;
  labelEn: string;
  labelDe: string;
  rdStr: number;
  rdGeo: number;
  rdEquStab: number;
  rdEquDestab: number;
  rdFat: number;
  span: number;
  deflectionW: number;
  deflectionLimitRatio: number;
  vibrationFrequency: number;
  vibrationFrequencyMin: number;
}

export interface En1990MemberEffect {
  memberId: string;
  actionId: string;
  influence: number;
}

export interface En1990BridgeSls {
  id: string;
  memberId: string;
  deckAcceleration: number;
  deckAccelerationLimit: number;
  deckTwist: number;
  deckTwistLimit: number;
  bridgeDeflection: number;
  bridgeDeflectionLimit: number;
}

export type En1990AnnexChoice = "En" | "De";

export type En1990ImportanceClass = "I" | "II" | "III" | "IV";

export const parseEn1990Snapshot: NormWireReader<En1990Snapshot> = normWireObject<En1990Snapshot>({ annex: normWireRequired(normWireLiteral("En", "De")), projectId: normWireRequired(normWireString), structureKind: normWireRequired(normWireString), altitudeM: normWireRequired(normWireNumber), consequenceClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), reliabilityClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), designWorkingLifeCategory: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), designWorkingLifeYears: normWireRequired(normWireNumber), referencePeriodYears: normWireRequired(normWireNumber), supervisionLevel: normWireRequired(normWireString), inspectionLevel: normWireRequired(normWireString), kFiDeclared: normWireRequired(normWireNumber), betaComputed: normWireRequired(normWireNumber), permanents: normWireRequired(normWireArray(normWireRef(() => parseEn1990PermanentAction))), variables: normWireRequired(normWireArray(normWireRef(() => parseEn1990VariableAction))), accidentals: normWireRequired(normWireArray(normWireRef(() => parseEn1990AccidentalAction))), seismics: normWireRequired(normWireArray(normWireRef(() => parseEn1990SeismicAction))), members: normWireRequired(normWireArray(normWireRef(() => parseEn1990Member))), bridgeSls: normWireRequired(normWireArray(normWireRef(() => parseEn1990BridgeSls))), effects: normWireRequired(normWireArray(normWireRef(() => parseEn1990MemberEffect))) });
export const parseEn1990PermanentAction: NormWireReader<En1990PermanentAction> = normWireObject<En1990PermanentAction>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireString), gk: normWireRequired(normWireNumber) });
export const parseEn1990VariableAction: NormWireReader<En1990VariableAction> = normWireObject<En1990VariableAction>({ id: normWireRequired(normWireString), category: normWireRequired(normWireString), qk: normWireRequired(normWireNumber) });
export const parseEn1990AccidentalAction: NormWireReader<En1990AccidentalAction> = normWireObject<En1990AccidentalAction>({ id: normWireRequired(normWireString), ad: normWireRequired(normWireNumber) });
export const parseEn1990SeismicAction: NormWireReader<En1990SeismicAction> = normWireObject<En1990SeismicAction>({ id: normWireRequired(normWireString), aEk: normWireRequired(normWireNumber), importanceClass: normWireRequired(normWireRef(() => parseEn1990ImportanceClass)) });
export const parseEn1990Member: NormWireReader<En1990Member> = normWireObject<En1990Member>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), rdStr: normWireRequired(normWireNumber), rdGeo: normWireRequired(normWireNumber), rdEquStab: normWireRequired(normWireNumber), rdEquDestab: normWireRequired(normWireNumber), rdFat: normWireRequired(normWireNumber), span: normWireRequired(normWireNumber), deflectionW: normWireRequired(normWireNumber), deflectionLimitRatio: normWireRequired(normWireNumber), vibrationFrequency: normWireRequired(normWireNumber), vibrationFrequencyMin: normWireRequired(normWireNumber) });
export const parseEn1990MemberEffect: NormWireReader<En1990MemberEffect> = normWireObject<En1990MemberEffect>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), influence: normWireRequired(normWireNumber) });
export const parseEn1990BridgeSls: NormWireReader<En1990BridgeSls> = normWireObject<En1990BridgeSls>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), deckAcceleration: normWireRequired(normWireNumber), deckAccelerationLimit: normWireRequired(normWireNumber), deckTwist: normWireRequired(normWireNumber), deckTwistLimit: normWireRequired(normWireNumber), bridgeDeflection: normWireRequired(normWireNumber), bridgeDeflectionLimit: normWireRequired(normWireNumber) });
export const parseEn1990AnnexChoice: NormWireReader<En1990AnnexChoice> = normWireLiteral("En", "De");
export const parseEn1990ImportanceClass: NormWireReader<En1990ImportanceClass> = normWireLiteral("I", "II", "III", "IV");
