/** 🧬️ Sequence artifact schema — every field with its state class. */
export interface SequenceArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ steps: SequenceStep[];
  /** @state artifact */ edges: SequenceEdge[];
  /** @state config */ lastRunJson: string;
  /** @state config */ orientation: string;
  /** @state config */ camera: SequenceCamera;
}
export interface SequenceStep { id: string; kind: string; params: Record<string, unknown>; x: number; y: number; slot?: SlotRef; collapsed: boolean; }
export interface SequenceEdge { id: string; from: string; to: string; }
export interface SlotRef { owner: string; name: string; }
export interface SequenceCamera { x: number; y: number; zoom: number; }

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class sequenceSequenceArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const sequenceSequenceArtifactGuardReject = (at: string, why: string): never => {
  throw new sequenceSequenceArtifactGuardRefusal(at, why);
};

type sequenceSequenceArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type sequenceSequenceArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type sequenceSequenceArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const sequenceSequenceArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : sequenceSequenceArtifactGuardReject(at, "value is not an object");
export const sequenceSequenceArtifactGuardArray = (value: unknown, at: string, bounds: sequenceSequenceArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return sequenceSequenceArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) sequenceSequenceArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) sequenceSequenceArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const sequenceSequenceArtifactGuardString = (value: unknown, at: string, bounds: sequenceSequenceArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return sequenceSequenceArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) sequenceSequenceArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) sequenceSequenceArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) sequenceSequenceArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const sequenceSequenceArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : sequenceSequenceArtifactGuardReject(at, "value is not a boolean"));
export const sequenceSequenceArtifactGuardNumber = (value: unknown, at: string, bounds: sequenceSequenceArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return sequenceSequenceArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) sequenceSequenceArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) sequenceSequenceArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const sequenceSequenceArtifactGuardInteger = (value: unknown, at: string, bounds: sequenceSequenceArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? sequenceSequenceArtifactGuardNumber(value, at, bounds) : sequenceSequenceArtifactGuardReject(at, "value is not an integer");
export const sequenceSequenceArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : sequenceSequenceArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const sequenceSequenceArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : sequenceSequenceArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSequenceArtifact(value: unknown, at = "$"): SequenceArtifact {
  const row = sequenceSequenceArtifactGuardObject(value, at);
  return {
    schema: sequenceSequenceArtifactGuardString(row["schema"], `${at}.schema`),
    steps: sequenceSequenceArtifactGuardArray(row["steps"], `${at}.steps`).map((item, index) => parseSequenceStep(item, `${at}.steps[${index}]`)),
    edges: sequenceSequenceArtifactGuardArray(row["edges"], `${at}.edges`).map((item, index) => parseSequenceEdge(item, `${at}.edges[${index}]`)),
    lastRunJson: sequenceSequenceArtifactGuardString(row["lastRunJson"], `${at}.lastRunJson`),
    orientation: sequenceSequenceArtifactGuardString(row["orientation"], `${at}.orientation`),
    camera: parseSequenceCamera(row["camera"], `${at}.camera`),
  };
}

export function parseSequenceStep(value: unknown, at = "$"): SequenceStep {
  const row = sequenceSequenceArtifactGuardObject(value, at);
  return {
    id: sequenceSequenceArtifactGuardString(row["id"], `${at}.id`),
    kind: sequenceSequenceArtifactGuardString(row["kind"], `${at}.kind`),
    params: sequenceSequenceArtifactGuardObject(row["params"], `${at}.params`),
    x: sequenceSequenceArtifactGuardNumber(row["x"], `${at}.x`),
    y: sequenceSequenceArtifactGuardNumber(row["y"], `${at}.y`),
    slot: row["slot"] === undefined ? undefined : parseSlotRef(row["slot"], `${at}.slot`),
    collapsed: sequenceSequenceArtifactGuardBoolean(row["collapsed"], `${at}.collapsed`),
  };
}

export function parseSequenceEdge(value: unknown, at = "$"): SequenceEdge {
  const row = sequenceSequenceArtifactGuardObject(value, at);
  return {
    id: sequenceSequenceArtifactGuardString(row["id"], `${at}.id`),
    from: sequenceSequenceArtifactGuardString(row["from"], `${at}.from`),
    to: sequenceSequenceArtifactGuardString(row["to"], `${at}.to`),
  };
}

export function parseSlotRef(value: unknown, at = "$"): SlotRef {
  const row = sequenceSequenceArtifactGuardObject(value, at);
  return {
    owner: sequenceSequenceArtifactGuardString(row["owner"], `${at}.owner`),
    name: sequenceSequenceArtifactGuardString(row["name"], `${at}.name`),
  };
}

export function parseSequenceCamera(value: unknown, at = "$"): SequenceCamera {
  const row = sequenceSequenceArtifactGuardObject(value, at);
  return {
    x: sequenceSequenceArtifactGuardNumber(row["x"], `${at}.x`),
    y: sequenceSequenceArtifactGuardNumber(row["y"], `${at}.y`),
    zoom: sequenceSequenceArtifactGuardNumber(row["zoom"], `${at}.zoom`),
  };
}
