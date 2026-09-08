/** 🧬️ En1993 snapshot schema — artifact-lane fields only. */

export interface En1993Snapshot {
  /** @state artifact */
  annex: string;
  /** @state artifact */
  nEdKn: number;
  /** @state artifact */
  mEdKnm: number;
  /** @state artifact */
  vEdKn: number;
  /** @state artifact */
  aMm2: number;
  /** @state artifact */
  aVMm2: number;
  /** @state artifact */
  wPlMm3: number;
  /** @state artifact */
  fYMpa: number;
  /** @state artifact */
  fUMpa: number;
  /** @state artifact */
  chi: number;
  /** @state artifact */
  aNetMm2: number;
  /** @state artifact */
  tensionNEdKn: number;
  /** @state artifact */
  fireThicknessMm: number;
  /** @state artifact */
  fireRating: string;
  /** @state artifact */
  fireMassivity: number;
  /** @state artifact */
  fireMu0: number;
  /** @state artifact */
  fireDesignTemperatureC: number;
  /** @state artifact */
  cfBBarMm: number;
  /** @state artifact */
  cfTMm: number;
  /** @state artifact */
  cfKSigma: number;
  /** @state artifact */
  cfPsi: number;
  /** @state artifact */
  cfNEdKn: number;
  /** @state artifact */
  cfGrossResistanceKn: number;
  /** @state artifact */
  stainlessMEdKnm: number;
  /** @state artifact */
  stainlessWPlMm3: number;
  /** @state artifact */
  stainlessFYMpa: number;
  /** @state artifact */
  platedLambdaP: number;
  /** @state artifact */
  platedSigmaEdMpa: number;
  /** @state artifact */
  siloTMm: number;
  /** @state artifact */
  siloRMm: number;
  /** @state artifact */
  shellSigmaXEdMpa: number;
  /** @state artifact */
  siloK: number;
  /** @state artifact */
  siloGammaKnM3: number;
  /** @state artifact */
  siloDepthM: number;
  /** @state artifact */
  boltFEdKn: number;
  /** @state artifact */
  boltNBolts: number;
  /** @state artifact */
  boltASMm2: number;
  /** @state artifact */
  boltE1Mm: number;
  /** @state artifact */
  boltE2Mm: number;
  /** @state artifact */
  boltD0Mm: number;
  /** @state artifact */
  boltDMm: number;
  /** @state artifact */
  boltTMm: number;
  /** @state artifact */
  boltFUMpa: number;
  /** @state artifact */
  boltFUbMpa: number;
  /** @state artifact */
  weldAMm: number;
  /** @state artifact */
  weldLMm: number;
  /** @state artifact */
  weldFUMpa: number;
  /** @state artifact */
  weldSteelGrade: string;
  /** @state artifact */
  weldFEdKn: number;
  /** @state artifact */
  deltaSigmaMpa: number;
  /** @state artifact */
  fatigueCategory: number;
  /** @state artifact */
  fatigueMethod: string;
  /** @state artifact */
  t10SteelSubgrade: string;
  /** @state artifact */
  t10ActualThicknessMm: number;
  /** @state artifact */
  t10TEdC: number;
  /** @state artifact */
  tensionComponentFUkKn: number;
  /** @state artifact */
  tensionComponentFKKn: number;
  /** @state artifact */
  tensionComponentNEdKn: number;
  /** @state artifact */
  hssWElMm3: number;
  /** @state artifact */
  hssFYMpa: number;
  /** @state artifact */
  hssSectionClass: number;
  /** @state artifact */
  hssMEdKnm: number;
  /** @state artifact */
  bridgeLambda: number;
  /** @state artifact */
  bridgePhi2: number;
  /** @state artifact */
  bridgeDeltaSigmaPMpa: number;
  /** @state artifact */
  towerWindFactor: number;
  /** @state artifact */
  towerNEdKn: number;
  /** @state artifact */
  pileSigmaMpa: number;
  /** @state artifact */
  pileKRed: number;
  /** @state artifact */
  pileNEdKn: number;
  /** @state artifact */
  craneFZEdKn: number;
  /** @state artifact */
  craneWheelContactLengthMm: number;
  /** @state artifact */
  craneDispersionMm: number;
  /** @state artifact */
  craneTWMm: number;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1993SnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1993SnapshotGuardReject = (at: string, why: string): never => {
  throw new normEn1993SnapshotGuardRefusal(at, why);
};

type normEn1993SnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1993SnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1993SnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1993SnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1993SnapshotGuardReject(at, "value is not an object");
export const normEn1993SnapshotGuardArray = (value: unknown, at: string, bounds: normEn1993SnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1993SnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1993SnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1993SnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1993SnapshotGuardString = (value: unknown, at: string, bounds: normEn1993SnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1993SnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1993SnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1993SnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1993SnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1993SnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1993SnapshotGuardReject(at, "value is not a boolean"));
export const normEn1993SnapshotGuardNumber = (value: unknown, at: string, bounds: normEn1993SnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1993SnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1993SnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1993SnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1993SnapshotGuardInteger = (value: unknown, at: string, bounds: normEn1993SnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1993SnapshotGuardNumber(value, at, bounds) : normEn1993SnapshotGuardReject(at, "value is not an integer");
export const normEn1993SnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1993SnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1993SnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1993SnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEn1993Snapshot(value: unknown, at = "$"): En1993Snapshot {
  const row = normEn1993SnapshotGuardObject(value, at);
  return {
    annex: normEn1993SnapshotGuardString(row["annex"], `${at}.annex`),
    nEdKn: normEn1993SnapshotGuardNumber(row["nEdKn"], `${at}.nEdKn`),
    mEdKnm: normEn1993SnapshotGuardNumber(row["mEdKnm"], `${at}.mEdKnm`),
    vEdKn: normEn1993SnapshotGuardNumber(row["vEdKn"], `${at}.vEdKn`),
    aMm2: normEn1993SnapshotGuardNumber(row["aMm2"], `${at}.aMm2`),
    aVMm2: normEn1993SnapshotGuardNumber(row["aVMm2"], `${at}.aVMm2`),
    wPlMm3: normEn1993SnapshotGuardNumber(row["wPlMm3"], `${at}.wPlMm3`),
    fYMpa: normEn1993SnapshotGuardNumber(row["fYMpa"], `${at}.fYMpa`),
    fUMpa: normEn1993SnapshotGuardNumber(row["fUMpa"], `${at}.fUMpa`),
    chi: normEn1993SnapshotGuardNumber(row["chi"], `${at}.chi`),
    aNetMm2: normEn1993SnapshotGuardNumber(row["aNetMm2"], `${at}.aNetMm2`),
    tensionNEdKn: normEn1993SnapshotGuardNumber(row["tensionNEdKn"], `${at}.tensionNEdKn`),
    fireThicknessMm: normEn1993SnapshotGuardNumber(row["fireThicknessMm"], `${at}.fireThicknessMm`),
    fireRating: normEn1993SnapshotGuardString(row["fireRating"], `${at}.fireRating`),
    fireMassivity: normEn1993SnapshotGuardNumber(row["fireMassivity"], `${at}.fireMassivity`),
    fireMu0: normEn1993SnapshotGuardNumber(row["fireMu0"], `${at}.fireMu0`),
    fireDesignTemperatureC: normEn1993SnapshotGuardNumber(row["fireDesignTemperatureC"], `${at}.fireDesignTemperatureC`),
    cfBBarMm: normEn1993SnapshotGuardNumber(row["cfBBarMm"], `${at}.cfBBarMm`),
    cfTMm: normEn1993SnapshotGuardNumber(row["cfTMm"], `${at}.cfTMm`),
    cfKSigma: normEn1993SnapshotGuardNumber(row["cfKSigma"], `${at}.cfKSigma`),
    cfPsi: normEn1993SnapshotGuardNumber(row["cfPsi"], `${at}.cfPsi`),
    cfNEdKn: normEn1993SnapshotGuardNumber(row["cfNEdKn"], `${at}.cfNEdKn`),
    cfGrossResistanceKn: normEn1993SnapshotGuardNumber(row["cfGrossResistanceKn"], `${at}.cfGrossResistanceKn`),
    stainlessMEdKnm: normEn1993SnapshotGuardNumber(row["stainlessMEdKnm"], `${at}.stainlessMEdKnm`),
    stainlessWPlMm3: normEn1993SnapshotGuardNumber(row["stainlessWPlMm3"], `${at}.stainlessWPlMm3`),
    stainlessFYMpa: normEn1993SnapshotGuardNumber(row["stainlessFYMpa"], `${at}.stainlessFYMpa`),
    platedLambdaP: normEn1993SnapshotGuardNumber(row["platedLambdaP"], `${at}.platedLambdaP`),
    platedSigmaEdMpa: normEn1993SnapshotGuardNumber(row["platedSigmaEdMpa"], `${at}.platedSigmaEdMpa`),
    siloTMm: normEn1993SnapshotGuardNumber(row["siloTMm"], `${at}.siloTMm`),
    siloRMm: normEn1993SnapshotGuardNumber(row["siloRMm"], `${at}.siloRMm`),
    shellSigmaXEdMpa: normEn1993SnapshotGuardNumber(row["shellSigmaXEdMpa"], `${at}.shellSigmaXEdMpa`),
    siloK: normEn1993SnapshotGuardNumber(row["siloK"], `${at}.siloK`),
    siloGammaKnM3: normEn1993SnapshotGuardNumber(row["siloGammaKnM3"], `${at}.siloGammaKnM3`),
    siloDepthM: normEn1993SnapshotGuardNumber(row["siloDepthM"], `${at}.siloDepthM`),
    boltFEdKn: normEn1993SnapshotGuardNumber(row["boltFEdKn"], `${at}.boltFEdKn`),
    boltNBolts: normEn1993SnapshotGuardInteger(row["boltNBolts"], `${at}.boltNBolts`),
    boltASMm2: normEn1993SnapshotGuardNumber(row["boltASMm2"], `${at}.boltASMm2`),
    boltE1Mm: normEn1993SnapshotGuardNumber(row["boltE1Mm"], `${at}.boltE1Mm`),
    boltE2Mm: normEn1993SnapshotGuardNumber(row["boltE2Mm"], `${at}.boltE2Mm`),
    boltD0Mm: normEn1993SnapshotGuardNumber(row["boltD0Mm"], `${at}.boltD0Mm`),
    boltDMm: normEn1993SnapshotGuardNumber(row["boltDMm"], `${at}.boltDMm`),
    boltTMm: normEn1993SnapshotGuardNumber(row["boltTMm"], `${at}.boltTMm`),
    boltFUMpa: normEn1993SnapshotGuardNumber(row["boltFUMpa"], `${at}.boltFUMpa`),
    boltFUbMpa: normEn1993SnapshotGuardNumber(row["boltFUbMpa"], `${at}.boltFUbMpa`),
    weldAMm: normEn1993SnapshotGuardNumber(row["weldAMm"], `${at}.weldAMm`),
    weldLMm: normEn1993SnapshotGuardNumber(row["weldLMm"], `${at}.weldLMm`),
    weldFUMpa: normEn1993SnapshotGuardNumber(row["weldFUMpa"], `${at}.weldFUMpa`),
    weldSteelGrade: normEn1993SnapshotGuardString(row["weldSteelGrade"], `${at}.weldSteelGrade`),
    weldFEdKn: normEn1993SnapshotGuardNumber(row["weldFEdKn"], `${at}.weldFEdKn`),
    deltaSigmaMpa: normEn1993SnapshotGuardNumber(row["deltaSigmaMpa"], `${at}.deltaSigmaMpa`),
    fatigueCategory: normEn1993SnapshotGuardInteger(row["fatigueCategory"], `${at}.fatigueCategory`),
    fatigueMethod: normEn1993SnapshotGuardString(row["fatigueMethod"], `${at}.fatigueMethod`),
    t10SteelSubgrade: normEn1993SnapshotGuardString(row["t10SteelSubgrade"], `${at}.t10SteelSubgrade`),
    t10ActualThicknessMm: normEn1993SnapshotGuardNumber(row["t10ActualThicknessMm"], `${at}.t10ActualThicknessMm`),
    t10TEdC: normEn1993SnapshotGuardNumber(row["t10TEdC"], `${at}.t10TEdC`),
    tensionComponentFUkKn: normEn1993SnapshotGuardNumber(row["tensionComponentFUkKn"], `${at}.tensionComponentFUkKn`),
    tensionComponentFKKn: normEn1993SnapshotGuardNumber(row["tensionComponentFKKn"], `${at}.tensionComponentFKKn`),
    tensionComponentNEdKn: normEn1993SnapshotGuardNumber(row["tensionComponentNEdKn"], `${at}.tensionComponentNEdKn`),
    hssWElMm3: normEn1993SnapshotGuardNumber(row["hssWElMm3"], `${at}.hssWElMm3`),
    hssFYMpa: normEn1993SnapshotGuardNumber(row["hssFYMpa"], `${at}.hssFYMpa`),
    hssSectionClass: normEn1993SnapshotGuardInteger(row["hssSectionClass"], `${at}.hssSectionClass`),
    hssMEdKnm: normEn1993SnapshotGuardNumber(row["hssMEdKnm"], `${at}.hssMEdKnm`),
    bridgeLambda: normEn1993SnapshotGuardNumber(row["bridgeLambda"], `${at}.bridgeLambda`),
    bridgePhi2: normEn1993SnapshotGuardNumber(row["bridgePhi2"], `${at}.bridgePhi2`),
    bridgeDeltaSigmaPMpa: normEn1993SnapshotGuardNumber(row["bridgeDeltaSigmaPMpa"], `${at}.bridgeDeltaSigmaPMpa`),
    towerWindFactor: normEn1993SnapshotGuardNumber(row["towerWindFactor"], `${at}.towerWindFactor`),
    towerNEdKn: normEn1993SnapshotGuardNumber(row["towerNEdKn"], `${at}.towerNEdKn`),
    pileSigmaMpa: normEn1993SnapshotGuardNumber(row["pileSigmaMpa"], `${at}.pileSigmaMpa`),
    pileKRed: normEn1993SnapshotGuardNumber(row["pileKRed"], `${at}.pileKRed`),
    pileNEdKn: normEn1993SnapshotGuardNumber(row["pileNEdKn"], `${at}.pileNEdKn`),
    craneFZEdKn: normEn1993SnapshotGuardNumber(row["craneFZEdKn"], `${at}.craneFZEdKn`),
    craneWheelContactLengthMm: normEn1993SnapshotGuardNumber(row["craneWheelContactLengthMm"], `${at}.craneWheelContactLengthMm`),
    craneDispersionMm: normEn1993SnapshotGuardNumber(row["craneDispersionMm"], `${at}.craneDispersionMm`),
    craneTWMm: normEn1993SnapshotGuardNumber(row["craneTWMm"], `${at}.craneTWMm`),
  };
}
