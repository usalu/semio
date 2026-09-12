/** 📓️ Persisted authored-record selection for one exact Architect Report window. */
export interface ArchitectReportWindowConfig { selectedReportId?: string }

/** 🔁️ Exact Report-window record selection mutation. */
export type ArchitectReportWindowConfigMutation = { kind: "select-report"; selectedReportId?: string };

/** 🚪️ Parses one exact Report-window configuration. */
export function parseArchitectReportWindowConfig(value: unknown): ArchitectReportWindowConfig {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("$ must be an object");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).some((key) => key !== "selectedReportId")) throw new TypeError("$ contains an unknown field");
  if (row.selectedReportId === undefined) return {};
  if (typeof row.selectedReportId !== "string") throw new TypeError("$.selectedReportId must be a string");
  return { selectedReportId: row.selectedReportId };
}

/** 🧬️ Applies one Report-window record selection mutation. */
export function applyArchitectReportWindowConfigMutation(_base: ArchitectReportWindowConfig, mutation: ArchitectReportWindowConfigMutation): ArchitectReportWindowConfig {
  return parseArchitectReportWindowConfig(mutation.selectedReportId === undefined ? {} : { selectedReportId: mutation.selectedReportId });
}
