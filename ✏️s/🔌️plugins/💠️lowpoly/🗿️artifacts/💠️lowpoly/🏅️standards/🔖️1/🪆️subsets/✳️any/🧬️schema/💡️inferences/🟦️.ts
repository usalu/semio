/** 💡️ Lowpoly inference schema — object count + 3d bounding box across every object's transform
 * position. */

export interface LowpolyBounds {
  min: [number, number, number];
  max: [number, number, number];
}

export interface LowpolyInference {
  /** @derived */
  objectCount: number;
  /** @derived */
  bounds: LowpolyBounds | null;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class lowpolyLowpolyInferenceGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const lowpolyLowpolyInferenceGuardReject = (at: string, why: string): never => {
  throw new lowpolyLowpolyInferenceGuardRefusal(at, why);
};

type lowpolyLowpolyInferenceGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type lowpolyLowpolyInferenceGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type lowpolyLowpolyInferenceGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const lowpolyLowpolyInferenceGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : lowpolyLowpolyInferenceGuardReject(at, "value is not an object");
export const lowpolyLowpolyInferenceGuardArray = (value: unknown, at: string, bounds: lowpolyLowpolyInferenceGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return lowpolyLowpolyInferenceGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) lowpolyLowpolyInferenceGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) lowpolyLowpolyInferenceGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const lowpolyLowpolyInferenceGuardString = (value: unknown, at: string, bounds: lowpolyLowpolyInferenceGuardTextBounds = {}): string => {
  if (typeof value !== "string") return lowpolyLowpolyInferenceGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) lowpolyLowpolyInferenceGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) lowpolyLowpolyInferenceGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) lowpolyLowpolyInferenceGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const lowpolyLowpolyInferenceGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : lowpolyLowpolyInferenceGuardReject(at, "value is not a boolean"));
export const lowpolyLowpolyInferenceGuardNumber = (value: unknown, at: string, bounds: lowpolyLowpolyInferenceGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return lowpolyLowpolyInferenceGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) lowpolyLowpolyInferenceGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) lowpolyLowpolyInferenceGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const lowpolyLowpolyInferenceGuardInteger = (value: unknown, at: string, bounds: lowpolyLowpolyInferenceGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? lowpolyLowpolyInferenceGuardNumber(value, at, bounds) : lowpolyLowpolyInferenceGuardReject(at, "value is not an integer");
export const lowpolyLowpolyInferenceGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : lowpolyLowpolyInferenceGuardReject(at, `value is not one of ${members.join(", ")}`);
export const lowpolyLowpolyInferenceGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : lowpolyLowpolyInferenceGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLowpolyBounds(value: unknown, at = "$"): LowpolyBounds {
  const row = lowpolyLowpolyInferenceGuardObject(value, at);
  const triple = (value: unknown, path: string): [number, number, number] => {
    const items = lowpolyLowpolyInferenceGuardArray(value, path, { minItems: 3, maxItems: 3 });
    return [0, 1, 2].map(index => lowpolyLowpolyInferenceGuardNumber(items[index], `${path}[${index}]`)) as [number, number, number];
  };
  return {
    min: triple(row["min"], `${at}.min`),
    max: triple(row["max"], `${at}.max`),
  };
}

import type { LowpolyTransform } from "../🟦️.ts";
import { binary32Value, type Binary32 } from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";

/** 🔄️ Semantic XYZ Euler degrees to a unit quaternion [x,y,z,w]. */
export function eulerDegreesToQuaternion(rotation: readonly [Binary32, Binary32, Binary32]): [number, number, number, number] {
  const [x, y, z] = rotation.map(angle => binary32Value(angle) * Math.PI / 360);
  const sx = Math.sin(x!), cx = Math.cos(x!), sy = Math.sin(y!), cy = Math.cos(y!), sz = Math.sin(z!), cz = Math.cos(z!);
  return [sx * cy * cz + cx * sy * sz, cx * sy * cz - sx * cy * sz, cx * cy * sz + sx * sy * cz, cx * cy * cz - sx * sy * sz];
}

/** 🧭️ Applies a unit quaternion to a local vector. */
export function rotate(quaternion: readonly [number, number, number, number], vector: readonly [number, number, number]): [number, number, number] {
  const [x, y, z, w] = quaternion, [vx, vy, vz] = vector;
  const tx = 2 * (y * vz - z * vy), ty = 2 * (z * vx - x * vz), tz = 2 * (x * vy - y * vx);
  return [vx + w * tx + y * tz - z * ty, vy + w * ty + z * tx - x * tz, vz + w * tz + x * ty - y * tx];
}

/** 🌍️ Scales, rotates and translates an intrinsic local position. */
export function applyTransform(transform: LowpolyTransform, local: readonly [number, number, number]): [number, number, number] {
  const scaled = local.map((value, index) => Math.fround(Math.fround(value) * binary32Value(transform.scale[index]!))) as [number, number, number];
  return rotate(eulerDegreesToQuaternion(transform.rotation), scaled).map((value, index) => value + binary32Value(transform.position[index]!)) as [number, number, number];
}

/** 📐️ Unit normal of a triangle, or zero for a degenerate triangle. */
export function triangleNormal(a: readonly [number, number, number], b: readonly [number, number, number], c: readonly [number, number, number]): [number, number, number] {
  const u = b.map((value, index) => value - a[index]!), v = c.map((value, index) => value - a[index]!);
  const normal: [number, number, number] = [u[1]! * v[2]! - u[2]! * v[1]!, u[2]! * v[0]! - u[0]! * v[2]!, u[0]! * v[1]! - u[1]! * v[0]!];
  const length = Math.hypot(...normal);
  return length < 1e-12 ? [0, 0, 0] : normal.map(value => value / length) as [number, number, number];
}
