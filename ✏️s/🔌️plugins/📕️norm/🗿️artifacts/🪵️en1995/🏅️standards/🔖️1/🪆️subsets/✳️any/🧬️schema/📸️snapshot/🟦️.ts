/** 📸️ `En1995Snapshot` wire twin: the persisted snapshot and every record it holds, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface En1995Snapshot {
  /** @state artifact */
  annex: "En" | "De";
  /** @state artifact */
  members: TimberMember[];
  /** @state artifact */
  connections: TimberConnection[];
}

export interface TimberConnection {
  id: string;
  labelEn: string;
  labelDe: string;
  fastenerType: string;
  strengthClass: string;
  serviceClass: number;
  diameterM: number;
  number: number;
  rows: number;
  spacingM: number;
  edgeDistanceM: number;
  endDistanceM: number;
  t1M: number;
  t2M: number;
  steelPlate: boolean;
  steelPlateThicknessM: number;
  shearPlanes: number;
  fUK: number;
  actions: ConnectionAction[];
}

export interface ConnectionAction {
  id: string;
  kind: string;
  loadDuration: string;
  fKN: number;
}

export interface TimberMember {
  id: string;
  labelEn: string;
  labelDe: string;
  role: MemberRole;
  strengthClass: string;
  serviceClass: number;
  support: SupportType;
  bM: number;
  hM: number;
  spanM: number;
  supportLengthM: number;
  bearingLengthM: number;
  bucklingLengthYM: number;
  bucklingLengthZM: number;
  lateralRestraintSpacingM: number;
  notchDepthM: number;
  notchDistanceM: number;
  mCritNm: number;
  massKgPerM: number;
  massKgPerM2: number;
  dampingXi: number;
  fireDurationS: number;
  bridgeNObs: number;
  bridgeTLYears: number;
  bridgeBeta: number;
  bridgeA: number;
  bridgeB: number;
  bridgeCrowdPerM2: number;
  actions: CharacteristicAction[];
}

export interface CharacteristicAction {
  id: string;
  kind: string;
  category: string;
  loadDuration: string;
  qLineNPerM: number;
  fPointN: number;
  mKNm: number;
  vKN: number;
  nKN: number;
  nTKN: number;
  fC90KN: number;
}

export type AnnexChoice = "En" | "De";

export type MemberRole = "beam" | "column" | "floor" | "bridge";

export type SupportType = "simplySupported" | "cantilever" | "continuousTwoSpan";

export const parseEn1995Snapshot: NormWireReader<En1995Snapshot> = normWireObject<En1995Snapshot>({ annex: normWireRequired(normWireLiteral("En", "De")), members: normWireRequired(normWireArray(normWireRef(() => parseTimberMember))), connections: normWireRequired(normWireArray(normWireRef(() => parseTimberConnection))) });
export const parseTimberConnection: NormWireReader<TimberConnection> = normWireObject<TimberConnection>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), fastenerType: normWireRequired(normWireString), strengthClass: normWireRequired(normWireString), serviceClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), diameterM: normWireRequired(normWireNumber), number: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), rows: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), spacingM: normWireRequired(normWireNumber), edgeDistanceM: normWireRequired(normWireNumber), endDistanceM: normWireRequired(normWireNumber), t1M: normWireRequired(normWireNumber), t2M: normWireRequired(normWireNumber), steelPlate: normWireRequired(normWireBoolean), steelPlateThicknessM: normWireRequired(normWireNumber), shearPlanes: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295})), fUK: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseConnectionAction))) });
export const parseConnectionAction: NormWireReader<ConnectionAction> = normWireObject<ConnectionAction>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireString), loadDuration: normWireRequired(normWireString), fKN: normWireRequired(normWireNumber) });
export const parseTimberMember: NormWireReader<TimberMember> = normWireObject<TimberMember>({ id: normWireRequired(normWireString), labelEn: normWireRequired(normWireString), labelDe: normWireRequired(normWireString), role: normWireRequired(normWireRef(() => parseMemberRole)), strengthClass: normWireRequired(normWireString), serviceClass: normWireRequired(normWireRange(normWireInteger, {"minimum":0,"maximum":255})), support: normWireRequired(normWireRef(() => parseSupportType)), bM: normWireRequired(normWireNumber), hM: normWireRequired(normWireNumber), spanM: normWireRequired(normWireNumber), supportLengthM: normWireRequired(normWireNumber), bearingLengthM: normWireRequired(normWireNumber), bucklingLengthYM: normWireRequired(normWireNumber), bucklingLengthZM: normWireRequired(normWireNumber), lateralRestraintSpacingM: normWireRequired(normWireNumber), notchDepthM: normWireRequired(normWireNumber), notchDistanceM: normWireRequired(normWireNumber), mCritNm: normWireRequired(normWireNumber), massKgPerM: normWireRequired(normWireNumber), massKgPerM2: normWireRequired(normWireNumber), dampingXi: normWireRequired(normWireNumber), fireDurationS: normWireRequired(normWireNumber), bridgeNObs: normWireRequired(normWireNumber), bridgeTLYears: normWireRequired(normWireNumber), bridgeBeta: normWireRequired(normWireNumber), bridgeA: normWireRequired(normWireNumber), bridgeB: normWireRequired(normWireNumber), bridgeCrowdPerM2: normWireRequired(normWireNumber), actions: normWireRequired(normWireArray(normWireRef(() => parseCharacteristicAction))) });
export const parseCharacteristicAction: NormWireReader<CharacteristicAction> = normWireObject<CharacteristicAction>({ id: normWireRequired(normWireString), kind: normWireRequired(normWireString), category: normWireRequired(normWireString), loadDuration: normWireRequired(normWireString), qLineNPerM: normWireRequired(normWireNumber), fPointN: normWireRequired(normWireNumber), mKNm: normWireRequired(normWireNumber), vKN: normWireRequired(normWireNumber), nKN: normWireRequired(normWireNumber), nTKN: normWireRequired(normWireNumber), fC90KN: normWireRequired(normWireNumber) });
export const parseAnnexChoice: NormWireReader<AnnexChoice> = normWireLiteral("En", "De");
export const parseMemberRole: NormWireReader<MemberRole> = normWireLiteral("beam", "column", "floor", "bridge");
export const parseSupportType: NormWireReader<SupportType> = normWireLiteral("simplySupported", "cantilever", "continuousTwoSpan");

export * from "./🪶️sqlite/🟦️.ts";
