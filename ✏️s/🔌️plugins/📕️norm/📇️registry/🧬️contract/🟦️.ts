/** 🧾️ Norm wire contract, TypeScript half: the readers every norm schema twin builds its `parse<Export>()` from.
 * A reader takes one decoded JSON position and returns a fresh value of its twin type, members in schema (wire) order,
 * or throws `NormWireRefusal` naming the position. Readers refuse whatever the position's schema refuses — types, closed
 * members, tags, enumerations, item counts and numeric bounds — and read an absent optional nullable member as `null`, as the
 * Rust `FromValue` default does.
 * @see ./🦀️.rs */

//#region 🚪️Wire
/** 🚫️ One refused wire position: where the instance breaks its contract and why. */
export class NormWireRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

/** 📖️ Reads one wire position into its typed twin, refusing whatever that position's structure refuses. */
export type NormWireReader<T> = (value: unknown, at?: string) => T;

/** 🧷️ One object member: its reader and whether the wire may omit it. */
export interface NormWireMember<T, Optional extends boolean = boolean> {
  readonly read: NormWireReader<T>;
  readonly optional: Optional;
  readonly missing?: () => T;
}

/** 📏️ The numeric bounds one schema position states. */
export interface NormWireBounds {
  readonly minimum?: number;
  readonly maximum?: number;
  readonly exclusiveMinimum?: number;
  readonly exclusiveMaximum?: number;
}

/** 🗺️ The member table of `T`, in wire order: a required member reads `T[K]`, an omissible one reads it without `undefined`. */
export type NormWireMembers<T> = { readonly [K in keyof T]-?: {} extends Pick<T, K> ? NormWireMember<Exclude<T[K], undefined>, true> : NormWireMember<T[K], false> };

/** 🏷️ One reader per tag value of a union discriminated by the member `K`. */
export type NormWireVariants<T, K extends keyof T> = { readonly [V in T[K] & string]: NormWireReader<Extract<T, Readonly<Record<K, V>>>> };

/** 🎁️ One payload reader per variant key of an externally tagged union `{ <Variant>: payload }`. */
export type NormWireExternalVariants<T> = { readonly [K in T extends unknown ? keyof T & string : never]: NormWireReader<T extends Readonly<Record<K, infer P>> ? P : never> };

/** 🧱️ Any JSON value, kept as decoded. */
export type NormJson = null | boolean | number | string | readonly NormJson[] | { readonly [key: string]: NormJson };

export const normWireRefuse = (at: string, why: string): never => {
  throw new NormWireRefusal(at, why);
};
const normWireRecord = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : normWireRefuse(at, "value is not an object");

export const normWireString: NormWireReader<string> = (value, at = "$") => (typeof value === "string" ? value : normWireRefuse(at, "value is not a string"));
export const normWireBoolean: NormWireReader<boolean> = (value, at = "$") => (typeof value === "boolean" ? value : normWireRefuse(at, "value is not a boolean"));
export const normWireNumber: NormWireReader<number> = (value, at = "$") => (typeof value === "number" && Number.isFinite(value) ? value : normWireRefuse(at, "value is not a finite number"));
export const normWireInteger: NormWireReader<number> = (value, at = "$") => (typeof value === "number" && Number.isSafeInteger(value) ? value : normWireRefuse(at, "value is not a safe integer"));
export const normWireRange = (read: NormWireReader<number>, bounds: NormWireBounds): NormWireReader<number> => (value, at = "$") => {
  const number = read(value, at);
  const { minimum, maximum, exclusiveMinimum, exclusiveMaximum } = bounds;
  if ((minimum !== undefined && number < minimum) || (maximum !== undefined && number > maximum) || (exclusiveMinimum !== undefined && number <= exclusiveMinimum) || (exclusiveMaximum !== undefined && number >= exclusiveMaximum)) normWireRefuse(at, `value ${number} is outside ${JSON.stringify(bounds)}`);
  return number;
};
export const normWireLiteral = <const T extends readonly (string | number | boolean | null)[]>(...members: T): NormWireReader<T[number]> => (value, at = "$") =>
  members.includes(value as T[number]) ? (value as T[number]) : normWireRefuse(at, `value is not one of ${members.join(", ")}`);
export const normWireNullable = <T>(read: NormWireReader<T>): NormWireReader<T | null> => (value, at = "$") => (value === null ? null : read(value, at));
export const normWireArray = <T>(item: NormWireReader<T>, minItems = 0, maxItems = Number.POSITIVE_INFINITY): NormWireReader<T[]> => (value, at = "$") => {
  if (!Array.isArray(value)) return normWireRefuse(at, "value is not an array");
  if (value.length < minItems || value.length > maxItems) normWireRefuse(at, `array holds ${value.length} items, outside ${minItems}..=${maxItems}`);
  return value.map((entry, index) => item(entry, `${at}[${index}]`));
};
export const normWireJson: NormWireReader<NormJson> = (value, at = "$") => {
  if (value === null || typeof value === "string" || typeof value === "boolean") return value;
  if (typeof value === "number") return normWireNumber(value, at);
  if (Array.isArray(value)) return value.map((entry, index) => normWireJson(entry, `${at}[${index}]`));
  return Object.fromEntries(Object.entries(normWireRecord(value, at)).map(([key, entry]) => [key, normWireJson(entry, `${at}.${key}`)]));
};
export const normWireMap = <T>(read: NormWireReader<T>): NormWireReader<{ [key: string]: T }> => (value, at = "$") =>
  Object.fromEntries(Object.entries(normWireRecord(value, at)).map(([key, entry]) => [key, read(entry, `${at}.${key}`)]));
export const normWireRef = <T>(target: () => NormWireReader<T>): NormWireReader<T> => (value, at = "$") => target()(value, at);
export const normWireRequired = <T>(read: NormWireReader<T>): NormWireMember<T, false> => ({ read, optional: false });
export const normWireDefault = <T>(read: NormWireReader<T>, missing: () => T): NormWireMember<T, false> => ({ read, optional: false, missing });
export const normWireOptional = <T>(read: NormWireReader<T>): NormWireMember<T, true> => ({ read, optional: true });
export const normWireObject = <T extends object>(members: NormWireMembers<T>, closed = true): NormWireReader<T> => (value, at = "$") => {
  const row = normWireRecord(value, at);
  if (closed) for (const key of Object.keys(row)) if (!Object.hasOwn(members, key)) normWireRefuse(`${at}.${key}`, "member is not part of the contract");
  return Object.fromEntries(
    Object.entries(members as Readonly<Record<string, NormWireMember<unknown>>>).flatMap(([key, member]) =>
      Object.hasOwn(row, key) ? [[key, member.read(row[key], `${at}.${key}`)]] : member.missing ? [[key, member.missing()]] : member.optional ? [] : normWireRefuse(`${at}.${key}`, "required member is absent"),
    ),
  ) as T;
};
export const normWireTagged = <T extends object, K extends keyof T & string>(tag: K, variants: NormWireVariants<T, K>): NormWireReader<T> => (value, at = "$") => {
  const key = normWireRecord(value, at)[tag];
  const table = variants as Readonly<Record<string, NormWireReader<T>>>;
  return (typeof key === "string" && Object.hasOwn(table, key) ? table[key]! : normWireRefuse(`${at}.${tag}`, `value is not one of ${Object.keys(table).join(", ")}`))(value, at);
};
export const normWireExternal = <T extends object>(variants: NormWireExternalVariants<T>): NormWireReader<T> => (value, at = "$") => {
  const keys = Object.keys(normWireRecord(value, at));
  const table = variants as Readonly<Record<string, NormWireReader<unknown>>>;
  if (keys.length !== 1 || !Object.hasOwn(table, keys[0]!)) return normWireRefuse(at, `value is not exactly one of ${Object.keys(table).join(", ")}`);
  const [key] = keys as [string];
  return { [key]: table[key]!((value as Record<string, unknown>)[key], `${at}.${key}`) } as T;
};
export const normWireAny = <T extends readonly unknown[]>(...readers: { readonly [I in keyof T]: NormWireReader<T[I]> }): NormWireReader<T[number]> => (value, at = "$") => {
  const refusals: string[] = [];
  for (const read of readers as readonly NormWireReader<T[number]>[]) {
    try {
      return read(value, at);
    } catch (error) {
      if (!(error instanceof NormWireRefusal)) throw error;
      refusals.push(error.message);
    }
  }
  return normWireRefuse(at, `value matches no branch (${refusals.join("; ")})`);
};
//#endregion 🚪️Wire
