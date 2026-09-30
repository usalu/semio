/** 🧬️ Generation3d artifact schema — every field with its state class. */

export interface Generation3dArtifact {
  /** @state artifact */
  hostSnapshot: FlowHostSnapshot;
  /** @state artifact */
  generation: GenerationPlayState;
}

export type CameraJson = { x: number; y: number; zoom: number };
export type WidgetLayout = { x: number; y: number };
export type SynapseSpec = { id: string; from: string; to: string; fromPort: string; toPort: string };
/** 🎛️ One flow widget, internally tagged by `kind` — the flow `Widget` enum's wire form (`tag = "kind"`, camelCase). */
export type Widget =
  | { kind: "neuron"; id: string; neuronKind: string; params: Record<string, unknown>; inputPorts: string[]; outputPorts: string[]; preview: boolean }
  | { kind: "inputSlider"; id: string; label: string; value: number; min: number; max: number; step: number }
  | { kind: "inputNote"; id: string; text: string }
  | { kind: "inputImage"; id: string; src: string }
  | { kind: "variable"; id: string; name: string; schema: string }
  | { kind: "outputPreview"; id: string; preview: Record<string, unknown>; expanded: string[] }
  | { kind: "outputAction"; id: string; action: string }
  | { kind: "outputExport"; id: string; format: string }
  | { kind: "cluster"; id: string; name: string; tree: FlowTree; flow: FlowUi };
export type FlowTree = { neurons: FlowNeuron[]; synapses: SynapseSpec[] };
export type FlowNeuron = { id: string; kind: string; params: Record<string, unknown>; tree: FlowTree | null };
export type FlowUi = { camera: CameraJson; nodes: Record<string, FlowNodeGui>; previews: FlowPreviewGui[] };
export type FlowNodeGui = { layout: WidgetLayout; chrome: NodeChrome };
export type NodeChrome =
  | { kind: "plain"; preview: boolean }
  | { kind: "slider"; label: string; min: number; max: number; step: number; value: number }
  | { kind: "note"; text: string }
  | { kind: "image"; src: string }
  | { kind: "variable"; name: string; schema: string };
export type FlowPreviewGui = { id: string; source: FlowChannelRef | null; mode: string; preview: Record<string, unknown>; expanded: string[]; layout: WidgetLayout | null };
export type FlowChannelRef = { neuron: string; channel: string };
export type FlowHostSnapshot = {
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
export type Generation3dPreviewCamera = {
  positionX: number;
  positionY: number;
  positionZ: number;
  targetX: number;
  targetY: number;
  targetZ: number;
  fov: number;
};

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class proceduralGeneration3dArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const proceduralGeneration3dArtifactGuardReject = (at: string, why: string): never => {
  throw new proceduralGeneration3dArtifactGuardRefusal(at, why);
};

type proceduralGeneration3dArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type proceduralGeneration3dArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type proceduralGeneration3dArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const proceduralGeneration3dArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : proceduralGeneration3dArtifactGuardReject(at, "value is not an object");
export const proceduralGeneration3dArtifactGuardArray = (value: unknown, at: string, bounds: proceduralGeneration3dArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return proceduralGeneration3dArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) proceduralGeneration3dArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) proceduralGeneration3dArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const proceduralGeneration3dArtifactGuardString = (value: unknown, at: string, bounds: proceduralGeneration3dArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return proceduralGeneration3dArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) proceduralGeneration3dArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) proceduralGeneration3dArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) proceduralGeneration3dArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const proceduralGeneration3dArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : proceduralGeneration3dArtifactGuardReject(at, "value is not a boolean"));
export const proceduralGeneration3dArtifactGuardNumber = (value: unknown, at: string, bounds: proceduralGeneration3dArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return proceduralGeneration3dArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) proceduralGeneration3dArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) proceduralGeneration3dArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const proceduralGeneration3dArtifactGuardInteger = (value: unknown, at: string, bounds: proceduralGeneration3dArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? proceduralGeneration3dArtifactGuardNumber(value, at, bounds) : proceduralGeneration3dArtifactGuardReject(at, "value is not an integer");
export const proceduralGeneration3dArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : proceduralGeneration3dArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const proceduralGeneration3dArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : proceduralGeneration3dArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGeneration3dArtifact(value: unknown, at = "$"): Generation3dArtifact {
  const row = proceduralGeneration3dArtifactGuardObject(value, at);
  return {
    hostSnapshot: parseFlowHostSnapshot(row["hostSnapshot"], `${at}.hostSnapshot`),
    generation: parseGenerationPlayState(row["generation"], `${at}.generation`),
  };
}

export function parseCameraJson(value: unknown, at = "$"): CameraJson {
  const row = proceduralGeneration3dArtifactGuardObject(value, at);
  return {
    x: proceduralGeneration3dArtifactGuardNumber(row["x"], `${at}.x`),
    y: proceduralGeneration3dArtifactGuardNumber(row["y"], `${at}.y`),
    zoom: proceduralGeneration3dArtifactGuardNumber(row["zoom"], `${at}.zoom`),
  };
}

export function parseWidgetLayout(value: unknown, at = "$"): WidgetLayout {
  const row = proceduralGeneration3dArtifactGuardObject(value, at);
  return {
    x: proceduralGeneration3dArtifactGuardNumber(row["x"], `${at}.x`),
    y: proceduralGeneration3dArtifactGuardNumber(row["y"], `${at}.y`),
  };
}

export function parseSynapseSpec(value: unknown, at = "$"): SynapseSpec {
  const row = proceduralGeneration3dArtifactGuardObject(value, at);
  return {
    id: proceduralGeneration3dArtifactGuardString(row["id"], `${at}.id`),
    from: proceduralGeneration3dArtifactGuardString(row["from"], `${at}.from`),
    to: proceduralGeneration3dArtifactGuardString(row["to"], `${at}.to`),
    fromPort: proceduralGeneration3dArtifactGuardString(row["fromPort"], `${at}.fromPort`),
    toPort: proceduralGeneration3dArtifactGuardString(row["toPort"], `${at}.toPort`),
  };
}

/** 🎛️ The member kinds every widget kind carries beside `kind` and `id`. */
const WIDGET_MEMBERS: Readonly<Record<Widget["kind"], Readonly<Record<string, "string" | "number" | "boolean" | "strings" | "object">>>> = {
  neuron: { neuronKind: "string", params: "object", inputPorts: "strings", outputPorts: "strings", preview: "boolean" },
  inputSlider: { label: "string", value: "number", min: "number", max: "number", step: "number" },
  inputNote: { text: "string" },
  inputImage: { src: "string" },
  variable: { name: "string", schema: "string" },
  outputPreview: { preview: "object", expanded: "strings" },
  outputAction: { action: "string" },
  outputExport: { format: "string" },
  cluster: { name: "string", tree: "object", flow: "object" },
};

export function parseWidget(value: unknown, at = "$"): Widget {
  const row = proceduralGeneration3dArtifactGuardObject(value, at);
  const kind = proceduralGeneration3dArtifactGuardMember(row["kind"], `${at}.kind`, Object.keys(WIDGET_MEMBERS) as Widget["kind"][]);
  proceduralGeneration3dArtifactGuardString(row["id"], `${at}.id`);
  for (const [member, shape] of Object.entries(WIDGET_MEMBERS[kind])) {
    const where = `${at}.${member}`;
    if (shape === "string") proceduralGeneration3dArtifactGuardString(row[member], where);
    else if (shape === "number") proceduralGeneration3dArtifactGuardNumber(row[member], where);
    else if (shape === "boolean") proceduralGeneration3dArtifactGuardBoolean(row[member], where);
    else if (shape === "strings") proceduralGeneration3dArtifactGuardArray(row[member], where).forEach((item, index) => proceduralGeneration3dArtifactGuardString(item, `${where}[${index}]`));
    else proceduralGeneration3dArtifactGuardObject(row[member], where);
  }
  return row as unknown as Widget;
}

export function parseFlowHostSnapshot(value: unknown, at = "$"): FlowHostSnapshot {
  const row = proceduralGeneration3dArtifactGuardObject(value, at);
  return {
    schema: proceduralGeneration3dArtifactGuardString(row["schema"], `${at}.schema`),
    camera: parseCameraJson(row["camera"], `${at}.camera`),
    widgets: proceduralGeneration3dArtifactGuardArray(row["widgets"], `${at}.widgets`).map((item, index) => parseWidget(item, `${at}.widgets[${index}]`)),
    synapses: proceduralGeneration3dArtifactGuardArray(row["synapses"], `${at}.synapses`).map((item, index) => parseSynapseSpec(item, `${at}.synapses[${index}]`)),
    layout: Object.fromEntries(Object.entries(proceduralGeneration3dArtifactGuardObject(row["layout"], `${at}.layout`)).map(([key, item]) => [key, parseWidgetLayout(item, `${at}.layout.${key}`)])),
  };
}

export function parseFormGeneration(value: unknown, at = "$"): FormGeneration {
  const row = proceduralGeneration3dArtifactGuardObject(value, at);
  return {
    id: proceduralGeneration3dArtifactGuardString(row["id"], `${at}.id`),
    name: proceduralGeneration3dArtifactGuardString(row["name"], `${at}.name`),
    values: proceduralGeneration3dArtifactGuardObject(row["values"], `${at}.values`) as Record<string, unknown>,
  };
}

export function parseGenerationPlayState(value: unknown, at = "$"): GenerationPlayState {
  const row = proceduralGeneration3dArtifactGuardObject(value, at);
  return {
    generations: proceduralGeneration3dArtifactGuardArray(row["generations"], `${at}.generations`).map((item, index) => parseFormGeneration(item, `${at}.generations[${index}]`)),
    selectedGenerationId: row["selectedGenerationId"] === undefined ? undefined : proceduralGeneration3dArtifactGuardString(row["selectedGenerationId"], `${at}.selectedGenerationId`),
    previewText: row["previewText"] === undefined ? undefined : proceduralGeneration3dArtifactGuardString(row["previewText"], `${at}.previewText`),
  };
}

export function parseGeneration3dPreviewCamera(value: unknown, at = "$"): Generation3dPreviewCamera {
  const row = proceduralGeneration3dArtifactGuardObject(value, at);
  return {
    positionX: proceduralGeneration3dArtifactGuardNumber(row["positionX"], `${at}.positionX`),
    positionY: proceduralGeneration3dArtifactGuardNumber(row["positionY"], `${at}.positionY`),
    positionZ: proceduralGeneration3dArtifactGuardNumber(row["positionZ"], `${at}.positionZ`),
    targetX: proceduralGeneration3dArtifactGuardNumber(row["targetX"], `${at}.targetX`),
    targetY: proceduralGeneration3dArtifactGuardNumber(row["targetY"], `${at}.targetY`),
    targetZ: proceduralGeneration3dArtifactGuardNumber(row["targetZ"], `${at}.targetZ`),
    fov: proceduralGeneration3dArtifactGuardNumber(row["fov"], `${at}.fov`),
  };
}

export interface Generation3dStringList {
  readonly values: readonly string[];
}

export function parseGeneration3dStringList(value: unknown, at = "$"): Generation3dStringList {
  const row = proceduralGeneration3dArtifactGuardObject(value, at);
  return {
    values: proceduralGeneration3dArtifactGuardArray(row["values"], `${at}.values`).map((item, index) => proceduralGeneration3dArtifactGuardString(item, `${at}.values[${index}]`)),
  };
}
