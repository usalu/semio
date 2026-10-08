/** 💡️ Generation3d inference schema — topology (DAG shape of `fixture`'s widget/synapse graph) and geometry (the evaluation of every widget, summarised). */

export interface Generation3dTopology {
  nodeCount: number;
  edgeCount: number;
  topoOrder: string[];
  depth: number;
  cycleFree: boolean;
}

export type Generation3dGeometryQuality = "exact-analytic" | "exact-numerical" | "approximate" | "mesh-derived-brep" | "polygon-mesh" | "tessellated-mesh";

export interface Generation3dFaultRecord {
  code: string;
  en: string;
  de: string;
  port: string | null;
}

export interface Generation3dOutputRecord {
  port: string;
  kind: string;
  detail: string;
}

export interface Generation3dWidgetRecord {
  quality: Generation3dGeometryQuality;
  fault: Generation3dFaultRecord | null;
  outputs: Generation3dOutputRecord[];
}

export interface Generation3dGeometryRecord {
  widgets: Record<string, Generation3dWidgetRecord>;
  faulted: number;
}

export interface Generation3dInference {
  /** @derived */
  topology: Generation3dTopology;
  /** @derived */
  geometry: Generation3dGeometryRecord;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class proceduralGeneration3dInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const proceduralGeneration3dInferenceGuardReject = (at: string, why: string): never => {
  throw new proceduralGeneration3dInferenceGuardRefusal(at, why);
};

type proceduralGeneration3dInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type proceduralGeneration3dInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type proceduralGeneration3dInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const proceduralGeneration3dInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : proceduralGeneration3dInferenceGuardReject(at, "value is not an object");
export const proceduralGeneration3dInferenceGuardArray = (value: unknown, at: string, bounds: proceduralGeneration3dInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return proceduralGeneration3dInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) proceduralGeneration3dInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) proceduralGeneration3dInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const proceduralGeneration3dInferenceGuardString = (value: unknown, at: string, bounds: proceduralGeneration3dInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return proceduralGeneration3dInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) proceduralGeneration3dInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) proceduralGeneration3dInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) proceduralGeneration3dInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const proceduralGeneration3dInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : proceduralGeneration3dInferenceGuardReject(at, "value is not a boolean"));
export const proceduralGeneration3dInferenceGuardNumber = (value: unknown, at: string, bounds: proceduralGeneration3dInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return proceduralGeneration3dInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) proceduralGeneration3dInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) proceduralGeneration3dInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const proceduralGeneration3dInferenceGuardInteger = (value: unknown, at: string, bounds: proceduralGeneration3dInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? proceduralGeneration3dInferenceGuardNumber(value, at, bounds) : proceduralGeneration3dInferenceGuardReject(at, "value is not an integer");
export const proceduralGeneration3dInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : proceduralGeneration3dInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const proceduralGeneration3dInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : proceduralGeneration3dInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGeneration3dInference(value: unknown, at = "$"): Generation3dInference {
  const row = proceduralGeneration3dInferenceGuardObject(value, at);
  return {
    topology: parseGeneration3dTopology(row["topology"], `${at}.topology`),
    geometry: parseGeneration3dGeometryRecord(row["geometry"], `${at}.geometry`),
  };
}

const geometryQualities: readonly Generation3dGeometryQuality[] = ["exact-analytic", "exact-numerical", "approximate", "mesh-derived-brep", "polygon-mesh", "tessellated-mesh"];

export function parseGeneration3dGeometryRecord(value: unknown, at = "$"): Generation3dGeometryRecord {
  const row = proceduralGeneration3dInferenceGuardObject(value, at);
  const widgets = proceduralGeneration3dInferenceGuardObject(row["widgets"], `${at}.widgets`);
  return {
    widgets: Object.fromEntries(Object.entries(widgets).map(([id, widget]) => [id, parseGeneration3dWidgetRecord(widget, `${at}.widgets.${id}`)])),
    faulted: proceduralGeneration3dInferenceGuardInteger(row["faulted"], `${at}.faulted`, {"minimum": 0}),
  };
}

export function parseGeneration3dWidgetRecord(value: unknown, at = "$"): Generation3dWidgetRecord {
  const row = proceduralGeneration3dInferenceGuardObject(value, at);
  return {
    quality: proceduralGeneration3dInferenceGuardMember(row["quality"], `${at}.quality`, geometryQualities),
    fault: row["fault"] === null ? null : parseGeneration3dFaultRecord(row["fault"], `${at}.fault`),
    outputs: proceduralGeneration3dInferenceGuardArray(row["outputs"], `${at}.outputs`).map((item, index) => parseGeneration3dOutputRecord(item, `${at}.outputs[${index}]`)),
  };
}

export function parseGeneration3dFaultRecord(value: unknown, at = "$"): Generation3dFaultRecord {
  const row = proceduralGeneration3dInferenceGuardObject(value, at);
  return {
    code: proceduralGeneration3dInferenceGuardString(row["code"], `${at}.code`),
    en: proceduralGeneration3dInferenceGuardString(row["en"], `${at}.en`),
    de: proceduralGeneration3dInferenceGuardString(row["de"], `${at}.de`),
    port: row["port"] === null ? null : proceduralGeneration3dInferenceGuardString(row["port"], `${at}.port`),
  };
}

export function parseGeneration3dOutputRecord(value: unknown, at = "$"): Generation3dOutputRecord {
  const row = proceduralGeneration3dInferenceGuardObject(value, at);
  return {
    port: proceduralGeneration3dInferenceGuardString(row["port"], `${at}.port`),
    kind: proceduralGeneration3dInferenceGuardString(row["kind"], `${at}.kind`),
    detail: proceduralGeneration3dInferenceGuardString(row["detail"], `${at}.detail`),
  };
}

export function parseGeneration3dTopology(value: unknown, at = "$"): Generation3dTopology {
  const row = proceduralGeneration3dInferenceGuardObject(value, at);
  return {
    nodeCount: proceduralGeneration3dInferenceGuardInteger(row["nodeCount"], `${at}.nodeCount`, {"minimum": 0}),
    edgeCount: proceduralGeneration3dInferenceGuardInteger(row["edgeCount"], `${at}.edgeCount`, {"minimum": 0}),
    topoOrder: proceduralGeneration3dInferenceGuardArray(row["topoOrder"], `${at}.topoOrder`).map((item, index) => proceduralGeneration3dInferenceGuardString(item, `${at}.topoOrder[${index}]`)),
    depth: proceduralGeneration3dInferenceGuardInteger(row["depth"], `${at}.depth`, {"minimum": 0}),
    cycleFree: proceduralGeneration3dInferenceGuardBoolean(row["cycleFree"], `${at}.cycleFree`),
  };
}
