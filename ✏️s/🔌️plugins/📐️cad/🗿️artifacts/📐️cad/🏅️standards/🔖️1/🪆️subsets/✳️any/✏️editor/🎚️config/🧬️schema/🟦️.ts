/** 🧬️ Persisted CAD application preferences that are independent of a window instance. */
export interface CadConfig {
  /** @state config */
  selectedNodeIds: string[];
  /** @state config */
  hoveredReferenceId?: string;
  /** @state config */
  activeExampleId?: string;
  /** @state config */
  selectedReferenceModelDefinitionId?: string;
  /** @state config */
  selectedReferenceId?: string;
  /** @state config */
  contributionsJson: string;
}

/** 🚪️ A strict CAD application config parse refusal. */
export class CadConfigRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const reject = (at: string, why: string): never => {
  throw new CadConfigRefusal(at, why);
};

const object = (value: unknown, at: string): Record<string, unknown> =>
  value !== null && typeof value === "object" && !Array.isArray(value)
    ? value as Record<string, unknown>
    : reject(at, "value is not an object");

const text = (value: unknown, at: string): string =>
  typeof value === "string" ? value : reject(at, "value is not a string");

const optionalText = (value: unknown, at: string): string | undefined =>
  value === undefined ? undefined : text(value, at);

const keys = [
  "selectedNodeIds",
  "hoveredReferenceId",
  "activeExampleId",
  "selectedReferenceModelDefinitionId",
  "selectedReferenceId",
  "contributionsJson",
] as const;

/** 🚪️ Parses one exact application preference record and rejects unknown fields. */
export function parseCadConfig(value: unknown, at = "$"): CadConfig {
  const row = object(value, at);
  const unknown = Object.keys(row).find((key) => !(keys as readonly string[]).includes(key));
  if (unknown !== undefined) reject(`${at}.${unknown}`, "unknown field");
  if (!Array.isArray(row.selectedNodeIds)) reject(`${at}.selectedNodeIds`, "value is not an array");
  return {
    selectedNodeIds: row.selectedNodeIds.map((item, index) => text(item, `${at}.selectedNodeIds[${index}]`)),
    hoveredReferenceId: optionalText(row.hoveredReferenceId, `${at}.hoveredReferenceId`),
    activeExampleId: optionalText(row.activeExampleId, `${at}.activeExampleId`),
    selectedReferenceModelDefinitionId: optionalText(row.selectedReferenceModelDefinitionId, `${at}.selectedReferenceModelDefinitionId`),
    selectedReferenceId: optionalText(row.selectedReferenceId, `${at}.selectedReferenceId`),
    contributionsJson: text(row.contributionsJson, `${at}.contributionsJson`),
  };
}
