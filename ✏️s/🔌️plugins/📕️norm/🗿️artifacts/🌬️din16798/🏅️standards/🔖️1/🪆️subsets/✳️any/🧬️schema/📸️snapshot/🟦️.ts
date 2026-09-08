/** 🧬️ Din16798 snapshot schema — artifact-lane fields only. */

export interface Din16798Snapshot {
  /** @state artifact */
  annex: string;
  /** @state artifact */
  occupancy: string;
  /** @state artifact */
  comfortCategory: string;
  /** @state artifact */
  tOpC: number;
  /** @state artifact */
  rhPercent: number;
  /** @state artifact */
  airSpeedMS: number;
  /** @state artifact */
  thetaRmC: number;
  /** @state artifact */
  co2Ppm: number;
  /** @state artifact */
  dfPercent: number;
  /** @state artifact */
  lAeqDb: number;
  /** @state artifact */
  persons: number;
  /** @state artifact */
  idaClass: string;
  /** @state artifact */
  ventilationM3H: number;
  /** @state artifact */
  floorAreaM2: number;
  /** @state artifact */
  bedrooms: number;
  /** @state artifact */
  dwellingVentilationM3H: number;
  /** @state artifact */
  occupants: number;
  /** @state artifact */
  residentialVentilationM3H: number;
  /** @state artifact */
  sfpWM3S: number;
  /** @state artifact */
  sfpRequiredClass: number;
  /** @state artifact */
  heatRecoveryEta: number;
  /** @state artifact */
  heatRecoveryEtaMin: number;
  /** @state artifact */
  systemType: string;
  /** @state artifact */
  yearsSinceInspection: number;
  /** @state artifact */
  humidificationRequiredKgH: number;
  /** @state artifact */
  humidificationProvidedKgH: number;
  /** @state artifact */
  fanQVM3S: number;
  /** @state artifact */
  fanTRunH: number;
  /** @state artifact */
  fanEnergyReferenceKwh: number;
  /** @state artifact */
  nightSetbackK: number;
  /** @state artifact */
  hrMDotKgS: number;
  /** @state artifact */
  hrCpJKgk: number;
  /** @state artifact */
  hrDeltaTC: number;
  /** @state artifact */
  hrTH: number;
  /** @state artifact */
  hrSavingsReferenceKwh: number;
  /** @state artifact */
  n50HInv: number;
  /** @state artifact */
  volumeM3: number;
  /** @state artifact */
  infiltrationAllowanceM3H: number;
  /** @state artifact */
  cellarAreaM2: number;
  /** @state artifact */
  cellarVentilationM3H: number;
  /** @state artifact */
  hTrWK: number;
  /** @state artifact */
  hVeWK: number;
  /** @state artifact */
  thetaEC: number;
  /** @state artifact */
  thetaSetC: number;
  /** @state artifact */
  coolingDeltaTH: number;
  /** @state artifact */
  coolingGainsKwh: number;
  /** @state artifact */
  coolingUtilizationFactor: number;
  /** @state artifact */
  coolingReferenceKwh: number;
  /** @state artifact */
  chillerType: string;
  /** @state artifact */
  eerActual: number;
  /** @state artifact */
  qCKwh: number;
  /** @state artifact */
  generationReferenceKwh: number;
  /** @state artifact */
  dataCenterSupplyC: number;
  /** @state artifact */
  hStWK: number;
  /** @state artifact */
  thetaStC: number;
  /** @state artifact */
  thetaAmbC: number;
  /** @state artifact */
  storageTH: number;
  /** @state artifact */
  storageAllowanceKwh: number;
  /** @state artifact */
  dhwDeliveryC: number;
  /** @state artifact */
  ductClass: string;
  /** @state artifact */
  ductTestPressurePa: number;
  /** @state artifact */
  ductLeakageM3SM2: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normDin16798SnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normDin16798SnapshotGuardReject = (at: string, why: string): never => {
  throw new normDin16798SnapshotGuardRefusal(at, why);
};

type normDin16798SnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normDin16798SnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normDin16798SnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normDin16798SnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normDin16798SnapshotGuardReject(at, "value is not an object");
export const normDin16798SnapshotGuardArray = (value: unknown, at: string, bounds: normDin16798SnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normDin16798SnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normDin16798SnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normDin16798SnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normDin16798SnapshotGuardString = (value: unknown, at: string, bounds: normDin16798SnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normDin16798SnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normDin16798SnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normDin16798SnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normDin16798SnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normDin16798SnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normDin16798SnapshotGuardReject(at, "value is not a boolean"));
export const normDin16798SnapshotGuardNumber = (value: unknown, at: string, bounds: normDin16798SnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normDin16798SnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normDin16798SnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normDin16798SnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normDin16798SnapshotGuardInteger = (value: unknown, at: string, bounds: normDin16798SnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normDin16798SnapshotGuardNumber(value, at, bounds) : normDin16798SnapshotGuardReject(at, "value is not an integer");
export const normDin16798SnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normDin16798SnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normDin16798SnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normDin16798SnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDin16798Snapshot(value: unknown, at = "$"): Din16798Snapshot {
  const row = normDin16798SnapshotGuardObject(value, at);
  return {
    annex: normDin16798SnapshotGuardString(row["annex"], `${at}.annex`),
    occupancy: normDin16798SnapshotGuardString(row["occupancy"], `${at}.occupancy`),
    comfortCategory: normDin16798SnapshotGuardString(row["comfortCategory"], `${at}.comfortCategory`),
    tOpC: normDin16798SnapshotGuardNumber(row["tOpC"], `${at}.tOpC`),
    rhPercent: normDin16798SnapshotGuardNumber(row["rhPercent"], `${at}.rhPercent`),
    airSpeedMS: normDin16798SnapshotGuardNumber(row["airSpeedMS"], `${at}.airSpeedMS`),
    thetaRmC: normDin16798SnapshotGuardNumber(row["thetaRmC"], `${at}.thetaRmC`),
    co2Ppm: normDin16798SnapshotGuardNumber(row["co2Ppm"], `${at}.co2Ppm`),
    dfPercent: normDin16798SnapshotGuardNumber(row["dfPercent"], `${at}.dfPercent`),
    lAeqDb: normDin16798SnapshotGuardNumber(row["lAeqDb"], `${at}.lAeqDb`),
    persons: normDin16798SnapshotGuardInteger(row["persons"], `${at}.persons`),
    idaClass: normDin16798SnapshotGuardString(row["idaClass"], `${at}.idaClass`),
    ventilationM3H: normDin16798SnapshotGuardNumber(row["ventilationM3H"], `${at}.ventilationM3H`),
    floorAreaM2: normDin16798SnapshotGuardNumber(row["floorAreaM2"], `${at}.floorAreaM2`),
    bedrooms: normDin16798SnapshotGuardInteger(row["bedrooms"], `${at}.bedrooms`),
    dwellingVentilationM3H: normDin16798SnapshotGuardNumber(row["dwellingVentilationM3H"], `${at}.dwellingVentilationM3H`),
    occupants: normDin16798SnapshotGuardInteger(row["occupants"], `${at}.occupants`),
    residentialVentilationM3H: normDin16798SnapshotGuardNumber(row["residentialVentilationM3H"], `${at}.residentialVentilationM3H`),
    sfpWM3S: normDin16798SnapshotGuardNumber(row["sfpWM3S"], `${at}.sfpWM3S`),
    sfpRequiredClass: normDin16798SnapshotGuardInteger(row["sfpRequiredClass"], `${at}.sfpRequiredClass`),
    heatRecoveryEta: normDin16798SnapshotGuardNumber(row["heatRecoveryEta"], `${at}.heatRecoveryEta`),
    heatRecoveryEtaMin: normDin16798SnapshotGuardNumber(row["heatRecoveryEtaMin"], `${at}.heatRecoveryEtaMin`),
    systemType: normDin16798SnapshotGuardString(row["systemType"], `${at}.systemType`),
    yearsSinceInspection: normDin16798SnapshotGuardInteger(row["yearsSinceInspection"], `${at}.yearsSinceInspection`),
    humidificationRequiredKgH: normDin16798SnapshotGuardNumber(row["humidificationRequiredKgH"], `${at}.humidificationRequiredKgH`),
    humidificationProvidedKgH: normDin16798SnapshotGuardNumber(row["humidificationProvidedKgH"], `${at}.humidificationProvidedKgH`),
    fanQVM3S: normDin16798SnapshotGuardNumber(row["fanQVM3S"], `${at}.fanQVM3S`),
    fanTRunH: normDin16798SnapshotGuardNumber(row["fanTRunH"], `${at}.fanTRunH`),
    fanEnergyReferenceKwh: normDin16798SnapshotGuardNumber(row["fanEnergyReferenceKwh"], `${at}.fanEnergyReferenceKwh`),
    nightSetbackK: normDin16798SnapshotGuardNumber(row["nightSetbackK"], `${at}.nightSetbackK`),
    hrMDotKgS: normDin16798SnapshotGuardNumber(row["hrMDotKgS"], `${at}.hrMDotKgS`),
    hrCpJKgk: normDin16798SnapshotGuardNumber(row["hrCpJKgk"], `${at}.hrCpJKgk`),
    hrDeltaTC: normDin16798SnapshotGuardNumber(row["hrDeltaTC"], `${at}.hrDeltaTC`),
    hrTH: normDin16798SnapshotGuardNumber(row["hrTH"], `${at}.hrTH`),
    hrSavingsReferenceKwh: normDin16798SnapshotGuardNumber(row["hrSavingsReferenceKwh"], `${at}.hrSavingsReferenceKwh`),
    n50HInv: normDin16798SnapshotGuardNumber(row["n50HInv"], `${at}.n50HInv`),
    volumeM3: normDin16798SnapshotGuardNumber(row["volumeM3"], `${at}.volumeM3`),
    infiltrationAllowanceM3H: normDin16798SnapshotGuardNumber(row["infiltrationAllowanceM3H"], `${at}.infiltrationAllowanceM3H`),
    cellarAreaM2: normDin16798SnapshotGuardNumber(row["cellarAreaM2"], `${at}.cellarAreaM2`),
    cellarVentilationM3H: normDin16798SnapshotGuardNumber(row["cellarVentilationM3H"], `${at}.cellarVentilationM3H`),
    hTrWK: normDin16798SnapshotGuardNumber(row["hTrWK"], `${at}.hTrWK`),
    hVeWK: normDin16798SnapshotGuardNumber(row["hVeWK"], `${at}.hVeWK`),
    thetaEC: normDin16798SnapshotGuardNumber(row["thetaEC"], `${at}.thetaEC`),
    thetaSetC: normDin16798SnapshotGuardNumber(row["thetaSetC"], `${at}.thetaSetC`),
    coolingDeltaTH: normDin16798SnapshotGuardNumber(row["coolingDeltaTH"], `${at}.coolingDeltaTH`),
    coolingGainsKwh: normDin16798SnapshotGuardNumber(row["coolingGainsKwh"], `${at}.coolingGainsKwh`),
    coolingUtilizationFactor: normDin16798SnapshotGuardNumber(row["coolingUtilizationFactor"], `${at}.coolingUtilizationFactor`),
    coolingReferenceKwh: normDin16798SnapshotGuardNumber(row["coolingReferenceKwh"], `${at}.coolingReferenceKwh`),
    chillerType: normDin16798SnapshotGuardString(row["chillerType"], `${at}.chillerType`),
    eerActual: normDin16798SnapshotGuardNumber(row["eerActual"], `${at}.eerActual`),
    qCKwh: normDin16798SnapshotGuardNumber(row["qCKwh"], `${at}.qCKwh`),
    generationReferenceKwh: normDin16798SnapshotGuardNumber(row["generationReferenceKwh"], `${at}.generationReferenceKwh`),
    dataCenterSupplyC: normDin16798SnapshotGuardNumber(row["dataCenterSupplyC"], `${at}.dataCenterSupplyC`),
    hStWK: normDin16798SnapshotGuardNumber(row["hStWK"], `${at}.hStWK`),
    thetaStC: normDin16798SnapshotGuardNumber(row["thetaStC"], `${at}.thetaStC`),
    thetaAmbC: normDin16798SnapshotGuardNumber(row["thetaAmbC"], `${at}.thetaAmbC`),
    storageTH: normDin16798SnapshotGuardNumber(row["storageTH"], `${at}.storageTH`),
    storageAllowanceKwh: normDin16798SnapshotGuardNumber(row["storageAllowanceKwh"], `${at}.storageAllowanceKwh`),
    dhwDeliveryC: normDin16798SnapshotGuardNumber(row["dhwDeliveryC"], `${at}.dhwDeliveryC`),
    ductClass: normDin16798SnapshotGuardString(row["ductClass"], `${at}.ductClass`),
    ductTestPressurePa: normDin16798SnapshotGuardNumber(row["ductTestPressurePa"], `${at}.ductTestPressurePa`),
    ductLeakageM3SM2: normDin16798SnapshotGuardNumber(row["ductLeakageM3SM2"], `${at}.ductLeakageM3SM2`),
  };
}
