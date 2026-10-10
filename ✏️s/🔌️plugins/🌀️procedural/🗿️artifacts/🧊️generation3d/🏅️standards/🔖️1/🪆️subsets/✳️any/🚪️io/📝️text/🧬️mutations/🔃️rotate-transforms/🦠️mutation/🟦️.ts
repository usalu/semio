import type { RotateTransforms } from "../../../../../🧬️schema/🧬️mutations/🔃️rotate-transforms/🦠️mutation/🟦️.ts";
import { assertRotationAxis } from "../../../../../🧬️schema/🧬️mutations/🔃️rotate-transforms/🦠️mutation/🟦️.ts";

/** 🚪️ Parses one `rotate-transforms` payload the way its JSON Schema admits it, or throws. */
export function parseRotateTransforms(value: unknown): RotateTransforms {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("rotate-transforms: payload is not an object");
  const row = value as Record<string, unknown>;
  const unknownKey = Object.keys(row).find((key) => !["targets", "ax", "ay", "az", "angle", "mutation"].includes(key));
  if (unknownKey !== undefined) throw new TypeError(`rotate-transforms: unknown field ${unknownKey}`);
  if (row.mutation !== undefined && row.mutation !== "rotateTransforms") throw new TypeError("rotate-transforms: wrong mutation tag");
  if (!Array.isArray(row.targets) || row.targets.length === 0 || row.targets.some((entry) => typeof entry !== "string" || entry.length === 0) || new Set(row.targets).size !== row.targets.length) throw new TypeError("rotate-transforms: targets must be a nonempty list of unique ids");
  if (typeof row.ax !== "number" || !Number.isFinite(row.ax)) throw new TypeError("rotate-transforms: ax must be a finite number");
  if (typeof row.ay !== "number" || !Number.isFinite(row.ay)) throw new TypeError("rotate-transforms: ay must be a finite number");
  if (typeof row.az !== "number" || !Number.isFinite(row.az)) throw new TypeError("rotate-transforms: az must be a finite number");
  if (typeof row.angle !== "number" || !Number.isFinite(row.angle)) throw new TypeError("rotate-transforms: angle must be a finite number");
  assertRotationAxis({ ax: row.ax, ay: row.ay, az: row.az });
  return { targets: row.targets as string[], ax: row.ax as number, ay: row.ay as number, az: row.az as number, angle: row.angle as number };
}
