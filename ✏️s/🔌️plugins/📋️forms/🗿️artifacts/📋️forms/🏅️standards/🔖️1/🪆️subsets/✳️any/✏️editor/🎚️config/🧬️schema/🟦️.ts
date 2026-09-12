/** 🧬️ Forms app configuration. */
export interface FormsConfig {
  /** @state config Host contribution projection. */
  contributionsJson: string;
}

export class formsFormsConfigGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) { super(`${at}: ${why}`); }
}

export function parseFormsConfig(value: unknown, at = "$"): FormsConfig {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new formsFormsConfigGuardRefusal(at, "value is not an object");
  const row = value as Record<string, unknown>;
  if (typeof row.contributionsJson !== "string") throw new formsFormsConfigGuardRefusal(`${at}.contributionsJson`, "value is not a string");
  return { contributionsJson: row.contributionsJson };
}
