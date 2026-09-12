/** 🧬️ Persisted CAD application preferences that are independent of a window instance. */
export interface CadConfig {
  /** @state config */
  selectedNodeIds: string[];
  /** @state config */
  hoveredReferenceId?: string;
  /** @state config */
  engagementInput: string;
  /** @state config */
  engagementStep: string;
  /** @state config */
  activeExampleId?: string;
  /** @state config */
  selectedReferenceModelDefinitionId?: string;
  /** @state config */
  selectedReferenceId?: string;
  /** @state config */
  engagementPane?: string;
  /** @state config */
  engagementSessionJson?: string;
  /** @state config */
  engagementPreviewOperationJson?: string;
  /** @state config @minimum 0 @maximum 2147483647 */
  engagementPreviewGeneration: number;
  /** @state config */
  lastFinalizedInteractionId?: string;
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
  "engagementInput",
  "engagementStep",
  "activeExampleId",
  "selectedReferenceModelDefinitionId",
  "selectedReferenceId",
  "engagementPane",
  "engagementSessionJson",
  "engagementPreviewOperationJson",
  "engagementPreviewGeneration",
  "lastFinalizedInteractionId",
  "contributionsJson",
] as const;

/** 🚪️ Parses one exact application preference record and rejects unknown fields. */
export function parseCadConfig(value: unknown, at = "$"): CadConfig {
  const row = object(value, at);
  const unknown = Object.keys(row).find((key) => !(keys as readonly string[]).includes(key));
  if (unknown !== undefined) reject(`${at}.${unknown}`, "unknown field");
  if (!Array.isArray(row.selectedNodeIds)) reject(`${at}.selectedNodeIds`, "value is not an array");
  const generation = row.engagementPreviewGeneration;
  if (!Number.isInteger(generation) || (generation as number) < 0 || (generation as number) > 2_147_483_647) {
    reject(`${at}.engagementPreviewGeneration`, "value is not an integer in 0..2147483647");
  }
  return {
    selectedNodeIds: row.selectedNodeIds.map((item, index) => text(item, `${at}.selectedNodeIds[${index}]`)),
    hoveredReferenceId: optionalText(row.hoveredReferenceId, `${at}.hoveredReferenceId`),
    engagementInput: text(row.engagementInput, `${at}.engagementInput`),
    engagementStep: text(row.engagementStep, `${at}.engagementStep`),
    activeExampleId: optionalText(row.activeExampleId, `${at}.activeExampleId`),
    selectedReferenceModelDefinitionId: optionalText(row.selectedReferenceModelDefinitionId, `${at}.selectedReferenceModelDefinitionId`),
    selectedReferenceId: optionalText(row.selectedReferenceId, `${at}.selectedReferenceId`),
    engagementPane: optionalText(row.engagementPane, `${at}.engagementPane`),
    engagementSessionJson: optionalText(row.engagementSessionJson, `${at}.engagementSessionJson`),
    engagementPreviewOperationJson: optionalText(row.engagementPreviewOperationJson, `${at}.engagementPreviewOperationJson`),
    engagementPreviewGeneration: generation as number,
    lastFinalizedInteractionId: optionalText(row.lastFinalizedInteractionId, `${at}.lastFinalizedInteractionId`),
    contributionsJson: text(row.contributionsJson, `${at}.contributionsJson`),
  };
}
