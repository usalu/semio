/** 🧬️ Equation artifact schema — every field with its state class. */

export interface EquationArtifact {
  /** @state artifact */
  graph: EquationGraph;
  /** @state artifact */
  geometry: EquationGeometry;
  /** @state config */
  cameraX: number;
  /** @state config */
  cameraY: number;
  /** @state config */
  cameraZoom: number;
  /** @state config */
}

export interface EquationGraph {
  directed: boolean;
  nodes: EquationNode[];
  edges: EquationEdge[];
  algorithm: string;
  algorithmSeed?: string;
}

export interface EquationNode {
  id: string;
  label: string;
  x: number;
  y: number;
}

export interface EquationEdge {
  id: string;
  source: string;
  target: string;
}

export interface EquationPoint {
  x: number;
  y: number;
}

export interface EquationGeometry {
  points: EquationPoint[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class equationEquationArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const equationEquationArtifactGuardReject = (at: string, why: string): never => {
  throw new equationEquationArtifactGuardRefusal(at, why);
};

type equationEquationArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type equationEquationArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type equationEquationArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const equationEquationArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : equationEquationArtifactGuardReject(at, "value is not an object");
export const equationEquationArtifactGuardArray = (value: unknown, at: string, bounds: equationEquationArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return equationEquationArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) equationEquationArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) equationEquationArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const equationEquationArtifactGuardString = (value: unknown, at: string, bounds: equationEquationArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return equationEquationArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) equationEquationArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) equationEquationArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) equationEquationArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const equationEquationArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : equationEquationArtifactGuardReject(at, "value is not a boolean"));
export const equationEquationArtifactGuardNumber = (value: unknown, at: string, bounds: equationEquationArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return equationEquationArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) equationEquationArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) equationEquationArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const equationEquationArtifactGuardInteger = (value: unknown, at: string, bounds: equationEquationArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? equationEquationArtifactGuardNumber(value, at, bounds) : equationEquationArtifactGuardReject(at, "value is not an integer");
export const equationEquationArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : equationEquationArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const equationEquationArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : equationEquationArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseEquationArtifact(value: unknown, at = "$"): EquationArtifact {
  const row = equationEquationArtifactGuardObject(value, at);
  return {
    graph: parseEquationGraph(row["graph"], `${at}.graph`),
    geometry: parseEquationGeometry(row["geometry"], `${at}.geometry`),
    cameraX: equationEquationArtifactGuardNumber(row["cameraX"], `${at}.cameraX`),
    cameraY: equationEquationArtifactGuardNumber(row["cameraY"], `${at}.cameraY`),
    cameraZoom: equationEquationArtifactGuardNumber(row["cameraZoom"], `${at}.cameraZoom`),
  };
}

export function parseEquationGraph(value: unknown, at = "$"): EquationGraph {
  const row = equationEquationArtifactGuardObject(value, at);
  return {
    directed: equationEquationArtifactGuardBoolean(row["directed"], `${at}.directed`),
    nodes: equationEquationArtifactGuardArray(row["nodes"], `${at}.nodes`).map((item, index) => parseEquationNode(item, `${at}.nodes[${index}]`)),
    edges: equationEquationArtifactGuardArray(row["edges"], `${at}.edges`).map((item, index) => parseEquationEdge(item, `${at}.edges[${index}]`)),
    algorithm: equationEquationArtifactGuardString(row["algorithm"], `${at}.algorithm`),
    algorithmSeed: row["algorithmSeed"] === undefined ? undefined : equationEquationArtifactGuardString(row["algorithmSeed"], `${at}.algorithmSeed`),
  };
}

export function parseEquationNode(value: unknown, at = "$"): EquationNode {
  const row = equationEquationArtifactGuardObject(value, at);
  return {
    id: equationEquationArtifactGuardString(row["id"], `${at}.id`),
    label: equationEquationArtifactGuardString(row["label"], `${at}.label`),
    x: equationEquationArtifactGuardNumber(row["x"], `${at}.x`),
    y: equationEquationArtifactGuardNumber(row["y"], `${at}.y`),
  };
}

export function parseEquationEdge(value: unknown, at = "$"): EquationEdge {
  const row = equationEquationArtifactGuardObject(value, at);
  return {
    id: equationEquationArtifactGuardString(row["id"], `${at}.id`),
    source: equationEquationArtifactGuardString(row["source"], `${at}.source`),
    target: equationEquationArtifactGuardString(row["target"], `${at}.target`),
  };
}

export function parseEquationPoint(value: unknown, at = "$"): EquationPoint {
  const row = equationEquationArtifactGuardObject(value, at);
  return {
    x: equationEquationArtifactGuardNumber(row["x"], `${at}.x`),
    y: equationEquationArtifactGuardNumber(row["y"], `${at}.y`),
  };
}

export function parseEquationGeometry(value: unknown, at = "$"): EquationGeometry {
  const row = equationEquationArtifactGuardObject(value, at);
  return {
    points: equationEquationArtifactGuardArray(row["points"], `${at}.points`).map((item, index) => parseEquationPoint(item, `${at}.points[${index}]`)),
  };
}
