/** 🧬️ Generation2d artifact schema — every field with its state class. */

export interface Generation2dArtifact {
  /** @state artifact */
  fixture: FlowFixture;
  /** @state artifact */
  generation: GenerationPlayState;
}

export type CameraJson = { x: number; y: number; zoom: number };
export type WidgetLayout = { x: number; y: number };
export type SynapseSpec = { id: string; from: string; to: string; fromPort: string; toPort: string };
/** @description Polymorphic flow widget — JSON blob. */
export type Widget = string;
export type FlowFixture = {
  schema: string;
  camera: CameraJson;
  widgets: Widget[];
  synapses: SynapseSpec[];
  layout: Record<string, WidgetLayout>;
};
export type FormGeneration = { id: string; name: string; values: Record<string, unknown> };
export type GenerationPlayState = {
  generations: FormGeneration[];
  selectedGenerationId?: string;
  previewText?: string;
};

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class proceduralGeneration2dArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const proceduralGeneration2dArtifactGuardReject = (at: string, why: string): never => {
  throw new proceduralGeneration2dArtifactGuardRefusal(at, why);
};

type proceduralGeneration2dArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type proceduralGeneration2dArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type proceduralGeneration2dArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const proceduralGeneration2dArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : proceduralGeneration2dArtifactGuardReject(at, "value is not an object");
export const proceduralGeneration2dArtifactGuardArray = (value: unknown, at: string, bounds: proceduralGeneration2dArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return proceduralGeneration2dArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) proceduralGeneration2dArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) proceduralGeneration2dArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const proceduralGeneration2dArtifactGuardString = (value: unknown, at: string, bounds: proceduralGeneration2dArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return proceduralGeneration2dArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) proceduralGeneration2dArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) proceduralGeneration2dArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) proceduralGeneration2dArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const proceduralGeneration2dArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : proceduralGeneration2dArtifactGuardReject(at, "value is not a boolean"));
export const proceduralGeneration2dArtifactGuardNumber = (value: unknown, at: string, bounds: proceduralGeneration2dArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return proceduralGeneration2dArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) proceduralGeneration2dArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) proceduralGeneration2dArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const proceduralGeneration2dArtifactGuardInteger = (value: unknown, at: string, bounds: proceduralGeneration2dArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? proceduralGeneration2dArtifactGuardNumber(value, at, bounds) : proceduralGeneration2dArtifactGuardReject(at, "value is not an integer");
export const proceduralGeneration2dArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : proceduralGeneration2dArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const proceduralGeneration2dArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : proceduralGeneration2dArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGeneration2dArtifact(value: unknown, at = "$"): Generation2dArtifact {
  const row = proceduralGeneration2dArtifactGuardObject(value, at);
  return {
    fixture: parseFlowFixture(row["fixture"], `${at}.fixture`),
    generation: parseGenerationPlayState(row["generation"], `${at}.generation`),
  };
}

export function parseCameraJson(value: unknown, at = "$"): CameraJson {
  const row = proceduralGeneration2dArtifactGuardObject(value, at);
  return {
    x: proceduralGeneration2dArtifactGuardNumber(row["x"], `${at}.x`),
    y: proceduralGeneration2dArtifactGuardNumber(row["y"], `${at}.y`),
    zoom: proceduralGeneration2dArtifactGuardNumber(row["zoom"], `${at}.zoom`),
  };
}

export function parseWidgetLayout(value: unknown, at = "$"): WidgetLayout {
  const row = proceduralGeneration2dArtifactGuardObject(value, at);
  return {
    x: proceduralGeneration2dArtifactGuardNumber(row["x"], `${at}.x`),
    y: proceduralGeneration2dArtifactGuardNumber(row["y"], `${at}.y`),
  };
}

export function parseSynapseSpec(value: unknown, at = "$"): SynapseSpec {
  const row = proceduralGeneration2dArtifactGuardObject(value, at);
  return {
    id: proceduralGeneration2dArtifactGuardString(row["id"], `${at}.id`),
    from: proceduralGeneration2dArtifactGuardString(row["from"], `${at}.from`),
    to: proceduralGeneration2dArtifactGuardString(row["to"], `${at}.to`),
    fromPort: proceduralGeneration2dArtifactGuardString(row["fromPort"], `${at}.fromPort`),
    toPort: proceduralGeneration2dArtifactGuardString(row["toPort"], `${at}.toPort`),
  };
}

export function parseWidget(value: unknown, at = "$"): Widget {
  return proceduralGeneration2dArtifactGuardString(value, `${at}`);
}

export function parseFlowFixture(value: unknown, at = "$"): FlowFixture {
  const row = proceduralGeneration2dArtifactGuardObject(value, at);
  return {
    schema: proceduralGeneration2dArtifactGuardString(row["schema"], `${at}.schema`),
    camera: parseCameraJson(row["camera"], `${at}.camera`),
    widgets: proceduralGeneration2dArtifactGuardArray(row["widgets"], `${at}.widgets`).map((item, index) => parseWidget(item, `${at}.widgets[${index}]`)),
    synapses: proceduralGeneration2dArtifactGuardArray(row["synapses"], `${at}.synapses`).map((item, index) => parseSynapseSpec(item, `${at}.synapses[${index}]`)),
    layout: proceduralGeneration2dArtifactGuardObject(row["layout"], `${at}.layout`),
  };
}

export function parseFormGeneration(value: unknown, at = "$"): FormGeneration {
  const row = proceduralGeneration2dArtifactGuardObject(value, at);
  return {
    id: proceduralGeneration2dArtifactGuardString(row["id"], `${at}.id`),
    name: proceduralGeneration2dArtifactGuardString(row["name"], `${at}.name`),
    valuesJson: proceduralGeneration2dArtifactGuardString(row["valuesJson"], `${at}.valuesJson`),
  };
}

export function parseGenerationPlayState(value: unknown, at = "$"): GenerationPlayState {
  const row = proceduralGeneration2dArtifactGuardObject(value, at);
  return {
    generations: proceduralGeneration2dArtifactGuardArray(row["generations"], `${at}.generations`).map((item, index) => parseFormGeneration(item, `${at}.generations[${index}]`)),
    previewText: row["previewText"] === undefined ? undefined : proceduralGeneration2dArtifactGuardString(row["previewText"], `${at}.previewText`),
  };
}

export interface Generation2dStringList {
  readonly values: readonly string[];
}

export function parseGeneration2dStringList(value: unknown, at = "$"): Generation2dStringList {
  const row = proceduralGeneration2dArtifactGuardObject(value, at);
  return {
    values: proceduralGeneration2dArtifactGuardArray(row["values"], `${at}.values`).map((item, index) => proceduralGeneration2dArtifactGuardString(item, `${at}.values[${index}]`)),
  };
}
