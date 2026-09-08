/** 🧬️ Generation3dPresence */
export interface Generation3dPresence {
  /** @state presence */
  camera: CameraJson;
  /** @state presence */
  previewCamera: Generation3dPreviewCamera;
  /** @state presence */
  activeUtilityId: string;
  /** @state presence */
  showMode: string;
}

export type CameraJson = { x: number; y: number; zoom: number };
export type Generation3dPreviewCamera = {
  position: number[];
  target: number[];
  fov: number;
};

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class procedural3dPresenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const procedural3dPresenceGuardReject = (at: string, why: string): never => {
  throw new procedural3dPresenceGuardRefusal(at, why);
};

type procedural3dPresenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type procedural3dPresenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type procedural3dPresenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const procedural3dPresenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : procedural3dPresenceGuardReject(at, "value is not an object");
export const procedural3dPresenceGuardArray = (value: unknown, at: string, bounds: procedural3dPresenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return procedural3dPresenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) procedural3dPresenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) procedural3dPresenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const procedural3dPresenceGuardString = (value: unknown, at: string, bounds: procedural3dPresenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return procedural3dPresenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) procedural3dPresenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) procedural3dPresenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) procedural3dPresenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const procedural3dPresenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : procedural3dPresenceGuardReject(at, "value is not a boolean"));
export const procedural3dPresenceGuardNumber = (value: unknown, at: string, bounds: procedural3dPresenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return procedural3dPresenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) procedural3dPresenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) procedural3dPresenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const procedural3dPresenceGuardInteger = (value: unknown, at: string, bounds: procedural3dPresenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? procedural3dPresenceGuardNumber(value, at, bounds) : procedural3dPresenceGuardReject(at, "value is not an integer");
export const procedural3dPresenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : procedural3dPresenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const procedural3dPresenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : procedural3dPresenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGeneration3dPreviewCamera(value: unknown, at = "$"): Generation3dPreviewCamera {
  const row = procedural3dPresenceGuardObject(value, at);
  return {
    position: procedural3dPresenceGuardArray(row["position"], `${at}.position`, {"minItems": 3, "maxItems": 3}).map((item, index) => procedural3dPresenceGuardNumber(item, `${at}.position[${index}]`)),
    target: procedural3dPresenceGuardArray(row["target"], `${at}.target`, {"minItems": 3, "maxItems": 3}).map((item, index) => procedural3dPresenceGuardNumber(item, `${at}.target[${index}]`)),
    fov: procedural3dPresenceGuardNumber(row["fov"], `${at}.fov`),
  };
}
