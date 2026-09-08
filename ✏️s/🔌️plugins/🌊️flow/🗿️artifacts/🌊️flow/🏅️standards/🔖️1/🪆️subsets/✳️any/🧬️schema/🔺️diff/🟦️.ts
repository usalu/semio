/** 🧬️ Flow diff schema — sparse field delta. */

export interface FlowDiff {
  /** @state artifact */
  artifact?: FlowArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  camera?: CameraJson;
  /** @state artifact */
  widgets?: FlowWidgetsDelta;
  /** @state artifact */
  synapses?: FlowSynapsesDelta;
  /** @state artifact */
  layout?: FlowLayoutMapDelta;
  /** @state presence */
  selectedNodeIds?: FlowStringList;
  /** @state presence */
  selectedEdgeIds?: FlowStringList;
  /** @state presence */
  selectedHandleIds?: FlowStringList;
  /** @state presence */
  previewOffNodeIds?: FlowStringList;
  /** @state config */
  lodMode?: string;
  /** @state config */
  proximityDistance?: number;
  /** @state config */
  gridVisible?: boolean;
  /** @state config */
  gridSnapEnabled?: boolean;
  /** @state config */
  gridFactor?: number;
  /** @state config */
  catalogueSectionsJson?: string;
  /** @state config */
  automationEnabledJson?: string;
  /** @state config */
  contributionsJson?: string;
  /** @state config */
  generationJson?: string;
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

export interface FlowArtifact {
  schema: string;
  camera: CameraJson;
  widgets: Widget[];
  synapses: SynapseSpec[];
  layout: Record<string, WidgetLayout>;
  selectedNodeIds: string[];
  selectedEdgeIds: string[];
  selectedHandleIds: string[];
  previewOffNodeIds: string[];
  lodMode: string;
  proximityDistance: number;
  gridVisible: boolean;
  gridSnapEnabled: boolean;
  gridFactor: number;
  catalogueSectionsJson: string;
  automationEnabledJson: string;
  contributionsJson: string;
  generationJson: string;
}

export interface FlowStringList {
  values: string[];
}

export interface FlowWidgetPatchEntry {
  id: string;
  patch: Widget;
}

export interface FlowWidgetsDelta {
  added: Widget[];
  removed: string[];
  patched: FlowWidgetPatchEntry[];
  reordered?: string[];
}

export interface FlowSynapsePatchEntry {
  id: string;
  patch: SynapseSpec;
}

export interface FlowSynapsesDelta {
  added: SynapseSpec[];
  removed: string[];
  patched: FlowSynapsePatchEntry[];
  reordered?: string[];
}

export interface FlowLayoutMapDelta {
  entries: Record<string, WidgetLayout | null>;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class flowFlowDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const flowFlowDiffGuardReject = (at: string, why: string): never => {
  throw new flowFlowDiffGuardRefusal(at, why);
};

type flowFlowDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type flowFlowDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type flowFlowDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const flowFlowDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : flowFlowDiffGuardReject(at, "value is not an object");
export const flowFlowDiffGuardArray = (value: unknown, at: string, bounds: flowFlowDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return flowFlowDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) flowFlowDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) flowFlowDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const flowFlowDiffGuardString = (value: unknown, at: string, bounds: flowFlowDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return flowFlowDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) flowFlowDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) flowFlowDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) flowFlowDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const flowFlowDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : flowFlowDiffGuardReject(at, "value is not a boolean"));
export const flowFlowDiffGuardNumber = (value: unknown, at: string, bounds: flowFlowDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return flowFlowDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) flowFlowDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) flowFlowDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const flowFlowDiffGuardInteger = (value: unknown, at: string, bounds: flowFlowDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? flowFlowDiffGuardNumber(value, at, bounds) : flowFlowDiffGuardReject(at, "value is not an integer");
export const flowFlowDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : flowFlowDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const flowFlowDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : flowFlowDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseFlowStringList(value: unknown, at = "$"): FlowStringList {
  const row = flowFlowDiffGuardObject(value, at);
  return {
    values: flowFlowDiffGuardArray(row["values"], `${at}.values`).map((item, index) => flowFlowDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

export function parseFlowLayoutMapDelta(value: unknown, at = "$"): FlowLayoutMapDelta {
  const row = flowFlowDiffGuardObject(value, at);
  return {
    entries: flowFlowDiffGuardObject(row["entries"], `${at}.entries`),
  };
}
