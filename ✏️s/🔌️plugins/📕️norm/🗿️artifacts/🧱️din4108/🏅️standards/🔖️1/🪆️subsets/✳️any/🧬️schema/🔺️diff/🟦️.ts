/** 🧬️ Din4108 diff schema — sparse field delta. */

export interface Din4108Diff {
  /** @state artifact */
  artifact?: Din4108Artifact;
  /** @state artifact */
  category?: string;
  /** @state artifact */
  layers?: Din4108LayerList;
  /** @state artifact */
  climate?: string;
  /** @state artifact */
  airtightnessN50?: number;
  /** @state artifact */
  psiTimesLSum?: number;
  /** @state artifact */
  rhInt?: number;
  /** @state artifact */
  catalogId?: string;
  /** @state artifact */
  materialId?: string;
  /** @state artifact */
  airtightnessClass?: string;
  /** @state artifact */
  tIntC?: number;
  /** @state artifact */
  solarAbsorptance?: number;
  /** @state artifact */
  irradianceWM2?: number;
  /** @state artifact */
  moistureMuExterior?: number;
  /** @state artifact */
  moistureMuInterior?: number;
  /** @state artifact */
  envelopeAreaM2?: number;
  /** @state artifact */
  bb2DetailsConform?: boolean;
  /** @state artifact */
  applicationType?: string;
  /** @state artifact */
  declaredApplicationClass?: string;
  /** @state presence */
  selectedCheckIndex?: number | null;
}

export interface Din4108Artifact {
  category: string;
  layers: Din4108LayerDocument[];
  climate: string;
  airtightnessN50: number;
  psiTimesLSum: number;
  rhInt: number;
  catalogId: string;
  materialId: string;
  airtightnessClass: string;
  tIntC: number;
  solarAbsorptance: number;
  irradianceWM2: number;
  moistureMuExterior: number;
  moistureMuInterior: number;
  envelopeAreaM2: number;
  bb2DetailsConform: boolean;
  applicationType: string;
  declaredApplicationClass: string;
  selectedCheckIndex?: number | null;
}
export interface Din4108LayerDocument { thicknessM: number; lambdaWMk: number; }

export interface Din4108LayerList { values: Din4108LayerDocument[]; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normDin4108DiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normDin4108DiffGuardReject = (at: string, why: string): never => {
  throw new normDin4108DiffGuardRefusal(at, why);
};

type normDin4108DiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normDin4108DiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normDin4108DiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normDin4108DiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normDin4108DiffGuardReject(at, "value is not an object");
export const normDin4108DiffGuardArray = (value: unknown, at: string, bounds: normDin4108DiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normDin4108DiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normDin4108DiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normDin4108DiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normDin4108DiffGuardString = (value: unknown, at: string, bounds: normDin4108DiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normDin4108DiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normDin4108DiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normDin4108DiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normDin4108DiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normDin4108DiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normDin4108DiffGuardReject(at, "value is not a boolean"));
export const normDin4108DiffGuardNumber = (value: unknown, at: string, bounds: normDin4108DiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normDin4108DiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normDin4108DiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normDin4108DiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normDin4108DiffGuardInteger = (value: unknown, at: string, bounds: normDin4108DiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normDin4108DiffGuardNumber(value, at, bounds) : normDin4108DiffGuardReject(at, "value is not an integer");
export const normDin4108DiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normDin4108DiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normDin4108DiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normDin4108DiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDin4108LayerList(value: unknown, at = "$"): Din4108LayerList {
  const row = normDin4108DiffGuardObject(value, at);
  return {
    values: normDin4108DiffGuardArray(row["values"], `${at}.values`).map((item, index) => normDin4108DiffGuardString(item, `${at}.values[${index}]`)),
  };
}
