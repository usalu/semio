/** 🧬️ EN 1998 snapshot schema. */

export interface En1998Snapshot {
  /** @state artifact */
  seismicZone: number;
  /** @state artifact */
  groundType: number;
  /** @state artifact */
  importanceClass: number;
  /** @state artifact */
  structuralSystem: number;
  /** @state artifact */
  t1S: number;
  /** @state artifact */
  massT: number;
  /** @state artifact */
  vRdKn: number;
  /** @state artifact */
  driftMm: number;
  /** @state artifact */
  heightM: number;
  /** @state artifact */
  multipleResistingSystems: boolean;
  /** @state artifact */
  annex: number;
  /** @state artifact */
  enAGr: number;
  /** @state artifact */
  enGroundType: number;
  /** @state artifact */
  enSpectrumType: number;
  /** @state artifact */
  periodRatio: number;
  /** @state artifact */
  bridgeVRdKn: number;
  /** @state artifact */
  bearingDEdMm: number;
  /** @state artifact */
  bearingDRdMm: number;
  /** @state artifact */
  retrofitKnowledgeLevel: number;
  /** @state artifact */
  retrofitLimitState: number;
  /** @state artifact */
  retrofitEDKn: number;
  /** @state artifact */
  retrofitRKKn: number;
  /** @state artifact */
  retrofitGammaEl: number;
  /** @state artifact */
  siloHeightM: number;
  /** @state artifact */
  siloRadiusM: number;
  /** @state artifact */
  siloNRdKn: number;
  /** @state artifact */
  siloVEdKn: number;
  /** @state artifact */
  siloVRdKn: number;
  /** @state artifact */
  siloQNominal: number;
  /** @state artifact */
  tankHeightM: number;
  /** @state artifact */
  tankRadiusM: number;
  /** @state artifact */
  tankMassT: number;
  /** @state artifact */
  tankVRdKn: number;
  /** @state artifact */
  towerMEdKnm: number;
  /** @state artifact */
  towerMRdKnm: number;
  /** @state artifact */
  towerIsChimney: boolean;
  /** @state artifact */
  towerQNominal: number;
  /** @state artifact */
  towerMassT: number;
  /** @state artifact */
  foundationAreaM2: number;
  /** @state artifact */
  foundationPRdKpa: number;
  /** @state artifact */
  foundationHEdKn: number;
  /** @state artifact */
  foundationHRdKn: number;
  /** @state artifact */
  kFoundation: number;
  /** @state artifact */
  kSoil: number;
  /** @state artifact */
  wallHeightM: number;
  /** @state artifact */
  wallPhiDeg: number;
  /** @state artifact */
  wallSoilGammaKnM3: number;
  /** @state artifact */
  wallR: number;
  /** @state artifact */
  wallHRdKn: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1998SnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1998SnapshotGuardReject = (at: string, why: string): never => {
  throw new normEn1998SnapshotGuardRefusal(at, why);
};

type normEn1998SnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1998SnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1998SnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1998SnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1998SnapshotGuardReject(at, "value is not an object");
export const normEn1998SnapshotGuardArray = (value: unknown, at: string, bounds: normEn1998SnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1998SnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1998SnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1998SnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1998SnapshotGuardString = (value: unknown, at: string, bounds: normEn1998SnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1998SnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1998SnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1998SnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1998SnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1998SnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1998SnapshotGuardReject(at, "value is not a boolean"));
export const normEn1998SnapshotGuardNumber = (value: unknown, at: string, bounds: normEn1998SnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1998SnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1998SnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1998SnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1998SnapshotGuardInteger = (value: unknown, at: string, bounds: normEn1998SnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1998SnapshotGuardNumber(value, at, bounds) : normEn1998SnapshotGuardReject(at, "value is not an integer");
export const normEn1998SnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1998SnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1998SnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1998SnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1998Snapshot(value: unknown, at = "$"): En1998Snapshot {
  const row = normEn1998SnapshotGuardObject(value, at);
  return {
    seismicZone: normEn1998SnapshotGuardInteger(row["seismicZone"], `${at}.seismicZone`),
    groundType: normEn1998SnapshotGuardString(row["groundType"], `${at}.groundType`),
    importanceClass: normEn1998SnapshotGuardString(row["importanceClass"], `${at}.importanceClass`),
    structuralSystem: normEn1998SnapshotGuardString(row["structuralSystem"], `${at}.structuralSystem`),
    t1S: normEn1998SnapshotGuardNumber(row["t1S"], `${at}.t1S`),
    massT: normEn1998SnapshotGuardNumber(row["massT"], `${at}.massT`),
    vRdKn: normEn1998SnapshotGuardNumber(row["vRdKn"], `${at}.vRdKn`),
    driftMm: normEn1998SnapshotGuardNumber(row["driftMm"], `${at}.driftMm`),
    heightM: normEn1998SnapshotGuardNumber(row["heightM"], `${at}.heightM`),
    multipleResistingSystems: normEn1998SnapshotGuardBoolean(row["multipleResistingSystems"], `${at}.multipleResistingSystems`),
    annex: normEn1998SnapshotGuardString(row["annex"], `${at}.annex`),
    enAGr: normEn1998SnapshotGuardNumber(row["enAGr"], `${at}.enAGr`),
    enGroundType: normEn1998SnapshotGuardString(row["enGroundType"], `${at}.enGroundType`),
    enSpectrumType: normEn1998SnapshotGuardString(row["enSpectrumType"], `${at}.enSpectrumType`),
    periodRatio: normEn1998SnapshotGuardNumber(row["periodRatio"], `${at}.periodRatio`),
    bridgeVRdKn: normEn1998SnapshotGuardNumber(row["bridgeVRdKn"], `${at}.bridgeVRdKn`),
    bearingDEdMm: normEn1998SnapshotGuardNumber(row["bearingDEdMm"], `${at}.bearingDEdMm`),
    bearingDRdMm: normEn1998SnapshotGuardNumber(row["bearingDRdMm"], `${at}.bearingDRdMm`),
    retrofitKnowledgeLevel: normEn1998SnapshotGuardString(row["retrofitKnowledgeLevel"], `${at}.retrofitKnowledgeLevel`),
    retrofitLimitState: normEn1998SnapshotGuardString(row["retrofitLimitState"], `${at}.retrofitLimitState`),
    retrofitEDKn: normEn1998SnapshotGuardNumber(row["retrofitEDKn"], `${at}.retrofitEDKn`),
    retrofitRKKn: normEn1998SnapshotGuardNumber(row["retrofitRKKn"], `${at}.retrofitRKKn`),
    retrofitGammaEl: normEn1998SnapshotGuardNumber(row["retrofitGammaEl"], `${at}.retrofitGammaEl`),
    siloHeightM: normEn1998SnapshotGuardNumber(row["siloHeightM"], `${at}.siloHeightM`),
    siloRadiusM: normEn1998SnapshotGuardNumber(row["siloRadiusM"], `${at}.siloRadiusM`),
    siloNRdKn: normEn1998SnapshotGuardNumber(row["siloNRdKn"], `${at}.siloNRdKn`),
    siloVEdKn: normEn1998SnapshotGuardNumber(row["siloVEdKn"], `${at}.siloVEdKn`),
    siloVRdKn: normEn1998SnapshotGuardNumber(row["siloVRdKn"], `${at}.siloVRdKn`),
    siloQNominal: normEn1998SnapshotGuardNumber(row["siloQNominal"], `${at}.siloQNominal`),
    tankHeightM: normEn1998SnapshotGuardNumber(row["tankHeightM"], `${at}.tankHeightM`),
    tankRadiusM: normEn1998SnapshotGuardNumber(row["tankRadiusM"], `${at}.tankRadiusM`),
    tankMassT: normEn1998SnapshotGuardNumber(row["tankMassT"], `${at}.tankMassT`),
    tankVRdKn: normEn1998SnapshotGuardNumber(row["tankVRdKn"], `${at}.tankVRdKn`),
    towerMEdKnm: normEn1998SnapshotGuardNumber(row["towerMEdKnm"], `${at}.towerMEdKnm`),
    towerMRdKnm: normEn1998SnapshotGuardNumber(row["towerMRdKnm"], `${at}.towerMRdKnm`),
    towerIsChimney: normEn1998SnapshotGuardBoolean(row["towerIsChimney"], `${at}.towerIsChimney`),
    towerQNominal: normEn1998SnapshotGuardNumber(row["towerQNominal"], `${at}.towerQNominal`),
    towerMassT: normEn1998SnapshotGuardNumber(row["towerMassT"], `${at}.towerMassT`),
    foundationAreaM2: normEn1998SnapshotGuardNumber(row["foundationAreaM2"], `${at}.foundationAreaM2`),
    foundationPRdKpa: normEn1998SnapshotGuardNumber(row["foundationPRdKpa"], `${at}.foundationPRdKpa`),
    foundationHEdKn: normEn1998SnapshotGuardNumber(row["foundationHEdKn"], `${at}.foundationHEdKn`),
    foundationHRdKn: normEn1998SnapshotGuardNumber(row["foundationHRdKn"], `${at}.foundationHRdKn`),
    kFoundation: normEn1998SnapshotGuardNumber(row["kFoundation"], `${at}.kFoundation`),
    kSoil: normEn1998SnapshotGuardNumber(row["kSoil"], `${at}.kSoil`),
    wallHeightM: normEn1998SnapshotGuardNumber(row["wallHeightM"], `${at}.wallHeightM`),
    wallPhiDeg: normEn1998SnapshotGuardNumber(row["wallPhiDeg"], `${at}.wallPhiDeg`),
    wallSoilGammaKnM3: normEn1998SnapshotGuardNumber(row["wallSoilGammaKnM3"], `${at}.wallSoilGammaKnM3`),
    wallR: normEn1998SnapshotGuardNumber(row["wallR"], `${at}.wallR`),
    wallHRdKn: normEn1998SnapshotGuardNumber(row["wallHRdKn"], `${at}.wallHRdKn`),
  };
}
