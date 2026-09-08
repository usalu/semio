/** 💡️ Architect program inference schema — topology (hierarchy shape of elements via parentId). */

export interface ProgramTopology {
  nodeCount: number;
  rootCount: number;
  maxDepth: number;
  cycleFree: boolean;
  topoOrder: string[];
}

export interface ProgramInference {
  /** @derived */
  topology: ProgramTopology;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class architectProgramInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const architectProgramInferenceGuardReject = (at: string, why: string): never => {
  throw new architectProgramInferenceGuardRefusal(at, why);
};

type architectProgramInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type architectProgramInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type architectProgramInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const architectProgramInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : architectProgramInferenceGuardReject(at, "value is not an object");
export const architectProgramInferenceGuardArray = (value: unknown, at: string, bounds: architectProgramInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return architectProgramInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) architectProgramInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) architectProgramInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const architectProgramInferenceGuardString = (value: unknown, at: string, bounds: architectProgramInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return architectProgramInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) architectProgramInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) architectProgramInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) architectProgramInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const architectProgramInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : architectProgramInferenceGuardReject(at, "value is not a boolean"));
export const architectProgramInferenceGuardNumber = (value: unknown, at: string, bounds: architectProgramInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return architectProgramInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) architectProgramInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) architectProgramInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const architectProgramInferenceGuardInteger = (value: unknown, at: string, bounds: architectProgramInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? architectProgramInferenceGuardNumber(value, at, bounds) : architectProgramInferenceGuardReject(at, "value is not an integer");
export const architectProgramInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : architectProgramInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const architectProgramInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : architectProgramInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseProgramInference(value: unknown, at = "$"): ProgramInference {
  const row = architectProgramInferenceGuardObject(value, at);
  return {
    topology: parseProgramTopology(row["topology"], `${at}.topology`),
  };
}

export function parseProgramTopology(value: unknown, at = "$"): ProgramTopology {
  const row = architectProgramInferenceGuardObject(value, at);
  return {
    nodeCount: architectProgramInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`, {"minimum": 0}),
    rootCount: architectProgramInferenceGuardInteger(row["rootCount"], `${at}.rootCount`, {"minimum": 0}),
    maxDepth: architectProgramInferenceGuardInteger(row["maxDepth"], `${at}.maxDepth`, {"minimum": 0}),
    cycleFree: architectProgramInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
    topoOrder: architectProgramInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => architectProgramInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
  };
}
