/** 🧬️ Flow artifact schema — every field with its state class. */

export interface FlowArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  camera: CameraJson;
  /** @state artifact */
  widgets: Widget[];
  /** @state artifact */
  synapses: SynapseSpec[];
  /** @state artifact */
  layout: Record<string, WidgetLayout>;
  /** @state presence */
  selectedNodeIds: string[];
  /** @state presence */
  selectedEdgeIds: string[];
  /** @state presence */
  selectedHandleIds: string[];
  /** @state presence */
  previewOffNodeIds: string[];
  /** @state config */
  lodMode: string;
  /** @state config */
  proximityDistance: number;
  /** @state config */
  gridVisible: boolean;
  /** @state config */
  gridSnapEnabled: boolean;
  /** @state config */
  gridFactor: number;
  /** @state config */
  catalogueSectionsJson: string;
  /** @state config */
  automationEnabledJson: string;
  /** @state config */
  contributionsJson: string;
  /** @state config */
  generationJson: string;
  /** @state config */
}

export interface CameraJson {
  x: number;
  y: number;
  zoom: number;
}

export interface WidgetLayout {
  x: number;
  y: number;
}

export interface SynapseSpec {
  id: string;
  from: string;
  to: string;
  fromPort: string;
  toPort: string;
}

/** Widget payload as JSON text (opaque enum). */
export type Widget = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class flowFlowArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const flowFlowArtifactGuardReject = (at: string, why: string): never => {
  throw new flowFlowArtifactGuardRefusal(at, why);
};

type flowFlowArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type flowFlowArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type flowFlowArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const flowFlowArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : flowFlowArtifactGuardReject(at, "value is not an object");
export const flowFlowArtifactGuardArray = (value: unknown, at: string, bounds: flowFlowArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return flowFlowArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) flowFlowArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) flowFlowArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const flowFlowArtifactGuardString = (value: unknown, at: string, bounds: flowFlowArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return flowFlowArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) flowFlowArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) flowFlowArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) flowFlowArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const flowFlowArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : flowFlowArtifactGuardReject(at, "value is not a boolean"));
export const flowFlowArtifactGuardNumber = (value: unknown, at: string, bounds: flowFlowArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return flowFlowArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) flowFlowArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) flowFlowArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const flowFlowArtifactGuardInteger = (value: unknown, at: string, bounds: flowFlowArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? flowFlowArtifactGuardNumber(value, at, bounds) : flowFlowArtifactGuardReject(at, "value is not an integer");
export const flowFlowArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : flowFlowArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const flowFlowArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : flowFlowArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFlowArtifact(value: unknown, at = "$"): FlowArtifact {
  const row = flowFlowArtifactGuardObject(value, at);
  return {
    schema: flowFlowArtifactGuardString(row["schema"], `${at}.schema`),
    camera: parseCameraJson(row["camera"], `${at}.camera`),
    widgets: flowFlowArtifactGuardArray(row["widgets"], `${at}.widgets`).map((item, index) => parseWidget(item, `${at}.widgets[${index}]`)),
    synapses: flowFlowArtifactGuardArray(row["synapses"], `${at}.synapses`).map((item, index) => parseSynapseSpec(item, `${at}.synapses[${index}]`)),
    layout: flowFlowArtifactGuardObject(row["layout"], `${at}.layout`),
    selectedNodeIds: flowFlowArtifactGuardArray(row["selectedNodeIds"], `${at}.selectedNodeIds`).map((item, index) => flowFlowArtifactGuardString(item, `${at}.selectedNodeIds[${index}]`)),
    selectedEdgeIds: flowFlowArtifactGuardArray(row["selectedEdgeIds"], `${at}.selectedEdgeIds`).map((item, index) => flowFlowArtifactGuardString(item, `${at}.selectedEdgeIds[${index}]`)),
    selectedHandleIds: flowFlowArtifactGuardArray(row["selectedHandleIds"], `${at}.selectedHandleIds`).map((item, index) => flowFlowArtifactGuardString(item, `${at}.selectedHandleIds[${index}]`)),
    previewOffNodeIds: flowFlowArtifactGuardArray(row["previewOffNodeIds"], `${at}.previewOffNodeIds`).map((item, index) => flowFlowArtifactGuardString(item, `${at}.previewOffNodeIds[${index}]`)),
    lodMode: flowFlowArtifactGuardString(row["lodMode"], `${at}.lodMode`),
    proximityDistance: flowFlowArtifactGuardNumber(row["proximityDistance"], `${at}.proximityDistance`),
    gridVisible: flowFlowArtifactGuardBoolean(row["gridVisible"], `${at}.gridVisible`),
    gridSnapEnabled: flowFlowArtifactGuardBoolean(row["gridSnapEnabled"], `${at}.gridSnapEnabled`),
    gridFactor: flowFlowArtifactGuardNumber(row["gridFactor"], `${at}.gridFactor`),
    catalogueSectionsJson: flowFlowArtifactGuardString(row["catalogueSectionsJson"], `${at}.catalogueSectionsJson`),
    automationEnabledJson: flowFlowArtifactGuardString(row["automationEnabledJson"], `${at}.automationEnabledJson`),
    contributionsJson: flowFlowArtifactGuardString(row["contributionsJson"], `${at}.contributionsJson`),
    generationJson: flowFlowArtifactGuardString(row["generationJson"], `${at}.generationJson`),
  };
}

export function parseCameraJson(value: unknown, at = "$"): CameraJson {
  const row = flowFlowArtifactGuardObject(value, at);
  return {
    x: flowFlowArtifactGuardNumber(row["x"], `${at}.x`),
    y: flowFlowArtifactGuardNumber(row["y"], `${at}.y`),
    zoom: flowFlowArtifactGuardNumber(row["zoom"], `${at}.zoom`),
  };
}

export function parseWidgetLayout(value: unknown, at = "$"): WidgetLayout {
  const row = flowFlowArtifactGuardObject(value, at);
  return {
    x: flowFlowArtifactGuardNumber(row["x"], `${at}.x`),
    y: flowFlowArtifactGuardNumber(row["y"], `${at}.y`),
  };
}

export function parseSynapseSpec(value: unknown, at = "$"): SynapseSpec {
  const row = flowFlowArtifactGuardObject(value, at);
  return {
    id: flowFlowArtifactGuardString(row["id"], `${at}.id`),
    from: flowFlowArtifactGuardString(row["from"], `${at}.from`),
    to: flowFlowArtifactGuardString(row["to"], `${at}.to`),
    fromPort: flowFlowArtifactGuardString(row["fromPort"], `${at}.fromPort`),
    toPort: flowFlowArtifactGuardString(row["toPort"], `${at}.toPort`),
  };
}

export function parseWidget(value: unknown, at = "$"): Widget {
  return flowFlowArtifactGuardString(value, `${at}`);
}
