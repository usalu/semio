import { parseViewport2d, type Viewport2d } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🟦️.ts";

/** 👥️ Shareable live BIM presence. */
export interface BimPresence {
  engagementInput: string;
  storey: string;
  camera: Viewport2d;
}

/** 🚪️ Parses one exact BIM presence value. */
export function parseBimPresence(value: unknown): BimPresence {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("$ must be an object");
  const row = value as Record<string, unknown>;
  const keys = Object.keys(row);
  if (keys.length !== 3 || !keys.includes("engagementInput") || !keys.includes("storey") || !keys.includes("camera")) throw new TypeError("$ must contain only engagementInput, storey and camera");
  if (typeof row.engagementInput !== "string") throw new TypeError("$.engagementInput must be a string");
  if (typeof row.storey !== "string") throw new TypeError("$.storey must be a string");
  return { engagementInput: row.engagementInput, storey: row.storey, camera: parseViewport2d(row.camera) };
}
