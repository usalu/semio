/** 🧬️ `En1990Artifact` wire twin: the artifact document across its state lanes, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1990Artifact {
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
  permanents: PermanentAction[];
  /** @state artifact */
  variables: VariableAction[];
  /** @state artifact */
  accidentals: AccidentalAction[];
  /** @state artifact */
  seismics: SeismicAction[];
  /** @state artifact */
  members: Member[];
  /** @state artifact */
  bridgeSls: BridgeSls[];
  /** @state artifact */
  effects: MemberEffect[];
}

export interface PermanentAction {
  id: string;
  kind: "g_sup" | "g_inf" | "prestress";
  gk: number;
}

export interface VariableAction {
  id: string;
  category: string;
  qk: number;
}

export interface AccidentalAction {
  id: string;
  ad: number;
}

export interface SeismicAction {
  id: string;
  aEk: number;
  importanceClass: "I" | "II" | "III" | "IV";
}

export interface Member {
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

export interface MemberEffect {
  memberId: string;
  actionId: string;
  influence: number;
}

export interface BridgeSls {
  id: string;
  memberId: string;
  deckAcceleration: number;
  deckAccelerationLimit: number;
  deckTwist: number;
  deckTwistLimit: number;
  bridgeDeflection: number;
  bridgeDeflectionLimit: number;
}

export const parseEn1990Artifact: NormWireReader<En1990Artifact> = normWireObject<En1990Artifact>({ annex: normWireRequired(normWireLiteral("En", "De")), projectId: normWireRequired(normWireString), structureKind: normWireRequired(normWireString), altitudeM: normWireRequired(normWireNumber), consequenceClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), reliabilityClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), designWorkingLifeCategory: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), designWorkingLifeYears: normWireRequired(normWireNumber), referencePeriodYears: normWireRequired(normWireNumber), supervisionLevel: normWireRequired(normWireString), inspectionLevel: normWireRequired(normWireString), kFiDeclared: normWireRequired(normWireNumber), betaComputed: normWireRequired(normWireNumber), permanents: normWireRequired(normWireArray(normWireRef(() => parsePermanentAction))), variables: normWireRequired(normWireArray(normWireRef(() => parseVariableAction))), accidentals: normWireRequired(normWireArray(normWireRef(() => parseAccidentalAction))), seismics: normWireRequired(normWireArray(normWireRef(() => parseSeismicAction))), members: normWireRequired(normWireArray(normWireRef(() => parseMember))), bridgeSls: normWireRequired(normWireArray(normWireRef(() => parseBridgeSls))), effects: normWireRequired(normWireArray(normWireRef(() => parseMemberEffect))) });
export const parsePermanentAction: NormWireReader<PermanentAction> = normWireObject<PermanentAction>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireLiteral("g_sup", "g_inf", "prestress")), gk: normWireRequired(normWireNumber) });
export const parseVariableAction: NormWireReader<VariableAction> = normWireObject<VariableAction>({ id: normWireRequired(normWireString), category: normWireRequired(normWireString), qk: normWireRequired(normWireNumber) });
export const parseAccidentalAction: NormWireReader<AccidentalAction> = normWireObject<AccidentalAction>({ id: normWireRequired(normWireString), ad: normWireRequired(normWireNumber) });
export const parseSeismicAction: NormWireReader<SeismicAction> = normWireObject<SeismicAction>({ id: normWireRequired(normWireString), aEk: normWireRequired(normWireNumber), importanceClass: normWireRequired(normWireLiteral("I", "II", "III", "IV")) });
export const parseMember: NormWireReader<Member> = normWireObject<Member>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), rdStr: normWireRequired(normWireNumber), rdGeo: normWireRequired(normWireNumber), rdEquStab: normWireRequired(normWireNumber), rdEquDestab: normWireRequired(normWireNumber), rdFat: normWireRequired(normWireNumber), span: normWireRequired(normWireNumber), deflectionW: normWireRequired(normWireNumber), deflectionLimitRatio: normWireRequired(normWireNumber), vibrationFrequency: normWireRequired(normWireNumber), vibrationFrequencyMin: normWireRequired(normWireNumber) });
export const parseMemberEffect: NormWireReader<MemberEffect> = normWireObject<MemberEffect>({ memberId: normWireRequired(normWireString), actionId: normWireRequired(normWireString), influence: normWireRequired(normWireNumber) });
export const parseBridgeSls: NormWireReader<BridgeSls> = normWireObject<BridgeSls>({ id: normWireRequired(normWireString), memberId: normWireRequired(normWireString), deckAcceleration: normWireRequired(normWireNumber), deckAccelerationLimit: normWireRequired(normWireNumber), deckTwist: normWireRequired(normWireNumber), deckTwistLimit: normWireRequired(normWireNumber), bridgeDeflection: normWireRequired(normWireNumber), bridgeDeflectionLimit: normWireRequired(normWireNumber) });
