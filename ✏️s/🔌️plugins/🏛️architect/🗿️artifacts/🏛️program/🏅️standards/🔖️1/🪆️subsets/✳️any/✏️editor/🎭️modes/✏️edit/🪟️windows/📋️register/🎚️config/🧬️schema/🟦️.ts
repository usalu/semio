/** 📋️ Persisted selection for one exact Architect Register window. */
export interface ArchitectRegisterWindowConfig { activeRegister: string }

/** 🔁️ Exact Register-window selection mutation. */
export type ArchitectRegisterWindowConfigMutation = { kind: "set-active-register"; activeRegister: string };

/** 🚪️ Parses one exact Register-window configuration. */
export function parseArchitectRegisterWindowConfig(value: unknown): ArchitectRegisterWindowConfig {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("$ must be an object");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).length !== 1 || typeof row.activeRegister !== "string") throw new TypeError("$ must contain only activeRegister");
  return { activeRegister: row.activeRegister };
}

/** 🧬️ Applies one Register-window selection mutation. */
export function applyArchitectRegisterWindowConfigMutation(_base: ArchitectRegisterWindowConfig, mutation: ArchitectRegisterWindowConfigMutation): ArchitectRegisterWindowConfig {
  return parseArchitectRegisterWindowConfig({ activeRegister: mutation.activeRegister });
}
