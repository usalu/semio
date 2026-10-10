import type { DragTransforms } from "../../../../../🧬️schema/🧬️mutations/✋️drag-transforms/🦠️mutation/🟦️.ts";

/** 🚪️ Parses one `drag-transforms` payload the way its JSON Schema admits it, or throws. */
export function parseDragTransforms(value: unknown): DragTransforms {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("drag-transforms: payload is not an object");
  const row = value as Record<string, unknown>;
  const unknownKey = Object.keys(row).find((key) => !["targets", "dx", "dy", "dz", "mutation"].includes(key));
  if (unknownKey !== undefined) throw new TypeError(`drag-transforms: unknown field ${unknownKey}`);
  if (row.mutation !== undefined && row.mutation !== "dragTransforms") throw new TypeError("drag-transforms: wrong mutation tag");
  if (!Array.isArray(row.targets) || row.targets.length === 0 || row.targets.some((entry) => typeof entry !== "string" || entry.length === 0) || new Set(row.targets).size !== row.targets.length) throw new TypeError("drag-transforms: targets must be a nonempty list of unique ids");
  if (typeof row.dx !== "number" || !Number.isFinite(row.dx)) throw new TypeError("drag-transforms: dx must be a finite number");
  if (typeof row.dy !== "number" || !Number.isFinite(row.dy)) throw new TypeError("drag-transforms: dy must be a finite number");
  if (typeof row.dz !== "number" || !Number.isFinite(row.dz)) throw new TypeError("drag-transforms: dz must be a finite number");
  return { targets: row.targets as string[], dx: row.dx as number, dy: row.dy as number, dz: row.dz as number };
}
