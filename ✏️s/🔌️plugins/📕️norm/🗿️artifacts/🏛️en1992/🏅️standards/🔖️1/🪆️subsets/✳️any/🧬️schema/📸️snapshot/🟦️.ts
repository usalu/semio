/** 🧬️ En1992 snapshot schema — artifact-lane fields only. */

export interface En1992Snapshot {
  /** @state artifact */
  annex: string;
  /** @state artifact */
  mEdKnm: number;
  /** @state artifact */
  vEdKn: number;
  /** @state artifact */
  fCk: number;
  /** @state artifact */
  bMm: number;
  /** @state artifact */
  dMm: number;
  /** @state artifact */
  aSMm2: number;
  /** @state artifact */
  fYk: number;
  /** @state artifact */
  rhoL: number;
  /** @state artifact */
  nEdKn: number;
  /** @state artifact */
  pKn: number;
  /** @state artifact */
  aCMm2: number;
  /** @state artifact */
  useFem: boolean;
  /** @state artifact */
  spanM: number;
  /** @state artifact */
  udlKnM: number;
  /** @state artifact */
  fireRating: string;
  /** @state artifact */
  providedAxisDistanceMm: number;
  /** @state artifact */
  bridgeSigmaCMpa: number;
  /** @state artifact */
  bridgeDeltaSigmaSMpa: number;
  /** @state artifact */
  tightnessClass: string;
  /** @state artifact */
  hdOverH: number;
  /** @state artifact */
  liquidSigmaSMpa: number;
  /** @state artifact */
  liquidRhoPEff: number;
  /** @state artifact */
  liquidFCtEffMpa: number;
  /** @state artifact */
  liquidESMpa: number;
  /** @state artifact */
  liquidSRMaxMm: number;
  /** @state artifact */
  anchorHEfMm: number;
  /** @state artifact */
  anchorCracked: boolean;
  /** @state artifact */
  anchorFUkMpa: number;
  /** @state artifact */
  anchorFYkMpa: number;
  /** @state artifact */
  anchorASMm2: number;
  /** @state artifact */
  anchorDMm: number;
  /** @state artifact */
  anchorC1Mm: number;
  /** @state artifact */
  anchorNEdKn: number;
  /** @state artifact */
  anchorVEdKn: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1992SnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1992SnapshotGuardReject = (at: string, why: string): never => {
  throw new normEn1992SnapshotGuardRefusal(at, why);
};

type normEn1992SnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1992SnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1992SnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1992SnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1992SnapshotGuardReject(at, "value is not an object");
export const normEn1992SnapshotGuardArray = (value: unknown, at: string, bounds: normEn1992SnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1992SnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1992SnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1992SnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1992SnapshotGuardString = (value: unknown, at: string, bounds: normEn1992SnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1992SnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1992SnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1992SnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1992SnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1992SnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1992SnapshotGuardReject(at, "value is not a boolean"));
export const normEn1992SnapshotGuardNumber = (value: unknown, at: string, bounds: normEn1992SnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1992SnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1992SnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1992SnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1992SnapshotGuardInteger = (value: unknown, at: string, bounds: normEn1992SnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1992SnapshotGuardNumber(value, at, bounds) : normEn1992SnapshotGuardReject(at, "value is not an integer");
export const normEn1992SnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1992SnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1992SnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1992SnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1992Snapshot(value: unknown, at = "$"): En1992Snapshot {
  const row = normEn1992SnapshotGuardObject(value, at);
  return {
    annex: normEn1992SnapshotGuardString(row["annex"], `${at}.annex`),
    mEdKnm: normEn1992SnapshotGuardNumber(row["mEdKnm"], `${at}.mEdKnm`),
    vEdKn: normEn1992SnapshotGuardNumber(row["vEdKn"], `${at}.vEdKn`),
    fCk: normEn1992SnapshotGuardNumber(row["fCk"], `${at}.fCk`),
    bMm: normEn1992SnapshotGuardNumber(row["bMm"], `${at}.bMm`),
    dMm: normEn1992SnapshotGuardNumber(row["dMm"], `${at}.dMm`),
    aSMm2: normEn1992SnapshotGuardNumber(row["aSMm2"], `${at}.aSMm2`),
    fYk: normEn1992SnapshotGuardNumber(row["fYk"], `${at}.fYk`),
    rhoL: normEn1992SnapshotGuardNumber(row["rhoL"], `${at}.rhoL`),
    nEdKn: normEn1992SnapshotGuardNumber(row["nEdKn"], `${at}.nEdKn`),
    pKn: normEn1992SnapshotGuardNumber(row["pKn"], `${at}.pKn`),
    aCMm2: normEn1992SnapshotGuardNumber(row["aCMm2"], `${at}.aCMm2`),
    useFem: normEn1992SnapshotGuardBoolean(row["useFem"], `${at}.useFem`),
    spanM: normEn1992SnapshotGuardNumber(row["spanM"], `${at}.spanM`),
    udlKnM: normEn1992SnapshotGuardNumber(row["udlKnM"], `${at}.udlKnM`),
    fireRating: normEn1992SnapshotGuardString(row["fireRating"], `${at}.fireRating`),
    providedAxisDistanceMm: normEn1992SnapshotGuardNumber(row["providedAxisDistanceMm"], `${at}.providedAxisDistanceMm`),
    bridgeSigmaCMpa: normEn1992SnapshotGuardNumber(row["bridgeSigmaCMpa"], `${at}.bridgeSigmaCMpa`),
    bridgeDeltaSigmaSMpa: normEn1992SnapshotGuardNumber(row["bridgeDeltaSigmaSMpa"], `${at}.bridgeDeltaSigmaSMpa`),
    tightnessClass: normEn1992SnapshotGuardString(row["tightnessClass"], `${at}.tightnessClass`),
    hdOverH: normEn1992SnapshotGuardNumber(row["hdOverH"], `${at}.hdOverH`),
    liquidSigmaSMpa: normEn1992SnapshotGuardNumber(row["liquidSigmaSMpa"], `${at}.liquidSigmaSMpa`),
    liquidRhoPEff: normEn1992SnapshotGuardNumber(row["liquidRhoPEff"], `${at}.liquidRhoPEff`),
    liquidFCtEffMpa: normEn1992SnapshotGuardNumber(row["liquidFCtEffMpa"], `${at}.liquidFCtEffMpa`),
    liquidESMpa: normEn1992SnapshotGuardNumber(row["liquidESMpa"], `${at}.liquidESMpa`),
    liquidSRMaxMm: normEn1992SnapshotGuardNumber(row["liquidSRMaxMm"], `${at}.liquidSRMaxMm`),
    anchorHEfMm: normEn1992SnapshotGuardNumber(row["anchorHEfMm"], `${at}.anchorHEfMm`),
    anchorCracked: normEn1992SnapshotGuardBoolean(row["anchorCracked"], `${at}.anchorCracked`),
    anchorFUkMpa: normEn1992SnapshotGuardNumber(row["anchorFUkMpa"], `${at}.anchorFUkMpa`),
    anchorFYkMpa: normEn1992SnapshotGuardNumber(row["anchorFYkMpa"], `${at}.anchorFYkMpa`),
    anchorASMm2: normEn1992SnapshotGuardNumber(row["anchorASMm2"], `${at}.anchorASMm2`),
    anchorDMm: normEn1992SnapshotGuardNumber(row["anchorDMm"], `${at}.anchorDMm`),
    anchorC1Mm: normEn1992SnapshotGuardNumber(row["anchorC1Mm"], `${at}.anchorC1Mm`),
    anchorNEdKn: normEn1992SnapshotGuardNumber(row["anchorNEdKn"], `${at}.anchorNEdKn`),
    anchorVEdKn: normEn1992SnapshotGuardNumber(row["anchorVEdKn"], `${at}.anchorVEdKn`),
  };
}
