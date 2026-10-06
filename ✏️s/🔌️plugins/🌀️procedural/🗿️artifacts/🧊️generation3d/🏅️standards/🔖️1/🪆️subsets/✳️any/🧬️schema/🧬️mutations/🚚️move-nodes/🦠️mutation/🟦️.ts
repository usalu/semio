import { binary64 } from "../../../🟦️.ts";
import {type Binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
/** 🚚️ generation3d direct `move-nodes` payload mirror of `MoveNodes`, with its closed-schema parser. */
export interface MoveNodes {
  ids: string[];
  dx: Binary64;
  dy: Binary64;
}

/** 🚪️ Parses one `move-nodes` payload the way its JSON Schema admits it, or throws. */
export function parseMoveNodes(value: unknown): MoveNodes {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("move-nodes: payload is not an object");
  const row = value as Record<string, unknown>;
  const unknownKey = Object.keys(row).find((key) => !["ids", "dx", "dy", "mutation"].includes(key));
  if (unknownKey !== undefined) throw new TypeError(`move-nodes: unknown field ${unknownKey}`);
  if (row.mutation !== undefined && row.mutation !== "moveNodes") throw new TypeError("move-nodes: wrong mutation tag");
  if (!Array.isArray(row.ids) || row.ids.length === 0 || row.ids.some((entry) => typeof entry !== "string" || entry.length === 0) || new Set(row.ids).size !== row.ids.length) throw new TypeError("move-nodes: ids must be a nonempty list of unique ids");
  if (typeof row.dx !== "number" || !Number.isFinite(row.dx)) throw new TypeError("move-nodes: dx must be a finite number");
  if (typeof row.dy !== "number" || !Number.isFinite(row.dy)) throw new TypeError("move-nodes: dy must be a finite number");
  return { ids: row.ids as string[], dx: binary64(row.dx), dy: binary64(row.dy) };
}
