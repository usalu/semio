/** 🫧️ Ephemeral local engagement state of one exact CAD world window (design §17.4) — the twin of Rust `CadWorldWindowTransient`. */
export interface CadWorldWindowTransient {
  engagementInput: string;
  engagementStep: string;
  engagementPane: string | null;
  engagementSessionJson: string | null;
  lastFinalizedInteractionId: string | null;
}
export interface CadWorldWindowTransientMutation { kind: "snapshot"; transient: CadWorldWindowTransient }

const KEYS = ["engagementInput", "engagementStep", "engagementPane", "engagementSessionJson", "lastFinalizedInteractionId"] as const;

/** 🚪️ Parses one exact window transient and refuses unknown, missing or mistyped fields by name. */
export function parseCadWorldWindowTransient(value: unknown, at = "$"): CadWorldWindowTransient {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error(`${at}: value is not an object`);
  const row = value as Record<string, unknown>;
  const unknown = Object.keys(row).find((key) => !(KEYS as readonly string[]).includes(key));
  if (unknown !== undefined) throw new Error(`${at}.${unknown}: unknown field`);
  const text = (key: (typeof KEYS)[number]): string => (typeof row[key] === "string" ? (row[key] as string) : (() => { throw new Error(`${at}.${key}: value is not a string`); })());
  const nullable = (key: (typeof KEYS)[number]): string | null => (row[key] === null ? null : text(key));
  const engagementStep = text("engagementStep");
  if (engagementStep.length === 0) throw new Error(`${at}.engagementStep: an engagement step is never empty`);
  return { engagementInput: text("engagementInput"), engagementStep, engagementPane: nullable("engagementPane"), engagementSessionJson: nullable("engagementSessionJson"), lastFinalizedInteractionId: nullable("lastFinalizedInteractionId") };
}

/** 🧮️ The state a `snapshot` mutation leaves: its transient, whatever the base. */
export const applyCadWorldWindowTransientMutation = (_base: CadWorldWindowTransient, mutation: CadWorldWindowTransientMutation): CadWorldWindowTransient => structuredClone(mutation.transient);
