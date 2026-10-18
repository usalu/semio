/** 🚪️ The guard helper block every projected parser file carries: a refusal class and the typed readers of objects, arrays, strings, numbers, members and constants.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/🟦️.ts
 */

//#region 🔖️GuardHelpers
/** 🚪️ The region marker that opens the helper block of a projected file. */
export const PARSER_REGION = "//#region 🚪️Parsers";

/** 🔒️ The end marker of the helper block of a projected file. */
export const PARSER_REGION_END = "//#endregion 🚪️Parsers";

/** 🔒️ `<prefix>Closed`: refuses every property of an object that the schema does not declare, where the schema states `additionalProperties: false`. */
export function closedHelper(helper: string): string {
  return `export const ${helper}Closed = (row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): Readonly<Record<string, unknown>> => {
  for (const key of Object.keys(row)) if (!keys.includes(key)) ${helper}Reject(\`\${at}.\${key}\`, "unknown property");
  return row;
};
`;
}

/** 🔒️ Adds `<prefix>Closed` to the helper block of a file that predates it, in front of the array reader (or at the end of the block); a file that has it is returned as it is. */
export function withClosedHelper(source: string, helper: string): string {
  if (new RegExp(`^export const ${helper}Closed\\b`, "m").test(source)) return source;
  const array = source.indexOf(`export const ${helper}Array`);
  const end = source.indexOf(PARSER_REGION_END);
  const at = array >= 0 && array < end ? array : end;
  if (at < 0) throw new Error(`the helper block of the \`${helper}\` guards is missing`);
  return `${source.slice(0, at)}${closedHelper(helper)}${source.slice(at)}`;
}

/** 🚪️ The helper block for one guard prefix: `<prefix>Refusal`, `<prefix>Object`, `<prefix>Closed`, `<prefix>Array`, `<prefix>String`, `<prefix>Boolean`, `<prefix>Number`, `<prefix>Integer`, `<prefix>Member` and `<prefix>Constant`. */
export function helpersBlock(helper: string): string {
  return `//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every \`parse<Export>\` below rejects with. */
export class ${helper}Refusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(\`\${at}: \${why}\`);
  }
}

const ${helper}Reject = (at: string, why: string): never => {
  throw new ${helper}Refusal(at, why);
};

type ${helper}TextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type ${helper}RangeBounds = { readonly minimum?: number; readonly maximum?: number };
type ${helper}SizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const ${helper}Object = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : ${helper}Reject(at, "value is not an object");
${closedHelper(helper)}export const ${helper}Array = (value: unknown, at: string, bounds: ${helper}SizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return ${helper}Reject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) ${helper}Reject(at, \`array has fewer than \${bounds.minItems} items\`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) ${helper}Reject(at, \`array has more than \${bounds.maxItems} items\`);
  return value;
};
export const ${helper}String = (value: unknown, at: string, bounds: ${helper}TextBounds = {}): string => {
  if (typeof value !== "string") return ${helper}Reject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) ${helper}Reject(at, \`string is shorter than \${bounds.minLength}\`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) ${helper}Reject(at, \`string is longer than \${bounds.maxLength}\`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) ${helper}Reject(at, \`string does not match \${bounds.pattern}\`);
  return value;
};
export const ${helper}Boolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : ${helper}Reject(at, "value is not a boolean"));
export const ${helper}Number = (value: unknown, at: string, bounds: ${helper}RangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return ${helper}Reject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) ${helper}Reject(at, \`number is below \${bounds.minimum}\`);
  if (bounds.maximum !== undefined && value > bounds.maximum) ${helper}Reject(at, \`number is above \${bounds.maximum}\`);
  return value;
};
export const ${helper}Integer = (value: unknown, at: string, bounds: ${helper}RangeBounds = {}): number =>
  Number.isSafeInteger(value) ? ${helper}Number(value, at, bounds) : ${helper}Reject(at, "value is not an integer");
export const ${helper}Member = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : ${helper}Reject(at, \`value is not one of \${members.join(", ")}\`);
export const ${helper}Constant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : ${helper}Reject(at, \`value is not \${String(expected)}\`);
${PARSER_REGION_END}
`;
}
//#endregion 🔖️GuardHelpers
