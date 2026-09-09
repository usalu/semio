/** 🧬️ En1990 diff schema — sparse field delta. */

export interface En1990Diff {
  /** @state artifact */
  gK?: number;
  /** @state artifact */
  qK?: ArtifactChildHandle;
  /** @state artifact */
  resistanceKn?: number;
  /** @state artifact */
  consequenceClass?: number;
  /** @state artifact */
  annex?: string;
  /** @state artifact */
  seismicAEdKn?: number;
}

/** 🌉️ Opaque mirror of `store::os_io::ArtifactRef` — a cross-cutting framework identity type, out of
 *  this facet's own domain. */
export interface ArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface ArtifactRef {
  artifactId: string;
  dialect: ArtifactDialect;
}

/** 🌉️ Mirrors `store::ArtifactChild<S>` (`#[serde(rename_all = "camelCase")]`, `child_id`/`target`
 *  fields only — the `local_owner`/`PhantomData<S>` fields are `#[serde(skip)]`). */
export interface ArtifactChildHandle {
  childId: string;
  target: ArtifactRef;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class normEn1990DiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const normEn1990DiffGuardReject = (at: string, why: string): never => {
  throw new normEn1990DiffGuardRefusal(at, why);
};

type normEn1990DiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type normEn1990DiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type normEn1990DiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const normEn1990DiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normEn1990DiffGuardReject(at, "value is not an object");
export const normEn1990DiffGuardArray = (value: unknown, at: string, bounds: normEn1990DiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return normEn1990DiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) normEn1990DiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) normEn1990DiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const normEn1990DiffGuardString = (value: unknown, at: string, bounds: normEn1990DiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return normEn1990DiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) normEn1990DiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) normEn1990DiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) normEn1990DiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const normEn1990DiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : normEn1990DiffGuardReject(at, "value is not a boolean"));
export const normEn1990DiffGuardNumber = (value: unknown, at: string, bounds: normEn1990DiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return normEn1990DiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) normEn1990DiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) normEn1990DiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const normEn1990DiffGuardInteger = (value: unknown, at: string, bounds: normEn1990DiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? normEn1990DiffGuardNumber(value, at, bounds) : normEn1990DiffGuardReject(at, "value is not an integer");
export const normEn1990DiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : normEn1990DiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const normEn1990DiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : normEn1990DiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface En1990QkList {
  readonly values: readonly En1990QkEntry[];
}
