/** 🧬️ EN 1995 snapshot schema. */

export interface En1995Snapshot {
  /** @state artifact */
  annex: number;
  /** @state artifact */
  mEdKnm: number;
  /** @state artifact */
  nEdKn: number;
  /** @state artifact */
  vEdKn: number;
  /** @state artifact */
  wMm3: number;
  /** @state artifact */
  aMm2: number;
  /** @state artifact */
  bMm: number;
  /** @state artifact */
  hMm: number;
  /** @state artifact */
  fMK: number;
  /** @state artifact */
  fC0K: number;
  /** @state artifact */
  serviceClass: number;
  /** @state artifact */
  loadDuration: number;
  /** @state artifact */
  mCritKnm: number;
  /** @state artifact */
  fEdKn: number;
  /** @state artifact */
  aEfMm2: number;
  /** @state artifact */
  fVK: number;
  /** @state artifact */
  fireDurationMin: number;
  /** @state artifact */
  sectionDepthMm: number;
  /** @state artifact */
  aVertMS2: number;
  /** @state artifact */
  nCyclesBridge: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1995SnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1995SnapshotGuardReject = (at: string, why: string): never => {
  throw new normEn1995SnapshotGuardRefusal(at, why);
};

type normEn1995SnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1995SnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1995SnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1995SnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1995SnapshotGuardReject(at, "value is not an object");
export const normEn1995SnapshotGuardArray = (value: unknown, at: string, bounds: normEn1995SnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1995SnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1995SnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1995SnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1995SnapshotGuardString = (value: unknown, at: string, bounds: normEn1995SnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1995SnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1995SnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1995SnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1995SnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1995SnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1995SnapshotGuardReject(at, "value is not a boolean"));
export const normEn1995SnapshotGuardNumber = (value: unknown, at: string, bounds: normEn1995SnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1995SnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1995SnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1995SnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1995SnapshotGuardInteger = (value: unknown, at: string, bounds: normEn1995SnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1995SnapshotGuardNumber(value, at, bounds) : normEn1995SnapshotGuardReject(at, "value is not an integer");
export const normEn1995SnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1995SnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1995SnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1995SnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1995Snapshot(value: unknown, at = "$"): En1995Snapshot {
  const row = normEn1995SnapshotGuardObject(value, at);
  return {
    annex: normEn1995SnapshotGuardString(row["annex"], `${at}.annex`),
    mEdKnm: normEn1995SnapshotGuardNumber(row["mEdKnm"], `${at}.mEdKnm`),
    nEdKn: normEn1995SnapshotGuardNumber(row["nEdKn"], `${at}.nEdKn`),
    vEdKn: normEn1995SnapshotGuardNumber(row["vEdKn"], `${at}.vEdKn`),
    wMm3: normEn1995SnapshotGuardNumber(row["wMm3"], `${at}.wMm3`),
    aMm2: normEn1995SnapshotGuardNumber(row["aMm2"], `${at}.aMm2`),
    bMm: normEn1995SnapshotGuardNumber(row["bMm"], `${at}.bMm`),
    hMm: normEn1995SnapshotGuardNumber(row["hMm"], `${at}.hMm`),
    fMK: normEn1995SnapshotGuardNumber(row["fMK"], `${at}.fMK`),
    fC0K: normEn1995SnapshotGuardNumber(row["fC0K"], `${at}.fC0K`),
    serviceClass: normEn1995SnapshotGuardString(row["serviceClass"], `${at}.serviceClass`),
    loadDuration: normEn1995SnapshotGuardString(row["loadDuration"], `${at}.loadDuration`),
    mCritKnm: normEn1995SnapshotGuardNumber(row["mCritKnm"], `${at}.mCritKnm`),
    fEdKn: normEn1995SnapshotGuardNumber(row["fEdKn"], `${at}.fEdKn`),
    aEfMm2: normEn1995SnapshotGuardNumber(row["aEfMm2"], `${at}.aEfMm2`),
    fVK: normEn1995SnapshotGuardNumber(row["fVK"], `${at}.fVK`),
    fireDurationMin: normEn1995SnapshotGuardNumber(row["fireDurationMin"], `${at}.fireDurationMin`),
    sectionDepthMm: normEn1995SnapshotGuardNumber(row["sectionDepthMm"], `${at}.sectionDepthMm`),
    aVertMS2: normEn1995SnapshotGuardNumber(row["aVertMS2"], `${at}.aVertMS2`),
    nCyclesBridge: normEn1995SnapshotGuardNumber(row["nCyclesBridge"], `${at}.nCyclesBridge`),
  };
}
