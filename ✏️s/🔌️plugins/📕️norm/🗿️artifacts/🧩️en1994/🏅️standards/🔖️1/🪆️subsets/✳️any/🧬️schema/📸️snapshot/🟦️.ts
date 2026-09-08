/** 🧬️ En1994 snapshot schema — artifact-lane fields only. */

export interface En1994Snapshot {
  /** @state artifact */
  annex: string;
  /** @state artifact */
  mEdKnm: number;
  /** @state artifact */
  vEdKn: number;
  /** @state artifact */
  mPla: number;
  /** @state artifact */
  mPlRd: number;
  /** @state artifact */
  eta: number;
  /** @state artifact */
  vLRd: number;
  /** @state artifact */
  insulationThicknessMm: number;
  /** @state artifact */
  fireRating: string;
  /** @state artifact */
  deckType: string;
  /** @state artifact */
  deltaSigmaMpa: number;
  /** @state artifact */
  fatigueDetail: string;
  /** @state artifact */
  dMm: number;
  /** @state artifact */
  hScMm: number;
  /** @state artifact */
  fCkMpa: number;
  /** @state artifact */
  fUMpa: number;
  /** @state artifact */
  eCmMpa: number;
  /** @state artifact */
  vEdPerStudKn: number;
  /** @state artifact */
  spanM: number;
  /** @state artifact */
  fYMpa: number;
  /** @state artifact */
  nCyclesStud: number;
  /** @state artifact */
  deltaTauStudMpa: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1994SnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1994SnapshotGuardReject = (at: string, why: string): never => {
  throw new normEn1994SnapshotGuardRefusal(at, why);
};

type normEn1994SnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1994SnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1994SnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1994SnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1994SnapshotGuardReject(at, "value is not an object");
export const normEn1994SnapshotGuardArray = (value: unknown, at: string, bounds: normEn1994SnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1994SnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1994SnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1994SnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1994SnapshotGuardString = (value: unknown, at: string, bounds: normEn1994SnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1994SnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1994SnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1994SnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1994SnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1994SnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1994SnapshotGuardReject(at, "value is not a boolean"));
export const normEn1994SnapshotGuardNumber = (value: unknown, at: string, bounds: normEn1994SnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1994SnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1994SnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1994SnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1994SnapshotGuardInteger = (value: unknown, at: string, bounds: normEn1994SnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1994SnapshotGuardNumber(value, at, bounds) : normEn1994SnapshotGuardReject(at, "value is not an integer");
export const normEn1994SnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1994SnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1994SnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1994SnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1994Snapshot(value: unknown, at = "$"): En1994Snapshot {
  const row = normEn1994SnapshotGuardObject(value, at);
  return {
    annex: normEn1994SnapshotGuardString(row["annex"], `${at}.annex`),
    mEdKnm: normEn1994SnapshotGuardNumber(row["mEdKnm"], `${at}.mEdKnm`),
    vEdKn: normEn1994SnapshotGuardNumber(row["vEdKn"], `${at}.vEdKn`),
    mPla: normEn1994SnapshotGuardNumber(row["mPla"], `${at}.mPla`),
    mPlRd: normEn1994SnapshotGuardNumber(row["mPlRd"], `${at}.mPlRd`),
    eta: normEn1994SnapshotGuardNumber(row["eta"], `${at}.eta`),
    vLRd: normEn1994SnapshotGuardNumber(row["vLRd"], `${at}.vLRd`),
    insulationThicknessMm: normEn1994SnapshotGuardNumber(row["insulationThicknessMm"], `${at}.insulationThicknessMm`),
    fireRating: normEn1994SnapshotGuardString(row["fireRating"], `${at}.fireRating`),
    deckType: normEn1994SnapshotGuardString(row["deckType"], `${at}.deckType`),
    deltaSigmaMpa: normEn1994SnapshotGuardNumber(row["deltaSigmaMpa"], `${at}.deltaSigmaMpa`),
    fatigueDetail: normEn1994SnapshotGuardString(row["fatigueDetail"], `${at}.fatigueDetail`),
    dMm: normEn1994SnapshotGuardNumber(row["dMm"], `${at}.dMm`),
    hScMm: normEn1994SnapshotGuardNumber(row["hScMm"], `${at}.hScMm`),
    fCkMpa: normEn1994SnapshotGuardNumber(row["fCkMpa"], `${at}.fCkMpa`),
    fUMpa: normEn1994SnapshotGuardNumber(row["fUMpa"], `${at}.fUMpa`),
    eCmMpa: normEn1994SnapshotGuardNumber(row["eCmMpa"], `${at}.eCmMpa`),
    vEdPerStudKn: normEn1994SnapshotGuardNumber(row["vEdPerStudKn"], `${at}.vEdPerStudKn`),
    spanM: normEn1994SnapshotGuardNumber(row["spanM"], `${at}.spanM`),
    fYMpa: normEn1994SnapshotGuardNumber(row["fYMpa"], `${at}.fYMpa`),
    nCyclesStud: normEn1994SnapshotGuardNumber(row["nCyclesStud"], `${at}.nCyclesStud`),
    deltaTauStudMpa: normEn1994SnapshotGuardNumber(row["deltaTauStudMpa"], `${at}.deltaTauStudMpa`),
  };
}
