/** 🧬️ EN 1999 snapshot schema. */

export interface En1999Snapshot {
  /** @state artifact */
  nEdKn: number;
  /** @state artifact */
  mEdKnm: number;
  /** @state artifact */
  aMm2: number;
  /** @state artifact */
  wElMm3: number;
  /** @state artifact */
  alloy: number;
  /** @state artifact */
  chi: number;
  /** @state artifact */
  iTMm4: number;
  /** @state artifact */
  lCrMm: number;
  /** @state artifact */
  thetaC: number;
  /** @state artifact */
  deltaSigmaEd: number;
  /** @state artifact */
  deltaSigmaC: number;
  /** @state artifact */
  fatigueM: number;
  /** @state artifact */
  nCycles: number;
  /** @state artifact */
  vWeldEdKn: number;
  /** @state artifact */
  weldThroatMm: number;
  /** @state artifact */
  weldLengthMm: number;
  /** @state artifact */
  betaW: number;
  /** @state artifact */
  sheetBMm: number;
  /** @state artifact */
  sheetTMm: number;
  /** @state artifact */
  sheetKSigma: number;
  /** @state artifact */
  sheetWElMm3: number;
  /** @state artifact */
  sheetMEdKnm: number;
  /** @state artifact */
  shellTMm: number;
  /** @state artifact */
  shellRMm: number;
  /** @state artifact */
  sigmaEdShellMpa: number;
  /** @state artifact */
  annex: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1999SnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1999SnapshotGuardReject = (at: string, why: string): never => {
  throw new normEn1999SnapshotGuardRefusal(at, why);
};

type normEn1999SnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1999SnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1999SnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1999SnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1999SnapshotGuardReject(at, "value is not an object");
export const normEn1999SnapshotGuardArray = (value: unknown, at: string, bounds: normEn1999SnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1999SnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1999SnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1999SnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1999SnapshotGuardString = (value: unknown, at: string, bounds: normEn1999SnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1999SnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1999SnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1999SnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1999SnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1999SnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1999SnapshotGuardReject(at, "value is not a boolean"));
export const normEn1999SnapshotGuardNumber = (value: unknown, at: string, bounds: normEn1999SnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1999SnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1999SnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1999SnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1999SnapshotGuardInteger = (value: unknown, at: string, bounds: normEn1999SnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1999SnapshotGuardNumber(value, at, bounds) : normEn1999SnapshotGuardReject(at, "value is not an integer");
export const normEn1999SnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1999SnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1999SnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1999SnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1999Snapshot(value: unknown, at = "$"): En1999Snapshot {
  const row = normEn1999SnapshotGuardObject(value, at);
  return {
    nEdKn: normEn1999SnapshotGuardNumber(row["nEdKn"], `${at}.nEdKn`),
    mEdKnm: normEn1999SnapshotGuardNumber(row["mEdKnm"], `${at}.mEdKnm`),
    aMm2: normEn1999SnapshotGuardNumber(row["aMm2"], `${at}.aMm2`),
    wElMm3: normEn1999SnapshotGuardNumber(row["wElMm3"], `${at}.wElMm3`),
    alloy: normEn1999SnapshotGuardString(row["alloy"], `${at}.alloy`),
    chi: normEn1999SnapshotGuardNumber(row["chi"], `${at}.chi`),
    iTMm4: normEn1999SnapshotGuardNumber(row["iTMm4"], `${at}.iTMm4`),
    lCrMm: normEn1999SnapshotGuardNumber(row["lCrMm"], `${at}.lCrMm`),
    thetaC: normEn1999SnapshotGuardNumber(row["thetaC"], `${at}.thetaC`),
    deltaSigmaEd: normEn1999SnapshotGuardNumber(row["deltaSigmaEd"], `${at}.deltaSigmaEd`),
    deltaSigmaC: normEn1999SnapshotGuardNumber(row["deltaSigmaC"], `${at}.deltaSigmaC`),
    fatigueM: normEn1999SnapshotGuardNumber(row["fatigueM"], `${at}.fatigueM`),
    nCycles: normEn1999SnapshotGuardNumber(row["nCycles"], `${at}.nCycles`),
    vWeldEdKn: normEn1999SnapshotGuardNumber(row["vWeldEdKn"], `${at}.vWeldEdKn`),
    weldThroatMm: normEn1999SnapshotGuardNumber(row["weldThroatMm"], `${at}.weldThroatMm`),
    weldLengthMm: normEn1999SnapshotGuardNumber(row["weldLengthMm"], `${at}.weldLengthMm`),
    betaW: normEn1999SnapshotGuardNumber(row["betaW"], `${at}.betaW`),
    sheetBMm: normEn1999SnapshotGuardNumber(row["sheetBMm"], `${at}.sheetBMm`),
    sheetTMm: normEn1999SnapshotGuardNumber(row["sheetTMm"], `${at}.sheetTMm`),
    sheetKSigma: normEn1999SnapshotGuardNumber(row["sheetKSigma"], `${at}.sheetKSigma`),
    sheetWElMm3: normEn1999SnapshotGuardNumber(row["sheetWElMm3"], `${at}.sheetWElMm3`),
    sheetMEdKnm: normEn1999SnapshotGuardNumber(row["sheetMEdKnm"], `${at}.sheetMEdKnm`),
    shellTMm: normEn1999SnapshotGuardNumber(row["shellTMm"], `${at}.shellTMm`),
    shellRMm: normEn1999SnapshotGuardNumber(row["shellRMm"], `${at}.shellRMm`),
    sigmaEdShellMpa: normEn1999SnapshotGuardNumber(row["sigmaEdShellMpa"], `${at}.sigmaEdShellMpa`),
    annex: normEn1999SnapshotGuardString(row["annex"], `${at}.annex`),
  };
}
