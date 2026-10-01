/** 📏️ generation3d direct `scale-transforms` payload mirror of `ScaleTransforms`, with its closed-schema parser. */
export interface ScaleTransforms {
  targets: string[];
  sx: number;
  sy: number;
  sz: number;
}

/** 🚪️ Parses one `scale-transforms` payload the way its JSON Schema admits it, or throws. */
export function parseScaleTransforms(value: unknown): ScaleTransforms {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("scale-transforms: payload is not an object");
  const row = value as Record<string, unknown>;
  const unknownKey = Object.keys(row).find((key) => !["targets", "sx", "sy", "sz", "mutation"].includes(key));
  if (unknownKey !== undefined) throw new TypeError(`scale-transforms: unknown field ${unknownKey}`);
  if (row.mutation !== undefined && row.mutation !== "scaleTransforms") throw new TypeError("scale-transforms: wrong mutation tag");
  if (!Array.isArray(row.targets) || row.targets.length === 0 || row.targets.some((entry) => typeof entry !== "string" || entry.length === 0) || new Set(row.targets).size !== row.targets.length) throw new TypeError("scale-transforms: targets must be a nonempty list of unique ids");
  if (typeof row.sx !== "number" || !Number.isFinite(row.sx) || row.sx <= 0) throw new TypeError("scale-transforms: sx must be a finite positive number");
  if (typeof row.sy !== "number" || !Number.isFinite(row.sy) || row.sy <= 0) throw new TypeError("scale-transforms: sy must be a finite positive number");
  if (typeof row.sz !== "number" || !Number.isFinite(row.sz) || row.sz <= 0) throw new TypeError("scale-transforms: sz must be a finite positive number");
  return { targets: row.targets as string[], sx: row.sx as number, sy: row.sy as number, sz: row.sz as number };
}
