/** 👁️ Shared analysis display vocabulary for FEM result windows. */
export type ResultMode = "static" | "modal" | "buckling";

/** 🧭️ Admit one result mode from an external value. */
export function parseResultMode(value: unknown): ResultMode {
  if (value !== "static" && value !== "modal" && value !== "buckling") throw new TypeError("Unknown FEM result mode");
  return value;
}
