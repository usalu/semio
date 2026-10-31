/** 💰️ The inferred quantity billed by one authored cost item, in SI units. */
export type CostBasis = "Count" | "Length" | "GrossArea" | "NetArea" | "SurfaceArea" | "GrossSideArea" | "NetSideArea" | "GrossVolume" | "NetVolume" | "Mass";

/** 🧾️ Authored pricing with an explicit currency label and line rounding precision. */
export interface CostItem {
  name: string;
  currency: string;
  basis: CostBasis;
  unit_cost: number;
  precision: number;
}

/** 🔗️ An independently addressable type-to-item link; factor accounts for waste or multiple applications. */
export interface TypeCostLink {
  type_id: string;
  item: string;
  factor: number;
}

//#region 🚪️Parsers
/** 🛑️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class bimModelCostsGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const bimModelCostsGuardReject = (at: string, why: string): never => {
  throw new bimModelCostsGuardRefusal(at, why);
};

type bimModelCostsGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type bimModelCostsGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number; readonly exclusiveMinimum?: number; readonly exclusiveMaximum?: number };
type bimModelCostsGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const bimModelCostsGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : bimModelCostsGuardReject(at, "value is not an object");
export const bimModelCostsGuardClosed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) bimModelCostsGuardReject(`${at}.${key}`, "unknown property");
  return row;
};
export const bimModelCostsGuardExactObject = (value: unknown, at: string, fields: readonly string[], required: readonly string[] = fields): Readonly<Record<string, unknown>> => {
  const row = bimModelCostsGuardClosed(bimModelCostsGuardObject(value, at), at, fields);
  for (const field of required) if (!Object.hasOwn(row, field)) bimModelCostsGuardReject(`${at}.${field}`, "field is missing");
  return row;
};
export const bimModelCostsGuardArray = (value: unknown, at: string, bounds: bimModelCostsGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return bimModelCostsGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) bimModelCostsGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) bimModelCostsGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const bimModelCostsGuardString = (value: unknown, at: string, bounds: bimModelCostsGuardTextBounds = {}): string => {
  if (typeof value !== "string") return bimModelCostsGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) bimModelCostsGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) bimModelCostsGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) bimModelCostsGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const bimModelCostsGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : bimModelCostsGuardReject(at, "value is not a boolean"));
export const bimModelCostsGuardNumber = (value: unknown, at: string, bounds: bimModelCostsGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return bimModelCostsGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) bimModelCostsGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) bimModelCostsGuardReject(at, `number is above ${bounds.maximum}`);
  if (bounds.exclusiveMinimum !== undefined && value <= bounds.exclusiveMinimum) bimModelCostsGuardReject(at, `number is not above ${bounds.exclusiveMinimum}`);
  if (bounds.exclusiveMaximum !== undefined && value >= bounds.exclusiveMaximum) bimModelCostsGuardReject(at, `number is not below ${bounds.exclusiveMaximum}`);
  return value;
};
export const bimModelCostsGuardInteger = (value: unknown, at: string, bounds: bimModelCostsGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? bimModelCostsGuardNumber(value, at, bounds) : bimModelCostsGuardReject(at, "value is not an integer");
export const bimModelCostsGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : bimModelCostsGuardReject(at, `value is not one of ${members.join(", ")}`);
export const bimModelCostsGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : bimModelCostsGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseCostBasis(value: unknown, at = "$"): CostBasis {
  return bimModelCostsGuardMember(value, `${at}`, ["Count", "Length", "GrossArea", "NetArea", "SurfaceArea", "GrossSideArea", "NetSideArea", "GrossVolume", "NetVolume", "Mass"] as const);
}

export function parseCostItem(value: unknown, at = "$"): CostItem {
  const row = bimModelCostsGuardClosed(bimModelCostsGuardObject(value, at), at, ["name", "currency", "basis", "unit_cost", "precision"]);
  return {
    name: bimModelCostsGuardString(row["name"], `${at}.name`, {"pattern": "\\S"}),
    currency: bimModelCostsGuardString(row["currency"], `${at}.currency`, {"pattern": "\\S"}),
    basis: parseCostBasis(row["basis"], `${at}.basis`),
    unit_cost: bimModelCostsGuardNumber(row["unit_cost"], `${at}.unit_cost`, {"minimum": 0}),
    precision: bimModelCostsGuardInteger(row["precision"], `${at}.precision`, {"minimum": 0, "maximum": 9}),
  };
}

export function parseTypeCostLink(value: unknown, at = "$"): TypeCostLink {
  const row = bimModelCostsGuardClosed(bimModelCostsGuardObject(value, at), at, ["type_id", "item", "factor"]);
  return {
    type_id: bimModelCostsGuardString(row["type_id"], `${at}.type_id`, {"pattern": "\\S"}),
    item: bimModelCostsGuardString(row["item"], `${at}.item`, {"pattern": "\\S"}),
    factor: bimModelCostsGuardNumber(row["factor"], `${at}.factor`, {"minimum": 0}),
  };
}
