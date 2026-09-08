/** 🧬️ EN 1996 snapshot schema. */

export interface En1996Snapshot {
  /** @state artifact */
  mEdKnm: number;
  /** @state artifact */
  nEdKn: number;
  /** @state artifact */
  vEdKn: number;
  /** @state artifact */
  hEdKn: number;
  /** @state artifact */
  zMm3: number;
  /** @state artifact */
  areaMm2: number;
  /** @state artifact */
  shearAreaMm2: number;
  /** @state artifact */
  fKMpa: number;
  /** @state artifact */
  fVkMpa: number;
  /** @state artifact */
  annex: number;
  /** @state artifact */
  masonryClass: number;
  /** @state artifact */
  designSituation: number;
  /** @state artifact */
  mu: number;
  /** @state artifact */
  wallThicknessMm: number;
  /** @state artifact */
  fireResistanceMin: number;
  /** @state artifact */
  unit: number;
  /** @state artifact */
  exposure: number;
  /** @state artifact */
  mortar: number;
  /** @state artifact */
  bedJointThicknessMm: number;
  /** @state artifact */
  storeys: number;
  /** @state artifact */
  hEfMm: number;
  /** @state artifact */
  tEfMm: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1996SnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1996SnapshotGuardReject = (at: string, why: string): never => {
  throw new normEn1996SnapshotGuardRefusal(at, why);
};

type normEn1996SnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1996SnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1996SnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1996SnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1996SnapshotGuardReject(at, "value is not an object");
export const normEn1996SnapshotGuardArray = (value: unknown, at: string, bounds: normEn1996SnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1996SnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1996SnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1996SnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1996SnapshotGuardString = (value: unknown, at: string, bounds: normEn1996SnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1996SnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1996SnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1996SnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1996SnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1996SnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1996SnapshotGuardReject(at, "value is not a boolean"));
export const normEn1996SnapshotGuardNumber = (value: unknown, at: string, bounds: normEn1996SnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1996SnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1996SnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1996SnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1996SnapshotGuardInteger = (value: unknown, at: string, bounds: normEn1996SnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1996SnapshotGuardNumber(value, at, bounds) : normEn1996SnapshotGuardReject(at, "value is not an integer");
export const normEn1996SnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1996SnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1996SnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1996SnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1996Snapshot(value: unknown, at = "$"): En1996Snapshot {
  const row = normEn1996SnapshotGuardObject(value, at);
  return {
    mEdKnm: normEn1996SnapshotGuardNumber(row["mEdKnm"], `${at}.mEdKnm`),
    nEdKn: normEn1996SnapshotGuardNumber(row["nEdKn"], `${at}.nEdKn`),
    vEdKn: normEn1996SnapshotGuardNumber(row["vEdKn"], `${at}.vEdKn`),
    hEdKn: normEn1996SnapshotGuardNumber(row["hEdKn"], `${at}.hEdKn`),
    zMm3: normEn1996SnapshotGuardNumber(row["zMm3"], `${at}.zMm3`),
    areaMm2: normEn1996SnapshotGuardNumber(row["areaMm2"], `${at}.areaMm2`),
    shearAreaMm2: normEn1996SnapshotGuardNumber(row["shearAreaMm2"], `${at}.shearAreaMm2`),
    fKMpa: normEn1996SnapshotGuardNumber(row["fKMpa"], `${at}.fKMpa`),
    fVkMpa: normEn1996SnapshotGuardNumber(row["fVkMpa"], `${at}.fVkMpa`),
    annex: normEn1996SnapshotGuardString(row["annex"], `${at}.annex`),
    masonryClass: normEn1996SnapshotGuardString(row["masonryClass"], `${at}.masonryClass`),
    designSituation: normEn1996SnapshotGuardString(row["designSituation"], `${at}.designSituation`),
    mu: normEn1996SnapshotGuardNumber(row["mu"], `${at}.mu`),
    wallThicknessMm: normEn1996SnapshotGuardNumber(row["wallThicknessMm"], `${at}.wallThicknessMm`),
    fireResistanceMin: normEn1996SnapshotGuardInteger(row["fireResistanceMin"], `${at}.fireResistanceMin`),
    unit: normEn1996SnapshotGuardString(row["unit"], `${at}.unit`),
    exposure: normEn1996SnapshotGuardString(row["exposure"], `${at}.exposure`),
    mortar: normEn1996SnapshotGuardString(row["mortar"], `${at}.mortar`),
    bedJointThicknessMm: normEn1996SnapshotGuardNumber(row["bedJointThicknessMm"], `${at}.bedJointThicknessMm`),
    storeys: normEn1996SnapshotGuardInteger(row["storeys"], `${at}.storeys`),
    hEfMm: normEn1996SnapshotGuardNumber(row["hEfMm"], `${at}.hEfMm`),
    tEfMm: normEn1996SnapshotGuardNumber(row["tEfMm"], `${at}.tEfMm`),
  };
}
