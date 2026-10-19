/** 👁️ Shared analysis display vocabulary for FEM result windows. */
export type ResultMode = "static" | "modal" | "buckling";

/** 🧭️ Admit one result mode from an external value. */
export function parseResultMode(value: unknown): ResultMode {
  if (value !== "static" && value !== "modal" && value !== "buckling") throw new TypeError("Unknown FEM result mode");
  return value;
}

/** 🎵️ Both eigenvalue analyses display normalized mode shapes. */
export function resultModeUsesShape(mode: ResultMode): boolean {
  return mode === "modal" || mode === "buckling";
}

/** 📏️ Projects an admitted eigenvalue into the mode's physical display unit. */
export function resultModeEigenvalue(mode: ResultMode, eigenvalue: number): number | null {
  return mode === "static" ? null : mode === "buckling" ? eigenvalue : Math.sqrt(Math.max(0, eigenvalue)) / (2 * Math.PI);
}
