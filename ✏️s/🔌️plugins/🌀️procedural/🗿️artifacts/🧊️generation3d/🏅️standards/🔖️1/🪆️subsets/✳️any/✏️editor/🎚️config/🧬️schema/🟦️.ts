/** 🧬️ Generation3dConfig */
export interface Generation3dConfig {
  /** @state config */
  lodMode: string;
  /** @state config */
  showMode: string;
  /** @state config */
  camera: CameraJson;
  /** @state config */
  previewCamera: Generation3dPreviewCamera;
  /** @state config */
  sunJson: string;
  /** @state config */
  selectedGenerationId?: string;
  /** @state config */
  previewEvalText?: string;
}

export type CameraJson = { x: number; y: number; zoom: number };
export type Generation3dPreviewCamera = {
  position: number[];
  target: number[];
  fov: number;
};

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class procedural3dConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const procedural3dConfigGuardReject = (at: string, why: string): never => {
  throw new procedural3dConfigGuardRefusal(at, why);
};

type procedural3dConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type procedural3dConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type procedural3dConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const procedural3dConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : procedural3dConfigGuardReject(at, "value is not an object");
export const procedural3dConfigGuardArray = (value: unknown, at: string, bounds: procedural3dConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return procedural3dConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) procedural3dConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) procedural3dConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const procedural3dConfigGuardString = (value: unknown, at: string, bounds: procedural3dConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return procedural3dConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) procedural3dConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) procedural3dConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) procedural3dConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const procedural3dConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : procedural3dConfigGuardReject(at, "value is not a boolean"));
export const procedural3dConfigGuardNumber = (value: unknown, at: string, bounds: procedural3dConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return procedural3dConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) procedural3dConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) procedural3dConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const procedural3dConfigGuardInteger = (value: unknown, at: string, bounds: procedural3dConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? procedural3dConfigGuardNumber(value, at, bounds) : procedural3dConfigGuardReject(at, "value is not an integer");
export const procedural3dConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : procedural3dConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const procedural3dConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : procedural3dConfigGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGeneration3dPreviewCamera(value: unknown, at = "$"): Generation3dPreviewCamera {
  const row = procedural3dConfigGuardObject(value, at);
  return {
    position: procedural3dConfigGuardArray(row["position"], `${at}.position`, {"minItems": 3, "maxItems": 3}).map((item, index) => procedural3dConfigGuardNumber(item, `${at}.position[${index}]`)),
    target: procedural3dConfigGuardArray(row["target"], `${at}.target`, {"minItems": 3, "maxItems": 3}).map((item, index) => procedural3dConfigGuardNumber(item, `${at}.target[${index}]`)),
    fov: procedural3dConfigGuardNumber(row["fov"], `${at}.fov`),
  };
}
