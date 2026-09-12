/** 🧬️ Sequence diff schema — sparse field delta. */
import type { ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface SequenceDiff {
  /** @state artifact */ artifact?: SequenceArtifact;
  /** @state artifact */ schema?: string;
  /** @state artifact @child kind=s.stdio.semio */ content?: ArtifactChild;
}
export interface SequenceStepsDelta { added: SequenceStep[]; removed: string[]; patched: SequenceStepPatchEntry[]; reordered?: string[]; }
export interface SequenceEdgesDelta { added: SequenceEdge[]; removed: string[]; patched: SequenceEdgePatchEntry[]; reordered?: string[]; }
export interface SequenceStepPatchEntry { id: string; patch: SequenceStepPatch; }
export interface SequenceEdgePatchEntry { id: string; patch: SequenceEdgePatch; }
export interface SequenceStep { id: string; kind: string; params: Record<string, unknown>; x: number; y: number; slot?: SlotRef; collapsed: boolean; }
export interface SequenceEdge { id: string; from: string; to: string; }
export interface SlotRef { owner: string; name: string; }
export interface SequenceStepPatch { params?: Record<string, unknown>; x?: number; y?: number; collapsed?: boolean; }
export interface SequenceEdgePatch { from?: string; to?: string; }
export interface SequenceCamera { x: number; y: number; zoom: number; }
export interface SequenceArtifact {
  schema: string; content: ArtifactChild;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class sequenceSequenceDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const sequenceSequenceDiffGuardReject = (at: string, why: string): never => {
  throw new sequenceSequenceDiffGuardRefusal(at, why);
};

type sequenceSequenceDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type sequenceSequenceDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type sequenceSequenceDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const sequenceSequenceDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : sequenceSequenceDiffGuardReject(at, "value is not an object");
export const sequenceSequenceDiffGuardArray = (value: unknown, at: string, bounds: sequenceSequenceDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return sequenceSequenceDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) sequenceSequenceDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) sequenceSequenceDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const sequenceSequenceDiffGuardString = (value: unknown, at: string, bounds: sequenceSequenceDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return sequenceSequenceDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) sequenceSequenceDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) sequenceSequenceDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) sequenceSequenceDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const sequenceSequenceDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : sequenceSequenceDiffGuardReject(at, "value is not a boolean"));
export const sequenceSequenceDiffGuardNumber = (value: unknown, at: string, bounds: sequenceSequenceDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return sequenceSequenceDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) sequenceSequenceDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) sequenceSequenceDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const sequenceSequenceDiffGuardInteger = (value: unknown, at: string, bounds: sequenceSequenceDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? sequenceSequenceDiffGuardNumber(value, at, bounds) : sequenceSequenceDiffGuardReject(at, "value is not an integer");
export const sequenceSequenceDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : sequenceSequenceDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const sequenceSequenceDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : sequenceSequenceDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface SequenceStringList {
  readonly values: readonly string[];
}

export function parseSequenceStringList(value: unknown, at = "$"): SequenceStringList {
  const row = sequenceSequenceDiffGuardObject(value, at);
  return {
    values: sequenceSequenceDiffGuardArray(row["values"], `${at}.values`).map((item, index) => sequenceSequenceDiffGuardString(item, `${at}.values[${index}]`)),
  };
}

export function parseSequenceStepPatchEntry(value: unknown, at = "$"): SequenceStepPatchEntry {
  const row = sequenceSequenceDiffGuardObject(value, at);
  return {
    id: sequenceSequenceDiffGuardString(row["id"], `${at}.id`),
    patch: parseSequenceStepPatch(row["patch"], `${at}.patch`),
  };
}

export function parseSequenceEdgePatchEntry(value: unknown, at = "$"): SequenceEdgePatchEntry {
  const row = sequenceSequenceDiffGuardObject(value, at);
  return {
    id: sequenceSequenceDiffGuardString(row["id"], `${at}.id`),
    patch: parseSequenceEdgePatch(row["patch"], `${at}.patch`),
  };
}

export function parseSequenceStepPatch(value: unknown, at = "$"): SequenceStepPatch {
  const row = sequenceSequenceDiffGuardObject(value, at);
  return {
    params: row["params"] === undefined ? undefined : sequenceSequenceDiffGuardObject(row["params"], `${at}.params`),
    x: row["x"] === undefined ? undefined : sequenceSequenceDiffGuardNumber(row["x"], `${at}.x`),
    y: row["y"] === undefined ? undefined : sequenceSequenceDiffGuardNumber(row["y"], `${at}.y`),
    collapsed: row["collapsed"] === undefined ? undefined : sequenceSequenceDiffGuardBoolean(row["collapsed"], `${at}.collapsed`),
  };
}

export function parseSequenceEdgePatch(value: unknown, at = "$"): SequenceEdgePatch {
  const row = sequenceSequenceDiffGuardObject(value, at);
  return {
    from: row["from"] === undefined ? undefined : sequenceSequenceDiffGuardString(row["from"], `${at}.from`),
    to: row["to"] === undefined ? undefined : sequenceSequenceDiffGuardString(row["to"], `${at}.to`),
  };
}
