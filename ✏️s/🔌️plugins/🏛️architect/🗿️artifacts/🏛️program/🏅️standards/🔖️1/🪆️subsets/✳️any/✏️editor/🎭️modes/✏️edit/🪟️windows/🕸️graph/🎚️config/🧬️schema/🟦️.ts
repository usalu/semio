import { parseViewport2d, type Viewport2d } from "../../../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/◻️2d/🧬️schema/🟦️.ts";

/** 🕸️ Persisted shared viewport for one exact Architect Graph window. */
export interface ArchitectGraphWindowConfig { viewport: Viewport2d }

/** 🔁️ Exact Graph-window viewport mutation. */
export type ArchitectGraphWindowConfigMutation = { kind: "set-viewport"; viewport: Viewport2d };

/** 🚪️ Parses one exact Graph-window configuration. */
export function parseArchitectGraphWindowConfig(value: unknown): ArchitectGraphWindowConfig {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("$ must be an object");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).length !== 1 || !("viewport" in row)) throw new TypeError("$ must contain only viewport");
  return { viewport: parseViewport2d(row.viewport) };
}

/** 🧬️ Applies one Graph-window viewport mutation. */
export function applyArchitectGraphWindowConfigMutation(_base: ArchitectGraphWindowConfig, mutation: ArchitectGraphWindowConfigMutation): ArchitectGraphWindowConfig {
  return parseArchitectGraphWindowConfig({ viewport: mutation.viewport });
}
