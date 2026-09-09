/** 🧬️ Din4108 artifact schema — every field with its state class. */

export interface Din4108Artifact {
  /** @state artifact */
  category: string;
  /** @state artifact */
  layers: Din4108LayerDocument[];
  /** @state artifact */
  climate: string;
  /** @state artifact */
  airtightnessN50: number;
  /** @state artifact */
  psiTimesLSum: number;
  /** @state artifact */
  rhInt: number;
  /** @state artifact */
  catalogId: string;
  /** @state artifact */
  materialId: string;
  /** @state artifact */
  airtightnessClass: string;
  /** @state artifact */
  tIntC: number;
  /** @state artifact */
  solarAbsorptance: number;
  /** @state artifact */
  irradianceWM2: number;
  /** @state artifact */
  moistureMuExterior: number;
  /** @state artifact */
  moistureMuInterior: number;
  /** @state artifact */
  envelopeAreaM2: number;
  /** @state artifact */
  bb2DetailsConform: boolean;
  /** @state artifact */
  applicationType: string;
  /** @state artifact */
  declaredApplicationClass: string;
}
export interface Din4108LayerDocument { thicknessM: number; lambdaWMk: number; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normDin4108ArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normDin4108ArtifactGuardReject = (at: string, why: string): never => {
  throw new normDin4108ArtifactGuardRefusal(at, why);
};

type normDin4108ArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normDin4108ArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normDin4108ArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normDin4108ArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normDin4108ArtifactGuardReject(at, "value is not an object");
export const normDin4108ArtifactGuardArray = (value: unknown, at: string, bounds: normDin4108ArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normDin4108ArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normDin4108ArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normDin4108ArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normDin4108ArtifactGuardString = (value: unknown, at: string, bounds: normDin4108ArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normDin4108ArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normDin4108ArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normDin4108ArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normDin4108ArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normDin4108ArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normDin4108ArtifactGuardReject(at, "value is not a boolean"));
export const normDin4108ArtifactGuardNumber = (value: unknown, at: string, bounds: normDin4108ArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normDin4108ArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normDin4108ArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normDin4108ArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normDin4108ArtifactGuardInteger = (value: unknown, at: string, bounds: normDin4108ArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normDin4108ArtifactGuardNumber(value, at, bounds) : normDin4108ArtifactGuardReject(at, "value is not an integer");
export const normDin4108ArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normDin4108ArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normDin4108ArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normDin4108ArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDin4108LayerDocument(value: unknown, at = "$"): Din4108LayerDocument {
  const row = normDin4108ArtifactGuardObject(value, at);
  return {
    thicknessM: normDin4108ArtifactGuardNumber(row["thicknessM"], `${at}.thicknessM`),
    lambdaWMk: normDin4108ArtifactGuardNumber(row["lambdaWMk"], `${at}.lambdaWMk`),
  };
}
