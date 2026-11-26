import { parseViewport2d, type Viewport2d } from "@semio-tech/framework/ui/viewport/2d/schema";

/** 👥️ Shareable live BIM presence. */
export interface BimPresence {
  engagementInput: string;
  storey: string;
  camera: Viewport2d;
  ownedWorksets: string[];
}

/** 🚪️ Parses one exact BIM presence value. */
export function parseBimPresence(value: unknown): BimPresence {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("$ must be an object");
  const row = value as Record<string, unknown>;
  const keys = Object.keys(row);
  if (keys.length !== 4 || !keys.includes("engagementInput") || !keys.includes("storey") || !keys.includes("camera") || !keys.includes("ownedWorksets")) throw new TypeError("$ must contain only engagementInput, storey, camera and ownedWorksets");
  if (typeof row.engagementInput !== "string") throw new TypeError("$.engagementInput must be a string");
  if (typeof row.storey !== "string") throw new TypeError("$.storey must be a string");
  if (!Array.isArray(row.ownedWorksets) || !row.ownedWorksets.every(id => typeof id === "string")) throw new TypeError("$.ownedWorksets must be an array of strings");
  return { engagementInput: row.engagementInput, storey: row.storey, camera: parseViewport2d(row.camera), ownedWorksets: row.ownedWorksets };
}
