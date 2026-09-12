//#region 🧬️Configuration
/** 🧬️ MapWindowConfig */
export interface MapWindowConfig {
  /** @state config */
  layerVisibility: Record<string, boolean>;
  /** @state config */
  cameraJson: string;
  /** @state config */
  renderMode: string;
  /** @state config */
  vectorStyle: string;
  /** @state config */
  lodMode: string;
  /** @state config */
  layerStrokeScale: Record<string, number>;
  /** @state config */
}

export type MapWindowConfigMutation =
  | { operation: "setLayerVisibility"; layerId: string; visible: boolean | null }
  | { operation: "setCamera"; cameraJson: string }
  | { operation: "setRenderMode"; value: string }
  | { operation: "setVectorStyle"; value: string }
  | { operation: "setLodMode"; value: string }
  | { operation: "setLayerStrokeScale"; layerId: string; value: number | null };
//#endregion 🧬️Configuration

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class gisMapWindowConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const gisMapWindowConfigGuardReject = (at: string, why: string): never => {
  throw new gisMapWindowConfigGuardRefusal(at, why);
};

type gisMapWindowConfigGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type gisMapWindowConfigGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type gisMapWindowConfigGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const gisMapWindowConfigGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : gisMapWindowConfigGuardReject(at, "value is not an object");
export const gisMapWindowConfigGuardArray = (value: unknown, at: string, bounds: gisMapWindowConfigGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return gisMapWindowConfigGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) gisMapWindowConfigGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) gisMapWindowConfigGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const gisMapWindowConfigGuardString = (value: unknown, at: string, bounds: gisMapWindowConfigGuardTextBounds = {}): string => {
  if (typeof value !== "string") return gisMapWindowConfigGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) gisMapWindowConfigGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) gisMapWindowConfigGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) gisMapWindowConfigGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const gisMapWindowConfigGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : gisMapWindowConfigGuardReject(at, "value is not a boolean"));
export const gisMapWindowConfigGuardNumber = (value: unknown, at: string, bounds: gisMapWindowConfigGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return gisMapWindowConfigGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) gisMapWindowConfigGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) gisMapWindowConfigGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const gisMapWindowConfigGuardInteger = (value: unknown, at: string, bounds: gisMapWindowConfigGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? gisMapWindowConfigGuardNumber(value, at, bounds) : gisMapWindowConfigGuardReject(at, "value is not an integer");
export const gisMapWindowConfigGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : gisMapWindowConfigGuardReject(at, `value is not one of ${members.join(", ")}`);
export const gisMapWindowConfigGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : gisMapWindowConfigGuardReject(at, `value is not ${String(expected)}`);

const gisMapWindowConfigBooleanRecord = (value: unknown, at: string): Record<string, boolean> => {
  const row = gisMapWindowConfigGuardObject(value, at);
  return Object.fromEntries(Object.entries(row).map(([key, member]) => [key, gisMapWindowConfigGuardBoolean(member, `${at}.${key}`)]));
};

const gisMapWindowConfigNumberRecord = (value: unknown, at: string): Record<string, number> => {
  const row = gisMapWindowConfigGuardObject(value, at);
  return Object.fromEntries(Object.entries(row).map(([key, member]) => [key, gisMapWindowConfigGuardNumber(member, `${at}.${key}`)]));
};
//#endregion 🚪️Parsers

export function parseMapWindowConfig(value: unknown, at = "$"): MapWindowConfig {
  const row = gisMapWindowConfigGuardObject(value, at);
  return {
    layerVisibility: gisMapWindowConfigBooleanRecord(row["layerVisibility"], `${at}.layerVisibility`),
    cameraJson: gisMapWindowConfigGuardString(row["cameraJson"], `${at}.cameraJson`),
    renderMode: gisMapWindowConfigGuardString(row["renderMode"], `${at}.renderMode`),
    vectorStyle: gisMapWindowConfigGuardString(row["vectorStyle"], `${at}.vectorStyle`),
    lodMode: gisMapWindowConfigGuardString(row["lodMode"], `${at}.lodMode`),
    layerStrokeScale: gisMapWindowConfigNumberRecord(row["layerStrokeScale"], `${at}.layerStrokeScale`),
  };
}

export function applyMapWindowConfigMutation(base: MapWindowConfig, mutation: MapWindowConfigMutation): MapWindowConfig {
  const next = structuredClone(base);
  switch (mutation.operation) {
    case "setLayerVisibility":
      if (mutation.visible === null) delete next.layerVisibility[mutation.layerId];
      else next.layerVisibility[mutation.layerId] = mutation.visible;
      break;
    case "setCamera": next.cameraJson = mutation.cameraJson; break;
    case "setRenderMode": next.renderMode = mutation.value; break;
    case "setVectorStyle": next.vectorStyle = mutation.value; break;
    case "setLodMode": next.lodMode = mutation.value; break;
    case "setLayerStrokeScale":
      if (mutation.value === null) delete next.layerStrokeScale[mutation.layerId];
      else next.layerStrokeScale[mutation.layerId] = mutation.value;
      break;
  }
  return parseMapWindowConfig(next);
}
