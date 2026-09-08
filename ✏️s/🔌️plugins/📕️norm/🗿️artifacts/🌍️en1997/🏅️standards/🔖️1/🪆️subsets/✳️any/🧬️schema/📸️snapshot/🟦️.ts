/** 🧬️ EN 1997 snapshot schema. */

export interface En1997Snapshot {
  /** @state artifact */
  vEdKn: number;
  /** @state artifact */
  hEdKn: number;
  /** @state artifact */
  footingAreaM2: number;
  /** @state artifact */
  phiDeg: number;
  /** @state artifact */
  cKpa: number;
  /** @state artifact */
  gammaKnM3: number;
  /** @state artifact */
  bM: number;
  /** @state artifact */
  dFM: number;
  /** @state artifact */
  eSMpa: number;
  /** @state artifact */
  nu: number;
  /** @state artifact */
  designApproach: number;
  /** @state artifact */
  annex: number;
  /** @state artifact */
  settlementLimitMm: number;
  /** @state artifact */
  nPileEdKn: number;
  /** @state artifact */
  alphaS: number;
  /** @state artifact */
  pileDM: number;
  /** @state artifact */
  qSKpa: number;
  /** @state artifact */
  pileLM: number;
  /** @state artifact */
  qBKpa: number;
  /** @state artifact */
  pileBaseAreaM2: number;
  /** @state artifact */
  pileNProfiles: number;
  /** @state artifact */
  zInvestigatedM: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1997SnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1997SnapshotGuardReject = (at: string, why: string): never => {
  throw new normEn1997SnapshotGuardRefusal(at, why);
};

type normEn1997SnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1997SnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1997SnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1997SnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1997SnapshotGuardReject(at, "value is not an object");
export const normEn1997SnapshotGuardArray = (value: unknown, at: string, bounds: normEn1997SnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1997SnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1997SnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1997SnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1997SnapshotGuardString = (value: unknown, at: string, bounds: normEn1997SnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1997SnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1997SnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1997SnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1997SnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1997SnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1997SnapshotGuardReject(at, "value is not a boolean"));
export const normEn1997SnapshotGuardNumber = (value: unknown, at: string, bounds: normEn1997SnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1997SnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1997SnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1997SnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1997SnapshotGuardInteger = (value: unknown, at: string, bounds: normEn1997SnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1997SnapshotGuardNumber(value, at, bounds) : normEn1997SnapshotGuardReject(at, "value is not an integer");
export const normEn1997SnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1997SnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1997SnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1997SnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1997Snapshot(value: unknown, at = "$"): En1997Snapshot {
  const row = normEn1997SnapshotGuardObject(value, at);
  return {
    vEdKn: normEn1997SnapshotGuardNumber(row["vEdKn"], `${at}.vEdKn`),
    hEdKn: normEn1997SnapshotGuardNumber(row["hEdKn"], `${at}.hEdKn`),
    footingAreaM2: normEn1997SnapshotGuardNumber(row["footingAreaM2"], `${at}.footingAreaM2`),
    phiDeg: normEn1997SnapshotGuardNumber(row["phiDeg"], `${at}.phiDeg`),
    cKpa: normEn1997SnapshotGuardNumber(row["cKpa"], `${at}.cKpa`),
    gammaKnM3: normEn1997SnapshotGuardNumber(row["gammaKnM3"], `${at}.gammaKnM3`),
    bM: normEn1997SnapshotGuardNumber(row["bM"], `${at}.bM`),
    dFM: normEn1997SnapshotGuardNumber(row["dFM"], `${at}.dFM`),
    eSMpa: normEn1997SnapshotGuardNumber(row["eSMpa"], `${at}.eSMpa`),
    nu: normEn1997SnapshotGuardNumber(row["nu"], `${at}.nu`),
    designApproach: normEn1997SnapshotGuardString(row["designApproach"], `${at}.designApproach`),
    annex: normEn1997SnapshotGuardString(row["annex"], `${at}.annex`),
    settlementLimitMm: normEn1997SnapshotGuardNumber(row["settlementLimitMm"], `${at}.settlementLimitMm`),
    nPileEdKn: normEn1997SnapshotGuardNumber(row["nPileEdKn"], `${at}.nPileEdKn`),
    alphaS: normEn1997SnapshotGuardNumber(row["alphaS"], `${at}.alphaS`),
    pileDM: normEn1997SnapshotGuardNumber(row["pileDM"], `${at}.pileDM`),
    qSKpa: normEn1997SnapshotGuardNumber(row["qSKpa"], `${at}.qSKpa`),
    pileLM: normEn1997SnapshotGuardNumber(row["pileLM"], `${at}.pileLM`),
    qBKpa: normEn1997SnapshotGuardNumber(row["qBKpa"], `${at}.qBKpa`),
    pileBaseAreaM2: normEn1997SnapshotGuardNumber(row["pileBaseAreaM2"], `${at}.pileBaseAreaM2`),
    pileNProfiles: normEn1997SnapshotGuardInteger(row["pileNProfiles"], `${at}.pileNProfiles`),
    zInvestigatedM: normEn1997SnapshotGuardNumber(row["zInvestigatedM"], `${at}.zInvestigatedM`),
  };
}
