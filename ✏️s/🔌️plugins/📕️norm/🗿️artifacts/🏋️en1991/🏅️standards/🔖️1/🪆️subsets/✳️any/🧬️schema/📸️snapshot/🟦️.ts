/** 🧬️ En1991 snapshot schema — artifact-lane fields only. */

export interface En1991Snapshot {
  /** @state artifact */
  areaM2: number;
  /** @state artifact */
  category: string;
  /** @state artifact */
  annex: string;
  /** @state artifact */
  selfWeightMaterial: string;
  /** @state artifact */
  selfWeightThicknessM: number;
  /** @state artifact */
  assumedGKKnM2: number;
  /** @state artifact */
  fireCurve: string;
  /** @state artifact */
  fireResistanceMin: number;
  /** @state artifact */
  fireMemberCapacityC: number;
  /** @state artifact */
  snowZone: number;
  /** @state artifact */
  snowAltitudeM: number;
  /** @state artifact */
  enSKKnM2: number;
  /** @state artifact */
  windZone: number;
  /** @state artifact */
  enVBMS: number;
  /** @state artifact */
  deltaTK: number;
  /** @state artifact */
  constructionActivity: string;
  /** @state artifact */
  accidentalMassT: number;
  /** @state artifact */
  accidentalSpeedKmH: number;
  /** @state artifact */
  bridgeLane: number;
  /** @state artifact */
  bridgeSpanM: number;
  /** @state artifact */
  bridgeLaneWidthM: number;
  /** @state artifact */
  bridgeMomentResistanceKnm: number;
  /** @state artifact */
  craneClass: string;
  /** @state artifact */
  hoistClass: string;
  /** @state artifact */
  hoistingSpeedMS: number;
  /** @state artifact */
  siloBulkDensityKnM3: number;
  /** @state artifact */
  siloHeightM: number;
  /** @state artifact */
  siloHydraulicRadiusM: number;
  /** @state artifact */
  siloMu: number;
  /** @state artifact */
  siloK: number;
  /** @state artifact */
  cS: number;
  /** @state artifact */
  cD: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1991SnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1991SnapshotGuardReject = (at: string, why: string): never => {
  throw new normEn1991SnapshotGuardRefusal(at, why);
};

type normEn1991SnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1991SnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1991SnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1991SnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1991SnapshotGuardReject(at, "value is not an object");
export const normEn1991SnapshotGuardArray = (value: unknown, at: string, bounds: normEn1991SnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1991SnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1991SnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1991SnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1991SnapshotGuardString = (value: unknown, at: string, bounds: normEn1991SnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1991SnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1991SnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1991SnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1991SnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1991SnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1991SnapshotGuardReject(at, "value is not a boolean"));
export const normEn1991SnapshotGuardNumber = (value: unknown, at: string, bounds: normEn1991SnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1991SnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1991SnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1991SnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1991SnapshotGuardInteger = (value: unknown, at: string, bounds: normEn1991SnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1991SnapshotGuardNumber(value, at, bounds) : normEn1991SnapshotGuardReject(at, "value is not an integer");
export const normEn1991SnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1991SnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1991SnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1991SnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1991Snapshot(value: unknown, at = "$"): En1991Snapshot {
  const row = normEn1991SnapshotGuardObject(value, at);
  return {
    areaM2: normEn1991SnapshotGuardNumber(row["areaM2"], `${at}.areaM2`),
    category: normEn1991SnapshotGuardString(row["category"], `${at}.category`),
    annex: normEn1991SnapshotGuardString(row["annex"], `${at}.annex`),
    selfWeightMaterial: normEn1991SnapshotGuardString(row["selfWeightMaterial"], `${at}.selfWeightMaterial`),
    selfWeightThicknessM: normEn1991SnapshotGuardNumber(row["selfWeightThicknessM"], `${at}.selfWeightThicknessM`),
    assumedGKKnM2: normEn1991SnapshotGuardNumber(row["assumedGKKnM2"], `${at}.assumedGKKnM2`),
    fireCurve: normEn1991SnapshotGuardString(row["fireCurve"], `${at}.fireCurve`),
    fireResistanceMin: normEn1991SnapshotGuardNumber(row["fireResistanceMin"], `${at}.fireResistanceMin`),
    fireMemberCapacityC: normEn1991SnapshotGuardNumber(row["fireMemberCapacityC"], `${at}.fireMemberCapacityC`),
    snowZone: normEn1991SnapshotGuardInteger(row["snowZone"], `${at}.snowZone`),
    snowAltitudeM: normEn1991SnapshotGuardNumber(row["snowAltitudeM"], `${at}.snowAltitudeM`),
    enSKKnM2: normEn1991SnapshotGuardNumber(row["enSKKnM2"], `${at}.enSKKnM2`),
    windZone: normEn1991SnapshotGuardInteger(row["windZone"], `${at}.windZone`),
    enVBMS: normEn1991SnapshotGuardNumber(row["enVBMS"], `${at}.enVBMS`),
    deltaTK: normEn1991SnapshotGuardNumber(row["deltaTK"], `${at}.deltaTK`),
    constructionActivity: normEn1991SnapshotGuardString(row["constructionActivity"], `${at}.constructionActivity`),
    accidentalMassT: normEn1991SnapshotGuardNumber(row["accidentalMassT"], `${at}.accidentalMassT`),
    accidentalSpeedKmH: normEn1991SnapshotGuardNumber(row["accidentalSpeedKmH"], `${at}.accidentalSpeedKmH`),
    bridgeLane: normEn1991SnapshotGuardInteger(row["bridgeLane"], `${at}.bridgeLane`),
    bridgeSpanM: normEn1991SnapshotGuardNumber(row["bridgeSpanM"], `${at}.bridgeSpanM`),
    bridgeLaneWidthM: normEn1991SnapshotGuardNumber(row["bridgeLaneWidthM"], `${at}.bridgeLaneWidthM`),
    bridgeMomentResistanceKnm: normEn1991SnapshotGuardNumber(row["bridgeMomentResistanceKnm"], `${at}.bridgeMomentResistanceKnm`),
    craneClass: normEn1991SnapshotGuardString(row["craneClass"], `${at}.craneClass`),
    hoistClass: normEn1991SnapshotGuardString(row["hoistClass"], `${at}.hoistClass`),
    hoistingSpeedMS: normEn1991SnapshotGuardNumber(row["hoistingSpeedMS"], `${at}.hoistingSpeedMS`),
    siloBulkDensityKnM3: normEn1991SnapshotGuardNumber(row["siloBulkDensityKnM3"], `${at}.siloBulkDensityKnM3`),
    siloHeightM: normEn1991SnapshotGuardNumber(row["siloHeightM"], `${at}.siloHeightM`),
    siloHydraulicRadiusM: normEn1991SnapshotGuardNumber(row["siloHydraulicRadiusM"], `${at}.siloHydraulicRadiusM`),
    siloMu: normEn1991SnapshotGuardNumber(row["siloMu"], `${at}.siloMu`),
    siloK: normEn1991SnapshotGuardNumber(row["siloK"], `${at}.siloK`),
    cS: normEn1991SnapshotGuardNumber(row["cS"], `${at}.cS`),
    cD: normEn1991SnapshotGuardNumber(row["cD"], `${at}.cD`),
  };
}
